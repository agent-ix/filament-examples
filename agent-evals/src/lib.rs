//! FSL-specific scenarios behind the published cli-evals external provider.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::PathBuf;

/// A scenario preparation or independent artifact grading failure.
pub type ScenarioResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Authoritative shipped tools and docs, referenced rather than copied into this repository.
#[derive(Debug, Clone)]
pub struct Tools {
    /// Accepted current-main FSL executable.
    pub fsl: PathBuf,
    /// Authoritative shipped documentation directory.
    pub docs: PathBuf,
    /// Public Quire executable.
    pub quire: PathBuf,
    /// Public Quoin executable.
    pub quoin: PathBuf,
}

/// Independent checks returned through the runner's external-provider assertion contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Assertion {
    /// Whether all required artifact and public-command checks passed.
    pub ok: bool,
    /// Structured, non-secret observations from the grader.
    pub checks: BTreeMap<String, Value>,
    /// Concrete failures; an agent's success marker cannot override them.
    pub failures: Vec<String>,
}

/// Original FSL scenarios, selected by the SDK provider catalogue.
pub mod scenarios {
    /// Source-owned object model authoring and independent model checks.
    pub mod author_model;
    /// Refusal-only repair with independent public function executions.
    pub mod refusal_repair;
}

/// Published external-provider integration and independent command evidence.
pub mod provider;
pub use provider::{agent_commands, invokes_fsl};
