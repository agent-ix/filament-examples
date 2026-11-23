//! EV-006: declare a protocol's intentional terminal state and analyze deadlock freedom.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use fsl_cst::{CstElement, CstNode, ParsedSource, Production, TokenKind};
use fsl_foundation::SourceIdentity;
use serde_json::{Value, json};

use crate::capture::{CaptureCode, CaptureError, before_deadline, capture_command};
use crate::{Assertion, ScenarioResult, Tools};

/// Stable scenario identifier in ticket order.
pub const ID: &str = "EV-006";
/// Human-readable catalogue title.
pub const TITLE: &str = "Declare intended protocol termination and prove deadlock freedom";

const MODEL: &str = include_str!("../../fixtures/EV-006/cell.semantic-ir.json");
const START: &str = include_str!("../../fixtures/EV-006/start.json");
const DEADLOCK_REQUEST: &str = include_str!("../../fixtures/EV-006/deadlock-request.json");
const REPAIRED_REQUEST: &str = include_str!("../../fixtures/EV-006/repaired-request.json");
const TASK: &str = include_str!("../../fixtures/EV-006/task.md");
const ASSERT_BUDGET: Duration = Duration::from_secs(30);
const FILE_LIMIT: u64 = 1_048_576;

#[derive(Clone, Copy, Debug)]
enum FaultCode {
    InvalidArtifact,
    ChangedInput,
    WrongProtocol,
    CliGap,
    WrongDeadlock,
    WrongProof,
    WrongSummary,
    ToolFailure,
    Timeout,
}

#[derive(Debug)]
struct Fault {
    code: FaultCode,
    detail: String,
}

impl Fault {
    fn new(code: FaultCode, detail: impl Into<String>) -> Self {
        Self {
            code,
            detail: detail.into(),
        }
    }
}

impl std::fmt::Display for Fault {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(out, "{}", self.detail)
    }
}
impl std::error::Error for Fault {}
type Checked<T> = Result<T, Fault>;

/// Write only the fixed model and analysis requests into the sample workspace.
pub fn prepare(workspace: &Path, _tools: &Tools) -> ScenarioResult<String> {
    fs::create_dir_all(workspace)?;
    for (name, contents) in [
        ("cell.semantic-ir.json", MODEL),
        ("start.json", START),
        ("deadlock-request.json", DEADLOCK_REQUEST),
        ("repaired-request.json", REPAIRED_REQUEST),
    ] {
        fs::write(workspace.join(name), contents)?;
    }
    Ok(TASK.to_owned())
}

fn read_file(workspace: &Path, name: &str) -> Checked<String> {
    let path = workspace.join(name);
    let metadata = fs::symlink_metadata(&path)
        .map_err(|_| Fault::new(FaultCode::InvalidArtifact, format!("missing {name}")))?;
    if !metadata.file_type().is_file() || metadata.len() > FILE_LIMIT {
        return Err(Fault::new(
            FaultCode::InvalidArtifact,
            format!("{name} must be a bounded regular file"),
        ));
    }
    fs::read_to_string(path)
        .map_err(|_| Fault::new(FaultCode::InvalidArtifact, format!("{name} must be UTF-8")))
}

fn json_file(workspace: &Path, name: &str) -> Checked<Value> {
    serde_json::from_str(&read_file(workspace, name)?)
        .map_err(|_| Fault::new(FaultCode::InvalidArtifact, format!("{name} must be JSON")))
}

fn check_inputs(workspace: &Path) -> Checked<(String, String)> {
    for (name, contents) in [
        ("cell.semantic-ir.json", MODEL),
        ("start.json", START),
        ("deadlock-request.json", DEADLOCK_REQUEST),
        ("repaired-request.json", REPAIRED_REQUEST),
    ] {
        if read_file(workspace, name)? != contents {
            return Err(Fault::new(
                FaultCode::ChangedInput,
                format!("the fixed {name} input was changed"),
            ));
        }
    }
    let baseline = read_file(workspace, "baseline.native")?;
    let repaired = read_file(workspace, "protocol.native")?;
    Ok((baseline, repaired))
}

fn significant_tokens(parsed: &ParsedSource) -> Vec<(usize, usize, String)> {
    parsed
        .cst()
        .tokens()
        .iter()
        .filter(|token| !matches!(token.kind(), TokenKind::Whitespace | TokenKind::Comment))
        .map(|token| {
            let span = token.span();
            (
                span.start,
                span.end,
                String::from_utf8_lossy(token.spelling()).into_owned(),
            )
        })
        .collect()
}

fn tokens_in(parsed: &ParsedSource, node: &CstNode) -> Vec<String> {
    let span = node.span();
    significant_tokens(parsed)
        .into_iter()
        .filter(|(start, end, _)| *start >= span.start && *end <= span.end)
        .map(|(_, _, spelling)| spelling)
        .collect()
}

fn tokens_outside(parsed: &ParsedSource, excluded: (usize, usize)) -> Vec<String> {
    significant_tokens(parsed)
        .into_iter()
        .filter(|(start, end, _)| *end <= excluded.0 || *start >= excluded.1)
        .map(|(_, _, spelling)| spelling)
        .collect()
}

fn nodes_within<'a>(
    parsed: &'a ParsedSource,
    parent: &CstNode,
    kind: Production,
) -> Vec<&'a CstNode> {
    let parent_span = parent.span();
    parsed
        .cst()
        .nodes()
        .iter()
        .filter(|node| {
            let span = node.span();
            node.production() == kind
                && span.start >= parent_span.start
                && span.end <= parent_span.end
        })
        .collect()
}

fn direct_nodes<'a>(
    parsed: &'a ParsedSource,
    parent: &CstNode,
    kind: Production,
) -> Vec<&'a CstNode> {
    parent
        .children()
        .iter()
        .filter_map(|child| match child {
            CstElement::Node(index) => Some(&parsed.cst().nodes()[*index]),
            CstElement::Token(_) => None,
        })
        .filter(|node| node.production() == kind)
        .collect()
}

fn parse_protocol(source: &str) -> Checked<(ParsedSource, CstNode)> {
    let parsed = fsl_cst::parse(
        SourceIdentity::new("eval", "protocol-deadlock"),
        "protocol.native",
        source.as_bytes(),
        fsl_cst::Limits::default(),
    )
    .map_err(|_| {
        Fault::new(
            FaultCode::CliGap,
            "the grader's FSL parser rejected source accepted by the trusted CLI",
        )
    })?;
    if !parsed.is_admissible() {
        return Err(Fault::new(
            FaultCode::CliGap,
            "the grader's FSL parser does not admit source accepted by the trusted CLI",
        ));
    }
    let protocols = parsed
        .cst()
        .nodes()
        .iter()
        .filter(|node| node.production() == Production::ProtocolClause)
        .filter(|node| {
            tokens_in(&parsed, node)
                .get(1)
                .is_some_and(|name| name == "Fill")
        })
        .cloned()
        .collect::<Vec<_>>();
    if protocols.len() != 1 {
        return Err(Fault::new(
            FaultCode::WrongProtocol,
            "source must declare exactly one Fill protocol",
        ));
    }
    Ok((parsed, protocols[0].clone()))
}

