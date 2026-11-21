---
id: SR-1520
title: QSL-2103 area 3 operations and arithmetic corpus code review
type: SpecReview
analysis: code-review
scope: 'agent-ix/filament-examples; examples/operations-arithmetic/**, README.md, index.json; PR 78 at 48edf15edd37d17738e84ea3df5d0ce8d289f730'
review_set: subset
---

## Summary

Ticket QSL-2103. Reviewed the `operations-arithmetic` system added by PR 78 (279 running, 7 staged) against the FSpec standard at filament-specification origin/main. Read the full manifest and coverage list; sampled 55 cases across FR-007, FR-044, FR-140, FR-142, FR-145, FR-146, FR-147, FR-148, FR-478, FR-570 and FR-783, every staged case and every refusal and undefined kind. No edits, builds or runs.

## Verdict

Fix round needed. One running case records a false Boolean under a "holds" verdict, and its statement contradicts FR-148. Three of the four staged product gaps are real defects; the fourth (integer literal above i128) is a specified refusal in FSL, not a defect. The division-law, rounding, decimal, quantity, collection, termination and text-conversion vectors that were sampled are correct, and refusals and undefined outcomes pin code, cause or reason in `expected/*.json`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `float32-nan-widening-keeps-sign-and-payload` expects 0xffc00005 to widen to 0xfff80000a0000000 (payload shifted up), but FR-148 keeps the integer payload: the required bits are 0xfff8000000000005. The recorded output is `{"boolean": false}` with exit 0 and verdict "holds", so the case passes while its program is false | examples/operations-arithmetic/rules/float32-nan-widening-keeps-sign-and-payload.fsl; expected/float32-nan-widening-keeps-sign-and-payload.json |
| FND-002 | medium | Six staged cases name `waits_on = "QSL-2103"` (the corpus ticket); the IEEE same-operand crash and the dropFirst/removeOne decrease forms are real defects still on FSL main and need tickets in the owning project | examples/operations-arithmetic/example.toml:787-833, 2413-2442 |
| FND-003 | medium | Staged `integer-literal-above-signed-128-bits` expects exit 0, but FSL FR-091-AC-38 requires exactly this literal to refuse `IntegerOutsideI128`, `ill_typed`/`type-mismatch`, and FSpec FR-142-AC-40 keeps "existing integer and frontend ceilings" as typed refusals; the case should be a running boundary refusal | examples/operations-arithmetic/example.toml:1519-1527 |
| FND-004 | medium | Source-reachable criteria are in neither list: FR-142-AC-3, AC-4, AC-10, AC-28, AC-29, AC-30, AC-33 to AC-38 (conversion loss, carried quantities and carried sums); FR-146-AC-12, AC-17 to AC-21; FR-147-AC-10 (`10 div y` at y = 5 over Int[1, 10] is 2, a plain run case), AC-11; FR-148-AC-10; FR-145-AC-8; FR-478-AC-4; FR-570-AC-6 | PR 78 body, Coverage |
| FND-005 | low | Six rule files and three request files have no manifest entry: count-runtime-out-of-domain and size-explicit-domain-runtime-refusal (kernel-only by FR-145-AC-16 and FR-783-AC-4, so they can never run from source), div-in-unreachable-branch-checked, recursion-structural-measure-over-sequence, quantity-add-mixed-units-in-first-unit, float-user-function-reserved-name-refused (the FR-148-AC-10 case, which should be wired) | examples/operations-arithmetic/rules/, requests/ |
| FND-006 | low | `mod-needs-no-law-header` and `mod-result-type-bounded` have the same program; collapse them into one case listed under FR-570-AC-2 and FR-147-AC-12 | examples/operations-arithmetic/example.toml |
| FND-007 | low | Positive-value vector families can be one conjunction per law or AC without losing coverage: the 12 div/rem sign cases become 3 cases (one per law header), and text-to-integer-zero/positive/negative become one case. Undefined and refusal vectors must stay separate, because each program yields only its first outcome | examples/operations-arithmetic/example.toml |

## Coverage

Examined (role examined): div-truncating-negative-dividend, div-floor-negative-divisor, div-euclidean-both-negative, mod-euclidean-whatever-the-law, div-slash-is-not-integer-division, div-literal-zero-refused, div-quotient-outside-domain, mod-needs-no-law-header, mod-result-type-bounded, int-mul-range-too-narrow, int-large-arithmetic-no-overflow, integer-literal-largest-signed-128-bits, integer-literal-above-signed-128-bits (staged), rounding-nearest-even-tie-down, rounding-nearest-away-negative-tie, rounding-toward-negative-negative, rounding-exact-tie-refused, decimal-exact-division-inexact-refused, decimal-result-outside-range-refused, decimal-division-by-zero-refused, quantity-power-zero-base-negative-exponent, quantity-affine-addition-refused, quantity-affine-ordering-compares-root-values, quantity-scale-by-integer-values, fold-set-noncommutative-step-refused, reduce-empty-source-unproved, sum-empty-nonzero-domain-unproved, sum-prefix-leaves-domain, map-ordered-set-coalesces-later-results, convert-set-to-sequence-uses-canonical-order, drop-first-bag-refused, drop-first-bound-too-tight, count-under-weaker-size-guard, case-missing-arm-refused, recursion-mutual-nondecreasing-refused, recursion-size-measure-dropfirst-with-fact (staged), recursion-size-measure-remove-one-with-membership (staged), float64-fused-multiply-add-single-rounding, float64-to-float32-narrowing-rounds, float32-nan-widening-keeps-sign-and-payload, float64-strict-exact-refuses-rounding, float64-nan-propagates, float64-division-rounding-mode-changes-bits, float64-comparison-false-vectors, float32-comparison-vectors, float64-less-equal-same-operand (staged), float64-less-than-same-operand (staged), float32-greater-equal-same-operand (staged), float64-infinity-less-equal-same-operand (staged), text-to-integer-leading-zero, text-to-integer-negative-zero, text-to-integer-nfkc-normalizes-fullwidth, text-to-integer-beyond-128-bits, text-to-decimal-nearest-even-tie, text-to-rational-negative-denominator, text-to-float64-overflow-toward-zero, text-to-float64-underflow-toward-positive, text-to-float64-hex-float, text-to-quantity-fractional, text-to-boolean-refused, law-headers-conflict-refused, law-header-alias-used-refused, div-without-law-refused.

Not-covered classes judged: FR-147-AC-9 (`division_out_of_domain` at runtime) and FR-783-AC-4 / FR-145-AC-16's kernel half are stated as "direct kernel evaluation", so they cannot be reached from source; the classification is correct. FR-147-AC-10 is wrongly grouped with them: it is a value outcome reachable by a run request (FND-004). Accounting criteria (FR-140-AC-6, FR-147-AC-6, FR-148-AC-7, AC-13, FR-478-AC-5) need work limits; the routing is acceptable. Unavailable-occurrence criteria (FR-145-AC-13 to AC-15) go to area 7, which is correct.

Integration: only Makefile, README.md, example.toml and generated index.json besides the cases. No Python, no logic-bearing shell, no committed generator.
