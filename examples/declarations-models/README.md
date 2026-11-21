---
type: reference
---

# Declarations and models cases

Declarations-models is a corpus for the declaration forms of FSL source and for source that is bound to a model. Each case is a short source file, checked with `fsl check` or compiled against a model with `fsl compile`, and each compares its outcome with a recorded result. A case's statement says what the program declares and what the standard requires of it.

Run the cases from the repository root:

```sh
cargo run --quiet --locked --manifest-path tools/fsl-examples/Cargo.toml -- check declarations-models
```

Set `FSL` to the driver binary. To run a subset, add `--case 'unit-*'`.

The corpus is driven by the FSL standard's declaration and model requirements, not by a list of known defects. Each area has valid cases, invalid cases that name the refusal the standard requires, and boundary cases at the edge of a rule. The areas are:

- Dimensions and units: the unit graph rules of the standard's unit table (one root per dimension, no cycles, no zero scale, targets of one dimension, derived dimensions that need a base unit), the order in which defects are reported, and the coexistence of a unit and a record of one name.
- Profile selection: the header, the unknown language, edition and profile refusals, the declaration forms each layer admits, and the missing selection of a `using` alias.
- Name resolution: duplicate declarations of every kind, missing imports, forward references, alias chains and cycles, and unresolved types.
- Declaration minimums: the smallest enumeration, record, tuple and union, and the refusal of an empty one.
- Object declarations: identity, fields of every type kind, `extends`, populations, operations and their `modifies` frames, and the invariant, precondition and postcondition clauses that bind to them.
- Models: source compiled against the library model, with clauses over model types, enumeration cases, model operations, reference literals, model and source types of one name, and the refusals for missing, duplicated and malformed package selections and for missing types, fields and operations. These cases exercise source-authored clauses; they do not exercise the clause text a domain package itself carries (FR-470).

The model cases use the [library model](../library/model/library.semantic-ir.json). The model generator cannot express units, namespaces, variant types or clauses carried by a domain package (QSL-2755), so the criteria below that need them are not covered here.

The source files carry requirement references. [example.toml](example.toml) lists each case and its expected exit code. [rules](rules) contains the submitted source, [requests](requests) the compile requests, and [expected](expected) the recorded outcomes.

## Not covered here

- FR-470 (names in clauses a domain package carries), AC-1 to AC-10: every clause in this system is written in the source unit, so none of them tests FR-470. They wait on QSL-2755.
- FR-154, the model intake criteria. Covered: AC-2 (a missing package and a different identity refuse as missing imports), AC-34 (no version or digest token), AC-35 (identity absent), AC-37 (one identity supplied twice). Not covered, waiting on QSL-2755 because they need hand-written or generated models the generator cannot write: AC-1, AC-3 to AC-33 (malformed nodes, identities, relationships, variants, field and parameter typing, units, constraints, dimensions, alias and clock forms), AC-36, AC-39 to AC-53 (option, rational, map, reference and value types). AC-5 and AC-47 are accounting and supplied-value criteria for areas 6 and 7; AC-38 is withdrawn.
- FR-002-AC-1 (an exact import resolves a qualified type name) and FR-002-AC-5 (`foreign_reference`): they need a second package or a foreign authority; QSL-2755.
- FR-142-AC-18 to AC-26, FR-141-AC-7 and AC-8, FR-143-AC-6 and AC-17, FR-610-AC-3 (node emission), AC-4, AC-7 and AC-9, FR-480-AC-3 to AC-5 (checked-package and wire criteria, or models with units, variants and universes): QSL-2755 or the checked-package tests. TC-187 rows U11m, U11q to U11u, U11x, U11y and U11z1 to U11z3 need imported dimensions.
- Accounting criteria (FR-142-AC-7, FR-143-AC-8 and AC-14): evaluator limit vectors.

## Coverage

Each line gives a requirement or acceptance criterion of the FSL specification and the cases that exercise it.

