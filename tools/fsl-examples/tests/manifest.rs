//! Manifest parsing.

use fsl_examples::ErrorCode;
use fsl_examples::manifest::{Status, parse_manifest, promote_text};
use fsl_examples::runner::load_all;
use serde_json::Value;
use std::path::{Path, PathBuf};

type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

const STAGED_CASE: &str = r#"
name = "demo"

[[case]]
id = "later"
statement = "A rule."
file = "rules/a.fsl"
command = "run requests/later.json"
status = "staged"
expect_exit = 21
ready_exit = 0
waits_on = "QSL-123"
why = "The operator is not implemented."
needs = "the operator"
"#;

/// Every manifest parses, and its cases, exit codes and staged fields are the ones `index.json`
/// lists. `index.json` was written by the Node runner this crate replaces and is kept fresh by
/// `make check`, so it is the reference for what the old parser read.
#[test]
fn parses_every_example_toml() -> TestResult {
    let root = repo_root();
    let systems = load_all(&root)?;
    assert_eq!(systems.len(), 7, "seven example systems");
    let index: Value = serde_json::from_str(&std::fs::read_to_string(root.join("index.json"))?)?;
    let listed = index
        .get("cases")
        .and_then(Value::as_array)
        .ok_or("index.json has no cases")?;
    let mut parsed = Vec::new();
    for system in &systems {
        for case in &system.manifest.cases {
            let (verdict, waits_on, needs) = match &case.status {
                Status::Running { expect_verdict } => (
                    Value::from(expect_verdict.as_str()),
                    Value::Null,
                    Value::Null,
                ),
                Status::Staged {
                    ready_exit,
                    waits_on,
                    why,
                    needs,
                } => {
                    assert_ne!(*ready_exit, case.expect_exit, "{}", case.id);
                    assert!(!why.trim().is_empty(), "{}: why", case.id);
                    (
                        Value::Null,
                        Value::from(waits_on.as_str()),
                        Value::from(needs.as_str()),
                    )
                }
            };
            parsed.push(serde_json::json!({
                "system": system.name, "case": case.id, "status": case.status.name(),
                "statement": case.statement, "verdict": verdict, "exit": case.expect_exit,
                "waits_on": waits_on, "needs": needs,
            }));
        }
    }
    let keys = [
        "system",
        "case",
        "status",
        "statement",
        "verdict",
        "exit",
        "waits_on",
        "needs",
    ];
    let reference: Vec<Value> = listed
        .iter()
        .map(|entry| {
            Value::Object(
                keys.iter()
                    .map(|k| {
                        (
                            (*k).to_owned(),
                            entry.get(*k).cloned().unwrap_or(Value::Null),
                        )
                    })
                    .collect(),
            )
        })
        .collect();
    assert_eq!(parsed, reference);
    let running = parsed.iter().filter(|c| c["status"] == "running").count();
    let staged = parsed.iter().filter(|c| c["status"] == "staged").count();
    assert_eq!((running, staged), (307, 17));
    Ok(())
}

#[test]
fn staged_without_waits_on_is_refused() -> TestResult {
    let without = STAGED_CASE.replace("waits_on = \"QSL-123\"\n", "");
    let err = parse_manifest(&without, "demo/example.toml")
        .err()
        .ok_or("a staged case without waits_on was accepted")?;
    assert_eq!(err.code(), ErrorCode::Manifest);
    assert_eq!(
        err.to_string(),
        "demo/example.toml: case later: waits_on must be a string"
    );

    let not_a_ticket = STAGED_CASE.replace("\"QSL-123\"", "\"soon\"");
    let err = parse_manifest(&not_a_ticket, "demo/example.toml")
        .err()
        .ok_or("a staged case whose waits_on is not a ticket id was accepted")?;
    assert_eq!(
        err.to_string(),
        "demo/example.toml: case later: waits_on must be a ticket id such as QSL-123"
    );

    let parsed = parse_manifest(STAGED_CASE, "demo/example.toml")?;
    assert_eq!(parsed.cases.len(), 1);
    Ok(())
}

#[test]
fn values_other_than_strings_and_integers_are_refused() -> TestResult {
    let nested = STAGED_CASE.replace("needs = \"the operator\"", "needs = [\"the operator\"]");
    let err = parse_manifest(&nested, "m.toml")
        .err()
        .ok_or("an array value was accepted")?;
    assert_eq!(
        err.to_string(),
        "m.toml: case later: needs: value must be a string or an integer"
    );
    Ok(())
}

#[test]
fn promote_text_flips_only_the_named_case() -> TestResult {
    let text = format!("{STAGED_CASE}\n[[case]]\nid = \"other\"\nstatus = \"staged\"\n");
    let out = promote_text(&text, "later", "holds")?;
    let promoted = parse_manifest(
        out.split("\n[[case]]\nid = \"other\"").next().unwrap_or(""),
        "m",
    )?;
    let case = promoted.cases.first().ok_or("no case")?;
    assert_eq!(case.expect_exit, 0);
    assert_eq!(
        case.status,
        Status::Running {
            expect_verdict: "holds".to_owned()
        }
    );
    assert!(out.ends_with("[[case]]\nid = \"other\"\nstatus = \"staged\"\n"));
    Ok(())
}

#[test]
fn engine_and_clock_tags_are_checked() -> TestResult {
    let tagged = |extra: &str| {
        STAGED_CASE.replace(
            "needs = \"the operator\"\n",
            &format!("needs = \"the operator\"\n{extra}"),
        )
    };
    let parsed = parse_manifest(&tagged("engine = \"timed\"\nclock = \"steps\"\n"), "m.toml")?;
    let case = parsed.cases.first().ok_or("no case")?;
    assert_eq!(case.engine.as_deref(), Some("timed"));
    assert_eq!(case.clock.as_deref(), Some("steps"));

    let refused = |extra: &str| -> std::result::Result<String, Box<dyn std::error::Error>> {
        Ok(parse_manifest(&tagged(extra), "m.toml")
            .err()
            .ok_or("an invalid tag was accepted")?
            .to_string())
    };
    assert_eq!(
        refused("engine = \"guessing\"\n")?,
        "m.toml: case later: engine must be one of model-checking, refinement, protocol, probability, timed, monitor"
    );
    assert_eq!(
        refused("engine = \"timed\"\n")?,
        "m.toml: case later: a timed case names its clock"
    );
    assert_eq!(
        refused("engine = \"monitor\"\nclock = \"steps\"\n")?,
        "m.toml: case later: clock is a non-empty string, and only a timed case has one"
    );
    Ok(())
}