fn branch_operation(parsed: &ParsedSource, branch: &CstNode) -> Checked<Vec<String>> {
    let attempts = nodes_within(parsed, branch, Production::EventNode)
        .into_iter()
        .filter(|node| {
            tokens_in(parsed, node)
                .first()
                .is_some_and(|token| token == "attempt")
        })
        .collect::<Vec<_>>();
    if attempts.len() != 1 {
        return Err(Fault::new(
            FaultCode::WrongProtocol,
            "each Fill branch must contain one attempt",
        ));
    }
    let tokens = tokens_in(parsed, attempts[0]);
    let Some(on) = tokens.iter().position(|token| token == "on") else {
        return Err(Fault::new(
            FaultCode::WrongProtocol,
            "attempt must identify its operation",
        ));
    };
    let Some(contracts) = tokens.iter().position(|token| token == "contracts") else {
        return Err(Fault::new(
            FaultCode::WrongProtocol,
            "attempt must list its contracts",
        ));
    };
    if on + 1 >= contracts {
        return Err(Fault::new(
            FaultCode::WrongProtocol,
            "attempt operation is empty",
        ));
    }
    Ok(tokens[on + 1..contracts].to_vec())
}

fn branch_contracts(parsed: &ParsedSource, branch: &CstNode) -> Checked<Vec<String>> {
    let attempts = nodes_within(parsed, branch, Production::EventNode)
        .into_iter()
        .filter(|node| {
            tokens_in(parsed, node)
                .first()
                .is_some_and(|token| token == "attempt")
        })
        .collect::<Vec<_>>();
    if attempts.len() != 1 {
        return Err(Fault::new(
            FaultCode::WrongProtocol,
            "each Fill branch must contain one attempt",
        ));
    }
    let tokens = tokens_in(parsed, attempts[0]);
    let Some(contracts) = tokens.iter().position(|token| token == "contracts") else {
        return Err(Fault::new(
            FaultCode::WrongProtocol,
            "attempt must list its contracts",
        ));
    };
    let Some(end) = tokens.iter().position(|token| token == "as") else {
        return Err(Fault::new(
            FaultCode::WrongProtocol,
            "attempt must declare its result binder",
        ));
    };
    if contracts >= end {
        return Err(Fault::new(
            FaultCode::WrongProtocol,
            "attempt contract list is empty",
        ));
    }
    Ok(tokens[contracts + 1..end]
        .iter()
        .filter(|token| !matches!(token.as_str(), "[" | "]" | ","))
        .cloned()
        .collect())
}

fn blocked_precondition_clause(deadlock: &Value) -> Checked<String> {
    let blocked = deadlock
        .get("items")
        .and_then(Value::as_array)
        .and_then(|items| items.first())
        .and_then(|item| item.get("counterexample"))
        .and_then(|counterexample| counterexample.get("blocked"))
        .and_then(Value::as_array)
        .ok_or_else(|| {
            Fault::new(
                FaultCode::WrongDeadlock,
                "trusted counterexample has no blocked threads",
            )
        })?;
    let clauses = blocked
        .iter()
        .filter(|entry| {
            entry.pointer("/cause/type").and_then(Value::as_str) == Some("precondition")
        })
        .filter_map(|entry| entry.pointer("/cause/clause").and_then(Value::as_str))
        .collect::<Vec<_>>();
    if clauses.len() != 1 || clauses[0].is_empty() {
        return Err(Fault::new(
            FaultCode::WrongDeadlock,
            "trusted counterexample must identify one blocked precondition clause",
        ));
    }
    Ok(clauses[0].to_owned())
}

fn precondition_block<'a>(parsed: &'a ParsedSource, operation: &[String]) -> Checked<&'a CstNode> {
    let matches = parsed
        .cst()
        .nodes()
        .iter()
        .filter(|node| node.production() == Production::StateClause)
        .filter(|node| {
            let tokens = tokens_in(parsed, node);
            let blocks = direct_nodes(parsed, node, Production::Block);
            let block_span = blocks.first().map(|block| block.span());
            let header = block_span.map_or_else(
                || tokens.clone(),
                |span| {
                    significant_tokens(parsed)
                        .into_iter()
                        .filter(|(start, end, _)| {
                            *start >= node.span().start
                                && *end <= node.span().end
                                && (*end <= span.start || *start >= span.end)
                        })
                        .map(|(_, _, token)| token)
                        .collect()
                },
            );
            tokens.first().is_some_and(|token| token == "pre")
                && header
                    .iter()
                    .position(|token| token == "on")
                    .is_some_and(|on| header[on + 1..] == *operation)
                && blocks.len() == 1
        })
        .collect::<Vec<_>>();
    if matches.len() != 1 {
        return Err(Fault::new(
            FaultCode::WrongProtocol,
            "the blocked branch must have one matching precondition",
        ));
    }
    let blocks = direct_nodes(parsed, matches[0], Production::Block);
    if blocks.len() != 1 {
        return Err(Fault::new(
            FaultCode::WrongProtocol,
            "branch precondition must have one predicate block",
        ));
    }
    Ok(blocks[0])
}

