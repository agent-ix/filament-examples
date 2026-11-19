---
id: SR-017
title: "spec-review of QSL-2484 agent eval harness (FR-001, TC-001)"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-examples; PR #55 at 87769de91cc8b526185cb8f6f8286b2619b610e2; agent-evals/spec/functional/FR-001-agent-eval-harness.md, agent-evals/spec/test-cases/TC-001-agent-eval-harness.md"
review_set: subset
---

## Summary

Ticket: QSL-2484. Spec review of FR-001 (the agent eval harness) and TC-001 in PR #55. The protocol and runner claims were checked against the source of `@agent-ix/cli-agent-evals` 0.3.0 (the npm-latest tarball, and `src/provider.ts`, `src/runner.ts`, `src/drivers.ts`, `src/transcript.ts` and `src/workspace.ts` at cli-agent-evals origin/main f2038ea, which is package version 0.3.0). The release claims were checked against filament-driver v0.1.4 and the `quoin` 0.36.7 and `quire` 0.52.0 releases (npm-latest is quoin 0.36.8 and quire-cli 0.52.0). `quire validate --scope . 'spec/**/*.md' --strict` from `agent-evals/` exits 0 with 6 of 6 documents grammar-clean.

## Examined scope

- FR-001 provider protocol: the request and result protocol names, the `describe`/`prepare`/`assert` operations, the scenario member `use_case`, the prepare result members `prompt` and `environment`, and the assertion members `ok`, `checks` and `failures` all match `src/provider.ts`. The provider inherits the runner's environment, so `AGENT_EVALS_ROOT` reaches it. (examined)
- FR-001 sandbox: Node's `os.tmpdir()` honours `TMPDIR`, so `defaultWorkspace` creates every workspace under `<root>/work`. The runner writes the transcript snapshot to `<workDir>/.cli-agent-evals/transcripts/<digest>.transcript` before it calls `assert`. Both claims hold. (examined)
- FR-001 isolated agent configuration against the driver transcript lookup: FND-001. (examined)
- FR-001 released tools: the archive name `fsl-<tag>-<triple>.tar.gz` and `SHA256SUMS.txt` match the filament-driver v0.1.4 release assets. (examined)
- FR-001 model ids: `claude-opus-5-5` is the current Claude Code model id. `gpt-6.1-sol` is the operator's configured Codex model and is in Codex's local model cache (a newer `gpt-6.2-sol` is also listed). Both are verifiable. No credential is committed. (examined)
- FR-001 pass rule, scenario ids and EV-000: these are consistent with the report's `passRate` string and `repeats`. (examined)
- Repository conventions: filament-examples has no CLAUDE.md or AGENTS.md. Other sub-projects already have `spec/` folders. `make check` (fsl-examples `discover_systems`) reads only `examples/`. `make test` globs do not reach `agent-evals/`. `target/` is gitignored. No root quire config exists. The PR adds no Python. A README entry can come with the implementation PR. (examined)
- FR-001-AC-1..9 and TC-001: each criterion can fail. The ids are unique in `agent-evals/spec`. (examined)

## Verdict

Not mergeable. FND-001 is high: cli-agent-evals 0.3.0 looks for Claude transcripts under `homedir()/.claude/projects` and for Codex rollouts under `homedir()/.codex/sessions` in the runner process. FR-001 moves both agents' state into the workspace (`CLAUDE_CONFIG_DIR`, `CODEX_HOME`), so the runner finds no transcript, every sample fails `harness-error` under FR-001's own rule, and FR-001-AC-8 can never pass. The provider protocol, TMPDIR and snapshot claims are otherwise accurate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Isolated CLAUDE_CONFIG_DIR/CODEX_HOME hide the transcript from cli-agent-evals 0.3.0, which reads homedir()/.claude/projects and homedir()/.codex/sessions; every sample becomes harness-error and AC-8 cannot pass | agent-evals/spec/functional/FR-001-agent-eval-harness.md:89 |
| FND-002 | medium | The provider call uses spawnSync with a default 15 s timeout; the config fixes the provider entry to command+args with no timeoutMs, while EV-000/EV-005 assert run fsl and several quoin/quire commands; a timeout surfaces as "provider assert failed", not harness-error | agent-evals/spec/functional/FR-001-agent-eval-harness.md:28 |
| FND-003 | medium | The breach scan misses relative escapes (../), $HOME/${HOME} paths (AC-6's own example), git@github.com:agent-ix/ and `gh repo clone/gh api repos/agent-ix` forms; it allows the run root, /tmp and /var/folders, which hold the FR-002 origin/main worktree with fixtures and reference solutions | agent-evals/spec/functional/FR-001-agent-eval-harness.md:108 |
| FND-004 | low | Runner integration is unstated: how `agent-evals` is on the runner's PATH (provider.command is a bare name resolved by spawnSync; under cargo run it is not), and that `cli-evals run` exits 1 whenever any scenario fails, which `run` must not treat as fatal | agent-evals/spec/functional/FR-001-agent-eval-harness.md:30 |
| FND-005 | low | The operator's Codex auth.json is copied into every workspace (and kept with --keep), so the agent can read or commit it. Codex rotates refresh tokens, so concurrent copies can invalidate the operator's login | agent-evals/spec/functional/FR-001-agent-eval-harness.md:90 |
| FND-006 | low | The Constraints sections of FR-001 and FR-002 are bullet lists; quire matrix warns section-holds-no-table for both | agent-evals/spec/functional/FR-001-agent-eval-harness.md:200 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | medium | The tightened breach scan now gives false sandbox-breach results: absolute workspace paths in the macOS /private/var/folders realpath form, absolute paths into the node and agent-command PATH folders (e.g. /opt/homebrew/bin), and `..` resolved against the workspace root instead of the call's cwd (`cd docs && cat ../unit.fsl`) all count as breaches, and AC-6 has no pass case for them | agent-evals/spec/functional/FR-001-agent-eval-harness.md:158 |
| FND-008 | low | `run` sets HOME/XDG for the runner but does not clear inherited CLAUDE_CONFIG_DIR, CODEX_HOME or CLAUDECODE; when set in the operator environment (an agent session, a profile) they override the per-run home and transcripts again land where the runner does not look, or nested claude refuses to start | agent-evals/spec/functional/FR-001-agent-eval-harness.md:114 |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
| FND-004 | fixed | - |
| FND-005 | fixed | - |
| FND-006 | fixed | - |
