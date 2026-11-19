---
id: TC-001
title: "Agent eval harness: provider protocol, released tools, sandbox and EV-000"
type: TC
relationships:
  - target: ix://agent-ix/filament-examples/FR-001
    type: verifies
---
# TC-001: Agent eval harness

## Description

Verify that the `agent-evals` program answers the runner's provider protocol,
installs only released tools, builds the sandbox, detects a sandbox breach,
scores EV-000, and that a live EV-000 run passes for both agents.

Scope: FR-001-AC-1 through FR-001-AC-9.

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
5. Run `run` with each credential missing (AC-5).
6. Run `assert` over the three transcript fixtures, a missing transcript and
   an unknown format (AC-6).
7. Run `assert` for EV-000 over a passing workspace and each failing variant
   (AC-7).
8. Run EV-000 live for `codex` and `claude` with three repeats and `--keep`
   and commit the two reports (AC-8).
9. Feed the cell rule each pass count and a report with `repeats` 2 (AC-9).

## Expected Results

- Every refusal exits non-zero and writes no result, report or `tools.json`.
- The live reports carry the `agents.toml` models, `repeats` 3 and EV-000
  `passRate` `3/3`.
