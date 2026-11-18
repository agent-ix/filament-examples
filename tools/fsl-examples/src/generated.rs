//! The files generated from the manifests and `expected/`: the recorded-output regions of the
//! system READMEs and `index.json`.
//!
//! A README shows recorded output between markers:
//!
//! ```text
//! <!-- output: <case-id> -->
//! (fenced block, rewritten from expected/<case-id>.json)
//! <!-- /output -->
//! ```

use crate::error::{Error, ErrorCode, Result};
use crate::manifest::{Case, Status};
use crate::outcome::{display, is_outcome, truthy};
use crate::runner::{Sink, System, absolute, read_expected, read_messages, write};
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::path::{Component, Path, PathBuf};

/// The longest message prefix a README shows.
const MESSAGE_CHARS: usize = 160;

/// The text of a README region for a case, from its recorded structure and messages.
pub fn render_output(case: &Case, structure: &Value, messages: &str) -> String {
    let head = format!("exit {}", case.expect_exit);
    if !is_outcome(structure) {
        return head;
    }
    let items = structure
        .get("items")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or_default();
    if !items.is_empty() {
        let mut lines = vec![head];
        for item in items {
            let mut rest = item.as_object().cloned().unwrap_or_default();
            for key in ["request_index", "cause", "decisive"] {
                rest.shift_remove(key);
            }
            if let Some(counterexample) = rest.shift_remove("counterexample")
                && truthy(&counterexample)
            {
                rest.insert(
                    "counterexample".to_owned(),
                    Value::String(format!("recorded in expected/{}.json", case.id)),
                );
            }
            lines.push(Value::Object(rest).to_string());
        }
        return lines.join("\n");
    }
    let category = structure.get("category").unwrap_or(&Value::Null);
    if let Some(result) = structure.get("result").filter(|r| truthy(r)) {
        let shown = result
            .get("value")
            .filter(|v| !v.is_null())
            .unwrap_or(result);
        return format!("{head}\nresult: {shown}");
    }
    if let Some(d) = structure.get("first_diagnostic").filter(|d| !d.is_null()) {
        let message: String = messages
            .split('\n')
            .next()
            .unwrap_or_default()
            .chars()
            .take(MESSAGE_CHARS)
            .collect();
        return format!(
            "{head}\n{}: {}: {message}",
            display(category),
            display(d.get("code").unwrap_or(&Value::Null))
        );
    }
    format!("{head}\n{}", display(category))
}

const OPEN: &str = "<!-- output: ";
const OPEN_END: &str = " -->\n```\n";
const CLOSE: &str = "\n```\n<!-- /output -->";

/// One region found in a README: its byte range and case id.
struct Region<'a> {
    start: usize,
    end: usize,
    id: &'a str,
}

fn find_regions(text: &str) -> Vec<Region<'_>> {
    let mut regions = Vec::new();
    let mut from = 0usize;
    while let Some(offset) = text.get(from..).and_then(|t| t.find(OPEN)) {
        let start = from + offset;
        let after = start + OPEN.len();
        let id_len = text
            .get(after..)
            .unwrap_or_default()
            .bytes()
            .take_while(|b| b.is_ascii_alphanumeric() || *b == b'_' || *b == b'-')
            .count();
        let body = after + id_len + OPEN_END.len();
        let found = if id_len > 0 && text.get(after + id_len..body) == Some(OPEN_END) {
            text.get(body..).and_then(|t| t.find(CLOSE))
        } else {
            None
        };
        match (found, text.get(after..after + id_len)) {
            (Some(close), Some(id)) => {
                let end = body + close + CLOSE.len();
                regions.push(Region { start, end, id });
                from = end;
            }
            _ => from = after,
        }
    }
    regions
}

