---
id: SR-1440
title: "Gap analysis of EV-002 refusal-only repair"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-examples; QSL-2486; fa959c7..b1ee66a7; EV-002 task, prepare/grade and provider registration"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-examples/agent-evals/FR-001
    type: references
---

## Summary

Ticket: QSL-2486. Performed a manual acceptance-to-code-and-tests comparison for the new repair scenario only. The task owns the charge rule and restricted information source; FR-001 supplies the existing provider and transcript-integrity context. No repository-wide assurance claim is made.

## Verdict

CONDITIONAL only on retained live acceptance. Both SR-1439 findings are fixed at 5addd7283ee6e1a14954644ad06e776ff8ec3bd0. The actual Codex sample passed the public check, all eight semantic checks and access policy, but its original SDK report remains 0/1 because command matching missed the editor heredoc followed by public invocations. The runtime parser and termination fix are approved for one retained command/boundary grade; no successful regrade is claimed here. Claude authentication is a known prerequisite with three startup attempts and zero tasks; no further Claude retry is approved by this review.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No additional findings (placeholder) | - |

## Coverage

Examined task rule and preservation requirements, actual-refusal preparation, independent Integer-result grading, expected command operands, withholding of reference links and all four new deterministic tests. The documentation restriction's missing assert enforcement is recorded once in SR-1439 FND-001, rather than duplicated here.

The new tests exercise the real public FSL route in an explicitly named ignored lane. Their supplied passing results are accepted as evidence, not as live-agent samples. They carry no criterion trace tags; this assessment uses the supplied task contract and manual mapping and does not claim computed criterion coverage.

No matrix, broad audit, old baseline, build, test or live-agent run was performed, as directed for this diff review. Shared framework code through fa959c7 was excluded except the callers needed to assess the new Entry policy. The later reviewed shared runtime fixes are not re-reviewed here. Actual Claude authentication/setup acceptance remains a separate pending prerequisite, not evidence of an FSL defect.

Plan completion: not assessed

## Disposition Context

Round 1 inspected only the existing documentation-policy finding's fix. The Entry policy now reaches transcript/path inspection, including canonicalized workspace symlinks, while preserving the author-model entry's permitted reference access. Accepted the supplied two passing new policy regressions without rerunning them. Shared runtime fixes and the later mechanical main integration were outside this disposition. No additional gap finding was opened.

The subsequent runtime review was limited to ff7f82fc..6dc18a9 and accepted the supplied actual sample and two new regression-test receipts. No additional gap finding beyond SR-1439 FND-002 was opened. There was no new live sample, retained-evidence grade, eight-input semantic rerun or repository matrix measurement.

The final runtime disposition was limited to 6dc18a9..5addd728 and the termination fix. FND-002 is fixed; accepted one new focused termination test pass without rerunning any check. No additional gap finding was opened and no live or semantic result was remeasured.
