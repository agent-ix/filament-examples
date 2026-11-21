---
type: reference
---

# Operations and arithmetic cases

Operations-arithmetic is a corpus for the operations of the FSL value language: integer division and range proofs, decimal and rational arithmetic and rounding, IEEE floating point, text-to-number conversion, collection operations, quantity arithmetic, and the refusals and undefined outcomes each of them states. Each case is a short source file, checked with `fsl check` or run through `fsl run`, and each compares its outcome with a recorded result. A case's statement says what the program does and what the standard requires of it.

Run the cases from the repository root:

```sh
cargo run --quiet --locked --manifest-path tools/fsl-examples/Cargo.toml -- check operations-arithmetic
```

Set `FSL` to the driver binary. To run a subset, add `--case 'rounding-*'`.

The corpus is driven by the FSL standard's expression requirements, not by a list of known defects. Each area has valid cases, invalid cases that name the refusal the standard requires, and boundary cases at the edge of a bound. The areas are:

- Integer arithmetic: result-range proofs for sums, differences, products and negation; `div`, `rem` and `mod` under the truncating, floor and Euclidean laws with negative operands, the law header and its refusals, nonzero-divisor proofs and the result type of `mod`.
- Decimal and rational arithmetic: exact alignment, multiplication and division, the six rounding modes on tie and non-tie values of both signs, strict `exact` refusal, range refusal, and rational reduction and range proofs.
- IEEE floating point: comparison, negation, equality, bit identity and total order vectors, rounding-mode differences, overflow, NaN propagation, signed zeros, fused multiply-add, strict `exact`, width conversions and refusals of mixed operands.
- Text to number: every numeric target with its accepted spellings, rounding and domain refusals, the `malformed-number` undefined outcome and the normalization profiles.
- Collections: `size`, `count` and `sum` with explicit result domains and prefix proofs, `filter`, `map`, `flatten`, `dropFirst`, `removeOne`, `contains`, `forall`, `exists`, `fold`, `reduce` and kind conversion for each collection kind.
- Quantities: addition, conversion, ordering, scaling, compound units, powers and affine units.
- Functions: `case` arm checking, total recursion with decreasing measures, short-circuit connectives and conditionals.

Cases that evaluate a value (`run`) all state a Boolean that must be true, except where the standard requires a refusal or an undefined outcome. An undefined outcome exits with 10. A staged case waits on a driver change and records the exit code the standard requires.

The source files carry requirement references. [example.toml](example.toml) lists each case and its expected exit code. [rules](rules) contains the submitted source, [requests](requests) the run requests, and [expected](expected) the recorded outcomes.

## Not covered here

These criteria of the area have no case in this system, and each belongs elsewhere or needs something source cannot supply.

- Accounting and charge-schedule criteria (FR-140-AC-6, FR-145-AC-8, FR-147-AC-6, FR-148-AC-7, FR-148-AC-13, FR-478-AC-5, FR-142-AC-7): they need evaluator limit vectors.
- Direct-kernel outcomes (FR-147-AC-9, FR-783-AC-4, the kernel half of FR-145-AC-16): the checker refuses these programs first, so a source cannot reach the runtime refusal. Area 7.
- Carried-sum value vectors (FR-142-AC-34, AC-37, AC-38) and the empty carried sum (FR-142-AC-36, value half): the sum values can be read only through a conversion back to a plain quantity, and the charge counts and the located undefined outcomes are kernel results. The linked admission and refusal rules of FR-142-AC-33, AC-35 and AC-36 are covered.
- FR-142-AC-10 (conversion with a compound side) and FR-148-AC-10's package refusal (`invalid_package`): the source grammar offers no way to name a compound unit as a conversion pivot or to bind a user declaration to a qualified intrinsic identity; the reserved-name case records the invalid-syntax refusal the driver gives instead.
- FR-570-AC-6 (a law supplied by a command option or request member): the request wire has no member for it in these cases. Area 7.
- FR-146-AC-18's remaining forms (`self.limit` chains and `+ 0` forms) and FR-146-AC-17's subtype and other-operation forms need object hierarchies with redefinition; model cases belong to area 4.
- Unavailable-occurrence criteria (FR-145-AC-13 to AC-15): area 7.

