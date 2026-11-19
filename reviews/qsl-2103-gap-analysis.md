---
id: SR-010
title: QSL-2103 grammar corpus gap analysis
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/filament-examples; examples/grammar/**, README.md, index.json; PR 51 at a888184f8919d1fcce6377b2d708b4f48cbf2c83'
review_set: subset
---

## Summary

Ticket QSL-2103. Reviewed the new corpus against its area deliverable and supplied grammar rulings. No missing or hollow case within the claimed initial area was identified.

## Verdict

PASS for this PR's area. Existing matrix output from the lead describes the compiler repository, not a runnable-test coverage claim for this examples repository. No new matrix was run: this repository has no spec document root, and the change contains cases rather than production implementation. Source comment references are documentation, not criterion verification tags; no coverage percentage is claimed from them.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Plan completion: not assessed

The acceptance slice supplies valid, invalid and boundary source cases through the existing Rust runner, records their outcomes, and identifies the synthesis implementation owner QSL-2235. New parser, evaluation and refusal cases can fail on real driver/parser/evaluator changes. The ordinary and staged paths remain separate; the staged case has a future success exit distinct from its present refusal. This is a diff-scoped manual requirement-to-case and oracle review; repository-wide matrix coverage and full language completeness were not assessed. No Rust implementation changed, so rust-review and Rust build/lint gates are not applicable to this diff. No spec or plan artifact changed, so spec-review is not applicable. No test execution was repeated; accepted coder results are recorded in SR-009.

## Examined units

```json
{
  "scope": [
    {
      "id": "header-only",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A source containing only its header parses; package completeness is checked later."
    },
    {
      "id": "unit-no-declarations",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A source containing a header and profile parses without a declaration."
    },
    {
      "id": "trailing-garbage",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "An identifier that begins no declaration form is reported as unsupported."
    },
    {
      "id": "synthesis-objective",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A synthesis declaration admits an objective before satisfies."
    },
    {
      "id": "type-rounding-exact",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "The exact rounding spelling parses in a decimal type."
    },
    {
      "id": "type-rounding-toward-zero",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "The toward-zero rounding spelling parses in a decimal type."
    },
    {
      "id": "type-rounding-toward-positive",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "The toward-positive rounding spelling parses in a decimal type."
    },
    {
      "id": "type-rounding-toward-negative",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "The toward-negative rounding spelling parses in a decimal type."
    },
    {
      "id": "type-rounding-nearest-even",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "The nearest-even rounding spelling parses in a decimal type."
    },
    {
      "id": "type-rounding-nearest-away",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "The nearest-away rounding spelling parses in a decimal type."
    },
    {
      "id": "type-text-unicode-scalars",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "The unicode-scalars text profile parses in a text type."
    },
    {
      "id": "type-text-nfc",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "The nfc text profile parses in a text type."
    },
    {
      "id": "type-text-nfd",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "The nfd text profile parses in a text type."
    },
    {
      "id": "type-text-nfkc",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "The nfkc text profile parses in a text type."
    },
    {
      "id": "type-text-nfkd",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "The nfkd text profile parses in a text type."
    },
    {
      "id": "type-text-binary-utf8",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "The binary-utf8 text profile parses in a text type."
    },
    {
      "id": "type-population",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A population maximum uses one unsigned bound."
    },
    {
      "id": "type-quantity-compound",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Compound unit factors take signed integer powers."
    },
    {
      "id": "object-members",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Source objects admit identity, fields, population, and operation members."
    },
    {
      "id": "model-import",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Imports precede model declarations and function declarations."
    },
    {
      "id": "temporal-eventually-bounded",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A bounded eventually temporal operator parses."
    },
    {
      "id": "temporal-eventually-unbounded",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A unbounded eventually temporal operator parses."
    },
    {
      "id": "temporal-always-bounded",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A bounded always temporal operator parses."
    },
    {
      "id": "temporal-always-unbounded",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A unbounded always temporal operator parses."
    },
    {
      "id": "temporal-once-bounded",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A bounded once temporal operator parses."
    },
    {
      "id": "temporal-once-unbounded",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A unbounded once temporal operator parses."
    },
    {
      "id": "temporal-historically-bounded",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A bounded historically temporal operator parses."
    },
    {
      "id": "temporal-historically-unbounded",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A unbounded historically temporal operator parses."
    },
    {
      "id": "temporal-until",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A bounded until temporal relation parses."
    },
    {
      "id": "temporal-release",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A bounded release temporal relation parses."
    },
    {
      "id": "temporal-since",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A bounded since temporal relation parses."
    },
    {
      "id": "temporal-triggered",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A bounded triggered temporal relation parses."
    },
    {
      "id": "temporal-activation-capture",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Temporal activation, filtering, clock, and capture syntax compose."
    },
    {
      "id": "temporal-fairness",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A temporal body admits fairness before its formula."
    },
    {
      "id": "protocol-sequence",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A protocol sequence contains a named check."
    },
    {
      "id": "protocol-choice",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A choice has two guarded control branches."
    },
    {
      "id": "protocol-join-all",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Parallel branches join with all."
    },
    {
      "id": "protocol-join-any",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Parallel branches join with any."
    },
    {
      "id": "protocol-join-quorum1",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Parallel branches join with quorum(1)."
    },
    {
      "id": "protocol-fence-acquire",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A protocol fence takes acquire ordering."
    },
    {
      "id": "protocol-fence-release",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A protocol fence takes release ordering."
    },
    {
      "id": "protocol-fence-acq_rel",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A protocol fence takes acq_rel ordering."
    },
    {
      "id": "protocol-fence-seq_cst",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A protocol fence takes seq_cst ordering."
    },
    {
      "id": "channel-delivery-unknown",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A channel admits delivery policy unknown."
    },
    {
      "id": "channel-delivery-at-most-once",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A channel admits delivery policy at-most-once."
    },
    {
      "id": "channel-delivery-at-least-once",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A channel admits delivery policy at-least-once."
    },
    {
      "id": "channel-delivery-exactly-once-premise",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A channel admits delivery policy exactly-once-premise."
    },
    {
      "id": "channel-delivery-0-2",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A channel admits delivery policy [0, 2]."
    },
    {
      "id": "state-invariant",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A state invariant has an explicit current anchor."
    },
    {
      "id": "state-pre-post",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Preconditions and postconditions name an operation."
    },
    {
      "id": "graph-possible",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A possible claim admits a model binding and temporal graph predicate."
    },
    {
      "id": "graph-unique-path",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A unique-path claim names source and target predicates."
    },
    {
      "id": "verification-plan",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A verification plan names claim, method, domain, and bound."
    },
    {
      "id": "temporal-missing-over",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A temporal declaration without its over binding is refused."
    },
    {
      "id": "temporal-malformed-interval",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "An interval needs a comma between its bounds."
    },
    {
      "id": "protocol-empty-join",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A parallel join list cannot be empty."
    },
    {
      "id": "boolean-true",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "boolean true"
    },
    {
      "id": "boolean-false",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "boolean false"
    },
    {
      "id": "integer-zero",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "integer zero"
    },
    {
      "id": "integer-large",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "integer large"
    },
    {
      "id": "integer-negative",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "integer negative"
    },
    {
      "id": "unary-chain",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "unary chain"
    },
    {
      "id": "decimal-exact",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "decimal exact"
    },
    {
      "id": "rational-signed",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "rational signed"
    },
    {
      "id": "text-comment-markers",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "text comment markers"
    },
    {
      "id": "text-json-escapes",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "text json escapes"
    },
    {
      "id": "text-unicode-escape",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "text unicode escape"
    },
    {
      "id": "text-unicode-scalar",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "text unicode scalar"
    },
    {
      "id": "arithmetic-grouping",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "arithmetic grouping"
    },
    {
      "id": "arithmetic-product",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "arithmetic product"
    },
    {
      "id": "arithmetic-left-chain",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "arithmetic left chain"
    },
    {
      "id": "division-rem-mod",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "division rem mod"
    },
    {
      "id": "boolean-precedence",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "boolean precedence"
    },
    {
      "id": "implication-right",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "implication right"
    },
    {
      "id": "comparison-equality",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "comparison equality"
    },
    {
      "id": "comparison-not-equal",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "comparison not equal"
    },
    {
      "id": "comparison-less-equal",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "comparison less equal"
    },
    {
      "id": "comparison-greater-equal",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "comparison greater equal"
    },
    {
      "id": "comparison-less",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "comparison less"
    },
    {
      "id": "comparison-greater",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "comparison greater"
    },
    {
      "id": "negated-comparison",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "negated comparison"
    },
    {
      "id": "conditional",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "conditional"
    },
    {
      "id": "nested-conditional",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "nested conditional"
    },
    {
      "id": "let-binding",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "let binding"
    },
    {
      "id": "let-annotated",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "let annotated"
    },
    {
      "id": "let-nested",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "let nested"
    },
    {
      "id": "call-zero-arguments",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "call zero arguments"
    },
    {
      "id": "call-multiple-arguments",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "call multiple arguments"
    },
    {
      "id": "qualified-reference",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "qualified reference"
    },
    {
      "id": "qualified-call",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "qualified call"
    },
    {
      "id": "field-selection",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "field selection"
    },
    {
      "id": "index-selection",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "index selection"
    },
    {
      "id": "member-reserved-word",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "member reserved word"
    },
    {
      "id": "member-field-reserved",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "member field reserved"
    },
    {
      "id": "sequence-literal",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "sequence literal"
    },
    {
      "id": "empty-sequence",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "empty sequence"
    },
    {
      "id": "set-literal",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "set literal"
    },
    {
      "id": "bag-literal",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "bag literal"
    },
    {
      "id": "ordered-set-literal",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "ordered set literal"
    },
    {
      "id": "record-literal",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "record literal"
    },
    {
      "id": "option-some",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "option some"
    },
    {
      "id": "option-none",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "option none"
    },
    {
      "id": "option-absent",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "option absent"
    },
    {
      "id": "case-union",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "case union"
    },
    {
      "id": "reference-solid",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "reference solid"
    },
    {
      "id": "reference-empty-key",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "reference empty key"
    },
    {
      "id": "text-constructor",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "text constructor"
    },
    {
      "id": "typed-conversion",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "typed conversion"
    },
    {
      "id": "quantifier-forall",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "quantifier forall"
    },
    {
      "id": "quantifier-exists",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "quantifier exists"
    },
    {
      "id": "query-filter",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "query filter"
    },
    {
      "id": "query-map",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "query map"
    },
    {
      "id": "query-size",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "query size"
    },
    {
      "id": "query-contains",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "query contains"
    },
    {
      "id": "query-count",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "query count"
    },
    {
      "id": "query-sum",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "query sum"
    },
    {
      "id": "lookup-absence",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "lookup absence"
    },
    {
      "id": "comments-before-between-after",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Line comments separate tokens without consuming adjacent syntax."
    },
    {
      "id": "whitespace-tabs",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Tabs separate tokens."
    },
    {
      "id": "line-endings-crlf",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "CRLF source parses without changing line boundaries."
    },
    {
      "id": "identifier-keyword-prefix",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A keyword prefix does not split a longer identifier."
    },
    {
      "id": "identifier-underscore",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "An ASCII underscore and following digit are valid identifier characters."
    },
    {
      "id": "identifier-helper",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "The spelling helper is an ordinary function identifier under 1-draft."
    },
    {
      "id": "identifier-rec",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "The spelling rec is an ordinary function identifier under 1-draft."
    },
    {
      "id": "identifier-cast",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "The spelling cast is an ordinary function identifier under 1-draft."
    },
    {
      "id": "identifier-tuple",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "The spelling Tuple is an ordinary function identifier under 1-draft."
    },
    {
      "id": "declarations-value-types",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Value declarations compose in one source unit."
    },
    {
      "id": "declarations-generic-types",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Nested type arguments and collection bounds parse."
    },
    {
      "id": "declarations-dimensions-units",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Dimensions and units have distinct declaration forms."
    },
    {
      "id": "default-profile",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "A function can omit the using selector."
    },
    {
      "id": "comparison-chain-less",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: comparison chain less."
    },
    {
      "id": "comparison-chain-mixed",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: comparison chain mixed."
    },
    {
      "id": "comparison-chain-sum",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: comparison chain sum."
    },
    {
      "id": "integer-leading-zero",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: integer leading zero."
    },
    {
      "id": "integer-hex-shortcut",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: integer hex shortcut."
    },
    {
      "id": "number-malformed-exponent",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: number malformed exponent."
    },
    {
      "id": "string-invalid-escape",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: string invalid escape."
    },
    {
      "id": "string-unpaired-high-surrogate",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: string unpaired high surrogate."
    },
    {
      "id": "string-unpaired-low-surrogate",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: string unpaired low surrogate."
    },
    {
      "id": "string-unclosed",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: string unclosed."
    },
    {
      "id": "block-comment",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: block comment."
    },
    {
      "id": "hash-not-comment",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: hash not comment."
    },
    {
      "id": "expression-empty",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: expression empty."
    },
    {
      "id": "operator-missing-right",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: operator missing right."
    },
    {
      "id": "operator-missing-left",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: operator missing left."
    },
    {
      "id": "conditional-no-then",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: conditional no then."
    },
    {
      "id": "conditional-no-else",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: conditional no else."
    },
    {
      "id": "let-no-in",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: let no in."
    },
    {
      "id": "let-no-initializer",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: let no initializer."
    },
    {
      "id": "let-reserved-binder",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: let reserved binder."
    },
    {
      "id": "call-missing-comma",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: call missing comma."
    },
    {
      "id": "call-trailing-comma",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: call trailing comma."
    },
    {
      "id": "parenthesis-unclosed",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: parenthesis unclosed."
    },
    {
      "id": "comparison-split-token",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: comparison split token."
    },
    {
      "id": "qualified-split-token",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: qualified split token."
    },
    {
      "id": "reference-space-before-hash",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: reference space before hash."
    },
    {
      "id": "reference-space-after-hash",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: reference space after hash."
    },
    {
      "id": "reference-non-string-key",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: reference non string key."
    },
    {
      "id": "reference-extra-hash",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: reference extra hash."
    },
    {
      "id": "reference-on-parenthesis",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: reference on parenthesis."
    },
    {
      "id": "reference-leading-hash",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: reference leading hash."
    },
    {
      "id": "power-chained",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed source is refused: power chained."
    },
    {
      "id": "reserved-function-name",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed declaration is refused: reserved function name."
    },
    {
      "id": "reserved-parameter",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed declaration is refused: reserved parameter."
    },
    {
      "id": "unicode-identifier",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed declaration is refused: unicode identifier."
    },
    {
      "id": "function-no-pure",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed declaration is refused: function no pure."
    },
    {
      "id": "function-no-return-type",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed declaration is refused: function no return type."
    },
    {
      "id": "function-no-closing-brace",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed declaration is refused: function no closing brace."
    },
    {
      "id": "profile-after-function",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed declaration is refused: profile after function."
    },
    {
      "id": "record-missing-semicolon",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed declaration is refused: record missing semicolon."
    },
    {
      "id": "decimal-missing-mode",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed declaration is refused: decimal missing mode."
    },
    {
      "id": "tuple-missing-semicolon",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed declaration is refused: tuple missing semicolon."
    },
    {
      "id": "model-version-suffix",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed declaration is refused: model version suffix."
    },
    {
      "id": "model-digest-suffix",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Malformed declaration is refused: model digest suffix."
    },
    {
      "id": "header-no-semicolon",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "An incomplete source unit is refused: header no semicolon."
    },
    {
      "id": "header-no-edition",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "An incomplete source unit is refused: header no edition."
    },
    {
      "id": "header-unquoted-edition",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "An incomplete source unit is refused: header unquoted edition."
    },
    {
      "id": "header-missing",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "An incomplete source unit is refused: header missing."
    },
    {
      "id": "evaluate-boolean-true",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed boolean true expression."
    },
    {
      "id": "evaluate-boolean-false",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed boolean false expression."
    },
    {
      "id": "evaluate-integer-zero",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed integer zero expression."
    },
    {
      "id": "evaluate-integer-large",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed integer large expression."
    },
    {
      "id": "evaluate-integer-negative",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed integer negative expression."
    },
    {
      "id": "evaluate-unary-chain",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed unary chain expression."
    },
    {
      "id": "evaluate-arithmetic-grouping",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed arithmetic grouping expression."
    },
    {
      "id": "evaluate-arithmetic-product",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed arithmetic product expression."
    },
    {
      "id": "evaluate-arithmetic-left-chain",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed arithmetic left chain expression."
    },
    {
      "id": "evaluate-division-rem-mod",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed division rem mod expression."
    },
    {
      "id": "evaluate-boolean-precedence",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed boolean precedence expression."
    },
    {
      "id": "evaluate-implication-right",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed implication right expression."
    },
    {
      "id": "evaluate-comparison-equality",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed comparison equality expression."
    },
    {
      "id": "evaluate-comparison-not-equal",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed comparison not equal expression."
    },
    {
      "id": "evaluate-comparison-less-equal",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed comparison less equal expression."
    },
    {
      "id": "evaluate-comparison-greater-equal",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed comparison greater equal expression."
    },
    {
      "id": "evaluate-comparison-less",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed comparison less expression."
    },
    {
      "id": "evaluate-comparison-greater",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed comparison greater expression."
    },
    {
      "id": "evaluate-negated-comparison",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed negated comparison expression."
    },
    {
      "id": "evaluate-conditional",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed conditional expression."
    },
    {
      "id": "evaluate-nested-conditional",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed nested conditional expression."
    },
    {
      "id": "evaluate-let-binding",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed let binding expression."
    },
    {
      "id": "evaluate-let-nested",
      "path": "examples/grammar/example.toml",
      "role": "examined",
      "excerpt": "Evaluation preserves the parsed let nested expression."
    }
  ]
}

```
