//! EV-006: declare a protocol's intentional terminal state and analyze deadlock freedom.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use crate::capture::{CaptureCode, CaptureError, before_deadline, capture_command};
use crate::{Assertion, ScenarioResult, Tools};

/// Stable scenario identifier in ticket order.
pub const ID: &str = "EV-006";
/// Human-readable catalogue title.
pub const TITLE: &str = "Declare intended protocol termination and prove deadlock freedom";

const MODEL: &str = include_str!("../../fixtures/EV-006/cell.semantic-ir.json");
const START: &str = include_str!("../../fixtures/EV-006/start.json");
const DEADLOCK_REQUEST: &str = include_str!("../../fixtures/EV-006/deadlock-request.json");
const REPAIRED_REQUEST: &str = include_str!("../../fixtures/EV-006/repaired-request.json");
const TASK: &str = include_str!("../../fixtures/EV-006/task.md");
const ASSERT_BUDGET: Duration = Duration::from_secs(30);
const FILE_LIMIT: u64 = 1_048_576;

#[derive(Clone, Copy, Debug)]
enum FaultCode {
    InvalidArtifact,
    ChangedInput,
    CliGap,
    WrongDeadlock,
    WrongProof,
    WrongSummary,
    ToolFailure,
    Timeout,
}

#[derive(Debug)]
struct Fault {
    code: FaultCode,
    detail: String,
}

impl Fault {
    fn new(code: FaultCode, detail: impl Into<String>) -> Self {
        Self {
            code,
            detail: detail.into(),
        }
    }
}

impl std::fmt::Display for Fault {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(out, "{}", self.detail)
    }
}
impl std::error::Error for Fault {}
type Checked<T> = Result<T, Fault>;

/// Write only the fixed model and analysis requests into the sample workspace.
pub fn prepare(workspace: &Path, _tools: &Tools) -> ScenarioResult<String> {
    fs::create_dir_all(workspace)?;
    for (name, contents) in [
        ("cell.semantic-ir.json", MODEL),
        ("start.json", START),
        ("deadlock-request.json", DEADLOCK_REQUEST),
        ("repaired-request.json", REPAIRED_REQUEST),
    ] {
        fs::write(workspace.join(name), contents)?;
    }
    Ok(TASK.to_owned())
}

fn read_file(workspace: &Path, name: &str) -> Checked<String> {
    let path = workspace.join(name);
    let metadata = fs::symlink_metadata(&path)
        .map_err(|_| Fault::new(FaultCode::InvalidArtifact, format!("missing {name}")))?;
    if !metadata.file_type().is_file() || metadata.len() > FILE_LIMIT {
        return Err(Fault::new(
            FaultCode::InvalidArtifact,
            format!("{name} must be a bounded regular file"),
        ));
    }
    fs::read_to_string(path)
        .map_err(|_| Fault::new(FaultCode::InvalidArtifact, format!("{name} must be UTF-8")))
}

fn json_file(workspace: &Path, name: &str) -> Checked<Value> {
    serde_json::from_str(&read_file(workspace, name)?)
        .map_err(|_| Fault::new(FaultCode::InvalidArtifact, format!("{name} must be JSON")))
}

fn check_inputs(workspace: &Path) -> Checked<()> {
    for (name, contents) in [
        ("cell.semantic-ir.json", MODEL),
        ("start.json", START),
        ("deadlock-request.json", DEADLOCK_REQUEST),
        ("repaired-request.json", REPAIRED_REQUEST),
    ] {
        if read_file(workspace, name)? != contents {
            return Err(Fault::new(
                FaultCode::ChangedInput,
                format!("the fixed {name} input was changed"),
            ));
        }
    }
    read_file(workspace, "baseline.native")?;
    read_file(workspace, "protocol.native")?;
    Ok(())
}

fn command_failure(error: CaptureError) -> Fault {
    let code = match error.code {
        CaptureCode::AssertionTimeout => FaultCode::Timeout,
        CaptureCode::OutputLimit | CaptureCode::ToolFailure => FaultCode::ToolFailure,
    };
    Fault::new(code, error.to_string())
}

