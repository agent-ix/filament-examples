//! A public monitor journey distinguishing a closed bounded window from an open prefix.

use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

use serde::Deserialize;
use serde_json::{Value, json};

use crate::capture::{CaptureCode, CaptureError, before_deadline, capture_command};
use crate::{Assertion, ScenarioResult, Tools};

/// Stable scenario identifier in ticket order.
pub const ID: &str = "EV-004";
/// Human-readable catalogue title.
pub const TITLE: &str = "Monitor bounded trace evidence and preserve an open-prefix unknown";

const GIVEN: &str = include_str!("../../fixtures/EV-004/given.fsl");
const WINDOW_RULE: &str = "temporal WindowBelowThree using win over (m: MeterReading) clock \"ticks\" on origin { always[0,2] holds(m.level <= 2) }";
const OPEN_RULE: &str = "temporal ForeverBelowThree using inf over (m: MeterReading) clock \"ticks\" on origin { always holds(m.level <= 2) }";
const TASK: &str = include_str!("../../fixtures/EV-004/task.md");
const ASSERT_BUDGET: Duration = Duration::from_secs(30);
const ARTIFACT_LIMIT: u64 = 1_048_576;
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
    Capture {
        code: CaptureCode,
        reaped_child: Option<u32>,
    },
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
impl From<CaptureError> for Fault {
    fn from(error: CaptureError) -> Self {
        Self {
            code: FaultCode::Capture {
                code: error.code,
                reaped_child: error.reaped_child,
            },
            detail: error.to_string(),
        }
    }
}
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
        metadata.file_type().is_file() && metadata.len() <= ARTIFACT_LIMIT,
        FaultCode::InvalidArtifact,
        "artifacts must be bounded regular files",
    )?;
    let file = fs::File::open(path).map_err(|_| Fault {
        code: FaultCode::InvalidArtifact,
        detail: format!("cannot open {name}"),
    })?;
    let mut bytes = Vec::new();
    file.take(ARTIFACT_LIMIT + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| Fault {
            code: FaultCode::InvalidArtifact,
            detail: format!("cannot read {name}"),
        })?;
    need(
        u64::try_from(bytes.len()).is_ok_and(|size| size <= ARTIFACT_LIMIT),
        FaultCode::InvalidArtifact,
        "artifact grew beyond its byte budget",
    )?;
    String::from_utf8(bytes).map_err(|_| Fault {
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
                });
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
    need(
        clause["basis"] == "closed-scope"
            && clause["disposition"]["truth"] == (case.kind == "holds")
            && clause["settlement"]["basis"] == basis
            && clause["settlement"]["horizon"] == "2"
            && clause["settlement"]["position"] == "0"
            && clause["settlement"]["trigger"] == "decided"
            && event["settlement"] == clause["settlement"],
        FaultCode::WrongPosition,
        "the bounded origin window and its settlement must remain distinct from the observed position",
    )?;
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
    need(
        actual == expected,
        FaultCode::SummaryMismatch,
        "report the bounded result, actual positions and open-prefix unknown without claiming an unbounded proof",
    )
}

