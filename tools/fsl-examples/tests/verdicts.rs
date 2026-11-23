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
use fsl_examples::runner::{absolute, load_all};
use serde_json::Value;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

const PASS: i64 = 0;
const FAIL: i64 = 10;
/// The exit of an inconclusive answer, such as a sampled claim whose measured value fell in the
/// indifference region.
const INCONCLUSIVE: i64 = 22;
/// The exit of a claim the engine refuses before it analyzes anything.
const UNSUPPORTED: i64 = 21;

/// One tagged entry of `index.json`.
struct Entry {
    label: String,
    running: bool,
    engine: String,
    clock: Option<String>,
    rule: Option<String>,
    rule_file: String,
    expected_file: String,
    request_file: String,
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
            request_file: text(case, "request_file").unwrap_or_default(),
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

/// The schema id of the request a case sends, such as `native-compile/1`.
fn request_format(entry: &Entry) -> Result<String, Box<dyn std::error::Error>> {
    let path = repo_root().join(&entry.request_file);
    let request: Value = serde_json::from_str(
        &std::fs::read_to_string(&path)
            .map_err(|e| format!("{}: {}: {e}", entry.label, path.display()))?,
    )?;
    Ok(text(&request, "format").unwrap_or_default())
}

/// The exit of a record whose outcome category is not a verdict: the tool's defined answer for a
/// claim it could not decide (inconclusive) or does not support (unsupported).
fn non_verdict_exit(record: &Value) -> Option<i64> {
    match record.get("category").and_then(Value::as_str) {
        Some("inconclusive") => Some(INCONCLUSIVE),
        Some("unsupported") => Some(UNSUPPORTED),
        _ => None,
    }
}

/// Whether a case only compiles. A compile request states no claim, so its record has no verdict
/// members and its exit 0 says the package compiled, not that a claim holds.
fn is_compile_only(entry: &Entry) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(request_format(entry)? == "native-compile/1")
}

