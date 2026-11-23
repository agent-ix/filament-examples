//! Shared test helpers: the repository root, a stand-in driver and a scratch copy of the repository.

use fsl_examples::outcome::Run;
use fsl_examples::runner::{Driver, System, load_all, read_expected, read_messages};
use fsl_examples::{Error, ErrorCode, Result};
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};

pub type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

/// The filament-examples checkout this crate lives in.
pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The verification plan document that `fsl run` prints for a recorded `plan_outcome`.
#[allow(clippy::panic)]
fn plan_document(case: &str, plan: &Value) -> String {
    let text = |value: &Value, key: &str| value.get(key).cloned().unwrap_or(Value::Null);
    let mut dispositions = serde_json::Map::new();
    let steps = plan
        .get("steps")
        .and_then(Value::as_array)
        .unwrap_or_else(|| panic!("stand-in: {case}: the plan_outcome record has no steps array"));
    for step in steps {
        let result = if ["category", "result", "cause", "basis", "certification"]
            .iter()
            .all(|key| text(step, key).is_null())
        {
            Value::Null
        } else {
            let mut result = json!({
                "category": text(step, "category"),
                "cause": text(step, "cause"),
                "result": text(step, "result"),
            });
            if let Some(kind) = step.get("basis").filter(|b| !b.is_null()) {
                result["basis"] = json!({ "type": kind });
            }
            if let Some(certification) = step.get("certification").filter(|c| !c.is_null()) {
                result["certification"] = certification.clone();
            }
            result
        };
        let id = step.get("id").and_then(Value::as_str).unwrap_or_default();
        dispositions.insert(
            id.to_owned(),
            json!({
                "blocked_by": text(step, "blocked_by"),
                "evidence": text(step, "evidence"),
                "method": text(step, "method"),
                "status": text(step, "status"),
                "result": result,
            }),
        );
    }
    json!({"verification_plan": {
        "format": "filament.verification-plan-outcome/v1",
        "plan": text(plan, "plan"),
        "status": text(plan, "status"),
        "failed_steps": text(plan, "failed_steps"),
        "dispositions": dispositions,
    }})
    .to_string()
}

/// The multi-item document that `fsl run` prints for a recorded `items_outcome`.
#[allow(clippy::panic)]
fn items_document(case: &str, items: &Value) -> String {
    let recorded = items
        .get("items")
        .and_then(Value::as_array)
        .unwrap_or_else(|| panic!("stand-in: {case}: the items_outcome record has no items array"));
    let list: Vec<Value> = recorded
        .iter()
        .map(|item| {
            json!({
                "selection": item.get("selection").cloned().unwrap_or(Value::Null),
                "result": {
                    "function": item.get("function").cloned().unwrap_or(Value::Null),
                    "outcome": {
                        "kind": item.get("kind").cloned().unwrap_or(Value::Null),
                        "value": item.get("value").cloned().unwrap_or(Value::Null),
                    },
                },
            })
        })
        .collect();
    json!({
        "format": "native-run-items/1",
        "category": items.get("category").cloned().unwrap_or(Value::Null),
        "items": list,
    })
    .to_string()
}

/// An outcome document rebuilt from a recorded structure and message snapshot, as the driver
/// would print it.
pub fn outcome_document(case: &str, structure: &Value, messages: &str) -> String {
    if structure.get("outcome") == Some(&Value::Null) {
        return messages.to_owned();
    }
    if let Some(plan) = structure.get("plan_outcome") {
        return plan_document(case, plan);
    }
    if let Some(items) = structure.get("items_outcome") {
        return items_document(case, items);
    }
    if let Some(replay) = structure.get("replay_outcome") {
        return json!({ "results": replay.get("results") }).to_string();
    }
    if let (Some(Value::Array(events)), Some(summary)) =
        (structure.get("events"), structure.get("summary"))
    {
        let mut lines: Vec<String> = events.iter().map(Value::to_string).collect();
        lines.push(summary.to_string());
        return lines.join("\n");
    }
    let field = |key: &str| structure.get(key).cloned().unwrap_or(Value::Null);
    let diagnostics = match structure.get("first_diagnostic") {
        Some(d) if !d.is_null() => json!([{
            "cause": d.get("cause").cloned().unwrap_or(Value::Null),
            "code": d.get("code").cloned().unwrap_or(Value::Null),
            "locus": null,
            "message": messages.lines().next().unwrap_or_default(),
        }]),
        _ => json!([]),
    };
    json!({
        "artifacts": [],
        "category": field("category"),
        "diagnostics": diagnostics,
        "format": "filament-outcome/1",
        "items": field("items"),
        "last_stage": field("last_stage"),
        "operation": "check",
        "result": field("result"),
    })
    .to_string()
}

type Edit = dyn Fn(&str, &mut Run);

/// A driver that answers each case with its recorded outcome and manifest exit code, then lets
/// the test change the answer for chosen cases. The edit gets the case label `<system>/<id>`.
pub struct StandIn {
    systems: Vec<System>,
    edit: Box<Edit>,
}

impl StandIn {
    pub fn new(root: &Path, edit: impl Fn(&str, &mut Run) + 'static) -> Result<Self> {
        Ok(Self {
            systems: load_all(root)?,
            edit: Box::new(edit),
        })
    }

    pub fn faithful(root: &Path) -> Result<Self> {
        Self::new(root, |_, _| {})
    }
}

impl Driver for StandIn {
    fn run(&self, dir: &Path, args: &[String]) -> Result<Run> {
        let missing = || Error::new(ErrorCode::Driver, format!("stand-in: no case for {args:?}"));
        let system = self
            .systems
            .iter()
            .find(|s| s.dir == dir)
            .ok_or_else(missing)?;
        let case = system
            .manifest
            .cases
            .iter()
            .find(|c| {
                let mut want: Vec<String> = c
                    .command
                    .split(' ')
                    .filter(|a| !a.is_empty())
                    .map(str::to_owned)
                    .collect();
                want.extend(["--format".to_owned(), "json".to_owned()]);
                want == args
            })
            .ok_or_else(missing)?;
        let structure = read_expected(&system.dir, &case.id)?.ok_or_else(missing)?;
        let messages = read_messages(&system.dir, &case.id)?;
        let mut run = Run {
            exit: case.expect_exit,
            stdout: outcome_document(
                &format!("{}/{}", system.name, case.id),
                &structure,
                &messages,
            ),
        };
        (self.edit)(&format!("{}/{}", system.name, case.id), &mut run);
        Ok(run)
    }
}

/// Copies the parts of the repository the runner reads into a new temporary folder.
pub fn scratch_repo() -> std::result::Result<tempfile::TempDir, Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let root = repo_root();
    copy_tree(&root.join("examples"), &dir.path().join("examples"))?;
    fs::copy(root.join("README.md"), dir.path().join("README.md"))?;
    fs::copy(root.join("index.json"), dir.path().join("index.json"))?;
    Ok(dir)
}

fn copy_tree(from: &Path, to: &Path) -> std::io::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}
