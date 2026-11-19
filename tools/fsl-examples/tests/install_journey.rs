//! The documented local source installer produces a usable fsl in an empty prefix.
//!
//! This explicit journey lane uses FSL_DRIVER_DIR for the authoritative driver checkout,
//! FSL_INSTALL_TARGET for its already-built target, and optional FSL_INSTALL_CONFIG for
//! the same build-only Cargo configuration. The debug profile reuses that current-main
//! build; installation still goes through Cargo, not a copied or wrapped executable.

use serde_json::{Value, json};
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn required_directory(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let value =
        std::env::var_os(name).ok_or_else(|| format!("set {name} for the install journey"))?;
    let path = fs::canonicalize(value)?;
    if !path.is_dir() {
        return Err(format!("{name}={} is not a directory", path.display()).into());
    }
    Ok(path)
}

fn local_install_command(readme: &str) -> Result<Vec<&str>, Box<dyn std::error::Error>> {
    let section = readme
        .split_once("To install from a local checkout instead:")
        .ok_or("driver README has no local-checkout installation instructions")?
        .1;
    let block = section
        .split_once("```")
        .and_then(|(_, rest)| rest.split_once("```"))
        .map(|(block, _)| block)
        .ok_or("driver README local installation command is not fenced")?;
    let command = block
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with("cargo install "))
        .ok_or("driver README local installation block contains no cargo install command")?;
    let args: Vec<_> = command.split_whitespace().collect();
    if !args.windows(2).any(|pair| pair == ["--path", "fsl-cli"])
        || !args.contains(&"--locked")
        || args
            .iter()
            .any(|arg| arg.contains([';', '|', '&', '$', '`']))
    {
        return Err(format!(
            "README local installer must install fsl-cli with locked dependencies: {command}"
        )
        .into());
    }
    Ok(args)
}

fn expect_exit(output: &Output, code: i32, step: &str) -> TestResult {
    if output.status.code() != Some(code) {
        return Err(format!(
            "{step}: expected exit {code}, got {:?}\nstdout:\n{}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(())
}

fn installed_command(binary: &Path, home: &Path, directory: &Path) -> Command {
    let mut command = Command::new(binary);
    command
        .env_clear()
        .env("HOME", home)
        .env("XDG_CONFIG_HOME", home.join("config"))
        .env("PATH", binary.parent().unwrap_or_else(|| Path::new("")))
        .current_dir(directory);
    command
}

fn outcome(
    output: &Output,
    operation: &str,
    category: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let value: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(value["format"], "filament-outcome/1", "{value}");
    assert_eq!(value["operation"], operation, "{value}");
    assert_eq!(value["category"], category, "{value}");
    Ok(value)
}

#[test]
#[ignore = "QSL-2479 native install journey: requires current driver checkout and its warm target"]
fn documented_local_install_runs_version_check_and_an_original_sample_in_a_clean_prefix()
-> TestResult {
    let driver = required_directory("FSL_DRIVER_DIR")?;
    let target = required_directory("FSL_INSTALL_TARGET")?;
    let readme = fs::read_to_string(driver.join("README.md"))?;
    let arguments = local_install_command(&readme)?;
    let cli_manifest: toml::Value =
        toml::from_str(&fs::read_to_string(driver.join("fsl-cli/Cargo.toml"))?)?;
    let expected_version = cli_manifest["package"]["version"]
        .as_str()
        .ok_or("driver fsl-cli manifest has no version")?;
    let clean = tempfile::tempdir()?;
    let prefix = clean.path().join("install");
    let home = clean.path().join("user");
    let work = clean.path().join("sample");
    fs::create_dir_all(&home)?;
    fs::create_dir_all(&work)?;
    assert!(
        !prefix.exists(),
        "the installation prefix must start absent"
    );

    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
    let mut install = Command::new(cargo);
    install
        .args(&arguments[1..])
        .args(["--debug", "--root"])
        .arg(&prefix)
        .env("CARGO_TARGET_DIR", &target)
        .env("CARGO_PROFILE_DEV_DEBUG", "0")
        .env("CARGO_PROFILE_TEST_DEBUG", "0")
        .env("CARGO_INCREMENTAL", "0")
        .env("CARGO_BUILD_JOBS", "2")
        .current_dir(&driver);
    if let Some(config) = std::env::var_os("FSL_INSTALL_CONFIG") {
        install.arg("--config").arg(config);
    }
    let installed = install.output()?;
    expect_exit(&installed, 0, "README cargo install into an empty prefix")?;
    let binary = prefix
        .join("bin")
        .join(format!("fsl{}", std::env::consts::EXE_SUFFIX));
    assert!(
        binary.is_file(),
        "installer did not produce {}",
        binary.display()
    );
    let version = installed_command(&binary, &home, &work)
        .arg("--version")
        .output()?;
    expect_exit(&version, 0, "installed fsl --version")?;
    assert_eq!(String::from_utf8(version.stdout)?.trim(), expected_version);
    assert!(
        version.stderr.is_empty(),
        "version wrote an unexpected diagnostic"
    );

    let source = "language \"ix:native\" edition \"1-draft\";\nprofile v = \"filament.value.complete/v1\";\nfunction installed(): Integer pure { 7 * 6 }\n";
    fs::write(work.join("sample.fsl"), source)?;
    let checked = installed_command(&binary, &home, &work)
        .args([
            "check",
            "example",
            "installed",
            "sample.fsl",
            "--format",
            "json",
        ])
        .output()?;
    expect_exit(
        &checked,
        0,
        "installed fsl checks an original model-free program",
    )?;
    let checked = outcome(&checked, "check", "success")?;
    assert!(
        checked["diagnostics"].as_array().is_some_and(Vec::is_empty),
        "{checked}"
    );
    let request = json!({"format": "native-run/1", "request": {
        "models": [],
        "program": {"source": {"file": "sample.fsl", "authority": "example",
            "identity": "installed", "document": "InstalledSample", "formal_revision": 1}},
        "call": {"function": "installed", "arguments": []}
    }});
    fs::write(work.join("run.json"), request.to_string())?;
    let run = installed_command(&binary, &home, &work)
        .args(["run", "run.json", "--format", "json"])
        .output()?;
    expect_exit(
        &run,
        0,
        "installed fsl runs the original sample without ambient configuration",
    )?;
    let run = outcome(&run, "execute", "success")?;
    assert_eq!(
        run["result"],
        json!({"kind": "completed", "value": {"integer": "42"}}),
        "{run}"
    );

    fs::write(
        work.join("broken.fsl"),
        "language \"ix:native\" edition \"1-draft\";\nfunction broken(): Integer pure { }\n",
    )?;
    let broken = installed_command(&binary, &home, &work)
        .args([
            "check",
            "example",
            "broken",
            "broken.fsl",
            "--format",
            "json",
        ])
        .output()?;
    expect_exit(
        &broken,
        20,
        "installed checker must reject a missing expression",
    )?;
    let broken = outcome(&broken, "check", "refusal")?;
    let diagnostics = broken["diagnostics"]
        .as_array()
        .ok_or("refusal has no diagnostics")?;
    assert!(
        diagnostics
            .iter()
            .any(|d| d["code"] == "invalid_syntax" && d["cause"] == "unexpected-token"),
        "{broken}"
    );
    Ok(())
}