fn monitor(
    workspace: &Path,
    tools: &Tools,
    case: Case,
    deadline: Instant,
) -> Checked<MonitorOutput> {
    let mut command = Command::new(&tools.fsl);
    command.current_dir(workspace).args([
        "monitor",
        case.request,
        "--input",
        case.input,
        "--format",
        "json",
    ]);
    let output = capture_command(&mut command, deadline).map_err(Fault::from)?;
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

fn semantic_stream(output: &MonitorOutput) -> Value {
    let events: Vec<Value> = output
        .events
        .iter()
        .map(|event| {
            let mut semantic = event.clone();
            if let Some(object) = semantic.as_object_mut() {
                object.remove("wall_ms");
            }
            semantic
        })
        .collect();
    let mut summary = output.summary.clone();
    if let Some(object) = summary.as_object_mut() {
        object.remove("elapsed_ms");
    }
    json!({"events":events,"summary":summary})
}

fn require_same_stream(saved: &MonitorOutput, trusted: &MonitorOutput) -> Checked<()> {
    need(
        semantic_stream(saved) == semantic_stream(trusted),
        FaultCode::EvidenceMismatch,
        "saved complete ordered semantic stream and input binding differ from independent public monitor",
    )
}

fn assess(workspace: &Path, tools: &Tools, deadline: Instant) -> Checked<BTreeMap<String, Value>> {
    before_deadline(deadline).map_err(Fault::from)?;
    check_inputs(workspace)?;
    check_report(workspace)?;
    before_deadline(deadline).map_err(Fault::from)?;
    let mut checks = BTreeMap::new();
    for &case in CASES {
        before_deadline(deadline).map_err(Fault::from)?;
        let saved = parse_output(&read_artifact(workspace, case.output)?)?;
        observe(case, &saved)?;
        before_deadline(deadline).map_err(Fault::from)?;
        let trusted = monitor(workspace, tools, case, deadline)?;
        let observation = observe(case, &trusted)?;
        require_same_stream(&saved, &trusted)?;
        before_deadline(deadline).map_err(Fault::from)?;
        checks.insert(case.input.to_owned(), observation);
    }
    Ok(checks)
}

/// Independently monitor all supplied traces with the exact requested rules and preserve finality.
pub fn grade(workspace: &Path, tools: &Tools) -> ScenarioResult<Assertion> {
    grade_with_budget(workspace, tools, ASSERT_BUDGET)
}

fn grade_with_budget(
    workspace: &Path,
    tools: &Tools,
    budget: Duration,
) -> ScenarioResult<Assertion> {
    let deadline = Instant::now() + budget;
    Ok(match assess(workspace, tools, deadline) {
        Ok(checks) => Assertion {
            ok: true,
            checks,
            failures: Vec::new(),
        },
        Err(Fault {
            code: FaultCode::Capture { code, reaped_child },
            detail,
        }) => {
            let label = match code {
                CaptureCode::AssertionTimeout => "assert budget exceeded",
                CaptureCode::OutputLimit => "monitor output budget exceeded",
                CaptureCode::ToolFailure => "monitor process failure",
            };
            let mut checks = BTreeMap::from([
                ("failure_class".into(), json!("harness-error")),
                ("capture_code".into(), json!(format!("{code:?}"))),
                ("assert_budget_ms".into(), json!(budget.as_millis())),
            ]);
            if let Some(child_id) = reaped_child {
                checks.insert("terminated_reaped_child".into(), json!(child_id));
            }
            Assertion {
                ok: false,
                checks,
                failures: vec![format!("harness-error: {label} ({ID}): {detail}")],
            }
        }
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

    /// Failure-only OS-child integration; no monitor verdict or real FSL/agent is fabricated.
    #[test]
    fn grade_deadline_exhaustion_bounds_validation_and_reaps_monitor_child() {
        let workspace = tempfile::tempdir().unwrap();
        let tools = tools(PathBuf::from("/bin/sh"));
        prepare(workspace.path(), &tools).unwrap();
        fs::write(
            workspace.path().join("monitor.fsl"),
            include_str!("../../fixtures/EV-004/reference/monitor.fsl"),
        )
        .unwrap();
        fs::write(
            workspace.path().join("result.json"),
            serde_json::to_vec(&report()).unwrap(),
        )
        .unwrap();
        for &case in CASES {
            fs::write(workspace.path().join(case.output), render(&synthetic(case))).unwrap();
        }
        // /bin/sh consumes this local script as its first argument and execs a
        // single delayed child; it emits no pretend FSL stream or proof.
        fs::write(workspace.path().join("monitor"), "exec sleep 30\n").unwrap();
        let result =
            grade_with_budget(workspace.path(), &tools, Duration::from_millis(150)).unwrap();
        assert!(!result.ok);
        assert_eq!(result.checks["failure_class"], "harness-error");
        assert_eq!(result.checks["capture_code"], "AssertionTimeout");
        assert!(result.failures[0].starts_with("harness-error: assert budget exceeded"));
        let child_id =
            u32::try_from(result.checks["terminated_reaped_child"].as_u64().unwrap()).unwrap();
        let alive = Command::new("/bin/kill")
            .args(["-0", &child_id.to_string()])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .unwrap();
        assert!(
            !alive.success(),
            "shared capture must reap the timed-out child before grade returns"
        );
        let no_inputs = tempfile::tempdir().unwrap();
        let expired = grade_with_budget(no_inputs.path(), &tools, Duration::ZERO).unwrap();
        assert_eq!(
            expired.checks["capture_code"], "AssertionTimeout",
            "budget starts before input validation"
        );
        assert!(!expired.checks.contains_key("terminated_reaped_child"));
    }

    /// New synthetic mutation check; it never executes the accepted public reference or agents.
    #[test]
    fn complete_stream_rejects_intermediate_and_summary_mutations() {
        let case = CASES[0];
        let mut original = synthetic(case);
        for position in [1, 2] {
            original.events.insert(position, json!({"format":"filament.monitor-event/1","clause":case.clause,
                "basis":"unsettled","category":"pending","settled":false,"position":position,"record":position,
                "disposition":{"category":"pending","stage":"evaluate","temporal":{"kind":"pending"}}}));
        }
        assert!(observe(case, &original).is_ok());
        let original_text = render(&original);
        let mut formatted = parse_output(&original_text).unwrap();
        formatted.events[1]["wall_ms"] = json!(37);
        formatted.summary["elapsed_ms"] = json!(91);
        assert!(require_same_stream(&formatted, &original).is_ok());
        for mutation in [
            "added",
            "omitted",
            "altered",
            "reordered",
            "summary-step",
            "input-binding",
            "nested-timing",
        ] {
            let mut saved = parse_output(&original_text).unwrap();
            match mutation {
                "added" => saved.events.insert(1, saved.events[1].clone()),
                "omitted" => {
                    saved.events.remove(1);
                }
                "altered" => saved.events[1]["disposition"]["temporal"]["kind"] = json!("violated"),
                "reordered" => saved.events.swap(1, 2),
                "summary-step" => {
                    saved.summary["clauses"][0]["settlement"]["step"] = json!("999");
                    saved.events.last_mut().unwrap()["settlement"]["step"] = json!("999");
                }
                "input-binding" => {
                    saved.summary["ledger"]["entries"][0]["binding"]["clause"] =
                        json!("another-rule")
                }
                "nested-timing" => saved.events[1]["disposition"]["wall_ms"] = json!(1),
                _ => unreachable!("closed mutation set"),
            }
            assert!(
                observe(case, &saved).is_ok(),
                "the old selected-field oracle permits {mutation}"
            );
            assert!(
                matches!(
                    require_same_stream(&saved, &original),
                    Err(Fault {
                        code: FaultCode::EvidenceMismatch,
                        ..
                    })
                ),
                "complete stream must reject {mutation}"
            );
        }
    }

    fn tools(fsl: PathBuf) -> Tools {
        Tools {
            fsl,
            docs: PathBuf::new(),
            quire: PathBuf::new(),
            quoin: PathBuf::new(),
        }
    }

    fn report() -> Value {
        json!({
            "holding":{"clause":"WindowBelowThree","kind":"holds","exit_code":0,"settled":true,"evaluation_position":0,"observed_position":2},
            "violating":{"clause":"WindowBelowThree","kind":"violated","exit_code":10,"settled":true,"evaluation_position":0,"observed_position":2,"bad_sample":2,"bad_level":3},
            "open":{"clause":"ForeverBelowThree","kind":"pending","exit_code":0,"settled":false,"unbounded_proof":false}
        })
    }

    // ORIGINAL synthetic public-wire unit data; it is not a live monitor receipt.
    fn synthetic(case: Case) -> MonitorOutput {
        let initial = json!({"format":"filament.monitor-event/1","clause":case.clause,
            "category":"pending","settled":false,"position":0,"record":0});
        let mut disposition =
            json!({"category":case.category,"stage":"evaluate","temporal":{"kind":case.kind}});
        let settled = case.kind != "pending";
        if settled {
            disposition["truth"] = json!(case.kind == "holds");
        }
        if case.kind == "violated" {
            disposition["temporal"]["position"] = json!(0);
            disposition["temporal"]["counterexample"] = json!({"kind":"formula","trace":"observed","trace_position":0,
                "interval":{"lower":"0","upper":"2"},"prefix":[{"sample":"0"},{"sample":"1"},{"sample":"2"}],"loop":[]});
        }
        let settlement = if settled {
            json!({"basis":if case.kind=="holds"{"decisive-witness"}else{"decisive-counterexample"},
                "horizon":"2","position":"0","step":"4","trigger":"decided"})
        } else {
            json!({"step":"3","trigger":"finish"})
        };
        let clause = json!({"clause":case.clause,"category":case.category,"settled":settled,
            "basis":if settled{"closed-scope"}else{"unsettled"},"disposition":disposition,"settlement":settlement});
        let mut events = vec![initial];
        if settled {
            events.push(
                json!({"format":"filament.monitor-event/1","clause":case.clause,
            "category":case.category,"settled":true,"position":2,"record":3,
            "disposition":disposition,"settlement":settlement}),
            );
        }
        let mut summary = json!({"format":"filament.monitor-summary/1","records":if settled{5}else{4},"refused":0,"clauses":[clause]});
        if settled {
            summary["ledger"] = json!({"format":"filament.progress-ledger/1","entries":[{
                "binding":{"package":"eval:trace-meter/rules","profile":"filament.temporal.event-position.false-extension/v1","clock":"ticks","clause":"synthetic-current-rule"},
                "watermark":2,"scope":"closed","execution":"closed","completeness":{"complete":{"through":2}},
                "covered":[{"stamp":0,"fingerprint":"sha256:unit-zero"},{"stamp":1,"fingerprint":"sha256:unit-one"},{"stamp":2,"fingerprint":"sha256:unit-two"}]
            }]});
        }
        MonitorOutput { events, summary }
    }

    fn render(output: &MonitorOutput) -> String {
        output
            .events
            .iter()
            .chain(std::iter::once(&output.summary))
            .map(Value::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Provenance: QSL-2488. Explicit public-monitor reference and counterfeit-evidence lane.
    #[test]
    #[ignore = "public FSL monitor reference lane: set FSL to the accepted shared binary"]
    fn public_monitor_reference_and_current_binding_rejection() {
        let fsl = PathBuf::from(std::env::var_os("FSL").expect("set FSL for this lane"));
        let tools = tools(fsl);
        let workspace = tempfile::tempdir().unwrap();
        prepare(workspace.path(), &tools).unwrap();
        fs::write(
            workspace.path().join("monitor.fsl"),
            include_str!("../../fixtures/EV-004/reference/monitor.fsl"),
        )
        .unwrap();
        for &case in CASES {
            let output = Command::new(&tools.fsl)
                .current_dir(workspace.path())
                .args([
                    "monitor",
                    case.request,
                    "--input",
                    case.input,
                    "--format",
                    "json",
                ])
                .output()
                .unwrap();
            assert_eq!(output.status.code(), Some(case.exit), "{}", case.input);
            fs::write(workspace.path().join(case.output), output.stdout).unwrap();
        }
        fs::write(
            workspace.path().join("result.json"),
            serde_json::to_vec(&report()).unwrap(),
        )
        .unwrap();
        let result = grade(workspace.path(), &tools).unwrap();
        assert!(result.ok, "{:?}", result.failures);
        assert_eq!(result.checks["holding.jsonl"]["kind"], "holds");
        assert_eq!(result.checks["violating.jsonl"]["observed_position"], 2);
        assert_eq!(result.checks["open.jsonl"]["settled"], false);
        let holding = read_artifact(workspace.path(), "holding-result.jsonl").unwrap();
        let mut parsed = parse_output(&holding).unwrap();
        parsed.summary["ledger"]["entries"][0]["binding"]["clause"] = json!("another-rule");
        fs::write(
            workspace.path().join("holding-result.jsonl"),
            render(&parsed),
        )
        .unwrap();
        let result = grade(workspace.path(), &tools).unwrap();
        assert!(!result.ok);
        assert!(
            result.failures[0].contains("EvidenceMismatch"),
            "{:?}",
            result.failures
        );
    }

    /// Provenance: QSL-2488.
    #[test]
    fn bounded_result_requires_decisive_position_and_complete_progress() {
        let case = CASES[0];
        let mut output = synthetic(case);
        assert!(observe(case, &output).is_ok());
        output.events[1]["position"] = json!(0);
        assert!(matches!(
            observe(case, &output),
            Err(Fault {
                code: FaultCode::WrongPosition,
                ..
            })
        ));
        output = synthetic(case);
        output.summary["ledger"]["entries"][0]["completeness"]["complete"]["through"] = json!(1);
        assert!(matches!(
            observe(case, &output),
            Err(Fault {
                code: FaultCode::MissingProgress,
                ..
            })
        ));
    }

    /// Provenance: QSL-2488.
    #[test]
    fn open_prefix_success_is_not_an_unbounded_proof() {
        let case = CASES[2];
        let mut output = synthetic(case);
        assert!(observe(case, &output).is_ok());
        output.summary["clauses"][0]["settled"] = json!(true);
        output.summary["clauses"][0]["disposition"]["temporal"]["kind"] = json!("holds");
        assert!(matches!(
            observe(case, &output),
            Err(Fault {
                code: FaultCode::WrongDisposition,
                ..
            })
        ));
    }

    /// Provenance: QSL-2488.
    #[test]
    fn semantic_evidence_ignores_timings_and_retains_current_rule_binding() {
        let case = CASES[0];
        let mut output = synthetic(case);
        let original = observe(case, &output).unwrap();
        output.summary["elapsed_ms"] = json!(99);
        output.events[1]["wall_ms"] = json!(123);
        assert_eq!(observe(case, &output).unwrap(), original);
        output.summary["ledger"]["entries"][0]["binding"]["clause"] = json!("another-rule");
        assert_ne!(observe(case, &output).unwrap(), original);
    }

    /// Provenance: QSL-2488.
    #[test]
    fn changed_inputs_tautologies_and_false_reports_are_rejected() {
        let workspace = tempfile::tempdir().unwrap();
        let tools = tools(PathBuf::from("/not-an-executable"));
        prepare(workspace.path(), &tools).unwrap();
        fs::write(
            workspace.path().join("monitor.fsl"),
            format!(
                "{GIVEN}{}{}",
                WINDOW_RULE.replace("m.level <= 2", "true"),
                OPEN_RULE
            ),
        )
        .unwrap();
        let result = grade(workspace.path(), &tools).unwrap();
        assert!(!result.ok);
        assert!(result.failures[0].contains("WrongRule"));
        fs::write(
            workspace.path().join("monitor.fsl"),
            format!("{GIVEN}{WINDOW_RULE}{OPEN_RULE}"),
        )
        .unwrap();
        fs::write(
            workspace.path().join("result.json"),
            serde_json::to_vec(&report()).unwrap(),
        )
        .unwrap();
        assert!(check_inputs(workspace.path()).is_ok());
        assert!(check_report(workspace.path()).is_ok());
        let mut invented = report();
        invented["open"]["unbounded_proof"] = json!(true);
        fs::write(
            workspace.path().join("result.json"),
            serde_json::to_vec(&invented).unwrap(),
        )
        .unwrap();
        assert!(matches!(
            check_report(workspace.path()),
            Err(Fault {
                code: FaultCode::SummaryMismatch,
                ..
            })
        ));
        fs::write(workspace.path().join("holding.jsonl"), "{\"end\":true}\n").unwrap();
        assert!(matches!(
            check_inputs(workspace.path()),
            Err(Fault {
                code: FaultCode::ChangedInput,
                ..
            })
        ));
    }

    /// Provenance: QSL-2488.
    #[test]
    fn jsonl_stream_rejects_malformed_or_invented_monitor_results() {
        let correct = render(&synthetic(CASES[1]));
        assert!(parse_output(&correct).is_ok());
        for text in [
            "not JSON",
            "{\"format\":\"filament-outcome/1\",\"category\":\"success\"}",
            "{\"format\":\"filament.monitor-summary/1\"}",
        ] {
            assert!(parse_output(text).is_err(), "{text}");
        }
        assert!(
            parse_output(&format!(
                "{correct}\n{{\"format\":\"filament.monitor-event/1\"}}"
            ))
            .is_err()
        );
    }

    /// ORIGINAL synthetic SDK transcript, not a live-agent command receipt.
    #[test]
    fn command_evidence_requires_all_three_actual_monitor_inputs() {
        let workspace = tempfile::tempdir().unwrap();
        let directory = workspace.path().join(".cli-agent-evals/transcripts");
        fs::create_dir_all(&directory).unwrap();
        let path = directory.join("monitor-unit.transcript");
        let holding = "fsl monitor window-request.json --input holding.jsonl --format json > holding-result.jsonl";
        let violating = "fsl monitor window-request.json --input violating.jsonl --format json > violating-result.jsonl";
        let open =
            "fsl monitor open-request.json --input open.jsonl --format json > open-result.jsonl";
        for (commands, expected) in [
            (vec![holding, violating, open], true),
            (vec![holding, violating], false),
            (
                vec![
                    holding,
                    violating,
                    "echo 'fsl monitor open-request.json --input open.jsonl'",
                ],
                false,
            ),
        ] {
            let records:Vec<String>=commands.into_iter().map(|command|json!({"type":"assistant","message":{"content":[{"type":"tool_use","name":"Bash","input":{"command":command}}]}}).to_string()).collect();
            fs::write(&path, records.join("\n")).unwrap();
            let commands = crate::agent_commands(workspace.path()).unwrap();
            let evidence = CASES.iter().all(|case| {
                commands
                    .iter()
                    .any(|command| crate::invokes_fsl(command, "monitor", Some(case.input)))
            });
            assert_eq!(evidence, expected, "{commands:?}");
        }
    }
}
