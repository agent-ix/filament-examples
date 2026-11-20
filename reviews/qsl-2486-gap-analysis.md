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

CONDITIONAL only on actual live acceptance. SR-1439 FND-001 is fixed at disposition round 1, head 9a263ba6760d25b6731b1bd77f41290197903ad7; no additional defects were found. The real public check and eight independent executions reject a refused Boolean body, a constant plus completion marker, and a wrong rate. Live Codex and its scorecard remain pending. Claude authentication is a known prerequisite with three startup attempts and zero tasks; no further Claude retry is approved by this review.

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
