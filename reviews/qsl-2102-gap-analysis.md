---
id: SR-1516
title: "gap-analysis of QSL-2102 language value examples"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-examples; QSL-2102; frozen 102ff1e; examples/language-values/, new index.json entries, README.md generated row"
review_set: subset
---

## Summary

Reviewed the 22 new original source/request/expected-outcome cases and their manifest and generated entries at frozen 102ff1e. Runtime results supplied by the author are accepted; no tests, builds, gates, or old examples were rerun.

## Verdict

**PASS** — all original findings are fixed at 386467a. This is a diff-scoped review, not acceptance of all QSL-2102 forms.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The OrderedSet indexing statement claims duplicate removal, but its source contains no duplicate. Put a duplicate before the indexed position or narrow the statement to insertion-order indexing. | examples/language-values/rules/index-ordered-set.fsl:3 |

## Coverage

- `float-specials-nan`; role: examined; examples/language-values/example.toml; excerpt: NaN is unequal to itself.
- `float-specials-positive-infinity`; role: examined; examples/language-values/example.toml; excerpt: Positive infinity exceeds the largest finite binary64 value.
- `float-specials-negative-infinity`; role: examined; examples/language-values/example.toml; excerpt: Negative infinity precedes the most negative finite binary64 value.
- `float-specials-positive-zero`; role: examined; examples/language-values/example.toml; excerpt: Negating negative zero produces positive zero bits.
- `float-specials-negative-zero`; role: examined; examples/language-values/example.toml; excerpt: Negative zero has different bits from positive zero.
- `bag-multiplicity`; role: examined; examples/language-values/example.toml; excerpt: Bag equality preserves duplicate multiplicity.
- `bag-permutation`; role: examined; examples/language-values/example.toml; excerpt: Bag equality ignores traversal order.
- `ordered-set-order`; role: examined; examples/language-values/example.toml; excerpt: OrderedSet equality observes member order.
- `ordered-set-duplicate`; role: examined; examples/language-values/example.toml; excerpt: OrderedSet retains the first occurrence of each member.
- `remove-one-sequence`; role: examined; examples/language-values/example.toml; excerpt: removeOne removes the first matching sequence member exactly once.
- `remove-one-bag`; role: examined; examples/language-values/example.toml; excerpt: removeOne removes one occurrence from a Bag.
- `remove-one-absent`; role: examined; examples/language-values/example.toml; excerpt: Removing an absent value leaves a sequence unchanged.
- `flat-map`; role: examined; examples/language-values/example.toml; excerpt: flatMap concatenates each produced sequence in traversal order.
- `function-selection`; role: examined; examples/language-values/example.toml; excerpt: The run request selects the requested named function beside a decoy function.
- `text-constructor`; role: examined; examples/language-values/example.toml; excerpt: Text constructs a bounded NFC value at its expected type.
- `rounding-toward-negative`; role: examined; examples/language-values/example.toml; excerpt: Rounding -1/8 toward negative at two decimal places yields -0.13.
- `recursive-run-values`; role: examined; examples/language-values/example.toml; excerpt: Recursive calls reduce a bounded argument and produce the triangular sum.
- `imported-library-call`; role: examined; examples/language-values/example.toml; excerpt: An imported library function is linked and called with its argument.
- `index-sequence`; role: examined; examples/language-values/example.toml; excerpt: A zero-based sequence index returns its second member.
- `index-ordered-set`; role: examined; examples/language-values/example.toml; excerpt: An OrderedSet index follows insertion order after duplicate removal.
- `bag-duplicate-drop-first-refusal`; role: examined; examples/language-values/example.toml; excerpt: dropFirst refuses an unordered Bag even when it contains duplicate occurrences.
- `ordered-set-ordering-mismatch`; role: examined; examples/language-values/example.toml; excerpt: An OrderedSet with a different member order fails the asserted equality.
- `generated-index`; role: examined; index.json; excerpt: New language-values case entries.
- `readme-row`; role: examined; README.md; excerpt: | [language-values](examples/language-values) | Language | 22 | 0 |

Five float oracles use Boolean IEEE relations or bit identity before JSON projection, distinguishing NaN, infinities, and signed zero without numeric JSON equality. Bag multiplicity/permutation, OrderedSet equality/order, removeOne, flatMap, rounding, recursion and imported triple calls carry concrete semantic assertions. The negative OrderedSet case requires Boolean false; the Bag refusal requires ill_typed/operator-ineligible at S3 and exit 20. No unused declaration or blanket completed verdict was accepted as the sole semantic assertion.

The existing ProcessDriver invokes the external driver subprocess; evaluate_case compares projected semantic structure, including result.value, with each authored expectation. These unchanged runner sections were read only as context. README review used docs-style for the one generated table row; no prose rewrite, documentation gates, or runtime walkthrough was needed.

Plan completion: not assessed. No plan was supplied. Matrix and repository-wide reverse-gap inventory were excluded by the dispatch; this is a scoped source/oracle gap assessment and makes no engine-backed matrix claim. No applicable local assurance profile was supplied or encountered in the changed paths. Unclaimed model/protocol/dimension forms and existing accepted cases remain outside this review.

## Dispositions

Round 1, fix commit 386467a. Only the three finding fixes were inspected. Author-reported focused checks: 3 PASS, exit 0; accepted without reruns.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The earlier repeated 8 is removed before index 1 selects 3. |

### Fixed excerpts

FND-001: `examples/language-values/rules/index-ordered-set.fsl:3`

```
function example using v(): Boolean pure { let xs: OrderedSet<Integer>[3, 3] = orderedSet[8, 8, 3, 6] in xs[1] = 3 }
```
