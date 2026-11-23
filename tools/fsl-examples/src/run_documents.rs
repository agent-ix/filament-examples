//! The structure of documents `fsl` prints besides a `filament-outcome/1`: the outcome of a
//! verification plan (`filament.verification-plan-outcome/v1`), multi-item request results
//! (`native-run-items/1`), and replay results (`results`).
//!
//! These documents are projected to the fields an example needs to keep stable. Paths, package
//! identities and driver request indexes are left out of the record.

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
    #[serde(deserialize_with = "required")]
    blocked_by: Option<String>,
    #[serde(deserialize_with = "required")]
    evidence: Option<String>,
    #[serde(deserialize_with = "required")]
    method: Option<String>,
    #[serde(deserialize_with = "required")]
    result: Option<StepResult>,
}

/// A step result. `basis` and `certification` appear only on a proof, so they stay optional.
#[derive(Deserialize)]
struct StepResult {
    #[serde(deserialize_with = "required")]
    category: Option<String>,
    #[serde(deserialize_with = "required")]
    result: Option<String>,
    #[serde(deserialize_with = "required")]
    cause: Option<Value>,
    basis: Option<Basis>,
    certification: Option<String>,
}

#[derive(Deserialize)]
struct Basis {
    #[serde(rename = "type")]
    kind: String,
}

/// A member the driver always prints, even when its value is null: absent is an error, null is `None`.
fn required<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer)
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
            "cause": step.result.as_ref().and_then(|r| r.cause.clone()),
            "basis": step.result.as_ref().and_then(|r| r.basis.as_ref().map(|b| b.kind.clone())),
            "certification": step.result.as_ref().and_then(|r| r.certification.clone()),
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

/// The record of a replay result: each result's arm, category, settlement and value.
/// `None` when the output is not a replay result document.
pub fn replay_structure(stdout: &str) -> Option<Value> {
    let document = serde_json::from_str::<Value>(stdout).ok()?;
    let results = document.get("results")?.as_array()?;
    let results = results
        .iter()
        .map(|result| {
            let mut projected = json!({
                "arm": result.get("arm")?.as_str()?,
                "category": result.get("category")?.as_str()?,
                "settlement": result.get("settlement")?.as_str()?,
            });
            if let Some(value) = result.get("value") {
                projected["value"] = value.clone();
            }
            Some(projected)
        })
        .collect::<Option<Vec<_>>>()?;
    Some(json!({ "replay_outcome": { "results": results } }))
}

/// The one-line verdict of a plan, multi-item or replay record, `None` for any other structure.
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
        let blocked: Vec<&str> = plan
            .get("steps")
            .and_then(Value::as_array)
            .map(|steps| {
                steps
                    .iter()
                    .filter(|s| s.get("status").and_then(Value::as_str) == Some("blocked"))
                    .filter_map(|s| s.get("id").and_then(Value::as_str))
                    .collect()
            })
            .unwrap_or_default();
        let mut line = format!("plan {status}");
        if !failed.is_empty() {
            line.push_str(&format!(": {}", failed.join(", ")));
        }
        if !blocked.is_empty() {
            line.push_str(&format!("; blocked: {}", blocked.join(", ")));
        }
        return Some(line);
    }
    if let Some(results) = structure
        .get("replay_outcome")
        .and_then(|outcome| outcome.get("results"))
        .and_then(Value::as_array)
    {
        if results.is_empty() {
            return Some("0 results".to_owned());
        }
        let mut outcomes: Vec<(String, usize)> = Vec::new();
        for result in results {
            let arm = result.get("arm")?.as_str()?;
            let settlement = result.get("settlement")?.as_str()?;
            let outcome = format!("{arm} {settlement}");
            match outcomes.iter_mut().find(|(known, _)| known == &outcome) {
                Some((_, count)) => *count += 1,
                None => outcomes.push((outcome, 1)),
            }
        }
        return Some(
            outcomes
                .iter()
                .map(|(outcome, count)| format!("{count} {outcome}"))
                .collect::<Vec<_>>()
                .join(", "),
        );
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
