---
id: TC-001
title: "Agent eval harness: provider protocol, released tools, sandbox and EV-000"
type: TC
relationships:
  - target: ix://agent-ix/filament-examples/agent-evals/FR-001
    type: verifies
---
# TC-001: Agent eval harness

## Description

Verify that the `agent-evals` program answers the runner's provider protocol,
installs only released tools, builds the sandbox, detects a sandbox breach,
scores EV-000, and that a live EV-000 run passes for both agents.

Scope: FR-001-AC-1 through FR-001-AC-14.

## Test Procedure

1. Load `cli-agent-evals.config.mjs` with the runner (`cli-evals list`) and
   check its members; add each forbidden member in a copy and check the
   refusal; walk `agent-evals/` for `.py` and shell files (AC-1).
2. Send `describe`, then each malformed request, to `agent-evals provider`
   and check the catalogue and the non-zero exits (AC-2).
3. Run `agent-evals tools` against a local release fixture with a matching
   archive, a mismatched checksum and a missing host archive, with stub `gh`
   and `npm` programs on `PATH` (AC-3).
4. Run `prepare` for EV-000 and check the workspace files, prompt and
   environment (AC-4).
5. Run `run` with each credential missing, then with both present and a stub
   runner that records its environment and exits 0 or 2 (AC-5).
6. Run `assert` over every breach and pass transcript fixture (working
   directory tracking, path aliases, tool folders), a missing transcript and
   an unknown format (AC-6).
7. Run `assert` for EV-000 over a passing workspace and each failing variant
   (AC-7).
8. Run EV-000 live for `codex` and `claude` with three repeats and `--keep`
   and commit the two reports (AC-8).
9. Feed the cell rule each pass count and a report with `repeats` 2 (AC-9).
10. Run EV-000 `assert` with a stub `fsl` that sleeps past the budget (AC-10).
11. Run `run` with stub `cli-evals` programs that exit 1 with a report, 1
    without one, and 2 (AC-11).
12. Run `clean` for a known and an unknown run id (AC-12).
13. Run `run` with `CLAUDE_CONFIG_DIR`, `CODEX_HOME` and `CLAUDECODE` set and a
    stub runner that records its environment (AC-13).
14. Read the root `Makefile` `.PHONY` line, run `make -n agent-evals` and
    `make agent-evals` with a stub `cli-evals` (AC-14).

## Expected Results

- Every refusal exits non-zero and writes no result, report or `tools.json`.
- The live reports carry the `agents.toml` models, `repeats` 3 and EV-000
  `passRate` `3/3`.
