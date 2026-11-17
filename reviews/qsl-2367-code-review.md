---
id: SR-005
title: "Code review — observation limit request example"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-examples; examples/shop/requests/limits/observation-document-bytes.json, examples/shop/example.toml, examples/shop/expected/limits-observation-document-bytes.messages.txt; QSL FR-106, FR-255, FR-312"
review_set: subset
---

## Summary

Ticket QSL-2367, PR agent-ix/filament-examples#41, exact head `f0ae61cd4eeac99f7fc6847ccce2d00dae55115f`. Reviewed the one request-member change against the merged observation-limit wire shape and the retained limit-reached case. The request uses the public `input_bytes` member; the diagnostic retains its internal document-byte terminology.

## Verdict

**PASS** — the wire member matches FR-312's FR-106 `ObservationLimits`, the case still expects exit 22 and `resource_exhausted`, and the expected diagnostic matches the focused run reported from driver main `895d2f727c4236b36ae48087d91fb14a95de4401`.

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | low      | No findings (placeholder) | - |
