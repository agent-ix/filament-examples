//! EV-009: learn supported public FSL verbs from CLI help and original inputs.

use crate::{Assertion, ScenarioResult, Tools};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Stable identity for the blind public-CLI journey.
pub const ID: &str = "EV-009";
/// User workflow exposed in the scenario catalogue.
pub const TITLE: &str = "Discover check, run, analyze and monitor using only CLI help";

const INPUTS: &[(&str, &str)] = &[
    (
        "calculation.fsl",
        include_str!("../../fixtures/EV-009/calculation.fsl"),
    ),
    (
        "run-request.json",
        include_str!("../../fixtures/EV-009/run-request.json"),
    ),
    (
        "inventory.fsl",
        include_str!("../../fixtures/EV-009/inventory.fsl"),
    ),
    (
        "initial.json",
        include_str!("../../fixtures/EV-009/initial.json"),
    ),
    (
        "analyze-request.json",
        include_str!("../../fixtures/EV-009/analyze-request.json"),
    ),
    (
        "monitor.fsl",
        include_str!("../../fixtures/EV-009/monitor.fsl"),
    ),
    (
        "monitor-request.json",
        include_str!("../../fixtures/EV-009/monitor-request.json"),
    ),
    (
        "readings.jsonl",
        include_str!("../../fixtures/EV-009/readings.jsonl"),
    ),
];

#[derive(Clone, Copy)]
enum Verb {
    Check,
    Run,
    Analyze,
    Monitor,
}

impl Verb {
    const ALL: &[Self] = &[Self::Check, Self::Run, Self::Analyze, Self::Monitor];

    fn name(self) -> &'static str {
        match self {
            Self::Check => "check",
            Self::Run => "run",
            Self::Analyze => "analyze",
            Self::Monitor => "monitor",
        }
    }

    fn operand(self) -> &'static str {
        match self {
            Self::Check => "calculation.fsl",
            Self::Run => "run-request.json",
            Self::Analyze => "analyze-request.json",
            Self::Monitor => "monitor-request.json",
        }
    }

    fn result_file(self) -> &'static str {
        match self {
            Self::Check => "checked.json",
            Self::Run => "run-result.json",
            Self::Analyze => "analysis-result.json",
            Self::Monitor => "monitor-result.jsonl",
        }
    }

    fn expected_exit(self) -> i32 {
        match self {
            Self::Check | Self::Run | Self::Analyze => 0,
            Self::Monitor => 10,
        }
    }
}

fn generated_domain() -> ScenarioResult<Vec<u8>> {
    let scratch = tempfile::tempdir()?;
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let destination = scratch.path().join("domain.json");
    let generated = Command::new("node")
        .arg(manifest.join("../tools/model.mjs"))
        .arg(manifest.join("fixtures/EV-009/domain.mjs"))
        .arg(&destination)
        .output()?;
    if !generated.status.success() {
        return Err("original domain input generation failed".into());
    }
    Ok(fs::read(destination)?)
}

/// Supply only original problem inputs; the provider must disable shipped references.
pub fn prepare(workspace: &Path, _tools: &Tools) -> ScenarioResult<String> {
    for &(name, content) in INPUTS {
        fs::write(workspace.join(name), content)?;
    }
    fs::write(workspace.join("domain.json"), generated_domain()?)?;
    let prompt = include_str!("../../fixtures/EV-009/task.md");
    fs::write(workspace.join("task.md"), prompt)?;
    Ok(prompt.to_owned())
}

fn regular_file(path: &Path) -> ScenarioResult<Vec<u8>> {
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.file_type().is_file() || metadata.len() > 1_048_576 {
        return Err("scenario artifact must be a bounded regular file".into());
    }
    Ok(fs::read(path)?)
}

fn immutable_inputs(workspace: &Path) -> ScenarioResult<Vec<String>> {
    let mut changed = Vec::new();
    for &(name, expected) in INPUTS {
        if !regular_file(&workspace.join(name)).is_ok_and(|actual| actual == expected.as_bytes()) {
            changed.push(name.to_owned());
        }
    }
    let domain = generated_domain()?;
    if !regular_file(&workspace.join("domain.json")).is_ok_and(|actual| actual == domain) {
        changed.push("domain.json".to_owned());
    }
    Ok(changed)
}