/// The READMEs whose regions differ from `expected/`; rewrites them when `write_files` is set.
pub fn sync_readmes(systems: &[System], write_files: bool) -> Result<Vec<PathBuf>> {
    let mut stale = Vec::new();
    for system in systems {
        let file = system.dir.join("README.md");
        if !file.exists() {
            continue;
        }
        let old = fs::read_to_string(&file).map_err(|e| Error::io(&file, &e))?;
        let mut next = String::with_capacity(old.len());
        let mut copied = 0usize;
        for region in find_regions(&old) {
            let case = system
                .manifest
                .cases
                .iter()
                .find(|c| c.id == region.id)
                .ok_or_else(|| {
                    Error::new(
                        ErrorCode::Recorded,
                        format!(
                            "{}: output region for unknown case {}",
                            file.display(),
                            region.id
                        ),
                    )
                })?;
            let structure = read_expected(&system.dir, region.id)?.ok_or_else(|| {
                Error::new(
                    ErrorCode::Recorded,
                    format!("{}: expected/{}.json is missing", file.display(), region.id),
                )
            })?;
            let messages = read_messages(&system.dir, region.id)?;
            next.push_str(old.get(copied..region.start).unwrap_or_default());
            next.push_str(&format!(
                "{OPEN}{}{OPEN_END}{}{CLOSE}",
                region.id,
                render_output(case, &structure, &messages)
            ));
            copied = region.end;
        }
        next.push_str(old.get(copied..).unwrap_or_default());
        if next != old {
            if write_files {
                write(&file, &next)?;
            }
            stale.push(file);
        }
    }
    Ok(stale)
}

/// Regenerates the case and staged counts in the root README's Systems table from manifests.
/// The system's grouping label remains editorial, while both counts are derived data.
pub fn sync_system_counts(systems: &[System], root: &Path, write_files: bool) -> Result<bool> {
    let file = absolute(root)?.join("README.md");
    let old = fs::read_to_string(&file).map_err(|e| Error::io(&file, &e))?;
    let mut seen = Vec::new();
    let mut next = String::with_capacity(old.len());
    for line in old.split_inclusive('\n') {
        let rewritten = if let Some(system) = systems.iter().find(|system| {
            line.starts_with(&format!("| [{}](examples/{}) |", system.name, system.name))
        }) {
            let columns: Vec<_> = line.trim_end_matches('\n').split('|').collect();
            if columns.len() != 6 {
                return Err(Error::new(
                    ErrorCode::Recorded,
                    format!(
                        "{}: malformed Systems table row for {}",
                        file.display(),
                        system.name
                    ),
                ));
            }
            if seen.contains(&system.name.as_str()) {
                return Err(Error::new(
                    ErrorCode::Recorded,
                    format!(
                        "{}: duplicate Systems table row for {}",
                        file.display(),
                        system.name
                    ),
                ));
            }
            seen.push(system.name.as_str());
            let staged = system
                .manifest
                .cases
                .iter()
                .filter(|case| matches!(case.status, Status::Staged { .. }))
                .count();
            let running = system.manifest.cases.len() - staged;
            format!(
                "|{}|{}| {} | {} |\n",
                columns[1], columns[2], running, staged
            )
        } else {
            line.to_owned()
        };
        next.push_str(&rewritten);
    }
    if seen.len() != systems.len() {
        let missing = systems
            .iter()
            .filter(|system| !seen.contains(&system.name.as_str()))
            .map(|system| system.name.as_str())
            .collect::<Vec<_>>();
        return Err(Error::new(
            ErrorCode::Recorded,
            format!(
                "{}: Systems table is missing rows for {}",
                file.display(),
                missing.join(", ")
            ),
        ));
    }
    if next != old && write_files {
        write(&file, &next)?;
    }
    Ok(next != old)
}

/// One entry of `index.json`.
#[derive(Debug, Serialize)]
struct IndexEntry {
    system: String,
    case: String,
    status: &'static str,
    rule: Option<String>,
    statement: String,
    rule_file: String,
    data_files: Vec<String>,
    request_file: Option<String>,
    expected_file: String,
    verdict: Option<String>,
    exit: i32,
    ready_exit: Option<i32>,
    engine: Option<String>,
    clock: Option<String>,
    waits_on: Option<String>,
    needs: Option<String>,
}

#[derive(Debug, Serialize)]
struct Index {
    format: &'static str,
    cases: Vec<IndexEntry>,
}