struct BranchShape {
    operation: Vec<String>,
    contracts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RepairPredicate {
    Terminal {
        expression: Vec<String>,
        binder: String,
    },
    Precondition {
        expression: Vec<String>,
    },
}

fn protocol_shape(parsed: &ParsedSource, protocol: &CstNode) -> Checked<Vec<BranchShape>> {
    let parallels = nodes_within(parsed, protocol, Production::Parallel);
    if parallels.len() != 1 {
        return Err(Fault::new(
            FaultCode::WrongProtocol,
            "Fill must contain one parallel join",
        ));
    }
    let branches = direct_nodes(parsed, parallels[0], Production::Branch);
    if branches.len() != 2 {
        return Err(Fault::new(
            FaultCode::WrongProtocol,
            "Fill must have two parallel branches",
        ));
    }
    branches
        .into_iter()
        .map(|branch| {
            Ok(BranchShape {
                operation: branch_operation(parsed, branch)?,
                contracts: branch_contracts(parsed, branch)?,
            })
        })
        .collect()
}

fn check_protocol_repair(
    baseline: &str,
    repaired: &str,
    deadlock: &Value,
) -> Checked<RepairPredicate> {
    let (base_parsed, base_protocol) = parse_protocol(baseline)?;
    let (repair_parsed, repair_protocol) = parse_protocol(repaired)?;
    let base_branches = protocol_shape(&base_parsed, &base_protocol)?;
    let repair_branches = protocol_shape(&repair_parsed, &repair_protocol)?;
    let blocked_clause = blocked_precondition_clause(deadlock)?;
    let base_targets = base_branches
        .iter()
        .filter(|branch| {
            branch
                .contracts
                .iter()
                .any(|contract| contract == &blocked_clause)
        })
        .collect::<Vec<_>>();
    let repair_targets = repair_branches
        .iter()
        .filter(|branch| {
            branch
                .contracts
                .iter()
                .any(|contract| contract == &blocked_clause)
        })
        .collect::<Vec<_>>();
    if base_targets.len() != 1 || repair_targets.len() != 1 {
        return Err(Fault::new(
            FaultCode::WrongProtocol,
            "the blocked precondition must identify exactly one protocol branch",
        ));
    }
    let base_target_operation = &base_targets[0].operation;
    let repair_target_operation = &repair_targets[0].operation;
    if base_target_operation != repair_target_operation
        || base_branches
            .iter()
            .map(|branch| (&branch.operation, &branch.contracts))
            .ne(repair_branches
                .iter()
                .map(|branch| (&branch.operation, &branch.contracts)))
    {
        return Err(Fault::new(
            FaultCode::WrongProtocol,
            "repair changed a branch operation or contract",
        ));
    }

    let base_terminals = direct_nodes(&base_parsed, &base_protocol, Production::ProtocolTerminal);
    let repair_terminals = direct_nodes(
        &repair_parsed,
        &repair_protocol,
        Production::ProtocolTerminal,
    );
    if base_terminals.is_empty() && repair_terminals.len() == 1 {
        let span = repair_terminals[0].span();
        let predicate = direct_nodes(&repair_parsed, repair_terminals[0], Production::Block);
        if predicate.len() == 1
            && significant_tokens(&base_parsed)
                .into_iter()
                .map(|(_, _, token)| token)
                .collect::<Vec<_>>()
                == tokens_outside(&repair_parsed, (span.start, span.end))
        {
            let binder = protocol_binder(&repair_parsed, &repair_protocol)?;
            return Ok(RepairPredicate::Terminal {
                expression: predicate_tokens(&repair_parsed, predicate[0]),
                binder,
            });
        }
    }

    if base_terminals.is_empty() && repair_terminals.is_empty() {
        let base_block = precondition_block(&base_parsed, base_target_operation)?;
        let repair_block = precondition_block(&repair_parsed, repair_target_operation)?;
        let base_span = base_block.span();
        let repair_span = repair_block.span();
        let base_body = tokens_in(&base_parsed, base_block);
        let repair_body = tokens_in(&repair_parsed, repair_block);
        if base_body != repair_body
            && tokens_outside(&base_parsed, (base_span.start, base_span.end))
                == tokens_outside(&repair_parsed, (repair_span.start, repair_span.end))
        {
            return Ok(RepairPredicate::Precondition {
                expression: predicate_tokens(&repair_parsed, repair_block),
            });
        }
    }
    Err(Fault::new(
        FaultCode::WrongProtocol,
        "repair may add one non-vacuous terminal member or change only the blocked branch precondition",
    ))
}

fn protocol_binder(parsed: &ParsedSource, protocol: &CstNode) -> Checked<String> {
    let tokens = tokens_in(parsed, protocol);
    let binder = tokens
        .iter()
        .position(|token| token == "over")
        .and_then(|over| tokens.get(over + 2))
        .filter(|name| name.as_str() != ")")
        .cloned()
        .ok_or_else(|| Fault::new(FaultCode::WrongProtocol, "Fill must bind its cell"))?;
    Ok(binder)
}

fn predicate_tokens(parsed: &ParsedSource, block: &CstNode) -> Vec<String> {
    tokens_in(parsed, block)
        .into_iter()
        .filter(|token| !matches!(token.as_str(), "{" | "}"))
        .collect()
}

fn replace_path_reference(tokens: &[String], reference: &[String]) -> Checked<String> {
    if reference.is_empty() {
        return Err(Fault::new(
            FaultCode::WrongProtocol,
            "predicate binder is empty",
        ));
    }
    let mut rewritten = Vec::with_capacity(tokens.len());
    let mut index = 0;
    while index < tokens.len() {
        if tokens[index..].starts_with(reference) {
            rewritten.push("probeValue".to_owned());
            index += reference.len();
        } else {
            rewritten.push(tokens[index].clone());
            index += 1;
        }
    }
    Ok(rewritten.join(" "))
}

fn predicate_run_request(value: u8) -> Value {
    json!({"format":"native-run/1","request":{"models":[],"program":{"source":{"file":"predicate.fsl","authority":"eval","identity":"protocol-deadlock-predicate","document":"ProtocolPredicate","formal_revision":1}},"call":{"function":"checkPredicate","arguments":[{"parameter":"probeValue","value":{"type":"integer","value":value.to_string()}}]}}})
}

fn evaluate_predicate(tools: &Tools, expression: &str, deadline: Instant) -> Checked<[bool; 4]> {
    let probe = tempfile::tempdir().map_err(|error| {
        Fault::new(
            FaultCode::ToolFailure,
            format!("cannot create predicate probe: {error}"),
        )
    })?;
    let source = format!(
        "language \"ix:native\" edition \"1-draft\";\nfunction checkPredicate(probeValue: Int[0,3]): Boolean pure {{ {expression} }}\n"
    );
    fs::write(probe.path().join("predicate.fsl"), source).map_err(|error| {
        Fault::new(
            FaultCode::ToolFailure,
            format!("cannot write predicate probe: {error}"),
        )
    })?;
    let mut samples = [false; 4];
    for value in 0..=3 {
        before_deadline(deadline).map_err(command_failure)?;
        fs::write(
            probe.path().join("run.json"),
            predicate_run_request(value).to_string(),
        )
        .map_err(|error| {
            Fault::new(
                FaultCode::ToolFailure,
                format!("cannot write run request: {error}"),
            )
        })?;
        let mut command = Command::new(&tools.fsl);
        command
            .current_dir(probe.path())
            .args(["run", "run.json", "--format", "json"]);
        let output = capture_command(&mut command, deadline).map_err(command_failure)?;
        let outcome: Value = serde_json::from_slice(&output.stdout).map_err(|_| {
            Fault::new(
                FaultCode::ToolFailure,
                "fsl run predicate probe did not emit JSON",
            )
        })?;
        if output.status.code() != Some(0)
            || outcome["category"] != "success"
            || outcome["result"]["kind"] != "completed"
        {
            return Err(Fault::new(
                FaultCode::CliGap,
                format!("fsl run could not evaluate repair predicate at v={value}"),
            ));
        }
        samples[usize::from(value)] =
            outcome["result"]["value"]["value"]
                .as_bool()
                .ok_or_else(|| {
                    Fault::new(
                        FaultCode::ToolFailure,
                        "fsl run predicate probe did not return a Boolean",
                    )
                })?;
    }
    Ok(samples)
}

fn deadlock_cell_value(deadlock: &Value) -> Checked<u8> {
    let key = deadlock
        .pointer("/items/0/counterexample/prefix")
        .and_then(Value::as_array)
        .and_then(|prefix| prefix.last())
        .and_then(|step| step.pointer("/post_state/key"))
        .and_then(Value::as_str)
        .ok_or_else(|| Fault::new(FaultCode::WrongDeadlock, "deadlock state key is missing"))?;
    let bytes = URL_SAFE_NO_PAD.decode(key).map_err(|_| {
        Fault::new(
            FaultCode::WrongDeadlock,
            "deadlock state key is invalid base64",
        )
    })?;
    let state: Value = serde_json::from_slice(&bytes).map_err(|_| {
        Fault::new(
            FaultCode::WrongDeadlock,
            "deadlock state key is invalid JSON",
        )
    })?;
    let mut values = Vec::new();
    collect_integer_field(&state, "v", &mut values);
    let [value] = values.as_slice() else {
        return Err(Fault::new(
            FaultCode::WrongDeadlock,
            "deadlock state must identify one cell value",
        ));
    };
    u8::try_from(*value)
        .ok()
        .filter(|value| *value <= 3)
        .ok_or_else(|| Fault::new(FaultCode::WrongDeadlock, "deadlock value is outside 0..3"))
}

fn collect_integer_field(value: &Value, field_name: &str, found: &mut Vec<i64>) {
    match value {
        Value::Object(object) => {
            if object.get("name").and_then(Value::as_str) == Some(field_name)
                && let Some(field) = object.get("value")
                && field.get("type").and_then(Value::as_str) == Some("integer")
                && let Some(integer) = field
                    .get("value")
                    .and_then(Value::as_str)
                    .and_then(|value| value.parse::<i64>().ok())
            {
                found.push(integer);
            }
            for child in object.values() {
                collect_integer_field(child, field_name, found);
            }
        }
        Value::Array(array) => {
            for child in array {
                collect_integer_field(child, field_name, found);
            }
        }
        _ => {}
    }
}

fn validate_predicate_samples(samples: &[bool; 4], deadlock_value: u8) -> Checked<()> {
    if samples.iter().all(|sample| *sample) {
        return Err(Fault::new(
            FaultCode::WrongProtocol,
            "repair predicate is true throughout the cell's 0..3 domain",
        ));
    }
    if !samples[usize::from(deadlock_value)] {
        return Err(Fault::new(
            FaultCode::WrongProtocol,
            "repair predicate must hold at the blocked state",
        ));
    }
    Ok(())
}

fn check_repair_predicate(
    tools: &Tools,
    repair: &RepairPredicate,
    deadlock: &Value,
    deadline: Instant,
) -> Checked<()> {
    let deadlock_value = deadlock_cell_value(deadlock)?;
    let expression = probe_expression(repair)?;
    let samples = evaluate_predicate(tools, &expression, deadline)?;
    validate_predicate_samples(&samples, deadlock_value)
}

fn probe_expression(repair: &RepairPredicate) -> Checked<String> {
    match repair {
        RepairPredicate::Terminal { expression, binder } => replace_path_reference(
            expression,
            &[binder.clone(), ".".to_owned(), "v".to_owned()],
        ),
        RepairPredicate::Precondition { expression } => replace_path_reference(
            expression,
            &["self".to_owned(), ".".to_owned(), "v".to_owned()],
        ),
    }
}

fn command_failure(error: CaptureError) -> Fault {
    let code = match error.code {
        CaptureCode::AssertionTimeout => FaultCode::Timeout,
        CaptureCode::OutputLimit | CaptureCode::ToolFailure => FaultCode::ToolFailure,
    };
    Fault::new(code, error.to_string())
}

fn analyze(
    workspace: &Path,
    tools: &Tools,
    request: &str,
    expected_exit: i32,
    deadline: Instant,
) -> Checked<Value> {
    before_deadline(deadline).map_err(command_failure)?;
    let mut command = Command::new(&tools.fsl);
    command
        .current_dir(workspace)
        .args(["analyze", request, "--format", "json"]);
    let output = capture_command(&mut command, deadline).map_err(command_failure)?;
    if output.status.code().is_none() {
        return Err(Fault::new(
            FaultCode::ToolFailure,
            format!("fsl analyze {request} terminated without an exit code"),
        ));
    }
    let document: Value = serde_json::from_slice(&output.stdout).map_err(|_| {
        Fault::new(
            FaultCode::ToolFailure,
            format!("fsl analyze {request} did not emit a JSON outcome"),
        )
    })?;
    if output.status.code() != Some(expected_exit) {
        if is_cli_gap(&document) {
            return Err(Fault::new(
                FaultCode::CliGap,
                format!("fsl analyze {request} returned an unsupported or incomplete outcome"),
            ));
        }
        return Err(Fault::new(
            if expected_exit == 10 {
                FaultCode::WrongDeadlock
            } else {
                FaultCode::WrongProof
            },
            format!("fsl analyze {request} did not produce the required outcome"),
        ));
    }
    if is_cli_gap(&document) {
        return Err(Fault::new(
            FaultCode::CliGap,
            format!("fsl analyze {request} returned an unsupported outcome"),
        ));
    }
    if expected_exit == 10 {
        check_deadlock(&document)?;
    } else {
        check_proof(&document)?;
    }
    Ok(document)
}

fn is_cli_gap(document: &Value) -> bool {
    document["category"] == "unsupported"
        || document["category"] == "incomplete"
        || document["items"].as_array().is_some_and(|items| {
            items
                .iter()
                .any(|item| item["record"]["result"] == "unsupported")
        })
}

fn one_item(document: &Value) -> Checked<&Value> {
    document["items"]
        .as_array()
        .filter(|items| items.len() == 1)
        .map(|items| &items[0])
        .ok_or_else(|| Fault::new(FaultCode::WrongDeadlock, "expected one analysis item"))
}

fn check_deadlock(document: &Value) -> Checked<&Value> {
    let item = one_item(document)?;
    if document["format"] != "filament-outcome/1"
        || document["category"] != "violation"
        || item["category"] != "violation"
        || item["claim_name"] != "Fill"
        || item["record"]["result"] != "refuted"
    {
        return Err(Fault::new(
            FaultCode::WrongDeadlock,
            "the baseline must settle the deadlock-freedom item as refuted",
        ));
    }
    let counterexample = &item["counterexample"];
    let Some(prefix) = counterexample["prefix"].as_array() else {
        return Err(Fault::new(
            FaultCode::WrongDeadlock,
            "counterexample prefix is missing",
        ));
    };
    let Some(blocked) = counterexample["blocked"].as_array() else {
        return Err(Fault::new(
            FaultCode::WrongDeadlock,
            "blocked thread list is missing",
        ));
    };
    let precondition = blocked
        .iter()
        .find(|entry| entry["cause"]["type"] == "precondition");
    let join_waiting = blocked
        .iter()
        .find(|entry| entry["cause"]["type"] == "join-waiting");
    let precondition_thread = precondition.map(|entry| &entry["thread"]);
    let valid_precondition_thread = precondition_thread.is_some_and(|thread| {
        thread["type"] == "thread-path"
            && thread["instance"]
                .as_str()
                .is_some_and(|instance| !instance.is_empty())
            && thread["steps"].as_array().is_some_and(|steps| {
                steps.len() == 1
                    && steps[0]["branch"]
                        .as_str()
                        .is_some_and(|branch| !branch.is_empty())
                    && steps[0]["node"]
                        .as_str()
                        .is_some_and(|node| !node.is_empty())
                    && steps[0]["ordinal"]
                        .as_str()
                        .is_some_and(|ordinal| !ordinal.is_empty())
            })
    });
    let valid_join_waiting = join_waiting.is_some_and(|entry| {
        entry["thread"]["type"] == "thread-path"
            && entry["thread"]["instance"]
                .as_str()
                .is_some_and(|instance| !instance.is_empty())
            && entry["thread"]["steps"]
                .as_array()
                .is_some_and(Vec::is_empty)
            && entry["cause"]["threads"].as_array().is_some_and(|threads| {
                threads.len() == 1
                    && precondition_thread.is_some_and(|thread| threads[0] == *thread)
            })
    });
    if counterexample["kind"] != "deadlock"
        || prefix.len() != 2
        || blocked.len() != 2
        || prefix[0]["transition"]["step"] != "fork"
        || !prefix[0]["transition"]["node"]
            .as_str()
            .is_some_and(|node| !node.is_empty())
        || prefix[1]["transition"]["step"] != "attempt"
        || !prefix[1]["transition"]["node"]
            .as_str()
            .is_some_and(|node| !node.is_empty())
        || !valid_precondition_thread
        || !valid_join_waiting
        || !precondition.is_some_and(|entry| {
            entry["cause"]["clause"]
                .as_str()
                .is_some_and(|clause| !clause.is_empty())
                && entry["rest"]["kind"] == "node"
                && entry["rest"]["node"]
                    .as_str()
                    .is_some_and(|node| !node.is_empty())
        })
        || !join_waiting.is_some_and(|entry| {
            entry["rest"]["kind"] == "join"
                && entry["rest"]["node"]
                    .as_str()
                    .is_some_and(|node| !node.is_empty())
        })
    {
        return Err(Fault::new(
            FaultCode::WrongDeadlock,
            "expected a fork/attempt deadlock with a blocked precondition and join waiter",
        ));
    }
    Ok(item)
}

fn check_proof(document: &Value) -> Checked<&Value> {
    let item = one_item(document)?;
    if document["format"] != "filament-outcome/1"
        || document["category"] != "success"
        || item["category"] != "success"
        || item["claim_name"] != "Fill"
        || item["record"]["result"] != "proved"
    {
        return Err(Fault::new(
            FaultCode::WrongProof,
            "the repaired deadlock-freedom item must settle as proved",
        ));
    }
    if item["record"]["basis"]["type"] != "exhaustive"
        || item["record"]["certification"] != "certified"
    {
        return Err(Fault::new(
            FaultCode::WrongProof,
            "the repaired protocol needs a certified exhaustive proof",
        ));
    }
    Ok(item)
}

fn compare_items(saved: &Value, trusted: &Value, repaired: bool) -> Checked<()> {
    let saved_item = if repaired {
        check_proof(saved)?
    } else {
        check_deadlock(saved)?
    };
    let trusted_item = if repaired {
        check_proof(trusted)?
    } else {
        check_deadlock(trusted)?
    };
    for field in [
        "claim_name",
        "category",
        "request_index",
        "request",
        "record",
        "counterexample",
        "certificate",
    ] {
        if saved_item[field] != trusted_item[field] {
            return Err(Fault::new(
                if repaired {
                    FaultCode::WrongProof
                } else {
                    FaultCode::WrongDeadlock
                },
                format!("saved analysis field {field} differs from trusted fsl analyze"),
            ));
        }
    }
    Ok(())
}

fn check_summary(workspace: &Path, deadlock: &Value, repaired: &Value) -> Checked<()> {
    let actual = json_file(workspace, "result.json")?;
    let deadlock_item = check_deadlock(deadlock)?;
    let repaired_item = check_proof(repaired)?;
    let counterexample = &deadlock_item["counterexample"];
    let prefix = counterexample["prefix"]
        .as_array()
        .ok_or_else(|| Fault::new(FaultCode::WrongDeadlock, "counterexample prefix is missing"))?
        .iter()
        .map(|step| {
            let transition = &step["transition"];
            json!({
                "step": transition.get("step").cloned().unwrap_or(Value::Null),
                "node": transition.get("node").cloned().unwrap_or(Value::Null),
            })
        })
        .collect::<Vec<_>>();
    let blocked = counterexample["blocked"]
        .as_array()
        .ok_or_else(|| Fault::new(FaultCode::WrongDeadlock, "blocked threads are missing"))?
        .iter()
        .map(|entry| {
            json!({
                "thread": entry.get("thread").cloned().unwrap_or(Value::Null),
                "rest": entry.get("rest").cloned().unwrap_or(Value::Null),
                "cause": entry.get("cause").cloned().unwrap_or(Value::Null),
            })
        })
        .collect::<Vec<_>>();
    let expected = json!({
        "deadlock": {
            "exit_code": 10,
            "result": "refuted",
            "prefix": prefix,
            "blocked": blocked,
        },
        "repaired": {
            "exit_code": 0,
            "result": "proved",
            "basis": repaired_item["record"]["basis"],
            "certification": repaired_item["record"]["certification"],
        }
    });
    if !summary_matches(&actual, &expected) {
        return Err(Fault::new(
            FaultCode::WrongSummary,
            "result.json must summarize the structured CLI output",
        ));
    }
    Ok(())
}

fn summary_matches(actual: &Value, expected: &Value) -> bool {
    if actual["deadlock"]["exit_code"] != expected["deadlock"]["exit_code"]
        || actual["deadlock"]["result"] != expected["deadlock"]["result"]
        || actual["deadlock"]["prefix"] != expected["deadlock"]["prefix"]
        || !unordered_json_eq(
            &actual["deadlock"]["blocked"],
            &expected["deadlock"]["blocked"],
        )
    {
        return false;
    }
    ["exit_code", "result", "basis", "certification"]
        .iter()
        .all(|field| actual["repaired"][field] == expected["repaired"][field])
}

fn unordered_json_eq(actual: &Value, expected: &Value) -> bool {
    let (Some(actual), Some(expected)) = (actual.as_array(), expected.as_array()) else {
        return false;
    };
    if actual.len() != expected.len() {
        return false;
    }
    let mut unmatched = expected.to_vec();
    for entry in actual {
        let Some(index) = unmatched.iter().position(|candidate| candidate == entry) else {
            return false;
        };
        unmatched.remove(index);
    }
    unmatched.is_empty()
}

fn assess(workspace: &Path, tools: &Tools, deadline: Instant) -> Checked<BTreeMap<String, Value>> {
    before_deadline(deadline).map_err(command_failure)?;
    let (baseline_source, repaired_source) = check_inputs(workspace)?;
    let trusted_deadlock = analyze(workspace, tools, "deadlock-request.json", 10, deadline)?;
    let trusted_repaired = analyze(workspace, tools, "repaired-request.json", 0, deadline)?;
    let repair = check_protocol_repair(&baseline_source, &repaired_source, &trusted_deadlock)?;
    check_repair_predicate(tools, &repair, &trusted_deadlock, deadline)?;
    let saved_deadlock = json_file(workspace, "deadlock.json")?;
    let saved_repaired = json_file(workspace, "repaired.json")?;
    check_deadlock(&saved_deadlock)?;
    check_proof(&saved_repaired)?;
    compare_items(&saved_deadlock, &trusted_deadlock, false)?;
    compare_items(&saved_repaired, &trusted_repaired, true)?;
    check_summary(workspace, &trusted_deadlock, &trusted_repaired)?;
    before_deadline(deadline).map_err(command_failure)?;
    Ok(BTreeMap::from([
        ("modeled_protocol_and_repair_match".into(), json!(true)),
        (
            "baseline_deadlock_matches_independent_analysis".into(),
            json!(true),
        ),
        (
            "repaired_exhaustive_proof_matches_independent_analysis".into(),
            json!(true),
        ),
    ]))
}

/// Independently verify the baseline deadlock and repaired terminal-state proof.
pub fn grade(workspace: &Path, tools: &Tools) -> ScenarioResult<Assertion> {
    let deadline = Instant::now() + ASSERT_BUDGET;
    match assess(workspace, tools, deadline) {
        Ok(checks) => Ok(Assertion {
            ok: true,
            checks,
            failures: Vec::new(),
        }),
        Err(fault) => Ok(Assertion {
            ok: false,
            checks: BTreeMap::new(),
            failures: vec![match fault.code {
                FaultCode::ToolFailure | FaultCode::Timeout => format!("harness-error: {fault}"),
                FaultCode::CliGap => format!("cli-gap: {fault}"),
                _ => format!("{fault}"),
            }],
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    use tempfile::tempdir;

    const DEADLOCK_OUTCOME: &str =
        include_str!("../../fixtures/EV-006/reference/deadlock-outcome.json");
    const TERMINAL_OUTCOME: &str =
        include_str!("../../fixtures/EV-006/reference/repaired-outcome.json");
    const PRECONDITION_OUTCOME: &str =
        include_str!("../../fixtures/EV-006/reference/precondition-outcome.json");
    const RESULT_SUMMARY: &str =
        include_str!("../../fixtures/EV-006/reference/result-summary.json");
    const BASELINE_SOURCE: &str = include_str!("../../fixtures/EV-006/reference/baseline.native");

    fn terminal_repair() -> String {
        BASELINE_SOURCE.replace(
            "  role w on M::Cell;",
            "  role w on M::Cell;\n  terminal when { k.v = 1 };",
        )
    }

    fn precondition_repair() -> String {
        BASELINE_SOURCE.replace("self.v = 0", "self.v <= 1")
    }

    fn deadlock_result() -> Value {
        json!({"format":"filament-outcome/1","category":"violation","items":[{
            "category":"violation","claim_name":"Fill","record":{"result":"refuted"},
            "counterexample":{"kind":"deadlock","prefix":[
                {"transition":{"step":"fork","node":"Outer/ForkGroup"}},
                {"transition":{"step":"attempt","node":"Outer/ForkGroup/alpha/first"}}],
                "blocked":[
                    {"thread":{"instance":"0","steps":[{"branch":"beta","node":"Outer/ForkGroup","ordinal":"0"}],"type":"thread-path"},
                     "rest":{"kind":"node","node":"Outer/ForkGroup/beta/second","type":"rest"},
                     "cause":{"type":"precondition","clause":"GuardBeforeSecond"}},
                    {"thread":{"instance":"0","steps":[],"type":"thread-path"},
                     "rest":{"kind":"join","node":"Outer/ForkGroup","type":"rest"},
                     "cause":{"type":"join-waiting","threads":[{"instance":"0","steps":[{"branch":"beta","node":"Outer/ForkGroup","ordinal":"0"}],"type":"thread-path"}]}}
                ]}
        }]})
    }

    fn proof_result() -> Value {
        json!({"format":"filament-outcome/1","category":"success","items":[{
            "category":"success","claim_name":"Fill","record":{"result":"proved",
                "basis":{"type":"exhaustive"},"certification":"certified"},
            "certificate":{"accepted":true}
        }]})
    }

    fn write_stub(path: &Path, body: &str) -> ScenarioResult<()> {
        fs::write(path, body)?;
        let mut permissions = fs::metadata(path)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions)?;
        Ok(())
    }

    #[test]
    fn source_check_does_not_require_a_hidden_reference_implementation() -> ScenarioResult<()> {
        let temp = tempdir()?;
        for (name, contents) in [
            ("cell.semantic-ir.json", MODEL),
            ("start.json", START),
            ("deadlock-request.json", DEADLOCK_REQUEST),
            ("repaired-request.json", REPAIRED_REQUEST),
            ("baseline.native", BASELINE_SOURCE),
            ("protocol.native", &terminal_repair()),
        ] {
            fs::write(temp.path().join(name), contents)?;
        }
        assert!(check_inputs(temp.path()).is_ok());
        fs::write(temp.path().join("start.json"), "changed")?;
        assert!(matches!(
            check_inputs(temp.path()),
            Err(Fault {
                code: FaultCode::ChangedInput,
                ..
            })
        ));
        Ok(())
    }

    #[test]
    fn structural_repair_accepts_only_terminal_or_second_branch_precondition_changes() {
        let terminal = terminal_repair();
        let precondition = precondition_repair();
        let deadlock: Value = serde_json::from_str(DEADLOCK_OUTCOME).unwrap();
        let terminal_check = check_protocol_repair(BASELINE_SOURCE, &terminal, &deadlock);
        assert!(terminal_check.is_ok(), "{terminal_check:?}");
        let precondition_check = check_protocol_repair(BASELINE_SOURCE, &precondition, &deadlock);
        assert!(precondition_check.is_ok(), "{precondition_check:?}");

        let deleted_branch = BASELINE_SOURCE.replace(
            "      branch right attempt B by w on M::Cell::setTwo contracts [TwoPre, TwoPost] as (b: Boolean) { true };\n",
            "",
        );
        assert!(check_protocol_repair(BASELINE_SOURCE, &deleted_branch, &deadlock).is_err());

        let serialized = BASELINE_SOURCE.replace(
            "    parallel Both {\n      branch left attempt A by w on M::Cell::inc contracts [IncPre, IncPost] as (a: Boolean) { true };\n      branch right attempt B by w on M::Cell::setTwo contracts [TwoPre, TwoPost] as (b: Boolean) { true };\n    } join all [left, right];",
            "    attempt A by w on M::Cell::inc contracts [IncPre, IncPost] as (a: Boolean) { true };\n    attempt B by w on M::Cell::setTwo contracts [TwoPre, TwoPost] as (b: Boolean) { true };",
        );
        assert!(check_protocol_repair(BASELINE_SOURCE, &serialized, &deadlock).is_err());

        let trivial = BASELINE_SOURCE.replace(
            "    parallel Both {\n      branch left attempt A by w on M::Cell::inc contracts [IncPre, IncPost] as (a: Boolean) { true };\n      branch right attempt B by w on M::Cell::setTwo contracts [TwoPre, TwoPost] as (b: Boolean) { true };\n    } join all [left, right];",
            "    check Done using v { true };",
        );
        assert!(check_protocol_repair(BASELINE_SOURCE, &trivial, &deadlock).is_err());
    }

    #[test]
    fn vacuous_repairs_fail_and_blocked_branch_order_does_not_matter() {
        let deadlock: Value = serde_json::from_str(DEADLOCK_OUTCOME).unwrap();
        let terminal = terminal_repair();
        let precondition = precondition_repair();
        assert_eq!(deadlock_cell_value(&deadlock).unwrap(), 1);

        let terminal_repair = check_protocol_repair(BASELINE_SOURCE, &terminal, &deadlock)
            .expect("intended terminal structure is accepted");
        assert_eq!(
            probe_expression(&terminal_repair).unwrap(),
            "probeValue = 1"
        );
        assert!(validate_predicate_samples(&[false, true, false, false], 1).is_ok());

        let precondition_repair = check_protocol_repair(BASELINE_SOURCE, &precondition, &deadlock)
            .expect("blocked precondition structure is accepted");
        assert_eq!(
            probe_expression(&precondition_repair).unwrap(),
            "probeValue <= 1"
        );
        assert!(validate_predicate_samples(&[true, true, false, false], 1).is_ok());

        for (source, expected_probe, label) in [
            (terminal.replace("k.v = 1", "true"), "true", "terminal true"),
            (
                terminal.replace("k.v = 1", "k.v >= 0"),
                "probeValue >= 0",
                "terminal k.v >= 0",
            ),
            (
                terminal.replace("k.v = 1", "k.v = k.v"),
                "probeValue = probeValue",
                "terminal k.v = k.v",
            ),
            (
                terminal.replace("k.v = 1", "k.v = 1 or true"),
                "probeValue = 1 or true",
                "terminal k.v = 1 or true",
            ),
            (
                terminal.replace("k.v = 1", "not (k.v < 0)"),
                "not ( probeValue < 0 )",
                "terminal not (k.v < 0)",
            ),
            (
                precondition.replace("self.v <= 1", "self.v >= 0"),
                "probeValue >= 0",
                "precondition self.v >= 0",
            ),
            (
                precondition.replace("self.v <= 1", "self.v = self.v"),
                "probeValue = probeValue",
                "precondition self.v = self.v",
            ),
            (
                precondition.replace("self.v <= 1", "true"),
                "true",
                "precondition true",
            ),
        ] {
            let repair = check_protocol_repair(BASELINE_SOURCE, &source, &deadlock)
                .unwrap_or_else(|fault| panic!("{label}: {fault}"));
            assert_eq!(
                probe_expression(&repair).unwrap(),
                expected_probe,
                "{label}"
            );
            assert!(
                validate_predicate_samples(&[true; 4], 1).is_err(),
                "semantic tautology passed the bounded-domain check: {label}"
            );
        }

        let branches = "      branch left attempt A by w on M::Cell::inc contracts [IncPre, IncPost] as (a: Boolean) { true };\n      branch right attempt B by w on M::Cell::setTwo contracts [TwoPre, TwoPost] as (b: Boolean) { true };";
        let reversed = "      branch right attempt B by w on M::Cell::setTwo contracts [TwoPre, TwoPost] as (b: Boolean) { true };\n      branch left attempt A by w on M::Cell::inc contracts [IncPre, IncPost] as (a: Boolean) { true };";
        let reversed_baseline = BASELINE_SOURCE.replace(branches, reversed);
        let reversed_repair = reversed_baseline.replace("self.v = 0", "self.v <= 1");
        let reordered_check =
            check_protocol_repair(&reversed_baseline, &reversed_repair, &deadlock);
        assert!(reordered_check.is_ok(), "{reordered_check:?}");
    }

    #[test]
    fn deadlock_requires_the_expected_protocol_prefix_and_both_thread_paths() {
        let deadlock = deadlock_result();
        assert!(check_deadlock(&deadlock).is_ok());
        let mut missing_wait = deadlock.clone();
        missing_wait["items"][0]["counterexample"]["blocked"] = json!([]);
        assert!(check_deadlock(&missing_wait).is_err());
        let mut substring_trap = deadlock.clone();
        substring_trap["items"][0]["counterexample"]["blocked"][0]["cause"] =
            json!({"type":"precondition-like","clause":"GuardBeforeSecond"});
        assert!(check_deadlock(&substring_trap).is_err());
        let mut wrong_path = deadlock;
        wrong_path["items"][0]["counterexample"]["blocked"][0]["thread"]["steps"][0]["branch"] =
            json!("alpha");
        assert!(check_deadlock(&wrong_path).is_err());
    }

    #[test]
    fn repaired_result_requires_certified_exhaustive_proof() {
        let proved = proof_result();
        assert!(check_proof(&proved).is_ok());
        let mut uncertified = proved;
        uncertified["items"][0]["record"]["certification"] = json!("uncertified");
        assert!(check_proof(&uncertified).is_err());
    }

    #[test]
    fn summary_uses_cli_structures_and_ignores_blocked_order() -> ScenarioResult<()> {
        let temp = tempdir()?;
        let deadlock: Value = serde_json::from_str(DEADLOCK_OUTCOME)?;
        let repaired: Value = serde_json::from_str(TERMINAL_OUTCOME)?;
        fs::write(temp.path().join("result.json"), RESULT_SUMMARY)?;
        assert!(check_summary(temp.path(), &deadlock, &repaired).is_ok());

        let mut reordered: Value = serde_json::from_str(RESULT_SUMMARY)?;
        reordered["deadlock"]["blocked"]
            .as_array_mut()
            .unwrap()
            .reverse();
        fs::write(
            temp.path().join("result.json"),
            serde_json::to_vec_pretty(&reordered)?,
        )?;
        assert!(check_summary(temp.path(), &deadlock, &repaired).is_ok());

        reordered["deadlock"]["prefix"]
            .as_array_mut()
            .unwrap()
            .reverse();
        fs::write(
            temp.path().join("result.json"),
            serde_json::to_vec_pretty(&reordered)?,
        )?;
        assert!(check_summary(temp.path(), &deadlock, &repaired).is_err());
        Ok(())
    }

    #[test]
    fn compare_items_checks_trusted_fields() {
        let deadlock = deadlock_result();
        assert!(compare_items(&deadlock, &deadlock, false).is_ok());
        let mut changed = deadlock.clone();
        changed["items"][0]["claim_name"] = json!("Other");
        assert!(compare_items(&deadlock, &changed, false).is_err());
        let proof = proof_result();
        assert!(compare_items(&proof, &proof, true).is_ok());
        let mut changed_proof = proof.clone();
        changed_proof["items"][0]["certificate"]["accepted"] = json!(false);
        assert!(compare_items(&proof, &changed_proof, true).is_err());
    }

    #[test]
    fn full_grade_accepts_both_repairs_and_separates_agent_cli_and_harness_failures()
    -> ScenarioResult<()> {
        let precondition: Value = serde_json::from_str(PRECONDITION_OUTCOME)?;
        assert!(check_proof(&precondition).is_ok());
        let workspace = tempdir()?;
        let stub_dir = tempdir()?;
        let reference = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/EV-006/reference");
        prepare(
            workspace.path(),
            &Tools {
                fsl: Path::new("fsl").into(),
                docs: Path::new(".").into(),
                quire: Path::new("quire").into(),
                quoin: Path::new("quoin").into(),
            },
        )?;
        fs::write(workspace.path().join("baseline.native"), BASELINE_SOURCE)?;
        fs::write(workspace.path().join("protocol.native"), terminal_repair())?;
        fs::copy(
            reference.join("deadlock-outcome.json"),
            workspace.path().join("deadlock.json"),
        )?;
        fs::copy(
            reference.join("repaired-outcome.json"),
            workspace.path().join("repaired.json"),
        )?;
        fs::copy(
            reference.join("result-summary.json"),
            workspace.path().join("result.json"),
        )?;

        let fsl = stub_dir.path().join("fsl");
        let script = |repaired: &str| {
            format!(
                r#"#!/bin/sh
if [ "$1" = run ]; then
 value=$(sed -n 's/.*"value":"\([0-3]\)".*/\1/p' "$2")
 result=false
 if grep -Fq 'probeValue = 1' predicate.fsl; then [ "$value" = 1 ] && result=true; elif grep -Fq 'probeValue <= 1' predicate.fsl; then [ "$value" -le 1 ] && result=true; else exit 21; fi
 printf '{{"category":"success","result":{{"kind":"completed","value":{{"type":"boolean","value":%s}}}}}}' "$result"
 exit 0
fi
case "$2" in
 deadlock-request.json) cat '{0}'; exit 10 ;;
 repaired-request.json) cat '{1}'; exit 0 ;;
 *) exit 21 ;;
esac
"#,
                reference.join("deadlock-outcome.json").display(),
                reference.join(repaired).display(),
            )
        };
        write_stub(&fsl, &script("repaired-outcome.json"))?;
        let tools = Tools {
            fsl: fsl.clone(),
            docs: Path::new(".").into(),
            quire: Path::new("quire").into(),
            quoin: Path::new("quoin").into(),
        };
        let assertion = grade(workspace.path(), &tools)?;
        assert!(assertion.ok, "{:?}", assertion.failures);

        fs::write(
            workspace.path().join("protocol.native"),
            precondition_repair(),
        )?;
        write_stub(&fsl, &script("precondition-outcome.json"))?;
        let assertion = grade(workspace.path(), &tools)?;
        assert!(assertion.ok, "{:?}", assertion.failures);

        fs::write(workspace.path().join("deadlock.json"), r#"{"items":[]}"#)?;
        let assertion = grade(workspace.path(), &tools)?;
        assert!(!assertion.ok);
        assert!(!assertion.failures[0].starts_with("harness-error:"));
        assert!(!assertion.failures[0].starts_with("cli-gap:"));

        fs::write(
            workspace.path().join("deadlock.json"),
            r#"{"category":"unsupported","items":[]}"#,
        )?;
        write_stub(
            &fsl,
            "#!/bin/sh\nprintf '%s\\n' '{\"category\":\"unsupported\",\"items\":[]}'\nexit 10\n",
        )?;
        let assertion = grade(workspace.path(), &tools)?;
        assert!(assertion.failures[0].starts_with("cli-gap:"));

        write_stub(&fsl, "#!/bin/sh\nexit 127\n")?;
        let assertion = grade(workspace.path(), &tools)?;
        assert!(assertion.failures[0].starts_with("harness-error:"));

        write_stub(&fsl, "#!/bin/sh\nkill -TERM $$\n")?;
        let assertion = grade(workspace.path(), &tools)?;
        assert!(assertion.failures[0].starts_with("harness-error:"));
        Ok(())
    }
}
