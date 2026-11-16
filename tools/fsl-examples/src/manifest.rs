//! `example.toml`: the manifest of one example system.
//!
//! A manifest has header keys (`name`, `summary`, `model` and the like) and one `[[case]]` table
//! per case. Every value is a string or an integer; anything else is refused.

use crate::error::{Error, ErrorCode, Result};
use toml::{Table, Value};

/// Whether a case runs in `make check` or waits on a ticket in `make staged`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    /// The case passes today and gives the recorded verdict.
    Running {
        /// The short verdict of the outcome, such as `holds`, `violated` or `refusal: ill_typed`.
        expect_verdict: String,
    },
    /// The case fails today the way it is recorded, until the feature it needs exists.
    Staged {
        /// The exit code the case gives once the feature exists.
        ready_exit: i32,
        /// The ticket the case waits on, such as `QSL-123`.
        waits_on: String,
        /// One line on why the case fails today.
        why: String,
        /// A short phrase for what the case needs.
        needs: String,
    },
}

impl Status {
    /// `running` or `staged`, as the manifest and the index spell it.
    pub fn name(&self) -> &'static str {
        match self {
            Status::Running { .. } => "running",
            Status::Staged { .. } => "staged",
        }
    }
}

/// One `[[case]]` entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Case {
    /// The case id, unique in its system.
    pub id: String,
    /// The rule the case exercises, when it names one.
    pub rule: Option<String>,
    /// One sentence stating the rule.
    pub statement: String,
    /// The FSL source file, relative to the system folder.
    pub file: String,
    /// The driver arguments, separated by spaces; `--format json` is added.
    pub command: String,
    /// The exit code the case gives today.
    pub expect_exit: i32,
    /// Running or staged, with the fields each needs.
    pub status: Status,
}

/// A parsed manifest.
#[derive(Debug, Clone)]
pub struct Manifest {
    /// The header keys.
    pub header: Table,
    /// The cases, in manifest order.
    pub cases: Vec<Case>,
}

/// Parses manifest text; `source` names the file in error messages.
pub fn parse_manifest(text: &str, source: &str) -> Result<Manifest> {
    let fail = |why: String| Error::new(ErrorCode::Manifest, format!("{source}: {why}"));
    let mut table: Table = toml::from_str(text).map_err(|e| fail(e.to_string()))?;
    let cases_value = table.remove("case");
    for (key, value) in &table {
        check_scalar(key, value).map_err(&fail)?;
    }
    let tables = match cases_value {
        None => Vec::new(),
        Some(Value::Array(items)) => items,
        Some(_) => return Err(fail("case must be written as [[case]] tables".to_owned())),
    };
    let mut cases: Vec<Case> = Vec::with_capacity(tables.len());
    for item in tables {
        let Value::Table(fields) = item else {
            return Err(fail("case must be written as [[case]] tables".to_owned()));
        };
        let case = read_case(&fields).map_err(&fail)?;
        if cases.iter().any(|c| c.id == case.id) {
            return Err(fail(format!("duplicate case id {}", case.id)));
        }
        cases.push(case);
    }
    Ok(Manifest {
        header: table,
        cases,
    })
}

fn check_scalar(key: &str, value: &Value) -> std::result::Result<(), String> {
    match value {
        Value::String(_) | Value::Integer(_) => Ok(()),
        _ => Err(format!("{key}: value must be a string or an integer")),
    }
}

