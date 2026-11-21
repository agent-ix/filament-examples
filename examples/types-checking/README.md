---
type: reference
---

# Types and checking cases

Types-checking is a corpus for the FSL type checker and the value types it admits. Each case is a short source file, checked with `fsl check` or run through `fsl run`, and each compares its outcome with a recorded result. A case's statement says what the program does and what the standard requires of it.

Run the cases from the repository root:

```sh
cargo run --quiet --locked --manifest-path tools/fsl-examples/Cargo.toml -- check types-checking
```

Set `FSL` to the driver binary. To run a subset, add `--case 'decimal-*'`.

The corpus is driven by the FSL standard's type-model and equality requirements, not by a list of known defects. Each area has valid cases, invalid cases that name the refusal the standard requires, and boundary cases at the edge of a bound. The areas are:

- Decimal and rational types: well-formed bounds, membership by value, literal typing, ambiguous literals, and the required rounding mode.
- Integer, text and enumeration types: bounds, text profiles and length, ordered and unordered enumerations.
- Records, tuples, unions and options: structural identity, construction, projection, recursion through named edges, absent and `Some`.
- Collections: bounds as part of type identity, occurrence and unique-member counting, permutation and duplicate behavior, indexing and map lookup guards.
- Quantities: unit typing of arguments and results, compound units, unit-expression syntax, scaling, carried quantities.
- Equality admission: which operand types compare, the closed set of equality conversions, integer widening at declared positions, and the native kinds (UUID, Timestamp, Bytes, JsonObject).

Cases that evaluate a value (`run`) all state a Boolean that must be true. A staged case waits on a driver change and records the exit code the standard requires.

The source files carry requirement references. [example.toml](example.toml) lists each case and its expected exit code. [rules](rules) contains the submitted source, [requests](requests) the run requests, and [expected](expected) the recorded outcomes.

## Not covered here

These criteria of the area have no case in this system, and each belongs elsewhere.

- Accounting and charge-schedule criteria (FR-140-AC-6, FR-141-AC-6, FR-144-AC-8, FR-149-AC-9, FR-149-AC-11): they need evaluator limit vectors, not source cases.
- FR-141-AC-4's runtime refusal `text_length_out_of_domain`/`outside-domain`: a source cannot reach it, because a literal over the bound is refused at check time as `type-mismatch`. It belongs to the run-input cases (area 7).
- Supplied-value criteria (FR-144-AC-16 to AC-30, FR-143-AC-15) and the evaluation of `s[-1]`-style undefined outcomes (FR-144-AC-34): area 7.
- Model-bound criteria (FR-143-AC-6, AC-11, AC-16 on selection identity, FR-144-AC-11, FR-149-AC-17 on record extension, FR-143-AC-5, FR-149-AC-18 on event types, FR-144-AC-15): they need a model with record conformance, event types or reference keys that the model generator cannot write. Area 4 and QSL-2755.
- FR-144-AC-14 (canonical visiting order of union members): the visiting order is observable only through a traversal that source cannot print; area 3 covers set and bag canonical order with integers.
- FR-149-AC-1, AC-4, AC-5, AC-7, AC-8: matrix-wide vectors are covered row by row by the equality cases below; FR-149-AC-5 and AC-7 need evaluator state, and AC-8 (accounting of set equality) needs limit vectors.
- Arithmetic, rounding, division, conversion and collection-operation criteria: area 3. Unit-graph, model and declaration criteria: area 4.

## Coverage

Each line gives a requirement or acceptance criterion of the FSL specification and the cases that exercise it.

