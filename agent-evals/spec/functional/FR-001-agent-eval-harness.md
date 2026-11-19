---
id: FR-001
title: "Agent eval harness: a cli-evals suite in a sandbox of the shipped FSL tools and docs"
type: FR
---

# FR-001: Agent eval harness

## Description

The agent eval harness measures whether a coding agent can do a real task with
only what the Filament project ships: the `fsl` driver, the `quoin` and `quire`
command-line tools, and the documentation. It runs fixed scenarios with live
agents (Codex and Claude Code) and records each sample's commands, turns, final
files and assertion results.

The harness is built on the cli-evals runner from the npm package
`@agent-ix/cli-agent-evals`. The runner owns scenario selection, the scenario
workspace, the agent process, transcript capture, metrics and the
`cli-agent-evals.report/v1` report. Everything specific to FSL (fixtures,
prompts, assertions, tool installation, scheduling and the scorecard) belongs
to one Rust program in this folder, `agent-evals`, which talks to the runner
through the runner's external scenario provider protocol
(`cli-agent-evals.external-provider-request/v1` and `-result/v1`).

### Decision: one Rust program, no logic in JavaScript, Python or shell

`agent-evals/cli-agent-evals.config.mjs` holds only data: the suite name
`fsl-agent-evals`, `rootDir`, `reportsDir` and a `provider` entry whose
`command` is `agent-evals`, whose `args` are `["provider"]` and whose
`timeoutMs` is `300000`. It declares no scenarios, setup, assertion, workspace
factory or environment function. The provider program is the Cargo package
`agent-evals` in `agent-evals/`, a standalone package like
`tools/fsl-examples`, with one binary `agent-evals` and these subcommands:

| Subcommand | Does |
| --- | --- |
| `provider` | Answers one provider request on standard input with one result on standard output (`describe`, `prepare`, `assert`). |
| `tools` | Installs the released tools and the docs snapshot into a run root and writes `tools.json`. |
| `run` | Runs `tools`, then the suite once per agent, then writes the committed run folder (FR-002). |
| `clean --run <run id>` | Deletes the run root and the run's kept transcripts (FR-002). |
| `scorecard`, `triage`, `scheduled`, `schedule install` | FR-002. |

The Makefile target `agent-evals` calls `agent-evals run` and nothing else. No
Python, no JavaScript assertion or setup code and no logic-bearing shell exists
in the harness. Scenario logic in Rust keeps it testable with `cargo test` and
reviewable with the rest of the FSL toolchain.

### Decision: how the runner finds the provider and reads its exit

The runner starts the provider by the bare name `agent-evals`, resolved on its
`PATH`. `run` copies its own executable to `<root>/bin/agent-evals` and starts
`cli-evals` with `<root>/bin` first on the runner's `PATH`; `<root>/bin` is
never on the agent's `PATH`. `cli-evals run` exits 1 whenever a scenario has a
failed sample, which is a normal outcome for an eval: `run` treats exit 0 or 1
with a written report as a completed agent run, and any other exit, or no
report, as a `harness-error` for that agent.

### Decision: provider time budget

The runner gives the provider one suite-wide timeout, `timeoutMs` in the
config, and reports a timeout only as "provider assert failed". Each scenario
therefore declares its own `assert` budget in its fixture manifest
(`fixtures/<scenario id>/scenario.toml`, `assert_budget_ms`; EV-000 30 000,
EV-005, EV-105 and EV-205 240 000), always below the config's `timeoutMs`.
The provider stops its own child commands (`fsl`, `quoin`, `quire`) when the
budget runs out and returns a failed assertion whose failure starts
`harness-error: assert budget exceeded`, so a slow tool is classified and never
read as an agent failure.

### Decision: released tools only

A run uses the tools a user can install on the day of the run, never a git
build or a source checkout:

- `fsl`: the latest release of `agent-ix/filament-driver`, the archive
  `fsl-<tag>-<host triple>.tar.gz`, checked against the release's
  `SHA256SUMS.txt` before it is unpacked.
- `quoin`, `quire` and `cli-evals`: `npm install --prefix <root>/tools/npm
  --registry https://registry.npmjs.org @agent-ix/quoin@latest
  @agent-ix/quire-cli@latest @agent-ix/cli-agent-evals@latest`.
- Spec modules: `quoin module ensure-defaults` with `IX_HOME=<root>/ix`.
- Documentation: the `src/content/docs` tree of `agent-ix/filament-docs` at the
  head of `main`, fetched as a tarball through `gh api`, into `<root>/docs`.