fn analyze(
    workspace: &Path,
    tools: &Tools,
    request: &str,
    expected_exit: i32,
    deadline: Instant,
) -> Checked<Value> {
    before_deadline(deadline).map_err(command_failure)?;
    let mut command = Command::new(&tools.fsl);
    command
        .current_dir(workspace)
        .args(["analyze", request, "--format", "json"]);
    let output = capture_command(&mut command, deadline).map_err(command_failure)?;
    if output.status.code().is_none() {
        return Err(Fault::new(
            FaultCode::ToolFailure,
            format!("fsl analyze {request} terminated without an exit code"),
        ));
    }
    let document: Value = serde_json::from_slice(&output.stdout).map_err(|_| {
        Fault::new(
            FaultCode::ToolFailure,
            format!("fsl analyze {request} did not emit a JSON outcome"),
        )
    })?;
    if output.status.code() != Some(expected_exit) {
        if is_cli_gap(&document) {
            return Err(Fault::new(
                FaultCode::CliGap,
                format!("fsl analyze {request} returned an unsupported or incomplete outcome"),
            ));
        }
        return Err(Fault::new(
            if expected_exit == 10 {
                FaultCode::WrongDeadlock
            } else {
                FaultCode::WrongProof
            },
            format!("fsl analyze {request} did not produce the required outcome"),
        ));
    }
    if is_cli_gap(&document) {
        return Err(Fault::new(
            FaultCode::CliGap,
            format!("fsl analyze {request} returned an unsupported outcome"),
        ));
    }
    if expected_exit == 10 {
        check_deadlock(&document)?;
    } else {
        check_proof(&document)?;
    }
    Ok(document)
}

fn is_cli_gap(document: &Value) -> bool {
    document["category"] == "unsupported"
        || document["category"] == "incomplete"
        || document["items"].as_array().is_some_and(|items| {
            items
                .iter()
                .any(|item| item["record"]["result"] == "unsupported")
        })
}

fn one_item(document: &Value) -> Checked<&Value> {
    document["items"]
        .as_array()
        .filter(|items| items.len() == 1)
        .map(|items| &items[0])
        .ok_or_else(|| Fault::new(FaultCode::WrongDeadlock, "expected one analysis item"))
}

fn check_deadlock(document: &Value) -> Checked<&Value> {
    let item = one_item(document)?;
    if document["format"] != "filament-outcome/1"
        || document["category"] != "violation"
        || item["category"] != "violation"
        || item["claim_name"] != "Fill"
        || item["record"]["result"] != "refuted"
    {
        return Err(Fault::new(
            FaultCode::WrongDeadlock,
            "the baseline must settle the deadlock-freedom item as refuted",
        ));
    }
    let counterexample = &item["counterexample"];
    let Some(prefix) = counterexample["prefix"].as_array() else {
        return Err(Fault::new(
            FaultCode::WrongDeadlock,
            "counterexample prefix is missing",
        ));
    };
    let Some(blocked) = counterexample["blocked"].as_array() else {
        return Err(Fault::new(
            FaultCode::WrongDeadlock,
            "blocked thread list is missing",
        ));
    };
    let expected_right_thread = json!({
        "instance": "0",
        "steps": [{"branch": "right", "node": "Main/Both", "ordinal": "0"}],
        "type": "thread-path"
    });
    let expected_join_waiters = json!([expected_right_thread]);
    if counterexample["kind"] != "deadlock"
        || prefix.len() != 2
        || blocked.len() != 2
        || prefix[0]["transition"]["step"] != "fork"
        || prefix[0]["transition"]["node"] != "Main/Both"
        || prefix[1]["transition"]["step"] != "attempt"
        || prefix[1]["transition"]["node"] != "Main/Both/left/A"
        || !blocked.iter().any(|entry| {
            entry["cause"]["type"] == "precondition"
                && entry["cause"]["clause"] == "TwoPre"
                && entry["rest"]["node"] == "Main/Both/right/B"
                && entry["thread"] == expected_right_thread
        })
        || !blocked.iter().any(|entry| {
            entry["cause"]["type"] == "join-waiting"
                && entry["cause"]["threads"] == expected_join_waiters
                && entry["rest"]["node"] == "Main/Both"
                && entry["thread"] == json!({"instance":"0","steps":[],"type":"thread-path"})
        })
    {
        return Err(Fault::new(
            FaultCode::WrongDeadlock,
            "the counterexample must show fork(Both), attempt(A), TwoPre and JoinWaiting",
        ));
    }
    Ok(item)
}

