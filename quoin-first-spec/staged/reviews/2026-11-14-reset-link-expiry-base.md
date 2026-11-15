---
id: SR-001
title: "Base review of the reset link spec"
type: SpecReview
analysis: base
scope: "spec/"
review_set: base
---

# SR-001: Base review of the reset link spec

## Summary

The base checklist examined US-001 and FR-001. IDs, links and the acceptance criteria table are well formed, and no requirement asks for a hash, digest or pin. One criterion has no test, and the user story promises a behavior that no requirement covers.

## Findings

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-001 | medium | FR-001-AC-3 is untagged: no test asserts that a link that was already used is refused. | FR-001 |
| FND-002 | medium | US-001-EX-2 says an expired user can ask for a new link, and no requirement covers requesting one. | US-001 |
| FND-003 | low | FR-001 does not say what a user sees when a link is refused as already used, only when it expired. | FR-001 |
