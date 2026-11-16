//! Running the cases of the example systems and recording their outcomes.

use crate::error::{Error, ErrorCode, Result};
use crate::generated::sync_generated;
use crate::manifest::{Case, Manifest, Status, parse_manifest, promote_text};
use crate::outcome::{Recorded, Run, evaluate_case, project, verdict_of};
use serde_json::Value;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command as Process;

/// Runs the FSL driver.
pub trait Driver {
    /// Runs the driver with `args` in the folder `dir`.
    fn run(&self, dir: &Path, args: &[String]) -> Result<Run>;
}

/// The FSL driver as a program on disk, named by the `FSL` variable.
#[derive(Debug, Clone)]
pub struct ProcessDriver {
    program: PathBuf,
}

impl ProcessDriver {
    /// A driver that starts `program`.
    pub fn new(program: impl Into<PathBuf>) -> Self {
        Self {
            program: program.into(),
        }
    }
}

impl Driver for ProcessDriver {
    fn run(&self, dir: &Path, args: &[String]) -> Result<Run> {
        let output = Process::new(&self.program)
            .args(args)
            .current_dir(dir)
            .output()
            .map_err(|e| {
                Error::new(
                    ErrorCode::Driver,
                    format!(
                        "cannot run FSL={}: {e}. Set FSL to the FSL driver binary, for example make check FSL=/path/to/driver",
                        self.program.display()
                    ),
                )
            })?;
        Ok(Run {
            exit: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        })
    }
}

/// Where the runner prints its lines.
pub trait Sink {
    /// A progress or passing line.
    fn info(&mut self, line: &str);
    /// A failing line.
    fn error(&mut self, line: &str);
}

/// Prints passing lines on standard output and failing lines on standard error.
#[derive(Debug, Default)]
pub struct StdSink;

impl Sink for StdSink {
    fn info(&mut self, line: &str) {
        println!("{line}");
    }
    fn error(&mut self, line: &str) {
        eprintln!("{line}");
    }
}

/// Collects the lines, for tests.
#[derive(Debug, Default)]
pub struct VecSink {
    /// Passing and progress lines.
    pub info: Vec<String>,
    /// Failing lines.
    pub errors: Vec<String>,
}

impl Sink for VecSink {
    fn info(&mut self, line: &str) {
        self.info.push(line.to_owned());
    }
    fn error(&mut self, line: &str) {
        self.errors.push(line.to_owned());
    }
}

/// One example system: a folder holding `example.toml`.
#[derive(Debug, Clone)]
pub struct System {
    /// The folder, as an absolute path.
    pub dir: PathBuf,
    /// The folder name, used in case labels such as `shop/paid-short`.
    pub name: String,
    /// The manifest file.
    pub file: PathBuf,
    /// The manifest text.
    pub text: String,
    /// The parsed manifest.
    pub manifest: Manifest,
}

/// Reads the system in `dir`.
pub fn load_system(dir: &Path) -> Result<System> {
    let dir = absolute(dir)?;
    let file = dir.join("example.toml");
    if !file.is_file() {
        return Err(Error::new(
            ErrorCode::Usage,
            format!("{}: no example.toml", dir.display()),
        ));
    }
    let text = fs::read_to_string(&file).map_err(|e| Error::io(&file, &e))?;
    let manifest = parse_manifest(&text, &file.display().to_string())?;
    let name = dir
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    Ok(System {
        dir,
        name,
        file,
        text,
        manifest,
    })
}

/// The system folders under `<root>/examples`, sorted.
pub fn discover_systems(root: &Path) -> Result<Vec<PathBuf>> {
    let base = absolute(root)?.join("examples");
    let entries = fs::read_dir(&base).map_err(|e| Error::io(&base, &e))?;
    let mut dirs: Vec<PathBuf> = Vec::new();
    for entry in entries {
        let path = entry.map_err(|e| Error::io(&base, &e))?.path();
        if path.is_dir() && path.join("example.toml").is_file() {
            dirs.push(path);
        }
    }
    dirs.sort();
    Ok(dirs)
}

/// Reads every system under `root`.
pub fn load_all(root: &Path) -> Result<Vec<System>> {
    discover_systems(root)?
        .iter()
        .map(|d| load_system(d))
        .collect()
}

/// The structure file of a case.
pub fn expected_path(dir: &Path, id: &str) -> PathBuf {
    dir.join("expected").join(format!("{id}.json"))
}

/// The message snapshot of a case.
pub fn messages_path(dir: &Path, id: &str) -> PathBuf {
    dir.join("expected").join(format!("{id}.messages.txt"))
}