`tools` writes `<root>/tools.json` with `format`
`agent-evals.tools/1` and the members `fsl` (release tag), `quoin`,
`quire_cli` and `cli_agent_evals` (the versions each tool prints for
`--version`), `docs_commit` (the full commit id of the docs snapshot), `host` (the target
triple) and `inactive`, a map from a scenario id to the refusal code that keeps
that scenario out of the catalogue because the released tools cannot do it yet
([FR-003](./FR-003-fsl-claim-evidence-link-scenario.md) EV-205). These are provenance of the run, read by the
scorecard; nothing compares them with a stored value.

### Decision: the sandbox

The run root is a new folder `agent-evals-<run id>` under the operating
system's temporary folder, outside every source checkout. Its layout:

| Path | Holds | Readable by the agent |
| --- | --- | --- |
| `<root>/bin` | The copied `agent-evals` executable | No |
| `<root>/tools` | `fsl` and the npm prefix | Yes (on `PATH`) |
| `<root>/docs`, `<root>/ix` | The docs snapshot and spec modules, copied into each workspace | Through the copy only |
| `<root>/home` | The per-run home folder of the runner and the agents | No |
| `<root>/work` | The scenario workspaces | Its own workspace only |

Scenario fixtures are compiled into the provider and written into a workspace
only by `prepare`. Reference solutions, the provider's test fixtures and the
harness source live in the `agent-evals` source tree (for a scheduled run, its
temporary worktree, FR-002), which is never inside the run root and never
copied into it; reading it is a breach.

`run` starts `cli-evals` with `TMPDIR` = `<root>/work`, so every workspace the
runner creates lies there, and with `HOME` = `<root>/home` and
`XDG_CONFIG_HOME`, `XDG_DATA_HOME`, `XDG_STATE_HOME` and `XDG_CACHE_HOME`
under it. The runner reads each agent's transcript from its home folder
(`.claude/projects`, `.codex/sessions`), and the agents inherit the runner's
environment, so runner and agents share the per-run home and the transcripts
land where the runner reads them. The agent therefore loads no operator
instructions, memory, skills or plugins.

Credentials: Claude Code authenticates through `CLAUDE_CODE_OAUTH_TOKEN`,
which `run` reads from the operator's environment (created once with
`claude setup-token`); `<root>/home/.claude` is seeded only with the settings
that suppress first-run prompts. For Codex, `run` copies the operator's
`~/.codex/auth.json` to `<root>/home/.codex/auth.json` and writes a
`config.toml` naming the model there; the file is never copied into a
workspace, so it is never kept with `--keep`. `run` deletes it when the run
ends, on success or failure. A refresh token Codex rotates during the run is
not copied back to the operator's file. When a credential is missing, `run`
refuses before any sample runs and names the missing credential.

For each sample, `prepare`:

- writes the scenario's fixture files into the workspace;
- copies `<root>/docs` to `docs/` and `<root>/ix` to `.ix/` in the workspace;
- returns the environment `PATH` = `<root>/tools/npm/bin`, the folder holding
  `fsl`, the folder of `node`, the folder of the agent command, `/usr/bin`,
  `/bin`, in that order; `IX_HOME` = `<workspace>/.ix`; and `TMPDIR` =
  `<workspace>/.tmp`;
