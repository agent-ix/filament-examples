//! EV-002: repair a refused program using its source contract and refusal text.

use crate::{Assertion, ScenarioResult, Tools};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

pub const ID: &str = "EV-002";
pub const TITLE: &str = "Repair an Integer postage function using only its refusal text";
const BROKEN: &str = include_str!("../../fixtures/EV-002/postage.fsl");
const TASK: &str = include_str!("../../fixtures/EV-002/task.md");
// Independent, fixed expected values, not computed by the submitted program.
const BOUNDARIES: [(&str, &str); 8] = [
    ("-10000", "7"),
    ("-1", "7"),
    ("0", "7"),
    ("1", "10"),
    ("2", "13"),
    ("4", "19"),
    ("29", "94"),
    ("10000", "30007"),
];

pub fn prepare(workspace: &Path, tools: &Tools) -> ScenarioResult<String> {
    prepare_with_fsl(workspace, &tools.fsl)
}

fn prepare_with_fsl(workspace: &Path, fsl: &Path) -> ScenarioResult<String> {
    require_trusted_path(fsl)?;
    fs::create_dir_all(workspace)?;
    fs::write(workspace.join("postage.fsl"), BROKEN)?;
    let refusal = Command::new(fsl)
        .args(["check", "postage.fsl"])
        .current_dir(workspace)
        .output()?;
    if refusal.status.code() != Some(20) {
        return Err(format!(
            "harness-error: original postage fixture did not refuse with exit20: {}",
            String::from_utf8_lossy(&refusal.stdout)
        )
        .into());
    }
    let text = String::from_utf8(refusal.stdout)?;
    if !text.contains("type-mismatch") {
        return Err("harness-error: original fixture refusal is not a type mismatch".into());
    }
    // Preserve the tool's actual refusal. No stored message golden or SDK view
    // supplies information the agent could not obtain from the public CLI.
    fs::write(workspace.join("refusal.txt"), &text)?;
    Ok(format!(
        "{TASK}\nInitial public FSL refusal:\n\n```text\n{text}```\n"
    ))
}

pub fn grade(workspace: &Path, tools: &Tools) -> ScenarioResult<Assertion> {
    grade_with_fsl(workspace, &tools.fsl)
}

fn require_trusted_path(fsl: &Path) -> ScenarioResult<()> {
    if !fsl.is_absolute() {
        return Err("harness-error: grader requires the trusted absolute FSL path".into());
    }
    Ok(())
}

fn grade_with_fsl(workspace: &Path, fsl: &Path) -> ScenarioResult<Assertion> {
    require_trusted_path(fsl)?;
    let mut checks = BTreeMap::new();
    let mut failures = Vec::new();
    let source = workspace.join("postage.fsl");
    let regular_file = fs::symlink_metadata(&source)
        .map(|metadata| metadata.file_type().is_file())
        .unwrap_or(false);
    checks.insert("regular_source_file".into(), json!(regular_file));
    if !regular_file {
        failures.push("agent error: postage.fsl must be a regular source file".into());
        return Ok(Assertion {
            ok: false,
            checks,
            failures,
        });
    }
    let checked = Command::new(fsl)
        .args(["check", "postage.fsl", "--format", "json"])
        .current_dir(workspace)
        .output()?;
    let check_doc = read_outcome(&checked)?;
    let checked_ok = checked.status.success()
        && check_doc["format"] == "filament-outcome/1"
        && check_doc["category"] == "success";
    checks.insert(
        "public_check".into(),
        json!({"ok": checked_ok, "exit": checked.status.code(), "outcome": check_doc}),
    );
    if !checked_ok {
        failures.push("agent error: repaired source is refused by public fsl check".into());
        return Ok(Assertion {
            ok: false,
            checks,
            failures,
        });
    }

    // The grader writes its requests outside the agent's editable workspace.
    // Only the generated source is referenced: no agent-authored request, tool
    // wrapper, sentinel or self-reported output is an execution oracle.
    let requests = tempfile::tempdir()?;
    let source = fs::canonicalize(source)?;
    for (index, (units, expected)) in BOUNDARIES.iter().enumerate() {
        let request_path = requests.path().join(format!("boundary-{index}.json"));
        let request = json!({
            "format": "native-run/1",
            "request": {
                "models": [],
                "program": {"source": {
                    "file": source,
                    "authority": "agent-evals",
                    "identity": "EV-002-postage",
                    "document": "PostageRepair",
                    "formal_revision": 1
                }},
                "call": {"function": "postage", "arguments": [{"integer": units}]}
            }
        });
        fs::write(&request_path, serde_json::to_vec(&request)?)?;
        let output = Command::new(fsl)
            .arg("run")
            .arg(&request_path)
            .args(["--format", "json", "--limit", "accounting.work_units=10000"])
            .current_dir(workspace)
            .output()?;
        let outcome = read_outcome(&output)?;
        let ok = output.status.success()
            && outcome["format"] == "filament-outcome/1"
            && outcome["operation"] == "execute"
            && outcome["category"] == "success"
            && outcome["result"]["kind"] == "completed"
            && outcome["result"]["value"] == json!({"integer": expected});
        checks.insert(
            format!("postage({units})"),
            json!({
                "ok": ok, "expected": {"integer": expected},
                "exit": output.status.code(), "outcome": outcome
            }),
        );
        if !ok {
            failures.push(format!(
                "agent error: postage({units}) must return Integer {expected}"
            ));
        }
    }
    Ok(Assertion {
        ok: failures.is_empty(),
        checks,
        failures,
    })
}