- FR-002: type-alias-unknown-name, type-alias-transparent
- FR-005: option-value-unguarded, option-value-guarded
- FR-007: int-type-bounds-inverted, int-type-single-point, int-literal-above-bound, int-literal-at-upper-bound, int-literal-below-bound
- FR-140-AC-1: decimal-equality-ignores-scale
- FR-140-AC-5: decimal-literal-padded-member, decimal-type-coefficient-bounds-inverted, decimal-type-scale-bounds-inverted, decimal-type-scale-max-u32, decimal-type-scale-above-u32, decimal-type-single-point
- FR-140-AC-7: decimal-literal-ambiguous-let, decimal-literal-ambiguous-equality, decimal-literal-takes-other-operand-type, decimal-distinct-types-refused
- FR-140-AC-8: decimal-literal-scale-above-smax-nonmember, decimal-literal-coefficient-nonmember, decimal-literal-retained-above-smax
- FR-140-AC-9: decimal-type-without-mode, decimal-type-with-mode-checks
- FR-141-AC-1: text-types-of-same-profile-compare, text-canonical-equivalents-equal-under-nfc, text-canonical-equivalents-differ-under-unicode-scalars
- FR-141-AC-2: enum-equality-same-declaration, enum-cross-declaration-equality-refused
- FR-141-AC-3: text-type-bounds-missing, text-type-profile-missing, text-type-bounds-inverted, enum-unknown-member, enum-duplicate-member
- FR-141-AC-4: text-types-of-different-profiles-differ, text-types-of-different-bounds-compare, text-length-counted-after-normalization, text-literal-over-bound-nfc, text-length-at-bound, text-literal-below-minimum, text-lexicographic-order
- FR-141-AC-5: enum-cross-declaration-equality-refused, enum-unordered-less-than-refused, enum-ordered-less-than, enum-ordered-cross-declaration-refused, enum-ordered-compare-run
- FR-142-AC-1: quantity-convert-between-units
- FR-142-AC-2: quantity-incompatible-dimensions-refused
- FR-142-AC-9: quantity-equality-distinct-units-refused, quantity-equality-same-unit
- FR-142-AC-11: quantity-argument-in-declared-unit, quantity-argument-in-other-unit-refused
- FR-142-AC-12: quantity-return-converted-refused
- FR-142-AC-27: quantity-carried-distinct-from-plain, quantity-carried-types-differ-by-carrier
- FR-142-AC-31: quantity-carried-rational-type
- FR-142-AC-39: quantity-compound-type-position
- FR-142-AC-40: quantity-unit-expression-empty, quantity-unit-expression-dangling-star, quantity-unit-expression-fractional-power, quantity-unit-unknown
- FR-142-AC-41: quantity-scale-by-integer, quantity-scale-by-division (staged)
- FR-142-AC-42: quantity-times-float-refused
- FR-143: record-field-order-irrelevant
- FR-143-AC-1: record-structural-equality, record-structural-equality-run, tuple-structural-equality-run
- FR-143-AC-2: record-equal-shape-different-type, tuple-equal-shape-different-type
- FR-143-AC-3: record-direct-containment-cycle, record-mutual-containment-cycle
- FR-143-AC-4: record-missing-field-refused, record-extra-field-refused, record-field-wrong-type, tuple-projection-out-of-arity, tuple-construction-wrong-arity
- FR-143-AC-7: record-direct-containment-cycle, record-containment-cycle-through-optional-admitted, record-mutual-containment-cycle, tuple-recursion-unnamed-edge-refused, record-recursion-through-sequence-admitted, record-recursion-through-nonempty-sequence-refused, record-recursion-through-optional-admitted
- FR-143-AC-9: reference-of-non-object-refused
- FR-143-AC-10: option-value-unguarded, option-optional-projection-unguarded
- FR-143-AC-12: union-construction-one-member, union-construction-wrong-arity, union-construction-unknown-member, union-equality-member-and-payload, union-equal-member-spellings-different-unions, union-equality-run
- FR-143-AC-13: union-recursion-escaping-admitted, union-recursion-without-escape-refused
- FR-143-AC-18: option-absent-with-expected-type
- FR-143-AC-19: option-absent-ambiguous-let, option-absent-ambiguous-equality, option-absent-in-present, option-value-of-absent-ambiguous
- FR-143-AC-20: option-optional-field-absent-literal-refused, option-optional-field-omitted, option-null-refused
- FR-143-AC-21: option-some-expected-type, option-some-out-of-bounds
- FR-143-AC-22: tuple-projection-typed, tuple-projection-wrong-type
- FR-144-AC-1: collection-same-bound-equality, collection-kinds-differ, set-permutation-equal, bag-permutation-equal, sequence-permutation-unequal, ordered-set-permutation-unequal
- FR-144-AC-2: set-drops-duplicate, sequence-keeps-duplicate, bag-keeps-duplicate, ordered-set-drops-duplicate
- FR-144-AC-3: collection-bound-inverted, collection-exceeds-bound-literal, collection-below-minimum-literal, collection-at-maximum-literal, collection-at-minimum-literal, empty-collection-at-zero-minimum
- FR-144-AC-4: set-permutation-equal, bag-permutation-equal
- FR-144-AC-5: collection-set-dedup-within-bound, collection-sequence-counts-duplicates, collection-bag-counts-duplicates, set-bound-counts-unique-members
- FR-144-AC-6: collection-set-of-float-admitted, nested-collection-element-types, float-set-total-order-members-run
- FR-144-AC-7: collection-set-ordering-comparison-refused
- FR-144-AC-9: collection-bound-part-of-type, collection-different-bounds-differ
- FR-144-AC-10: sequence-literal-without-expected-type, sequence-literal-with-annotation
- FR-144-AC-12: unbounded-collection-admits-many
- FR-144-AC-13: collection-bound-part-of-type, collection-different-bounds-differ
- FR-144-AC-31: collection-index-guarded
- FR-144-AC-32: collection-index-unguarded, collection-index-on-set-refused
- FR-144-AC-33: collection-map-type, collection-map-lookup-unguarded, collection-map-lookup-guarded
- FR-148-AC-8: float-types-widths-differ, float-types-rounding-modes-differ, float-ieee-and-exact-mix
- FR-149-AC-2: reference-equality-same-type
- FR-149-AC-3: integer-and-text-differ, boolean-and-integer-differ
- FR-149-AC-6: option-and-payload-compare, option-equality-same-type, option-absent-differs-from-present
- FR-149-AC-10: int-and-integer-implicit-equality-refused, int-to-integer-equality-conversion, integer-to-int-equality-conversion-refused, int-to-rational-equality-conversion, int-to-rational-equality-conversion-too-narrow, int-to-decimal-equality-conversion, int-to-decimal-equality-conversion-too-narrow, float-and-int-equality-refused
- FR-149-AC-12: reference-upcast-to-supertype-compares, reference-nonconforming-pair-refused, reference-downcast-comparison-checks-by-supertype-only
- FR-149-AC-13: float-ordering-rounding-modes-differ, float-ordering-against-exact-type, float-ordering-widths-differ, float-total-order-same-type-admitted
- FR-149-AC-14: uuid-equality-same-kind
- FR-149-AC-15: uuid-and-timestamp-differ, uuid-versus-integer, timestamp-versus-boolean
- FR-149-AC-16: uuid-ordering-operator-ineligible, timestamp-ordering-operator-ineligible
- FR-149-AC-19: sum-equality-guarded-range-checks, field-sum-equality-guarded-admitted
- FR-149-AC-20: field-sum-equality-unguarded-refused, field-sum-equality-same-field-refused
- FR-149-AC-21: ordering-int-and-integer-accepted, equality-integer-path-against-int-refused, equality-unary-minus-against-int-refused
- FR-149-AC-22: string-and-text-differ, string-equality-checks, string-never-normalized
- FR-149-AC-23: bytes-equality-same-kind
- FR-149-AC-24: bytes-versus-string, json-object-versus-boolean, bytes-versus-json-object
- FR-149-AC-25: float-equality-signed-zero-and-nan, float-record-leaf-numeric-equality
- FR-149-AC-26: float-set-total-order-members-run
- FR-149-AC-27: rational-type-well-formed, rational-type-numerator-bounds-inverted, rational-type-denominator-zero, rational-type-denominator-negative, rational-literal-member, rational-literal-over-numerator, rational-literal-over-denominator
- FR-149-AC-28: map-equality-same-type
- FR-149-AC-29: sum-equality-unproved-range
- FR-149-AC-30: arg-widening-integer-parameter, arg-narrowing-unproved-range, arg-narrowing-from-integer-unproved-range, arg-narrowing-proved-by-guard, union-payload-widening, union-payload-narrowing-unproved, some-integer-range-no-widening, int-and-integer-equality-refused-x-n
- FR-149-AC-31: tuple-position-widening, tuple-position-narrowing-unproved
