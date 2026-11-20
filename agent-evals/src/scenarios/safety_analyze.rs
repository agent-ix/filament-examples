//! Agent journey for a finite safety proof and its counterexample.

use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::Path;
use std::process::Command;

use serde::Deserialize;
use serde_json::{json, Value};

use crate::{Assertion, ScenarioResult, Tools};

/// Stable scenario identifier in ticket order.
pub const ID: &str = "EV-003";
/// Human-readable catalogue title.
pub const TITLE: &str = "Prove a finite safety property and read its counterexample";

const GIVEN: &str = include_str!("../../fixtures/EV-003/given.fsl");
const MODEL: &str = include_str!("../../fixtures/EV-003/counter.semantic-ir.json");
const START: &str = include_str!("../../fixtures/EV-003/start.json");
const HOLDING_REQUEST: &str = include_str!("../../fixtures/EV-003/holding-request.json");
const VIOLATING_REQUEST: &str = include_str!("../../fixtures/EV-003/violating-request.json");
const TASK: &str = include_str!("../../fixtures/EV-003/task.md");
const WITHIN: &str = "temporal WithinLimit using inf over (c: Meter::Counter) clock \"model-steps\" on origin { always holds(c.level <= 3) }";
const SMALL: &str = "temporal TooSmall using inf over (c: Meter::Counter) clock \"model-steps\" on origin { always holds(c.level <= 2) }";
const FILE_LIMIT: u64 = 524_288;

#[derive(Clone, Copy, Debug)]
enum FailureCode {
    InvalidArtifact,
    ChangedInput,
    WrongProperty,
    NonterminalAnalysis,
    CounterexampleMismatch,
    OutputMismatch,
    WrongSummary,
    ToolFailure,
}

#[derive(Debug)]
struct Failure {
    code: FailureCode,
    detail: String,
}

impl Failure {
    fn new(code: FailureCode, detail: impl Into<String>) -> Self {
        Self {
            code,
            detail: detail.into(),
        }
    }
}

impl fmt::Display for Failure {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "{:?}: {}", self.code, self.detail)
    }
}

impl std::error::Error for Failure {}