/// Whether an outcome item is the measurement of a sampled claim: its record says it was measured.
fn is_sampled(verdict: &Value) -> bool {
    verdict
        .get("record")
        .and_then(|r| r.get("result"))
        .and_then(Value::as_str)
        == Some("measured")
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

/// The arms of a possible claim's graph evidence the documents use.
const GRAPH_ARMS: [&str; 3] = ["trap", "path-pair", "witness"];

/// A decimal count the documents print as a string or a number.
fn count(value: Option<&Value>) -> Option<u64> {
    match value? {
        Value::String(s) => s.parse().ok(),
        Value::Number(n) => n.as_u64(),
        _ => None,
    }
}

/// A refuted possible claim has no trace: its counterexample is evidence from the explored state
/// graph, the arm of the claim it refutes and the path from the initial state, and the item
/// records the size of the graph it explored and that replaying the evidence reproduced it.
fn graph_evidence_is_replayed(item: &Value, counterexample: &Value) -> bool {
    counterexample.get("type").and_then(Value::as_str) == Some("graph-evidence")
        && counterexample
            .get("arm")
            .and_then(Value::as_str)
            .is_some_and(|arm| GRAPH_ARMS.contains(&arm))
        && count(counterexample.get("stem").and_then(|s| s.get("initial"))).is_some()
        && count(item.get("exploration").and_then(|e| e.get("states"))).is_some_and(|n| n > 0)
        && item
            .get("replay")
            .and_then(|r| r.get("reproduced"))
            .and_then(Value::as_bool)
            == Some(true)
}

fn non_empty_array(value: Option<&Value>) -> bool {
    value
        .and_then(Value::as_array)
        .is_some_and(|a| !a.is_empty())
}

/// A non-negative integer of any size, as little-endian decimal digits.
struct Nat(Vec<u8>);

impl Nat {
    fn parse(text: &str) -> Option<Nat> {
        if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let mut digits: Vec<u8> = text.bytes().rev().map(|b| b - b'0').collect();
        while digits.last() == Some(&0) {
            digits.pop();
        }
        Some(Nat(digits))
    }

    fn is_zero(&self) -> bool {
        self.0.is_empty()
    }

    fn mul(&self, other: &Nat) -> Nat {
        let mut sums = vec![0u32; self.0.len() + other.0.len() + 1];
        for (i, a) in self.0.iter().enumerate() {
            for (j, b) in other.0.iter().enumerate() {
                sums[i + j] += u32::from(*a) * u32::from(*b);
            }
        }
        let mut carry = 0u32;
        let mut digits = Vec::with_capacity(sums.len());
        for sum in sums {
            let total = sum + carry;
            digits.push((total % 10) as u8);
            carry = total / 10;
        }
        while digits.last() == Some(&0) {
            digits.pop();
        }
        Nat(digits)
    }

    fn cmp(&self, other: &Nat) -> Ordering {
        self.0
            .len()
            .cmp(&other.0.len())
            .then_with(|| self.0.iter().rev().cmp(other.0.iter().rev()))
    }
}

/// A rational number with a non-zero denominator, compared exactly.
struct Ratio {
    num: Nat,
    den: Nat,
}

impl Ratio {
    /// A rational written as `{"numerator": "1", "denominator": "2"}`.
    fn from_members(value: &Value) -> Option<Ratio> {
        let num = Nat::parse(value.get("numerator")?.as_str()?)?;
        let den = Nat::parse(value.get("denominator")?.as_str()?)?;
        (!den.is_zero()).then_some(Ratio { num, den })
    }

    /// A rational written as `p/q` or as an integer.
    fn from_text(text: &str) -> Option<Ratio> {
        let (num, den) = text.split_once('/').unwrap_or((text, "1"));
        let den = Nat::parse(den)?;
        (!den.is_zero()).then_some(Ratio {
            num: Nat::parse(num)?,
            den,
        })
    }

    fn cmp(&self, other: &Ratio) -> Ordering {
        self.num.mul(&other.den).cmp(&other.num.mul(&self.den))
    }
}

/// Which side of its bound a probability claim asks for.
#[derive(PartialEq)]
enum Direction {
    AtLeast,
    AtMost,
}

/// The bound a probability claim states, read from the claim node's path, where the document
/// prints it as the members `bound`, `direction`, a direction and `value`, a number kind and the
/// number (for example `at_least`, `rational`, `99/100`).
fn claim_bound(item: &Value) -> Option<(Direction, Ratio)> {
    let path = item
        .get("request")?
        .get("claim")?
        .get("node")?
        .get("path")?
        .as_array()?;
    let at = path.iter().position(|p| p.as_str() == Some("bound"))?;
    let word = |offset: usize| path.get(at + offset).and_then(Value::as_str);
    if word(1)? != "direction" || word(3)? != "value" || !matches!(word(4)?, "rational" | "integer")
    {
        return None;
    }
    let direction = match word(2)? {
        "at_least" => Direction::AtLeast,
        "at_most" => Direction::AtMost,
        _ => return None,
    };
    Some((direction, Ratio::from_text(word(5)?)?))
}

/// Checks a sampled claim against the decision it states. Interval methods must place the
/// estimate inside an interval on the correct side of the claim's bound. SPRT reports no interval,
/// so its estimate and sample counts must support the decision directly.
fn sampled_measurement_problem(item: &Value, decision: &str) -> Option<String> {
    let verdict = item.get("verdict")?;
    let stated = verdict
        .get("decision")
        .and_then(|d| d.get("type"))
        .and_then(Value::as_str);
    if stated != Some(decision) {
        return Some(format!("the decision is {stated:?}, not {decision}"));
    }
    let ratio = |value: Option<&Value>| value.and_then(Ratio::from_members);
    if verdict
        .get("basis")
        .and_then(|basis| basis.get("method"))
        .and_then(|method| method.get("type"))
        .and_then(Value::as_str)
        == Some("sprt")
    {
        if ratio(verdict.get("estimate").and_then(|e| e.get("value"))).is_none() {
            return Some("the SPRT estimate is not a rational".to_owned());
        }
        if verdict.get("interval").is_some() {
            return Some("SPRT unexpectedly reports an interval".to_owned());
        }
        let samples = count(verdict.get("basis").and_then(|b| b.get("samples")));
        let Some(tests) = verdict.get("tests").and_then(Value::as_array) else {
            return Some("the SPRT per-test results are not an array".to_owned());
        };
        let drawn: Option<u64> = tests.iter().map(|test| count(test.get("samples"))).sum();
        if tests.is_empty() || !samples.is_some_and(|n| n > 0) || samples != drawn {
            return Some(format!(
                "the SPRT samples {samples:?} are not positive and equal to the per-test count {drawn:?}"
            ));
        }
        if tests
            .iter()
            .any(|test| ratio(test.get("estimate").and_then(|e| e.get("value"))).is_none())
        {
            return Some("an SPRT test estimate is not a rational".to_owned());
        }
        return None;
    }
    let (Some(estimate), Some(lower), Some(upper)) = (
        ratio(verdict.get("estimate").and_then(|e| e.get("value"))),
        ratio(verdict.get("interval").and_then(|i| i.get("lower"))),
        ratio(verdict.get("interval").and_then(|i| i.get("upper"))),
    ) else {
        return Some("the estimate or its interval is not a rational".to_owned());
    };
    if lower.cmp(&estimate) == Ordering::Greater || estimate.cmp(&upper) == Ordering::Greater {
        return Some("the estimate is outside its interval".to_owned());
    }
    let samples = count(verdict.get("basis").and_then(|b| b.get("samples")));
    let drawn: Option<u64> = verdict
        .get("tests")
        .and_then(Value::as_array)
        .and_then(|tests| {
            tests
                .iter()
                .map(|t| count(t.get("samples")))
                .sum::<Option<u64>>()
        });
    if !samples.is_some_and(|n| n > 0) || samples != drawn {
        return Some(format!(
            "the samples {samples:?} are not a positive count equal to the tests' {drawn:?}"
        ));
    }
    let Some((direction, bound)) = claim_bound(item) else {
        return Some("the claim's bound is not readable from its node path".to_owned());
    };
    let holds = match (&direction, decision) {
        (Direction::AtLeast, "accepted") => lower.cmp(&bound) != Ordering::Less,
        (Direction::AtMost, "accepted") => upper.cmp(&bound) != Ordering::Greater,
        (Direction::AtLeast, _) => upper.cmp(&bound) == Ordering::Less,
        (Direction::AtMost, _) => lower.cmp(&bound) == Ordering::Greater,
    };
    (!holds).then(|| format!("the interval is not on the {decision} side of the claim's bound"))
}

/// Checks an exact proof: the value it computed satisfies the claim's bound in the stated
/// direction and the certificate was accepted.
fn exact_proof_problem(item: &Value) -> Option<String> {
    let Some(value) = item
        .get("record")
        .and_then(|r| r.get("basis"))
        .and_then(|b| b.get("value"))
        .and_then(Ratio::from_members)
    else {
        return Some("the proved value is not a rational".to_owned());
    };
    let Some((direction, bound)) = claim_bound(item) else {
        return Some("the claim's bound is not readable from its node path".to_owned());
    };
    let satisfied = match direction {
        Direction::AtLeast => value.cmp(&bound) != Ordering::Less,
        Direction::AtMost => value.cmp(&bound) != Ordering::Greater,
    };
    if !satisfied {
        return Some("the proved value does not satisfy the claim's bound".to_owned());
    }
    let accepted = item
        .get("certificate")
        .and_then(|c| c.get("accepted"))
        .and_then(Value::as_bool);
    (accepted != Some(true)).then(|| "the certificate was not accepted".to_owned())
}

/// What is wrong with a passing probability item that states no threshold member, if anything: a
/// sampled acceptance or an exact proof is read against the bound in the claim's path.
fn probability_pass_problem(item: &Value) -> Option<String> {
    match item
        .get("record")
        .and_then(|r| r.get("result"))
        .and_then(Value::as_str)
    {
        Some("measured") => sampled_measurement_problem(item, "accepted"),
        Some("proved") => exact_proof_problem(item),
        other => Some(format!("a passing item with result {other:?}")),
    }
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

/// Groups the tagged cases by the category of their record and checks that exit and category
/// agree: a running case whose record is `unsupported` exits 21, one whose record is
/// `inconclusive` exits 22, and every other case exits 0 or 10 (a staged case is held to its
/// ready exit). For the 21 and 22 cases it also checks that the manifest's `expect_exit` equals
/// the exit in `index.json`, so the manifest, the index and the record agree.
///
/// It does not pin which cases may be unsupported or inconclusive. Each case's `expect_exit` in
/// its example.toml pins that, and the runner's `check_systems` compares it with a fresh run of
/// the driver, so a case that changes category fails there and not here.
#[test]
fn every_running_verdict_case_exits_with_a_verdict() -> TestResult {
    let root = absolute(&repo_root())?;
    let systems = load_all(&root)?;
    for entry in tagged_entries()? {
        // A staged case names the exit it will give once fixed, so its record says nothing about it.
        if entry.running
            && let Some(exit) = non_verdict_exit(&record(&entry)?)
        {
            assert_eq!(
                entry.verdict_exit, exit,
                "{}: a record of this category exits {exit}",
                entry.label
            );
            let declared = systems
                .iter()
                .flat_map(|s| s.manifest.cases.iter().map(move |c| (s, c)))
                .find(|(s, c)| format!("{}/{}", s.name, c.id) == entry.label)
                .map(|(_, c)| i64::from(c.expect_exit))
                .ok_or_else(|| format!("{}: not in any manifest", entry.label))?;
            assert_eq!(
                declared, entry.verdict_exit,
                "{}: the manifest's expect_exit and the index exit differ",
                entry.label
            );
            continue;
        }
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
        let mut problems: Vec<String> = Vec::new();
        let witnessed = match entry.engine.as_str() {
            "model-checking" => found.iter().any(|v| {
                counterexample(v).is_some_and(|c| {
                    (c.get("kind").and_then(Value::as_str).is_some()
                        && (non_empty_array(c.get("prefix")) || non_empty_array(c.get("loop"))))
                        || graph_evidence_is_replayed(v, c)
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
            "probability" => found.iter().any(|v| {
                if is_sampled(v) {
                    let problem = sampled_measurement_problem(v, "rejected");
                    problems.extend(problem.clone());
                    problem.is_none()
                } else {
                    has_member(v, "threshold")
                }
            }),
            _ => found.iter().any(|v| counterexample(v).is_some()),
        };
        assert!(
            witnessed,
            "{}: the {} record lacks the fields that make its failure checkable: {problems:?}",
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
        if is_compile_only(entry)? {
            // A compilation states no verdict: it records success and no member to settle.
            assert!(
                found.is_empty(),
                "{}: a compile case records no verdict",
                entry.label
            );
            assert_eq!(
                record.get("category").and_then(Value::as_str),
                Some("success"),
                "{}: a compile case that exits 0 records success",
                entry.label
            );
            continue;
        }
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
            for verdict in &found {
                if has_member(verdict, "threshold") {
                    continue;
                }
                let problem = probability_pass_problem(verdict);
                assert!(
                    problem.is_none(),
                    "{}: a probability record names no threshold and does not settle against the claim's bound: {problem:?}",
                    entry.label
                );
            }
        }
    }
    Ok(())
}
