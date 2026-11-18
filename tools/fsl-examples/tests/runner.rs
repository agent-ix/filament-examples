//! The runner commands against the examples of this repository, with a stand-in driver.

mod support;

use fsl_examples::manifest::Status;
use fsl_examples::outcome::Run;
use fsl_examples::runner::{
    Command, Driver, VecSink, absolute, execute, expected_path, load_all, messages_path,
};
use serde_json::{Value, json};
use std::cell::Cell;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use support::{StandIn, TestResult, repo_root, scratch_repo};

/// The label, ready exit and folder of the first staged case.
fn first_staged(root: &Path) -> Result<(String, i32, PathBuf, String), Box<dyn std::error::Error>> {
    for system in load_all(root)? {
        for case in &system.manifest.cases {
            if let Status::Staged { ready_exit, .. } = case.status {
                return Ok((
                    format!("{}/{}", system.name, case.id),
                    ready_exit,
                    system.dir.clone(),
                    case.id.clone(),
                ));
            }
        }
    }
    Err("no staged case".into())
}

fn success_document() -> String {
    json!({
        "artifacts": [], "category": "success", "diagnostics": [],
        "format": "filament-outcome/1", "items": [], "last_stage": "S6a",
        "operation": "execute",
        "result": {"kind": "completed", "value": {"kind": "boolean", "value": true}}
    })
    .to_string()
}

/// Every structure file under `root/examples`, by path.
fn structure_files(root: &Path) -> Result<BTreeMap<PathBuf, String>, Box<dyn std::error::Error>> {
    let mut files = BTreeMap::new();
    for system in load_all(root)? {
        for case in &system.manifest.cases {
            let path = expected_path(&system.dir, &case.id);
            files.insert(path.clone(), fs::read_to_string(&path)?);
        }
    }
    Ok(files)
}

fn expected_files(root: &Path) -> Result<BTreeMap<PathBuf, String>, Box<dyn std::error::Error>> {
    let mut files = BTreeMap::new();
    for system in load_all(root)? {
        for entry in fs::read_dir(system.dir.join("expected"))? {
            let path = entry?.path();
            if path.is_file() {
                files.insert(path.clone(), fs::read_to_string(path)?);
            }
        }
    }
    Ok(files)
}

struct CountingDriver {
    inner: StandIn,
    calls: Cell<usize>,
}

impl Driver for CountingDriver {
    fn run(&self, dir: &Path, args: &[String]) -> fsl_examples::Result<Run> {
        self.calls.set(self.calls.get() + 1);
        self.inner.run(dir, args)
    }
}

#[test]
fn check_passes_on_all_running_cases() -> TestResult {
    let root = absolute(&repo_root())?;
    let driver = StandIn::faithful(&root)?;
    let mut sink = VecSink::default();
    let ok = execute(
        &Command::Check {
            dirs: Vec::new(),
            cases: Vec::new(),
            excluded_cases: Vec::new(),
        },
        &root,
        &driver,
        &mut sink,
    )?;
    assert_eq!(sink.errors, Vec::<String>::new());
    assert!(ok);
    assert_eq!(
        sink.info.last().map(String::as_str),
        Some("check: 307 passed, 0 failed")
    );
    Ok(())
}

#[test]
fn check_selects_only_matching_cases_and_accepts_globs() -> TestResult {
    let scratch = scratch_repo()?;
    let root = absolute(scratch.path())?;
    let driver = CountingDriver {
        inner: StandIn::faithful(&root)?,
        calls: Cell::new(0),
    };
    let mut sink = VecSink::default();
    let ok = execute(
        &Command::Check {
            dirs: vec![PathBuf::from("shop")],
            cases: vec![
                "misspelled-*".to_owned(),
                "shop/misspelled-fiel?".to_owned(),
            ],
            excluded_cases: Vec::new(),
        },
        &root,
        &driver,
        &mut sink,
    )?;
    assert!(ok, "{:?}", sink.errors);
    assert_eq!(driver.calls.get(), 1, "only the selected case runs");
    assert_eq!(
        sink.info.last().map(String::as_str),
        Some("check: 1 passed, 0 failed")
    );
    Ok(())
}

#[test]
fn check_rejects_unknown_case_selector_clearly() -> TestResult {
    let scratch = scratch_repo()?;
    let root = absolute(scratch.path())?;
    let driver = CountingDriver {
        inner: StandIn::faithful(&root)?,
        calls: Cell::new(0),
    };
    let mut sink = VecSink::default();
    let error = execute(
        &Command::Check {
            dirs: vec![PathBuf::from("shop")],
            cases: vec!["does-not-exist".to_owned()],
            excluded_cases: Vec::new(),
        },
        &root,
        &driver,
        &mut sink,
    )
    .err()
    .ok_or("unknown selector must fail")?;
    assert!(
        error
            .to_string()
            .contains("case selector \"does-not-exist\" matched no cases")
    );
    assert_eq!(
        driver.calls.get(),
        0,
        "validation precedes driver execution"
    );
    Ok(())
}