fn empty_diagnostics(outcome: &Value) -> bool {
    outcome["diagnostics"].as_array().is_some_and(Vec::is_empty)
}

fn semantic_result(verb: Verb, outcome: &Value) -> Option<Value> {
    if outcome["format"] != "filament-outcome/1" || !empty_diagnostics(outcome) {
        return None;
    }
    match verb {
        Verb::Check => (outcome["operation"] == "check" && outcome["category"] == "success")
            .then(|| json!({"checked":true})),
        Verb::Run => (outcome["operation"] == "execute"
            && outcome["category"] == "success"
            && outcome["result"] == json!({"kind":"completed","value":{"integer":"29"}}))
        .then(|| json!({"value":29})),
        Verb::Analyze => {
            let items = outcome["items"].as_array()?;
            let [item] = items.as_slice() else {
                return None;
            };
            let states = item["exploration"]["states"]
                .as_str()?
                .parse::<u64>()
                .ok()?;
            let transitions = item["exploration"]["transitions"]
                .as_str()?
                .parse::<u64>()
                .ok()?;
            (outcome["operation"] == "analyze"
                && outcome["category"] == "success"
                && item["claim_name"] == "WithinLimit"
                && item["record"]["result"] == "proved"
                && item["record"]["certification"] == "certified"
                && item["record"]["basis"]["type"] == "exhaustive"
                && item["certificate"]["accepted"] == true
                && item["axes"]["truth"] == "true"
                && item["axes"]["execution"] == "completed"
                && item["axes"]["settlement"] == "closed-scope"
                && states >= 4
                && transitions >= 3)
                .then(|| json!({"result":"proved","claim":item["request"]["claim"],"subject":item["request"]["temporal"]["subject"],"certificate":item["certificate"]["identity"]}))
        }
        Verb::Monitor => None,
    }
}

fn monitor_result(bytes: &[u8]) -> Option<Value> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut events = Vec::new();
    let mut summary = None;
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        if summary.is_some() {
            return None;
        }
        let row: Value = serde_json::from_str(line).ok()?;
        match row["format"].as_str()? {
            "filament.monitor-event/1" => events.push(row),
            "filament.monitor-summary/1" => summary = Some(row),
            _ => return None,
        }
    }
    let summary = summary?;
    let clauses = summary["clauses"].as_array()?;
    let [clause] = clauses.as_slice() else {
        return None;
    };
    let decisive: Vec<&Value> = events
        .iter()
        .filter(|event| event["settled"] == true)
        .collect();
    let [event] = decisive.as_slice() else {
        return None;
    };
    let entries = summary["ledger"]["entries"].as_array()?;
    let [entry] = entries.as_slice() else {
        return None;
    };
    let counterexample = &clause["disposition"]["temporal"]["counterexample"];
    (summary["records"] == 5
        && summary["refused"] == 0
        && events.first()?["category"] == "pending"
        && events.first()?["position"] == 0
        && events.iter().all(|row| row["clause"] == "WithinThree")
        && clause["clause"] == "WithinThree"
        && clause["category"] == "violation"
        && clause["settled"] == true
        && clause["basis"] == "closed-scope"
        && clause["disposition"]["temporal"]["kind"] == "violated"
        && clause["disposition"]["truth"] == false
        && clause["settlement"]["basis"] == "decisive-counterexample"
        && clause["settlement"]["horizon"] == "2"
        && clause["settlement"]["position"] == "0"
        && event["position"] == 2
        && event["record"] == 3
        && event["category"] == "violation"
        && event["disposition"] == clause["disposition"]
        && event["settlement"] == clause["settlement"]
        && counterexample["trace"] == "observed"
        && counterexample["trace_position"] == 0
        && counterexample["interval"] == json!({"lower":"0","upper":"2"})
        && counterexample["prefix"] == json!([{"sample":"0"},{"sample":"1"},{"sample":"2"}])
        && entry["binding"]["package"] == "eval:blind-help/monitor"
        && entry["binding"]["clock"] == "stock-events"
        && entry["binding"]["profile"] == "filament.temporal.event-position.false-extension/v1"
        && entry["watermark"] == 2
        && entry["scope"] == "closed"
        && entry["execution"] == "closed"
        && entry["completeness"]["complete"]["through"] == 2)
        .then(|| json!({"events":events,"summary":summary}))
}