type Checked<T> = Result<T, Failure>;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct HoldingSummary {
    claim: String,
    verdict: String,
    exit_code: i32,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct ViolatingSummary {
    claim: String,
    verdict: String,
    exit_code: i32,
    trace_position: usize,
    counterexample_levels: Vec<i64>,
    violating_level: i64,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct Summary {
    holding: HoldingSummary,
    violating: ViolatingSummary,
}

/// Write only the user's original fixed inputs; reference solutions stay outside the workspace.
pub fn prepare(workspace: &Path, _tools: &Tools) -> ScenarioResult<String> {
    fs::create_dir_all(workspace)?;
    for (name, contents) in [
        ("counter.fsl", GIVEN),
        ("counter.semantic-ir.json", MODEL),
        ("start.json", START),
        ("holding-request.json", HOLDING_REQUEST),
        ("violating-request.json", VIOLATING_REQUEST),
    ] {
        fs::write(workspace.join(name), contents)?;
    }
    Ok(TASK.to_owned())
}

fn read_artifact(workspace: &Path, name: &str) -> Checked<String> {
    let path = workspace.join(name);
    let metadata = fs::symlink_metadata(&path)
        .map_err(|_| Failure::new(FailureCode::InvalidArtifact, format!("missing {name}")))?;
    if !metadata.file_type().is_file() || metadata.len() > FILE_LIMIT {
        return Err(Failure::new(
            FailureCode::InvalidArtifact,
            format!("{name} must be a bounded regular file"),
        ));
    }
    fs::read_to_string(path).map_err(|_| {
        Failure::new(
            FailureCode::InvalidArtifact,
            format!("{name} must be UTF-8"),
        )
    })
}

fn json_artifact(workspace: &Path, name: &str) -> Checked<Value> {
    serde_json::from_str(&read_artifact(workspace, name)?)
        .map_err(|_| Failure::new(FailureCode::InvalidArtifact, format!("{name} must be JSON")))
}

fn compact(source: &str) -> String {
    source.chars().filter(|c| !c.is_whitespace()).collect()
}

fn check_inputs(workspace: &Path) -> Checked<()> {
    for (name, contents) in [
        ("counter.semantic-ir.json", MODEL),
        ("start.json", START),
        ("holding-request.json", HOLDING_REQUEST),
        ("violating-request.json", VIOLATING_REQUEST),
    ] {
        if read_artifact(workspace, name)? != contents {
            return Err(Failure::new(
                FailureCode::ChangedInput,
                format!("the user's fixed {name} was changed"),
            ));
        }
    }
    let source = read_artifact(workspace, "counter.fsl")?;
    let appended = source.strip_prefix(GIVEN).ok_or_else(|| {
        Failure::new(
            FailureCode::ChangedInput,
            "the supplied transition contract was changed",
        )
    })?;
    let actual = compact(appended);
    let within = compact(WITHIN);
    let small = compact(SMALL);
    if actual != format!("{within}{small}") && actual != format!("{small}{within}") {
        return Err(Failure::new(
            FailureCode::WrongProperty,
            "append the two requested safety declarations, without changing their meaning",
        ));
    }
    Ok(())
}

fn terminal_item<'a>(doc: &'a Value, claim: &str, result: &str) -> Checked<&'a Value> {
    let category = match result {
        "proved" => "success",
        "refuted" => "violation",
        _ => {
            return Err(Failure::new(
                FailureCode::NonterminalAnalysis,
                "unsupported grading verdict",
            ))
        }
    };
    if doc["format"] != "filament-outcome/1"
        || doc["category"] != category
        || doc["diagnostics"] != json!([])
    {
        return Err(Failure::new(
            FailureCode::NonterminalAnalysis,
            format!("{claim} did not return an unrefused {result} outcome"),
        ));
    }
    let items = doc["items"]
        .as_array()
        .filter(|items| items.len() == 1)
        .ok_or_else(|| {
            Failure::new(
                FailureCode::NonterminalAnalysis,
                "expected exactly one analysis item",
            )
        })?;
    let item = &items[0];
    if item["claim_name"] != claim
        || item["category"] != category
        || item["record"]["result"] != result
        || item["record"]["settled_by"] != "result"
        || !item["record"]["cause"].is_null()
    {
        return Err(Failure::new(
            FailureCode::NonterminalAnalysis,
            format!(
                "{claim} must actually settle {result}; incomplete or unsupported is not proof"
            ),
        ));
    }
    if result == "proved"
        && (item["record"]["basis"]["type"] != "exhaustive"
            || item["record"]["certification"] != "certified")
    {
        return Err(Failure::new(
            FailureCode::NonterminalAnalysis,
            "WithinLimit needs a certified exhaustive proof",
        ));
    }
    Ok(item)
}

