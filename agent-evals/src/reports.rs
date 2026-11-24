//! Redaction and deterministic rendering for committed evaluation runs.
use regex::Regex;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

/// Build a sanitized committed run folder from its private source files.
pub fn write_run_folder(
    reports_dir: &Path,
    run_id: &str,
    run_root: &Path,
    home: &Path,
    tools_file: &Path,
    triage_file: &Path,
    codex_file: &Path,
    claude_file: &Path,
) -> Result<PathBuf, String> {
    if run_id.is_empty()
        || !run_id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    {
        return Err("run id must contain only ASCII letters, digits, '-' or '_'".into());
    }
    let destination = reports_dir.join(run_id);
    if destination.exists() {
        return Err(format!(
            "run folder already exists: {}",
            destination.display()
        ));
    }
    let tools: Value = serde_json::from_slice(&fs::read(tools_file).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let triage = fs::read_to_string(triage_file).map_err(|e| e.to_string())?;
    let triage = redact_base_paths(&triage, run_root, &home.to_string_lossy());
    let triage = absolute_path_regex()
        .replace_all(&triage, "<path>")
        .into_owned();
    fs::create_dir_all(&destination).map_err(|e| e.to_string())?;
    let result = (|| {
        fs::write(
            destination.join("tools.json"),
            serde_json::to_vec_pretty(&tools).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        fs::write(destination.join("triage.md"), triage).map_err(|e| e.to_string())?;
        for (agent, source) in [("codex", codex_file), ("claude", claude_file)] {
            let report: Value =
                serde_json::from_slice(&fs::read(source).map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?;
            let report = redact_report(report, run_root, home)?;
            fs::write(
                destination.join(format!("{agent}.json")),
                serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
        }
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_dir_all(&destination);
    }
    result.map(|()| destination)
}

/// Redact one private SDK report for publication under a run directory.
pub fn redact_report(mut report: Value, run_root: &Path, home: &Path) -> Result<Value, String> {
    if report["reportVersion"] != "cli-agent-evals.report/v1" || !report.is_object() {
        return Err("expected cli-agent-evals.report/v1 object".into());
    }
    let allowed = [
        "reportVersion",
        "ok",
        "generatedAt",
        "suite",
        "agent",
        "model",
        "repeats",
        "results",
        "aggregates",
    ];
    if report
        .as_object()
        .is_some_and(|object| object.keys().any(|key| !allowed.contains(&key.as_str())))
    {
        return Err("report/v1 contains an unknown member".into());
    }
    let home = home.to_string_lossy().into_owned();
    walk_strings(&mut report, &|s| redact_base_paths(s, run_root, &home));
    if let Some(results) = report["results"].as_array_mut() {
        for result in results {
            if let Some(runs) = result["runs"].as_array_mut() {
                for sample in runs {
                    for key in ["workDir", "transcriptPath"] {
                        if let Some(value) = sample[key].as_str() {
                            if !value.starts_with("$RUN/") && key == "workDir" {
                                return Err(format!("{key} must be run-relative after redaction"));
                            }
                        }
                    }
                    for key in ["failures", "checks"] {
                        if !sample.get(key).is_some() {
                            continue;
                        }
                        walk_strings(&mut sample[key], &|s| {
                            absolute_path_regex().replace_all(s, "<path>").into_owned()
                        });
                    }
                }
            }
        }
    }
    Ok(report)
}

fn redact_base_paths(input: &str, run_root: &Path, home: &str) -> String {
    let root = run_root.to_string_lossy().into_owned();
    let mut output = input.replace(&root, "$RUN");
    if let Ok(path) = run_root.canonicalize() {
        output = output.replace(&path.to_string_lossy().to_string(), "$RUN");
    }
    if let Some(rest) = root.strip_prefix("/private/var/folders/") {
        output = output.replace(&format!("/var/folders/{rest}"), "$RUN");
    }
    if let Some(rest) = root.strip_prefix("/var/folders/") {
        output = output.replace(&format!("/private/var/folders/{rest}"), "$RUN");
    }
    output.replace(home, "$HOME")
}

fn absolute_path_regex() -> &'static Regex {
    static REGEX: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    REGEX.get_or_init(|| {
        Regex::new(r#"(?:^|[\s"'=(:])(/(?:[A-Za-z0-9._@+-]+/)*[A-Za-z0-9._@+-]+)"#)
            .expect("valid path expression")
    })
}

fn walk_strings(value: &mut Value, map: &dyn Fn(&str) -> String) {
    match value {
        Value::String(s) => *s = map(s),
        Value::Array(items) => items.iter_mut().for_each(|item| walk_strings(item, map)),
        Value::Object(items) => items.values_mut().for_each(|item| walk_strings(item, map)),
        _ => {}
    }
}

/// Render a scorecard from all committed run folders in stable run-id order.
pub fn render_scorecard(reports_dir: &Path) -> Result<String, String> {
    let mut runs = Vec::new();
    let entries =
        fs::read_dir(reports_dir).map_err(|e| format!("read {}: {e}", reports_dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        if !entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            continue;
        }
        let id = entry.file_name().to_string_lossy().into_owned();
        let tools: Value = serde_json::from_slice(
            &fs::read(entry.path().join("tools.json"))
                .map_err(|e| format!("{id}/tools.json: {e}"))?,
        )
        .map_err(|e| e.to_string())?;
        let mut agents = Vec::new();
        for agent in ["codex", "claude"] {
            let file = entry.path().join(format!("{agent}.json"));
            if !file.exists() {
                continue;
            }
            let report: Value =
                serde_json::from_slice(&fs::read(&file).map_err(|e| e.to_string())?)
                    .map_err(|e| format!("{}: {e}", file.display()))?;
            if report["reportVersion"] != "cli-agent-evals.report/v1" {
                return Err(format!("{} is not report/v1", file.display()));
            }
            agents.push((agent.to_owned(), report));
        }
        runs.push((id, tools, agents));
    }
    runs.sort_by(|a, b| a.0.cmp(&b.0));
    let mut out = String::from(
        "# Agent evaluation scorecard\n\n## Runs\n\n| Run | fsl | quoin | quire_cli | cli_agent_evals | docs_commit | Codex model | Claude model |\n|---|---|---|---|---|---|---|---|\n",
    );
    for (id, tools, agents) in &runs {
        let model = |a: &str| {
            agents
                .iter()
                .find(|(name, _)| name == a)
                .map(|(_, r)| r["model"].as_str().unwrap_or("n/a"))
                .unwrap_or("n/a")
        };
        out.push_str(&format!(
            "| {id} | {} | {} | {} | {} | {} | {} | {} |\n",
            cell(&tools["fsl"]),
            cell(&tools["quoin"]),
            cell(&tools["quire_cli"]),
            cell(&tools["cli_agent_evals"]),
            cell(&tools["docs_commit"]),
            model("codex"),
            model("claude")
        ));
    }
    let mut scenarios = std::collections::BTreeSet::new();
    for (_, _, agents) in &runs {
        for (_, report) in agents {
            if let Some(rs) = report["results"].as_array() {
                for r in rs {
                    if let Some(id) = r["id"].as_str() {
                        scenarios.insert(id.to_owned());
                    }
                }
            }
        }
    }
    out.push_str("\n## Latest run\n\n| Scenario | Agent | Cell | Trend | Median tool calls | Median wall seconds | Failed classes |\n|---|---|---|---|---:|---:|---|\n");
    for scenario in &scenarios {
        for agent in ["codex", "claude"] {
            let found = runs.iter().rev().find_map(|(id, _, agents)| {
                agents.iter().find(|(a, _)| a == agent).and_then(|(_, r)| {
                    r["results"]
                        .as_array()?
                        .iter()
                        .find(|x| x["id"].as_str() == Some(scenario.as_str()))
                        .map(|x| (id, x))
                })
            });
            if let Some((latest_id, r)) = found {
                let runs_n = r["runs"].as_array().cloned().unwrap_or_default();
                let pass = runs_n.iter().filter(|s| s["ok"] == true).count();
                let n = runs_n.len();
                let latest_model = runs
                    .iter()
                    .find(|(id, _, _)| id == latest_id)
                    .and_then(|(_, _, a)| a.iter().find(|(name, _)| name == agent))
                    .map(|(_, report)| report["model"].as_str().unwrap_or(""));
                let prior_pass = runs
                    .iter()
                    .filter(|(id, _, _)| id < latest_id)
                    .rev()
                    .find_map(|(_, _, prior_agents)| {
                        let report = prior_agents
                            .iter()
                            .find(|(name, report)| {
                                name == agent && report_model(report) == latest_model.unwrap_or("")
                            })
                            .map(|(_, report)| report)?;
                        let result = report["results"]
                            .as_array()?
                            .iter()
                            .find(|row| row["id"].as_str() == Some(scenario.as_str()))?;
                        let samples = result["runs"].as_array()?;
                        Some(samples.iter().filter(|sample| sample["ok"] == true).count())
                    });
                let trend = match prior_pass {
                    None => "new",
                    Some(previous) if pass > previous => "up",
                    Some(previous) if pass < previous => "down",
                    Some(_) => "same",
                };
                let cell = if n > 0 && pass == n {
                    "pass"
                } else if pass > 0 {
                    "unstable"
                } else {
                    "fail"
                };
                let calls = r["aggregate"]["toolCalls"]["p50"]
                    .as_f64()
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "n/a".into());
                let seconds = r["aggregate"]["latencyMs"]["p50"]
                    .as_f64()
                    .map(|v| (v / 1000.0).to_string())
                    .unwrap_or_else(|| "n/a".into());
                let mut classes = std::collections::BTreeMap::<String, usize>::new();
                for failure in runs_n
                    .iter()
                    .filter(|sample| sample["ok"] == false)
                    .flat_map(|sample| sample["failures"].as_array().into_iter().flatten())
                {
                    let text = failure.as_str().unwrap_or("");
                    let class = text
                        .split_once(':')
                        .map(|(class, _)| class)
                        .unwrap_or("unclassified");
                    *classes.entry(class.to_owned()).or_default() += 1;
                }
                let classes = classes
                    .iter()
                    .map(|(class, count)| format!("{class}: {count}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                out.push_str(&format!("| {scenario} | {agent} | {cell} {pass}/{n} | {trend} | {calls} | {seconds} | {classes} |\n"));
            }
        }
    }
    out.push_str("\n## History\n\n| Scenario | Agent | Runs, oldest first |\n|---|---|---|\n");
    for scenario in scenarios {
        for agent in ["codex", "claude"] {
            let mut values = Vec::new();
            for (_, _, agents) in &runs {
                let value = agents
                    .iter()
                    .find(|(a, _)| a == agent)
                    .and_then(|(_, r)| {
                        r["results"]
                            .as_array()?
                            .iter()
                            .find(|x| x["id"].as_str() == Some(scenario.as_str()))
                    })
                    .map(|r| {
                        let samples = r["runs"].as_array().cloned().unwrap_or_default();
                        let pass = samples.iter().filter(|s| s["ok"] == true).count();
                        format!("{pass}/{}", samples.len())
                    })
                    .unwrap_or_else(|| "-".into());
                values.push(value);
            }
            out.push_str(&format!(
                "| {scenario} | {agent} | {} |\n",
                values.join(" ")
            ));
        }
    }
    out.push_str("\n## Inactive scenarios\n\n| Scenario | Reason |\n|---|---|\n");
    for (_, tools, _) in &runs {
        if let Some(inactive) = tools["inactive"].as_object() {
            for (id, reason) in inactive {
                out.push_str(&format!("| {id} | {} |\n", cell(reason)));
            }
        }
    }
    Ok(out)
}

fn cell(value: &Value) -> &str {
    value.as_str().unwrap_or("n/a")
}

fn report_model(report: &Value) -> &str {
    report["model"].as_str().unwrap_or("")
}

/// Write or compare the generated scorecard.
pub fn scorecard(reports_dir: &Path, output: &Path, check: bool) -> Result<(), String> {
    let expected = render_scorecard(reports_dir)?;
    if check {
        let actual = fs::read_to_string(output).unwrap_or_default();
        if actual != expected {
            return Err(format!(
                "{} differs from generated scorecard",
                output.display()
            ));
        }
    } else {
        fs::write(output, expected).map_err(|e| e.to_string())?;
    }
    Ok(())
}
