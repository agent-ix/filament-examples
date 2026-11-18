//! The structural comparison of a run with its record.

use fsl_examples::manifest::{Case, Status};
use fsl_examples::outcome::{Evaluation, Run, evaluate_case, project};
use serde_json::{Value, json};

type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

fn case(expect_exit: i32) -> Case {
    Case {
        id: "demo".to_owned(),
        rule: None,
        statement: "A rule.".to_owned(),
        file: "rules/demo.fsl".to_owned(),
        command: "run requests/demo.json".to_owned(),
        expect_exit,
        engine: None,
        clock: None,
        status: Status::Running {
            expect_verdict: "demo".to_owned(),
        },
    }
}

fn refusal() -> Value {
    json!({
        "artifacts": [], "category": "refusal",
        "diagnostics": [{
            "cause": "type-mismatch", "code": "ill_typed",
            "locus": {"end": 333, "kind": "region", "start": 324},
            "message": "the checker refused the package: ill_typed (type-mismatch)"
        }],
        "format": "filament-outcome/1", "items": [], "last_stage": "S3",
        "operation": "monitor", "result": null
    })
}

fn violation() -> Value {
    json!({
        "artifacts": [], "category": "violation", "diagnostics": [],
        "format": "filament-outcome/1",
        "items": [{
            "category": "violation", "cause": null, "clause": "EventuallyStopped",
            "counterexample": {"kind": "formula", "trace": "observed", "trace_position": 0,
                "prefix": [{"document": {"identity": "ix://example/rover/data/t0-driving"}}]},
            "decisive": {"element": {"kind": "boolean", "value": false}, "position": 0},
            "position": 0, "request_index": 0, "result": "refuted"
        }],
        "last_stage": "S6a", "operation": "monitor", "result": null
    })
}

fn success(value: bool) -> Value {
    json!({
        "artifacts": [], "category": "success", "diagnostics": [],
        "format": "filament-outcome/1", "items": [], "last_stage": "S6a",
        "operation": "execute",
        "result": {"kind": "completed", "value": {"kind": "boolean", "value": value}}
    })
}

/// Evaluates a run of `live` against the record of `recorded`, both with `exit`.
fn evaluate(recorded: &Value, live: &Value, exit: i32) -> Evaluation {
    let expected = project(&recorded.to_string()).structure;
    let run = Run {
        exit,
        stdout: live.to_string(),
    };
    evaluate_case(&case(exit), &run, Some(&expected), "demo/demo")
}

fn edited(mut doc: Value, pointer: &str, value: Value) -> Result<Value, String> {
    let slot = doc
        .pointer_mut(pointer)
        .ok_or_else(|| format!("no {pointer}"))?;
    *slot = value;
    Ok(doc)
}

fn assert_fails_on(evaluation: &Evaluation, field: &str) {
    assert!(!evaluation.ok, "expected a failure: {}", evaluation.message);
    assert!(
        evaluation.message.contains(&format!(" in {field}")),
        "message names {field}: {}",
        evaluation.message
    );
}

#[test]
fn reworded_message_still_passes() -> TestResult {
    let live = edited(
        refusal(),
        "/diagnostics/0/message",
        json!("this package does not type-check: a field has the wrong kind"),
    )?;
    let evaluation = evaluate(&refusal(), &live, 20);
    assert!(evaluation.ok, "{}", evaluation.message);
    assert_eq!(evaluation.message, "demo/demo: exit 20 ok");
    let moved = edited(refusal(), "/diagnostics/0/locus", json!(null))?;
    assert!(
        evaluate(&refusal(), &moved, 20).ok,
        "the locus is not compared"
    );
    Ok(())
}

#[test]
fn wrong_verdict_fails() -> TestResult {
    let evaluation = evaluate(&success(true), &success(false), 0);
    assert_fails_on(&evaluation, "result");
    let refuted = edited(violation(), "/items/0/result", json!("tested"))?;
    assert_fails_on(&evaluate(&violation(), &refuted, 10), "items");
    Ok(())
}