fn counterexample_levels(item: &Value) -> Checked<Vec<i64>> {
    let counterexample = &item["counterexample"];
    if counterexample["kind"] != "formula"
        || counterexample["trace_position"] != 3
        || counterexample["over_binding"]["identity"] != "meter"
        || counterexample["loop"] != json!([])
    {
        return Err(Failure::new(
            FailureCode::CounterexampleMismatch,
            "TooSmall must identify meter's finite bad prefix at position 3",
        ));
    }
    let prefix = counterexample["prefix"].as_array().ok_or_else(|| {
        Failure::new(
            FailureCode::CounterexampleMismatch,
            "missing counterexample prefix",
        )
    })?;
    let mut levels = vec![0];
    for step in prefix {
        let populations = step["post_state"]["state"]["semantic"]["entries"]
            .as_array()
            .ok_or_else(|| {
                Failure::new(
                    FailureCode::CounterexampleMismatch,
                    "missing public semantic state",
                )
            })?;
        let mut found = Vec::new();
        for population in populations {
            let objects = population["value"]["entries"].as_array().ok_or_else(|| {
                Failure::new(FailureCode::CounterexampleMismatch, "missing object state")
            })?;
            for object in objects {
                if object["key"]["identity"] != "meter"
                    || object["key"]["object_type"]["original"]["node"]
                        != "ix://eval/safety-counter/Counter"
                {
                    continue;
                }
                let fields = object["value"]["fields"].as_array().ok_or_else(|| {
                    Failure::new(FailureCode::CounterexampleMismatch, "missing meter fields")
                })?;
                for field in fields {
                    if field["name"] == "level" && field["value"]["type"] == "integer" {
                        let value = field["value"]["value"]
                            .as_str()
                            .and_then(|s| s.parse::<i64>().ok())
                            .ok_or_else(|| {
                                Failure::new(
                                    FailureCode::CounterexampleMismatch,
                                    "invalid level in counterexample",
                                )
                            })?;
                        found.push(value);
                    }
                }
            }
        }
        if found.len() != 1 {
            return Err(Failure::new(
                FailureCode::CounterexampleMismatch,
                "each prefix state must contain exactly one meter level",
            ));
        }
        levels.push(found[0]);
    }
    if levels != [0, 1, 2, 3] {
        return Err(Failure::new(
            FailureCode::CounterexampleMismatch,
            "TooSmall must fail after levels 0, 1, 2, 3",
        ));
    }
    Ok(levels)
}

fn check_summary(workspace: &Path) -> Checked<()> {
    let actual: Summary =
        serde_json::from_str(&read_artifact(workspace, "result.json")?).map_err(|_| {
            Failure::new(
                FailureCode::WrongSummary,
                "result.json does not match the requested closed shape",
            )
        })?;
    let expected = Summary {
        holding: HoldingSummary {
            claim: "WithinLimit".into(),
            verdict: "proved".into(),
            exit_code: 0,
        },
        violating: ViolatingSummary {
            claim: "TooSmall".into(),
            verdict: "refuted".into(),
            exit_code: 10,
            trace_position: 3,
            counterexample_levels: vec![0, 1, 2, 3],
            violating_level: 3,
        },
    };
    if actual != expected {
        return Err(Failure::new(
            FailureCode::WrongSummary,
            "the reported verdicts or counterexample do not describe the requested problem",
        ));
    }
    Ok(())
}

