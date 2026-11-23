//! Thin Rust consumer of the SDK's published external-provider protocol.
use crate::{
    Assertion, ScenarioResult, Tools,
    scenarios::{author_model, blind_help, refusal_repair, safety_analyze, trace_monitor},
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

struct Entry {
    id: &'static str,
    title: &'static str,
    use_case: &'static str,
    prepare: fn(&Path, &Tools) -> ScenarioResult<String>,
    grade: fn(&Path, &Tools) -> ScenarioResult<Assertion>,
    commands: &'static [(&'static str, &'static str)],
    shipped_references: bool,
}
const ENTRIES: &[Entry] = &[
    Entry {
        id: author_model::ID,
        title: author_model::TITLE,
        use_case: "source-owned-model-authoring",
        prepare: author_model::prepare,
        grade: author_model::grade,
        commands: &[("check", "model.fsl")],
        shipped_references: true,
    },
    Entry {
        id: refusal_repair::ID,
        title: refusal_repair::TITLE,
        use_case: "refusal-only-program-repair",
        prepare: refusal_repair::prepare,
        grade: refusal_repair::grade,
        commands: &[("check", "postage.fsl"), ("run", "postage-example.json")],
        shipped_references: false,
    },
    Entry {
        id: blind_help::ID,
        title: blind_help::TITLE,
        use_case: "blind-help-supported-verbs",
        prepare: blind_help::prepare,
        grade: blind_help::grade,
        commands: &[
            ("check", "calculation.fsl"),
            ("run", "run-request.json"),
            ("analyze", "analyze-request.json"),
            ("monitor", "readings.jsonl"),
        ],
        shipped_references: false,
    },
    Entry {
        id: safety_analyze::ID,
        title: safety_analyze::TITLE,
        use_case: "finite-safety-proof-and-counterexample",
        prepare: safety_analyze::prepare,
        grade: safety_analyze::grade,
        commands: &[
            ("analyze", "holding-request.json"),
            ("analyze", "violating-request.json"),
        ],
        shipped_references: true,
    },
    Entry {
        id: trace_monitor::ID,
        title: trace_monitor::TITLE,
        use_case: "bounded-trace-and-open-prefix-monitoring",
        prepare: trace_monitor::prepare,
        grade: trace_monitor::grade,
        commands: &[
            ("monitor", "holding.jsonl"),
            ("monitor", "violating.jsonl"),
            ("monitor", "open.jsonl"),
        ],
        shipped_references: true,
    },
];

/// Whether the catalogue exposes this individual scenario for a live SDK run.
pub fn is_active_scenario(id: &str) -> bool {
    ENTRIES.iter().any(|entry| entry.id == id)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Metadata {
    id: String,
    #[serde(default, rename = "use_case")]
    _use_case: Option<String>,
    #[serde(default, rename = "title")]
    _title: Option<String>,
    #[serde(default, rename = "canary")]
    _canary: Option<bool>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Context {
    scenario: Metadata,
    work_dir: PathBuf,
    #[serde(rename = "cwd")]
    _cwd: PathBuf,
    #[serde(rename = "session_id")]
    _session_id: String,
    #[serde(rename = "report_dir")]
    _report_dir: PathBuf,
}
#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Operation {
    Describe,
    Prepare,
    Assert,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    protocol: String,
    operation: Operation,
    context: Context,
    #[serde(default, rename = "run")]
    _run: Option<RunnerResult>,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum ExitReason {
    Complete,
    Failed,
    Timeout,
    Exit,
    Error,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RunnerResult {
    #[serde(rename = "ok")]
    _ok: bool,
    #[serde(rename = "exitReason")]
    _exit_reason: ExitReason,
    #[serde(rename = "wallMs")]
    _wall_ms: u64,
    #[serde(rename = "screenTail")]
    _screen_tail: Option<String>,
    #[serde(rename = "stdout")]
    _stdout: Option<String>,
    #[serde(rename = "stderr")]
    _stderr: Option<String>,
    #[serde(rename = "exitCode")]
    _exit_code: Option<i32>,
}

fn executable(name: &str) -> ScenarioResult<PathBuf> {
    let path = Path::new(name);
    if path.components().count() > 1 {
        return Ok(fs::canonicalize(path)?);
    }
    std::env::split_paths(&std::env::var_os("PATH").ok_or("PATH is missing")?)
        .map(|p| p.join(name))
        .find(|p| p.is_file())
        .ok_or_else(|| format!("public executable {name} is not on PATH").into())
}
impl Tools {
    /// Resolve the configured current-main binary and shipped public tools/docs.
    pub fn from_environment() -> ScenarioResult<Self> {
        Ok(Self {
            fsl: fs::canonicalize(
                std::env::var_os("FSL").ok_or("set FSL to the accepted current-main executable")?,
            )?,
            docs: fs::canonicalize(
                std::env::var_os("FSL_DOCS_DIR")
                    .ok_or("set FSL_DOCS_DIR to the shipped documentation tree")?,
            )?,
            quire: executable("quire")?,
            quoin: executable("quoin")?,
        })
    }
}

fn collect_commands(value: &Value, commands: &mut Vec<String>) {
    if let Some(cmd) = value
        .get("cmd")
        .or_else(|| value.get("command"))
        .and_then(Value::as_str)
    {
        commands.push(cmd.to_owned());
    }
    if let Some(parts) = value.get("command").and_then(Value::as_array)
        && parts.iter().all(Value::is_string)
    {
        let words = parts.iter().filter_map(Value::as_str).collect::<Vec<_>>();
        if words.first().is_some_and(|p| {
            ["sh", "bash", "zsh"].contains(
                &Path::new(p)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .as_ref(),
            )
        }) && words.iter().any(|w| ["-c", "-lc"].contains(w))
        {
            if let Some(script) = words.last() {
                commands.push((*script).to_owned());
            }
        } else {
            commands.push(words.join(" "));
        }
    }
    if let Some(action) = value.get("action") {
        collect_commands(action, commands);
    }
}
/// Read actual tool-call commands from the SDK's retained transcript, never assistant prose.
struct RecordedInput {
    value: Value,
    persistent_cwd: bool,
    writes_files: bool,
    typed_execution: bool,
}
fn agent_inputs(workspace: &Path) -> ScenarioResult<Vec<RecordedInput>> {
    let directory = workspace.join(".cli-agent-evals/transcripts");
    let mut commands = Vec::new();
    let entries = fs::read_dir(&directory)
        .map_err(|e| format!("harness-error: SDK transcript is unavailable: {e}"))?;
    let mut found = false;
    for entry in entries {
        let file = entry?.path();
        if file.extension().is_none_or(|e| e != "transcript") {
            continue;
        }
        found = true;
        for line in fs::read_to_string(file)?
            .lines()
            .filter(|s| !s.trim().is_empty())
        {
            let record: Value = serde_json::from_str(line)
                .map_err(|e| format!("harness-error: invalid SDK transcript: {e}"))?;
            if let Some(content) = record.pointer("/message/content").and_then(Value::as_array) {
                for item in content {
                    if item["type"] == "tool_use"
                        && let Some(input) = item.get("input")
                    {
                        commands.push(RecordedInput {
                            value: input.clone(),
                            persistent_cwd: true,
                            typed_execution: false,
                            writes_files: matches!(
                                item["name"].as_str(),
                                Some("Write" | "Edit" | "NotebookEdit")
                            ),
                        });
                    }
                }
            }
            let payload = record.get("payload").unwrap_or(&record);
            // The current published SDK derives Codex command metrics from
            // these typed execution events. Consume argv/cwd, never stdout or
            // by evaluating opaque custom-tool JavaScript.
            if record["type"] == "event_msg"
                && payload["type"] == "item_completed"
                && payload["item"]["type"] == "CommandExecution"
            {
                let item = &payload["item"];
                let command = item
                    .get("command")
                    .and_then(Value::as_array)
                    .filter(|args| args.iter().all(Value::is_string))
                    .ok_or("harness-error: invalid command execution argv")?;
                let cwd = item["cwd"]
                    .as_str()
                    .ok_or("harness-error: execution cwd missing")?;
                let cwd = url::Url::parse(cwd)?
                    .to_file_path()
                    .map_err(|_| "harness-error: execution cwd is not a local file URI")?;
                commands.push(RecordedInput {
                    value: json!({"command":command,"workdir":cwd}),
                    persistent_cwd: false,
                    writes_files: false,
                    typed_execution: true,
                });
            } else if payload["type"] == "custom_tool_call" {
                if let Some(input) = payload["input"].as_str() {
                    commands.push(RecordedInput {
                        value: json!({"code":input}),
                        persistent_cwd: false,
                        typed_execution: false,
                        writes_files: payload["name"]
                            .as_str()
                            .is_some_and(|name| name.ends_with("apply_patch")),
                    });
                }
            } else if payload["type"] == "function_call" {
                if let Some(args) = payload["arguments"].as_str() {
                    let args: Value = serde_json::from_str(args)
                        .map_err(|e| format!("harness-error: invalid tool arguments: {e}"))?;
                    commands.push(RecordedInput {
                        value: args,
                        persistent_cwd: false,
                        typed_execution: false,
                        writes_files: payload["name"]
                            .as_str()
                            .is_some_and(|name| name.ends_with("apply_patch")),
                    });
                }
            } else if payload["type"] == "local_shell_call" {
                commands.push(RecordedInput {
                    value: payload.clone(),
                    persistent_cwd: false,
                    writes_files: false,
                    typed_execution: false,
                });
            }
        }
    }
    if !found {
        return Err("harness-error: SDK supplied no retained transcript".into());
    }
    Ok(commands)
}
/// Read actual shell tool calls, excluding assistant prose and tool results.
pub fn agent_commands(workspace: &Path) -> ScenarioResult<Vec<String>> {
    let mut commands = Vec::new();
    for input in agent_inputs(workspace)? {
        collect_commands(&input.value, &mut commands);
    }
    Ok(commands)
}

// Deliberately recognize literal simple shell commands only. This is evidence
// parsing, not a shell interpreter or an operating-system sandbox. Expansions,
// substitutions and unbalanced quoting cannot establish positive evidence.
#[derive(Debug, PartialEq, Eq)]
enum ShellToken {
    Word(String),
    Separator,
    Redirect(bool),
}
fn shell_tokens(script: &str) -> Option<Vec<ShellToken>> {
    let mut tokens = Vec::new();
    let mut word = String::new();
    let mut started = false;
    let mut quote = None;
    let mut escaped = false;
    let mut comment = false;
    for c in script.chars() {
        if comment && c != '\n' {
            continue;
        }
        comment = false;
        if escaped {
            word.push(c);
            escaped = false;
            continue;
        }
        if c == '\\' && quote != Some('\'') {
            escaped = true;
            started = true;
            continue;
        }
        if matches!(c, '\'' | '"') && (quote.is_none() || quote == Some(c)) {
            quote = if quote.is_some() { None } else { Some(c) };
            started = true;
            continue;
        }
        if quote != Some('\'') && matches!(c, '$' | '`') {
            return None;
        }
        if quote.is_none() {
            if c == '#' && !started {
                comment = true;
                continue;
            }
            if c.is_whitespace() || matches!(c, ';' | '|' | '&' | '<' | '>') {
                if started {
                    tokens.push(ShellToken::Word(std::mem::take(&mut word)));
                    started = false;
                }
                if matches!(c, '\n' | ';' | '|' | '&') {
                    tokens.push(ShellToken::Separator);
                } else if matches!(c, '<' | '>') {
                    tokens.push(ShellToken::Redirect(c == '>'));
                }
                continue;
            }
            if matches!(c, '(' | ')') {
                return None;
            }
        }
        word.push(c);
        started = true;
    }
    if escaped || quote.is_some() {
        return None;
    }
    if started {
        tokens.push(ShellToken::Word(word));
    }
    Some(tokens)
}

// Parse shell structure before recognizing literal argv. Here-document bodies
// are data, never statements. Compound control flow cannot prove unconditional
// execution, and shell-state modifiers cannot prove trusted program resolution.
// Nothing here executes or expands the recorded script.
fn command_evidence_tokens(script: &str) -> Option<Vec<ShellToken>> {
    if script.len() > 1_048_576 {
        return None;
    }
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_bash::LANGUAGE.into())
        .ok()?;
    let tree = parser.parse(script, None)?;
    let root = tree.root_node();
    if root.has_error() {
        return None;
    }
    let mut tokens = Vec::new();
    let mut cursor = root.walk();
    for statement in root.named_children(&mut cursor) {
        let command = match statement.kind() {
            "comment" => continue,
            "variable_assignment" => {
                let name = statement
                    .child_by_field_name("name")?
                    .utf8_text(script.as_bytes())
                    .ok()?;
                let value = statement
                    .child_by_field_name("value")?
                    .utf8_text(script.as_bytes())
                    .ok()?;
                if value == "$?"
                    && !matches!(
                        name,
                        "PATH"
                            | "CDPATH"
                            | "BASH_ENV"
                            | "ENV"
                            | "SHELLOPTS"
                            | "BASHOPTS"
                            | "IFS"
                            | "fsl"
                    )
                {
                    // Recording an exit code does not change literal command resolution.
                    continue;
                }
                break;
            }
            "command" => statement,
            "redirected_statement" => {
                let mut redirects = statement.walk();
                if statement
                    .children_by_field_name("redirect", &mut redirects)
                    .any(|redirect| {
                        (redirect.child_by_field_name("right").is_some()
                            || redirect.child_by_field_name("operator").is_some())
                            && !literal_stdout_capture(redirect, script)
                    })
                {
                    return None;
                }
                statement.child_by_field_name("body")?
            }
            _ => return None,
        };
        if command.kind() != "command" {
            return None;
        }
        let mut children = command.walk();
        if command
            .named_children(&mut children)
            .any(|child| child.kind() == "variable_assignment")
        {
            return None;
        }
        let Some(literal) = shell_tokens(command.utf8_text(script.as_bytes()).ok()?) else {
            // Later opaque formatting cannot invalidate an earlier unconditional call.
            break;
        };
        if let Some(ShellToken::Word(program)) = literal.first()
            && (matches!(program.as_str(), "exit" | "exec" | "return")
                || (program == "command"
                    && !matches!(literal.get(1), Some(ShellToken::Word(flag)) if matches!(flag.as_str(), "-v" | "-V"))))
        {
            // Keep earlier unconditional evidence, but never credit commands
            // after termination or an unrecognized command-builtin execution.
            break;
        }
        if let Some(ShellToken::Word(program)) = literal.first()
            && matches!(
                program.as_str(),
                "." | "source"
                    | "eval"
                    | "alias"
                    | "unalias"
                    | "hash"
                    | "builtin"
                    | "enable"
                    | "shopt"
                    | "set"
                    | "export"
                    | "unset"
                    | "declare"
                    | "typeset"
                    | "readonly"
                    | "read"
                    | "readarray"
                    | "mapfile"
                    | "trap"
            )
        {
            return None;
        }
        tokens.extend(literal);
        tokens.push(ShellToken::Separator);
    }
    Some(tokens)
}

fn literal_stdout_capture(redirect: tree_sitter::Node<'_>, script: &str) -> bool {
    if redirect.kind() != "file_redirect" {
        return false;
    }
    let Some(tokens) = redirect
        .utf8_text(script.as_bytes())
        .ok()
        .and_then(shell_tokens)
    else {
        return false;
    };
    match tokens.as_slice() {
        [ShellToken::Redirect(true), ShellToken::Word(_)] => true,
        [
            ShellToken::Word(fd),
            ShellToken::Redirect(true),
            ShellToken::Word(_),
        ] => fd == "1",
        _ => false,
    }
}
fn changes_tool_resolution(command: &str) -> bool {
    command
        .split_whitespace()
        .any(|word| word.starts_with("PATH=") || word == "PATH" || word.starts_with("fsl="))
        || command.contains("alias fsl")
        || command.contains("hash -p")
}
fn trusted_program(program: &str, trusted: Option<&Path>) -> bool {
    if program == "fsl" {
        // Prepare constructs PATH with the trusted FSL directory first.
        return true;
    }
    trusted.is_some_and(|path| fs::canonicalize(program).is_ok_and(|actual| actual == path))
}
fn matches_invocation(
    command: &str,
    verb: &str,
    operand: Option<&str>,
    trusted: Option<&Path>,
) -> bool {
    matches_invocation_in(command, verb, operand, trusted, None)
}
fn matches_invocation_in(
    command: &str,
    verb: &str,
    operand: Option<&str>,
    trusted: Option<&Path>,
    workspace: Option<&Path>,
) -> bool {
    matches_invocation_at(
        command,
        verb,
        operand,
        trusted,
        workspace,
        workspace.unwrap_or(Path::new(".")),
    )
}
fn matches_invocation_at(
    command: &str,
    verb: &str,
    operand: Option<&str>,
    trusted: Option<&Path>,
    workspace: Option<&Path>,
    initial_cwd: &Path,
) -> bool {
    let Some(tokens) = command_evidence_tokens(command) else {
        return false;
    };
    let mut cwd = initial_cwd.to_owned();
    tokens
        .split(|t| *t == ShellToken::Separator)
        .any(|statement| {
            let mut words = Vec::new();
            let mut redirected = false;
            for token in statement {
                match token {
                    ShellToken::Redirect(_) => redirected = true,
                    ShellToken::Word(_) if redirected => redirected = false,
                    ShellToken::Word(word) => words.push(word.as_str()),
                    ShellToken::Separator => unreachable!("split removes separators"),
                }
            }
            if words.first() == Some(&"cd") && words.len() == 2 {
                cwd = resolve_path(&cwd, words[1]);
                return false;
            }
            if words.len() < 2 || !trusted_program(words[0], trusted) || words[1] != verb {
                return false;
            }
            // fsl check [KIND NAME] SOURCE; analyze/run REQUEST; monitor --input INPUT.
            // Bind the actual operand slot, never a later flag value or redirect.
            let args = &words[2..];
            let actual = if verb == "monitor" {
                args.windows(2)
                    .find(|pair| pair[0] == "--input")
                    .map(|pair| pair[1])
            } else if verb == "check" && args.first() == Some(&"eval") {
                args.get(2).copied()
            } else {
                args.first().copied()
            };
            operand.is_none_or(|expected| {
                actual.is_some_and(|actual| {
                    !actual.starts_with('-')
                        && if let Some(workspace) = workspace {
                            resolve_path(&cwd, actual) == resolve_path(workspace, expected)
                        } else {
                            Path::new(actual)
                                .components()
                                .filter(|p| !matches!(p, std::path::Component::CurDir))
                                .eq(Path::new(expected).components())
                        }
                })
            })
        })
}
/// Match literal public invocations against the configured trusted executable.
/// Comments, redirects, unrelated operands, substitutions and other binaries do not count.
pub fn invokes_fsl(command: &str, verb: &str, operand: Option<&str>) -> bool {
    let trusted = std::env::var_os("FSL").and_then(|p| fs::canonicalize(p).ok());
    matches_invocation(command, verb, operand, trusted.as_deref())
}

fn input_strings<'a>(value: &'a Value, strings: &mut Vec<&'a str>) {
    match value {
        Value::String(s) => strings.push(s),
        Value::Array(values) => values.iter().for_each(|v| input_strings(v, strings)),
        Value::Object(values) => values.values().for_each(|v| input_strings(v, strings)),
        _ => {}
    }
}
fn normalized(path: &Path) -> PathBuf {
    // Resolve existing components before '..', so a symlink escape cannot be
    // hidden by lexical simplification. Also resolves new output ancestors.
    let mut result = PathBuf::new();
    for part in path.components() {
        match part {
            std::path::Component::ParentDir => {
                result.pop();
            }
            std::path::Component::CurDir => {}
            other => result.push(other.as_os_str()),
        }
        if let Ok(actual) = fs::canonicalize(&result) {
            result = actual;
        }
    }
    result
}
fn resolve_path(cwd: &Path, word: &str) -> PathBuf {
    normalized(&cwd.join(word))
}
fn inspect_paths(
    text: &str,
    cwd: &Path,
    workspace: &Path,
    tools: &Tools,
    shipped_references: bool,
    writes: bool,
    violations: &mut Vec<String>,
) {
    let grammar = fs::canonicalize(workspace.join("docs/native-grammar.md")).ok();
    if text.contains("${HOME}") || text.contains("$HOME") {
        violations.push("sandbox-breach: home-directory access in tool input".to_owned());
    }
    let words = if (Path::new(text).is_absolute()
        || text.starts_with("../")
        || text.starts_with("./")
        || text.starts_with("~/"))
        && !text.contains('\n')
    {
        vec![text]
    } else {
        text.split(|c: char| c.is_whitespace() || "\"'`;(){}[],<>".contains(c))
            .collect()
    };
    for word in words {
        let word = word.trim_end_matches(':');
        // An assignment/option value may name a path, but '=' inside sed
        // replacement data is not a shell word boundary (e.g. =/profile).
        let word = word
            .split_once('=')
            .filter(|(name, _)| {
                !name.is_empty()
                    && (name.starts_with("--")
                        || name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'))
            })
            .map_or(word, |(_, value)| value);
        if word.starts_with('~') || word.starts_with("$HOME") || word.starts_with("${HOME}") {
            violations.push("sandbox-breach: home-directory access in tool input".to_owned());
            continue;
        }
        if word.is_empty() || !(word.contains('/') || word == ".." || cwd.join(word).exists()) {
            continue;
        }
        let path = resolve_path(cwd, word);
        let published_reference =
            path.starts_with(&tools.docs) || grammar.as_ref().is_some_and(|g| &path == g);
        if published_reference && !shipped_references {
            violations.push(
                "sandbox-breach: this scenario permits no shipped documentation or grammar access"
                    .to_owned(),
            );
            continue;
        }
        let in_workspace = path.starts_with(workspace);
        let read_only = published_reference
            || [&tools.fsl, &tools.quire, &tools.quoin].contains(&&path)
            || ["/usr", "/bin", "/sbin", "/etc", "/dev"]
                .iter()
                .any(|p| path.starts_with(normalized(Path::new(p))));
        if !(in_workspace || (!writes && read_only) || path == Path::new("/dev/null")) {
            violations
                .push("sandbox-breach: path outside allowed workspace/reference access".to_owned());
        }
    }
}
/// Indexes of the words of a `sed` command that are scripts. Every other word
/// is a file operand or option argument and is path-scanned as written.
fn sed_script_words(words: &[&str]) -> BTreeSet<usize> {
    let mut scripts = BTreeSet::new();
    let mut options = true;
    let mut expect_script = false;
    let mut expect_file = false;
    let mut have_script = false;
    for (index, &word) in words.iter().enumerate().skip(1) {
        if expect_script {
            scripts.insert(index);
            have_script = true;
            expect_script = false;
        } else if expect_file {
            expect_file = false;
        } else if options && word == "--" {
            options = false;
        } else if options && word.starts_with("--") {
            if word.starts_with("--expression=") {
                scripts.insert(index);
                have_script = true;
            } else if word == "--expression" {
                expect_script = true;
            } else if word == "--file" {
                expect_file = true;
                have_script = true;
            }
        } else if options && word.starts_with('-') && word.len() > 1 {
            if word.ends_with('e') {
                expect_script = true;
            } else if word.ends_with('f') {
                expect_file = true;
                have_script = true;
            }
        } else if !have_script {
            scripts.insert(index);
            have_script = true;
        }
    }
    scripts
}

/// The parts of a sed script that can name a file or run a command. Pattern and
/// replacement text of `s` commands is data and is dropped; file arguments of
/// `r`, `R`, `w`, `W` and the `s///w` flag are kept (flagged as writes where
/// they write). The boolean is true when the script can execute a command
/// (`e` command or `s///e` flag).
fn sed_script_fragments(script: &str) -> (Vec<(String, bool)>, bool) {
    let chars: Vec<char> = script.chars().collect();
    let mut fragments = Vec::new();
    let mut other = String::new();
    let mut executes = false;
    let mut i = 0;
    let mut command_start = true;
    while i < chars.len() {
        let c = chars[i];
        if !command_start {
            other.push(c);
            if matches!(c, ';' | '\n' | '{' | '}') {
                command_start = true;
            }
            i += 1;
            continue;
        }
        if c == '/' {
            // A regular-expression address: data up to the closing delimiter.
            i += 1;
            while i < chars.len() && chars[i] != '/' {
                if chars[i] == '\\' {
                    i += 1;
                }
                i += 1;
            }
            i += 1;
            continue;
        }
        if c.is_ascii_digit() || matches!(c, '$' | ',' | '~' | '+') {
            other.push(c);
            i += 1;
            continue;
        }
        if c.is_whitespace() || matches!(c, ';' | '{' | '}' | '!') {
            other.push(c);
            i += 1;
            continue;
        }
        match c {
            's' if i + 1 < chars.len() => {
                let delimiter = chars[i + 1];
                i += 2;
                for _ in 0..2 {
                    while i < chars.len() && chars[i] != delimiter {
                        if chars[i] == '\\' {
                            i += 1;
                        }
                        i += 1;
                    }
                    i += 1;
                }
                while i < chars.len() && !matches!(chars[i], ';' | '\n' | '}') {
                    match chars[i] {
                        'e' => executes = true,
                        'w' => {
                            let file: String =
                                chars[i + 1..].iter().take_while(|c| **c != '\n').collect();
                            fragments.push((format!("w {}", file.trim_start()), true));
                            i = chars.len();
                            continue;
                        }
                        _ => {}
                    }
                    i += 1;
                }
                command_start = false;
            }
            'r' | 'R' | 'w' | 'W' => {
                let file: String = chars[i + 1..].iter().take_while(|c| **c != '\n').collect();
                i += 1 + file.chars().count();
                fragments.push((format!("{c} {}", file.trim_start()), matches!(c, 'w' | 'W')));
            }
            'e' => {
                executes = true;
                i += 1;
            }
            _ => {
                other.push(c);
                i += 1;
                command_start = false;
            }
        }
    }
    if !other.trim().is_empty() {
        fragments.push((other, false));
    }
    (fragments, executes)
}

fn rg_path_operands<'a>(words: &[&'a str]) -> Option<Vec<Option<&'a str>>> {
    #[derive(Clone, Copy)]
    enum Argument {
        Pattern,
        PatternFile,
        Path,
        Data,
    }
    let mut paths: Vec<Option<&str>> = words.iter().map(|word| Some(*word)).collect();
    let mut next = None;
    let mut options = true;
    let mut files = false;
    let mut supplied_pattern = false;
    let mut implicit_pattern = None;
    for (index, &word) in words.iter().enumerate().skip(1) {
        if let Some(role) = next.take() {
            match role {
                Argument::Pattern | Argument::Data => paths[index] = None,
                Argument::PatternFile | Argument::Path => {}
            }
            continue;
        }
        if options && word == "--" {
            options = false;
            paths[index] = None;
            continue;
        }
        if options && word == "--files" {
            files = true;
            paths[index] = None;
            if let Some(pattern) = implicit_pattern.take() {
                paths[pattern] = Some(words[pattern]);
            }
            continue;
        }
        if options {
            match word {
                "-n"
                | "-L"
                | "-i"
                | "-S"
                | "-s"
                | "-l"
                | "-c"
                | "-H"
                | "-w"
                | "-F"
                | "-v"
                | "-u"
                | "-uu"
                | "--no-heading"
                | "--hidden"
                | "--follow"
                | "--ignore-case"
                | "--smart-case"
                | "--line-number"
                | "--files-with-matches"
                | "--fixed-strings"
                | "--no-ignore"
                | "--color=never" => {
                    paths[index] = None;
                    continue;
                }
                "-e" | "--regexp" => {
                    next = Some(Argument::Pattern);
                    supplied_pattern = true;
                    paths[index] = None;
                    if let Some(pattern) = implicit_pattern.take() {
                        paths[pattern] = Some(words[pattern]);
                    }
                    continue;
                }
                "-f" | "--file" => {
                    next = Some(Argument::PatternFile);
                    supplied_pattern = true;
                    paths[index] = None;
                    if let Some(pattern) = implicit_pattern.take() {
                        paths[pattern] = Some(words[pattern]);
                    }
                    continue;
                }
                "--pre" | "--ignore-file" => {
                    next = Some(Argument::Path);
                    paths[index] = None;
                    continue;
                }
                "-g" | "--glob" | "--iglob" | "-A" | "-B" | "-C" | "--after-context"
                | "--before-context" | "--context" => {
                    next = Some(Argument::Data);
                    paths[index] = None;
                    continue;
                }
                _ => {}
            }
            if let Some(path) = word
                .strip_prefix("--file=")
                .or_else(|| word.strip_prefix("-f").filter(|s| !s.is_empty()))
            {
                supplied_pattern = true;
                paths[index] = Some(path);
                if let Some(pattern) = implicit_pattern.take() {
                    paths[pattern] = Some(words[pattern]);
                }
                continue;
            }
            if word.starts_with("--regexp=")
                || (word.starts_with("-e") && !word.starts_with("--") && word.len() > 2)
            {
                supplied_pattern = true;
                paths[index] = None;
                if let Some(pattern) = implicit_pattern.take() {
                    paths[pattern] = Some(words[pattern]);
                }
                continue;
            }
            if let Some(path) = word
                .strip_prefix("--pre=")
                .or_else(|| word.strip_prefix("--ignore-file="))
            {
                paths[index] = Some(path);
                continue;
            }
            if word.starts_with('-') {
                // Unknown or combined options can change which word is a file.
                // Refuse to infer pattern-data roles without a known arity.
                return None;
            }
        }
        if !files && !supplied_pattern {
            supplied_pattern = true;
            implicit_pattern = Some(index);
            paths[index] = None;
        }
    }
    if next.is_some() { None } else { Some(paths) }
}