fn semantic_artifact(verb: Verb, bytes: &[u8]) -> Option<Value> {
    match verb {
        Verb::Monitor => monitor_result(bytes),
        Verb::Check | Verb::Run | Verb::Analyze => {
            semantic_result(verb, &serde_json::from_slice::<Value>(bytes).ok()?)
        }
    }
}

fn public_outcome(workspace: &Path, tools: &Tools, verb: Verb) -> ScenarioResult<(i32, Vec<u8>)> {
    let mut command = Command::new(&tools.fsl);
    command
        .current_dir(workspace)
        .args([verb.name(), verb.operand()]);
    if matches!(verb, Verb::Monitor) {
        command.args(["--input", "readings.jsonl"]);
    }
    let result = command.args(["--format", "json"]).output()?;
    let exit = result
        .status
        .code()
        .ok_or("public FSL command was signalled")?;
    Ok((exit, result.stdout))
}

fn grade_case(workspace: &Path, tools: &Tools, verb: Verb) -> ScenarioResult<(bool, Value)> {
    let (exit, actual) = public_outcome(workspace, tools, verb)?;
    let expected = semantic_artifact(verb, &actual);
    let saved = regular_file(&workspace.join(verb.result_file()))
        .ok()
        .and_then(|bytes| semantic_artifact(verb, &bytes));
    let pass = exit == verb.expected_exit() && expected.is_some() && saved == expected;
    Ok((
        pass,
        json!({"pass":pass,"trusted_exit":exit,"semantic_result":expected}),
    ))
}

