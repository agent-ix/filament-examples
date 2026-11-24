---
id: SR-202611240004
title: "QSL-3010 failure-domain review"
type: SpecReview
analysis: failure-domain
scope: "FR-001-AC-15; TC-001 step 15"
review_set: subset
---

## Summary

Reviewed the changed value boundary for shape confusion across `fsl run`, snapshot and monitor observations. The criterion explicitly limits its typed-value rule to `fsl run` and preserves the distinct EV-009 snapshot and monitor formats. Legacy `fsl run` input refusal is named with both refusal codes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No failure-domain issue found in the changed scope. | - |
