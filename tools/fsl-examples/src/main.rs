//! `fsl-examples`: the command line of the example runner.
//!
//! ```text
//! fsl-examples [--root DIR] check   [dir ...] [--case PATTERN ...] running cases must match their record
//! fsl-examples [--root DIR] staged  [dir ...]           staged cases must still fail as recorded
//! fsl-examples [--root DIR] regenerate [dir ...] [--case PATTERN ...] [--dry-run]
//! fsl-examples [--root DIR] record  [dir ...]           alias for regenerate
//! fsl-examples [--root DIR] promote <case> [--verdict V]  flip a staged case to running
//! fsl-examples [--root DIR] readme | index              rewrite the README regions and index.json
//! ```
//!
//! The driver comes from the `FSL` environment variable. The root is the repository folder and
//! defaults to the current folder. A `<dir>` can be a system name or a folder holding
//! `example.toml`; the default is every folder under `examples/`. `--case` accepts an id or
//! `<system>/<id>` and may be repeated; `*` and `?` match case-name characters.

use fsl_examples::runner::{Command, Driver, ProcessDriver, StdSink, execute};
use fsl_examples::{Error, ErrorCode, Result};
use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str = "usage: fsl-examples [--root DIR] check|staged [dir ...] [--case PATTERN ...] | regenerate|record [dir ...] [--case PATTERN ...] [--dry-run] | promote <system>/<case> [--verdict V] | readme | index";

fn parse_case_options(
    operands: &[String],
    dry_run: bool,
) -> Result<(Vec<PathBuf>, Vec<String>, bool)> {
    let mut dirs = Vec::new();
    let mut cases = Vec::new();
    let mut dry = false;
    let mut it = operands.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--case" => cases.push(
                it.next()
                    .ok_or_else(|| Error::new(ErrorCode::Usage, USAGE))?
                    .clone(),
            ),
            "--dry-run" if dry_run => dry = true,
            "--dry-run" => return Err(Error::new(ErrorCode::Usage, USAGE)),
            _ if arg.starts_with('-') => return Err(Error::new(ErrorCode::Usage, USAGE)),
            _ => dirs.push(PathBuf::from(arg)),
        }
    }
    Ok((dirs, cases, dry))
}

fn parse(args: &[String]) -> Result<(PathBuf, Command)> {
    let usage = || Error::new(ErrorCode::Usage, USAGE);
    let mut rest: Vec<String> = args.to_vec();
    let mut root = PathBuf::from(".");
    if rest.first().map(String::as_str) == Some("--root") {
        root = PathBuf::from(rest.get(1).ok_or_else(usage)?);
        rest.drain(..2);
    }
    let (name, operands) = rest.split_first().ok_or_else(usage)?;
    let command = match name.as_str() {
        "check" => {
            let (dirs, cases, _) = parse_case_options(operands, false)?;
            Command::Check { dirs, cases }
        }
        "staged" => {
            let (dirs, cases, _) = parse_case_options(operands, false)?;
            if !cases.is_empty() {
                return Err(Error::new(
                    ErrorCode::Usage,
                    "--case is supported by check and regenerate",
                ));
            }
            Command::Staged(dirs)
        }
        "record" | "regenerate" => {
            let (dirs, cases, dry_run) = parse_case_options(operands, true)?;
            Command::Record {
                dirs,
                cases,
                dry_run,
            }
        }
        "readme" | "index" => Command::Generate,
        "promote" => {
            let mut verdict = None;
            let mut case = None;
            let mut it = operands.iter();
            while let Some(arg) = it.next() {
                if arg == "--verdict" {
                    verdict = Some(it.next().ok_or_else(usage)?.clone());
                } else if case.is_none() {
                    case = Some(arg.clone());
                } else {
                    return Err(usage());
                }
            }
            Command::Promote {
                case: case.ok_or_else(usage)?,
                verdict,
            }
        }
        _ => return Err(usage()),
    };
    Ok((root, command))
}

/// Stands in for the driver of commands that never run it.
struct NoDriver;

impl Driver for NoDriver {
    fn run(&self, _: &std::path::Path, _: &[String]) -> Result<fsl_examples::outcome::Run> {
        Err(Error::new(
            ErrorCode::Driver,
            "this command does not run the driver",
        ))
    }
}

fn run() -> Result<bool> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (root, command) = parse(&args)?;
    let mut sink = StdSink;
    if !command.needs_driver() {
        return execute(&command, &root, &NoDriver, &mut sink);
    }
    let fsl = std::env::var_os("FSL")
        .filter(|v| !v.is_empty())
        .ok_or_else(|| {
            Error::new(
                ErrorCode::Usage,
                "FSL is not set; run through make or set FSL to the FSL driver binary",
            )
        })?;
    execute(&command, &root, &ProcessDriver::new(fsl), &mut sink)
}

fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Command, parse};
    use std::path::PathBuf;

    #[test]
    fn check_parses_repeated_case_selectors() {
        let args = [
            "check".to_owned(),
            "shop".to_owned(),
            "--case".to_owned(),
            "paid-*".to_owned(),
            "--case".to_owned(),
            "rover/safe-stop-late".to_owned(),
        ];
        let (_, command) = parse(&args).expect("valid options");
        assert_eq!(
            command,
            Command::Check {
                dirs: vec![PathBuf::from("shop")],
                cases: vec!["paid-*".to_owned(), "rover/safe-stop-late".to_owned()],
            }
        );
    }

    #[test]
    fn record_aliases_regenerate_and_accepts_dry_run() {
        for name in ["record", "regenerate"] {
            let args = [
                name.to_owned(),
                "shop".to_owned(),
                "--case".to_owned(),
                "paid-short".to_owned(),
                "--dry-run".to_owned(),
            ];
            let (_, command) = parse(&args).expect("valid options");
            assert_eq!(
                command,
                Command::Record {
                    dirs: vec![PathBuf::from("shop")],
                    cases: vec!["paid-short".to_owned()],
                    dry_run: true,
                }
            );
        }
    }
}
