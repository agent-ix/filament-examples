//! A public monitor journey distinguishing a closed bounded window from an open prefix.

use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::Path;
use std::process::Command;

use serde::Deserialize;
use serde_json::{json, Value};

use crate::{Assertion, ScenarioResult, Tools};

/// Stable scenario identifier in ticket order.
pub const ID: &str = "EV-004";
/// Human-readable catalogue title.
pub const TITLE: &str = "Monitor bounded trace evidence and preserve an open-prefix unknown";

const GIVEN: &str = include_str!("../../fixtures/EV-004/given.fsl");
const WINDOW_RULE: &str = "temporal WindowBelowThree using win over (m: MeterReading) clock \"ticks\" on origin { always[0,2] holds(m.level <= 2) }";
const OPEN_RULE: &str = "temporal ForeverBelowThree using inf over (m: MeterReading) clock \"ticks\" on origin { always holds(m.level <= 2) }";
const TASK: &str = include_str!("../../fixtures/EV-004/task.md");
const INPUTS: &[(&str, &str)] = &[
    (
        "window-request.json",
        include_str!("../../fixtures/EV-004/window-request.json"),
    ),
    (
        "open-request.json",
        include_str!("../../fixtures/EV-004/open-request.json"),
    ),
    (
        "holding.jsonl",
        include_str!("../../fixtures/EV-004/holding.jsonl"),
    ),
    (
        "violating.jsonl",
        include_str!("../../fixtures/EV-004/violating.jsonl"),
    ),
    (
        "open.jsonl",
        include_str!("../../fixtures/EV-004/open.jsonl"),
    ),
];

#[derive(Clone, Copy)]
struct Case {
    input: &'static str,
    request: &'static str,
    output: &'static str,
    clause: &'static str,
    category: &'static str,
    kind: &'static str,
    exit: i32,
}

const CASES: &[Case] = &[
    Case {
        input: "holding.jsonl",
        request: "window-request.json",
        output: "holding-result.jsonl",
        clause: "WindowBelowThree",
        category: "success",
        kind: "holds",
        exit: 0,
    },
    Case {
        input: "violating.jsonl",
        request: "window-request.json",
        output: "violating-result.jsonl",
        clause: "WindowBelowThree",
        category: "violation",
        kind: "violated",
        exit: 10,
    },
    Case {
        input: "open.jsonl",
        request: "open-request.json",
        output: "open-result.jsonl",
        clause: "ForeverBelowThree",
        category: "pending",
        kind: "pending",
        exit: 0,
    },
];

#[derive(Debug, Clone, Copy)]
enum FaultCode {
    InvalidArtifact,
    ChangedInput,
    WrongRule,
    InvalidStream,
    WrongDisposition,
    WrongPosition,
    MissingProgress,
    WrongCounterexample,
    ToolFailure,
    SummaryMismatch,
    EvidenceMismatch,
}

#[derive(Debug)]
struct Fault {
    code: FaultCode,
    detail: String,
}
impl fmt::Display for Fault {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "{:?}: {}", self.code, self.detail)
    }
}
impl std::error::Error for Fault {}
type Checked<T> = Result<T, Fault>;

