---
id: TC-002
title: "Scheduled runs, committed history, scorecard and triage"
type: TC
relationships:
  - target: ix://agent-ix/filament-examples/FR-002
    type: verifies
---
# TC-002: Scheduled runs, scorecard and triage

## Description

Verify the schedule entry, the scheduled run's git handling, report
redaction, the scorecard and its trend rule, the triage check, and two
consecutive live scheduled runs.

Scope: FR-002-AC-1 through FR-002-AC-7.

## Test Procedure

1. Run `schedule install` twice with a temporary home folder and compare the
   plist (macOS) or crontab line (Linux) with the expected text (AC-1).
2. Run `scheduled` against a local bare `origin` with stub `cli-evals` and
   `gh` programs; inspect the pushed branch, the commit's paths, the `gh`
   arguments and the checkout's state; run it again on the same date (AC-2).
3. Redact a raw report fixture holding run-root and home paths, decode the
   result, and feed a report with an unknown member (AC-3).
4. Build `SCORECARD.md` from the two fixture runs and compare it with the
   golden file; build it twice (AC-4).
5. Run `scorecard --check` after a rebuild, after a one-character edit and
   after adding a run folder (AC-5).
6. Run `run` over the two-failure report fixture and `triage --check` over
   each row state (AC-6).
7. Merge two consecutive scheduled runs and read `SCORECARD.md` on `main`
   (AC-7).

## Expected Results

- The scorecard is byte-identical across rebuilds and matches the golden file.
- No committed file holds a run-root or home path.
- `triage --check` passes only when every row has a class and a Linear key.
