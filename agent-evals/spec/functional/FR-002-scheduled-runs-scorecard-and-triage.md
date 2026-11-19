---
id: FR-002
title: "Scheduled agent eval runs, committed history, scorecard and triage"
type: FR
relationships:
  - target: ix://agent-ix/filament-examples/FR-001
    type: depends_on
---

# FR-002: Scheduled runs, scorecard and triage

## Description

The agent eval suite of [FR-001](./FR-001-agent-eval-harness.md) runs on a
weekly schedule, not per pull request. Each run's reports are committed to this
repository, a scorecard rebuilt from every committed run shows the trend, and
every failed sample gets a class and a Linear ticket before the run is merged.

### Decision: the schedule runs on the dev host

The live agents need the operator's credentials and a terminal multiplexer, and
CI in this organisation is started by hand only. The schedule therefore runs on
the dev host, as a per-user launchd agent on macOS or a user crontab entry on
Linux. `agent-evals schedule install --repo <checkout>` writes it:

- macOS: `~/Library/LaunchAgents/ai.agent-ix.agent-evals.plist`, label
  `ai.agent-ix.agent-evals`, `StartCalendarInterval` weekday 1, hour 3,
  minute 0 (Monday 03:00 local time), standard output and error to
  `~/Library/Logs/agent-evals.log`, and `ProgramArguments`
  `["/bin/zsh", "-lc", "exec cargo run --quiet --locked --manifest-path <checkout>/agent-evals/Cargo.toml -- scheduled --repo <checkout>"]`.
  The login shell gives the job the operator's `PATH` (cargo, node, git, gh and
  the agent commands) and `CLAUDE_CODE_OAUTH_TOKEN`.
- Linux: one crontab line `0 3 * * 1 /bin/bash -lc 'exec cargo run --quiet --locked --manifest-path <checkout>/agent-evals/Cargo.toml -- scheduled --repo <checkout>' >> ~/.local/state/agent-evals.log 2>&1`.

Installing again replaces the entry; it never adds a second one. A scheduled
routine in the cloud is not used: it has neither the operator's agent
credentials nor the terminal session the runner drives.

### Decision: a scheduled run never touches the operator's checkout

`agent-evals scheduled --repo <checkout>` fetches `origin`, adds a temporary
detached worktree of `origin/main` under the run root, and runs
`agent-evals run` from that worktree's own `agent-evals` package, so the run
uses the harness on `main`, whatever branch the checkout has. It then commits
the run folder and the rebuilt scorecard on a new branch
`agent-evals/run-<run id>`, pushes it, opens a pull request titled
`agent-evals: run <run id>` with `gh pr create`, and removes the worktree. The
checkout's branch, index and working tree are unchanged. A run whose `tools`
step refuses (FR-001) opens no pull request and logs the refusal.

### Decision: the history is committed run folders

Each run writes `agent-evals/reports/<run id>/`, where the run id is the UTC
date `YYYY-MM-DD` (a second run on one date takes `YYYY-MM-DD-2`):

| File | Holds |
| --- | --- |
| `codex.json`, `claude.json` | The runner's `cli-agent-evals.report/v1` report for that agent, redacted. |
| `tools.json` | The FR-001 tool and docs provenance of the run. |
| `triage.md` | One row per failed sample with its class and ticket. |

Redaction replaces the run root in every string with `$RUN` and the operator's
home folder with `$HOME`, because the repository admits no workstation paths.
The redacted report still decodes as a closed `cli-agent-evals.report/v1`
report. Transcripts and workspaces are not committed; they stay in the run root
until `agent-evals clean --run <run id>` removes them after the run's pull
request merges. The reports are the run's results, read by the scorecard; they
are not copies of a version or a digest kept for comparison.

The pull request is reviewed and merged like any other change. It can merge
only when `agent-evals triage --check` and `agent-evals scorecard --check` pass
for it.

### Decision: the scorecard

`agent-evals scorecard` reads every run folder under `agent-evals/reports/` in
run-id order and writes `agent-evals/SCORECARD.md`. The output is a function of
those files only: the same folders give the same bytes. It has:

- a runs table: run id, `fsl`, `quoin`, `quire_cli`, `cli_agent_evals`,
  `docs_commit` and the model of each agent;
- a latest-run table with one row per scenario and, per agent: the cell
  (`pass`, `unstable` or `fail`, FR-001) with its pass count such as `2/3`;
  the trend; the median tool calls (the report's `toolCalls` p50, `n/a` when
  the metric is unavailable); the median wall time in seconds (the report's
  `latencyMs` p50); and the classes of its failed samples with their counts;
- a history table with one row per scenario and agent and the pass count of
  every run, oldest first, `-` where the scenario did not run;
- the scenarios `tools.json` marks inactive, with the reason.

The trend of a cell compares its pass count with the most recent earlier run in
which the same agent ran the same scenario with the same model id: `up` when it
is higher, `down` when lower, `same` when equal, and `new` when there is no such
run. A model change therefore starts a new baseline.
`agent-evals scorecard --check` exits 1 when `SCORECARD.md` differs from what
the command would write.