fn read_outcome(output: &Output) -> ScenarioResult<Value> {
    serde_json::from_slice(&output.stdout).map_err(|error| {
        format!(
            "harness-error: public FSL did not write an outcome document ({error}); exit {:?}, stderr {}",
            output.status.code(), String::from_utf8_lossy(&output.stderr)
        )
        .into()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn binary() -> ScenarioResult<PathBuf> {
        let path = std::env::var_os("FSL").ok_or("set FSL for the new repair-grader lane")?;
        Ok(PathBuf::from(path))
    }

    fn candidate(body: &str) -> String {
        format!(
            "language \"ix:native\" edition \"1-draft\";\nprofile v = \"filament.value.complete/v1\";\nfunction postage using v(units: Integer): Integer pure {{ {body} }}\n"
        )
    }

    #[test]
    #[ignore = "QSL-2486 new repair-grader lane requires the accepted current-main FSL"]
    fn reference_repair_passes_all_boundary_executions() -> ScenarioResult<()> {
        let workspace = tempfile::tempdir()?;
        fs::write(
            workspace.path().join("postage.fsl"),
            candidate("if units > 0 then 7 + 3 * units else 7"),
        )?;
        let result = grade_with_fsl(workspace.path(), &binary()?)?;
        assert!(result.ok, "{:?}", result.failures);
        for (units, _) in BOUNDARIES {
            assert_eq!(result.checks[&format!("postage({units})")]["ok"], true);
        }
        Ok(())
    }

    #[test]
    #[ignore = "QSL-2486 new repair-grader lane requires the accepted current-main FSL"]
    fn unchanged_boolean_source_is_not_a_repair() -> ScenarioResult<()> {
        let workspace = tempfile::tempdir()?;
        let fsl = binary()?;
        let prompt = prepare_with_fsl(workspace.path(), &fsl)?;
        assert!(prompt.contains("type-mismatch") && prompt.contains("postage.fsl"));
        assert!(!workspace.path().join("docs").exists());
        let result = grade_with_fsl(workspace.path(), &fsl)?;
        assert!(!result.ok);
        assert_eq!(result.checks["public_check"]["ok"], false);
        Ok(())
    }

    #[test]
    #[ignore = "QSL-2486 new repair-grader lane requires the accepted current-main FSL"]
    fn constant_answer_and_success_marker_do_not_pass() -> ScenarioResult<()> {
        let workspace = tempfile::tempdir()?;
        fs::write(workspace.path().join("postage.fsl"), candidate("7"))?;
        fs::write(workspace.path().join("answer.txt"), "<<<EVAL-COMPLETE>>>")?;
        let result = grade_with_fsl(workspace.path(), &binary()?)?;
        assert!(!result.ok);
        assert_eq!(result.checks["public_check"]["ok"], true);
        assert_eq!(result.checks["postage(0)"]["ok"], true);
        assert_eq!(result.checks["postage(1)"]["ok"], false);
        Ok(())
    }

    #[test]
    #[ignore = "QSL-2486 new repair-grader lane requires the accepted current-main FSL"]
    fn changing_the_required_rate_is_not_a_repair() -> ScenarioResult<()> {
        let workspace = tempfile::tempdir()?;
        fs::write(
            workspace.path().join("postage.fsl"),
            candidate("if units > 0 then 7 + 2 * units else 7"),
        )?;
        let result = grade_with_fsl(workspace.path(), &binary()?)?;
        assert!(!result.ok);
        assert_eq!(result.checks["public_check"]["ok"], true);
        assert_eq!(result.checks["postage(-1)"]["ok"], true);
        assert_eq!(result.checks["postage(10000)"]["ok"], false);
        Ok(())
    }
}
