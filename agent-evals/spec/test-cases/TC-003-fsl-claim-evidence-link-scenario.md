---
id: TC-003
title: "Scenario EV-005: link an FSL claim to its matrix row through a quoin evidence run"
type: TC
relationships:
  - target: ix://agent-ix/filament-examples/agent-evals/FR-003
    type: verifies
---
# TC-003: Link an FSL claim to a test matrix row

## Description

Verify the EV-005 fixture settlements, the outcome map, the EV-005 and
EV-105 assertions over reference and broken workspaces, the activation of
EV-205, and a live scheduled run of EV-005 and EV-105.

Scope: FR-003-AC-1 through FR-003-AC-7.

## Test Procedure

1. Run the release `fsl analyze` on the EV-005 fixture request and on the
   EV-105 fixture's second commit (AC-1).
2. Map every settlement, including the three `measured` decisions, and a
   disposition with no result; record a `rejected` entry with quoin (AC-2).
3. Build the EV-005 reference workspace with the released `quire` and
   `quoin` and run `assert` (AC-3).
4. Break the reference workspace in each listed way and run `assert` (AC-4).
5. Build the EV-105 reference workspace (passing run, then failing run), run
   `assert`, then break it by re-recording the
   entry as `pass`, dropping the second run and deleting the first (AC-5).
6. Run `tools` with the release `fsl compile`, a stub refusing with another
   code, and a stub `fsl` that accepts the
   extraction probe, then `describe` (AC-6).
7. Run EV-005 and EV-105 in a scheduled run and check `triage.md` (AC-7).

## Expected Results

- `assert` passes only on the reference workspaces and names the failed item
  for each broken one.
- EV-205 is listed by `describe` only when the sandbox `fsl` accepts the
  extraction member.
