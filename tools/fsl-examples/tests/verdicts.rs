//! Every analysis engine family is gated by a verdict that holds and a verdict that fails.
//!
//! `index.json` tags each verdict case with its `engine` (and, for a timed case, its `clock`).
//! Each engine needs a case that passes (exit 0) and a case that fails (exit 10). A running case
//! counts by its exit; a staged case counts by its `ready_exit`, so an engine whose cases all wait
//! on a ticket still names both verdicts it will gate. `make check` compares every running case's
//! outcome with its record by structure, so a flipped verdict fails it; the checks below make
//! sure the records hold the fields that make a flip visible: the counterexample's witness for
//! model checking and protocols, the refusal cause for a refinement that does not hold, the
//! threshold for probability, and the clock for timed rules.

use fsl_examples::manifest::ENGINES;
use serde_json::Value;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

const PASS: i64 = 0;
const FAIL: i64 = 10;

/// One tagged entry of `index.json`.
struct Entry {
    label: String,
    running: bool,
    engine: String,
    clock: Option<String>,
    rule: Option<String>,
    rule_file: String,
    expected_file: String,
    /// The exit the verdict gives: the recorded exit when running, the ready exit when staged.
    verdict_exit: i64,
}

fn text(entry: &Value, key: &str) -> Option<String> {
    entry.get(key).and_then(Value::as_str).map(str::to_owned)
}

fn tagged_entries() -> Result<Vec<Entry>, Box<dyn std::error::Error>> {
    let index: Value =
        serde_json::from_str(&std::fs::read_to_string(repo_root().join("index.json"))?)?;
    let cases = index
        .get("cases")
        .and_then(Value::as_array)
        .ok_or("index.json has no cases")?;
    let mut entries = Vec::new();
    for case in cases {
        let Some(engine) = text(case, "engine") else {
            continue;
        };
        let label = format!(
            "{}/{}",
            text(case, "system").unwrap_or_default(),
            text(case, "case").unwrap_or_default()
        );
        let running = text(case, "status").as_deref() == Some("running");
        let exit_key = if running { "exit" } else { "ready_exit" };
        let verdict_exit = case
            .get(exit_key)
            .and_then(Value::as_i64)
            .ok_or_else(|| format!("{label}: no {exit_key}"))?;
        entries.push(Entry {
            label,
            running,
            engine,
            clock: text(case, "clock"),
            rule: text(case, "rule"),
            rule_file: text(case, "rule_file").unwrap_or_default(),
            expected_file: text(case, "expected_file").unwrap_or_default(),
            verdict_exit,
        });
    }
    Ok(entries)
}

fn record(entry: &Entry) -> Result<Value, Box<dyn std::error::Error>> {
    let path = repo_root().join(&entry.expected_file);
    let text = std::fs::read_to_string(&path)
        .map_err(|e| format!("{}: {}: {e}", entry.label, path.display()))?;
    Ok(serde_json::from_str(&text)?)
}

/// The outcome items of a record, or the clauses of a monitor summary.
fn verdicts(record: &Value) -> Vec<&Value> {
    let items = record.get("items").and_then(Value::as_array);
    let clauses = record
        .get("summary")
        .and_then(|s| s.get("clauses"))
        .and_then(Value::as_array);
    items
        .or(clauses)
        .map(|v| v.iter().collect())
        .unwrap_or_default()
}

/// The counterexample of an outcome item or of a monitor clause.
fn counterexample(verdict: &Value) -> Option<&Value> {
    verdict
        .get("counterexample")
        .or_else(|| {
            verdict
                .get("disposition")
                .and_then(|d| d.get("temporal"))
                .and_then(|t| t.get("counterexample"))
        })
        .filter(|c| !c.is_null())
}

/// Whether `value` holds the member `key` anywhere below it.
fn has_member(value: &Value, key: &str) -> bool {
    match value {
        Value::Object(map) => map.contains_key(key) || map.values().any(|v| has_member(v, key)),
        Value::Array(items) => items.iter().any(|v| has_member(v, key)),
        _ => false,
    }
}

fn non_empty_array(value: Option<&Value>) -> bool {
    value
        .and_then(Value::as_array)
        .is_some_and(|a| !a.is_empty())
}

#[test]
fn every_engine_has_a_passing_and_a_failing_case() -> TestResult {
    let entries = tagged_entries()?;
    let mut missing = Vec::new();
    for engine in ENGINES {
        for (exit, verdict) in [(PASS, "passing"), (FAIL, "failing")] {
            if !entries
                .iter()
                .any(|e| e.engine == *engine && e.verdict_exit == exit)
            {
                missing.push(format!("{engine}: no {verdict} case (exit {exit})"));
            }
        }
    }
    assert!(missing.is_empty(), "{}", missing.join("\n"));
    let unknown: Vec<&str> = entries
        .iter()
        .filter(|e| !ENGINES.contains(&e.engine.as_str()))
        .map(|e| e.label.as_str())
        .collect();
    assert!(unknown.is_empty(), "unknown engine on {unknown:?}");
    Ok(())
}

/// A verdict case passes with exit 0 or fails with exit 10; any other exit is not a verdict.
#[test]
fn every_running_verdict_case_exits_with_a_verdict() -> TestResult {
    for entry in tagged_entries()? {
        assert!(
            entry.verdict_exit == PASS || entry.verdict_exit == FAIL,
            "{}: exit {} is not a verdict",
            entry.label,
            entry.verdict_exit
        );
    }
    Ok(())
}