## Coverage

Each line gives a requirement or acceptance criterion of the FSL specification and the cases that exercise it.

- FR-002: let-binding-scope
- FR-005: option-present-and-value-guarded
- FR-006: boolean-connectives-short-circuit-truth, boolean-or-short-circuits-guarded-division, boolean-and-short-circuits-guarded-division, conditional-branches-checked-with-guard
- FR-007: int-add-range-widening, int-add-range-too-narrow, int-sub-range, int-mul-range, int-mul-range-too-narrow, int-unary-minus-range, integer-literal-largest-signed-128-bits
- FR-044: int-add-range-widening, int-add-range-too-narrow, int-sub-range, int-mul-range, int-mul-range-too-narrow, int-composed-range-proved, rational-add-reduced
- FR-091-AC-38: integer-literal-above-signed-128-bits
- FR-140: decimal-negate
- FR-140-AC-2: decimal-add-aligns-scales, decimal-multiply-adds-scales, decimal-exact-division-exact
- FR-140-AC-3: decimal-exact-division-inexact-refused, decimal-rounded-division-nearest-even, decimal-result-outside-range-refused, rounding-exact-integer-multiple-admitted, rounding-result-outside-range-refused
- FR-140-AC-4: rounding-nearest-even-tie-down, rounding-nearest-even-tie-up, rounding-nearest-away-positive-tie, rounding-nearest-away-negative-tie, rounding-toward-zero-positive, rounding-toward-zero-negative, rounding-toward-positive-positive, rounding-toward-positive-negative, rounding-toward-negative-positive, rounding-toward-negative-negative, rounding-exact-tie-refused, rounding-non-tie-nearest-even
- FR-140-AC-5: decimal-division-by-zero-refused, decimal-result-outside-range-refused, decimal-result-at-range-bound
- FR-142-AC-1: quantity-add-same-unit, quantity-subtract-same-unit, quantity-add-after-convert, quantity-convert-exact
- FR-142-AC-2: quantity-add-mixed-units-refused, quantity-convert-between-dimensions-refused, quantity-affine-addition-refused
- FR-142-AC-3: quantity-conversion-inexact-to-carrier-refused, quantity-conversion-nearest-even-rounds
- FR-142-AC-4: quantity-dimension-normalization-inverse
- FR-142-AC-5: quantity-affine-addition-refused, quantity-affine-conversion-composes, quantity-affine-ordering-compares-root-values
- FR-142-AC-6: quantity-multiplication-compound-unit, quantity-division-same-unit-dimensionless, quantity-division-different-units, quantity-power-zero-base-negative-exponent, quantity-power-zero-base-zero-exponent, quantity-power-integer, quantity-power-zero-exponent
- FR-142-AC-9: quantity-ordering-compares-values
- FR-142-AC-25: quantity-negation-keeps-unit
- FR-142-AC-28: quantity-conversion-exact-to-carrier-grid, quantity-conversion-inexact-to-carrier-refused
- FR-142-AC-29: quantity-conversion-above-carrier-bound-refused, quantity-conversion-at-carrier-bounds, quantity-conversion-above-int-carrier-bound-refused
- FR-142-AC-30: quantity-conversion-nearest-even-rounds
- FR-142-AC-33: carried-sum-admitted-over-carried-type, carried-sum-over-other-carrier-refused, carried-sum-over-plain-quantity-refused
- FR-142-AC-35: carried-sum-linked-prefix-unproved
- FR-142-AC-36: carried-sum-empty-nonzero-carrier-unproved, carried-sum-nonempty-nonzero-carrier-admitted
- FR-142-AC-40: integer-literal-above-signed-128-bits
- FR-142-AC-41: quantity-scale-by-integer-values
- FR-144-AC-3: convert-collection-above-bound-refused
- FR-144-AC-7: convert-set-to-sequence-uses-canonical-order
- FR-145: sum-empty-collection-zero, sum-left-fold-values, sum-prefix-leaves-domain, sum-prefix-at-domain-bound, contains-stops-and-empty-false, forall-exists-empty-cases, forall-exists-values
- FR-145-AC-1: filter-sequence-preserves-order, map-sequence-preserves-multiplicity, flatten-sequence-order
- FR-145-AC-2: fold-sequence-order-from-identity, fold-empty-returns-identity, reduce-sequence-order, reduce-empty-source-unproved, reduce-nonempty-source-admitted
- FR-145-AC-3: convert-sequence-to-set-loses-order-and-multiplicity, convert-sequence-to-bag-loses-order, convert-sequence-to-ordered-set-keeps-order, convert-set-to-sequence-uses-canonical-order
- FR-145-AC-4: count-sequence-counts-occurrences, count-bag-counts-occurrences, count-set-counts-members, sum-over-bag-multiplicity, sum-over-set-members, filter-sequence-preserves-order, filter-bag-preserves-multiplicity, map-sequence-preserves-multiplicity, map-set-coalesces-equal-results, map-bag-preserves-multiplicity, map-ordered-set-coalesces-later-results, flatten-sequence-order, flatten-bag-sums-multiplicities
- FR-145-AC-5: fold-set-commutative-step, fold-set-noncommutative-step-refused, fold-bag-noncommutative-step-refused
- FR-145-AC-6: filter-result-lower-bound-zero, flatten-sequence-of-set-refused
- FR-145-AC-7: convert-sequence-to-set-loses-order-and-multiplicity, convert-collection-element-type-refused, convert-collection-above-bound-refused
- FR-145-AC-9: fold-set-noncommutative-step-refused, fold-bag-noncommutative-step-refused
- FR-145-AC-10: drop-first-sequence-order, drop-first-empty-is-empty, drop-first-ordered-set, drop-first-set-refused, drop-first-bag-refused, drop-first-bound-shrinks, drop-first-bound-too-tight
- FR-145-AC-11: remove-one-sequence-first-occurrence, remove-one-bag-reduces-multiplicity, remove-one-set-drops-member, remove-one-absent-value-unchanged, remove-one-wrong-type-refused
- FR-145-AC-12: sum-empty-nonzero-domain-unproved, sum-nonempty-nonzero-domain-admitted
- FR-145-AC-16: count-into-narrow-domain-unproved, count-into-wide-domain-admitted, count-under-size-guard, count-under-weaker-size-guard, count-lower-bound-zero-unproved-for-positive-domain, count-literal-source-out-of-domain
- FR-146-AC-1: recursion-decreasing-measure
- FR-146-AC-2: recursion-no-measure-refused, recursion-measure-not-decreasing-refused
- FR-146-AC-3: function-wrong-argument-type-refused, function-wrong-result-type-refused
- FR-146-AC-4: recursion-mutual-decreasing, recursion-mutual-nondecreasing-refused
- FR-146-AC-7: recursion-no-measure-refused, recursion-measure-not-decreasing-refused
- FR-146-AC-9: rational-sum-range-unproved, rational-divisor-may-be-zero, rational-divisor-positive-admitted
- FR-146-AC-10: case-missing-arm-refused, case-duplicate-arm-refused, case-unknown-member-refused, case-arm-arity-refused
- FR-146-AC-11: case-selects-matching-arm, case-arm-body-checked-against-result
- FR-146-AC-12: case-result-type-from-declared-result, case-arms-of-different-kinds-refused, case-empty-sequence-arm-ambiguous
- FR-146-AC-14: case-scrutinee-not-union-refused
- FR-146-AC-15: recursion-size-measure-unchanged-argument, recursion-size-measure-removal-filter
- FR-146-AC-16: recursion-size-measure-dropfirst-with-fact (staged), recursion-size-measure-dropfirst-without-fact, recursion-size-measure-remove-one-with-membership (staged), recursion-size-measure-remove-one-without-membership
- FR-146-AC-17: postcondition-discharged-by-precondition, postcondition-without-precondition-unproved
- FR-146-AC-18: postcondition-precondition-strict-less-than, postcondition-precondition-equality-not-enough, postcondition-precondition-disjunction-not-enough, implication-guard-discharges-subtraction
- FR-146-AC-19: rational-product-interval-exact, rational-product-interval-too-narrow, rational-sum-interval-exact
- FR-146-AC-20: map-lookup-guard-forms-admitted, map-lookup-guard-mismatched-key-refused, map-lookup-guard-wrong-branch-refused
- FR-146-AC-21: argument-narrowing-guard-forms-admitted, argument-narrowing-guard-too-weak-refused, argument-narrowing-guard-different-sum-refused, argument-narrowing-guard-not-equal-refused
- FR-147: div-slash-is-not-integer-division, int-large-arithmetic-no-overflow
- FR-147-AC-1: div-rem-truncating-all-sign-combinations, div-rem-floor-all-sign-combinations, div-rem-euclidean-all-sign-combinations
- FR-147-AC-2: div-zero-divisor-range-refused, div-literal-zero-refused
- FR-147-AC-4: div-rem-truncating-all-sign-combinations, div-rem-floor-all-sign-combinations, div-rem-euclidean-all-sign-combinations
- FR-147-AC-5: mod-euclidean-whatever-the-law, mod-zero-divisor-range-refused
- FR-147-AC-10: div-unexposed-member-never-refuses-quotient, rem-unexposed-member-never-refuses-remainder
- FR-147-AC-11: div-without-complete-value-unsupported
- FR-147-AC-12: mod-result-type-bounded, mod-result-type-too-narrow, div-quotient-outside-domain
- FR-147-AC-13: div-zero-divisor-range-refused, rem-zero-divisor-range-refused, mod-zero-divisor-range-refused, div-literal-zero-refused, div-guard-proves-nonzero, conditional-branches-checked-with-guard
- FR-147-AC-14: mod-small-dividend-type, mod-singleton-dividend-type, mod-singleton-dividend-type-too-narrow
- FR-148-AC-1: float64-comparison-true-vectors, float64-comparison-false-vectors, float64-less-equal-same-operand (staged), float64-less-than-same-operand (staged)
- FR-148-AC-2: float64-addition-exact, float64-division-rounds-to-nearest-even, float64-division-rounding-mode-changes-bits
- FR-148-AC-4: float64-fused-multiply-add-single-rounding
- FR-148-AC-5: float64-bit-identity-distinguishes-signed-zero, float64-bit-identity-same-nan-bits, float64-total-order-signed-zero
- FR-148-AC-6: float64-overflow-gives-infinity, float64-nan-propagates, float64-strict-exact-refuses-rounding, float64-strict-exact-admits-exact-result, float64-square-root-exact
- FR-148-AC-8: float64-zero-minus-zero-sign, float64-strict-exact-refuses-rounding, float-width-mix-addition-refused, float-comparison-width-mix-refused, float-ieee-with-integer-refused
- FR-148-AC-9: float32-to-float64-widening-exact, float64-to-float32-narrowing-rounds, float32-nan-widening-keeps-sign-and-payload, rational-to-float64-exact, float64-to-rational-exact
- FR-148-AC-10: float-user-function-reserved-name-refused
- FR-148-AC-11: float64-negation-flips-sign-bit, float32-negation-flips-sign-bit, float64-double-negation-identity
- FR-148-AC-12: float64-comparison-true-vectors, float64-comparison-false-vectors, float32-comparison-vectors, float64-less-equal-same-operand (staged), float64-less-than-same-operand (staged), float32-greater-equal-same-operand (staged), float64-infinity-less-equal-same-operand (staged)
- FR-149-AC-27: rational-add-reduced, rational-equal-values-different-spelling
- FR-478-AC-1: text-to-integer-valid-spellings, text-to-integer-beyond-128-bits, text-to-bounded-integer-in-range, text-to-bounded-integer-above-range, text-to-bounded-integer-below-range, text-to-bounded-integer-at-upper-bound
- FR-478-AC-2: text-to-integer-empty, text-to-integer-plus-sign, text-to-integer-leading-zero, text-to-integer-negative-zero, text-to-integer-leading-space, text-to-integer-trailing-space, text-to-integer-thousands-separator, text-to-integer-exponent, text-to-integer-fraction, text-to-integer-ratio, text-to-integer-arabic-indic-digit, text-to-integer-fullwidth-digit, text-to-integer-not-a-number, text-to-integer-nfkc-normalizes-fullwidth, text-to-integer-nfc-keeps-fullwidth, text-to-decimal-leading-dot, text-to-decimal-trailing-dot, text-to-decimal-exponent, text-to-decimal-two-points
- FR-478-AC-3: text-to-number-targets-admitted, text-to-boolean-refused, text-to-enum-refused, text-to-text-refused, text-to-uuid-refused
- FR-478-AC-4: text-to-integer-not-a-number, text-to-bounded-integer-above-range
- FR-478-AC-6: text-to-decimal-exact-inexact-refused, text-to-decimal-exact-trailing-zero, text-to-decimal-short-spelling, text-to-decimal-out-of-domain, text-to-decimal-nearest-even-rounds, text-to-decimal-nearest-even-tie
- FR-478-AC-7: text-to-rational-reduces, text-to-rational-integer-spelling, text-to-rational-decimal-spelling, text-to-rational-denominator-out-of-domain, text-to-rational-numerator-out-of-domain, text-to-rational-zero-denominator, text-to-rational-negative-denominator, text-to-rational-spaced-slash
- FR-478-AC-8: text-to-float64-nearest-even, text-to-float64-toward-zero, text-to-float64-exactly-representable, text-to-float64-exponent-spelling, text-to-float64-negative-zero, text-to-float64-overflow-infinity, text-to-float64-overflow-toward-zero, text-to-float64-underflow-zero, text-to-float64-underflow-toward-positive, text-to-float64-infinity-spellings, text-to-float64-nan-spelling, text-to-float32-nearest-even
- FR-478-AC-9: text-to-float64-strict-exact-representable, text-to-float64-strict-exact-inexact-refused, text-to-float64-uppercase-exponent, text-to-float64-explicit-plus-exponent, text-to-float64-infinity-word, text-to-float64-hex-float
- FR-478-AC-10: text-to-quantity-bare-number, text-to-quantity-with-unit, text-to-quantity-fractional, text-to-quantity-other-unit, text-to-quantity-no-space, text-to-quantity-two-spaces, text-to-quantity-upper-case-unit
- FR-478-AC-11: text-to-number-from-string-operand, text-to-number-from-string-fullwidth-undefined
- FR-478-AC-12: text-to-bare-decimal-syntax, text-to-bare-rational-syntax, text-to-decimal-without-mode-syntax
- FR-570-AC-1: mod-euclidean-whatever-the-law
- FR-570-AC-2: div-without-law-refused, rem-without-law-refused, mod-result-type-bounded
- FR-570-AC-3: law-headers-conflict-refused, law-headers-repeated-select-once
- FR-570-AC-4: law-header-alias-used-refused
- FR-570-AC-5: law-header-without-division-admitted
- FR-783-AC-1: size-explicit-domain-checks, size-explicit-domain-too-narrow, size-explicit-domain-guarded, size-explicit-domain-minimum-one-needed
- FR-783-AC-2: size-type-argument-boolean-refused, size-type-argument-rational-refused, size-of-non-collection-refused, size-undeclared-type-refused
- FR-783-AC-3: size-values-sequence-and-set, size-alias-domain
