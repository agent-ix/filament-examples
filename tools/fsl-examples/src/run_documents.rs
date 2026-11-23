//! The structure of the two documents `fsl run` prints besides a `filament-outcome/1`: the outcome of
//! a verification plan (`filament.verification-plan-outcome/v1`) and the results of a multi-item
//! request (`native-run-items/1`).
//!
//! Both are read through typed fields and matched by their exact schema id; a document with another
//! id is not read, so the caller records it as unreadable output. Paths, package identities and the
//! driver's request indexes are left out of the record: only what a reader of the example cares
//! about, and what must not change silently, is kept.

use serde::Deserialize;
use serde_json::{Value, json};

/// The schema id of a verification plan outcome.
pub const PLAN_FORMAT: &str = "filament.verification-plan-outcome/v1";
/// The schema id of a multi-item run result.
pub const ITEMS_FORMAT: &str = "native-run-items/1";

#[derive(Deserialize)]
struct PlanEnvelope {
    verification_plan: Plan,
}

#[derive(Deserialize)]
struct Plan {
    format: String,
    plan: String,
    status: String,
    failed_steps: Vec<String>,
    dispositions: serde_json::Map<String, Value>,
}

#[derive(Deserialize)]
struct Step {
    status: String,
    blocked_by: Option<String>,
    evidence: Option<String>,
    method: Option<String>,
    result: Option<StepResult>,
}

#[derive(Deserialize)]
struct StepResult {
    category: Option<String>,
    result: Option<String>,
}

#[derive(Deserialize)]
struct Items {
    format: String,
    category: String,
    items: Vec<Item>,
}

#[derive(Deserialize)]
struct Item {
    selection: u64,
    result: ItemResult,
}

#[derive(Deserialize)]
struct ItemResult {
    function: Option<String>,
    outcome: ItemOutcome,
}

#[derive(Deserialize)]
struct ItemOutcome {
    kind: String,
    value: Option<Value>,
}

/// The record of a verification plan outcome: the plan, its status, the failed steps and each
/// step's disposition in the document's order. `None` when the text is not a plan outcome.
pub fn plan_structure(stdout: &str) -> Option<Value> {
    let plan = serde_json::from_str::<PlanEnvelope>(stdout)
        .ok()?
        .verification_plan;
    if plan.format != PLAN_FORMAT {
        return None;
    }
    let mut steps = Vec::new();
    for (id, raw) in plan.dispositions {
        let step: Step = serde_json::from_value(raw).ok()?;
        steps.push(json!({
            "id": id,
            "status": step.status,
            "blocked_by": step.blocked_by,
            "evidence": step.evidence,
            "method": step.method,
            "category": step.result.as_ref().and_then(|r| r.category.clone()),
            "result": step.result.as_ref().and_then(|r| r.result.clone()),
        }));
    }
    Some(json!({
        "plan_outcome": {
            "plan": plan.plan,
            "status": plan.status,
            "failed_steps": plan.failed_steps,
            "steps": steps,
        }
    }))
}

/// The record of a multi-item run: the category and, per selection in order, the function and its
/// outcome with the value. `None` when the text is not a multi-item result.
pub fn items_structure(stdout: &str) -> Option<Value> {
    let doc: Items = serde_json::from_str(stdout).ok()?;
    if doc.format != ITEMS_FORMAT {
        return None;
    }
    let items: Vec<Value> = doc
        .items
        .into_iter()
        .map(|item| {
            json!({
                "selection": item.selection,
                "function": item.result.function,
                "kind": item.result.outcome.kind,
                "value": item.result.outcome.value,
            })
        })
        .collect();
    Some(json!({ "items_outcome": { "category": doc.category, "items": items } }))
}

/// The one-line verdict of a plan or multi-item record, `None` for any other structure.
pub fn verdict(structure: &Value) -> Option<String> {
    if let Some(plan) = structure.get("plan_outcome") {
        let status = plan
            .get("status")
            .and_then(Value::as_str)
            .unwrap_or("unknown");
        let failed: Vec<&str> = plan
            .get("failed_steps")
            .and_then(Value::as_array)
            .map(|a| a.iter().filter_map(Value::as_str).collect())
            .unwrap_or_default();
        return Some(if failed.is_empty() {
            format!("plan {status}")
        } else {
            format!("plan {status}: {}", failed.join(", "))
        });
    }
    let items = structure.get("items_outcome")?;
    let list = items.get("items").and_then(Value::as_array)?;
    let mut kinds: Vec<(String, usize)> = Vec::new();
    for item in list {
        let kind = item
            .get("kind")
            .and_then(Value::as_str)
            .unwrap_or("unknown");
        match kinds.iter_mut().find(|(k, _)| k == kind) {
            Some((_, n)) => *n += 1,
            None => kinds.push((kind.to_owned(), 1)),
        }
    }
    let parts: Vec<String> = kinds.iter().map(|(k, n)| format!("{n} {k}")).collect();
    Some(parts.join(", "))
}
