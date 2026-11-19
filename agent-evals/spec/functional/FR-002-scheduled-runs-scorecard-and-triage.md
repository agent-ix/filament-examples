---
id: FR-002
title: "Scheduled agent eval runs, committed history, scorecard and triage"
type: FR
relationships:
  - target: ix://agent-ix/filament-examples/agent-evals/FR-001
    type: depends_on
---

# FR-002: Scheduled runs, scorecard and triage

## Description

The agent eval suite of [FR-001](./FR-001-agent-eval-harness.md) runs on a
weekly schedule, not per pull request. Each run's reports are committed to this
repository, a scorecard rebuilt from every committed run shows the trend, and
every failed sample gets a class and a Linear ticket before the run is merged.
The requirement ids of this folder belong to the `agent-evals` sub-project
(`ix://agent-ix/filament-examples/agent-evals/...`), not to the example
projects elsewhere in the repository that also number from FR-001.

### Decision: the schedule runs on the dev host

The live agents need the operator's credentials and a terminal multiplexer, and
CI in this organisation is started by hand only. The schedule therefore runs on
the dev host, as a per-user launchd agent on macOS or a user crontab entry on
Linux. `agent-evals schedule install --repo <checkout>` installs the launcher
with `cargo install --locked --path <checkout>/agent-evals --root
~/.local/share/agent-evals` and writes the entry:

- macOS: `~/Library/LaunchAgents/ai.agent-ix.agent-evals.plist`, label
  `ai.agent-ix.agent-evals`, `StartCalendarInterval` weekday 1, hour 3,
  minute 0 (Monday 03:00 local time), standard output and error to
  `~/Library/Logs/agent-evals.log`, and `ProgramArguments`
  `["/bin/zsh", "-lc", "exec ~/.local/share/agent-evals/bin/agent-evals scheduled --repo <checkout>"]`.
  The login shell gives the job the operator's `PATH` (cargo, node, git, gh and
  the agent commands) and `CLAUDE_CODE_OAUTH_TOKEN`.
- Linux: one crontab line `0 3 * * 1 /bin/bash -lc 'exec ~/.local/share/agent-evals/bin/agent-evals scheduled --repo <checkout>' >> ~/.local/state/agent-evals/agent-evals.log 2>&1`.

Installing again replaces the launcher and the entry; it never adds a second
entry. A scheduled routine in the cloud is not used: it has neither the
operator's agent credentials nor the terminal session the runner drives.

### Decision: a scheduled run builds from a clean worktree of main

The installed launcher only prepares and hands over: `scheduled --repo
<checkout>` fetches `origin`, adds a clean temporary detached worktree of
`origin/main` in a new folder `agent-evals-src-<run id>` under the operating
system's temporary folder (outside the run root, so agents cannot read it,
FR-001), builds that worktree's `agent-evals` package with `cargo build
--locked --release` and `CARGO_TARGET_DIR` inside that folder, and runs the
built program's `run`. The run therefore uses the harness exactly as it is on
`main`, never the checkout's branch, index or uncommitted files, whatever
version the launcher is. It then commits the run folder and the rebuilt
scorecard on a new branch `agent-evals/run-<run id>`, pushes it, opens a pull
request titled `agent-evals: run <run id>` with `gh pr create`, and removes the
worktree and its build folder. The checkout's branch, index and working tree
are unchanged. A run whose `tools` step refuses (FR-001) opens no pull request
and logs the refusal.

### Decision: one open eval pull request at a time

Before it creates anything, `scheduled` lists the open pull requests whose head
branch starts with `agent-evals/run-`. When one is open, the run is skipped: it
appends `skipped <date>: run PR #<n> is still open` to the log, creates no run
root, branch or pull request, and exits 0. Run pull requests therefore never
stack, and each one's `SCORECARD.md` builds on every earlier run, already on
`main`.

The run id is the UTC date `YYYY-MM-DD`; when that id is already taken by a
folder on `origin/main` or by a remote branch `agent-evals/run-<id>`, the run
takes the first free `YYYY-MM-DD-<n>`, `n` from 2.

### Decision: the history is committed run folders

Each run writes `agent-evals/reports/<run id>/`:

| File | Holds |
| --- | --- |
| `codex.json`, `claude.json` | The runner's `cli-agent-evals.report/v1` report for that agent, redacted. |
| `tools.json` | The FR-001 tool and docs provenance of the run. |
| `triage.md` | One row per failed sample with its class and ticket. |

This repository is public and admits no workstation paths, so every string in
the committed files is redacted, in this order: the literal run root, and its
`/private/var/folders` or `/var/folders` alias, becomes `$RUN`; the operator's
home folder becomes `$HOME`; then, in `failures`, `checks` and `triage.md`,
every remaining absolute path (a match of
`(?:^|[\s"'=(:])(/(?:[A-Za-z0-9._@+-]+/)*[A-Za-z0-9._@+-]+)`) becomes `<path>`.
`workDir` and `transcriptPath` keep only the `$RUN`-relative form. The redacted
report still decodes as a closed `cli-agent-evals.report/v1` report. The
reports are the run's results, read by the scorecard; they are not copies of a
version or a digest kept for comparison.

