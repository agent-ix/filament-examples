//! The structure of a `filament-outcome/1` document, and the comparison of a run with its record.
//!
//! A case is recorded in two files. `expected/<id>.json` holds the structure: the category, the
//! result value, the last stage, the code and cause of the first diagnostic, and the items with
//! their witness fields. `expected/<id>.messages.txt` holds the diagnostic message text. The gate
//! compares the structure only, so rewording a message never fails a case.

use crate::manifest::{Case, Status};
use serde_json::{Map, Value, json};

/// The format name of the driver's outcome documents.
pub const OUTCOME_FORMAT: &str = "filament-outcome/1";

/// What one driver run printed, split into the compared structure and the message text.
#[derive(Debug, Clone, PartialEq)]
pub struct Recorded {
    /// The compared fields. Output that is not an outcome document is `{"outcome": null}`.
    pub structure: Value,
    /// The diagnostic messages, one per line; the raw output when it is not an outcome document.
    pub messages: String,
}

/// Splits driver output into structure and message text.
pub fn project(stdout: &str) -> Recorded {
    if let Some(structure) = monitor_stream(stdout) {
        return Recorded {
            structure,
            messages: String::new(),
        };
    }
    let doc = match serde_json::from_str::<Value>(stdout) {
        Ok(Value::Object(doc))
            if doc.get("format").and_then(Value::as_str) == Some(OUTCOME_FORMAT) =>
        {
            doc
        }
        _ => {
            return Recorded {
                structure: json!({ "outcome": null }),
                messages: stdout.to_owned(),
            };
        }
    };
    let field = |key: &str| doc.get(key).cloned().unwrap_or(Value::Null);
    let diagnostics = doc
        .get("diagnostics")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or_default();
    let first_diagnostic = match diagnostics.first() {
        Some(d) => json!({
            "code": d.get("code").cloned().unwrap_or(Value::Null),
            "cause": d.get("cause").cloned().unwrap_or(Value::Null),
        }),
        None => Value::Null,
    };
    let mut messages = String::new();
    for d in diagnostics {
        if let Some(text) = d.get("message").and_then(Value::as_str) {
            messages.push_str(text);
            messages.push('\n');
        }
    }
    let items = match doc.get("items") {
        Some(Value::Array(items)) => Value::Array(items.clone()),
        _ => Value::Array(Vec::new()),
    };
    let mut structure = Map::new();
    structure.insert("category".to_owned(), field("category"));
    structure.insert("last_stage".to_owned(), field("last_stage"));
    structure.insert("result".to_owned(), field("result"));
    structure.insert("first_diagnostic".to_owned(), first_diagnostic);
    structure.insert("items".to_owned(), items);
    Recorded {
        structure: Value::Object(structure),
        messages,
    }
}

/// The structure of `fsl monitor` output: JSON lines holding `filament.monitor-event/1` objects
/// and ending in one `filament.monitor-summary/1`. `None` when the output is not such a stream.
fn monitor_stream(stdout: &str) -> Option<Value> {
    let mut lines: Vec<Value> = stdout
        .lines()
        .map(serde_json::from_str::<Value>)
        .collect::<Result<_, _>>()
        .ok()?;
    let summary = lines.pop()?;
    let format_of = |v: &Value| v.get("format").and_then(Value::as_str).map(str::to_owned);
    if format_of(&summary).as_deref() != Some("filament.monitor-summary/1")
        || !lines
            .iter()
            .all(|e| format_of(e).as_deref() == Some("filament.monitor-event/1"))
    {
        return None;
    }
    Some(json!({ "events": lines, "summary": summary }))
}

/// Whether a structure came from an outcome document.
pub fn is_outcome(structure: &Value) -> bool {
    !matches!(structure.get("outcome"), Some(Value::Null))
}

