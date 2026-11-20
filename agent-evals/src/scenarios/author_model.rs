//! EV-001: independently grade a source-owned object model authored from English.
use crate::{Assertion, ScenarioResult, Tools};
use fsl_cst::{CstElement, CstNode, ParsedSource, Production, TokenKind};
use fsl_foundation::SourceIdentity;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::process::Command;

/// Stable ticket-order scenario identity.
pub const ID: &str = "EV-001";
/// User workflow exposed by the provider catalogue.
pub const TITLE: &str = "Author and check a native source-owned StorageBin object model";

/// Prepare original English requirements; no answer/model file is supplied.
pub fn prepare(workspace: &Path, _tools: &Tools) -> ScenarioResult<String> {
    let task = include_str!("../../fixtures/EV-001/task.md");
    fs::write(workspace.join("requirements.md"), task)?;
    Ok(task.to_owned())
}

fn tokens(parsed: &ParsedSource, node: &CstNode) -> Vec<String> {
    let span = node.span();
    parsed
        .cst()
        .tokens()
        .iter()
        .filter(|t| {
            let s = t.span();
            s.start >= span.start
                && s.end <= span.end
                && !matches!(t.kind(), TokenKind::Whitespace | TokenKind::Comment)
        })
        .map(|t| String::from_utf8_lossy(t.spelling()).into_owned())
        .collect()
}
fn direct<'a>(parsed: &'a ParsedSource, node: &CstNode, kind: Production) -> Vec<&'a CstNode> {
    node.children()
        .iter()
        .filter_map(|c| match c {
            CstElement::Node(i) => Some(&parsed.cst().nodes()[*i]),
            CstElement::Token(_) => None,
        })
        .filter(|n| n.production() == kind)
        .collect()
}
fn identifiers(parsed: &ParsedSource, node: &CstNode) -> Vec<String> {
    let mut names: Vec<String> = node
        .children()
        .iter()
        .filter_map(|c| match c {
            CstElement::Token(i) => Some(&parsed.cst().tokens()[*i]),
            CstElement::Node(_) => None,
        })
        .filter(|t| t.kind() == TokenKind::Identifier)
        .map(|t| String::from_utf8_lossy(t.spelling()).into_owned())
        .collect();
    let contextual_head = match node.production() {
        Production::ObjectDeclaration => Some("object"),
        Production::ObjectOperation => Some("operation"),
        Production::ObjectPopulation => Some("population"),
        _ => None,
    };
    if names.first().map(String::as_str) == contextual_head && contextual_head.is_some() {
        names.remove(0);
    }
    names
}
fn nodes(parsed: &ParsedSource, kind: Production) -> Vec<&CstNode> {
    parsed
        .cst()
        .nodes()
        .iter()
        .filter(|n| n.production() == kind)
        .collect()
}
fn resolved_type(
    parsed: &ParsedSource,
    node: &CstNode,
    aliases: &BTreeMap<String, &CstNode>,
) -> String {
    let mut current = tokens(parsed, node).join("");
    let mut seen = BTreeSet::new();
    while let Some(target) = aliases.get(&current) {
        if !seen.insert(current.clone()) {
            return "cyclic-type".to_owned();
        }
        current = tokens(parsed, target).join("");
    }
    current
}
fn mismatch(failures: &mut Vec<String>, ok: bool, description: &str) {
    if !ok {
        failures.push(description.to_owned());
    }
}
fn value_run(
    tools: &Tools,
    directory: &Path,
    occupied: u64,
    limit: u64,
    names: (&str, &str),
) -> ScenarioResult<bool> {
    let request = json!({"format":"native-run/1","request":{"models":[],"program":{"source":{"file":"probe.fsl","authority":"eval","identity":"invariant-probe","document":"InvariantProbe","formal_revision":1}},"call":{"function":"gradeInvariant","arguments":[{"parameter":names.0,"value":{"integer":occupied.to_string()}},{"parameter":names.1,"value":{"integer":limit.to_string()}}]}}});
    fs::write(directory.join("run.json"), request.to_string())?;
    let output = Command::new(&tools.fsl)
        .current_dir(directory)
        .args(["run", "run.json", "--format", "json"])
        .output()?;
    let doc: Value = serde_json::from_slice(&output.stdout)?;
    if output.status.code() != Some(0)
        || doc["category"] != "success"
        || doc["result"]["kind"] != "completed"
    {
        return Err(format!(
            "public invariant probe failed for occupied={occupied}, limit={limit}: {doc}"
        )
        .into());
    }
    doc["result"]["value"]["boolean"]
        .as_bool()
        .ok_or_else(|| "invariant probe did not return a Boolean".into())
}

