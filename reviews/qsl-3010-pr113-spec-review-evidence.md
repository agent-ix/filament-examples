---
id: SR-202611240003
title: "QSL-3010 evidence review"
type: SpecReview
analysis: evidence
scope: "agent-evals/spec/functional/FR-001-agent-eval-harness.md; agent-evals/spec/test-cases/TC-001-agent-eval-harness.md"
review_set: subset
---

## Summary

The criterion appropriately names a Test and TC-001 step 15 exercises both valid value shapes and legacy rejection. The repository-level `quoin advise --json` could not derive obligations because this repository has no root `spec/` document root; the changed artifacts are nested under `agent-evals/spec`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The named golden contains only the completed `result.value`, not the EV-002 request; the evidence method must name a separate request fixture/assertion to demonstrate the request's expected member shape. | FR-001-AC-15; TC-001 step 15 |