fn check_proof(document: &Value) -> Checked<&Value> {
    let item = one_item(document)?;
    if document["format"] != "filament-outcome/1"
        || document["category"] != "success"
        || item["category"] != "success"
        || item["claim_name"] != "Fill"
        || item["record"]["result"] != "proved"
    {
        return Err(Fault::new(
            FaultCode::WrongProof,
            "the repaired deadlock-freedom item must settle as proved",
        ));
    }
    if item["record"]["basis"]["type"] != "exhaustive"
        || item["record"]["certification"] != "certified"
    {
        return Err(Fault::new(
            FaultCode::WrongProof,
            "the repaired protocol needs a certified exhaustive proof",
        ));
    }
    Ok(item)
}

fn compare_items(saved: &Value, trusted: &Value, repaired: bool) -> Checked<()> {
    let saved_item = if repaired {
        check_proof(saved)?
    } else {
        check_deadlock(saved)?
    };
    let trusted_item = if repaired {
        check_proof(trusted)?
    } else {
        check_deadlock(trusted)?
    };
    for field in [
        "claim_name",
        "category",
        "request_index",
        "request",
        "record",
        "counterexample",
        "certificate",
    ] {
        if saved_item[field] != trusted_item[field] {
            return Err(Fault::new(
                if repaired {
                    FaultCode::WrongProof
                } else {
                    FaultCode::WrongDeadlock
                },
                format!("saved analysis field {field} differs from trusted fsl analyze"),
            ));
        }
    }
    Ok(())
}

fn check_summary(workspace: &Path, deadlock: &Value, repaired: &Value) -> Checked<()> {
    let actual = json_file(workspace, "result.json")?;
    let deadlock_item = check_deadlock(deadlock)?;
    let repaired_item = check_proof(repaired)?;
    let counterexample = &deadlock_item["counterexample"];
    let prefix = counterexample["prefix"]
        .as_array()
        .ok_or_else(|| Fault::new(FaultCode::WrongDeadlock, "counterexample prefix is missing"))?
        .iter()
        .map(|step| {
            let transition = &step["transition"];
            json!({
                "step": transition.get("step").cloned().unwrap_or(Value::Null),
                "node": transition.get("node").cloned().unwrap_or(Value::Null),
            })
        })
        .collect::<Vec<_>>();
    let blocked = counterexample["blocked"]
        .as_array()
        .ok_or_else(|| Fault::new(FaultCode::WrongDeadlock, "blocked threads are missing"))?
        .iter()
        .map(|entry| {
            json!({
                "thread": entry.get("thread").cloned().unwrap_or(Value::Null),
                "rest": entry.get("rest").cloned().unwrap_or(Value::Null),
                "cause": entry.get("cause").cloned().unwrap_or(Value::Null),
            })
        })
        .collect::<Vec<_>>();
    let expected = json!({
        "deadlock": {
            "exit_code": 10,
            "result": "refuted",
            "prefix": prefix,
            "blocked": blocked,
        },
        "repaired": {
            "exit_code": 0,
            "result": "proved",
            "basis": repaired_item["record"]["basis"],
            "certification": repaired_item["record"]["certification"],
        }
    });
    if !summary_matches(&actual, &expected) {
        return Err(Fault::new(
            FaultCode::WrongSummary,
            "result.json must summarize the structured CLI output",
        ));
    }
    Ok(())
}

fn summary_matches(actual: &Value, expected: &Value) -> bool {
    if actual["deadlock"]["exit_code"] != expected["deadlock"]["exit_code"]
        || actual["deadlock"]["result"] != expected["deadlock"]["result"]
        || actual["deadlock"]["prefix"] != expected["deadlock"]["prefix"]
        || !unordered_json_eq(
            &actual["deadlock"]["blocked"],
            &expected["deadlock"]["blocked"],
        )
    {
        return false;
    }
    ["exit_code", "result", "basis", "certification"]
        .iter()
        .all(|field| actual["repaired"][field] == expected["repaired"][field])
}

