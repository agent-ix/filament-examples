//! Thin Rust consumer of the SDK's published external-provider protocol.
use crate::{Assertion, ScenarioResult, Tools, scenarios::author_model};
use serde::Deserialize;
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};

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
    // Opaque SDK-owned process result; it never supplies the model grader's verdict.
    #[serde(default, rename = "run")]
    _run: Option<Value>,
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
    if let Some(parts) = value.get("command").and_then(Value::as_array) {
        if parts.iter().all(Value::is_string) {
            commands.push(
                parts
                    .iter()
                    .filter_map(Value::as_str)
                    .collect::<Vec<_>>()
                    .join(" "),
            );
        }
    }
    if let Some(action) = value.get("action") {
        collect_commands(action, commands);
    }
}
/// Read actual tool-call commands from the SDK's retained transcript, never assistant prose.
pub fn agent_commands(workspace: &Path) -> ScenarioResult<Vec<String>> {
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
                    if item["type"] == "tool_use" {
                        if let Some(input) = item.get("input") {
                            collect_commands(input, &mut commands);
                        }
                    }
                }
            }
            let payload = record.get("payload").unwrap_or(&record);
            if payload["type"] == "function_call" {
                if let Some(args) = payload["arguments"].as_str() {
                    let args: Value = serde_json::from_str(args)
                        .map_err(|e| format!("harness-error: invalid tool arguments: {e}"))?;
                    collect_commands(&args, &mut commands);
                }
            } else if payload["type"] == "local_shell_call" {
                collect_commands(payload, &mut commands);
            }
        }
    }
    if !found {
        return Err("harness-error: SDK supplied no retained transcript".into());
    }
    Ok(commands)
}
/// Match a direct public FSL invocation, excluding an echo/comment of that command.
pub fn invokes_fsl(command: &str, verb: &str, operand: Option<&str>) -> bool {
    command.split(['\n', ';', '|', '&']).any(|part| {
        let words: Vec<&str> = part.split_whitespace().collect();
        let Some(program) = words.first() else {
            return false;
        };
        let program = program.trim_matches(['\'', '"']);
        Path::new(program)
            .file_name()
            .is_some_and(|name| name == "fsl")
            && words.get(1) == Some(&verb)
            && operand.is_none_or(|file| {
                words
                    .iter()
                    .any(|word| word.trim_matches(['\'', '"']).ends_with(file))
            })
    })
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
        return Ok(
            json!({"protocol":protocol,"operation":"describe","scenarios":[{"id":author_model::ID,"title":author_model::TITLE,"use_case":"source-owned-model-authoring","canary":false}]}),
        );
    }
    if request.context.scenario.id != author_model::ID {
        return Err("undescribed scenario".into());
    }
    let tools = Tools::from_environment()?;
    let workspace = fs::canonicalize(request.context.work_dir)?;
    match request.operation {
        Operation::Describe => Err("unreachable describe dispatch".into()),
        Operation::Prepare => {
            references(&workspace, &tools)?;
            let prompt = author_model::prepare(&workspace, &tools)?;
            let mut paths = Vec::new();
            for program in [&tools.fsl, &tools.quire, &tools.quoin] {
                if let Some(parent) = program.parent() {
                    paths.push(parent.to_owned());
                }
            }
            for program in ["node", "codex", "claude"] {
                if let Ok(path) = executable(program) {
                    if let Some(parent) = path.parent() {
                        paths.push(parent.to_owned());
                    }
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
            let mut assertion = match author_model::grade(&workspace, &tools) {
                Ok(result) => result,
                Err(e) => Assertion {
                    ok: false,
                    checks: Default::default(),
                    failures: vec![format!("harness-error: grader failed: {e}")],
                },
            };
            match agent_commands(&workspace) {
                Ok(commands) => {
                    let check = commands
                        .iter()
                        .any(|command| invokes_fsl(command, "check", Some("model.fsl")));
                    assertion
                        .checks
                        .insert("agent_command_count".to_owned(), json!(commands.len()));
                    assertion
                        .checks
                        .insert("agent_checked_model".to_owned(), json!(check));
                    if !check {
                        assertion.ok = false;
                        assertion.failures.push(
                            "agent never invoked the public fsl check on model.fsl".to_owned(),
                        );
                    }
                }
                Err(e) => {
                    assertion.ok = false;
                    assertion.failures.push(e.to_string());
                }
            }
            Ok(json!({"protocol":protocol,"operation":"assert","assertion":assertion}))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
