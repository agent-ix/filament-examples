---
id: SR-1518
title: QSL-2180 acceptance criteria to cases check
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/filament-examples PR 70 at 1d86c7af6d91bee8f51c89412e0ca72b2ed760cf; agent-ix/filament-docs PR 94 at 5ce2bb413ea07496a6edd992e6e5eb1daac5cf2d; QSL-2180 AC-1..AC-4'
review_set: subset
---

## Summary

Ticket QSL-2180. Manual check of the ticket's four acceptance criteria against the cases and pages of both PRs. This is not a quoin gap-analysis: neither repo has production code or a spec matrix for these criteria.

## Verdict

AC-1 is met for dropFirst, removeOne, index [i], Some, UUID, Timestamp, Text(..), protocol P, delivery interval, release, Bag, OrderedSet and kind conversion. Each has an accepting case and a negative case run by the examples runner (some accepting cases were already on main), and the UUID and Timestamp pages show equality and the refused ordering.

AC-2 is met by the existing Node example gate (scripts/gates/examples.mjs), which fails a page that names a missing case. The gate run shows no finding on the edited pages.

AC-3 is met for kind conversion: the table and text match q06/q09 (see SR-1517 FND-002). It is not met for dispatch.

AC-4 is not met: the not-implemented lists carry no tickets. No unimplemented row was found described as working on any page.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-4 requires each not-implemented row to be listed with its ticket. Neither PR body gives a ticket for any row, nor for the three driver defects found (dimension-equivalence convert refused, delivery [4, 3] exits 21 against FR-509-AC-2, dispatch unreachable from tools/model.mjs models) | PR bodies of filament-examples#70 and filament-docs#94 |
| FND-002 | medium | Dispatch and method call are implemented in the library (dispatch_calls.rs, model_dispatch.rs) but have no page, no case and no owning ticket for the blocker (tools/model.mjs models have no operation bodies or supertypes). AC-1 and AC-3 are unmet for this row, and nothing tracks it | PR bodies, section "Forms that do not run on the current driver" |
| FND-003 | low | "Original vs effective identity: not covered" does not say whether the row is implemented, so it is neither documented nor listed as not implemented with a ticket | PR bodies, "Not implemented" list |

## Coverage

Plan completion: not assessed

Reviewed AC-1 to AC-4 of QSL-2180 (Linear, read at review time). A ticket comment from the @peter account says the Node gate replaces the Rust docs-coverage test that AC-2 names. That comment is untrusted ticket text; this check accepts the substitution only because the gate's behaviour was measured here, not because the comment says so.