#[test]
fn check_excludes_only_matching_cases_and_keeps_generated_checks() -> TestResult {
    let scratch = scratch_repo()?;
    let root = absolute(scratch.path())?;
    let running = load_all(&root)?
        .iter()
        .flat_map(|system| &system.manifest.cases)
        .filter(|case| matches!(case.status, Status::Running { .. }))
        .count();
    let driver = CountingDriver {
        inner: StandIn::faithful(&root)?,
        calls: Cell::new(0),
    };
    let command = Command::Check {
        dirs: Vec::new(),
        cases: Vec::new(),
        excluded_cases: vec!["shop/misspelled-field".to_owned()],
    };
    let mut sink = VecSink::default();
    assert!(
        execute(&command, &root, &driver, &mut sink)?,
        "{:?}",
        sink.errors
    );
    assert_eq!(driver.calls.get(), running - 1);
    assert_eq!(
        sink.info.last().map(String::as_str),
        Some(format!("check: {} passed, 0 failed", running - 1).as_str())
    );

    let index = root.join("index.json");
    let fresh_index = fs::read_to_string(&index)?;
    fs::write(&index, fresh_index.replace("\"exit\": 10", "\"exit\": 11"))?;
    driver.calls.set(0);
    let mut sink = VecSink::default();
    assert!(!execute(&command, &root, &driver, &mut sink)?);
    assert_eq!(driver.calls.get(), running - 1);
    assert_eq!(
        sink.errors,
        vec!["index.json is stale: merge origin/main, then run make index".to_owned()]
    );

    fs::write(&index, fresh_index)?;
    let readme = root.join("README.md");
    let fresh_readme = fs::read_to_string(&readme)?;
    let stale_counts = fresh_readme.replace(
        "| [library](examples/library) | Shop | 6 | 0 |",
        "| [library](examples/library) | Shop | 5 | 0 |",
    );
    assert_ne!(stale_counts, fresh_readme, "the Systems table has a library row");
    fs::write(&readme, stale_counts)?;
    driver.calls.set(0);
    let mut sink = VecSink::default();
    assert!(!execute(&command, &root, &driver, &mut sink)?);
    assert_eq!(driver.calls.get(), running - 1);
    assert_eq!(
        sink.errors,
        vec!["README.md system counts are stale: merge origin/main, then run make readme".to_owned()]
    );
    Ok(())
}

#[test]
fn check_rejects_unknown_case_exclusion_clearly() -> TestResult {
    let scratch = scratch_repo()?;
    let root = absolute(scratch.path())?;
    let driver = CountingDriver {
        inner: StandIn::faithful(&root)?,
        calls: Cell::new(0),
    };
    let mut sink = VecSink::default();
    let error = execute(
        &Command::Check {
            dirs: vec![PathBuf::from("shop")],
            cases: Vec::new(),
            excluded_cases: vec!["does-not-exist".to_owned()],
        },
        &root,
        &driver,
        &mut sink,
    )
    .err()
    .ok_or("unknown exclusion must fail")?;
    assert!(
        error
            .to_string()
            .contains("case exclusion \"does-not-exist\" matched no cases")
    );
    assert_eq!(
        driver.calls.get(),
        0,
        "validation precedes driver execution"
    );
    Ok(())
}

#[test]
fn filtered_regenerate_changes_only_selected_record_files() -> TestResult {
    let scratch = scratch_repo()?;
    let root = absolute(scratch.path())?;
    let label = "shop/misspelled-field";
    let reworded = "the selected diagnostic has new wording";
    let driver = StandIn::new(&root, move |current, run| {
        if current != label {
            return;
        }
        if let Ok(mut doc) = serde_json::from_str::<Value>(&run.stdout)
            && let Some(message) = doc.pointer_mut("/diagnostics/0/message")
        {
            *message = Value::from(reworded);
            run.stdout = doc.to_string();
        }
    })?;
    let before = expected_files(&root)?;
    let readme_before = fs::read(root.join("README.md"))?;
    let index_before = fs::read(root.join("index.json"))?;
    let mut sink = VecSink::default();
    assert!(execute(
        &Command::Record {
            dirs: vec![PathBuf::from("shop")],
            cases: vec!["shop/misspelled-field".to_owned()],
            dry_run: false,
        },
        &root,
        &driver,
        &mut sink,
    )?);
    let after = expected_files(&root)?;
    let selected = root.join("examples/shop/expected/misspelled-field.messages.txt");
    assert_eq!(
        after.get(&selected).map(String::as_str),
        Some(format!("{reworded}\n").as_str())
    );
    assert_ne!(
        before.get(&selected),
        after.get(&selected),
        "selected snapshot changed"
    );
    assert_eq!(fs::read(root.join("README.md"))?, readme_before);
    assert_eq!(fs::read(root.join("index.json"))?, index_before);
    for (path, bytes) in before {
        if path != selected {
            assert_eq!(
                after.get(&path),
                Some(&bytes),
                "unselected file changed: {}",
                path.display()
            );
        }
    }
    Ok(())
}

