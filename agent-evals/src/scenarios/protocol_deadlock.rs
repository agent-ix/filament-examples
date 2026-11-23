//! EV-006: declare a protocol's intentional terminal state and analyze deadlock freedom.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

use serde::Deserialize;
use serde_json::{Value, json};

use crate::capture::{CaptureCode, CaptureError, before_deadline, capture_command};
use crate::{Assertion, ScenarioResult, Tools};

/// Stable scenario identifier in ticket order.
pub const ID: &str = "EV-006";
/// Human-readable catalogue title.
pub const TITLE: &str = "Declare intended protocol termination and prove deadlock freedom";

const GIVEN: &str = include_str!("../../fixtures/EV-006/given.native");
const MODEL: &str = include_str!("../../fixtures/EV-006/cell.semantic-ir.json");
const START: &str = include_str!("../../fixtures/EV-006/start.json");
const DEADLOCK_REQUEST: &str = include_str!("../../fixtures/EV-006/deadlock-request.json");
const REPAIRED_REQUEST: &str = include_str!("../../fixtures/EV-006/repaired-request.json");
const TASK: &str = include_str!("../../fixtures/EV-006/task.md");
const TERMINAL: &str = "terminal when { k.v = 1 };";
const ASSERT_BUDGET: Duration = Duration::from_secs(30);
const FILE_LIMIT: u64 = 1_048_576;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct DeadlockSummary {
    result: String,
    exit_code: i32,
    steps: Vec<String>,
    blocked: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct RepairedSummary {
    result: String,
    exit_code: i32,
    basis: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct Summary {
    deadlock: DeadlockSummary,
    repaired: RepairedSummary,
}

#[derive(Clone, Copy, Debug)]
enum FaultCode {
    InvalidArtifact,
    ChangedInput,
    WrongProtocol,
    UnsupportedAnalysis,
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
        write!(out, "{:?}: {}", self.code, self.detail)
    }
}
impl std::error::Error for Fault {}
type Checked<T> = Result<T, Fault>;

/// Write the fixed Fill protocol inputs into the sample workspace.
pub fn prepare(workspace: &Path, _tools: &Tools) -> ScenarioResult<String> {
    fs::create_dir_all(workspace)?;
    for (name, contents) in [
        ("given.native", GIVEN),
        ("protocol.native", GIVEN),
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

fn compact(source: &str) -> String {
    source
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect()
}

fn expected_repaired_source() -> String {
    GIVEN.replacen(
        "  role w on M::Cell;",
        &format!("  role w on M::Cell;\n  {TERMINAL}"),
        1,
    )
}

fn check_inputs(workspace: &Path) -> Checked<()> {
    for (name, contents) in [
        ("given.native", GIVEN),
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
    let submitted = read_file(workspace, "protocol.native")?;
    if compact(&submitted) != compact(&expected_repaired_source()) {
        return Err(Fault::new(
            FaultCode::WrongProtocol,
            "add only terminal when { k.v = 1 }; to the supplied Fill protocol",
        ));
    }
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
    if output.status.code() != Some(expected_exit) {
        return Err(Fault::new(
            FaultCode::ToolFailure,
            format!("trusted fsl analyze {request} did not exit {expected_exit}"),
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(|_| {
        Fault::new(
            FaultCode::ToolFailure,
            "trusted fsl analyze did not emit JSON",
        )
    })
}

fn one_item(document: &Value) -> Checked<&Value> {
    document["items"]
        .as_array()
        .filter(|items| items.len() == 1)
        .map(|items| &items[0])
        .ok_or_else(|| {
            Fault::new(
                FaultCode::UnsupportedAnalysis,
                "expected one deadlock-freedom item",
            )
        })
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
            FaultCode::UnsupportedAnalysis,
            "the baseline must settle the deadlock-freedom item as refuted",
        ));
    }
    let counterexample = &item["counterexample"];
    let encoded = counterexample.to_string();
    let prefix = counterexample["prefix"].as_array();
    let blocked = counterexample["blocked"].as_array();
    if counterexample["kind"] != "deadlock"
        || !prefix.is_some_and(|steps| steps.len() == 2)
        || !encoded.contains("fork")
        || !encoded.contains("Main/Both")
        || !encoded.contains("Main/Both/left/A")
        || !encoded.contains("TwoPre")
        || !encoded.contains("JoinWaiting")
        || !encoded.contains("right")
        || !encoded.contains("B")
        || !blocked.is_some_and(|threads| threads.len() == 2)
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
            FaultCode::UnsupportedAnalysis,
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

fn check_summary(workspace: &Path) -> Checked<()> {
    let actual: Summary =
        serde_json::from_value(json_file(workspace, "result.json")?).map_err(|_| {
            Fault::new(
                FaultCode::InvalidArtifact,
                "result.json has the wrong shape",
            )
        })?;
    let expected = Summary {
        deadlock: DeadlockSummary {
            result: "refuted".into(),
            exit_code: 10,
            steps: vec!["fork(Both)".into(), "attempt(A)".into()],
            blocked: vec![
                "B: Precondition{TwoPre}".into(),
                "join Both: JoinWaiting".into(),
            ],
        },
        repaired: RepairedSummary {
            result: "proved".into(),
            exit_code: 0,
            basis: "Exhaustive".into(),
        },
    };
    if actual != expected {
        return Err(Fault::new(
            FaultCode::WrongSummary,
            "result.json must report the specified deadlock and exhaustive repair",
        ));
    }
    Ok(())
}

fn assess(workspace: &Path, tools: &Tools, deadline: Instant) -> Checked<BTreeMap<String, Value>> {
    before_deadline(deadline).map_err(command_failure)?;
    check_inputs(workspace)?;
    check_summary(workspace)?;
    let saved_deadlock = json_file(workspace, "deadlock.json")?;
    let saved_repaired = json_file(workspace, "repaired.json")?;
    check_deadlock(&saved_deadlock)?;
    check_proof(&saved_repaired)?;
    let trusted_deadlock = analyze(workspace, tools, "deadlock-request.json", 10, deadline)?;
    let trusted_repaired = analyze(workspace, tools, "repaired-request.json", 0, deadline)?;
    compare_items(&saved_deadlock, &trusted_deadlock, false)?;
    compare_items(&saved_repaired, &trusted_repaired, true)?;
    before_deadline(deadline).map_err(command_failure)?;
    Ok(BTreeMap::from([
        (
            "protocol_and_intended_terminal_state_match".into(),
            json!(true),
        ),
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
            failures: vec![format!("{:?}: {fault}", fault.code)],
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;
    use tempfile::tempdir;

    #[trace("TC-656", "FR-211-AC-2")]
    #[test]
    fn only_the_intended_terminal_declaration_is_accepted() -> ScenarioResult<()> {
        let temp = tempdir()?;
        fs::write(temp.path().join("given.native"), GIVEN)?;
        fs::write(temp.path().join("cell.semantic-ir.json"), MODEL)?;
        fs::write(temp.path().join("start.json"), START)?;
        fs::write(temp.path().join("deadlock-request.json"), DEADLOCK_REQUEST)?;
        fs::write(temp.path().join("repaired-request.json"), REPAIRED_REQUEST)?;
        fs::write(
            temp.path().join("protocol.native"),
            expected_repaired_source(),
        )?;
        assert!(check_inputs(temp.path()).is_ok());
        let wrong = expected_repaired_source().replace("k.v = 1", "k.v = 2");
        fs::write(temp.path().join("protocol.native"), wrong)?;
        assert!(matches!(
            check_inputs(temp.path()),
            Err(Fault {
                code: FaultCode::WrongProtocol,
                ..
            })
        ));
        Ok(())
    }

    fn deadlock_result() -> Value {
        json!({"format":"filament-outcome/1","category":"violation","items":[{
            "category":"violation","claim_name":"Fill","record":{"result":"refuted"},
        "counterexample":{"kind":"deadlock","prefix":[{"transition":{"step":"fork","node":"Main/Both"}},
                {"transition":{"step":"attempt","node":"Main/Both/left/A"}}],
                "blocked":[{"thread":"(0, Both, right, 0)","rest_point":"B","cause":{"Precondition":"TwoPre"}},
                    {"thread":"(0)","rest_point":"join Both","cause":{"JoinWaiting":["(0, Both, right, 0)"]}}]}
        }]})
    }

    #[trace("TC-656", "FR-211-AC-2")]
    #[test]
    fn deadlock_requires_the_expected_protocol_prefix_and_wait_causes() {
        assert!(check_deadlock(&deadlock_result()).is_ok());
        let mut missing_wait = deadlock_result();
        missing_wait["items"][0]["counterexample"]["blocked"] = json!([]);
        assert!(check_deadlock(&missing_wait).is_err());
    }

    #[trace("TC-656", "FR-211-AC-2")]
    #[test]
    fn repaired_result_requires_certified_exhaustive_proof() {
        let proved = json!({"format":"filament-outcome/1","category":"success","items":[{
            "category":"success","claim_name":"Fill","record":{"result":"proved",
                "basis":{"type":"exhaustive"},"certification":"certified"}
        }]});
        assert!(check_proof(&proved).is_ok());
        let mut uncertified = proved.clone();
        uncertified["items"][0]["record"]["certification"] = json!("uncertified");
        assert!(check_proof(&uncertified).is_err());
    }
}
