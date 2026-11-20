//! Thin Rust consumer of the SDK's published external-provider protocol.
use crate::{Assertion, ScenarioResult, Tools, scenarios::author_model};
use serde::Deserialize;
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};

struct Entry {
    id: &'static str,
    title: &'static str,
    use_case: &'static str,
    prepare: fn(&Path, &Tools) -> ScenarioResult<String>,
    grade: fn(&Path, &Tools) -> ScenarioResult<Assertion>,
    commands: &'static [(&'static str, &'static str)],
}
const ENTRIES: &[Entry] = &[Entry {
    id: author_model::ID,
    title: author_model::TITLE,
    use_case: "source-owned-model-authoring",
    prepare: author_model::prepare,
    grade: author_model::grade,
    commands: &[("check", "model.fsl")],
}];

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
                });
            } else if payload["type"] == "custom_tool_call" {
                if let Some(input) = payload["input"].as_str() {
                    commands.push(RecordedInput {
                        value: json!({"code":input}),
                        persistent_cwd: false,
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
    if changes_tool_resolution(command) {
        return false;
    }
    let Some(tokens) = shell_tokens(command) else {
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
        text.split(|c: char| c.is_whitespace() || "\"'`;(){}[],<>=".contains(c))
            .collect()
    };
    for word in words {
        let word = word.trim_end_matches(':');
        if word.starts_with('~') || word.starts_with("$HOME") || word.starts_with("${HOME}") {
            violations.push("sandbox-breach: home-directory access in tool input".to_owned());
            continue;
        }
        if word.is_empty() || !(word.contains('/') || word == ".." || cwd.join(word).exists()) {
            continue;
        }
        let path = resolve_path(cwd, word);
        let in_workspace = path.starts_with(workspace);
        let read_only = path.starts_with(&tools.docs)
            || grammar.as_ref().is_some_and(|g| &path == g)
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
fn inspect_command(
    command: &str,
    cwd: &mut PathBuf,
    workspace: &Path,
    tools: &Tools,
    violations: &mut Vec<String>,
) {
    if changes_tool_resolution(command) {
        violations.push("harness-error: agent changed trusted tool resolution".to_owned());
    }
    let Some(tokens) = shell_tokens(command) else {
        // Opaque code cannot establish command evidence. Still inspect literal
        // path accesses in code/heredocs, without pretending to execute a shell.
        inspect_paths(command, cwd, workspace, tools, false, violations);
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
                inspect_paths(words[1], cwd, workspace, tools, false, violations);
                *cwd = resolve_path(cwd, words[1]);
            }
            continue;
        }
        let mut redirect = false;
        let program = words
            .first()
            .and_then(|w| Path::new(w).file_name())
            .and_then(|p| p.to_str());
        for (index, token) in statement.iter().enumerate() {
            match token {
                ShellToken::Redirect(writes) => redirect = *writes,
                ShellToken::Word(word) => {
                    let destination = matches!(program, Some("cp" | "mv" | "ln"))
                        && statement[index + 1..]
                            .iter()
                            .all(|t| !matches!(t, ShellToken::Word(_)));
                    let writes = redirect
                        || destination
                        || (index > 0 && matches!(program, Some("rm" | "mkdir" | "touch" | "tee")));
                    inspect_paths(word, cwd, workspace, tools, writes, violations);
                    redirect = false;
                }
                ShellToken::Separator => {}
            }
        }
    }
}
fn boundary_violations(workspace: &Path, tools: &Tools, inputs: &[RecordedInput]) -> Vec<String> {
    let mut violations = Vec::new();
    let mut persistent_cwd = workspace.to_owned();
    for input in inputs {
        let mut cwd = if input.persistent_cwd {
            persistent_cwd.clone()
        } else {
            workspace.to_owned()
        };
        if let Some(workdir) = input.value.get("workdir").and_then(Value::as_str) {
            inspect_paths(workdir, workspace, workspace, tools, false, &mut violations);
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
                inspect_paths(
                    text,
                    &cwd,
                    workspace,
                    tools,
                    input.writes_files,
                    &mut violations,
                );
            }
        }
        for command in commands {
            inspect_command(&command, &mut cwd, workspace, tools, &mut violations);
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
    assertion: &mut Assertion,
) -> ScenarioResult<()> {
    let inputs = agent_inputs(workspace)?;
    let violations = boundary_violations(workspace, tools, &inputs);
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
            inspect_command(&command, &mut cwd, workspace, tools, &mut Vec::new());
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
    enforce_transcript_boundary(workspace, tools, assertion)?;
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
                let check = recorded_command_evidence(workspace, tools, &inputs, verb, operand);
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
            references(&workspace, &tools)?;
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
    fn recorded(value: Value) -> RecordedInput {
        RecordedInput {
            value,
            persistent_cwd: false,
            writes_files: false,
        }
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
                recorded_command_evidence(&workspace, &tools, &inputs, "check", "model.fsl"),
                expected
            );
            assert!(boundary_violations(&workspace, &tools, &inputs).is_empty());
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
                recorded_command_evidence(&workspace, &tools, &inputs, "check", "model.fsl"),
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
            "model.fsl"
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
                boundary_violations(&workspace, &tools, &[recorded(value.clone())]).is_empty(),
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
                !boundary_violations(&workspace, &tools, &[recorded(value.clone())]).is_empty(),
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
        assert!(boundary_violations(&workspace, &tools, &agent_inputs(&workspace)?).is_empty());
        let unknown = boundary_violations(
            &workspace,
            &tools,
            &[recorded(json!({"cmd":"cd \"$(pwd)/docs\""}))],
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
                !boundary_violations(&workspace, &tools, &[recorded(input.clone())]).is_empty(),
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
                ]
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
        enforce_transcript_boundary(&workspace, &tools, &mut assertion)?;
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
}
