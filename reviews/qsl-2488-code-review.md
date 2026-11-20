---
id: SR-1447
title: "EV-004 public monitor grader code and Rust review"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-examples; QSL-2488; agent-evals/src/scenarios/trace_monitor.rs; agent-evals/fixtures/EV-004/; disposition 1: agent-evals/src/capture.rs and EV-003 thin capture wrappers"
review_set: subset
---

## Summary

Ticket: QSL-2488. Reviewed the new EV-004 monitor grader, original fixtures and task at frozen head 677430820f1160c567e4da488ff17f8b12df2951. Shared provider/framework code and later mechanical registration are excluded.

## Verdict

**PASS** — original deadline finding fixed at 5d7f430dacb80315e1ac26acded93ab4a3992a60; the authoritative shared capture primitive is approved. Live acceptance remains pending.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | The scenario declares a 30-second assertion budget, but monitor uses unbounded Command::output and assess carries no deadline. A hung CLI or unbounded capture can outlive the assertion budget, with no scenario-owned timeout termination/reaping or harness-error classification. | agent-evals/src/scenarios/trace_monitor.rs:448-459;476-492 | correct-requirement-no-evidence |

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
| FND-001 | fixed | One 30-second absolute deadline starts before validation and reaches all three monitor children and final checks. File-backed capture checks output size while the child runs, performs bounded final reads, and kills and reaps its owned child on exhaustion. EV-003 thin wrappers preserve its 30-second budget and existing error-code mapping. |

### Disposition 1 coverage

Inspected only the fix diff through 5d7f430dacb80315e1ac26acded93ab4a3992a60: EV-004 caller/deadline/stream fixes, first formal review of capture.rs, and EV-003 thin error-map wrapper preservation. No additional findings. Accepted the new stream/deadline/quota regressions and current Clippy result supplied by the coder; none was rerun. Shared primitive review is complete here and need not be repeated by later scenario integration.
