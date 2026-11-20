//! The new scenario is prepared through the real external-provider executable.
use serde_json::{Value, json};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
#[ignore = "new EV-001 setup validation with configured current FSL and authoritative doc/grammar references"]
fn provider_prepares_original_requirements_and_references_without_leaking_its_answers() -> TestResult
{
    let workspace = tempfile::tempdir()?;
    let input = json!({"protocol":"cli-agent-evals.external-provider-request/v1","operation":"prepare","context":{"scenario":{"id":"EV-001"},"work_dir":workspace.path(),"cwd":workspace.path(),"session_id":"prepare-check","report_dir":workspace.path().join("private-report")}});
    let mut child = Command::new(env!("CARGO_BIN_EXE_agent-evals"))
        .arg("provider")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    child
        .stdin
        .take()
        .ok_or("provider stdin missing")?
        .write_all(input.to_string().as_bytes())?;
    let output = child.wait_with_output()?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let response: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(
        response["protocol"],
        "cli-agent-evals.external-provider-result/v1"
    );
    assert_eq!(response["operation"], "prepare");
    assert!(
        response["prompt"]
            .as_str()
            .is_some_and(|s| s.contains("StorageBin") && s.contains("<<<EVAL-COMPLETE>>>"))
    );
    assert!(workspace.path().join("requirements.md").is_file());
    assert!(!workspace.path().join("model.fsl").exists());
    assert!(!workspace.path().join("reference.fsl").exists());
    for (link, setting) in [
        ("docs/shipped", "FSL_DOCS_DIR"),
        ("docs/native-grammar.md", "FSL_NATIVE_GRAMMAR"),
    ] {
        let link = workspace.path().join(link);
        assert!(fs::symlink_metadata(&link)?.file_type().is_symlink());
        assert_eq!(
            fs::canonicalize(link)?,
            fs::canonicalize(std::env::var_os(setting).ok_or("reference setting missing")?)?
        );
    }
    let provider_folder = Path::new(env!("CARGO_BIN_EXE_agent-evals"))
        .parent()
        .ok_or("provider folder missing")?;
    let paths: Vec<PathBuf> = std::env::split_paths(
        response["environment"]["PATH"]
            .as_str()
            .ok_or("prepared PATH missing")?,
    )
    .collect();
    assert!(
        !paths.contains(&provider_folder.to_owned()),
        "private grader executable leaked onto agent PATH"
    );
    Ok(())
}
