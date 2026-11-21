---
id: SR-1521
title: QSL-2103 area 4 declarations and models corpus code review
type: SpecReview
analysis: code-review
scope: 'agent-ix/filament-examples; examples/declarations-models/**, README.md, index.json; PR 79 at c1480dfdeb185839ce5106ecb805881a41ce9b32'
review_set: subset
---

## Summary

Ticket QSL-2103. Reviewed the `declarations-models` system added by PR 79 (107 running, 0 staged) against the FSpec standard at filament-specification origin/main. Read the full manifest and coverage list; sampled 40 cases across FR-001, FR-002, FR-019, FR-142 (TC-187 unit-graph rows), FR-339, FR-470, FR-480 and FR-610, and every refusal code used, including the five `unsupported` cases. No edits, builds or runs.

## Verdict

Fix round needed. The sampled outcomes are true of their programs: the unit-graph defect rows and first-defect orderings (U06, U11a, U11d, U11i, U11j, U11k, U11l, U11g1, U11g3, U11o, U11w) give the code and cause of FR-142's row table, and the profile, header, alias, reference-literal and object cases match their ACs. The coverage list is not honest for FR-470, though, and omits nearly all of FR-154.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-470-AC-1, AC-2, AC-3, AC-7, AC-8 and AC-9 are claimed covered, but FR-470 governs domain clauses, "clause text that a domain package itself carries"; every listed case is a source-authored invariant in the unit, and the library model carries no clauses. For example, model-clause-header-in-clause-refused is refused because `model` is a reserved word in a source body | PR 79 body, Coverage FR-470-*; examples/declarations-models/example.toml |
| FND-002 | medium | FR-154 is listed only at FR level (5 cases); none of its ~53 ACs is in the coverage list or the not-covered paragraph, although its model-intake refusals are reachable by compiling a hand-written model | PR 79 body, Coverage FR-154 |
| FND-003 | medium | FR-002-AC-1 ("An exact import resolves a qualified domain type name to its stable declaration key") is claimed by declaration-forward-reference-admitted and alias-through-record-field-admitted, which involve no import; FR-002-AC-5 (`foreign_reference`) is neither covered nor listed | PR 79 body, Coverage FR-002-AC-1 |
| FND-004 | low | FR-610-AC-2 and AC-3 are claimed covered, while the not-covered paragraph lists FR-610-AC-2 to AC-4 as uncovered; object-duplicate-name-refused does not exercise AC-3 (node emission), whose testable part ("`ghost` refuses missing-name") is object-identity-unknown-field-refused | PR 79 body |
| FND-005 | low | FR-019-AC-4 (the complete collection facet, "including in unused declarations") is claimed by declaration-unused-body-still-checked, an integer-range proof in an unused function with no collection | examples/declarations-models/example.toml (case declaration-unused-body-still-checked) |
| FND-006 | low | Three rule files have no manifest entry: enum-label-duplicate-refused, function-measure-on-nonrecursive-admitted (same program as function-nonrecursive-needs-no-measure), record-empty-admitted-or-refused | examples/declarations-models/rules/ |

## Coverage

Examined (role examined): header-unknown-edition-refused, header-missing-refused, profile-unknown-refused, profile-use-site-wrong-alias-refused, profile-state-queries-predicate-admitted, profile-state-queries-function-refused, profile-state-core-function-refused, profile-state-core-filter-refused, type-alias-cycle-of-two-refused, declaration-forward-reference-admitted, function-duplicate-name-refused, record-duplicate-field-refused, enum-and-record-duplicate-name-refused, model-alias-duplicate-refused, model-not-supplied-refused, model-wrong-identity-refused, object-invariant-without-anchor-refused, unit-duplicate-name-refused, unit-and-record-of-one-name-coexist, unit-zero-scale-refused, unit-two-roots-refused, unit-cycle-reversed-source-order, unit-target-other-dimension-refused, unit-earliest-unit-checked-first, unit-two-roots-before-self-cycle, unit-first-dimension-refused-second-not-examined, unit-derived-dimension-without-base-unit-refused, unit-dimensionless-derived-dimension-admitted, dimension-unknown-term-before-cycle, dimension-self-cycle-refused, enum-empty-refused, record-empty-refused, tuple-single-component-admitted, model-reference-literal-chained-refused, model-reference-literal-types-as-reference, model-clause-header-in-clause-refused, object-field-population-type-refused, model-invariant-alias-independent, model-enum-case-unknown-refused, declaration-unused-body-still-checked, object-duplicate-name-refused, model-and-source-type-of-one-name-differ.

Not-covered classes judged: "no committed model can express units, namespaces or variant types" is a limit of the model generator, not of FSL examples. A hand-written model, or a generator extension, makes FR-142-AC-18 to AC-26, FR-141-AC-7/8, FR-143-AC-6/17, FR-470 and the TC-187 import rows coverable, so this needs a follow-up ticket rather than standing as "cannot be covered". Checked-package wire criteria (FR-610-AC-4, AC-7 to AC-9, FR-480-AC-3) cannot be reached by a source example; accepted. Accounting criteria (FR-142-AC-7, FR-143-AC-8, AC-14) need work limits; accepted.

Integration: only Makefile, README.md, example.toml and generated index.json besides the cases. No Python, no logic-bearing shell, no committed generator.
