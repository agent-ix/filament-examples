//! Project setup and the thin Rust provider for the published cli-evals SDK.
use agent_evals::{ScenarioResult, provider};
use serde::Deserialize;
use serde_json::{Value, json};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Agents {
    codex: String,
    claude: String,
    repeats: usize,
}

struct CredentialHome(PathBuf);
impl Drop for CredentialHome {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn copy_auth(source: &Path, destination: &Path) -> ScenarioResult<()> {
    let parent = destination
        .parent()
        .ok_or("credential destination has no parent")?;
    fs::create_dir_all(parent)?;
    fs::copy(source, destination)?;
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(destination, fs::Permissions::from_mode(0o600))?;
    Ok(())
}
fn private_claude_config(operator: &Value) -> Value {
    let mut config = json!({"hasCompletedOnboarding":true});
    if let Some(account) = operator
        .get("oauthAccount")
        .filter(|value| value.is_object())
    {
        config["oauthAccount"] = account.clone();
    }
    config
}
fn initialize_home(agent: &str, home: &Path, model: &str) -> ScenarioResult<()> {
    fs::create_dir_all(home)?;
    let operator_home =
        PathBuf::from(std::env::var_os("HOME").ok_or("operator HOME is unavailable")?);
    match agent {
        "codex" => {
            let auth_root = std::env::var_os("CODEX_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| operator_home.join(".codex"));
            let auth = auth_root.join("auth.json");
            fs::create_dir_all(home.join(".codex"))?;
            if auth.is_file() {
                copy_auth(&auth, &home.join(".codex/auth.json"))?;
            } else if std::env::var_os("OPENAI_API_KEY").is_none() {
                return Err(
                    "Codex prerequisite missing: existing auth.json or OPENAI_API_KEY".into(),
                );
            }
            fs::write(
                home.join(".codex/config.toml"),
                format!("model = {}\n", serde_json::to_string(model)?),
            )?;
        }
        "claude" => {
            let auth = operator_home.join(".claude/.credentials.json");
            if auth.is_file() {
                copy_auth(&auth, &home.join(".claude/.credentials.json"))?;
            } else if std::env::var_os("CLAUDE_CODE_OAUTH_TOKEN").is_none()
                && std::env::var_os("ANTHROPIC_API_KEY").is_none()
            {
                return Err("Claude prerequisite missing: existing credentials or CLAUDE_CODE_OAUTH_TOKEN/ANTHROPIC_API_KEY".into());
            }
            let operator_config = match fs::read(operator_home.join(".claude.json")) {
                Ok(bytes) => serde_json::from_slice(&bytes)?,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => json!({}),
                Err(error) => return Err(error.into()),
            };
            fs::write(
                home.join(".claude.json"),
                serde_json::to_vec(&private_claude_config(&operator_config))?,
            )?;
        }
        _ => return Err("select codex or claude".into()),
    }
    Ok(())
}
fn run(scenario: &str, agent: &str) -> ScenarioResult<()> {
    if !provider::is_active_scenario(scenario) {
        return Err(
            "run requires an implemented individual scenario; EV-101 is pending QSL-1980".into(),
        );
    }
    let suite = Path::new(env!("CARGO_MANIFEST_DIR"));
    let agents: Agents = toml::from_str(&fs::read_to_string(suite.join("agents.toml"))?)?;
    if agents.repeats != 1 {
        return Err(
            "this authoring phase requires one sample per reviewed scenario/provider".into(),
        );
    }
    let model = match agent {
        "codex" => &agents.codex,
        "claude" => &agents.claude,
        _ => return Err("select codex or claude".into()),
    };
    let root = tempfile::Builder::new()
        .prefix("fsl-agent-eval-")
        .tempdir()?
        .keep();
    let home = root.join("home");
    if let Err(error) = initialize_home(agent, &home, model) {
        let _ = fs::remove_dir_all(&root);
        return Err(error);
    }
    let auth_cleanup = CredentialHome(home.clone());
    let work = root.join("work");
    fs::create_dir_all(&work)?;
    let raw = root.join("raw-report.json");
    let cli = suite.join("node_modules/.bin/cli-evals");
    let provider_dir = std::env::current_exe()?
        .parent()
        .ok_or("provider executable has no folder")?
        .to_owned();
    let mut paths = vec![provider_dir];
    paths.extend(std::env::split_paths(
        &std::env::var_os("PATH").ok_or("PATH missing")?,
    ));
    let mut command = Command::new(cli);
    command
        .current_dir(suite)
        .args(["run", "--suite"])
        .arg(suite.join("cli-agent-evals.config.mjs"))
        .args([
            "--filter",
            scenario,
            "--agent",
            agent,
            "--model",
            model,
            "--repeats",
            "1",
            "--keep",
            "--report",
        ])
        .arg(&raw)
        .env("HOME", &home)
        .env("FSL_EVAL_HOME", &home)
        .env("FSL_EVAL_AGENT", agent)
        .env("TMPDIR", &work)
        .env("PATH", std::env::join_paths(paths)?)
        .env_remove("CLAUDE_CONFIG_DIR")
        .env_remove("CODEX_HOME")
        .env_remove("CLAUDECODE")
        .env_remove("CARGO_TARGET_DIR")
        .env_remove("CARGO_MANIFEST_DIR");
    for kind in ["CONFIG", "DATA", "STATE", "CACHE"] {
        command.env(format!("XDG_{kind}_HOME"), home.join(kind.to_lowercase()));
    }
    let status = command.status();
    drop(auth_cleanup);
    let status = status?;
    if !matches!(status.code(), Some(0 | 1)) || !raw.is_file() {
        return Err(format!(
            "harness-error: SDK did not complete a report; exit {:?}; private run {}",
            status.code(),
            root.display()
        )
        .into());
    }
    println!(
        "{}",
        json!({"format":"fsl-agent-eval-run/v1","scenario":scenario,"agent":agent,"model":model,"runner_exit":status.code(),"raw_report":raw,"private_run":root})
    );
    Ok(())
}
fn summarize(file: &Path) -> ScenarioResult<()> {
    let report: Value = serde_json::from_slice(&fs::read(file)?)?;
    if report["reportVersion"] != "cli-agent-evals.report/v1" || report["repeats"] != 1 {
        return Err("expected one-sample SDK report/v1".into());
    }
    let results = report["results"]
        .as_array()
        .ok_or("report has no results")?;
    let rows:Vec<Value>=results.iter().map(|r|json!({"scenario":r["id"],"ok":r["ok"],"pass_rate":r["passRate"],"tool_calls_p50":r["aggregate"]["toolCalls"]["p50"],"turns_p50":r["aggregate"]["assistantTurns"]["p50"],"wall_ms_p50":r["aggregate"]["latencyMs"]["p50"],"samples":r["runs"].as_array().map(|runs|runs.iter().map(|sample|json!({"ok":sample["ok"],"exit_reason":sample["exitReason"],"tool_calls":sample["toolCalls"],"recorded_commands":sample["checks"]["agent_command_count"],"checks":sample["checks"],"failure_count":sample["failures"].as_array().map(Vec::len)})).collect::<Vec<_>>())})).collect();
    println!(
        "{}",
        serde_json::to_string_pretty(
            &json!({"format":"fsl-agent-score/v1","agent":report["agent"],"model":report["model"],"samples_per_scenario":1,"rows":rows})
        )?
    );
    Ok(())
}
fn execute() -> ScenarioResult<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [operation] if operation=="provider"=>{
            let mut bytes=Vec::new();std::io::stdin().take(1_048_577).read_to_end(&mut bytes)?;
            let response=provider::respond(&bytes)?;
            let mut output=std::io::stdout().lock();serde_json::to_writer(&mut output,&response)?;writeln!(output)?;
            Ok(())
        },
        [operation,scenario,agent] if operation=="run"=>run(scenario,agent),
        [operation,file] if operation=="summarize"=>summarize(Path::new(file)),
        [operation,scenario,workspace] if operation=="evidence"=>{
            let result=provider::retained_evidence(Path::new(workspace),&agent_evals::Tools::from_environment()?,scenario)?;
            println!("{}",serde_json::to_string_pretty(&result)?);Ok(())
        },
        _=>Err("usage: agent-evals provider | run <EV-ID> <codex|claude> | summarize <private-report.json> | evidence <EV-ID> <retained-workspace>".into()),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    /// Trace: FR-001 AC-6
    #[test]
    fn private_claude_config_retains_only_existing_account_metadata() {
        let account = json!({"accountUuid":"fake-account","organizationUuid":"fake-organization"});
        let operator = json!({"oauthAccount":account,"projects":{"/private/operator/project":{"hasTrustDialogAccepted":true}},"apiKeyHelper":"operator-only-helper","plugins":{"operator":"private"},"hasCompletedOnboarding":false});
        let private = private_claude_config(&operator);
        assert_eq!(
            private,
            json!({"hasCompletedOnboarding":true,"oauthAccount":account})
        );
        assert_eq!(
            private_claude_config(&json!({"projects":{}})),
            json!({"hasCompletedOnboarding":true})
        );
    }
}
fn main() -> ExitCode {
    match execute() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}
