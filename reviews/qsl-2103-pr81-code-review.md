---
id: "SR-013"
type: SpecReview
title: "code-review of filament-examples#81 (QSL-2103)"
date: "2026-11-21"
scope: "agent-ix/filament-examples; examples/run-inputs, tools/models, Makefile, README.md, index.json (PR #81)"
---

# SR-013: code-review of filament-examples#81

## Summary

Ticket: QSL-2103. Changes requested.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Valid call-* and option-* running cases send and record values as {"integer": "5"} / {"sequence": [...]}; FR-550 requires arguments read and results written in the FR-181 typed canonical form {"type":"integer","value":"5"}, so the expected outputs bless a wrong wire | - |
| FND-002 | medium | Refusal cases do not assert what FR-550-AC-2 names: missing / unknown parameter and unknown function have cause null; range and cardinality refusals name 'position 0' and location "" instead of the parameter | - |
| FND-003 | medium | waits_on: call-tagged-* name QSL-2103 (this corpus ticket); plan-* name QSL-2502, Canceled as a duplicate of QSL-2511 | - |
| FND-004 | medium | Not-covered list: FR-550-AC-3 multi-item, reference / population arguments and the FR-824 member refusals are coverable as staged cases (the tank model is already in this system) | - |
| FND-005 | low | unit-header-only.messages.txt records 'a type alias has none' for a unit that has no alias | - |

## Verdict

Changes requested. Full finding data is in the Linear review comment for SR-013.