/// The short verdict of a structure, as the manifest's `expect_verdict` spells it.
pub fn verdict_of(structure: &Value) -> String {
    if !is_outcome(structure) {
        return "unreadable output".to_owned();
    }
    if let Some(clauses) = structure
        .get("summary")
        .and_then(|s| s.get("clauses"))
        .and_then(Value::as_array)
    {
        let mut categories: Vec<String> = Vec::new();
        for clause in clauses {
            let category = display(clause.get("category").unwrap_or(&Value::Null));
            if !categories.contains(&category) {
                categories.push(category);
            }
        }
        return categories.join(", ");
    }
    let category = structure.get("category").unwrap_or(&Value::Null);
    if category.as_str() == Some("violation") {
        return "violated".to_owned();
    }
    if category.as_str() == Some("success") {
        let value = structure
            .get("result")
            .and_then(|r| r.get("value"))
            .filter(|v| truthy(v));
        return match value {
            Some(v) => [v.get("value"), v.get("decimal")]
                .into_iter()
                .flatten()
                .find(|x| !x.is_null())
                .map_or_else(|| "holds".to_owned(), display),
            None => "holds".to_owned(),
        };
    }
    match structure.get("first_diagnostic").filter(|d| !d.is_null()) {
        Some(d) => format!(
            "{}: {}",
            display(category),
            display(d.get("code").unwrap_or(&Value::Null))
        ),
        None => display(category),
    }
}

/// A value as JavaScript's `String()` would print it: strings bare, everything else as JSON.
pub fn display(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

/// JavaScript truthiness of a JSON value.
pub fn truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().is_some_and(|f| f != 0.0),
        Value::String(s) => !s.is_empty(),
        Value::Array(_) | Value::Object(_) => true,
    }
}

/// One driver run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Run {
    /// The exit code; -1 when the driver was stopped by a signal.
    pub exit: i32,
    /// What the driver printed on standard output.
    pub stdout: String,
}

/// The verdict on one case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evaluation {
    /// Whether the case passed.
    pub ok: bool,
    /// The line to print.
    pub message: String,
}

/// Compares one run with the manifest entry and the recorded structure (`None` when the
/// `expected/` file is missing). `label` names the case in the message.
pub fn evaluate_case(case: &Case, run: &Run, expected: Option<&Value>, label: &str) -> Evaluation {
    let fail = |message: String| Evaluation { ok: false, message };
    let staged = matches!(case.status, Status::Staged { .. });
    let changed = if staged { "staged case changed: " } else { "" };
    if let Status::Staged { ready_exit, .. } = case.status
        && run.exit == ready_exit
    {
        return fail(format!(
            "{label}: now exits {}, as it should once fixed: promote {label}",
            run.exit
        ));
    }
    if run.exit != case.expect_exit {
        return fail(format!(
            "{label}: {changed}exit {}, expected {}",
            run.exit, case.expect_exit
        ));
    }
    let Some(expected) = expected else {
        return fail(format!(
            "{label}: expected/{}.json is missing; run make record",
            case.id
        ));
    };
    let live = project(&run.stdout).structure;
    if &live != expected {
        let hint = if is_outcome(expected) && !is_outcome(&live) {
            " (the output is not a filament-outcome/1 document: check that FSL names the FSL driver)"
        } else {
            ""
        };
        return fail(format!(
            "{label}: {changed}outcome differs from expected/{}.json in {}{hint}",
            case.id,
            differing_fields(&live, expected).join(", ")
        ));
    }
    Evaluation {
        ok: true,
        message: format!("{label}: exit {} ok", run.exit),
    }
}

/// The top-level structure fields whose values differ.
fn differing_fields(live: &Value, expected: &Value) -> Vec<String> {
    let empty = Map::new();
    let a = live.as_object().unwrap_or(&empty);
    let b = expected.as_object().unwrap_or(&empty);
    let mut keys: Vec<&String> = a.keys().chain(b.keys()).collect();
    keys.dedup();
    let mut out: Vec<String> = Vec::new();
    for key in keys {
        if a.get(key) != b.get(key) && !out.iter().any(|k| k == key) {
            out.push(key.clone());
        }
    }
    out
}
