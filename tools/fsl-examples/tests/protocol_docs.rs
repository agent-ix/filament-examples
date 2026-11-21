//! The protocol documentation cases through the production driver and runner.

use std::cell::RefCell;
use std::path::{Path, PathBuf};

use fsl_examples::outcome::Run;
use fsl_examples::runner::{Command, Driver, ProcessDriver, VecSink, execute};
use serde_json::Value;

type TestResult = Result<(), Box<dyn std::error::Error>>;

struct CapturingDriver {
    process: ProcessDriver,
    outcomes: RefCell<Vec<(String, Value)>>,
}

impl Driver for CapturingDriver {
    fn run(&self, directory: &Path, arguments: &[String]) -> fsl_examples::Result<Run> {
        let run = self.process.run(directory, arguments)?;
        if let Ok(document) = serde_json::from_str(&run.stdout) {
            self.outcomes
                .borrow_mut()
                .push((arguments.join(" "), document));
        }
        Ok(run)
    }
}

/// Trace: FR-570-AC-1, FR-571-AC-1, FR-571-AC-2, FR-571-AC-4.
#[test]
#[ignore = "protocol docs lane: set FSL to a current-main filament-driver build"]
fn protocol_docs_examples_run_and_match_records() -> TestResult {
    let binary = std::env::var_os("FSL").ok_or("set FSL for the protocol docs lane")?;
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()?;
    let driver = CapturingDriver {
        process: ProcessDriver::new(PathBuf::from(binary)),
        outcomes: RefCell::new(Vec::new()),
    };
    let cases = [
        "protocol-project-asker",
        "protocol-project-holder",
        "protocol-realizable",
        "protocol-unrealizable",
        "protocol-choice-repaired",
        "protocol-composition",
    ];
    let mut sink = VecSink::default();
    let passed = execute(
        &Command::Check {
            dirs: vec![PathBuf::from("lock")],
            cases: cases.iter().map(|case| format!("lock/{case}")).collect(),
            excluded_cases: Vec::new(),
        },
        &root,
        &driver,
        &mut sink,
    )?;
    assert!(passed, "{}", sink.errors.join("\n"));
    let outcomes = driver.outcomes.borrow();
    assert_eq!(outcomes.len(), cases.len(), "every documented case ran");
    for case in &cases[..5] {
        let document = outcomes
            .iter()
            .find(|(arguments, _)| arguments.contains(&format!("/{case}.json")))
            .map(|(_, document)| document)
            .ok_or_else(|| format!("no outcome for {case}"))?;
        let result = if *case == "protocol-unrealizable" {
            "refuted"
        } else {
            "proved"
        };
        assert_eq!(
            document["items"][0]["record"]["result"], result,
            "{case}: {document}"
        );
    }
    Ok(())
}