fn unordered_json_eq(actual: &Value, expected: &Value) -> bool {
    let (Some(actual), Some(expected)) = (actual.as_array(), expected.as_array()) else {
        return false;
    };
    if actual.len() != expected.len() {
        return false;
    }
    let mut unmatched = expected.to_vec();
    for entry in actual {
        let Some(index) = unmatched.iter().position(|candidate| candidate == entry) else {
            return false;
        };
        unmatched.remove(index);
    }
    unmatched.is_empty()
}

fn assess(workspace: &Path, tools: &Tools, deadline: Instant) -> Checked<BTreeMap<String, Value>> {
    before_deadline(deadline).map_err(command_failure)?;
    check_inputs(workspace)?;
    let saved_deadlock = json_file(workspace, "deadlock.json")?;
    let saved_repaired = json_file(workspace, "repaired.json")?;
    check_deadlock(&saved_deadlock)?;
    check_proof(&saved_repaired)?;
    let trusted_deadlock = analyze(workspace, tools, "deadlock-request.json", 10, deadline)?;
    let trusted_repaired = analyze(workspace, tools, "repaired-request.json", 0, deadline)?;
    compare_items(&saved_deadlock, &trusted_deadlock, false)?;
    compare_items(&saved_repaired, &trusted_repaired, true)?;
    check_summary(workspace, &trusted_deadlock, &trusted_repaired)?;
    before_deadline(deadline).map_err(command_failure)?;
    Ok(BTreeMap::from([
        ("modeled_protocol_and_repair_match".into(), json!(true)),
        (
            "baseline_deadlock_matches_independent_analysis".into(),
            json!(true),
        ),
        (
            "repaired_exhaustive_proof_matches_independent_analysis".into(),
            json!(true),
        ),
    ]))
}

