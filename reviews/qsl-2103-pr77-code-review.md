---
id: "SR-011"
type: SpecReview
title: "code-review of filament-examples#77 (QSL-2103)"
date: "2026-11-21"
scope: "agent-ix/filament-examples; examples/claims, tools/models, Makefile, README.md, index.json (PR #77)"
---

# SR-011: code-review of filament-examples#77

## Summary

Ticket: QSL-2103. Changes requested.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Staged model-possible-trap-cyclic waits_on QSL-2103 (this corpus ticket), not a product ticket; no ticket was filed for the defect | - |
| FND-002 | medium | Coverage list: AC-12 pending deadline and AC-16 refusal position are not in the recorded outputs; AC-13 conflicting-valuation is covered but listed as uncovered; AC-13 absent, AC-16 order keys, AC-18/20/25 and AC-15 half-open watermark are coverable as staged cases | - |
| FND-003 | low | Orphan request files requests/act1..act5.json and requests/window-unkeyed-ties-eventually.json are referenced by no case | - |
| FND-004 | low | README.md conflicts with main after #76 (types-checking row) and with #78, #79, #81 | - |

## Verdict

Changes requested. Full finding data is in the Linear review comment for SR-011.