fn need(condition: bool, code: FaultCode, detail: &str) -> Checked<()> {
    if condition {
        Ok(())
    } else {
        Err(Fault {
            code,
            detail: detail.to_owned(),
        })
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct HoldingReport {
    clause: String,
    kind: String,
    exit_code: i32,
    settled: bool,
    evaluation_position: u64,
    observed_position: u64,
}
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct ViolatingReport {
    clause: String,
    kind: String,
    exit_code: i32,
    settled: bool,
    evaluation_position: u64,
    observed_position: u64,
    bad_sample: u64,
    bad_level: i64,
}
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct OpenReport {
    clause: String,
    kind: String,
    exit_code: i32,
    settled: bool,
    unbounded_proof: bool,
}
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct Report {
    holding: HoldingReport,
    violating: ViolatingReport,
    open: OpenReport,
}

/// Supply original record/trace inputs; the agent authors the two requested rules.
pub fn prepare(workspace: &Path, _tools: &Tools) -> ScenarioResult<String> {
    fs::create_dir_all(workspace)?;
    fs::write(workspace.join("monitor.fsl"), GIVEN)?;
    for (name, contents) in INPUTS {
        fs::write(workspace.join(name), contents)?;
    }
    Ok(TASK.to_owned())
}

fn read_artifact(workspace: &Path, name: &str) -> Checked<String> {
    let path = workspace.join(name);
    let metadata = fs::symlink_metadata(&path).map_err(|_| Fault {
        code: FaultCode::InvalidArtifact,
        detail: format!("missing {name}"),
    })?;
    need(
        metadata.file_type().is_file() && metadata.len() <= 1_048_576,
        FaultCode::InvalidArtifact,
        "artifacts must be bounded regular files",
    )?;
    fs::read_to_string(path).map_err(|_| Fault {
        code: FaultCode::InvalidArtifact,
        detail: format!("{name} must be UTF-8"),
    })
}

fn compact(source: &str) -> String {
    source.chars().filter(|c| !c.is_whitespace()).collect()
}

fn check_inputs(workspace: &Path) -> Checked<()> {
    for (name, contents) in INPUTS {
        need(
            read_artifact(workspace, name)? == *contents,
            FaultCode::ChangedInput,
            "the supplied requests and traces must remain unchanged",
        )?;
    }
    let source = read_artifact(workspace, "monitor.fsl")?;
    let tail = source.strip_prefix(GIVEN).ok_or_else(|| Fault {
        code: FaultCode::ChangedInput,
        detail: "the supplied profiles or record type changed".into(),
    })?;
    let actual = compact(tail);
    let a = compact(WINDOW_RULE);
    let b = compact(OPEN_RULE);
    need(
        actual == format!("{a}{b}") || actual == format!("{b}{a}"),
        FaultCode::WrongRule,
        "append the two exact requested temporal properties",
    )
}

struct MonitorOutput {
    events: Vec<Value>,
    summary: Value,
}

fn parse_output(text: &str) -> Checked<MonitorOutput> {
    let mut events = Vec::new();
    let mut summary = None;
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        need(
            summary.is_none(),
            FaultCode::InvalidStream,
            "a monitor summary must be the final record",
        )?;
        let row: Value = serde_json::from_str(line).map_err(|_| Fault {
            code: FaultCode::InvalidStream,
            detail: "monitor output must contain JSON records".into(),
        })?;
        match row["format"].as_str() {
            Some("filament.monitor-event/1") => events.push(row),
            Some("filament.monitor-summary/1") => summary = Some(row),
            _ => {
                return Err(Fault {
                    code: FaultCode::InvalidStream,
                    detail:
                        "expected real monitor events and a summary, not a refusal or self-report"
                            .into(),
                })
            }
        }
    }
    let summary = summary.ok_or_else(|| Fault {
        code: FaultCode::InvalidStream,
        detail: "missing monitor summary".into(),
    })?;
    need(
        !events.is_empty(),
        FaultCode::InvalidStream,
        "missing monitor event evidence",
    )?;
    Ok(MonitorOutput { events, summary })
}

fn observe(case: Case, output: &MonitorOutput) -> Checked<Value> {
    let summary = &output.summary;
    let clauses = summary["clauses"]
        .as_array()
        .filter(|c| c.len() == 1)
        .ok_or_else(|| Fault {
            code: FaultCode::WrongDisposition,
            detail: "expected exactly one monitored clause".into(),
        })?;
    let clause = &clauses[0];
    let settled = case.kind != "pending";
    need(
        summary["refused"] == 0 && summary["records"] == if settled { 5 } else { 4 },
        FaultCode::InvalidStream,
        "all supplied trace records must be consumed without refusals",
    )?;
    need(
        clause["clause"] == case.clause
            && clause["category"] == case.category
            && clause["settled"] == settled
            && clause["disposition"]["category"] == case.category
            && clause["disposition"]["stage"] == "evaluate"
            && clause["disposition"]["temporal"]["kind"] == case.kind,
        FaultCode::WrongDisposition,
        "the requested clause must have the intended actual temporal disposition",
    )?;
    let initial = &output.events[0];
    need(
        initial["position"] == 0
            && initial["record"] == 0
            && initial["category"] == "pending"
            && initial["settled"] == false,
        FaultCode::WrongPosition,
        "monitoring starts pending at origin 0",
    )?;
    for event in &output.events {
        need(
            event["clause"] == case.clause,
            FaultCode::WrongDisposition,
            "event evidence names another clause",
        )?;
    }
    if !settled {
        need(
            clause["basis"] == "unsettled"
                && clause["settlement"]["trigger"] == "finish"
                && output
                    .events
                    .iter()
                    .all(|event| event["settled"] == false && event["category"] == "pending"),
            FaultCode::WrongDisposition,
            "a finite good prefix cannot settle the unbounded rule",
        )?;
        return Ok(
            json!({"clause":case.clause,"kind":"pending","category":"pending","settled":false,"records":4}),
        );
    }
    let decisive: Vec<&Value> = output
        .events
        .iter()
        .filter(|event| event["settled"] == true)
        .collect();
    need(
        decisive.len() == 1,
        FaultCode::WrongDisposition,
        "expected one decisive bounded-window event",
    )?;
    let event = decisive[0];
    need(
        event["position"] == 2
            && event["record"] == 3
            && event["category"] == case.category
            && event["disposition"] == clause["disposition"],
        FaultCode::WrongPosition,
        "the bounded result must settle after observation 2, at progress record 3",
    )?;
    let basis = if case.kind == "holds" {
        "decisive-witness"
    } else {
        "decisive-counterexample"
    };
    need(clause["basis"] == "closed-scope" && clause["disposition"]["truth"] == (case.kind == "holds")
        && clause["settlement"]["basis"] == basis && clause["settlement"]["horizon"] == "2"
        && clause["settlement"]["position"] == "0" && clause["settlement"]["trigger"] == "decided"
        && event["settlement"] == clause["settlement"],
        FaultCode::WrongPosition, "the bounded origin window and its settlement must remain distinct from the observed position")?;
    let entries = summary["ledger"]["entries"]
        .as_array()
        .filter(|e| e.len() == 1)
        .ok_or_else(|| Fault {
            code: FaultCode::MissingProgress,
            detail: "missing authoritative progress binding".into(),
        })?;
    let entry = &entries[0];
    need(
        summary["ledger"]["format"] == "filament.progress-ledger/1"
            && entry["binding"]["package"] == "eval:trace-meter/rules"
            && entry["binding"]["profile"] == "filament.temporal.event-position.false-extension/v1"
            && entry["binding"]["clock"] == "ticks"
            && entry["watermark"] == 2
            && entry["scope"] == "closed"
            && entry["execution"] == "closed"
            && entry["completeness"]["complete"]["through"] == 2,
        FaultCode::MissingProgress,
        "the holding/violating window needs its real closure/completeness evidence",
    )?;
    let covered = entry["covered"]
        .as_array()
        .filter(|c| c.len() == 3)
        .ok_or_else(|| Fault {
            code: FaultCode::MissingProgress,
            detail: "progress must cover all three observations".into(),
        })?;
    for (position, sample) in covered.iter().enumerate() {
        let position = u64::try_from(position).map_err(|_| Fault {
            code: FaultCode::MissingProgress,
            detail: "sample position overflow".into(),
        })?;
        need(
            sample["stamp"] == position
                && sample["fingerprint"]
                    .as_str()
                    .is_some_and(|s| s.starts_with("sha256:")),
            FaultCode::MissingProgress,
            "closure evidence must retain each ordered sample fingerprint",
        )?;
    }
    let counterexample = &clause["disposition"]["temporal"]["counterexample"];
    if case.kind == "violated" {
        need(
            clause["disposition"]["temporal"]["position"] == 0
                && counterexample["trace"] == "observed"
                && counterexample["trace_position"] == 0
                && counterexample["kind"] == "formula"
                && counterexample["interval"] == json!({"lower":"0","upper":"2"})
                && counterexample["prefix"]
                    == json!([{"sample":"0"},{"sample":"1"},{"sample":"2"}])
                && counterexample["loop"] == json!([]),
            FaultCode::WrongCounterexample,
            "the violation must retain its observed origin-window counterexample",
        )?;
    }
    Ok(
        json!({"clause":case.clause,"kind":case.kind,"category":case.category,"settled":true,
        "evaluation_position":0,"observed_position":2,"disposition":clause["disposition"],
        "settlement_basis":basis,"horizon":"2","progress_binding":entry["binding"],
        "covered":entry["covered"],"completeness":entry["completeness"],"records":5}),
    )
}

fn check_report(workspace: &Path) -> Checked<()> {
    let actual: Report =
        serde_json::from_str(&read_artifact(workspace, "result.json")?).map_err(|_| Fault {
            code: FaultCode::SummaryMismatch,
            detail: "result.json must use the requested closed shape".into(),
        })?;
    let expected = Report {
        holding: HoldingReport {
            clause: "WindowBelowThree".into(),
            kind: "holds".into(),
            exit_code: 0,
            settled: true,
            evaluation_position: 0,
            observed_position: 2,
        },
        violating: ViolatingReport {
            clause: "WindowBelowThree".into(),
            kind: "violated".into(),
            exit_code: 10,
            settled: true,
            evaluation_position: 0,
            observed_position: 2,
            bad_sample: 2,
            bad_level: 3,
        },
        open: OpenReport {
            clause: "ForeverBelowThree".into(),
            kind: "pending".into(),
            exit_code: 0,
            settled: false,
            unbounded_proof: false,
        },
    };
    need(actual == expected, FaultCode::SummaryMismatch, "report the bounded result, actual positions and open-prefix unknown without claiming an unbounded proof")
}

fn monitor(workspace: &Path, tools: &Tools, case: Case) -> Checked<MonitorOutput> {
    let output = Command::new(&tools.fsl)
        .current_dir(workspace)
        .args([
            "monitor",
            case.request,
            "--input",
            case.input,
            "--format",
            "json",
        ])
        .output()
        .map_err(|_| Fault {
            code: FaultCode::ToolFailure,
            detail: "trusted public fsl monitor could not start".into(),
        })?;
    need(
        output.status.code() == Some(case.exit),
        FaultCode::ToolFailure,
        "trusted monitor exited with an unexpected status",
    )?;
    let stdout = std::str::from_utf8(&output.stdout).map_err(|_| Fault {
        code: FaultCode::InvalidStream,
        detail: "monitor must emit UTF-8 JSON lines".into(),
    })?;
    parse_output(stdout)
}

fn assess(workspace: &Path, tools: &Tools) -> Checked<BTreeMap<String, Value>> {
    check_inputs(workspace)?;
    check_report(workspace)?;
    let mut checks = BTreeMap::new();
    for &case in CASES {
        let saved = parse_output(&read_artifact(workspace, case.output)?)?;
        let observed = observe(case, &saved)?;
        let trusted = observe(case, &monitor(workspace, tools, case)?)?;
        need(observed == trusted, FaultCode::EvidenceMismatch, "saved semantic result/closure binding differs from independent public monitor on the current artifacts")?;
        checks.insert(case.input.to_owned(), trusted);
    }
    Ok(checks)
}

/// Independently monitor all supplied traces with the exact requested rules and preserve finality.
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