/// Independently verify the baseline deadlock and repaired terminal-state proof.
pub fn grade(workspace: &Path, tools: &Tools) -> ScenarioResult<Assertion> {
    let deadline = Instant::now() + ASSERT_BUDGET;
    match assess(workspace, tools, deadline) {
        Ok(checks) => Ok(Assertion {
            ok: true,
            checks,
            failures: Vec::new(),
        }),
        Err(fault) => Ok(Assertion {
            ok: false,
            checks: BTreeMap::new(),
            failures: vec![match fault.code {
                FaultCode::ToolFailure | FaultCode::Timeout => format!("harness-error: {fault}"),
                FaultCode::CliGap => format!("cli-gap: {fault}"),
                _ => format!("{fault}"),
            }],
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    use tempfile::tempdir;

    const DEADLOCK_OUTCOME: &str =
        include_str!("../../fixtures/EV-006/reference/deadlock-outcome.json");
    const TERMINAL_OUTCOME: &str =
        include_str!("../../fixtures/EV-006/reference/repaired-outcome.json");
    const PRECONDITION_OUTCOME: &str =
        include_str!("../../fixtures/EV-006/reference/precondition-outcome.json");
    const RESULT_SUMMARY: &str =
        include_str!("../../fixtures/EV-006/reference/result-summary.json");

    fn deadlock_result() -> Value {
        json!({"format":"filament-outcome/1","category":"violation","items":[{
            "category":"violation","claim_name":"Fill","record":{"result":"refuted"},
            "counterexample":{"kind":"deadlock","prefix":[
                {"transition":{"step":"fork","node":"Main/Both"}},
                {"transition":{"step":"attempt","node":"Main/Both/left/A"}}],
                "blocked":[
                    {"thread":{"instance":"0","steps":[{"branch":"right","node":"Main/Both","ordinal":"0"}],"type":"thread-path"},
                     "rest":{"kind":"node","node":"Main/Both/right/B","type":"rest"},
                     "cause":{"type":"precondition","clause":"TwoPre"}},
                    {"thread":{"instance":"0","steps":[],"type":"thread-path"},
                     "rest":{"kind":"join","node":"Main/Both","type":"rest"},
                     "cause":{"type":"join-waiting","threads":[{"instance":"0","steps":[{"branch":"right","node":"Main/Both","ordinal":"0"}],"type":"thread-path"}]}}
                ]}
        }]})
    }

    fn proof_result() -> Value {
        json!({"format":"filament-outcome/1","category":"success","items":[{
            "category":"success","claim_name":"Fill","record":{"result":"proved",
                "basis":{"type":"exhaustive"},"certification":"certified"},
            "certificate":{"accepted":true}
        }]})
    }

    fn write_stub(path: &Path, body: &str) -> ScenarioResult<()> {
        fs::write(path, body)?;
        let mut permissions = fs::metadata(path)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions)?;
        Ok(())
    }

    #[test]
    fn source_check_does_not_require_a_hidden_reference_implementation() -> ScenarioResult<()> {
        let temp = tempdir()?;
        for (name, contents) in [
            ("cell.semantic-ir.json", MODEL),
            ("start.json", START),
            ("deadlock-request.json", DEADLOCK_REQUEST),
            ("repaired-request.json", REPAIRED_REQUEST),
            (
                "baseline.native",
                "English-derived source with arbitrary names",
            ),
            (
                "protocol.native",
                "Equivalent repaired protocol with another spelling",
            ),
        ] {
            fs::write(temp.path().join(name), contents)?;
        }
        assert!(check_inputs(temp.path()).is_ok());
        fs::write(temp.path().join("start.json"), "changed")?;
        assert!(matches!(
            check_inputs(temp.path()),
            Err(Fault {
                code: FaultCode::ChangedInput,
                ..
            })
        ));
        Ok(())
    }

    #[test]
    fn deadlock_requires_the_expected_protocol_prefix_and_both_thread_paths() {
        let deadlock = deadlock_result();
        assert!(check_deadlock(&deadlock).is_ok());
        let mut missing_wait = deadlock.clone();
        missing_wait["items"][0]["counterexample"]["blocked"] = json!([]);
        assert!(check_deadlock(&missing_wait).is_err());
        let mut substring_trap = deadlock.clone();
        substring_trap["items"][0]["counterexample"]["blocked"][0]["cause"] =
            json!({"type":"precondition","clause":"Both"});
        assert!(check_deadlock(&substring_trap).is_err());
        let mut wrong_path = deadlock;
        wrong_path["items"][0]["counterexample"]["blocked"][0]["thread"]["steps"][0]["branch"] =
            json!("left");
        assert!(check_deadlock(&wrong_path).is_err());
    }

    #[test]
    fn repaired_result_requires_certified_exhaustive_proof() {
        let proved = proof_result();
        assert!(check_proof(&proved).is_ok());
        let mut uncertified = proved;
        uncertified["items"][0]["record"]["certification"] = json!("uncertified");
        assert!(check_proof(&uncertified).is_err());
    }

    #[test]
    fn summary_uses_cli_structures_and_ignores_blocked_order() -> ScenarioResult<()> {
        let temp = tempdir()?;
        let deadlock: Value = serde_json::from_str(DEADLOCK_OUTCOME)?;
        let repaired: Value = serde_json::from_str(TERMINAL_OUTCOME)?;
        fs::write(temp.path().join("result.json"), RESULT_SUMMARY)?;
        assert!(check_summary(temp.path(), &deadlock, &repaired).is_ok());

        let mut reordered: Value = serde_json::from_str(RESULT_SUMMARY)?;
        reordered["deadlock"]["blocked"]
            .as_array_mut()
            .unwrap()
            .reverse();
        fs::write(
            temp.path().join("result.json"),
            serde_json::to_vec_pretty(&reordered)?,
        )?;
        assert!(check_summary(temp.path(), &deadlock, &repaired).is_ok());

        reordered["deadlock"]["prefix"]
            .as_array_mut()
            .unwrap()
            .reverse();
        fs::write(
            temp.path().join("result.json"),
            serde_json::to_vec_pretty(&reordered)?,
        )?;
        assert!(check_summary(temp.path(), &deadlock, &repaired).is_err());
        Ok(())
    }

    #[test]
    fn compare_items_checks_trusted_fields() {
        let deadlock = deadlock_result();
        assert!(compare_items(&deadlock, &deadlock, false).is_ok());
        let mut changed = deadlock.clone();
        changed["items"][0]["claim_name"] = json!("Other");
        assert!(compare_items(&deadlock, &changed, false).is_err());
        let proof = proof_result();
        assert!(compare_items(&proof, &proof, true).is_ok());
        let mut changed_proof = proof.clone();
        changed_proof["items"][0]["certificate"]["accepted"] = json!(false);
        assert!(compare_items(&proof, &changed_proof, true).is_err());
    }

    #[test]
    fn full_grade_accepts_both_repairs_and_separates_agent_cli_and_harness_failures()
    -> ScenarioResult<()> {
        let precondition: Value = serde_json::from_str(PRECONDITION_OUTCOME)?;
        assert!(check_proof(&precondition).is_ok());
        let workspace = tempdir()?;
        let stub_dir = tempdir()?;
        let reference = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/EV-006/reference");
        prepare(
            workspace.path(),
            &Tools {
                fsl: Path::new("fsl").into(),
                docs: Path::new(".").into(),
                quire: Path::new("quire").into(),
                quoin: Path::new("quoin").into(),
            },
        )?;
        fs::write(
            workspace.path().join("baseline.native"),
            "agent-authored baseline",
        )?;
        fs::write(
            workspace.path().join("protocol.native"),
            "terminal repair with a different source layout",
        )?;
        fs::copy(
            reference.join("deadlock-outcome.json"),
            workspace.path().join("deadlock.json"),
        )?;
        fs::copy(
            reference.join("repaired-outcome.json"),
            workspace.path().join("repaired.json"),
        )?;
        fs::copy(
            reference.join("result-summary.json"),
            workspace.path().join("result.json"),
        )?;

        let fsl = stub_dir.path().join("fsl");
        let script = |repaired: &str| {
            format!(
                "#!/bin/sh\ncase \"$2\" in\n deadlock-request.json) cat '{}'; exit 10 ;;\n repaired-request.json) cat '{}'; exit 0 ;;\n *) exit 21 ;;\nesac\n",
                reference.join("deadlock-outcome.json").display(),
                reference.join(repaired).display()
            )
        };
        write_stub(&fsl, &script("repaired-outcome.json"))?;
        let tools = Tools {
            fsl: fsl.clone(),
            docs: Path::new(".").into(),
            quire: Path::new("quire").into(),
            quoin: Path::new("quoin").into(),
        };
        let assertion = grade(workspace.path(), &tools)?;
        assert!(assertion.ok, "{:?}", assertion.failures);

        fs::write(
            workspace.path().join("protocol.native"),
            "precondition repair at a different declaration position",
        )?;
        write_stub(&fsl, &script("precondition-outcome.json"))?;
        let assertion = grade(workspace.path(), &tools)?;
        assert!(assertion.ok, "{:?}", assertion.failures);

        fs::write(workspace.path().join("deadlock.json"), r#"{"items":[]}"#)?;
        let assertion = grade(workspace.path(), &tools)?;
        assert!(!assertion.ok);
        assert!(!assertion.failures[0].starts_with("harness-error:"));
        assert!(!assertion.failures[0].starts_with("cli-gap:"));

        fs::copy(
            reference.join("deadlock-outcome.json"),
            workspace.path().join("deadlock.json"),
        )?;
        write_stub(
            &fsl,
            "#!/bin/sh\nprintf '%s\\n' '{\"category\":\"unsupported\",\"items\":[]}'\nexit 10\n",
        )?;
        let assertion = grade(workspace.path(), &tools)?;
        assert!(assertion.failures[0].starts_with("cli-gap:"));

        write_stub(&fsl, "#!/bin/sh\nexit 127\n")?;
        let assertion = grade(workspace.path(), &tools)?;
        assert!(assertion.failures[0].starts_with("harness-error:"));

        write_stub(&fsl, "#!/bin/sh\nkill -TERM $$\n")?;
        let assertion = grade(workspace.path(), &tools)?;
        assert!(assertion.failures[0].starts_with("harness-error:"));
        Ok(())
    }
}
