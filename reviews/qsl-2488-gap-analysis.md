---
id: SR-1448
title: "EV-004 complete monitor evidence acceptance review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-examples; QSL-2488; agent-evals/src/scenarios/trace_monitor.rs; agent-evals/fixtures/EV-004/; disposition 1: agent-evals/src/capture.rs and EV-003 thin capture wrappers"
review_set: subset
---

## Summary

Ticket: QSL-2488. Reviewed the new EV-004 monitor grader, original fixtures and task at frozen head 677430820f1160c567e4da488ff17f8b12df2951. Shared provider/framework code and later mechanical registration are excluded.

## Verdict

**CONDITIONAL** — original complete-stream finding fixed at 5d7f430dacb80315e1ac26acded93ab4a3992a60; actual live acceptance remains pending.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | Saved and trusted streams are compared only after observe drops intermediate event details. Inserting a same-clause pending event with fabricated position/record/disposition, or omitting or altering non-decisive events, leaves the projection equal and can pass despite not saving the complete real output. Compare the complete ordered semantic stream, excluding only explicitly harmless timing metadata. | agent-evals/src/scenarios/trace_monitor.rs:288-316;481-484 | implementation-bug-despite-evidence |

## Coverage

Plan completion: not assessed. This is a diff-scoped acceptance and Rust review, not a repository inventory. The accepted seven new grader/reference checks were supplied by the coder and were not rerun; no tests, builds, public CLI probes, matrices, auth checks or live samples were executed. The original inputs, exact requested rule forms, report finality/positions, current ledger binding and input-integrity checks were examined. Repository-wide duplicate/helper searches and assurance-profile inventories are excluded by the explicit review scope. Model identifier is unavailable in this reviewer runtime and is not inferred.

### Examined units

- FR-001 (examined), agent-evals/spec/functional/FR-001-agent-eval-harness.md: The provider stops its own child commands (`fsl`, `quoin`, `quire`) when the budget runs out and returns a failed assertion whose failure starts `harness-error: assert budget exceeded`, so a slow tool is classified and never read as an agent failure.
- EV-004 (examined), agent-evals/fixtures/EV-004/task.md: Run these three public monitor checks and save each complete JSON-lines standard output under the indicated name:
- EV-004 (examined), agent-evals/fixtures/EV-004/task.md: The evaluation_position is the origin at which the rule is evaluated, reported by settlement.position. The observed_position is the event's position when it settles.
- EV-004 (examined), agent-evals/fixtures/EV-004/task.md: Only when the files and checks are complete, make your final action:

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The grader compares the complete ordered events and complete summary against the trusted monitor. Only event top-level wall_ms and summary top-level elapsed_ms are removed; intermediate changes, array order, input bindings and nested fields remain semantic. |

### Disposition 1 coverage

Inspected only the fix diff through 5d7f430dacb80315e1ac26acded93ab4a3992a60: EV-004 caller/deadline/stream fixes, first formal review of capture.rs, and EV-003 thin error-map wrapper preservation. No additional findings. Accepted the new stream/deadline/quota regressions and current Clippy result supplied by the coder; none was rerun. Shared primitive review is complete here and need not be repeated by later scenario integration.