/// Grade each verb's actual public result and saved semantic artifact independently.
pub fn grade(workspace: &Path, tools: &Tools) -> ScenarioResult<Assertion> {
    let mut failures = Vec::new();
    let mut checks = BTreeMap::new();
    let changed = immutable_inputs(workspace)?;
    checks.insert("changed_inputs".to_owned(), json!(changed));
    if !changed.is_empty() {
        return Ok(Assertion {
            ok: false,
            checks,
            failures: vec!["original problem inputs were changed or replaced".to_owned()],
        });
    }
    let mut per_verb = serde_json::Map::new();
    for &verb in Verb::ALL {
        let (pass, detail) = grade_case(workspace, tools, verb)?;
        per_verb.insert(verb.name().to_owned(), detail);
        if !pass {
            failures.push(format!(
                "{} did not produce the required independently checked result",
                verb.name()
            ));
        }
    }
    checks.insert("per_verb".to_owned(), Value::Object(per_verb));
    checks.insert(
        "pending_verbs".to_owned(),
        json!({"export":{"owner":"QSL-1979","status":"pending"}}),
    );
    checks.insert("program_status".to_owned(), json!("partial/export-pending"));
    checks.insert("overall_completion".to_owned(), json!("pending"));
    Ok(Assertion {
        ok: failures.is_empty(),
        checks,
        failures,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tools() -> ScenarioResult<Tools> {
        Ok(Tools {
            fsl: std::env::var_os("FSL")
                .ok_or("set FSL to the accepted public executable")?
                .into(),
            docs: ".".into(),
            quire: "quire".into(),
            quoin: "quoin".into(),
        })
    }

    fn reference_case(verb: Verb) -> ScenarioResult<Vec<u8>> {
        let tools = tools()?;
        let workspace = tempfile::tempdir()?;
        prepare(workspace.path(), &tools)?;
        let (exit, bytes) = public_outcome(workspace.path(), &tools, verb)?;
        assert_eq!(
            exit,
            verb.expected_exit(),
            "{}: {}",
            verb.name(),
            String::from_utf8_lossy(&bytes)
        );
        assert!(
            semantic_artifact(verb, &bytes).is_some(),
            "{}: {}",
            verb.name(),
            String::from_utf8_lossy(&bytes)
        );
        fs::write(workspace.path().join(verb.result_file()), &bytes)?;
        let (pass, observed) = grade_case(workspace.path(), &tools, verb)?;
        assert!(pass, "{}: {observed}", verb.name());
        assert!(semantic_artifact(verb, b"FSL-EVAL-DONE").is_none());
        Ok(bytes)
    }

    /// Provenance: QSL-2493
    #[test]
    #[ignore = "EV-009 check grader with the accepted public FSL executable"]
    fn check_reference_rejects_a_refusal_result() -> ScenarioResult<()> {
        let mut altered: Value = serde_json::from_slice(&reference_case(Verb::Check)?)?;
        altered["category"] = json!("refusal");
        assert!(semantic_result(Verb::Check, &altered).is_none());
        Ok(())
    }

    /// Provenance: QSL-2493
    #[test]
    #[ignore = "EV-009 run grader with the accepted public FSL executable"]
    fn run_reference_rejects_a_different_quote() -> ScenarioResult<()> {
        let mut altered: Value = serde_json::from_slice(&reference_case(Verb::Run)?)?;
        altered["result"]["value"]["integer"] = json!("28");
        assert!(semantic_result(Verb::Run, &altered).is_none());
        Ok(())
    }

    /// Provenance: QSL-2493
    #[test]
    #[ignore = "EV-009 analyze grader with the accepted public FSL executable"]
    fn analyze_reference_rejects_a_different_claim() -> ScenarioResult<()> {
        let mut altered: Value = serde_json::from_slice(&reference_case(Verb::Analyze)?)?;
        altered["items"][0]["claim_name"] = json!("OtherClaim");
        assert!(semantic_result(Verb::Analyze, &altered).is_none());
        Ok(())
    }

    /// Provenance: QSL-2493
    #[test]
    #[ignore = "EV-009 monitor grader with the accepted public FSL executable"]
    fn monitor_reference_rejects_a_different_observed_position() -> ScenarioResult<()> {
        let text = String::from_utf8(reference_case(Verb::Monitor)?)?;
        let mut changed = false;
        let mut altered = Vec::new();
        for line in text.lines().filter(|line| !line.trim().is_empty()) {
            let mut row: Value = serde_json::from_str(line)?;
            if row["format"] == "filament.monitor-event/1" && row["settled"] == true {
                row["position"] = json!(1);
                changed = true;
            }
            altered.extend(serde_json::to_vec(&row)?);
            altered.push(b'\n');
        }
        assert!(changed, "reference must contain a decisive monitor event");
        assert!(monitor_result(&altered).is_none());
        assert!(monitor_result(b"{\"kind\":\"violated\"}\n").is_none());
        Ok(())
    }

    /// Provenance: QSL-2493
    #[test]
    fn changed_or_linked_problem_inputs_fail_before_public_execution() -> ScenarioResult<()> {
        let workspace = tempfile::tempdir()?;
        let tools = Tools {
            fsl: "/not-a-real-driver".into(),
            docs: ".".into(),
            quire: "quire".into(),
            quoin: "quoin".into(),
        };
        prepare(workspace.path(), &tools)?;
        fs::write(workspace.path().join("calculation.fsl"), "true")?;
        let result = grade(workspace.path(), &tools)?;
        assert!(!result.ok);
        assert_eq!(result.checks["changed_inputs"], json!(["calculation.fsl"]));
        fs::write(workspace.path().join("calculation.fsl"), INPUTS[0].1)?;
        fs::remove_file(workspace.path().join("initial.json"))?;
        std::os::unix::fs::symlink(
            workspace.path().join("run-request.json"),
            workspace.path().join("initial.json"),
        )?;
        let result = grade(workspace.path(), &tools)?;
        assert!(!result.ok);
        assert_eq!(result.checks["changed_inputs"], json!(["initial.json"]));
        Ok(())
    }

    fn synthetic_monitor_rows() -> Vec<Value> {
        let settlement = json!({"basis":"decisive-counterexample","horizon":"2","position":"0","trigger":"decided"});
        let disposition = json!({"category":"violation","stage":"evaluate","truth":false,
            "temporal":{"kind":"violated","position":0,"counterexample":{
                "trace":"observed","trace_position":0,"kind":"formula",
                "interval":{"lower":"0","upper":"2"},
                "prefix":[{"sample":"0"},{"sample":"1"},{"sample":"2"}],"loop":[]}}});
        let mut rows: Vec<Value> = (0..3)
            .map(|position| {
                json!({
                    "format":"filament.monitor-event/1","clause":"WithinThree",
                    "position":position,"record":position,"category":"pending","settled":false,
                    "disposition":{"category":"pending","temporal":{"kind":"pending"}},
                    "basis":"unsettled"
                })
            })
            .collect();
        rows.push(
            json!({"format":"filament.monitor-event/1","clause":"WithinThree",
            "position":2,"record":3,"category":"violation","settled":true,
            "disposition":disposition,"settlement":settlement}),
        );
        rows.push(json!({"format":"filament.monitor-summary/1","records":5,"refused":0,
            "clauses":[{"clause":"WithinThree","category":"violation","settled":true,
                "basis":"closed-scope","disposition":disposition,"settlement":settlement}],
            "ledger":{"format":"filament.progress-ledger/1","entries":[{
                "binding":{"package":"eval:blind-help/monitor","clock":"stock-events",
                    "profile":"filament.temporal.event-position.false-extension/v1"},
                "watermark":2,"scope":"closed","execution":"closed",
                "completeness":{"complete":{"through":2}},
                "covered":[{"stamp":0,"fingerprint":"sample-0"},{"stamp":1,"fingerprint":"sample-1"},{"stamp":2,"fingerprint":"sample-2"}]}]}}));
        rows
    }

    fn encode_rows(rows: &[Value]) -> ScenarioResult<Vec<u8>> {
        let mut bytes = Vec::new();
        for row in rows {
            serde_json::to_writer(&mut bytes, row)?;
            bytes.push(b'\n');
        }
        Ok(bytes)
    }

    /// Provenance: QSL-2493
    #[test]
    fn complete_monitor_stream_ignores_only_json_formatting() -> ScenarioResult<()> {
        let rows = synthetic_monitor_rows();
        let original = encode_rows(&rows)?;
        let expected =
            monitor_result(&original).ok_or("synthetic wire fixture must be admitted")?;
        let spaced = String::from_utf8(original)?
            .lines()
            .map(|line| format!(" \t{line}  \n"))
            .collect::<String>();
        assert_eq!(monitor_result(spaced.as_bytes()), Some(expected));
        Ok(())
    }

    /// Provenance: QSL-2493
    #[test]
    fn complete_monitor_stream_rejects_missing_added_and_changed_pending_events()
    -> ScenarioResult<()> {
        let rows = synthetic_monitor_rows();
        let expected = monitor_result(&encode_rows(&rows)?)
            .ok_or("synthetic wire fixture must be admitted")?;
        let mut omitted = rows.clone();
        omitted.remove(1);
        let mut added = rows.clone();
        added.insert(2, rows[1].clone());
        let mut wrong_position = rows.clone();
        wrong_position[1]["position"] = json!(2);
        let mut wrong_record = rows.clone();
        wrong_record[1]["record"] = json!(9);
        let mut wrong_result = rows.clone();
        wrong_result[1]["disposition"]["temporal"]["kind"] = json!("holds");
        let mut wrong_binding = rows.clone();
        wrong_binding[4]["ledger"]["entries"][0]["binding"]["package"] = json!("another-input");
        for (name, altered) in [
            ("omitted pending event", omitted),
            ("additional pending event", added),
            ("changed pending position", wrong_position),
            ("changed pending record", wrong_record),
            ("changed pending result", wrong_result),
            ("changed current input binding", wrong_binding),
        ] {
            assert_ne!(
                monitor_result(&encode_rows(&altered)?),
                Some(expected.clone()),
                "{name} must not match the complete actual stream"
            );
        }
        Ok(())
    }
}