#[test]
fn wrong_exit_fails() {
    let expected = project(&success(true).to_string()).structure;
    let run = Run {
        exit: 10,
        stdout: success(true).to_string(),
    };
    let evaluation = evaluate_case(&case(0), &run, Some(&expected), "demo/demo");
    assert!(!evaluation.ok);
    assert_eq!(evaluation.message, "demo/demo: exit 10, expected 0");
}

#[test]
fn wrong_category_fails() -> TestResult {
    let live = edited(refusal(), "/category", json!("unsupported"))?;
    assert_fails_on(&evaluate(&refusal(), &live, 20), "category");
    Ok(())
}

#[test]
fn wrong_first_diagnostic_code_fails() -> TestResult {
    let live = edited(refusal(), "/diagnostics/0/code", json!("invalid_syntax"))?;
    assert_fails_on(&evaluate(&refusal(), &live, 20), "first_diagnostic");
    Ok(())
}

#[test]
fn wrong_cause_fails() -> TestResult {
    let live = edited(refusal(), "/diagnostics/0/cause", json!("unproved-range"))?;
    assert_fails_on(&evaluate(&refusal(), &live, 20), "first_diagnostic");
    Ok(())
}

#[test]
fn changed_witness_field_fails() -> TestResult {
    for (pointer, value) in [
        ("/items/0/counterexample/trace_position", json!(1)),
        ("/items/0/decisive/position", json!(1)),
        ("/items/0/position", json!(2)),
        ("/items/0/clause", json!("SafeStop")),
    ] {
        let live = edited(violation(), pointer, value)?;
        assert_fails_on(&evaluate(&violation(), &live, 10), "items");
    }
    Ok(())
}

#[test]
fn changed_last_stage_fails() -> TestResult {
    let live = edited(refusal(), "/last_stage", json!("S2"))?;
    assert_fails_on(&evaluate(&refusal(), &live, 20), "last_stage");
    Ok(())
}

#[test]
fn output_that_is_not_an_outcome_names_the_driver() {
    let expected = project(&success(true).to_string()).structure;
    let run = Run {
        exit: 0,
        stdout: "fsl-other 1.0\n".to_owned(),
    };
    let evaluation = evaluate_case(&case(0), &run, Some(&expected), "demo/demo");
    assert!(!evaluation.ok);
    assert!(
        evaluation
            .message
            .contains("check that FSL names the FSL driver"),
        "{}",
        evaluation.message
    );
}

fn monitor_stream(category: &str) -> String {
    let event = json!({"format": "filament.monitor-event/1", "clause": "C", "category": category});
    let summary = json!({
        "format": "filament.monitor-summary/1",
        "clauses": [{"clause": "C", "category": category}],
        "records": 2,
        "refused": 0
    });
    format!("{event}\n{summary}\n")
}

#[test]
fn monitor_stream_is_recorded_as_events_and_summary() {
    let structure = project(&monitor_stream("pending")).structure;
    assert_eq!(structure["events"].as_array().map(Vec::len), Some(1));
    assert_eq!(structure["summary"]["records"], 2);
    assert_eq!(
        fsl_examples::outcome::verdict_of(&structure),
        "pending",
        "{structure}"
    );
}

/// Trace: QSL-2185; the runner preserves the driver's indeterminate verdict label.
#[test]
fn indeterminate_outcome_is_reported_as_indeterminate() {
    let structure = json!({
        "format": "filament-outcome/1",
        "category": "indeterminate",
        "result": null,
        "items": []
    });
    assert_eq!(
        fsl_examples::outcome::verdict_of(&structure),
        "indeterminate"
    );
}

#[test]
fn changed_monitor_event_fails() {
    let expected = project(&monitor_stream("pending")).structure;
    let run = Run {
        exit: 0,
        stdout: monitor_stream("violation"),
    };
    let evaluation = evaluate_case(&case(0), &run, Some(&expected), "demo/demo");
    assert!(!evaluation.ok);
    assert!(
        evaluation.message.contains("events"),
        "{}",
        evaluation.message
    );
}