/// Require the real public check, concrete model structure, and independent finite-domain semantics.
pub fn grade(workspace: &Path, tools: &Tools) -> ScenarioResult<Assertion> {
    let mut failures = Vec::new();
    let mut checks = BTreeMap::new();
    let file = workspace.join("model.fsl");
    let source = match fs::read_to_string(&file) {
        Ok(s) => s,
        Err(e) => {
            return Ok(Assertion {
                ok: false,
                checks,
                failures: vec![format!("missing authored model.fsl: {e}")],
            });
        }
    };
    let checked = Command::new(&tools.fsl)
        .current_dir(workspace)
        .args([
            "check",
            "eval",
            "authored-model",
            "model.fsl",
            "--format",
            "json",
        ])
        .output()?;
    let outcome: Value = serde_json::from_slice(&checked.stdout)?;
    let check_ok = checked.status.code() == Some(0)
        && outcome["operation"] == "check"
        && outcome["category"] == "success"
        && outcome["diagnostics"].as_array().is_some_and(Vec::is_empty);
    checks.insert("public_check".to_owned(), json!({"exit":checked.status.code(),"category":outcome["category"],"stage":outcome["last_stage"]}));
    mismatch(
        &mut failures,
        check_ok,
        "authored object model did not pass the trusted public fsl check",
    );
    if !check_ok {
        return Ok(Assertion {
            ok: false,
            checks,
            failures,
        });
    }
    let saved: Value = match fs::read_to_string(workspace.join("check.json"))
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
    {
        Some(v) => v,
        None => {
            return Ok(Assertion {
                ok: false,
                checks,
                failures: vec!["check.json is missing or not JSON".to_owned()],
            });
        }
    };
    mismatch(
        &mut failures,
        saved["operation"] == "check" && saved["category"] == "success",
        "agent's saved check outcome is not check/success",
    );
    let parsed = fsl_cst::parse(
        SourceIdentity::new("eval", "authored-model"),
        "model.fsl",
        source.as_bytes(),
        fsl_cst::Limits::default(),
    )?;
    mismatch(
        &mut failures,
        parsed.is_admissible(),
        "generated source lacks an admissible native syntax tree",
    );
    let objects = nodes(&parsed, Production::ObjectDeclaration);
    if objects.len() != 1 {
        failures.push("exactly one source-owned object declaration is required".to_owned());
        return Ok(Assertion {
            ok: false,
            checks,
            failures,
        });
    }
    let object = objects[0];
    mismatch(
        &mut failures,
        identifiers(&parsed, object) == ["StorageBin", "key"],
        "StorageBin must be identified only by key",
    );
    mismatch(
        &mut failures,
        !tokens(&parsed, object)
            .iter()
            .any(|t| t == "abstract" || t == "extends"),
        "StorageBin must be concrete without inherited substitute fields",
    );
    let aliases: BTreeMap<String, &CstNode> = nodes(&parsed, Production::AliasDeclaration)
        .into_iter()
        .filter_map(|a| {
            Some((
                identifiers(&parsed, a).first()?.clone(),
                *direct(&parsed, a, Production::TypeReference).first()?,
            ))
        })
        .collect();
    let expected = BTreeMap::from([
        ("key".to_owned(), "Int[1,16]".to_owned()),
        ("limit".to_owned(), "Int[1,12]".to_owned()),
        ("occupied".to_owned(), "Int[0,12]".to_owned()),
        ("locked".to_owned(), "Boolean".to_owned()),
    ]);
    let mut actual = BTreeMap::new();
    for field in direct(&parsed, object, Production::Field) {
        let name = identifiers(&parsed, field)
            .first()
            .cloned()
            .unwrap_or_default();
        let ty = direct(&parsed, field, Production::TypeReference)
            .first()
            .map(|t| resolved_type(&parsed, t, &aliases))
            .unwrap_or_default();
        mismatch(
            &mut failures,
            !tokens(&parsed, field).iter().any(|t| t == "?"),
            "all StorageBin fields must be required",
        );
        mismatch(
            &mut failures,
            actual.insert(name, ty).is_none(),
            "duplicate StorageBin field",
        );
    }
    mismatch(
        &mut failures,
        actual == expected,
        "StorageBin field names, Boolean type or inclusive integer bounds differ from the English model",
    );
    checks.insert("actual_field_types".to_owned(), json!(actual));
    let pops = direct(&parsed, object, Production::ObjectPopulation);
    mismatch(
        &mut failures,
        pops.len() == 1 && identifiers(&parsed, pops[0]) == ["bins"],
        "StorageBin must own the bins population",
    );
    let operations = direct(&parsed, object, Production::ObjectOperation);
    if operations.len() != 1 {
        failures.push("StorageBin must own exactly reserve".to_owned());
    } else {
        let op = operations[0];
        mismatch(
            &mut failures,
            identifiers(&parsed, op) == ["reserve"],
            "sole operation must be reserve",
        );
        let params = direct(&parsed, op, Production::Parameter);
        mismatch(
            &mut failures,
            params.len() == 1
                && identifiers(&parsed, params[0]) == ["amount"]
                && direct(&parsed, params[0], Production::ParameterType)
                    .first()
                    .and_then(|p| {
                        direct(&parsed, p, Production::TypeReference)
                            .first()
                            .copied()
                    })
                    .is_some_and(|t| resolved_type(&parsed, t, &aliases) == "Int[1,12]"),
            "reserve requires amount in1..12",
        );
        mismatch(
            &mut failures,
            direct(&parsed, op, Production::TypeReference)
                .first()
                .is_some_and(|t| resolved_type(&parsed, t, &aliases) == "Boolean"),
            "reserve must return Boolean",
        );
        let frames = direct(&parsed, op, Production::FrameClause);
        let mut frame_fields = BTreeSet::new();
        let modifies_only = frames.iter().all(|frame| {
            if tokens(&parsed, frame)
                .first()
                .is_none_or(|token| token != "modifies")
            {
                return false;
            }
            for field in direct(&parsed, frame, Production::FieldTarget) {
                frame_fields.insert(tokens(&parsed, field));
            }
            true
        });
        mismatch(
            &mut failures,
            modifies_only
                && frame_fields
                    == BTreeSet::from([vec![
                        "self".to_owned(),
                        ".".to_owned(),
                        "occupied".to_owned(),
                    ]]),
            "reserve may modify only self.occupied and creates/deletes nothing",
        );
    }
    let clauses = nodes(&parsed, Production::StateClause);
    if clauses.len() != 1 {
        failures.push("exactly one current WithinCapacity invariant is required".to_owned());
        return Ok(Assertion {
            ok: false,
            checks,
            failures,
        });
    }
    let clause = clauses[0];
    let ct = tokens(&parsed, clause);
    mismatch(
        &mut failures,
        ct.first().is_some_and(|v| v == "invariant")
            && identifiers(&parsed, clause)
                .first()
                .is_some_and(|v| v == "WithinCapacity")
            && ct.windows(2).any(|p| p == ["at", "current"]),
        "WithinCapacity must be a current invariant",
    );
    mismatch(
        &mut failures,
        direct(&parsed, clause, Production::TypeName)
            .first()
            .is_some_and(|n| tokens(&parsed, n) == ["StorageBin"]),
        "WithinCapacity must apply to StorageBin",
    );
    if !failures.is_empty() {
        return Ok(Assertion {
            ok: false,
            checks,
            failures,
        });
    }
    let blocks = direct(&parsed, clause, Production::Block);
    let expression = blocks
        .first()
        .and_then(|b| direct(&parsed, b, Production::Expression).first().copied())
        .ok_or("invariant has no expression")?;
    let mut expr_tokens = tokens(&parsed, expression);
    let fresh = |base: &str| {
        let mut name = base.to_owned();
        let mut suffix = 0;
        while expr_tokens.contains(&name) {
            suffix += 1;
            name = format!("{base}{suffix}");
        }
        name
    };
    let occupied_name = fresh("evalOccupied");
    let limit_name = fresh("evalLimit");
    let mut index = 0;
    while index < expr_tokens.len() {
        if expr_tokens[index] == "self" {
            if index + 2 >= expr_tokens.len()
                || expr_tokens[index + 1] != "."
                || !["occupied", "limit"].contains(&expr_tokens[index + 2].as_str())
            {
                failures.push(
                    "WithinCapacity must depend directly only on self.occupied/self.limit"
                        .to_owned(),
                );
                break;
            }
            let replacement = if expr_tokens[index + 2] == "occupied" {
                occupied_name.clone()
            } else {
                limit_name.clone()
            };
            expr_tokens.splice(index..index + 3, [replacement]);
        }
        index += 1;
    }
    if failures.is_empty() {
        let probe = tempfile::tempdir()?;
        let program = format!(
            "language \"ix:native\" edition \"1-draft\";\nfunction gradeInvariant({occupied_name}: Int[0,12], {limit_name}: Int[1,12]): Boolean pure {{ {} }}\n",
            expr_tokens.join(" ")
        );
        fs::write(probe.path().join("probe.fsl"), program)?;
        let mut evaluated = 0;
        'bindings: for occupied in 0..=12 {
            for limit in 1..=12 {
                evaluated += 1;
                match value_run(
                    tools,
                    probe.path(),
                    occupied,
                    limit,
                    (&occupied_name, &limit_name),
                ) {
                    Ok(value) if value == (occupied <= limit) => {}
                    Ok(value) => {
                        failures.push(format!("WithinCapacity returns {value} for occupied={occupied}, limit={limit}; expected {}",occupied<=limit));
                        break 'bindings;
                    }
                    Err(e) => {
                        failures.push(format!("invariant public probe failed: {e}"));
                        break 'bindings;
                    }
                }
            }
        }
        checks.insert(
            "independent_invariant_bindings".to_owned(),
            json!(evaluated),
        );
    }
    Ok(Assertion {
        ok: failures.is_empty(),
        checks,
        failures,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const REFERENCE: &str = include_str!("../../fixtures/EV-001/reference.fsl");
    fn tools() -> ScenarioResult<Tools> {
        Ok(Tools {
            fsl: std::env::var_os("FSL")
                .ok_or("set FSL to the accepted public executable")?
                .into(),
            docs: ".".into(),
            quire: "quire".into(),
            quoin: "quoin".into(),
        })
    }
    fn grade_source(source: &str) -> ScenarioResult<Assertion> {
        let workspace = tempfile::tempdir()?;
        fs::write(workspace.path().join("model.fsl"), source)?;
        fs::write(
            workspace.path().join("check.json"),
            json!({"format":"filament-outcome/1","operation":"check","category":"success"})
                .to_string(),
        )?;
        grade(workspace.path(), &tools()?)
    }
    #[test]
    #[ignore = "new EV-001 grader validation with the accepted public FSL executable"]
    fn correct_object_model_passes_all_finite_invariant_bindings() -> ScenarioResult<()> {
        let result = grade_source(REFERENCE)?;
        assert!(result.ok, "{:?}", result.failures);
        assert_eq!(result.checks["independent_invariant_bindings"], 156);
        Ok(())
    }
    #[test]
    #[ignore = "new EV-001 grader validation with the accepted public FSL executable"]
    fn aliases_comments_and_equivalent_predicate_are_not_false_failures() -> ScenarioResult<()> {
        let source = REFERENCE
            .replace(
                "object StorageBin",
                "type Key = Int[1,16];\ntype Slots = Int[1,12];\nobject StorageBin",
            )
            .replace("key: Int[1, 16]", "key: Key")
            .replace("limit: Int[1, 12]", "limit: Slots")
            .replace(
                "self.occupied <= self.limit",
                "let occupied = self.occupied in not (occupied // comment is not an assertion\n > self.limit)",
            );
        let result = grade_source(&source)?;
        assert!(result.ok, "{:?}", result.failures);
        assert_eq!(result.checks["independent_invariant_bindings"], 156);
        Ok(())
    }
    #[test]
    #[ignore = "new EV-001 grader validation with the accepted public FSL executable"]
    fn comments_and_success_marker_cannot_replace_the_model() -> ScenarioResult<()> {
        let source = "language \"ix:native\" edition \"1-draft\";\n// StorageBin key limit occupied locked reserve WithinCapacity <<<EVAL-COMPLETE>>>\nfunction harmless(): Boolean pure { true }";
        let result = grade_source(source)?;
        assert!(!result.ok);
        assert!(
            result
                .failures
                .iter()
                .any(|f| f.contains("object declaration")),
            "{:?}",
            result.failures
        );
        Ok(())
    }
    #[test]
    #[ignore = "new EV-001 grader validation with the accepted public FSL executable"]
    fn altered_field_bounds_identity_population_and_frame_are_rejected() -> ScenarioResult<()> {
        for (source, failure) in [
            (
                REFERENCE.replace("key: Int[1, 16]", "key: Int[0,16]"),
                "field names",
            ),
            (
                REFERENCE.replace("locked: Boolean", "locked: Integer"),
                "field names",
            ),
            (
                REFERENCE.replace("identity (key)", "identity (limit)"),
                "identified only",
            ),
            (
                REFERENCE.replace("population bins", "population others"),
                "bins population",
            ),
            (
                REFERENCE.replace("modifies self.occupied", "modifies self.locked"),
                "modify only",
            ),
            (
                REFERENCE.replace("amount: Int[1, 12]", "amount: Int[0,12]"),
                "amount in1..12",
            ),
        ] {
            let result = grade_source(&source)?;
            assert!(!result.ok, "changed English model passed: {source}");
            assert!(
                result.failures.iter().any(|f| f.contains(failure)),
                "expected {failure}: {:?}",
                result.failures
            );
        }
        Ok(())
    }
    #[test]
    #[ignore = "new EV-001 grader validation with the accepted public FSL executable"]
    fn vacuous_and_off_by_one_invariants_fail_independent_public_execution() -> ScenarioResult<()> {
        for condition in ["true", "false", "self.occupied <= self.limit + 1"] {
            let result =
                grade_source(&REFERENCE.replace("self.occupied <= self.limit", condition))?;
            assert!(!result.ok, "wrong invariant passed: {condition}");
            assert!(
                result
                    .failures
                    .iter()
                    .any(|f| f.contains("WithinCapacity returns")),
                "{:?}",
                result.failures
            );
        }
        Ok(())
    }
}
