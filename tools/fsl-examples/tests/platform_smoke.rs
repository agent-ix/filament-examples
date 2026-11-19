//! Native-platform quickstart smoke using the existing real-process examples runner.
//!
//! Run this explicit lane with FSL and FSL_DRIVER_DIR naming one current-main
//! platform binary and its authoritative driver checkout. FSL_SMOKE_PLATFORM is
//! linux-x86_64 or macos-arm64. The host must actually report the selected OS and
//! architecture; one platform's pass never supplies evidence for the other.

use fsl_examples::runner::{Driver, ProcessDriver};
use serde_json::{Value, json};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn configured_path(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let value =
        std::env::var_os(name).ok_or_else(|| format!("set {name} for the native platform lane"))?;
    Ok(fs::canonicalize(value)?)
}

fn host_field(flag: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("/usr/bin/uname").arg(flag).output()?;
    if !output.status.success() {
        return Err(format!("native uname {flag} failed: {:?}", output.status).into());
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}

fn selected_native_platform() -> Result<String, Box<dyn std::error::Error>> {
    let selected = std::env::var("FSL_SMOKE_PLATFORM")?;
    let expected = match selected.as_str() {
        "linux-x86_64" => ("Linux", "x86_64"),
        "macos-arm64" => ("Darwin", "arm64"),
        _ => {
            return Err(format!(
                "unsupported smoke platform {selected}; select linux-x86_64 or macos-arm64"
            )
            .into());
        }
    };
    let actual = (host_field("-s")?, host_field("-m")?);
    if (actual.0.as_str(), actual.1.as_str()) != expected {
        return Err(format!("{selected} requires native {expected:?}, actual host is {actual:?}; this is not platform pass evidence").into());
    }
    Ok(selected)
}

fn example_blocks(readme: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let example = readme
        .split_once("### Example")
        .ok_or("current driver README has no Example quickstart")?
        .1;
    let section = example
        .split("\n## ")
        .next()
        .ok_or("empty Example section")?;
    let mut blocks = Vec::new();
    let mut current: Option<String> = None;
    for line in section.lines() {
        if line.starts_with("```") {
            match current.take() {
                Some(block) => blocks.push(block),
                None => current = Some(String::new()),
            }
        } else if let Some(block) = current.as_mut() {
            block.push_str(line);
            block.push('\n');
        }
    }
    if current.is_some() || blocks.len() < 3 {
        return Err("README quickstart must have source, request and command blocks".into());
    }
    Ok(blocks)
}

fn user_path(directory: &Path, name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let relative = Path::new(name);
    if relative.components().next().is_none()
        || !relative
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
    {
        return Err(format!(
            "README sample file must stay inside the user's temporary directory: {name}"
        )
        .into());
    }
    let path = directory.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(path)
}

#[test]
#[ignore = "QSL-2483 native platform lane: select an actual Linux x86_64 or macOS arm64 host and current-main fsl"]
fn quickstart_runs_on_the_selected_native_platform() -> TestResult {
    let platform = selected_native_platform()?;
    let binary = configured_path("FSL")?;
    let driver_checkout = configured_path("FSL_DRIVER_DIR")?;
    let readme = fs::read_to_string(driver_checkout.join("README.md"))?;
    let blocks = example_blocks(&readme)?;
    let request: Value = serde_json::from_str(&blocks[1])?;
    let source_name = request["request"]["program"]["source"]["file"]
        .as_str()
        .ok_or("README request has no source filename")?;
    let arguments: Vec<String> = blocks[2].split_whitespace().map(str::to_owned).collect();
    if arguments.len() < 3
        || arguments[0] != "fsl"
        || arguments[1] != "run"
        || !arguments
            .windows(2)
            .any(|pair| pair == ["--format", "json"])
    {
        return Err(format!(
            "README quickstart command must invoke fsl run <request> --format json: {}",
            blocks[2]
        )
        .into());
    }
    let temporary = tempfile::tempdir()?;
    fs::write(user_path(temporary.path(), source_name)?, &blocks[0])?;
    fs::write(user_path(temporary.path(), &arguments[2])?, &blocks[1])?;
    let process = ProcessDriver::new(&binary);
    let manifest: toml::Value = toml::from_str(&fs::read_to_string(
        driver_checkout.join("fsl-cli/Cargo.toml"),
    )?)?;
    let version = process.run(temporary.path(), &["--version".to_owned()])?;
    assert_eq!(version.exit, 0, "{platform}: version: {}", version.stdout);
    assert_eq!(
        Some(version.stdout.trim()),
        manifest["package"]["version"].as_str()
    );
    let run = process.run(temporary.path(), &arguments[1..])?;
    assert_eq!(run.exit, 0, "{platform}: README quickstart: {}", run.stdout);
    let outcome: Value = serde_json::from_str(&run.stdout)?;
    assert_eq!(
        outcome["format"], "filament-outcome/1",
        "{platform}: {outcome}"
    );
    assert_eq!(outcome["operation"], "execute", "{platform}: {outcome}");
    assert_eq!(outcome["category"], "success", "{platform}: {outcome}");
    assert_eq!(
        outcome["result"],
        json!({"kind": "completed", "value": {"boolean": true}}),
        "{platform}: {outcome}"
    );
    assert!(
        outcome["diagnostics"].as_array().is_some_and(Vec::is_empty),
        "{platform}: {outcome}"
    );
    eprintln!(
        "native platform {platform}: fsl {} at {} ran README quickstart with completed/true",
        version.stdout.trim(),
        binary.display()
    );
    Ok(())
}
