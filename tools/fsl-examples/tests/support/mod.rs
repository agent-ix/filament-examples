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

/// An outcome document rebuilt from a recorded structure and message snapshot, as the driver
/// would print it.
pub fn outcome_document(structure: &Value, messages: &str) -> String {
    if structure.get("outcome") == Some(&Value::Null) {
        return messages.to_owned();
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
            stdout: outcome_document(&structure, &messages),
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