Transcripts and workspaces are not committed. When the run ends, `run` copies
each kept transcript out of the operating system's temporary folder (which the
system may clear) to `~/.local/state/agent-evals/<run id>/transcripts/`, outside
the repository, for triage; `agent-evals clean --run <run id>` deletes the run
root and that folder after the run's pull request merges.

### Decision: the merge gate is a local check

CI here is started by hand only, so no workflow enforces the gate. The merger
runs `make agent-evals-check RUN=<run id>` (a target of the root `Makefile`,
listed in `.PHONY`, FR-001) on the pull request's branch before
merging; the target calls `agent-evals triage --check --run <run id>` and
`agent-evals scorecard --check` and nothing else. The pull request is merged
only when both exit 0, and is otherwise reviewed like any other change.
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
| FR-002-AC-1 | On macOS, `schedule install --repo R` installs the launcher under `~/.local/share/agent-evals` and writes a plist with label `ai.agent-ix.agent-evals`, `StartCalendarInterval` weekday 1, hour 3, minute 0, and the exact `ProgramArguments` above with `R` substituted; on Linux it writes the exact crontab line. Running it twice leaves exactly one entry. | Test (TC-002) |
| FR-002-AC-2 | `scheduled` against a local bare `origin` whose `main` differs from the checkout's branch, and with an uncommitted edit to the checkout's `agent-evals/src`, with stub `cli-evals` and `gh` programs on `PATH`: builds and runs the harness of `origin/main` (the stub sees the `main` version string, not the edited one) from a worktree outside the run root; pushes branch `agent-evals/run-<run id>` whose one commit changes only `agent-evals/reports/<run id>/` and `agent-evals/SCORECARD.md`; calls `gh pr create` with title `agent-evals: run <run id>`; and leaves the checkout's `HEAD`, index and `git status` output unchanged, no worktree registered and no build folder left. | Test (TC-002) |
| FR-002-AC-3 | A committed report and `triage.md`, built from a raw report whose `workDir`, `failures` and `checks` hold the run root, its `/private/var/folders` alias, the home folder and an unrelated absolute path `/opt/x/y`, contain none of them: every `workDir` starts with `$RUN/`, the home folder reads `$HOME` and `/opt/x/y` reads `<path>`. The report decodes as a closed `cli-agent-evals.report/v1` report; a raw report with an unknown member is refused before anything is committed. | Test (TC-002) |
| FR-002-AC-4 | Over the fixture runs `2026-01-05` (codex model A: EV-000 `3/3`, EV-001 `1/3`; claude: EV-000 `2/3`) and `2026-01-12` (codex model A: EV-000 `3/3`, EV-001 `2/3`; claude with a different model: EV-000 `3/3`), `scorecard` writes the golden `SCORECARD.md`: codex EV-000 `pass 3/3 same`, codex EV-001 `unstable 2/3 up`, claude EV-000 `pass 3/3 new`; a scenario with null `toolCalls` shows `n/a`; and the history row of codex EV-001 reads `1/3 2/3`. Writing twice gives identical bytes. | Test (TC-002) |
| FR-002-AC-5 | `scorecard --check` exits 0 after `scorecard` and 1 after one character of `SCORECARD.md` changes or a new run folder is added; `make agent-evals-check RUN=<id>` exits non-zero when either check fails. | Test (TC-002) |
| FR-002-AC-6 | For a report with two failed samples, one whose failure starts `sandbox-breach:` and one other, `run` writes `triage.md` with two rows classed `sandbox-breach` and `unclassified`; `triage --check` exits 1 for it, exits 1 for a `cli-gap` row with no tool and for a Ticket `#12`, and exits 0 once both rows have a class and a key such as `QSL-1234`. | Test (TC-002) |
| FR-002-AC-7 | Two consecutive scheduled runs are merged to `main`; `SCORECARD.md` on `main` lists both runs, and EV-000 shows a trend other than `new` for each agent whose model did not change. | Demonstration (TC-002) |
| FR-002-AC-8 | With an open pull request on a branch `agent-evals/run-2026-01-05`, `scheduled` exits 0, appends one `skipped` line naming that pull request to the log, and creates no run root, branch, push or pull request. With none open, a folder `2026-01-12` on `origin/main` and a remote branch `agent-evals/run-2026-01-12-2`, a run on 2026-01-12 takes the id `2026-01-12-3`. | Test (TC-002) |
| FR-002-AC-9 | After a run with `--keep`, each kept transcript exists under `~/.local/state/agent-evals/<run id>/transcripts/` with the digest the report names, and none is under `agent-evals/`. | Test (TC-002) |

## Dependencies

- [FR-001](./FR-001-agent-eval-harness.md): the harness, the cell rule and the
  provider's failure classes.
- The `cli-agent-evals.report/v1` contract of `@agent-ix/cli-agent-evals`.

## Constraints

| ID | Constraint | Type | Validation |
| --- | --- | --- | --- |
| FR-002-CON-1 | No run is triggered per pull request. | Operational | Inspection |
| FR-002-CON-2 | No GitHub issue is filed for a failure. | Operational | Inspection |
| FR-002-CON-3 | All logic is in the `agent-evals` Rust program; the schedule entry runs that program and holds no other logic. | Implementation | Test (TC-002, AC-1) |
| FR-002-CON-4 | No transcript, workspace, credential or workstation path is committed. | Security | Test (TC-002, AC-3, AC-9) |
