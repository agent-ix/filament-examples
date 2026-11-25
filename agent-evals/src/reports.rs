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
        .replace_all(&triage, |captures: &regex::Captures<'_>| {
            format!("{}<path>", &captures[1])
        })
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
    let home = home.to_string_lossy().into_owned();
    walk_strings(&mut report, &|s| redact_base_paths(s, run_root, &home));
    if let Some(results) = report["results"].as_array_mut() {
        for result in results {
            if let Some(runs) = result["runs"].as_array_mut() {
                for sample in runs {
                    for key in ["workDir", "transcriptPath"] {
                        if let Some(value) = sample[key].as_str() {
                            if !value.starts_with("$RUN/") {
                                return Err(format!("{key} must be run-relative after redaction"));
                            }
                        }
                    }
                    for key in ["failures", "checks"] {
                        if !sample.get(key).is_some() {
                            continue;
                        }
                        walk_strings(&mut sample[key], &|s| {
                            absolute_path_regex()
                                .replace_all(s, |captures: &regex::Captures<'_>| {
                                    format!("{}<path>", &captures[1])
                                })
                                .into_owned()
                        });
                    }
                }
            }
        }
    }
    validate_report(&report)?;
    Ok(report)
}

fn validate_report(report: &Value) -> Result<(), String> {
    fn object(value: &Value, allowed: &[&str], required: &[&str], at: &str) -> Result<(), String> {
        let map = value
            .as_object()
            .ok_or_else(|| format!("{at} must be an object"))?;
        if let Some(key) = map.keys().find(|key| !allowed.contains(&key.as_str())) {
            return Err(format!("{at} has unknown member {key}"));
        }
        if let Some(key) = required.iter().find(|key| !map.contains_key(**key)) {
            return Err(format!("{at} is missing {key}"));
        }
        Ok(())
    }
    object(
        report,
        &[
            "reportVersion",
            "ok",
            "generatedAt",
            "suite",
            "agent",
            "model",
            "repeats",
            "results",
            "aggregates",
        ],
        &[
            "reportVersion",
            "ok",
            "generatedAt",
            "suite",
            "repeats",
            "results",
            "aggregates",
        ],
        "report",
    )?;
    if !report["ok"].is_boolean()
        || !report["generatedAt"].is_string()
        || !report["suite"].is_string()
        || report.get("agent").is_some_and(|value| !value.is_string())
        || report.get("model").is_some_and(|value| !value.is_string())
        || report["repeats"].as_u64().is_none()
        || report["repeats"].as_u64() == Some(0)
        || !report["results"].is_array()
        || !report["aggregates"].is_object()
    {
        return Err("report/v1 has malformed repeats, results, or aggregates".into());
    }
    for (i, result) in report["results"].as_array().unwrap().iter().enumerate() {
        let at = format!("results[{i}]");
        object(
            result,
            &["id", "useCase", "ok", "passRate", "aggregate", "runs"],
            &["id", "ok", "passRate", "aggregate", "runs"],
            &at,
        )?;
        if !result["id"].is_string()
            || !result["ok"].is_boolean()
            || !result["passRate"].is_string()
            || !result["runs"].is_array()
            || !result["aggregate"].is_object()
        {
            return Err(format!("{at} has malformed runs or aggregate"));
        }
        for (j, sample) in result["runs"].as_array().unwrap().iter().enumerate() {
            let at = format!("{at}.runs[{j}]");
            object(
                sample,
                &[
                    "ok",
                    "latencyMs",
                    "exitReason",
                    "metricStatus",
                    "tokenUsage",
                    "toolCalls",
                    "toolBreakdown",
                    "classified",
                    "checks",
                    "failures",
                    "workDir",
                    "sessionId",
                    "transcriptDigest",
                    "transcriptRetention",
                    "transcriptPath",
                ],
                &[
                    "ok",
                    "latencyMs",
                    "exitReason",
                    "metricStatus",
                    "tokenUsage",
                    "toolCalls",
                    "toolBreakdown",
                    "classified",
                    "checks",
                    "failures",
                    "workDir",
                    "sessionId",
                    "transcriptDigest",
                    "transcriptRetention",
                ],
                &at,
            )?;
            if !sample["tokenUsage"].is_object()
                || !sample["failures"].is_array()
                || !sample["checks"].is_object()
                || !sample["workDir"].is_string()
            {
                return Err(format!("{at} has malformed required fields"));
            }
            object(
                &sample["tokenUsage"],
                &[
                    "input",
                    "output",
                    "cacheCreation",
                    "cacheRead",
                    "contextInput",
                    "total",
                ],
                &[
                    "input",
                    "output",
                    "cacheCreation",
                    "cacheRead",
                    "contextInput",
                    "total",
                ],
                &format!("{at}.tokenUsage"),
            )?;
            if !sample["ok"].is_boolean()
                || !sample["latencyMs"].is_number()
                || !matches!(
                    sample["exitReason"].as_str(),
                    Some("complete" | "failed" | "timeout" | "exit" | "error")
                )
                || !matches!(
                    sample["metricStatus"].as_str(),
                    Some("available" | "unavailable")
                )
                || !matches!(
                    sample["transcriptRetention"].as_str(),
                    Some("retained" | "not-retained" | "unavailable")
                )
                || (!sample["toolCalls"].is_null() && !sample["toolCalls"].is_number())
                || !sample["failures"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .all(Value::is_string)
                || !sample["workDir"].is_string()
                || !sample["sessionId"].is_string()
            {
                return Err(format!("{at} has invalid field values"));
            }
            if sample["tokenUsage"]
                .as_object()
                .unwrap()
                .values()
                .any(|value| !value.is_number())
            {
                return Err(format!("{at}.tokenUsage values must be numbers"));
            }
            for key in ["toolBreakdown", "classified"] {
                let values = sample[key]
                    .as_object()
                    .ok_or_else(|| format!("{at}.{key} must be an object"))?;
                if values.values().any(|value| !value.is_number()) {
                    return Err(format!("{at}.{key} values must be numbers"));
                }
            }
            if let Some(path) = sample.get("transcriptPath") {
                let path = path
                    .as_str()
                    .ok_or_else(|| format!("{at}.transcriptPath must be a string"))?;
                if path.starts_with('/')
                    || path.as_bytes().get(1) == Some(&b':')
                    || path.contains('\\')
                    || path.bytes().any(|byte| byte < 0x20 || byte == 0x7f)
                    || path
                        .split('/')
                        .any(|segment| segment.is_empty() || segment == "." || segment == "..")
                {
                    return Err(format!("{at}.transcriptPath must be a safe relative path"));
                }
            }
            let digest = &sample["transcriptDigest"];
            if !digest.is_null()
                && (!digest.is_string()
                    || digest.as_str().is_none_or(|s| {
                        s.len() != 64
                            || !s
                                .bytes()
                                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
                    }))
            {
                return Err(format!("{at}.transcriptDigest is invalid"));
            }
            let retained = sample["transcriptRetention"].as_str().unwrap();
            let has_digest = digest.is_string();
            let has_path = sample.get("transcriptPath").is_some();
            if !matches!(
                (retained, has_digest, has_path),
                ("retained", true, true)
                    | ("not-retained", true, false)
                    | ("unavailable", false, false)
            ) {
                return Err(format!("{at} transcript retention fields contradict"));
            }
        }
        if let Some(use_case) = result.get("useCase") {
            if !use_case.is_string() {
                return Err(format!("{at}.useCase must be a string"));
            }
        }
        for (_, metric) in result["aggregate"].as_object().unwrap() {
            object(
                metric,
                &["p50", "p95"],
                &["p50", "p95"],
                &format!("{at}.aggregate metric"),
            )?;
            if ["p50", "p95"]
                .iter()
                .any(|key| !metric[*key].is_null() && !metric[*key].is_number())
            {
                return Err(format!("{at}.aggregate values must be numbers or null"));
            }
        }
    }
    Ok(())
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
        Regex::new(r#"(^|[\s"'=(:])(/(?:[A-Za-z0-9._@+-]+/)*[A-Za-z0-9._@+-]+)"#)
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
            validate_report(&report).map_err(|error| format!("{}: {error}", file.display()))?;
            if report["repeats"].as_u64() != Some(3)
                || report["results"].as_array().is_some_and(|rows| {
                    rows.iter().any(|row| {
                        row["runs"]
                            .as_array()
                            .is_none_or(|samples| samples.len() != 3)
                    })
                })
            {
                return Err(format!(
                    "{} must contain exactly three samples per scenario",
                    file.display()
                ));
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
                let classes = triage_classes(
                    &reports_dir.join(latest_id).join("triage.md"),
                    scenario,
                    agent,
                );
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

fn triage_classes(
    file: &Path,
    scenario: &str,
    agent: &str,
) -> std::collections::BTreeMap<String, usize> {
    let mut classes = std::collections::BTreeMap::new();
    let Ok(text) = fs::read_to_string(file) else {
        return classes;
    };
    for line in text.lines().filter(|line| line.starts_with('|')) {
        let fields: Vec<_> = line.trim_matches('|').split('|').map(str::trim).collect();
        if fields.len() < 7 || fields[0] != scenario || fields[1] != agent {
            continue;
        }
        let class = fields[4];
        if class != "" && class != "class" && class != "---" {
            *classes.entry(class.to_owned()).or_default() += 1;
        }
    }
    classes
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
        if !scorecard_matches_expected(&expected, output)? {
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

/// Return whether the existing scorecard matches the reports directory.
pub fn scorecard_matches(reports_dir: &Path, output: &Path) -> Result<bool, String> {
    let expected = render_scorecard(reports_dir)?;
    scorecard_matches_expected(&expected, output)
}

fn scorecard_matches_expected(expected: &str, output: &Path) -> Result<bool, String> {
    let actual = fs::read_to_string(output).unwrap_or_default();
    Ok(actual == expected)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;
    use serde_json::json;
    use tempfile::tempdir;

    const DIGEST: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    fn report(
        model: &str,
        samples: &[bool],
        tool_calls: Option<u64>,
        work_dir: &str,
        transcript_path: Option<&str>,
        failure: &str,
    ) -> Value {
        let results = vec![result_row(
            "EV-001",
            samples,
            tool_calls,
            work_dir,
            failure,
            transcript_path,
        )];
        json!({
            "reportVersion": "cli-agent-evals.report/v1",
            "ok": samples.iter().all(|sample| *sample),
            "generatedAt": "2026-01-12T00:00:00Z",
            "suite": "agent-evals",
            "agent": "codex",
            "model": model,
            "repeats": samples.len(),
            "results": results,
            "aggregates": {},
        })
    }

    fn result_row(
        id: &str,
        samples: &[bool],
        tool_calls: Option<u64>,
        work_dir: &str,
        failure: &str,
        transcript_path: Option<&str>,
    ) -> Value {
        let runs = samples
            .iter()
            .map(|ok| {
                let mut sample = json!({
                    "ok": ok,
                    "latencyMs": 1000,
                    "exitReason": if *ok { "complete" } else { "failed" },
                    "metricStatus": "available",
                    "tokenUsage": { "input": 1, "output": 2, "cacheCreation": 0, "cacheRead": 0, "contextInput": 1, "total": 3 },
                    "toolCalls": tool_calls,
                    "toolBreakdown": {},
                    "classified": {},
                    "checks": { "detail": failure },
                    "failures": [failure],
                    "workDir": work_dir,
                    "sessionId": "session-1",
                    "transcriptDigest": Value::String(DIGEST.into()),
                    "transcriptRetention": if transcript_path.is_some() { "retained" } else { "not-retained" },
                });
                if let Some(path) = transcript_path {
                    sample["transcriptPath"] = json!(path);
                }
                sample
            })
            .collect::<Vec<_>>();
        let passes = samples.iter().filter(|sample| **sample).count();
        json!({
            "id": id,
            "ok": passes == samples.len(),
            "passRate": format!("{passes}/{}", samples.len()),
            "aggregate": {
                "toolCalls": { "p50": tool_calls, "p95": tool_calls },
                "latencyMs": { "p50": 1000, "p95": 1000 },
            },
            "runs": runs,
        })
    }

    fn write_report(path: &Path, value: &Value) {
        fs::write(path, serde_json::to_vec(value).expect("serialize report"))
            .expect("write report");
    }

    fn scorecard_report(
        model: &str,
        ev000: &[bool],
        ev001: &[bool],
        tool_calls: Option<u64>,
    ) -> Value {
        let results = vec![
            result_row("EV-000", ev000, tool_calls, "$RUN/workspace", "", None),
            result_row("EV-001", ev001, tool_calls, "$RUN/workspace", "", None),
        ];
        json!({
            "reportVersion": "cli-agent-evals.report/v1",
            "ok": ev000.iter().all(|sample| *sample) && ev001.iter().all(|sample| *sample),
            "generatedAt": "2026-01-12T00:00:00Z",
            "suite": "agent-evals",
            "agent": "codex",
            "model": model,
            "repeats": 3,
            "results": results,
            "aggregates": {},
        })
    }

    fn write_scorecard_run(
        reports: &Path,
        run_id: &str,
        codex_model: &str,
        codex_samples: &[bool],
        claude: Option<(&str, &[bool])>,
        tool_calls: Option<u64>,
    ) {
        let run = reports.join(run_id);
        fs::create_dir_all(&run).expect("create run directory");
        fs::write(
            run.join("tools.json"),
            serde_json::to_vec(&json!({
                "fsl": "fsl-version",
                "quoin": "quoin-version",
                "quire_cli": "quire-version",
                "cli_agent_evals": "evals-version",
                "docs_commit": "docs-sha",
            }))
            .expect("serialize tool versions"),
        )
        .expect("write tool versions");
        let codex_ev000 = [true, true, true];
        write_report(
            &run.join("codex.json"),
            &scorecard_report(codex_model, &codex_ev000, codex_samples, tool_calls),
        );
        if let Some((model, samples)) = claude {
            write_report(
                &run.join("claude.json"),
                &scorecard_report(model, samples, &[true, true, true], Some(4)),
            );
        }
        fs::write(
            run.join("triage.md"),
            "| EV-001 | codex | 1 | failed | doc-gap | fsl | QSL-1234 |\n",
        )
        .expect("write triage");
    }

    #[trace("FR-002-AC-3", "TC-002")]
    #[test]
    fn run_folder_redacts_paths_and_rejects_unknown_report_members_before_retaining_output() {
        let temp = tempdir().expect("temporary directory");
        let reports = temp.path().join("reports");
        fs::create_dir_all(&reports).expect("reports directory");
        let input = temp.path().join("input");
        fs::create_dir_all(&input).expect("input directory");
        let run_root = Path::new("/private/var/folders/test/run-root");
        let home = Path::new("/Users/test-home");
        let tools = input.join("tools.json");
        let triage = input.join("triage.md");
        let codex = input.join("codex.json");
        let claude = input.join("claude.json");
        fs::write(&tools, r#"{"fsl":"version","docs_commit":"sha"}"#).expect("write tools");
        fs::write(
            &triage,
            format!(
                "run {0}; alias /var/folders/test/run-root; home {1}; other /opt/x/y\n",
                run_root.display(),
                home.display()
            ),
        )
        .expect("write triage");
        let input_report = report(
            "model-A",
            &[true],
            Some(2),
            &format!("{}/workspace", run_root.display()),
            Some(&format!("{}/transcripts/sample.json", run_root.display())),
            &format!("{} /opt/x/y", home.display()),
        );
        write_report(&codex, &input_report);
        write_report(&claude, &input_report);

        let folder = write_run_folder(
            &reports,
            "2026-01-12",
            run_root,
            home,
            &tools,
            &triage,
            &codex,
            &claude,
        )
        .expect("write redacted run");
        let saved: Value = serde_json::from_slice(
            &fs::read(folder.join("codex.json")).expect("read committed report"),
        )
        .expect("decode committed report");
        let sample = &saved["results"][0]["runs"][0];
        assert_eq!(sample["workDir"], "$RUN/workspace");
        assert_eq!(sample["transcriptPath"], "$RUN/transcripts/sample.json");
        assert_eq!(sample["failures"][0], "$HOME <path>");
        assert_eq!(sample["checks"]["detail"], "$HOME <path>");
        let saved_triage = fs::read_to_string(folder.join("triage.md")).expect("read triage");
        assert_eq!(
            saved_triage,
            "run $RUN; alias $RUN; home $HOME; other <path>\n"
        );
        assert!(!saved_triage.contains("/private/var/folders"));
        assert!(!saved_triage.contains("/var/folders"));
        assert!(!saved_triage.contains(home.to_str().expect("home path")));

        let mut unknown = input_report;
        unknown["unexpected"] = json!(true);
        write_report(&codex, &unknown);
        let refused = write_run_folder(
            &reports,
            "2026-01-13",
            run_root,
            home,
            &tools,
            &triage,
            &codex,
            &claude,
        );
        assert!(refused.is_err());
        assert!(!reports.join("2026-01-13").exists());
    }

    #[trace("FR-002-AC-3", "TC-002")]
    #[test]
    fn redacted_report_rejects_transcript_paths_outside_run_root() {
        let input = report(
            "model-A",
            &[true],
            Some(2),
            "$RUN/workspace",
            Some("transcripts/sample.json"),
            "",
        );
        assert!(redact_report(input, Path::new("/tmp/run"), Path::new("/home/test")).is_err());
    }

    #[trace("FR-002-AC-4", "TC-002")]
    #[test]
    fn scorecard_is_stable_and_reports_latest_trends_history_and_unavailable_metrics() {
        let temp = tempdir().expect("temporary directory");
        let reports = temp.path().join("reports");
        fs::create_dir_all(&reports).expect("reports directory");
        write_scorecard_run(
            &reports,
            "2026-01-05",
            "model-A",
            &[true, false, false],
            Some(("claude-model-A", &[true, true, false])),
            Some(7),
        );
        write_scorecard_run(
            &reports,
            "2026-01-12",
            "model-A",
            &[true, false, true],
            Some(("claude-model-B", &[true, true, true])),
            None,
        );
        let output = temp.path().join("SCORECARD.md");
        scorecard(&reports, &output, false).expect("write scorecard");
        let first = fs::read(&output).expect("read first scorecard");
        scorecard(&reports, &output, false).expect("rewrite scorecard");
        let second = fs::read(&output).expect("read rewritten scorecard");
        assert_eq!(first, second);
        let text = String::from_utf8(first).expect("scorecard UTF-8");
        assert!(text.contains("| 2026-01-05 |"));
        assert!(text.contains("| 2026-01-12 |"));
        assert!(text.contains("EV-000 | codex | pass 3/3 | same"));
        assert!(text.contains("EV-001 | codex | unstable 2/3 | up"));
        assert!(text.contains("EV-000 | claude | pass 3/3 | new"));
        assert!(text.contains("| EV-001 | codex | 1/3 2/3 |"));
        assert!(text.contains("| EV-001 | codex | unstable 2/3 | up | n/a | 1 | doc-gap: 1 |"));
    }

    #[trace("FR-002-AC-5", "TC-002")]
    #[test]
    fn scorecard_check_detects_changed_output_and_new_run_folders() {
        let temp = tempdir().expect("temporary directory");
        let reports = temp.path().join("reports");
        fs::create_dir_all(&reports).expect("reports directory");
        write_scorecard_run(
            &reports,
            "2026-01-12",
            "model-A",
            &[true, true, true],
            Some(("claude-model-B", &[true, true, true])),
            Some(4),
        );
        let output = temp.path().join("SCORECARD.md");
        scorecard(&reports, &output, false).expect("write scorecard");
        assert!(scorecard(&reports, &output, true).is_ok());
        fs::write(&output, "changed").expect("alter scorecard");
        assert!(scorecard(&reports, &output, true).is_err());
        scorecard(&reports, &output, false).expect("restore scorecard");
        write_scorecard_run(
            &reports,
            "2026-01-19",
            "model-A",
            &[true, true, true],
            None,
            Some(4),
        );
        assert!(scorecard(&reports, &output, true).is_err());
    }
}