### Decision: triage

`agent-evals run` writes `triage.md` with one row per failed sample:
scenario, agent, sample number, the first assertion failure, class, tool and
ticket. The provider tags the failures it can decide mechanically
(`sandbox-breach`, `harness-error`); every other row starts as
`unclassified`. The triager (the code lead's agent or a person) completes each
row with this checklist, in order:

1. The task needs a documented step and no page under `docs/` covers it, or
   the page is wrong: `doc-gap`.
2. A command behaves other than its documentation says, or lacks an option or
   output the task needs: `cli-gap`, with the tool (`fsl`, `quoin` or `quire`).
3. `fsl` gives a wrong verdict, crashes, or refuses valid input: `fsl-bug`.
4. The documentation and the tools were right and the agent went wrong:
   `agent-error`.

Every classified row is ticketed in Linear, never as a GitHub issue, with a
project, a milestone and a `related` relation to the scenario's ticket:
`fsl-bug` in the project that owns the violated FSL requirement; `cli-gap` for
`fsl` in "FSL run inputs, results and CLI surface"; `cli-gap` for `quoin` or
`quire` in their own team's backlog; `doc-gap` in "FSL example findings" with
the `docs` label; `agent-error`, `sandbox-breach` and `harness-error` in
"FSL L5: agent journey evals". When an open ticket already covers the same
scenario, class and first failure, the triager comments on it with the run id
and sample instead of filing another. The ticket key goes in the row's Ticket
column. `agent-evals triage --check --run <run id>` exits 1 when a row is
`unclassified`, has no tool for `cli-gap`, or has a Ticket that is not a Linear
key (`[A-Z]+-[0-9]+`).

## Inputs

- The operator's checkout path, its `origin` remote and the credentials named
  in FR-001.
- The run folders under `agent-evals/reports/`.

## Outputs

- The launchd agent or crontab entry.
- One run folder, one branch and one pull request per scheduled run.
- `agent-evals/SCORECARD.md`.
- Linear tickets for classified failures.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-002-AC-1 | On macOS, `schedule install --repo R` writes a plist with label `ai.agent-ix.agent-evals`, `StartCalendarInterval` weekday 1, hour 3, minute 0, and the exact `ProgramArguments` above with `R` substituted; on Linux it writes the exact crontab line. Running it twice leaves exactly one entry. | Test (TC-002) |
| FR-002-AC-2 | `scheduled` against a local bare `origin` whose `main` differs from the checkout's branch, with stub `cli-evals` and `gh` programs on `PATH`, runs the harness from `origin/main`, pushes branch `agent-evals/run-<run id>` whose one commit changes only `agent-evals/reports/<run id>/` and `agent-evals/SCORECARD.md`, calls `gh pr create` with title `agent-evals: run <run id>`, and leaves the checkout's `HEAD`, index and `git status` output unchanged and no worktree registered. A second run on the same date uses `<date>-2`. | Test (TC-002) |
| FR-002-AC-3 | A committed report contains no occurrence of the run root or the operator's home folder, every `workDir` starts with `$RUN/`, and the report decodes as a closed `cli-agent-evals.report/v1` report; a raw report with an unknown member is refused before anything is committed. | Test (TC-002) |
| FR-002-AC-4 | Over the fixture runs `2026-01-05` (codex model A: EV-000 `3/3`, EV-001 `1/3`; claude: EV-000 `2/3`) and `2026-01-12` (codex model A: EV-000 `3/3`, EV-001 `2/3`; claude with a different model: EV-000 `3/3`), `scorecard` writes the golden `SCORECARD.md`: codex EV-000 `pass 3/3 same`, codex EV-001 `unstable 2/3 up`, claude EV-000 `pass 3/3 new`; a scenario with null `toolCalls` shows `n/a`; and the history row of codex EV-001 reads `1/3 2/3`. Writing twice gives identical bytes. | Test (TC-002) |
| FR-002-AC-5 | `scorecard --check` exits 0 after `scorecard` and 1 after one character of `SCORECARD.md` changes or a new run folder is added. | Test (TC-002) |
| FR-002-AC-6 | For a report with two failed samples, one whose failure starts `sandbox-breach:` and one other, `run` writes `triage.md` with two rows classed `sandbox-breach` and `unclassified`; `triage --check` exits 1 for it, exits 1 for a `cli-gap` row with no tool and for a Ticket `#12`, and exits 0 once both rows have a class and a key such as `QSL-1234`. | Test (TC-002) |
| FR-002-AC-7 | Two consecutive scheduled runs are merged to `main`; `SCORECARD.md` on `main` lists both runs, and EV-000 shows a trend other than `new` for each agent whose model did not change. | Demonstration (TC-002) |

## Dependencies

- [FR-001](./FR-001-agent-eval-harness.md): the harness, the cell rule and the
  provider's failure classes.
- The `cli-agent-evals.report/v1` contract of `@agent-ix/cli-agent-evals`.

## Constraints

- No run is triggered per pull request.
- No GitHub issue is filed for a failure.
- All of it is in the `agent-evals` Rust program; the schedule entry runs that
  program and holds no other logic.