#[test]
fn filtered_regenerate_dry_run_prints_changes_without_writing() -> TestResult {
    let scratch = scratch_repo()?;
    let root = absolute(scratch.path())?;
    let label = "shop/misspelled-field";
    let reworded = "dry run wording";
    let driver = StandIn::new(&root, move |current, run| {
        if current != label {
            return;
        }
        if let Ok(mut doc) = serde_json::from_str::<Value>(&run.stdout)
            && let Some(message) = doc.pointer_mut("/diagnostics/0/message")
        {
            *message = Value::from(reworded);
            run.stdout = doc.to_string();
        }
    })?;
    let before = expected_files(&root)?;
    let mut sink = VecSink::default();
    assert!(execute(
        &Command::Record {
            dirs: vec![PathBuf::from("shop")],
            cases: vec!["misspelled-field".to_owned()],
            dry_run: true,
        },
        &root,
        &driver,
        &mut sink,
    )?);
    assert!(
        sink.info
            .iter()
            .any(|line| line.contains("dry run wording")),
        "{:?}",
        sink.info
    );
    assert_eq!(
        expected_files(&root)?,
        before,
        "dry-run must not write records"
    );
    Ok(())
}

#[test]
fn staged_fails_when_a_case_reaches_ready_exit() -> TestResult {
    let root = absolute(&repo_root())?;
    let (label, ready_exit, _, _) = first_staged(&root)?;
    let target = label.clone();
    let driver = StandIn::new(&root, move |l, run| {
        if l == target {
            run.exit = ready_exit;
        }
    })?;
    let mut sink = VecSink::default();
    let ok = execute(&Command::Staged(Vec::new()), &root, &driver, &mut sink)?;
    assert!(!ok);
    assert_eq!(
        sink.errors,
        vec![format!(
            "{label}: now exits {ready_exit}, as it should once fixed: promote {label}"
        )]
    );
    assert_eq!(
        sink.info.last().map(String::as_str),
        Some("staged: 17 passed, 1 failed")
    );
    Ok(())
}

#[test]
fn staged_fails_when_a_staged_case_changes() -> TestResult {
    let root = absolute(&repo_root())?;
    let (label, _, _, id) = first_staged(&root)?;
    let target = label.clone();
    let driver = StandIn::new(&root, move |l, run| {
        if l == target {
            run.stdout = success_document();
        }
    })?;
    let mut sink = VecSink::default();
    let ok = execute(&Command::Staged(Vec::new()), &root, &driver, &mut sink)?;
    assert!(!ok);
    assert_eq!(sink.errors.len(), 1, "{:?}", sink.errors);
    let message = sink.errors.first().map(String::as_str).unwrap_or_default();
    assert!(
        message.starts_with(&format!(
            "{label}: staged case changed: outcome differs from expected/{id}.json in "
        )),
        "{message}"
    );
    Ok(())
}

#[test]
fn stale_index_or_readme_region_fails() -> TestResult {
    let scratch = scratch_repo()?;
    let root = absolute(scratch.path())?;
    let driver = StandIn::faithful(&root)?;
    let check = |sink: &mut VecSink| {
        execute(
            &Command::Check {
                dirs: Vec::new(),
                cases: Vec::new(),
                excluded_cases: Vec::new(),
            },
            &root,
            &driver,
            sink,
        )
    };

    let index = root.join("index.json");
    let fresh_index = fs::read_to_string(&index)?;
    fs::write(&index, fresh_index.replace("\"exit\": 10", "\"exit\": 11"))?;
    let mut sink = VecSink::default();
    assert!(!check(&mut sink)?);
    assert_eq!(
        sink.errors,
        vec!["index.json is stale: merge origin/main, then run make index".to_owned()]
    );
    fs::write(&index, &fresh_index)?;

    let readme = root.join("examples/rover/README.md");
    let fresh_readme = fs::read_to_string(&readme)?;
    let stale = fresh_readme.replacen(" -->\n```\nexit ", " -->\n```\nexit 9", 1);
    assert_ne!(stale, fresh_readme, "the rover README has an output region");
    fs::write(&readme, &stale)?;
    let mut sink = VecSink::default();
    assert!(!check(&mut sink)?);
    assert_eq!(
        sink.errors,
        vec![
            "examples/rover/README.md: recorded output is stale: merge origin/main, then run make readme"
                .to_owned()
        ]
    );

    let mut sink = VecSink::default();
    assert!(execute(&Command::Generate, &root, &driver, &mut sink)?);
    assert_eq!(fs::read_to_string(&readme)?, fresh_readme);

    let root_readme = root.join("README.md");
    let fresh_root_readme = fs::read_to_string(&root_readme)?;
    let stale_counts = fresh_root_readme.replace(
        "| [library](examples/library) | Shop | 6 | 0 |",
        "| [library](examples/library) | Shop | 5 | 0 |",
    );
    assert_ne!(
        stale_counts, fresh_root_readme,
        "the Systems table has a library row"
    );
    fs::write(&root_readme, stale_counts)?;
    let mut sink = VecSink::default();
    assert!(!check(&mut sink)?);
    assert_eq!(
        sink.errors,
        vec![
            "README.md system counts are stale: merge origin/main, then run make readme".to_owned()
        ]
    );

    let mut sink = VecSink::default();
    assert!(execute(&Command::Generate, &root, &driver, &mut sink)?);
    assert_eq!(fs::read_to_string(&root_readme)?, fresh_root_readme);
    let mut sink = VecSink::default();
    assert!(check(&mut sink)?, "{:?}", sink.errors);
    Ok(())
}