- FR-001: header-unknown-edition-refused, header-unknown-language-refused, header-missing-refused
- FR-001-AC-2: profile-unknown-refused
- FR-001-AC-4: profile-use-site-no-header-refused, profile-use-site-wrong-alias-refused
- FR-001-AC-5: profile-model-complete-admits-functions, profile-temporal-complete-admits-functions, profile-state-graph-function-refused, profile-state-queries-function-refused, profile-state-core-clause-admitted
- FR-001-AC-6: profile-state-core-function-refused, profile-state-queries-predicate-admitted, profile-state-core-predicate-refused, profile-state-queries-function-refused, profile-state-core-filter-refused
- FR-001-AC-8: profile-temporal-complete-admits-functions
- FR-002: declaration-forward-reference-admitted, type-alias-cycle-of-two-refused, type-alias-self-cycle-refused, type-alias-chain-admitted, function-calls-undeclared-function-refused, declaration-unused-admitted, model-invariant-alias-independent, alias-of-collection-admitted, alias-through-record-field-admitted, function-parameter-of-undeclared-record-refused, function-result-of-undeclared-type-refused
- FR-002-AC-2: import-missing-package-refused, model-not-supplied-refused, model-wrong-identity-refused
- FR-002-AC-3: function-duplicate-name-refused, record-duplicate-name-refused, type-duplicate-alias-refused, enum-and-record-duplicate-name-refused, record-duplicate-field-refused, union-duplicate-member-refused, function-duplicate-parameter-refused, object-duplicate-name-refused, model-alias-duplicate-refused
- FR-007: declaration-unused-body-still-checked
- FR-019: header-unknown-edition-refused, header-unknown-language-refused, header-missing-refused
- FR-019-AC-6: object-invariant-bound-anchor-admitted, object-invariant-unknown-field-refused, object-invariant-without-anchor-refused, object-invariant-on-unknown-object-refused, object-pre-post-on-operation-admitted, object-pre-on-unknown-operation-refused, model-invariant-on-model-type-compiles, model-pre-and-post-on-model-operation-compile, model-pre-on-unknown-operation-refused, model-invariant-without-anchor-refused
- FR-020: profile-two-aliases-of-one-definition-admitted
- FR-044: model-range-proof-over-field
- FR-141-AC-2: enum-declared-with-label-admitted
- FR-142-AC-1: unit-valid-graph-admitted
- FR-142-AC-13: unit-and-record-of-one-name-coexist, unit-duplicate-name-refused
- FR-142-AC-15: unit-zero-scale-refused, unit-two-roots-refused, unit-cycle-of-two-refused, unit-cycle-reversed-source-order, unit-self-cycle-refused, unit-target-missing-refused, unit-target-other-dimension-refused, unit-zero-scale-on-missing-target, unit-earliest-unit-checked-first, unit-two-roots-before-self-cycle, unit-first-dimension-refused-second-not-examined
- FR-142-AC-16: unit-derived-dimension-without-base-unit-refused, unit-derived-product-dimension-without-base-unit-refused, unit-derived-fourth-power-without-base-unit-refused, unit-dimensionless-derived-dimension-admitted, unit-derived-dimension-with-base-unit-admitted
- FR-142-AC-17: dimension-cycle-of-two-refused, dimension-self-cycle-refused, dimension-unknown-term-refused, dimension-unknown-term-before-cycle
- FR-143-AC-9: model-record-typed-function-parameter, model-reference-of-other-type-refused
- FR-143-AC-12: union-duplicate-member-refused
- FR-144-AC-9: alias-of-collection-admitted
- FR-146-AC-7: function-nonrecursive-needs-no-measure, function-call-to-record-refused
- FR-146-AC-10: union-single-member-admitted
- FR-149: model-field-wrong-type-comparison-refused
- FR-154-AC-2: model-invariant-on-model-type-compiles
- FR-154-AC-34: model-version-suffix-refused, model-digest-suffix-refused
- FR-154-AC-35: model-identity-absent-refused
- FR-154-AC-37: model-supplied-twice-refused
- FR-339: enum-empty-refused, tuple-empty-refused, record-empty-refused, model-clause-header-in-clause-refused, enum-single-member-admitted, record-single-field-admitted, tuple-single-component-admitted, union-single-member-admitted
- FR-480-AC-1: model-reference-literal-unquoted-refused
- FR-480-AC-2: model-reference-literal-types-as-reference, model-reference-literal-unknown-type-refused
- FR-480-AC-6: model-reference-literal-space-before-hash-refused, model-reference-literal-chained-refused
- FR-610-AC-1: object-declaration-with-members-admitted, object-extends-admitted, object-identity-unknown-field-refused, object-duplicate-field-refused
- FR-610-AC-2: model-and-source-type-of-one-name-differ
- FR-610-AC-3: object-identity-unknown-field-refused
- FR-610-AC-5: object-field-of-record-type-admitted, object-field-of-enum-and-option-type-admitted
- FR-610-AC-6: object-field-population-type-refused, object-field-of-unknown-type-refused, object-field-of-function-name-refused
- FR-610-AC-8: object-operation-modifies-unknown-field-refused