/// The recorded structure of a case, or `None` when it is missing.
pub fn read_expected(dir: &Path, id: &str) -> Result<Option<Value>> {
    let path = expected_path(dir, id);
    if !path.exists() {
        return Ok(None);
    }
    let text = fs::read_to_string(&path).map_err(|e| Error::io(&path, &e))?;
    serde_json::from_str(&text).map(Some).map_err(|e| {
        Error::new(
            ErrorCode::Recorded,
            format!("{}: {e}; run make record", path.display()),
        )
    })
}

/// The recorded message snapshot of a case; empty when there is none.
pub fn read_messages(dir: &Path, id: &str) -> Result<String> {
    let path = messages_path(dir, id);
    if !path.exists() {
        return Ok(String::new());
    }
    fs::read_to_string(&path).map_err(|e| Error::io(&path, &e))
}

/// Writes both record files of a case. The snapshot is removed when there is no message text.
pub fn write_recorded(dir: &Path, id: &str, recorded: &Recorded) -> Result<()> {
    let path = expected_path(dir, id);
    let text = serde_json::to_string_pretty(&recorded.structure)
        .map_err(|e| Error::new(ErrorCode::Recorded, format!("{}: {e}", path.display())))?;
    write(&path, &format!("{text}\n"))?;
    let snapshot = messages_path(dir, id);
    if recorded.messages.is_empty() {
        if snapshot.exists() {
            fs::remove_file(&snapshot).map_err(|e| Error::io(&snapshot, &e))?;
        }
        Ok(())
    } else {
        write(&snapshot, &recorded.messages)
    }
}

pub(crate) fn write(path: &Path, text: &str) -> Result<()> {
    fs::write(path, text).map_err(|e| Error::io(path, &e))
}

fn driver_args(case: &Case) -> Vec<String> {
    case.command
        .split(' ')
        .filter(|a| !a.is_empty())
        .map(str::to_owned)
        .chain(["--format".to_owned(), "json".to_owned()])
        .collect()
}

/// Runs one case.
pub fn run_case(system: &System, case: &Case, driver: &dyn Driver) -> Result<Run> {
    driver.run(&system.dir, &driver_args(case))
}

/// Which cases a check runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Selection {
    /// The running cases (`make check`).
    Running,
    /// The staged cases (`make staged`).
    Staged,
}

/// The counts of a check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Tally {
    /// Cases that passed.
    pub passed: usize,
    /// Cases that failed.
    pub failed: usize,
}

/// Runs every case of `selection` and prints one line per case and a summary line.
pub fn check_systems(
    systems: &[System],
    selection: Selection,
    driver: &dyn Driver,
    sink: &mut dyn Sink,
) -> Result<Tally> {
    let mut tally = Tally::default();
    for system in systems {
        for case in &system.manifest.cases {
            let selected = match selection {
                Selection::Running => matches!(case.status, Status::Running { .. }),
                Selection::Staged => matches!(case.status, Status::Staged { .. }),
            };
            if !selected {
                continue;
            }
            let run = run_case(system, case, driver)?;
            let expected = read_expected(&system.dir, &case.id)?;
            let label = format!("{}/{}", system.name, case.id);
            let evaluation = evaluate_case(case, &run, expected.as_ref(), &label);
            if evaluation.ok {
                tally.passed += 1;
                sink.info(&evaluation.message);
            } else {
                tally.failed += 1;
                sink.error(&evaluation.message);
            }
        }
    }
    let name = match selection {
        Selection::Running => "check",
        Selection::Staged => "staged",
    };
    sink.info(&format!(
        "{name}: {} passed, {} failed",
        tally.passed, tally.failed
    ));
    Ok(tally)
}

/// Records every case whose exit matches its manifest entry; returns how many were not recorded.
pub fn record_systems(
    systems: &[System],
    driver: &dyn Driver,
    sink: &mut dyn Sink,
) -> Result<usize> {
    let mut recorded = 0usize;
    let mut failed = 0usize;
    for system in systems {
        for case in &system.manifest.cases {
            let run = run_case(system, case, driver)?;
            let label = format!("{}/{}", system.name, case.id);
            if let Status::Staged { ready_exit, .. } = case.status
                && run.exit == ready_exit
            {
                failed += 1;
                sink.error(&format!("{label}: now exits {}: promote {label}", run.exit));
            } else if run.exit != case.expect_exit {
                failed += 1;
                sink.error(&format!(
                    "{label}: exit {}, manifest says {}; update the manifest, then record",
                    run.exit, case.expect_exit
                ));
            } else {
                write_recorded(&system.dir, &case.id, &project(&run.stdout))?;
                recorded += 1;
            }
        }
    }
    sink.info(&format!(
        "record: {recorded} recorded, {failed} not recorded"
    ));
    Ok(failed)
}