#[test]
fn each_timed_clock_has_a_satisfied_and_a_violated_bound() -> TestResult {
    let entries = tagged_entries()?;
    let timed: Vec<&Entry> = entries.iter().filter(|e| e.engine == "timed").collect();
    let clocks: BTreeSet<&str> = timed.iter().filter_map(|e| e.clock.as_deref()).collect();
    assert!(clocks.len() >= 2, "timed cases name clocks {clocks:?}");
    for clock in &clocks {
        for exit in [PASS, FAIL] {
            assert!(
                timed.iter().any(|e| e.running
                    && e.clock.as_deref() == Some(clock)
                    && e.verdict_exit == exit),
                "clock {clock}: no running timed case with exit {exit}"
            );
        }
    }
    for entry in &timed {
        let clock = entry
            .clock
            .as_deref()
            .ok_or("a timed case names its clock")?;
        let source = std::fs::read_to_string(repo_root().join(&entry.rule_file))?;
        assert!(
            source.contains(&format!("clock \"{clock}\"")),
            "{}: {} declares no clock \"{clock}\"",
            entry.label,
            entry.rule_file
        );
        if !entry.running {
            continue;
        }
        // A monitor stream records the clock each settled clause was counted on.
        let record = record(entry)?;
        if let Some(entries) = record
            .get("summary")
            .and_then(|s| s.get("ledger"))
            .and_then(|l| l.get("entries"))
            .and_then(Value::as_array)
        {
            for ledger in entries {
                assert_eq!(
                    ledger.get("binding").and_then(|b| b.get("clock")),
                    Some(&Value::from(clock)),
                    "{}: the ledger names another clock",
                    entry.label
                );
            }
        }
        if entry.verdict_exit == FAIL {
            assert!(
                verdicts(&record)
                    .iter()
                    .any(|v| counterexample(v).is_some()),
                "{}: a violated bound records its counterexample",
                entry.label
            );
        }
    }
    Ok(())
}

#[test]
fn the_monitor_gates_safe_stop_within_three_steps() -> TestResult {
    let entries = tagged_entries()?;
    for exit in [PASS, FAIL] {
        assert!(
            entries.iter().any(|e| e.engine == "monitor"
                && e.running
                && e.verdict_exit == exit
                && e.rule.as_deref() == Some("SafeStopWithin3Steps")),
            "no running SafeStopWithin3Steps monitor case with exit {exit}"
        );
    }
    for entry in entries
        .iter()
        .filter(|e| e.engine == "monitor" && e.running && e.verdict_exit == FAIL)
    {
        let record = record(entry)?;
        assert!(
            verdicts(&record).iter().any(|v| {
                v.get("category").and_then(Value::as_str) == Some("violation")
                    && counterexample(v).is_some_and(|c| non_empty_array(c.get("prefix")))
            }),
            "{}: the violated clause records the observed prefix",
            entry.label
        );
    }
    Ok(())
}

/// The fields of a failing case's record that make its verdict checkable, per engine: a model
/// checking counterexample names its kind and the states of its prefix or loop, a refinement
/// failure its kind and cause, a protocol counterexample its blocked threads or the violated
/// obligations of a recording with their cause, and a missed probability claim its threshold.
#[test]
fn failing_records_carry_their_witness_or_cause() -> TestResult {
    for entry in tagged_entries()?
        .iter()
        .filter(|e| e.running && e.verdict_exit == FAIL)
    {
        let record = record(entry)?;
        let found = verdicts(&record);
        let witnessed = match entry.engine.as_str() {
            "model-checking" => found.iter().any(|v| {
                counterexample(v).is_some_and(|c| {
                    c.get("kind").and_then(Value::as_str).is_some()
                        && (non_empty_array(c.get("prefix")) || non_empty_array(c.get("loop")))
                })
            }),
            "refinement" => found.iter().any(|v| {
                counterexample(v)
                    .and_then(|c| c.get("refinement_failure"))
                    .is_some_and(|f| {
                        f.get("kind").and_then(Value::as_str).is_some()
                            && f.get("cause").is_some_and(|c| !c.is_null())
                    })
            }),
            "protocol" => found.iter().any(|v| {
                counterexample(v).is_some_and(|c| non_empty_array(c.get("blocked")))
                    || v.get("obligations")
                        .and_then(Value::as_array)
                        .is_some_and(|o| {
                            o.iter().any(|ob| {
                                ob.get("verdict").and_then(Value::as_str) == Some("violated")
                                    && ob.get("cause").is_some_and(|c| !c.is_null())
                            })
                        })
            }),
            "probability" => found.iter().any(|v| has_member(v, "threshold")),
            _ => found.iter().any(|v| counterexample(v).is_some()),
        };
        assert!(
            witnessed,
            "{}: the {} record lacks the fields that make its failure checkable",
            entry.label, entry.engine
        );
    }
    Ok(())
}

/// A passing case's record settles every verdict without a counterexample, and a running
/// probability case records the threshold it compared against.
#[test]
fn passing_records_settle_without_a_counterexample() -> TestResult {
    for entry in tagged_entries()?
        .iter()
        .filter(|e| e.running && e.verdict_exit == PASS)
    {
        let record = record(entry)?;
        let found = verdicts(&record);
        assert!(!found.is_empty(), "{}: no verdict recorded", entry.label);
        for verdict in &found {
            assert!(
                counterexample(verdict).is_none(),
                "{}: a passing verdict carries a counterexample",
                entry.label
            );
            assert_ne!(
                verdict.get("category").and_then(Value::as_str),
                Some("violation"),
                "{}",
                entry.label
            );
        }
        if entry.engine == "probability" {
            assert!(
                found.iter().any(|v| has_member(v, "threshold")),
                "{}: a probability record names its threshold",
                entry.label
            );
        }
    }
    Ok(())
}
