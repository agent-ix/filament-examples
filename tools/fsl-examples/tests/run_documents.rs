//! The records of verification plan outcomes and multi-item run results.

use fsl_examples::outcome::{project, verdict_of};
use serde_json::{Value, json};

fn plan(format: &str) -> String {
    json!({"verification_plan": {
        "format": format,
        "plan": "Release",
        "package": "example:x",
        "status": "failed",
        "failed_steps": ["low"],
        "inputs": {"initial": "../data/tank-0.json", "seed": 0},
        "document": null,
        "dispositions": {
            "low": {"blocked_by": null, "evidence": "refutation", "method": "model_check", "status": "refuted",
                "result": {"category": "violation", "cause": null, "request_index": 0, "result": "refuted"}},
            "safe": {"blocked_by": "low", "evidence": null, "method": "model_check", "status": "blocked", "result": null}
        }
    }})
    .to_string()
}

fn items(format: &str) -> String {
    let item = |n: u64, v: &str| {
        json!({"selection": n, "result": {"format": "spine-run-result/1", "function": "IdInt",
            "outcome": {"kind": "completed", "value": {"type": "integer", "value": v}},
            "source": {"path": "../rules/calls.fsl"}}})
    };
    json!({"format": format, "category": "success", "items": [item(0, "3"), item(1, "5")],
        "selections": [{"expanded": 1, "index": 0}], "source": {"path": "../rules/calls.fsl"}})
    .to_string()
}

#[test]
fn a_plan_outcome_records_status_failed_steps_and_each_disposition_in_order() {
    let recorded = project(&plan("filament.verification-plan-outcome/v1"));
    assert_eq!(
        recorded.structure,
        json!({"plan_outcome": {
            "plan": "Release", "status": "failed", "failed_steps": ["low"],
            "steps": [
                {"id": "low", "status": "refuted", "blocked_by": null, "evidence": "refutation",
                 "method": "model_check", "category": "violation", "result": "refuted",
                 "cause": null, "basis": null, "certification": null},
                {"id": "safe", "status": "blocked", "blocked_by": "low", "evidence": null,
                 "method": "model_check", "category": null, "result": null,
                 "cause": null, "basis": null, "certification": null}
            ]
        }})
    );
    assert_eq!(
        verdict_of(&recorded.structure),
        "plan failed: low; blocked: safe"
    );
}

#[test]
fn a_passed_plan_has_a_plain_verdict() {
    assert_eq!(
        verdict_of(&project(&proved_plan().to_string()).structure),
        "plan passed"
    );
}

#[test]
fn a_multi_item_result_records_each_selection_function_and_value() {
    let recorded = project(&items("native-run-items/1"));
    assert_eq!(
        recorded.structure,
        json!({"items_outcome": {"category": "success", "items": [
            {"selection": 0, "function": "IdInt", "kind": "completed", "value": {"type": "integer", "value": "3"}},
            {"selection": 1, "function": "IdInt", "kind": "completed", "value": {"type": "integer", "value": "5"}}
        ]}})
    );
    assert_eq!(verdict_of(&recorded.structure), "2 completed");
}

#[test]
fn an_unknown_schema_id_stays_unreadable() {
    for text in [
        plan("filament.verification-plan-outcome/v2"),
        items("native-run-items/2"),
    ] {
        let recorded = project(&text);
        assert_eq!(recorded.structure, json!({"outcome": null}));
        assert_eq!(verdict_of(&recorded.structure), "unreadable output");
    }
}

fn unreadable(text: &str) {
    let recorded = project(text);
    assert_eq!(recorded.structure, json!({"outcome": null}), "{text}");
    assert_eq!(verdict_of(&recorded.structure), "unreadable output");
}

fn proved_plan() -> Value {
    json!({"verification_plan": {
    "format": "filament.verification-plan-outcome/v1", "plan": "Release", "status": "passed",
    "failed_steps": [], "dispositions": {
        "safe": {"blocked_by": null, "evidence": "proof", "method": "model_check", "status": "proved",
            "result": {"basis": {"type": "exhaustive"}, "category": "success", "cause": null,
                       "certification": "certified", "request_index": 0, "result": "proved"}}
    }}})
}

#[test]
fn a_proof_records_its_basis_and_certification() {
    let recorded = project(&proved_plan().to_string());
    let step = &recorded.structure["plan_outcome"]["steps"][0];
    assert_eq!(step["basis"], "exhaustive");
    assert_eq!(step["certification"], "certified");
    assert_eq!(step["cause"], Value::Null);
    let mut bounded = proved_plan();
    bounded["verification_plan"]["dispositions"]["safe"]["result"]["basis"] =
        json!({"type": "bounded"});
    assert_ne!(project(&bounded.to_string()).structure, recorded.structure);
}

#[test]
fn the_verdicts_name_failed_and_blocked_steps() {
    let mut doc: Value =
        serde_json::from_str(&plan("filament.verification-plan-outcome/v1")).unwrap_or_default();
    doc["verification_plan"]["dispositions"]["safe"] = json!({"blocked_by": null, "evidence": "proof", "method": "model_check",
        "status": "proved", "result": {"category": "success", "cause": null, "result": "proved"}});
    assert_eq!(
        verdict_of(&project(&doc.to_string()).structure),
        "plan failed: low"
    );
}

#[test]
fn a_plan_without_dispositions_is_unreadable() {
    let mut doc = proved_plan();
    if let Some(plan) = doc["verification_plan"].as_object_mut() {
        plan.remove("dispositions");
    }
    unreadable(&doc.to_string());
}

#[test]
fn an_item_without_selection_is_unreadable() {
    let mut doc: Value = serde_json::from_str(&items("native-run-items/1")).unwrap_or_default();
    if let Some(item) = doc["items"][0].as_object_mut() {
        item.remove("selection");
    }
    unreadable(&doc.to_string());
}

#[test]
fn non_json_is_unreadable() {
    unreadable("run: success (exit 0)\n");
}

#[test]
fn a_vanished_always_present_key_is_unreadable_not_null() {
    for (path, key) in [
        (vec![], "blocked_by"),
        (vec![], "evidence"),
        (vec![], "method"),
        (vec![], "result"),
        (vec!["result"], "category"),
        (vec!["result"], "result"),
        (vec!["result"], "cause"),
    ] {
        let mut doc = proved_plan();
        let mut node = &mut doc["verification_plan"]["dispositions"]["safe"];
        for p in &path {
            node = &mut node[*p];
        }
        if let Some(map) = node.as_object_mut() {
            map.remove(key);
        }
        unreadable(&doc.to_string());
    }
}
