// SPDX-License-Identifier: MIT
//! The driver README's complete Example runs against a caller-selected current-main CLI.
//!
//! Run the live lane with `FSL` naming that CLI and `FSL_DRIVER_DIR` naming its
//! authoritative driver checkout:
//! `cargo test --manifest-path tools/fsl-examples/Cargo.toml --test readme_quickstart -- --ignored --test-threads=1`.

use std::error::Error;
use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};

use fsl_examples::runner::{Driver, ProcessDriver};
use serde_json::{Value, json};

struct Example<'a> {
    source_name: &'a str,
    source: &'a str,
    request_name: &'a str,
    request: &'a str,
    command: &'a str,
}

fn missing(member: &str) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("driver README Example lacks {member}"),
    )
}

/// One fenced block and the prose before and after it, retaining the code bytes.
fn fence(text: &str) -> io::Result<(&str, &str, &str)> {
    let (prose, opening) = text.split_once("```").ok_or_else(|| missing("a fence"))?;
    let (_, code) = opening
        .split_once('\n')
        .ok_or_else(|| missing("a fenced block opening line"))?;
    let closing = code
        .find("\n```")
        .ok_or_else(|| missing("a closing fence"))?;
    Ok((prose, &code[..closing + 1], &code[closing + 4..]))
}

fn example(readme: &str) -> io::Result<Example<'_>> {
    let (_, section) = readme
        .split_once("### Example")
        .ok_or_else(|| missing("its Example section"))?;
    let (_, source, after_source) = fence(section)?;
    let (save_instructions, request, after_request) = fence(after_source)?;
    let (_, command, _) = fence(after_request)?;
    // These are the two names the reader is told to save, in the prose
    // between the source and request. Taking names from the request itself
    // would silently repair a broken save instruction.
    let mut names = save_instructions.split('`').skip(1).step_by(2);
    let source_name = names.next().ok_or_else(|| missing("a source filename"))?;
    let request_name = names.next().ok_or_else(|| missing("a request filename"))?;
    Ok(Example {
        source_name,
        source,
        request_name,
        request,
        command: command.trim(),
    })
}

fn write_user_file(directory: &Path, filename: &str, bytes: &[u8]) -> io::Result<()> {
    let relative = Path::new(filename);
    if relative
        .components()
        .any(|part| !matches!(part, Component::Normal(_) | Component::CurDir))
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("README user file must stay in its working directory: {filename}"),
        ));
    }
    let path = directory.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, bytes)
}

/// Provenance: QSL-2480.
#[test]
#[ignore = "QSL-2480 live README journey: requires current-main FSL and authoritative FSL_DRIVER_DIR"]
fn readme_example_saved_files_and_verbatim_run_return_the_documented_true_value()
-> Result<(), Box<dyn Error>> {
    let binary = std::env::var_os("FSL").ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "set FSL to the current-main CLI",
        )
    })?;
    let driver = std::env::var_os("FSL_DRIVER_DIR").ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "set FSL_DRIVER_DIR to the authoritative driver checkout",
        )
    })?;
    let readme = fs::read_to_string(PathBuf::from(driver).join("README.md"))?;
    let example = example(&readme)?;
    let workspace = tempfile::tempdir()?;
    write_user_file(
        workspace.path(),
        example.source_name,
        example.source.as_bytes(),
    )?;
    write_user_file(
        workspace.path(),
        example.request_name,
        example.request.as_bytes(),
    )?;

    let mut words = example.command.split_ascii_whitespace();
    assert_eq!(
        words.next(),
        Some("fsl"),
        "README command: {}",
        example.command
    );
    let arguments: Vec<String> = words.map(str::to_owned).collect();
    let result = ProcessDriver::new(PathBuf::from(binary)).run(workspace.path(), &arguments)?;
    assert_eq!(
        result.exit, 0,
        "README command {:?} returned {}: {}",
        example.command, result.exit, result.stdout
    );
    let document: Value = serde_json::from_str(&result.stdout)?;
    assert_eq!(document["operation"], "execute", "{document}");
    assert_eq!(document["category"], "success", "{document}");
    assert_eq!(document["result"]["kind"], "completed", "{document}");
    assert_eq!(
        document["result"]["value"],
        json!({"boolean": true}),
        "{document}"
    );
    Ok(())
}