/// Finds a case by `<system>/<id>` or a bare `<id>`.
pub fn find_case<'a>(systems: &'a [System], reference: &str) -> Result<(&'a System, &'a Case)> {
    let (system_name, id) = match reference.split_once('/') {
        Some((s, i)) => (Some(s), i),
        None => (None, reference),
    };
    let hits: Vec<(&System, &Case)> = systems
        .iter()
        .filter(|s| system_name.is_none_or(|n| n == s.name))
        .flat_map(|s| s.manifest.cases.iter().map(move |c| (s, c)))
        .filter(|(_, c)| c.id == id)
        .collect();
    match hits.as_slice() {
        [] => Err(Error::new(ErrorCode::Usage, format!("no case {reference}"))),
        [one] => Ok(*one),
        _ => Err(Error::new(
            ErrorCode::Usage,
            format!("case {reference} is ambiguous; use <system>/<id>"),
        )),
    }
}

/// Flips a staged case that now gives its ready exit to running, and records its outcome.
pub fn promote_case(
    systems: &[System],
    reference: &str,
    driver: &dyn Driver,
    verdict: Option<&str>,
    sink: &mut dyn Sink,
) -> Result<()> {
    let (system, case) = find_case(systems, reference)?;
    let label = format!("{}/{}", system.name, case.id);
    let Status::Staged { ready_exit, .. } = case.status else {
        return Err(Error::new(
            ErrorCode::Usage,
            format!("{label} is not staged"),
        ));
    };
    let run = run_case(system, case, driver)?;
    if run.exit != ready_exit {
        return Err(Error::new(
            ErrorCode::Usage,
            format!(
                "{label} exits {}, not {ready_exit}; it is not ready to promote",
                run.exit
            ),
        ));
    }
    let recorded = project(&run.stdout);
    let verdict = verdict.map_or_else(|| verdict_of(&recorded.structure), str::to_owned);
    write(
        &system.file,
        &promote_text(&system.text, &case.id, &verdict)?,
    )?;
    write_recorded(&system.dir, &case.id, &recorded)?;
    sink.info(&format!(
        "promoted {label}: exit {}, verdict \"{verdict}\"",
        run.exit
    ));
    sink.info("Check the recorded output against what the case is meant to show, then commit.");
    Ok(())
}

/// A runner command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Running cases must match their record; README regions and `index.json` must be fresh.
    Check(Vec<PathBuf>),
    /// Staged cases must still fail as recorded.
    Staged(Vec<PathBuf>),
    /// Re-record `expected/` from the driver.
    Record(Vec<PathBuf>),
    /// Flip a staged case to running.
    Promote {
        /// `<system>/<id>` or a bare id.
        case: String,
        /// The verdict to write; the default is the verdict of the outcome.
        verdict: Option<String>,
    },
    /// Rewrite the README regions and `index.json`.
    Generate,
}

impl Command {
    /// Whether the command runs the driver.
    pub fn needs_driver(&self) -> bool {
        !matches!(self, Command::Generate)
    }
}

/// Runs a command for the repository at `root`. Returns whether it passed.
pub fn execute(
    command: &Command,
    root: &Path,
    driver: &dyn Driver,
    sink: &mut dyn Sink,
) -> Result<bool> {
    let load = |dirs: &[PathBuf]| -> Result<Vec<System>> {
        if dirs.is_empty() {
            load_all(root)
        } else {
            dirs.iter().map(|d| load_system(d)).collect()
        }
    };
    match command {
        Command::Generate => {
            sync_generated(&load_all(root)?, root, true, sink)?;
            Ok(true)
        }
        Command::Promote { case, verdict } => {
            promote_case(&load_all(root)?, case, driver, verdict.as_deref(), sink)?;
            sync_generated(&load_all(root)?, root, true, sink)?;
            Ok(true)
        }
        Command::Record(dirs) => {
            let systems = load(dirs)?;
            let failed = record_systems(&systems, driver, sink)?;
            if dirs.is_empty() {
                sync_generated(&systems, root, true, sink)?;
            }
            Ok(failed == 0)
        }
        Command::Check(dirs) | Command::Staged(dirs) => {
            let systems = load(dirs)?;
            let selection = if matches!(command, Command::Check(_)) {
                Selection::Running
            } else {
                Selection::Staged
            };
            let tally = check_systems(&systems, selection, driver, sink)?;
            let mut failed = tally.failed;
            if selection == Selection::Running && dirs.is_empty() {
                for problem in sync_generated(&systems, root, false, sink)? {
                    sink.error(&problem);
                    failed += 1;
                }
            }
            Ok(failed == 0)
        }
    }
}

/// `path` as an absolute path with `.` and `..` removed, without touching the file system.
pub fn absolute(path: &Path) -> Result<PathBuf> {
    let joined = std::path::absolute(path).map_err(|e| Error::io(path, &e))?;
    let mut out = PathBuf::new();
    for part in joined.components() {
        match part {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    Ok(out)
}
