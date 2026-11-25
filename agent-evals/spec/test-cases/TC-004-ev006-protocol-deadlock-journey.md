---
id: TC-004
title: "Scenario EV-006: model and repair a protocol deadlock"
type: TC
relationships:
  - target: ix://agent-ix/filament-examples/agent-evals/FR-004
    type: verifies
---

# TC-004: Scenario EV-006 protocol deadlock journey

## Description

Verify the prepared inputs, structured deadlock evidence, result comparison,
accepted source repairs, bounded predicate checks, certified exhaustive proof,
and grader failure classes for EV-006. The tests exercise fixed fixtures and
stubbed public CLI calls; this case does not start Codex or Claude runs.

Scope: FR-004-AC-1 through FR-004-AC-8.

## Test Procedure

1. Check that preparation writes the fixed task inputs and that a changed input
   is rejected without a hidden reference implementation (AC-1).
2. Check the trusted deadlock's ordered prefix, both blocked paths, and causes;
   mutate each required part and confirm the grader refuses it (AC-2).
3. Compare saved summaries with fresh CLI structures, reorder the blocked list,
   and reorder the execution prefix (AC-3).
4. Check the terminal repair's preserved branches and attempts and required
   terminal position (AC-4).
5. Check the precondition repair changes only the blocked operation's
   precondition and preserves both branches and attempts (AC-5).
6. Probe all bounded cell values for each accepted predicate and reject literal
   and semantic tautologies (AC-6).
7. Check that a repaired result lacking proof, exhaustiveness, certification,
   or an accepted certificate is refused (AC-7).
8. Run the grader with each intended repair and with stubbed unsupported,
   failing, and timing-out tools; verify the `cli-gap` and `harness-error`
   boundaries (AC-8).

## Expected Results

- Every EV-006 acceptance criterion is covered by scenario grader tests.
- Either intended repair passes only with the required predicate behavior and
  certified exhaustive proof.
- No live model run is part of this case.

