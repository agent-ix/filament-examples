---
id: SR-1519
title: QSL-2103 area 2 types and checking corpus code review
type: SpecReview
analysis: code-review
scope: 'agent-ix/filament-examples; examples/types-checking/**, README.md, index.json; PR 76 at c2314fba07e4102e21939d4fb119c524df3b16f1'
review_set: subset
---

## Summary

Ticket QSL-2103. Reviewed the `types-checking` system added by PR 76 (192 running, 1 staged) against the FSpec standard at filament-specification origin/main. Read the full manifest and the PR coverage list; sampled 45 cases across every criterion group (FR-002, FR-005, FR-007, FR-140, FR-141, FR-142, FR-143, FR-144, FR-148, FR-149), the one staged case and every refusal code the manifest uses. No edits, builds or runs.

## Verdict

Fix round needed. Every sampled case's recorded outcome matches its program, and refusals pin the exact code and cause in `expected/*.json` (`first_diagnostic.code`/`cause`), not just an exit code. The text cases use genuinely different byte spellings (precomposed U+00E9 against e + U+0301). The equality, option, recursion, collection-bound and unit-expression cases match the AC texts quoted in the examined list. The defects are bookkeeping: the staged case waits on the corpus ticket instead of a product ticket, criteria are missing from both lists, there are orphan files, and one caption is wrong.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Staged `quantity-scale-by-division` names `waits_on = "QSL-2103"`, the corpus ticket itself; the defect is real and still on FSL main, and the QSL-2103 acceptance requires new bugs to be filed in the owning project | examples/types-checking/example.toml:1048-1056 |
| FND-002 | medium | Criteria the area names are in neither the coverage list nor the not-covered paragraph: FR-143-AC-5, AC-11, AC-16; FR-144-AC-11, AC-14, AC-15, AC-34; FR-149-AC-1, AC-2, AC-4, AC-5, AC-7, AC-8, AC-12, AC-13, AC-17, AC-18, AC-20 (FR-149-AC-9 and AC-11 are accounting) | PR 76 body, Coverage |
| FND-003 | low | Eight rule files and two request files have no manifest entry, so they never run: int-and-decimal-equality-conversion, int-and-rational-equality-conversion, int-range-and-integer-compare (same program as int-and-integer-implicit-equality-refused), quantity-divide-by-literal-zero, quantity-divide-by-unproved-nonzero, text-length-below-minimum, text-length-over-bound-nfc (same programs as the text-literal-* cases), text-types-of-different-bounds-differ (same program as text-types-of-different-bounds-compare) | examples/types-checking/rules/, examples/types-checking/requests/ |
| FND-004 | low | The `decimal-literal-coefficient-nonmember` statement blames the coefficient, but decimal(1234, 3) = 1.234 fails because it needs scale 3 > smax 1; the case is the AC's own vector and its outcome is right, only the caption is wrong | examples/types-checking/example.toml (case decimal-literal-coefficient-nonmember) |
| FND-005 | low | FR-141-AC-4 names the runtime refusal `text_length_out_of_domain`/`outside-domain`; every FR-141-AC-4 length case is a check-time literal refusal (`ill_typed`/`type-mismatch`), and the runtime refusal is not listed as uncovered or routed to area 7 | PR 76 body, Coverage FR-141-AC-4 |

## Coverage

Examined (role examined): decimal-literal-coefficient-nonmember, decimal-literal-scale-above-smax-nonmember, decimal-literal-retained-above-smax, decimal-literal-padded-member, decimal-type-scale-above-u32, decimal-type-scale-max-u32, decimal-type-coefficient-bounds-inverted, decimal-equality-ignores-scale, decimal-literal-ambiguous-let, decimal-type-without-mode, decimal-type-with-mode-checks, text-canonical-equivalents-equal-under-nfc, text-canonical-equivalents-differ-under-unicode-scalars, text-types-of-different-profiles-differ, text-types-of-different-bounds-compare, text-length-counted-after-normalization, text-literal-over-bound-nfc, text-literal-below-minimum, text-type-bounds-missing, text-type-profile-missing, enum-cross-declaration-equality-refused, enum-unordered-less-than-refused, quantity-equality-distinct-units-refused, quantity-return-converted-refused, quantity-times-float-refused, quantity-unit-expression-fractional-power, quantity-unit-unknown, quantity-carried-types-differ-by-carrier, quantity-argument-in-other-unit-refused, quantity-scale-by-integer, quantity-scale-by-division (staged), record-containment-cycle-through-optional-admitted, record-direct-containment-cycle, tuple-recursion-unnamed-edge-refused, record-recursion-through-nonempty-sequence-refused, union-recursion-without-escape-refused, option-absent-ambiguous-let, option-null-refused, option-some-out-of-bounds, option-value-unguarded, tuple-projection-out-of-arity, collection-kinds-differ, ordered-set-permutation-unequal, collection-exceeds-bound-literal, set-bound-counts-unique-members, set-drops-duplicate, float-equality-signed-zero-and-nan, int-and-integer-implicit-equality-refused, int-to-integer-equality-conversion, integer-to-int-equality-conversion-refused, ordering-int-and-integer-accepted, equality-integer-path-against-int-refused, int-literal-above-bound, arg-narrowing-unproved-range, collection-index-on-set-refused, collection-set-ordering-comparison-refused, unbounded-collection-admits-many.

Not-covered classes judged: accounting and charge-schedule criteria (FR-140-AC-6, FR-141-AC-6, FR-144-AC-8) need work limits a source case cannot set; they belong with the evaluator vectors or area 7 run requests, so the routing is acceptable. Model, unit-graph and declaration criteria go to area 4 (PR 79 covers FR-142-AC-13 to AC-17 only). Supplied-value criteria (FR-144-AC-16 to AC-30, FR-143-AC-15) go to area 7, which is correct. The FR-142-AC-42 refusals `d / 0` and `d / k` cannot be told apart from today's `type-mismatch` by exit code, so leaving them uncovered until FND-001's fix is correct; their rule files are orphans (FND-003).

Integration: only Makefile (two-line delegate to `$(FSL_EXAMPLES)`), README.md, example.toml and generated index.json besides the cases. No Python, no logic-bearing shell, no committed generator.