fn inspect_command(
    command: &str,
    cwd: &mut PathBuf,
    workspace: &Path,
    tools: &Tools,
    shipped_references: bool,
    violations: &mut Vec<String>,
) {
    if changes_tool_resolution(command) {
        violations.push("harness-error: agent changed trusted tool resolution".to_owned());
    }
    let Some(tokens) = shell_tokens(command) else {
        // Opaque code cannot establish command evidence. Still inspect literal
        // path accesses in code/heredocs, without pretending to execute a shell.
        inspect_paths(
            command,
            cwd,
            workspace,
            tools,
            shipped_references,
            false,
            violations,
        );
        if command.split_whitespace().any(|w| w == "cd") {
            violations
                .push("harness-error: cannot resolve shell working-directory change".to_owned());
        }
        return;
    };
    for statement in tokens.split(|t| *t == ShellToken::Separator) {
        let words = statement
            .iter()
            .filter_map(|t| {
                if let ShellToken::Word(w) = t {
                    Some(w.as_str())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if words.first() == Some(&"cd") {
            if words.len() != 2 || words[1] == "-" {
                violations.push(
                    "harness-error: cannot resolve shell working-directory change".to_owned(),
                );
            } else {
                inspect_paths(
                    words[1],
                    cwd,
                    workspace,
                    tools,
                    shipped_references,
                    false,
                    violations,
                );
                *cwd = resolve_path(cwd, words[1]);
            }
            continue;
        }
        let mut redirect = false;
        let program = words
            .first()
            .and_then(|w| Path::new(w).file_name())
            .and_then(|p| p.to_str());
        let rg_paths = if program == Some("rg") {
            match rg_path_operands(&words) {
                Some(paths) => Some(paths),
                None => {
                    violations.push(
                        "harness-error: unsupported rg option arity in boundary evidence"
                            .to_owned(),
                    );
                    continue;
                }
            }
        } else {
            None
        };
        let sed_scripts = if program == Some("sed") {
            sed_script_words(&words)
        } else {
            BTreeSet::new()
        };
        let mut word_index = 0;
        for (index, token) in statement.iter().enumerate() {
            match token {
                ShellToken::Redirect(writes) => redirect = *writes,
                ShellToken::Word(word) => {
                    let path_word = if redirect {
                        Some(word.as_str())
                    } else if let Some(paths) = &rg_paths {
                        paths[word_index]
                    } else {
                        Some(word.as_str())
                    };
                    let this_word = word_index;
                    word_index += 1;
                    if program == Some("sed") && !redirect && sed_scripts.contains(&this_word) {
                        let (fragments, executes) = sed_script_fragments(word);
                        if executes {
                            violations
                                .push("sandbox-breach: sed script executes a command".to_owned());
                        }
                        for (fragment, writes) in fragments {
                            inspect_paths(
                                &fragment,
                                cwd,
                                workspace,
                                tools,
                                shipped_references,
                                writes,
                                violations,
                            );
                        }
                        continue;
                    }
                    let destination = matches!(program, Some("cp" | "mv" | "ln"))
                        && statement[index + 1..]
                            .iter()
                            .all(|t| !matches!(t, ShellToken::Word(_)));
                    let writes = redirect
                        || destination
                        || (index > 0 && matches!(program, Some("rm" | "mkdir" | "touch" | "tee")));
                    if let Some(path_word) = path_word {
                        inspect_paths(
                            path_word,
                            cwd,
                            workspace,
                            tools,
                            shipped_references,
                            writes,
                            violations,
                        );
                    }
                    redirect = false;
                }
                ShellToken::Separator => {}
            }
        }
    }
}
// Only duplicate shell-source literals with an actual typed execution record
// are removed from the generic path scan. The real command is still checked
// at its recorded cwd; opaque JavaScript never supplies command evidence.
fn corroborated_cmd_data(code: &str, executed: &[String]) -> Option<String> {
    if code.len() > 1_048_576 {
        return None;
    }
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_javascript::LANGUAGE.into())
        .ok()?;
    let tree = parser.parse(code, None)?;
    let root = tree.root_node();
    if root.has_error() {
        return None;
    }
    let mut pending = vec![root];
    let mut ranges = Vec::new();
    while let Some(node) = pending.pop() {
        let mut cursor = node.walk();
        pending.extend(node.named_children(&mut cursor));
        if node.kind() != "call_expression" {
            continue;
        }
        let Some(function) = node.child_by_field_name("function") else {
            continue;
        };
        if function.kind() != "member_expression" {
            continue;
        }
        let Some(object) = function.child_by_field_name("object") else {
            continue;
        };
        let Some(property) = function.child_by_field_name("property") else {
            continue;
        };
        if object.kind() != "identifier"
            || object.utf8_text(code.as_bytes()).ok()? != "tools"
            || property.kind() != "property_identifier"
            || property.utf8_text(code.as_bytes()).ok()? != "exec_command"
        {
            continue;
        }
        let Some(arguments) = node.child_by_field_name("arguments") else {
            continue;
        };
        let mut cursor = arguments.walk();
        let mut values = arguments
            .named_children(&mut cursor)
            .filter(|v| v.kind() != "comment");
        let Some(argument) = values.next() else {
            continue;
        };
        if argument.kind() != "object" || values.next().is_some() {
            continue;
        }
        let mut cursor = argument.walk();
        for pair in argument
            .named_children(&mut cursor)
            .filter(|pair| pair.kind() == "pair")
        {
            let Some(key) = pair.child_by_field_name("key") else {
                continue;
            };
            let key_text = key.utf8_text(code.as_bytes()).ok()?;
            let cmd_key = (key.kind() == "property_identifier" && key_text == "cmd")
                || (key.kind() == "string"
                    && serde_json::from_str::<String>(key_text).is_ok_and(|key| key == "cmd"));
            if !cmd_key {
                continue;
            }
            let Some(value) = pair.child_by_field_name("value") else {
                continue;
            };
            if value.kind() != "string" {
                continue;
            }
            let Ok(command) =
                serde_json::from_str::<String>(value.utf8_text(code.as_bytes()).ok()?)
            else {
                continue;
            };
            if executed.contains(&command) {
                ranges.push(value.byte_range());
            }
        }
    }
    if ranges.is_empty() {
        return None;
    }
    ranges.sort_unstable_by_key(|range| range.start);
    ranges.dedup();
    let mut masked = String::new();
    let mut previous = 0;
    for range in ranges {
        masked.push_str(code.get(previous..range.start)?);
        masked.push_str("\"\"");
        previous = range.end;
    }
    masked.push_str(code.get(previous..)?);
    Some(masked)
}

fn boundary_violations(
    workspace: &Path,
    tools: &Tools,
    inputs: &[RecordedInput],
    shipped_references: bool,
) -> Vec<String> {
    let mut violations = Vec::new();
    let mut typed_commands = Vec::new();
    for input in inputs.iter().filter(|input| input.typed_execution) {
        collect_commands(&input.value, &mut typed_commands);
    }
    let mut persistent_cwd = workspace.to_owned();
    for input in inputs {
        let mut cwd = if input.persistent_cwd {
            persistent_cwd.clone()
        } else {
            workspace.to_owned()
        };
        if let Some(workdir) = input.value.get("workdir").and_then(Value::as_str) {
            inspect_paths(
                workdir,
                workspace,
                workspace,
                tools,
                shipped_references,
                false,
                &mut violations,
            );
            cwd = resolve_path(workspace, workdir);
        }
        let mut commands = Vec::new();
        collect_commands(&input.value, &mut commands);
        let mut strings = Vec::new();
        input_strings(&input.value, &mut strings);
        for text in strings {
            let lower = text.to_ascii_lowercase();
            if [
                "git clone",
                "git fetch",
                "git pull",
                "github.com/agent-ix/",
                "raw.githubusercontent.com/agent-ix/",
                "api.github.com/repos/agent-ix/",
                "github.com:agent-ix/",
                "gh repo clone agent-ix/",
                "gh api repos/agent-ix/",
            ]
            .iter()
            .any(|p| lower.contains(p))
            {
                violations.push(
                    "sandbox-breach: repository fetch/source access in tool input".to_owned(),
                );
            }
            if !commands.iter().any(|c| c == text) {
                let cmd_data = input
                    .value
                    .get("code")
                    .and_then(Value::as_str)
                    .filter(|code| *code == text)
                    .and_then(|code| corroborated_cmd_data(code, &typed_commands));
                inspect_paths(
                    cmd_data.as_deref().unwrap_or(text),
                    &cwd,
                    workspace,
                    tools,
                    shipped_references,
                    input.writes_files,
                    &mut violations,
                );
            }
        }
        for command in commands {
            inspect_command(
                &command,
                &mut cwd,
                workspace,
                tools,
                shipped_references,
                &mut violations,
            );
        }
        if input.persistent_cwd {
            persistent_cwd = cwd;
        }
    }
    violations.sort();
    violations.dedup();
    violations
}

fn enforce_transcript_boundary(
    workspace: &Path,
    tools: &Tools,
    shipped_references: bool,
    assertion: &mut Assertion,
) -> ScenarioResult<()> {
    let inputs = agent_inputs(workspace)?;
    let violations = boundary_violations(workspace, tools, &inputs, shipped_references);
    assertion.checks.insert(
        "transcript_boundary".to_owned(),
        json!({"ok":violations.is_empty(),"violations":violations}),
    );
    if !violations.is_empty() {
        assertion.ok = false;
        assertion.failures.extend(violations);
    }
    Ok(())
}

fn recorded_command_evidence(
    workspace: &Path,
    tools: &Tools,
    inputs: &[RecordedInput],
    verb: &str,
    operand: &str,
    shipped_references: bool,
) -> bool {
    let mut persistent_cwd = workspace.to_owned();
    let mut found = false;
    for input in inputs {
        let mut cwd = if input.persistent_cwd {
            persistent_cwd.clone()
        } else {
            workspace.to_owned()
        };
        if let Some(workdir) = input.value.get("workdir").and_then(Value::as_str) {
            cwd = resolve_path(workspace, workdir);
        }
        let mut commands = Vec::new();
        collect_commands(&input.value, &mut commands);
        for command in commands {
            found |= matches_invocation_at(
                &command,
                verb,
                Some(operand),
                Some(&tools.fsl),
                Some(workspace),
                &cwd,
            );
            // Reuse the boundary scan's exact shell/cd handling. Boundary
            // violations were already applied; here its cwd is the evidence
            // context for subsequent calls, never a fresh workspace default.
            inspect_command(
                &command,
                &mut cwd,
                workspace,
                tools,
                shipped_references,
                &mut Vec::new(),
            );
        }
        if input.persistent_cwd {
            persistent_cwd = cwd;
        }
    }
    found
}

// Claude's native per-home project trust setting. The caller must prove this is
// the isolated run home, and only the fresh SDK workspace is trusted.
fn trust_run_workspace(home: &Path, workspace: &Path) -> ScenarioResult<()> {
    let root = home.parent().ok_or("private run home has no root")?;
    let allowed = fs::canonicalize(root.join("work"))?;
    let workspace = fs::canonicalize(workspace)?;
    if !workspace.starts_with(&allowed) || workspace == allowed {
        return Err("refusing project trust outside the fresh run workspace".into());
    }
    let config = home.join(".claude.json");
    let mut value: Value = serde_json::from_slice(&fs::read(&config)?)?;
    let object = value
        .as_object_mut()
        .ok_or("private Claude config is not an object")?;
    let projects = object
        .entry("projects")
        .or_insert_with(|| json!({}))
        .as_object_mut()
        .ok_or("private Claude projects is not an object")?;
    projects.insert(
        workspace
            .to_str()
            .ok_or("nonUTF8 Claude workspace")?
            .to_owned(),
        json!({"hasTrustDialogAccepted":true}),
    );
    fs::write(&config, serde_json::to_vec(&value)?)?;
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(config, fs::Permissions::from_mode(0o600))?;
    Ok(())
}
fn prepare_private_home(workspace: &Path) -> ScenarioResult<()> {
    if std::env::var("FSL_EVAL_AGENT").ok().as_deref() != Some("claude") {
        return Ok(());
    }
    let home =
        fs::canonicalize(std::env::var_os("FSL_EVAL_HOME").ok_or("private eval home missing")?)?;
    if home != fs::canonicalize(std::env::var_os("HOME").ok_or("HOME missing")?)? {
        return Err("refusing to configure a different or global Claude home".into());
    }
    trust_run_workspace(&home, workspace)
}

fn references(workspace: &Path, tools: &Tools) -> ScenarioResult<()> {
    let docs = workspace.join("docs");
    fs::create_dir_all(&docs)?;
    let shipped = docs.join("shipped");
    if !shipped.exists() {
        std::os::unix::fs::symlink(&tools.docs, &shipped)?;
    }
    let grammar = fs::canonicalize(
        std::env::var_os("FSL_NATIVE_GRAMMAR")
            .ok_or("set FSL_NATIVE_GRAMMAR to the published native grammar")?,
    )?;
    let grammar_link = docs.join("native-grammar.md");
    if !grammar_link.exists() {
        std::os::unix::fs::symlink(grammar, &grammar_link)?;
    }
    Ok(())
}

fn apply_evidence(
    workspace: &Path,
    tools: &Tools,
    entry: &Entry,
    assertion: &mut Assertion,
) -> ScenarioResult<()> {
    enforce_transcript_boundary(workspace, tools, entry.shipped_references, assertion)?;
    match agent_inputs(workspace) {
        Ok(inputs) => {
            let mut commands = Vec::new();
            for input in &inputs {
                collect_commands(&input.value, &mut commands);
            }
            assertion
                .checks
                .insert("agent_command_count".to_owned(), json!(commands.len()));
            let mut evidence = serde_json::Map::new();
            for &(verb, operand) in entry.commands {
                let check = recorded_command_evidence(
                    workspace,
                    tools,
                    &inputs,
                    verb,
                    operand,
                    entry.shipped_references,
                );
                evidence.insert(format!("{verb}:{operand}"), json!(check));
                if !check {
                    assertion.ok = false;
                    assertion.failures.push(format!(
                        "no recorded direct public fsl {verb} invocation for {operand}"
                    ));
                }
            }
            assertion
                .checks
                .insert("command_evidence".to_owned(), json!(evidence));
        }
        Err(e) => {
            assertion.ok = false;
            assertion.failures.push(e.to_string());
        }
    }
    Ok(())
}
/// Regrade only retained transcript integrity and command evidence, without
/// rerunning model checking, semantic probes, or a live agent.
pub fn retained_evidence(
    workspace: &Path,
    tools: &Tools,
    scenario: &str,
) -> ScenarioResult<Assertion> {
    let entry = ENTRIES
        .iter()
        .find(|entry| entry.id == scenario)
        .ok_or("undescribed scenario")?;
    let workspace = fs::canonicalize(workspace)?;
    let mut assertion = Assertion {
        ok: true,
        checks: Default::default(),
        failures: Vec::new(),
    };
    apply_evidence(&workspace, tools, entry, &mut assertion)?;
    Ok(assertion)
}

/// Answer one bounded request from cli-evals; scenario execution and metrics stay in the SDK.
pub fn respond(input: &[u8]) -> ScenarioResult<Value> {
    if input.len() > 1_048_576 {
        return Err("provider request exceeds SDK input limit".into());
    }
    let request: Request = serde_json::from_slice(input)?;
    if request.protocol != "cli-agent-evals.external-provider-request/v1" {
        return Err("unknown provider protocol".into());
    }
    let protocol = "cli-agent-evals.external-provider-result/v1";
    if matches!(request.operation, Operation::Describe) {
        let scenarios:Vec<Value>=ENTRIES.iter().map(|entry|json!({"id":entry.id,"title":entry.title,"use_case":entry.use_case,"canary":false})).collect();
        return Ok(json!({"protocol":protocol,"operation":"describe","scenarios":scenarios}));
    }
    let entry = ENTRIES
        .iter()
        .find(|entry| entry.id == request.context.scenario.id)
        .ok_or("undescribed scenario")?;
    let tools = Tools::from_environment()?;
    let workspace = fs::canonicalize(request.context.work_dir)?;
    match request.operation {
        Operation::Describe => Err("unreachable describe dispatch".into()),
        Operation::Prepare => {
            if entry.shipped_references {
                references(&workspace, &tools)?;
            }
            prepare_private_home(&workspace)?;
            let prompt = (entry.prepare)(&workspace, &tools)?;
            let mut paths = Vec::new();
            for program in [&tools.fsl, &tools.quire, &tools.quoin] {
                if let Some(parent) = program.parent() {
                    paths.push(parent.to_owned());
                }
            }
            for program in ["node", "codex", "claude"] {
                if let Ok(path) = executable(program)
                    && let Some(parent) = path.parent()
                {
                    paths.push(parent.to_owned());
                }
            }
            paths.extend([PathBuf::from("/usr/bin"), PathBuf::from("/bin")]);
            paths.dedup();
            let path = std::env::join_paths(paths)?;
            Ok(
                json!({"protocol":protocol,"operation":"prepare","prompt":prompt,"environment":{"PATH":path.to_str().ok_or("nonUTF8 tool PATH")?}}),
            )
        }
        Operation::Assert => {
            let mut assertion = match (entry.grade)(&workspace, &tools) {
                Ok(result) => result,
                Err(e) => Assertion {
                    ok: false,
                    checks: Default::default(),
                    failures: vec![format!("harness-error: grader failed: {e}")],
                },
            };
            apply_evidence(&workspace, &tools, entry, &mut assertion)?;
            Ok(json!({"protocol":protocol,"operation":"assert","assertion":assertion}))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Original two-wrapper/typed-event fixture; never executes JavaScript, shell, or FSL.
    #[test]
    fn runtime_safety_opaque_cmd_literals_use_typed_execution_roles() -> ScenarioResult<()> {
        let (_temp, workspace, tools) = reference_policy_fixture()?;
        let directory = workspace.join(".cli-agent-evals/transcripts");
        fs::create_dir_all(&directory)?;
        let sed = "sed -n '10,47p' docs/shipped/fsl/cli/at-a-glance.md; fsl --help";
        let rg = r#"rg --files -L docs/shipped | rg '/fsl\\.(md|mdx)$|/fsl/' | tail -8"#;
        let wrappers = format!(
            "text(await tools.exec_command({{cmd:{},\"max_output_tokens\":3000}}));\ntext(await tools.exec_command({{cmd:{},\"max_output_tokens\":1000}}));\n",
            serde_json::to_string(sed)?,
            serde_json::to_string(rg)?
        );
        let cwd = url::Url::from_directory_path(&workspace)
            .map_err(|_| "workspace URL")?
            .to_string();
        let typed = |command: &str| {
            json!({"type":"event_msg","payload":{"type":"item_completed","item":{
                "type":"CommandExecution","command":["/bin/bash","-lc",command],"cwd":cwd,"status":"completed","exit_code":0
            }}})
        };
        let analyze = "fsl analyze holding-request.json --format json > holding.json\nfsl analyze violating-request.json --format json > violating.json";
        for (extra_code, extra_typed, expected) in [
            ("", None, true),
            (
                "text(await tools.view_image({path:\"/outside/evaluator.png\"}));",
                None,
                false,
            ),
            (
                "text(await tools.exec_command({cmd:\"cat /outside/source.fsl\"}));",
                None,
                false,
            ),
            (
                "text(await tools.exec_command({cmd:\"cat /outside/source.fsl\"}));",
                Some("cat /outside/source.fsl"),
                false,
            ),
        ] {
            let mut rows = vec![
                json!({"type":"response_item","payload":{"type":"custom_tool_call","name":"exec","input":format!("{wrappers}{extra_code}")}}),
                typed(sed),
                typed(rg),
                typed(analyze),
            ];
            if let Some(command) = extra_typed {
                rows.push(typed(command));
            }
            fs::write(
                directory.join("two-wrapper-unit.transcript"),
                rows.iter()
                    .map(Value::to_string)
                    .collect::<Vec<_>>()
                    .join("\n"),
            )?;
            let result = retained_evidence(&workspace, &tools, safety_analyze::ID)?;
            assert_eq!(
                result.checks["command_evidence"]["analyze:holding-request.json"],
                true
            );
            assert_eq!(
                result.checks["command_evidence"]["analyze:violating-request.json"],
                true
            );
            assert_eq!(result.ok, expected, "{extra_code}: {:?}", result.failures);
            assert_eq!(result.checks["transcript_boundary"]["ok"], expected);
        }
        Ok(())
    }

    /// Trace: FR-001 AC-6
    #[test]
    fn runtime_plain_rg_flags_and_sed_replacement_slashes_are_not_breaches() -> ScenarioResult<()> {
        let (_temp, workspace, tools) = reference_policy_fixture()?;
        for command in [
            "rg -n -i 'grammar|native' docs",
            "sed -i 's/  field /  /' model.fsl",
            "sed -i ':a;N;$!ba;s/x\\n/y/' model.fsl",
            "sed -n '1,3p' model.fsl",
            "sed -e 's/a/b/' -e 's/c/d/g' model.fsl",
        ] {
            let violations = boundary_violations(
                &workspace,
                &tools,
                &[recorded(json!({"cmd":command}))],
                true,
            );
            assert!(violations.is_empty(), "{command}: {violations:?}");
        }
        Ok(())
    }

    /// Trace: FR-001 AC-6
    #[test]
    fn runtime_sed_file_commands_flags_and_path_looking_operands_stay_breaches()
    -> ScenarioResult<()> {
        let (temp, workspace, tools) = reference_policy_fixture()?;
        let reference = temp.path().join("evaluator/reference.fsl");
        let reference = reference.display();
        let outside = temp.path().join("evaluator/out.fsl");
        let outside = outside.display();
        fs::create_dir_all(workspace.join("s"))?;
        for command in [
            format!("sed -i 's/a/a/;r {reference}' model.fsl"),
            format!("sed -i 'R {reference}' model.fsl"),
            format!("sed -i 's/a/a/;r{reference}' model.fsl"),
            format!("sed -n 'w {outside}' model.fsl"),
            format!("sed -n 'W {outside}' model.fsl"),
            format!("sed 's/a/b/w {outside}' model.fsl"),
            format!("sed -e 's/a/b/' -e 'r {reference}' model.fsl"),
            format!("sed --expression='r {reference}' model.fsl"),
            format!("sed -f {reference} model.fsl"),
            "sed -n p s/../../x".to_owned(),
            "sed -n p s/../../../etc/passwd".to_owned(),
        ] {
            let violations = boundary_violations(
                &workspace,
                &tools,
                &[recorded(json!({"cmd":command}))],
                true,
            );
            assert!(
                violations.iter().any(|v| v.starts_with("sandbox-breach:")),
                "{command}: {violations:?}"
            );
        }
        for command in [
            "sed 's/a/b/e' model.fsl",
            "sed 'e echo x' model.fsl",
            "sed -i '1e id' model.fsl",
        ] {
            let violations = boundary_violations(
                &workspace,
                &tools,
                &[recorded(json!({"cmd":command}))],
                true,
            );
            assert!(
                violations.iter().any(|v| v.contains("executes a command")),
                "{command}: {violations:?}"
            );
        }
        Ok(())
    }

    #[test]
    fn runtime_safety_combined_rg_file_option_fails_closed() -> ScenarioResult<()> {
        let (_temp, workspace, tools) = reference_policy_fixture()?;
        for command in [
            "rg -nf /outside/patterns docs",
            "rg --unknown-option /outside/patterns docs",
        ] {
            let violations = boundary_violations(
                &workspace,
                &tools,
                &[recorded(json!({"cmd":command}))],
                true,
            );
            assert!(
                violations.iter().any(|v| v.starts_with("harness-error:")),
                "unresolved rg arity must not become pattern data: {command}"
            );
        }
        for command in [
            r#"rg --files -L docs/shipped | rg '/fsl\.(md|mdx)$|/fsl/' | tail -8"#,
            "rg -n '/outside/pattern' docs",
        ] {
            let violations = boundary_violations(
                &workspace,
                &tools,
                &[recorded(json!({"cmd":command}))],
                true,
            );
            assert!(
                violations.is_empty(),
                "known pattern roles remain usable: {command}: {violations:?}"
            );
        }
        Ok(())
    }

    /// ORIGINAL retained-input reproduction, parsed only; no shell/FSL/model is executed.
    #[test]
    fn runtime_safety_actual_script_records_redirected_analyzers() -> ScenarioResult<()> {
        let (_temp, workspace, tools) = reference_policy_fixture()?;
        let transcript = workspace.join(".cli-agent-evals/transcripts");
        fs::create_dir_all(&transcript)?;
        let script = r#"cat >> counter.fsl <<'EOF'
temporal WithinLimit using inf over (c: Meter::Counter) clock "model-steps" on origin {
  always holds(c.level <= 3)
}

temporal TooSmall using inf over (c: Meter::Counter) clock "model-steps" on origin {
  always holds(c.level <= 2)
}
EOF
fsl analyze holding-request.json --format json > holding.json
holding_exit=$?
fsl analyze violating-request.json --format json > violating.json
violating_exit=$?
printf 'holding_exit=%s\nviolating_exit=%s\n' "$holding_exit" "$violating_exit"
cat holding.json violating.json
"#;
        let cwd = url::Url::from_directory_path(&workspace)
            .map_err(|_| "workspace URL")?
            .to_string();
        let records = [script, r#"rg --files -L docs/shipped | rg '/fsl\.(md|mdx)$|/fsl/' | tail -8"#].iter().map(|command| {
            json!({"type":"event_msg","payload":{"type":"item_completed","item":{
                "type":"CommandExecution","command":["/bin/bash","-lc",command],"cwd":cwd,"status":"completed","exit_code":0
            }}}).to_string()
        }).collect::<Vec<_>>();
        fs::write(
            transcript.join("actual-shape-unit.transcript"),
            records.join("\n"),
        )?;
        let assertion = retained_evidence(&workspace, &tools, safety_analyze::ID)?;
        assert!(
            assertion.ok,
            "{:?}; tokens {:?}",
            assertion.failures,
            command_evidence_tokens(script)
        );
        assert_eq!(
            assertion.checks["command_evidence"]["analyze:holding-request.json"],
            true
        );
        assert_eq!(
            assertion.checks["command_evidence"]["analyze:violating-request.json"],
            true
        );
        assert_eq!(assertion.checks["transcript_boundary"]["ok"], true);
        Ok(())
    }

    #[test]
    fn runtime_safety_redirection_targets_and_unreachable_text_are_not_inputs() {
        for script in [
            "echo fake > holding-request.json",
            "fsl analyze other.json > holding-request.json",
            "cat <<'EOF'\nfsl analyze holding-request.json > holding.json\nEOF\n",
            "exit 0\nfsl analyze holding-request.json > holding.json",
            "false && fsl analyze holding-request.json > holding.json",
            "fsl() { echo fake; }\nfsl analyze holding-request.json > holding.json",
            "PATH=$?\nfsl analyze holding-request.json > holding.json",
            "cat < holding-request.json",
        ] {
            assert!(
                !matches_invocation(script, "analyze", Some("holding-request.json"), None),
                "false evidence: {script}"
            );
        }
        let prefix = "fsl analyze holding-request.json > holding.json\nexit 0\nfsl analyze violating-request.json > violating.json";
        assert!(matches_invocation(
            prefix,
            "analyze",
            Some("holding-request.json"),
            None
        ));
        assert!(!matches_invocation(
            prefix,
            "analyze",
            Some("violating-request.json"),
            None
        ));
    }

    #[test]
    fn runtime_safety_rg_patterns_are_data_but_sources_and_evaluators_are_guarded()
    -> ScenarioResult<()> {
        let (_temp, workspace, tools) = reference_policy_fixture()?;
        for command in [
            r#"rg --files -L docs/shipped | rg '/fsl\.(md|mdx)$|/fsl/' | tail -8"#,
            "rg '/outside/pattern' docs",
            "rg -e '/outside/pattern' docs",
            "rg --regexp='/outside/pattern' docs",
            "rg -n -A 2 '/outside/pattern' docs",
        ] {
            let violations = boundary_violations(
                &workspace,
                &tools,
                &[recorded(json!({"cmd":command}))],
                true,
            );
            assert!(violations.is_empty(), "{command}: {violations:?}");
        }
        for command in [
            "rg --files /outside/tree",
            "rg --files '/outside/tree'",
            "rg -f /outside/patterns docs",
            "rg --file=/outside/patterns docs",
            "rg -f/outside/patterns docs",
            "rg --pre /outside/evaluator needle docs",
            "rg --pre=/outside/evaluator needle docs",
            "rg '/outside/pattern' /outside/source",
            "rg /outside/source -e needle",
            "rg '/outside/pattern' > /outside/output",
            "cat '/outside/source'",
            "fsl analyze /outside/request.json",
        ] {
            let violations = boundary_violations(
                &workspace,
                &tools,
                &[recorded(json!({"cmd":command}))],
                true,
            );
            assert!(!violations.is_empty(), "unguarded real path: {command}");
        }
        Ok(())
    }
    fn recorded(value: Value) -> RecordedInput {
        RecordedInput {
            value,
            persistent_cwd: false,
            writes_files: false,
            typed_execution: false,
        }
    }
    /// Trace: FR-001 AC-6
    #[test]
    fn runtime_sed_replacement_is_data_while_absolute_source_operands_are_paths()
    -> ScenarioResult<()> {
        let temp = tempfile::tempdir()?;
        let workspace = temp.path().join("work");
        fs::create_dir_all(&workspace)?;
        fs::write(workspace.join("model.fsl"), "owned")?;
        let tools = Tools {
            fsl: temp.path().join("fsl"),
            docs: temp.path().join("published"),
            quire: temp.path().join("quire"),
            quoin: temp.path().join("quoin"),
        };
        let sed = "sed -i 's/profile model =/profile m =/; s/using model on/using m on/' model.fsl";
        let values = [
            recorded(json!({"cmd":sed})),
            recorded(
                json!({"code":format!("text(await tools.exec_command({{cmd:{}}}));",serde_json::to_string(sed)?)}),
            ),
        ];
        assert!(boundary_violations(&workspace, &tools, &values, true).is_empty());
        for command in [
            format!(
                "cat '{}'",
                temp.path().join("evaluator/reference.fsl").display()
            ),
            format!(
                "SOURCE={} cat model.fsl",
                temp.path().join("evaluator/reference.fsl").display()
            ),
        ] {
            assert!(
                boundary_violations(
                    &workspace,
                    &tools,
                    &[recorded(json!({"cmd":command}))],
                    true
                )
                .iter()
                .any(|v| v.starts_with("sandbox-breach:"))
            );
        }
        Ok(())
    }
    /// Trace: FR-001 AC-6
    #[test]
    fn runtime_codex_execution_events_supply_context_but_code_and_output_do_not()
    -> ScenarioResult<()> {
        let temp = tempfile::tempdir()?;
        let workspace = temp.path().join("work with space");
        fs::create_dir_all(workspace.join("docs"))?;
        fs::write(workspace.join("model.fsl"), "owned")?;
        let transcript = workspace.join(".cli-agent-evals/transcripts");
        fs::create_dir_all(&transcript)?;
        let tools = Tools {
            fsl: temp.path().join("fsl"),
            docs: temp.path().join("published"),
            quire: temp.path().join("quire"),
            quoin: temp.path().join("quoin"),
        };
        let cwd = url::Url::from_directory_path(workspace.join("docs"))
            .map_err(|_| "test URL")?
            .to_string();
        for (command, expected) in [
            ("fsl check eval own ../model.fsl --format json", true),
            ("fsl check eval other other.fsl # model.fsl", false),
        ] {
            let records = [
                json!({"type":"response_item","payload":{"type":"custom_tool_call","name":"exec","input":"text(await tools.exec_command({cmd:\"fsl check eval own model.fsl\"}));"}}),
                json!({"type":"event_msg","payload":{"type":"item_completed","item":{"type":"CommandExecution","command":["/bin/bash","-lc",command],"cwd":cwd,"aggregated_output":"fsl check eval own model.fsl"}}}),
            ];
            fs::write(
                transcript.join("sample.transcript"),
                records
                    .iter()
                    .map(Value::to_string)
                    .collect::<Vec<_>>()
                    .join("\n"),
            )?;
            let inputs = agent_inputs(&workspace)?;
            assert_eq!(agent_commands(&workspace)?.len(), 1);
            assert_eq!(
                recorded_command_evidence(&workspace, &tools, &inputs, "check", "model.fsl", true),
                expected
            );
            assert!(boundary_violations(&workspace, &tools, &inputs, true).is_empty());
        }
        fs::write(transcript.join("sample.transcript"),json!({"type":"event_msg","payload":{"type":"item_completed","item":{"type":"CommandExecution","command":["echo","hello"],"cwd":"https://example.test/work"}}}).to_string())?;
        assert!(agent_inputs(&workspace).is_err());
        Ok(())
    }
    /// Trace: FR-001 AC-6
    #[test]
    fn runtime_claude_trust_is_scoped_to_one_private_fresh_workspace() -> ScenarioResult<()> {
        let temp = tempfile::tempdir()?;
        let home = temp.path().join("home");
        let workspace = temp.path().join("work/fresh-scenario");
        fs::create_dir_all(&home)?;
        fs::create_dir_all(&workspace)?;
        fs::write(
            home.join(".claude.json"),
            "{\"hasCompletedOnboarding\":true}",
        )?;
        trust_run_workspace(&home, &workspace)?;
        let config: Value = serde_json::from_slice(&fs::read(home.join(".claude.json"))?)?;
        assert_eq!(config["hasCompletedOnboarding"], true);
        assert_eq!(config["projects"].as_object().ok_or("projects")?.len(), 1);
        assert_eq!(
            config["projects"][workspace.to_str().ok_or("UTF8")?]["hasTrustDialogAccepted"],
            true
        );
        let before = fs::read(home.join(".claude.json"))?;
        assert!(trust_run_workspace(&home, temp.path()).is_err());
        assert_eq!(fs::read(home.join(".claude.json"))?, before);
        Ok(())
    }
    /// Trace: FR-001 AC-6
    #[test]
    fn evidence_context_binds_workspace_model_across_claude_and_codex_calls() -> ScenarioResult<()>
    {
        let temp = tempfile::tempdir()?;
        let workspace = temp.path().join("work");
        fs::create_dir_all(workspace.join("docs"))?;
        fs::write(workspace.join("model.fsl"), "workspace model")?;
        fs::write(workspace.join("docs/model.fsl"), "different model")?;
        let tools = Tools {
            fsl: temp.path().join("fsl"),
            docs: temp.path().join("published"),
            quire: temp.path().join("quire"),
            quoin: temp.path().join("quoin"),
        };
        let transcript = workspace.join(".cli-agent-evals/transcripts");
        fs::create_dir_all(&transcript)?;
        for (persistent, operand, expected) in [
            (true, "model.fsl", false),
            (true, "../model.fsl", true),
            (false, "model.fsl", false),
            (false, "../model.fsl", true),
        ] {
            let calls = if persistent {
                vec![
                    json!({"message":{"content":[{"type":"tool_use","name":"Bash","input":{"command":"cd docs"}}]}}),
                    json!({"message":{"content":[{"type":"tool_use","name":"Bash","input":{"command":format!("fsl check eval authored {operand} --format json > check.json")}}]}}),
                ]
            } else {
                vec![
                    json!({"payload":{"type":"function_call","name":"exec_command","arguments":json!({"workdir":"docs","cmd":format!("fsl check eval authored {operand} --format json > check.json")}).to_string()}}),
                ]
            };
            fs::write(
                transcript.join("sample.transcript"),
                calls
                    .iter()
                    .map(Value::to_string)
                    .collect::<Vec<_>>()
                    .join("\n"),
            )?;
            let inputs = agent_inputs(&workspace)?;
            assert_eq!(
                recorded_command_evidence(&workspace, &tools, &inputs, "check", "model.fsl", true),
                expected,
                "persistent={persistent}, operand={operand}"
            );
        }
        // A later ordinary Codex call starts back in the workspace; a workdir
        // override on its predecessor must not leak into its execution context.
        let inputs = [
            recorded(json!({"workdir":"docs","cmd":"pwd"})),
            recorded(json!({"cmd":"fsl check eval authored model.fsl --format json"})),
        ];
        assert!(recorded_command_evidence(
            &workspace,
            &tools,
            &inputs,
            "check",
            "model.fsl",
            true
        ));
        Ok(())
    }
    /// Trace: FR-001 AC-6
    #[test]
    fn integrity_resolution_changes_do_not_supply_trusted_evidence() {
        for command in [
            "export PATH=.; fsl check model.fsl",
            "alias fsl=./fake; fsl check model.fsl",
        ] {
            assert!(!matches_invocation(
                command,
                "check",
                Some("model.fsl"),
                None
            ));
            let mut violations = Vec::new();
            let workspace = Path::new("/work");
            let tools = Tools {
                fsl: "/tools/fsl".into(),
                docs: "/docs".into(),
                quire: "/tools/quire".into(),
                quoin: "/tools/quoin".into(),
            };
            inspect_command(
                command,
                &mut workspace.to_owned(),
                workspace,
                &tools,
                true,
                &mut violations,
            );
            assert!(violations.iter().any(|v| v.starts_with("harness-error:")));
        }
    }
    /// Trace: FR-001 AC-6
    #[test]
    fn integrity_cwd_and_write_policy_use_actual_tool_inputs() -> ScenarioResult<()> {
        let temp = tempfile::tempdir()?;
        let workspace = temp.path().join("work");
        fs::create_dir_all(workspace.join("docs"))?;
        fs::write(workspace.join("unit.fsl"), "owned")?;
        let tools = Tools {
            fsl: temp.path().join("fsl"),
            docs: temp.path().join("published"),
            quire: temp.path().join("quire"),
            quoin: temp.path().join("quoin"),
        };
        for value in [
            json!({"command":"cd docs && cat ../unit.fsl"}),
            json!({"workdir":"docs","cmd":"cat ../unit.fsl"}),
            json!({"command":"/usr/bin/env"}),
            json!({"command":"cat < docs/readme.md"}),
        ] {
            assert!(
                boundary_violations(&workspace, &tools, &[recorded(value.clone())], true)
                    .is_empty(),
                "{value}"
            );
        }
        for value in [
            json!({"command":"cd docs && cat ../../outside"}),
            json!({"command":"cat ${HOME}/dev/reference.fsl"}),
            json!({"command":"cat $HOME/dev/reference.fsl"}),
            json!({"command":"echo bad > /usr/bin/agent-eval-test"}),
            json!({"command":"gh api repos/agent-ix/filament-examples/contents"}),
        ] {
            assert!(
                !boundary_violations(&workspace, &tools, &[recorded(value.clone())], true)
                    .is_empty(),
                "{value}"
            );
        }
        let transcript = workspace.join(".cli-agent-evals/transcripts");
        fs::create_dir_all(&transcript)?;
        let calls = [
            json!({"message":{"content":[{"type":"tool_use","name":"Bash","input":{"command":"cd docs"}}]}}),
            json!({"message":{"content":[{"type":"tool_use","name":"Read","input":{"file_path":"../unit.fsl"}}]}}),
            json!({"payload":{"type":"function_call","name":"exec_command","arguments":"{\"cmd\":\"cat ../unit.fsl\",\"workdir\":\"docs\"}"}}),
        ];
        fs::write(
            transcript.join("sample.transcript"),
            calls
                .iter()
                .map(Value::to_string)
                .collect::<Vec<_>>()
                .join("\n"),
        )?;
        assert!(
            boundary_violations(&workspace, &tools, &agent_inputs(&workspace)?, true).is_empty()
        );
        let unknown = boundary_violations(
            &workspace,
            &tools,
            &[recorded(json!({"cmd":"cd \"$(pwd)/docs\""}))],
            true,
        );
        assert!(unknown.iter().any(|s| s.starts_with("harness-error:")));
        // Bind an absolute source operand to this workspace, and reject the
        // same basename in another directory or after changing directories.
        assert!(matches_invocation_in(
            &format!(
                "fsl check eval model '{}'",
                workspace.join("model.fsl").display()
            ),
            "check",
            Some("model.fsl"),
            Some(&tools.fsl),
            Some(&workspace)
        ));
        assert!(!matches_invocation_in(
            "cd docs && fsl check eval model model.fsl",
            "check",
            Some("model.fsl"),
            Some(&tools.fsl),
            Some(&workspace)
        ));
        Ok(())
    }
    /// Trace: FR-001 AC-6
    #[test]
    fn integrity_command_evidence_binds_literal_operand_and_trusted_program() -> ScenarioResult<()>
    {
        let temp = tempfile::tempdir()?;
        let trusted = temp.path().join("fsl");
        fs::write(&trusted, "trusted")?;
        let other = temp.path().join("other/fsl");
        fs::create_dir_all(other.parent().ok_or("parent")?)?;
        fs::write(&other, "other")?;
        for script in [
            "fsl check eval owned model.fsl --format json > check.json",
            "fsl check eval owned './model.fsl' --format json # done",
            "fsl check model.fsl --format json",
        ] {
            assert!(
                matches_invocation(script, "check", Some("model.fsl"), Some(&trusted)),
                "{script}"
            );
        }
        assert!(matches_invocation(
            &format!("'{}' check model.fsl", trusted.display()),
            "check",
            Some("model.fsl"),
            Some(&trusted)
        ));
        for script in [
            "fsl check eval other other.fsl --format json # model.fsl",
            "fsl check other.fsl > model.fsl",
            "fsl check other.fsl --format model.fsl",
            "fsl check nested/model.fsl",
            "echo 'fsl check model.fsl'",
            "echo 'hello && fsl check model.fsl'",
            "fsl check \"$(echo model.fsl)\"",
            "fsl check other.fsl\n# fsl check model.fsl",
        ] {
            assert!(
                !matches_invocation(script, "check", Some("model.fsl"), Some(&trusted)),
                "{script}"
            );
        }
        assert!(!matches_invocation(
            &format!("{} check model.fsl", other.display()),
            "check",
            Some("model.fsl"),
            Some(&trusted)
        ));
        Ok(())
    }
    /// Trace: FR-001 AC-6
    #[test]
    fn integrity_boundary_rejects_reference_reads_source_paths_and_fetches() -> ScenarioResult<()> {
        let temp = tempfile::tempdir()?;
        let workspace = temp.path().join("work");
        let docs = temp.path().join("published");
        let source = temp.path().join("evaluator");
        fs::create_dir_all(&workspace)?;
        fs::create_dir_all(&docs)?;
        fs::create_dir_all(&source)?;
        fs::write(source.join("reference.fsl"), "private answer")?;
        fs::write(docs.join("guide.md"), "published docs")?;
        fs::create_dir_all(workspace.join("docs"))?;
        std::os::unix::fs::symlink(&docs, workspace.join("docs/shipped"))?;
        std::os::unix::fs::symlink(source.join("reference.fsl"), workspace.join("answer"))?;
        let tools = Tools {
            fsl: temp.path().join("fsl"),
            docs: docs.clone(),
            quire: temp.path().join("quire"),
            quoin: temp.path().join("quoin"),
        };
        for input in [
            json!({"command":format!("cat {}",source.join("reference.fsl").display())}),
            json!({"file_path":source.join("reference.fsl")}),
            json!({"path":"../evaluator/reference.fsl"}),
            json!({"command":"cat answer"}),
            json!({"cmd":"git clone https://github.com/agent-ix/filament-examples"}),
            json!({"cmd":"curl https://raw.githubusercontent.com/agent-ix/filament-examples/main/agent-evals/fixtures/EV-001/reference.fsl"}),
        ] {
            assert!(
                !boundary_violations(&workspace, &tools, &[recorded(input.clone())], true)
                    .is_empty(),
                "{input}"
            );
        }
        assert!(
            boundary_violations(
                &workspace,
                &tools,
                &[
                    recorded(json!({"command":"cat docs/shipped/guide.md"})),
                    recorded(json!({"file_path":docs.join("guide.md")})),
                    recorded(json!({"command":"fsl check model.fsl --format json > check.json"})),
                ],
                true
            )
            .is_empty()
        );
        // Exercise enforcement on a real retained SDK tool call, with an
        // otherwise-passing assertion: inspecting the answer must flip it.
        let transcript = workspace.join(".cli-agent-evals/transcripts");
        fs::create_dir_all(&transcript)?;
        fs::write(transcript.join("sample.transcript"),json!({"message":{"content":[{"type":"tool_use","input":{"file_path":source.join("reference.fsl")}}]}}).to_string())?;
        let mut assertion = Assertion {
            ok: true,
            checks: Default::default(),
            failures: Vec::new(),
        };
        enforce_transcript_boundary(&workspace, &tools, true, &mut assertion)?;
        assert!(!assertion.ok);
        assert_eq!(assertion.checks["transcript_boundary"]["ok"], false);
        Ok(())
    }
    #[test]
    fn actual_tool_calls_supply_command_evidence_but_assistant_text_does_not() -> ScenarioResult<()>
    {
        let workspace = tempfile::tempdir()?;
        let transcript = workspace.path().join(".cli-agent-evals/transcripts");
        fs::create_dir_all(&transcript)?;
        let records = [
            json!({"message":{"content":[{"type":"text","text":"fsl check fake"},{"type":"tool_use","input":{"command":"fsl check eval model model.fsl --format json > check.json"}}]}}),
            json!({"type":"response_item","payload":{"type":"function_call","arguments":"{\"cmd\":\"fsl analyze holding-request.json --format json\"}"}}),
            json!({"message":{"content":[{"type":"text","text":"fsl analyze violating-request.json"}]}}),
        ];
        fs::write(
            transcript.join("sample.transcript"),
            records
                .iter()
                .map(Value::to_string)
                .collect::<Vec<_>>()
                .join("\n"),
        )?;
        let commands = agent_commands(workspace.path())?;
        assert_eq!(commands.len(), 2);
        assert!(
            commands
                .iter()
                .any(|c| invokes_fsl(c, "check", Some("model.fsl")))
        );
        assert!(
            commands
                .iter()
                .any(|c| invokes_fsl(c, "analyze", Some("holding-request.json")))
        );
        assert!(!commands.iter().any(|c| invokes_fsl(
            c,
            "analyze",
            Some("violating-request.json")
        )));
        assert!(!invokes_fsl(
            "echo 'fsl check model.fsl'",
            "check",
            Some("model.fsl")
        ));
        assert!(!invokes_fsl(
            "echo 'example && fsl check model.fsl'",
            "check",
            Some("model.fsl")
        ));
        assert!(!invokes_fsl(
            "fsl check other-model.fsl",
            "check",
            Some("model.fsl")
        ));
        Ok(())
    }
    #[test]
    fn provider_rejects_unknown_protocol_members_and_scenarios() {
        let valid = json!({"protocol":"cli-agent-evals.external-provider-request/v1","operation":"describe","context":{"scenario":{"id":""},"work_dir":"","cwd":"","session_id":"","report_dir":""}});
        let described = respond(valid.to_string().as_bytes()).unwrap();
        assert_eq!(described["scenarios"][0]["id"], "EV-001");
        let mut unknown = valid.clone();
        unknown["extra"] = json!(true);
        assert!(respond(unknown.to_string().as_bytes()).is_err());
        let mut version = valid.clone();
        version["protocol"] = json!("cli-agent-evals.external-provider-request/v2");
        assert!(respond(version.to_string().as_bytes()).is_err());
        let mut scenario = valid;
        scenario["operation"] = json!("prepare");
        scenario["context"]["scenario"]["id"] = json!("EV-101");
        assert!(respond(scenario.to_string().as_bytes()).is_err());
    }
    fn reference_policy_fixture() -> ScenarioResult<(tempfile::TempDir, PathBuf, Tools)> {
        let temp = tempfile::tempdir()?;
        let workspace = temp.path().join("work");
        let docs = temp.path().join("published");
        fs::create_dir_all(workspace.join("docs"))?;
        fs::create_dir_all(&docs)?;
        fs::write(
            docs.join("guide.md"),
            "Original public guide used by this access-policy test",
        )?;
        fs::write(temp.path().join("grammar.md"), "Original grammar marker")?;
        fs::write(workspace.join("postage.fsl"), "owned source")?;
        fs::write(workspace.join("postage-example.json"), "owned request")?;
        fs::write(workspace.join("model.fsl"), "owned model")?;
        std::os::unix::fs::symlink(&docs, workspace.join("docs/shipped"))?;
        std::os::unix::fs::symlink(
            temp.path().join("grammar.md"),
            workspace.join("docs/native-grammar.md"),
        )?;
        let tools = Tools {
            fsl: temp.path().join("fsl"),
            docs,
            quire: temp.path().join("quire"),
            quoin: temp.path().join("quoin"),
        };
        Ok((temp, workspace, tools))
    }

    fn write_policy_transcript(workspace: &Path, access: Value) -> ScenarioResult<()> {
        let transcript = workspace.join(".cli-agent-evals/transcripts");
        fs::create_dir_all(&transcript)?;
        let records = [
            access,
            json!({"command":"fsl check postage.fsl --format json"}),
            json!({"command":"fsl run postage-example.json --format json"}),
            json!({"command":"fsl check model.fsl --format json"}),
        ];
        let records = records
            .iter()
            .map(|input| {
                json!({"message":{"content":[{"type":"tool_use","name":"Bash","input":input}]}})
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(transcript.join("sample.transcript"), records)?;
        Ok(())
    }

    #[test]
    fn scenario_reference_policy_refusal_only_rejects_docs_but_allows_task_source()
    -> ScenarioResult<()> {
        let (_temp, workspace, tools) = reference_policy_fixture()?;
        for access in [
            json!({"file_path":tools.docs.join("guide.md")}),
            json!({"command":"cat docs/shipped/guide.md"}),
            json!({"command":"cat docs/native-grammar.md"}),
        ] {
            write_policy_transcript(&workspace, access.clone())?;
            let assertion = retained_evidence(&workspace, &tools, refusal_repair::ID)?;
            assert!(!assertion.ok, "forbidden access was accepted: {access}");
            assert_eq!(assertion.checks["transcript_boundary"]["ok"], false);
            assert!(
                assertion
                    .failures
                    .iter()
                    .any(|failure| failure.contains("this scenario permits no shipped")),
                "{:?}",
                assertion.failures
            );
            assert_eq!(
                assertion.checks["command_evidence"]["check:postage.fsl"],
                true
            );
            assert_eq!(
                assertion.checks["command_evidence"]["run:postage-example.json"],
                true
            );
        }
        write_policy_transcript(
            &workspace,
            json!({"file_path":workspace.join("postage.fsl")}),
        )?;
        let assertion = retained_evidence(&workspace, &tools, refusal_repair::ID)?;
        assert!(
            assertion.ok,
            "task-source access must remain allowed: {:?}",
            assertion.failures
        );
        Ok(())
    }

    #[test]
    fn scenario_reference_policy_model_authoring_keeps_published_reads_allowed()
    -> ScenarioResult<()> {
        let (_temp, workspace, tools) = reference_policy_fixture()?;
        for access in [
            json!({"file_path":tools.docs.join("guide.md")}),
            json!({"command":"cat docs/shipped/guide.md"}),
            json!({"command":"cat docs/native-grammar.md"}),
        ] {
            write_policy_transcript(&workspace, access.clone())?;
            let assertion = retained_evidence(&workspace, &tools, author_model::ID)?;
            assert!(
                assertion.ok,
                "published access rejected: {access}; {:?}",
                assertion.failures
            );
            assert_eq!(assertion.checks["transcript_boundary"]["ok"], true);
            assert_eq!(
                assertion.checks["command_evidence"]["check:model.fsl"],
                true
            );
        }
        Ok(())
    }
    #[test]
    fn heredoc_command_evidence_accepts_real_trailing_public_invocations() -> ScenarioResult<()> {
        let (_temp, workspace, tools) = reference_policy_fixture()?;
        let transcript = workspace.join(".cli-agent-evals/transcripts");
        fs::create_dir_all(&transcript)?;
        // Recorded shell input only: this editor body is never executed by the test.
        let script = r#"python - <<'PY'
with open('postage.fsl', 'r+') as source:
    repaired = source.read().replace('units >= 0', 'if units >= 0 then 7 + 3 * units else 7')
    source.seek(0)
    source.write(repaired)
    source.truncate()
PY
fsl check postage.fsl
fsl run postage-example.json --format json
"#;
        let cwd = url::Url::from_directory_path(&workspace)
            .map_err(|_| "workspace URL")?
            .to_string();
        fs::write(
            transcript.join("sample.transcript"),
            json!({
                "type":"event_msg", "payload":{"type":"item_completed", "item":{
                    "type":"CommandExecution", "command":["/bin/bash","-lc",script],
                    "cwd":cwd, "exit_code":0, "status":"completed",
                    "aggregated_output":"Output is not command evidence."
                }}
            })
            .to_string(),
        )?;
        let assertion = retained_evidence(&workspace, &tools, refusal_repair::ID)?;
        assert!(
            assertion.ok,
            "{:?}; literal tokens {:?}",
            assertion.failures,
            command_evidence_tokens(script)
        );
        assert_eq!(assertion.checks["agent_command_count"], 1);
        assert_eq!(
            assertion.checks["command_evidence"]["check:postage.fsl"],
            true
        );
        assert_eq!(
            assertion.checks["command_evidence"]["run:postage-example.json"],
            true
        );
        assert_eq!(assertion.checks["transcript_boundary"]["ok"], true);
        Ok(())
    }

    #[test]
    fn heredoc_command_evidence_rejects_body_data_and_nonexecuted_claims() {
        for script in [
            "python - <<'PY'\nprint('fsl check postage.fsl')\nprint('fsl run postage-example.json')\nPY\n",
            "cat <<'TEXT'\nfsl check postage.fsl\nfsl run postage-example.json\nTEXT\n",
            "cat <<TEXT\nfsl check postage.fsl\nfsl run postage-example.json\nTEXT\n",
            "cat <<'TEXT'\nignored data\nTEXT\necho 'fsl check postage.fsl; fsl run postage-example.json'",
            "# fsl check postage.fsl\n# fsl run postage-example.json",
            "echo 'fsl check postage.fsl'\nprintf '%s\\n' 'fsl run postage-example.json'",
            "if false; then\nfsl check postage.fsl\nfsl run postage-example.json\nfi",
            "false && fsl check postage.fsl && fsl run postage-example.json",
            "fake() { fsl check postage.fsl; fsl run postage-example.json; }",
            "fsl() { echo fake; }\nfsl check postage.fsl\nfsl run postage-example.json",
            "source ./replacement.sh\nfsl check postage.fsl\nfsl run postage-example.json",
            ". ./replacement.sh\nfsl check postage.fsl\nfsl run postage-example.json",
            "PATH=/fake\nfsl check postage.fsl\nfsl run postage-example.json",
            "PATH=/fake fsl check postage.fsl\nfsl run postage-example.json",
            "export PATH=/fake\nfsl check postage.fsl\nfsl run postage-example.json",
            "alias fsl='echo fake'\nfsl check postage.fsl\nfsl run postage-example.json",
            "hash -p /fake/fsl fsl\nfsl check postage.fsl\nfsl run postage-example.json",
            "cat <<'TEXT' && fsl check postage.fsl && fsl run postage-example.json\nbody\nTEXT\n",
        ] {
            assert!(
                !matches_invocation(script, "check", Some("postage.fsl"), None),
                "false check evidence: {script}"
            );
            assert!(
                !matches_invocation(script, "run", Some("postage-example.json"), None),
                "false run evidence: {script}"
            );
        }
    }
    #[test]
    fn heredoc_command_evidence_stops_at_top_level_termination() {
        // Parser input, never executed. Body text cannot alter parent-shell flow.
        let editor = "python - <<'PY'\nprint('editor data only')\nPY\n";
        for barrier in ["exit 0", "exec /bin/true", "return 0", "command exit 0"] {
            let unreachable = format!(
                "{editor}{barrier}\nfsl check postage.fsl\nfsl run postage-example.json --format json\n"
            );
            assert!(
                !matches_invocation(&unreachable, "check", Some("postage.fsl"), None),
                "unreachable check after {barrier}"
            );
            assert!(
                !matches_invocation(&unreachable, "run", Some("postage-example.json"), None),
                "unreachable run after {barrier}"
            );
            let partly_reachable = format!(
                "{editor}fsl check postage.fsl\n{barrier}\nfsl run postage-example.json --format json\n"
            );
            assert!(
                matches_invocation(&partly_reachable, "check", Some("postage.fsl"), None),
                "reachable check before {barrier}"
            );
            assert!(
                !matches_invocation(&partly_reachable, "run", Some("postage-example.json"), None),
                "unreachable run after {barrier}"
            );
        }
        let reachable = format!(
            "{editor}fsl check postage.fsl\nfsl run postage-example.json --format json\nexit 0\n"
        );
        assert!(matches_invocation(
            &reachable,
            "check",
            Some("postage.fsl"),
            None
        ));
        assert!(matches_invocation(
            &reachable,
            "run",
            Some("postage-example.json"),
            None
        ));
    }
}
