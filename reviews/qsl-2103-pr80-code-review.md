---
id: "SR-012"
type: SpecReview
title: "code-review of filament-examples#80 (QSL-2103)"
date: "2026-11-21"
scope: "agent-ix/filament-examples; examples/engines, tools/models, Makefile, README.md, index.json (PR #80)"
---

# SR-012: code-review of filament-examples#80

## Summary

Ticket: QSL-2103. Changes requested.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | schedulability-response-meets / -misses: rules/sched.fsl gives a priority 2 and b priority 1, and FR-419 says a smaller value is a higher priority, so a's response is 15 ms; 'proved' at 5 ms (ready_exit 0) is wrong | - |
| FND-002 | medium | deadlock-* are staged as a product gap, but unsupported/unbounded-extent for the contract items is FR-711-AC-2's disposition and exit 21 follows; rebuild the requests so the deadlock item is not mixed with contract items | - |
| FND-003 | medium | waits_on: schedulability-* name QSL-2131, which is Done with its wire tests (75d3b40ac7) inside the driver rev 529282e9; deadlock-* name QSL-2103 (this corpus ticket) | - |
| FND-004 | medium | No staged case for refinement (QSL-2243), composition (QSL-2739), hyperproperties (QSL-2248), synthesis / planning (QSL-1978), or FR-366-AC-3 terminal any | - |
| FND-005 | low | FR-724 is the 'model without random parameters' requirement; the coin has a random parameter, so the atom / always rows belong to FR-407 / FR-411 | - |
| FND-006 | low | verb-integer-division-on-compile caption says the option belongs to run and analyze requests; FR-301-AC-6 says it refuses on run, analyze, prove and monitor too | - |

## Verdict

Changes requested. Full finding data is in the Linear review comment for SR-012.