- returns the prompt: a fixed preamble ("Use only the commands on PATH and the
  documentation under docs/. Work only inside this folder.") followed by the
  scenario's task text.

"Only the shipped tools and docs" means: no other Filament source, repository
or program. Standard system tools in `/usr/bin` and `/bin` are allowed.

### Decision: sandbox breach

`assert` reads the transcript snapshot the runner writes before it calls
`assert`, `<workspace>/.cli-agent-evals/transcripts/<digest>.transcript`, and
extracts the inputs of every tool call: for a Claude Code transcript the
`input` of each `tool_use` item, for a Codex transcript the `arguments` of each
`function_call` and the command of each `local_shell_call`. A sample fails with
class `sandbox-breach` when a tool-call input names:

- an absolute path outside the workspace, `<root>/tools`, `/usr`, `/bin`,
  `/sbin`, `/etc` and `/dev` (the run root's other folders, other temporary
  folders and the source tree are outside);
- a relative path whose `..` components, resolved against the workspace,
  leave the workspace;
- a path starting with `~`, `$HOME` or `${HOME}`;
- a URL under `github.com/agent-ix/`, `raw.githubusercontent.com/agent-ix/` or
  `api.github.com/repos/agent-ix/`, or the SSH remote `git@github.com:agent-ix/`;
  or
- a `gh` command naming an `agent-ix/` repository, such as
  `gh repo clone agent-ix/...` or `gh api repos/agent-ix/...`.

Only tool-call inputs are scanned, because the transcript also holds text the
agent did not act on. A transcript that is absent or in neither format fails
the sample with class `harness-error`, never as a pass.

### Decision: what is fixed instead of a seed

Agent CLIs take no seed. A run fixes instead: the fixture bytes and prompt
text, compiled into the provider from `agent-evals/fixtures/<scenario id>/`;
the model id per agent, passed to the runner with `--model` for both agents and
read from `agent-evals/agents.toml` (initially `codex` = `gpt-6.1-sol`,
`claude` = `claude-opus-5-5`, full model ids, never aliases); and the repeat
count, `repeats = 3` in the same file. A scenario that runs a seeded FSL engine
(a statistical claim) states the seed in its request fixture and in the prompt.
Changing a model id is a reviewed change to `agents.toml`; the scorecard starts
a new trend baseline for that agent (FR-002).

### Decision: pass rate

A sample passes when the runner reports it `ok`: the agent run completed and
the provider's assertion passed. For one run, one scenario and one agent, the
cell is `pass` at 3 of 3 samples, `unstable` at 1 or 2 of 3, and `fail` at 0
of 3. Every failed sample, in any cell, is triaged (FR-002). A scenario's own
acceptance asks that it runs for both agents and that every failure is
classified; the harness's acceptance asks that EV-000 is `pass` for both agents.

### Decision: scenario identifiers

The trivial scenario is `EV-000`. Each scenario ticket of the agent-journey
project takes `EV-<nnn>` in ticket order (the first scenario is `EV-001`). A
variant of scenario `EV-0nn` takes `EV-1nn`, a second variant `EV-2nn`.
`describe` returns every scenario with its `use_case` and `title`; `EV-000` is
the only canary.

### Decision: the trivial scenario EV-000

Fixture: `unit.fsl`, one unit with one clause that `fsl check` accepts. Task:
print the `fsl` version, run `fsl check unit.fsl`, and write `result.json` with
`fsl_version` (the printed version) and `check_exit` (the exit status).
`assert` passes when `result.json` parses, `fsl_version` equals `tools.json`'s
`fsl` tag without its leading `v`, `check_exit` equals the status the provider
gets when it runs the sandbox `fsl check` on the fixture itself, and the
sandbox-breach scan is clean.

## Inputs

- The runner's provider requests (`describe`, `prepare`, `assert`) on standard
  input, one per invocation.
- `agent-evals/agents.toml` (agents, model ids, repeats), the fixtures under
  `agent-evals/fixtures/`, and the run root named by `AGENT_EVALS_ROOT`, which
  `run` exports to the runner and so to the provider.
- The latest `fsl` release, the npm registry and the `filament-docs` head of
  `main`, read by `tools`.

## Outputs

- One provider result per request on standard output, exit status 0; a
  malformed or unknown request exits non-zero, which the runner records as a
  failed sample.
- `<root>/tools.json` and the run root's tools, docs and modules.
- One `cli-agent-evals.report/v1` report per agent per run (FR-002 commits it).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-001-AC-1 | `cli-agent-evals.config.mjs` loads with the runner's `loadSuite` and exports exactly the members `name`, `rootDir`, `reportsDir` and `provider`; `provider.command` is `agent-evals`, `provider.args` is `["provider"]` and `provider.timeoutMs` is `300000`. A config that adds `scenarios`, `workspace`, `assert` or `extraEnv` fails the check. The `agent-evals` folder holds no `.py` file and no shell script. | Test (TC-001) |
| FR-001-AC-2 | `agent-evals provider` answers a `describe` request with `EV-000` as the only canary and every active scenario under `fixtures/`; a request with protocol `cli-agent-evals.external-provider-request/v2`, an unknown operation, an unknown member or an undescribed scenario exits non-zero with no result on standard output. | Test (TC-001) |
| FR-001-AC-3 | `agent-evals tools` against a release whose archive for the host does not match `SHA256SUMS.txt`, or whose archive for the host is missing, exits non-zero, writes no `tools.json` and leaves no `fsl` in the run root. Against a matching release it writes `tools.json` with `format` `agent-evals.tools/1` and non-empty `fsl`, `quoin`, `quire_cli`, `cli_agent_evals`, `docs_commit` (40 or 64 hexadecimal characters) and `host`, and an `inactive` map. | Test (TC-001) |
| FR-001-AC-4 | `prepare` for EV-000 writes `unit.fsl`, `docs/` and `.ix/` into the workspace and no credential file; it returns a non-empty prompt starting with the fixed preamble, and an environment whose `PATH` lists `<root>/tools/npm/bin` and the `fsl` folder before any other folder, contains neither `<root>/bin` nor any folder under the operator's home other than the node and agent-command folders, and whose `IX_HOME` and `TMPDIR` lie inside the workspace. | Test (TC-001) |
| FR-001-AC-5 | `run` with `CLAUDE_CODE_OAUTH_TOKEN` unset, or with no Codex `auth.json`, exits non-zero before any sample and names the missing credential; no report is written. With both present, the stub runner `run` starts sees `HOME` = `<root>/home`, every `XDG_*_HOME` under it and `<root>/bin` first on `PATH`; `<root>/home/.codex/auth.json` exists while the stub runs, is absent after `run` ends (also when the stub exits 2), and the operator's `auth.json` is byte-identical before and after. | Test (TC-001) |
| FR-001-AC-6 | `assert` fails with a failure starting `sandbox-breach:` over transcript fixtures whose tool-call input: reads `$HOME/dev/filament-spec-language/README.md`; reads `~/dev/x`; reads `../../x` from the workspace; reads an absolute path in another workspace under `<root>/work` or in the source tree; runs `git clone git@github.com:agent-ix/filament-driver`; runs `gh repo clone agent-ix/filament-driver` or `gh api repos/agent-ix/filament-driver/contents`; or fetches `https://raw.githubusercontent.com/agent-ix/filament-driver/main/README.md` (one Claude Code and one Codex fixture each). It passes the scan for a fixture with all of these only in assistant text and for tool calls that read `docs/fsl/index.md`, `./unit.fsl` and `/usr/bin/env`. A missing transcript or an unknown format fails with a failure starting `harness-error:`. | Test (TC-001) |
| FR-001-AC-7 | EV-000 `assert` passes for a workspace whose `result.json` matches `tools.json` and the provider's own `fsl check` status, and fails when `fsl_version` differs, when `check_exit` differs, or when `result.json` is missing or is not JSON. | Test (TC-001) |
| FR-001-AC-8 | A live run of EV-000 with `--agent codex` and with `--agent claude`, each with three repeats and `--keep`, in the per-run home, produces two `cli-agent-evals.report/v1` reports whose `model` members equal `agents.toml`, whose `repeats` is 3, whose EV-000 samples each have `transcriptRetention` `retained`, and whose EV-000 `passRate` is `3/3`; both reports are committed under `agent-evals/reports/` (FR-002). | Demonstration (TC-001) |
| FR-001-AC-9 | The cell rule maps `3/3` to `pass`, `2/3` and `1/3` to `unstable` and `0/3` to `fail`; a report whose `repeats` is not 3 is refused by the scorecard with a message naming the report. | Test (TC-001) |
| FR-001-AC-10 | With a stub `fsl` that sleeps past EV-000's `assert_budget_ms`, `assert` returns within the budget plus 5 seconds with a failed assertion whose failure starts `harness-error: assert budget exceeded`, and every `fsl` it started has exited. | Test (TC-001) |
| FR-001-AC-11 | `run` with a stub `cli-evals` that writes a report and exits 1 records that agent's report as completed; a stub that exits 1 without a report, or exits 2, records `harness-error` for that agent and still runs the other agent. | Test (TC-001) |
| FR-001-AC-12 | `clean --run <run id>` deletes the run root and the run's kept-transcript folder (FR-002) and nothing else; for an unknown run id it exits non-zero and deletes nothing. | Test (TC-001) |

## Dependencies

- cli-agent-evals FR-005 (external scenario provider protocol), FR-002 (runner
  and report aggregation) and FR-004 (retained reports), as published in
  `@agent-ix/cli-agent-evals`.
- filament-driver releases with `SHA256SUMS.txt`.
- [FR-002](./FR-002-scheduled-runs-scorecard-and-triage.md) commits the
  reports and builds the scorecard.

## Constraints

| ID | Constraint | Type | Validation |
| --- | --- | --- | --- |
| FR-001-CON-1 | No Python, no JavaScript logic and no logic-bearing shell in the harness. | Implementation | Test (TC-001, AC-1) |
| FR-001-CON-2 | No git build, source checkout or globally installed copy of `fsl`, `quoin`, `quire` or `cli-evals` is used by a run. | Implementation | Test (TC-001, AC-3) |
| FR-001-CON-3 | No operator credential file is written into a scenario workspace. | Security | Test (TC-001, AC-4, AC-5) |