/// The text of `index.json` for `systems`, with paths relative to `root`.
pub fn build_index(systems: &[System], root: &Path) -> Result<String> {
    let root = absolute(root)?;
    let rel = |p: &Path| -> Result<String> { Ok(relative(&root, &absolute(p)?)) };
    let mut cases = Vec::new();
    for system in systems {
        for case in &system.manifest.cases {
            let operand = case.command.split(' ').nth(1).unwrap_or_default();
            let request_file = operand.ends_with(".json").then(|| system.dir.join(operand));
            let mut data_files = Vec::new();
            if let Some(request_file) = &request_file {
                let text =
                    fs::read_to_string(request_file).map_err(|e| Error::io(request_file, &e))?;
                let request: Value = serde_json::from_str(&text).map_err(|e| {
                    Error::new(
                        ErrorCode::Recorded,
                        format!("{}: {e}", request_file.display()),
                    )
                })?;
                let base = request_file.parent().unwrap_or(&system.dir);
                if let Some(Value::Object(observations)) = request
                    .get("request")
                    .and_then(|r| r.get("clause"))
                    .and_then(|c| c.get("observations"))
                {
                    for value in observations.values() {
                        let Value::String(p) = value else {
                            return Err(Error::new(
                                ErrorCode::Recorded,
                                format!(
                                    "{}: an observation path is not a string",
                                    request_file.display()
                                ),
                            ));
                        };
                        data_files.push(rel(&base.join(p))?);
                    }
                }
            }
            let (verdict, waits_on, needs) = match &case.status {
                Status::Running { expect_verdict } => (Some(expect_verdict.clone()), None, None),
                Status::Staged {
                    waits_on, needs, ..
                } => (None, Some(waits_on.clone()), Some(needs.clone())),
            };
            cases.push(IndexEntry {
                system: system.name.clone(),
                case: case.id.clone(),
                status: case.status.name(),
                rule: case.rule.clone().filter(|r| !r.is_empty()),
                statement: case.statement.clone(),
                rule_file: rel(&system.dir.join(&case.file))?,
                data_files,
                request_file: request_file.as_deref().map(rel).transpose()?,
                expected_file: rel(&system
                    .dir
                    .join("expected")
                    .join(format!("{}.json", case.id)))?,
                verdict,
                exit: case.expect_exit,
                ready_exit: match case.status {
                    Status::Staged { ready_exit, .. } => Some(ready_exit),
                    Status::Running { .. } => None,
                },
                engine: case.engine.clone(),
                clock: case.clock.clone(),
                waits_on,
                needs,
            });
        }
    }
    let index = Index {
        format: "filament-examples-index/1",
        cases,
    };
    let text = serde_json::to_string_pretty(&index)
        .map_err(|e| Error::new(ErrorCode::Recorded, format!("index.json: {e}")))?;
    Ok(format!("{text}\n"))
}

/// `path` relative to `base`, both absolute and normalized, with `/` separators.
fn relative(base: &Path, path: &Path) -> String {
    let b: Vec<Component<'_>> = base.components().collect();
    let p: Vec<Component<'_>> = path.components().collect();
    let common = b.iter().zip(&p).take_while(|(x, y)| x == y).count();
    let mut parts: Vec<String> = Vec::new();
    parts.extend(b.iter().skip(common).map(|_| "..".to_owned()));
    parts.extend(
        p.iter()
            .skip(common)
            .map(|c| c.as_os_str().to_string_lossy().into_owned()),
    );
    parts.join("/")
}

/// The stale generated files, as messages; rewrites them when `write_files` is set.
pub fn sync_generated(
    systems: &[System],
    root: &Path,
    write_files: bool,
    sink: &mut dyn Sink,
) -> Result<Vec<String>> {
    let root = absolute(root)?;
    let mut problems = Vec::new();
    for file in sync_readmes(systems, write_files)? {
        problems.push(format!(
            "{}: recorded output is stale: merge origin/main, then run make readme",
            relative(&root, &file)
        ));
    }
    if sync_system_counts(systems, &root, write_files)? {
        problems.push(
            "README.md system counts are stale: merge origin/main, then run make readme".to_owned(),
        );
    }
    let index_file = root.join("index.json");
    let next = build_index(systems, &root)?;
    let current = fs::read_to_string(&index_file).ok();
    if current.as_deref() != Some(next.as_str()) {
        problems.push("index.json is stale: merge origin/main, then run make index".to_owned());
        if write_files {
            write(&index_file, &next)?;
        }
    }
    if write_files {
        sink.info(&format!("wrote {} generated file(s)", problems.len()));
    }
    Ok(problems)
}