#[test]
fn record_refreshes_the_message_snapshot_and_keeps_the_structure() -> TestResult {
    let scratch = scratch_repo()?;
    let root = absolute(scratch.path())?;
    let label = "shop/misspelled-field";
    let reworded = "the field name is not declared on this object type";
    let driver = StandIn::new(&root, move |l, run| {
        if l != label {
            return;
        }
        if let Ok(mut doc) = serde_json::from_str::<Value>(&run.stdout)
            && let Some(message) = doc.pointer_mut("/diagnostics/0/message")
        {
            *message = Value::from(reworded);
            run.stdout = doc.to_string();
        }
    })?;
    let snapshot = messages_path(&root.join("examples/shop"), "misspelled-field");
    let before_snapshot = fs::read_to_string(&snapshot)?;
    assert_ne!(before_snapshot, format!("{reworded}\n"));
    let before = structure_files(&root)?;

    let mut sink = VecSink::default();
    assert!(
        execute(
            &Command::Check {
                dirs: Vec::new(),
                cases: Vec::new(),
                excluded_cases: Vec::new(),
            },
            &root,
            &driver,
            &mut sink
        )?,
        "a reworded message keeps the gate green before record: {:?}",
        sink.errors
    );

    let mut sink = VecSink::default();
    assert!(execute(
        &Command::Record {
            dirs: Vec::new(),
            cases: Vec::new(),
            dry_run: false
        },
        &root,
        &driver,
        &mut sink
    )?);
    assert_eq!(fs::read_to_string(&snapshot)?, format!("{reworded}\n"));
    assert_eq!(
        structure_files(&root)?,
        before,
        "record kept every structure file"
    );

    let mut sink = VecSink::default();
    assert!(
        execute(
            &Command::Check {
                dirs: Vec::new(),
                cases: Vec::new(),
                excluded_cases: Vec::new(),
            },
            &root,
            &driver,
            &mut sink
        )?,
        "{:?}",
        sink.errors
    );
    Ok(())
}

#[test]
fn promote_flips_a_ready_staged_case_to_running() -> TestResult {
    let scratch = scratch_repo()?;
    let root = absolute(scratch.path())?;
    let (label, ready_exit, dir, id) = first_staged(&root)?;
    let target = label.clone();
    let driver = StandIn::new(&root, move |l, run| {
        if l == target {
            run.exit = ready_exit;
            run.stdout = success_document();
        }
    })?;
    let mut sink = VecSink::default();
    let promote = Command::Promote {
        case: label.clone(),
        verdict: None,
    };
    assert!(execute(&promote, &root, &driver, &mut sink)?);
    assert_eq!(
        sink.info.first().map(String::as_str),
        Some(format!("promoted {label}: exit {ready_exit}, verdict \"true\"").as_str())
    );
    let promoted = load_all(&root)?
        .into_iter()
        .find(|s| s.dir == dir)
        .and_then(|s| s.manifest.cases.into_iter().find(|c| c.id == id))
        .ok_or("promoted case is gone")?;
    assert_eq!(promoted.expect_exit, ready_exit);
    assert_eq!(
        promoted.status,
        Status::Running {
            expect_verdict: "true".to_owned()
        }
    );
    let structure: Value = serde_json::from_str(&fs::read_to_string(expected_path(&dir, &id))?)?;
    assert_eq!(structure.get("category"), Some(&json!("success")));
    let index = fs::read_to_string(root.join("index.json"))?;
    assert!(index.contains(&format!(
        "\"case\": \"{id}\",\n      \"status\": \"running\""
    )));
    Ok(())
}