fn analyze(workspace: &Path, tools: &Tools, request: &str, exit: i32) -> Checked<Value> {
    let output = Command::new(&tools.fsl)
        .current_dir(workspace)
        .args(["analyze", request, "--format", "json"])
        .output()
        .map_err(|_| {
            Failure::new(
                FailureCode::ToolFailure,
                "trusted public fsl analyze could not start",
            )
        })?;
    if output.status.code() != Some(exit) {
        return Err(Failure::new(
            FailureCode::ToolFailure,
            format!("trusted public analyze of {request} did not exit {exit}"),
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(|_| {
        Failure::new(
            FailureCode::ToolFailure,
            "trusted public analyze did not emit JSON",
        )
    })
}

fn assess(workspace: &Path, tools: &Tools) -> Checked<BTreeMap<String, Value>> {
    check_inputs(workspace)?;
    check_summary(workspace)?;
    let holding = json_artifact(workspace, "holding.json")?;
    let violating = json_artifact(workspace, "violating.json")?;
    terminal_item(&holding, "WithinLimit", "proved")?;
    counterexample_levels(terminal_item(&violating, "TooSmall", "refuted")?)?;

    // Invoke the trusted executable directly, never an agent-supplied wrapper or result.
    let trusted_holding = analyze(workspace, tools, "holding-request.json", 0)?;
    let trusted_violating = analyze(workspace, tools, "violating-request.json", 10)?;
    terminal_item(&trusted_holding, "WithinLimit", "proved")?;
    let levels = counterexample_levels(terminal_item(&trusted_violating, "TooSmall", "refuted")?)?;
    if holding != trusted_holding || violating != trusted_violating {
        return Err(Failure::new(
            FailureCode::OutputMismatch,
            "saved analysis output differs from independent public analyze",
        ));
    }
    Ok(BTreeMap::from([
        ("fixed_inputs_and_requested_properties".into(), json!(true)),
        ("independent_holding_verdict".into(), json!("proved")),
        ("independent_violating_verdict".into(), json!("refuted")),
        ("independent_counterexample_levels".into(), json!(levels)),
        ("saved_outcomes_and_summary_match".into(), json!(true)),
    ]))
}

/// Independently prove/refute the exact submitted properties using the trusted public CLI.
pub fn grade(workspace: &Path, tools: &Tools) -> ScenarioResult<Assertion> {
    Ok(match assess(workspace, tools) {
        Ok(checks) => Assertion {
            ok: true,
            checks,
            failures: Vec::new(),
        },
        Err(error) => Assertion {
            ok: false,
            checks: BTreeMap::new(),
            failures: vec![format!("{ID}: {error}")],
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn tools(fsl: PathBuf) -> Tools {
        Tools {
            fsl,
            docs: PathBuf::new(),
            quire: PathBuf::new(),
            quoin: PathBuf::new(),
        }
    }

    fn reference_summary() -> Value {
        json!({
            "holding": {"claim":"WithinLimit", "verdict":"proved", "exit_code":0},
            "violating": {"claim":"TooSmall", "verdict":"refuted", "exit_code":10,
                "trace_position":3, "counterexample_levels":[0,1,2,3], "violating_level":3}
        })
    }

    fn proof() -> Value {
        json!({"format":"filament-outcome/1", "category":"success", "diagnostics":[],
            "items":[{"claim_name":"WithinLimit", "category":"success",
                "record":{"result":"proved", "settled_by":"result", "cause":null,
                    "basis":{"type":"exhaustive"}, "certification":"certified"}}]})
    }

    /// Provenance: QSL-2487. Explicit public-CLI reference lane; no live agent is invoked.
    #[test]
    #[ignore = "public FSL reference lane: set FSL to the accepted shared binary"]
    fn reference_proves_and_refutes_through_public_cli() {
        let fsl = std::env::var_os("FSL").expect("set FSL for this explicit lane");
        let tools = tools(PathBuf::from(fsl));
        let workspace = tempfile::tempdir().expect("reference workspace");
        prepare(workspace.path(), &tools).expect("original input preparation");
        fs::write(
            workspace.path().join("counter.fsl"),
            include_str!("../../fixtures/EV-003/reference/counter.fsl"),
        )
        .expect("reference source");
        let holding =
            analyze(workspace.path(), &tools, "holding-request.json", 0).expect("public proof");
        let violating = analyze(workspace.path(), &tools, "violating-request.json", 10)
            .expect("public refutation");
        for (name, value) in [
            ("holding.json", holding),
            ("violating.json", violating),
            ("result.json", reference_summary()),
        ] {
            fs::write(
                workspace.path().join(name),
                serde_json::to_vec(&value).unwrap(),
            )
            .expect("reference artifact");
        }
        let assertion = grade(workspace.path(), &tools).expect("independent grade");
        assert!(assertion.ok, "{:?}", assertion.failures);
        assert_eq!(assertion.checks["independent_holding_verdict"], "proved");
        assert_eq!(assertion.checks["independent_violating_verdict"], "refuted");
        assert_eq!(
            assertion.checks["independent_counterexample_levels"],
            json!([0, 1, 2, 3])
        );
    }

    /// Provenance: QSL-2487.
    #[test]
    fn changed_inputs_and_tautologies_are_rejected_before_execution() {
        let tools = tools(PathBuf::from("/not-an-executable"));
        let workspace = tempfile::tempdir().unwrap();
        prepare(workspace.path(), &tools).unwrap();
        fs::write(
            workspace.path().join("counter.fsl"),
            format!("{GIVEN}{WITHIN}{SMALL}"),
        )
        .unwrap();
        assert!(check_inputs(workspace.path()).is_ok());
        fs::write(
            workspace.path().join("counter.fsl"),
            format!("{GIVEN}{}{}", WITHIN.replace("c.level <= 3", "true"), SMALL),
        )
        .unwrap();
        assert!(matches!(
            check_inputs(workspace.path()),
            Err(Failure {
                code: FailureCode::WrongProperty,
                ..
            })
        ));
        fs::write(
            workspace.path().join("counter.fsl"),
            format!("{GIVEN}{WITHIN}{SMALL}"),
        )
        .unwrap();
        fs::write(
            workspace.path().join("start.json"),
            START.replace("\"0\"", "\"3\""),
        )
        .unwrap();
        assert!(matches!(
            check_inputs(workspace.path()),
            Err(Failure {
                code: FailureCode::ChangedInput,
                ..
            })
        ));
    }

    /// Provenance: QSL-2487.
    #[test]
    fn incomplete_unsupported_or_uncertified_results_are_not_proofs() {
        assert!(terminal_item(&proof(), "WithinLimit", "proved").is_ok());
        for result in ["incomplete", "unsupported", "inconclusive", "refuted"] {
            let mut doc = proof();
            doc["items"][0]["record"]["result"] = json!(result);
            assert!(
                terminal_item(&doc, "WithinLimit", "proved").is_err(),
                "{result}"
            );
        }
        let mut doc = proof();
        doc["items"][0]["record"]["certification"] = json!("uncertified");
        assert!(terminal_item(&doc, "WithinLimit", "proved").is_err());
    }

    /// Provenance: QSL-2487.
    #[test]
    fn invented_or_wrong_counterexample_levels_are_rejected() {
        let step = |level| {
            json!({"post_state":{"state":{"semantic":{"entries":[{"value":{"entries":[{
                "key":{"identity":"meter", "object_type":{"original":{"node":"ix://eval/safety-counter/Counter"}}},
                "value":{"fields":[{"name":"level", "value":{"type":"integer", "value":format!("{level}")}}]}
            }]}}]}}}})
        };
        let mut item = json!({"counterexample":{"kind":"formula", "trace_position":3, "over_binding":{"identity":"meter"},
            "loop":[], "prefix":[step(1),step(2),step(3)]}});
        assert_eq!(counterexample_levels(&item).unwrap(), [0, 1, 2, 3]);
        item["counterexample"]["prefix"][2] = step(2);
        assert!(counterexample_levels(&item).is_err());
        item["counterexample"]["prefix"] = json!([]);
        assert!(counterexample_levels(&item).is_err());
    }

    /// Provenance: QSL-2487.
    #[test]
    fn forged_summaries_malformed_json_and_extra_fields_fail() {
        let workspace = tempfile::tempdir().unwrap();
        let path = workspace.path().join("result.json");
        fs::write(&path, serde_json::to_vec(&reference_summary()).unwrap()).unwrap();
        assert!(check_summary(workspace.path()).is_ok());
        let mut doc = reference_summary();
        doc["violating"]["violating_level"] = json!(2);
        fs::write(&path, serde_json::to_vec(&doc).unwrap()).unwrap();
        assert!(check_summary(workspace.path()).is_err());
        doc = reference_summary();
        doc["claimed_success"] = json!(true);
        fs::write(&path, serde_json::to_vec(&doc).unwrap()).unwrap();
        assert!(check_summary(workspace.path()).is_err());
        fs::write(&path, "not JSON").unwrap();
        assert!(check_summary(workspace.path()).is_err());
    }
}
