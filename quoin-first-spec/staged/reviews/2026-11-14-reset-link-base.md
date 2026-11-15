---
id: SR-001
title: "base review of the reset link specification"
type: SpecReview
analysis: base
scope: "spec/functional/FR-001-password-reset-link-expiry.md"
review_set: base
---

## Summary

Base checklist review of FR-001 (password reset link expiry and single use): ID formats, FR quality, coverage rules, and the hash/digest/pin antipattern scan. Verdict: PASS on the antipattern scan (no hash, digest, pin, manifest, ledger or tracking-record requirement found) and on ID format; the spec is not yet ready for implementation because five of seven acceptance criteria and both constraints have no tagged test, and Outputs contradicts the non-disclosure criterion.

Assurance profiles: none found in the repository (only the catalog skeleton exists), so no `review_policy` applies and no enforced run is required. Direct review is therefore permitted. Only the base checklist was run; no analysis lenses.

## Findings

| ID      | Severity | Summary                                                                                                                                                                      | Refs                                      |
| ------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------- |
| FND-001 | medium   | `quire matrix` reports FR-001-AC-3 to AC-7 as untagged; only AC-1 and AC-2 are carried by tests (tests/test_reset_links.py).                                                 | FR-001-AC-3, FR-001-AC-4, FR-001-AC-5, FR-001-AC-6, FR-001-AC-7 |
| FND-002 | medium   | Both constraints (server-clock expiry, no account disclosure) are untagged, so their `Test` validation is unevidenced.                                                       | FR-001-CON-1, FR-001-CON-2                |
| FND-003 | medium   | Outputs promises a rejection "stating the link is invalid, expired, or already used", which contradicts AC-7 and CON-2 requiring identical status and body for unknown, malformed and expired tokens. | FR-001 Outputs, FR-001-AC-7, FR-001-CON-2 |
| FND-004 | medium   | Error codes are incomplete: AC-7 requires an identical response but names no status; AC-2, AC-3, AC-6 give HTTP 410. Unknown and malformed tokens have no stated code, and AC-7 does not cover already-used or superseded tokens. | FR-001-AC-7, FR-001-AC-2, FR-001-AC-3, FR-001-AC-6 |
| FND-005 | low      | FR is not linked to a user story (Upstream: none), and no priority is specified.                                                                                              | FR-001 Dependencies                       |
| FND-006 | low      | Inputs lack type and validation (token format, length, encoding), though AC-7 depends on a defined "malformed" notion.                                                       | FR-001 Inputs, FR-001-AC-7                |
| FND-007 | low      | Edge cases missing: presentation before issue time (clock skew or negative elapsed), and concurrent issuance of two links for one account racing the invalidation behavior.  | FR-001 Behavior                           |
| FND-008 | low      | Constraints carry no rationale; AC-5 largely restates AC-2 plus AC-3 and may be redundant unless it targets a stored "expired" state.                                         | FR-001-CON-1, FR-001-CON-2, FR-001-AC-5   |
| FND-009 | low      | No performance or availability targets for the reset endpoint, which the checklist expects to be specific or explicitly out of scope.                                         | FR-001                                    |