fn read_case(fields: &Table) -> std::result::Result<Case, String> {
    let label = match fields.get("id") {
        Some(Value::String(id)) => id.clone(),
        _ => "?".to_owned(),
    };
    for (key, value) in fields {
        check_scalar(key, value).map_err(|why| format!("case {label}: {why}"))?;
    }
    let string = |key: &str| match fields.get(key) {
        Some(Value::String(s)) => Ok(s.clone()),
        _ => Err(format!("case {label}: {key} must be a string")),
    };
    let integer = |key: &str| match fields.get(key) {
        Some(Value::Integer(n)) => {
            i32::try_from(*n).map_err(|_| format!("case {label}: {key} is out of range"))
        }
        _ => Err(format!("case {label}: {key} must be a number")),
    };
    let id = string("id")?;
    let command = string("command")?;
    let statement = string("statement")?;
    let expect_exit = integer("expect_exit")?;
    let status_name = match fields.get("status") {
        Some(Value::String(s)) if s == "running" || s == "staged" => s.clone(),
        _ => return Err(format!("case {id}: status must be running or staged")),
    };
    let file = string("file")?;
    let rule = match fields.get("rule") {
        Some(Value::String(s)) => Some(s.clone()),
        _ => None,
    };
    let status = if status_name == "running" {
        Status::Running {
            expect_verdict: string("expect_verdict")?,
        }
    } else {
        let ready_exit = integer("ready_exit")?;
        let waits_on = string("waits_on")?;
        if !is_ticket_id(&waits_on) {
            return Err(format!(
                "case {id}: waits_on must be a ticket id such as QSL-123"
            ));
        }
        let why = string("why")?;
        let needs = string("needs")?;
        if needs.trim().is_empty() {
            return Err(format!(
                "case {id}: needs must be a short phrase a newcomer understands"
            ));
        }
        if ready_exit == expect_exit {
            return Err(format!("case {id}: ready_exit equals expect_exit"));
        }
        Status::Staged {
            ready_exit,
            waits_on,
            why,
            needs,
        }
    };
    Ok(Case {
        id,
        rule,
        statement,
        file,
        command,
        expect_exit,
        status,
    })
}

/// A ticket id: upper-case letters, a dash, digits.
fn is_ticket_id(text: &str) -> bool {
    match text.split_once('-') {
        Some((team, number)) => {
            !team.is_empty()
                && team.bytes().all(|b| b.is_ascii_uppercase())
                && !number.is_empty()
                && number.bytes().all(|b| b.is_ascii_digit())
        }
        None => false,
    }
}

/// Flips one staged case to running in the manifest text. It works line by line, so comments and
/// the order of the other cases stay as written.
pub fn promote_text(text: &str, id: &str, verdict: &str) -> Result<String> {
    let lines: Vec<&str> = text.split('\n').collect();
    let id_line = format!("id = {}", json_string(id));
    let start = lines
        .iter()
        .position(|l| *l == id_line)
        .ok_or_else(|| Error::new(ErrorCode::Usage, format!("case {id} not found in manifest")))?;
    let end = lines
        .iter()
        .enumerate()
        .skip(start + 1)
        .find(|(_, l)| l.trim() == "[[case]]")
        .map_or(lines.len(), |(i, _)| i);
    let block = lines.get(start..end).unwrap_or_default();
    let ready = block
        .iter()
        .find_map(|l| l.strip_prefix("ready_exit = "))
        .ok_or_else(|| Error::new(ErrorCode::Usage, format!("case {id} has no ready_exit")))?;
    let mut out: Vec<String> = Vec::with_capacity(block.len() + 1);
    for line in block {
        let dropped = ["ready_exit = ", "waits_on = ", "why = ", "needs = "]
            .iter()
            .any(|p| line.starts_with(p));
        if dropped {
            continue;
        }
        if line.starts_with("status = ") {
            out.push("status = \"running\"".to_owned());
        } else if line.starts_with("expect_exit = ") {
            out.push(format!("expect_exit = {ready}"));
            out.push(format!("expect_verdict = {}", json_string(verdict)));
        } else {
            out.push((*line).to_owned());
        }
    }
    let mut all: Vec<String> = Vec::with_capacity(lines.len() + 1);
    all.extend(lines.iter().take(start).map(|l| (*l).to_owned()));
    all.extend(out);
    all.extend(lines.iter().skip(end).map(|l| (*l).to_owned()));
    Ok(all.join("\n"))
}

fn json_string(text: &str) -> String {
    serde_json::Value::String(text.to_owned()).to_string()
}
