---
id: SR-1517
title: QSL-2180 working forms docs and examples code-review
type: SpecReview
analysis: code-review
scope: 'agent-ix/filament-examples PR 70 at 1d86c7af6d91bee8f51c89412e0ca72b2ed760cf (examples/language-values, examples/lock, examples/parts-saga, README.md, index.json); agent-ix/filament-docs PR 94 at 5ce2bb413ea07496a6edd992e6e5eb1daac5cf2d (src/content/docs/fsl/**, docs-gates.yaml)'
review_set: subset
---

## Summary

Ticket QSL-2180. Reviewed the 26 new example cases (20 language-values, 4 lock, 2 parts-saga) and the nine edited docs files against the FSL functional spec (read-only checkout qsl-2144-timed-reader) and fsl-eval/tests/it/collection_queries.rs q06/q09. The "language" lane here is FSL source cases and Markdown docs; there is no Rust.

Each new case exercises the form it names, and its recorded code and cause match the spec row: UUID and Timestamp ordering and Timestamp against Integer (FR-056-AC-36), malformed and uppercase UUID (FR-091-AC-43), Text over bound (FR-056-AC-52), Some (FR-983), delivery interval and negative bound (FR-509-AC-1/AC-2), protocol reference and missing name (FR-218-AC-8), release declaration and unknown operation (FR-249-AC-6), and the kind conversions (q06 cases 1 to 3, the above-maximum refusal and the element-type refusal; q09). One error-table row on the identifiers page states a refusal that the driver does not give.

## Verdict

Mergeable if fixed. The examples PR is correct as it stands apart from the low items. The docs PR must fix FND-001. The other findings are low.

Verified: the examples runner passes 42 of 42 language-values cases and all 6 new lock and parts-saga cases with the driver build the coder used (fsl-main, 0.1.4). With the released 0.1.4 driver, the new lock and parts-saga cases pass, and some-present and index-past-end-refused fail (FND-005). The docs example gate (FSL_EXAMPLES_DIR pointed at the examples worktree) reports no finding on the edited pages; its 13 findings are all on other pages. `pnpm test` and `pnpm lint` pass. The schedulability page now uses the `release S.t on M::...;` form of FR-249, and no docs page still uses `releases <task>`. The docs pages link cases through `example` fences, and no file was copied from one repo to the other.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The identifiers errors table says UUID("a") < UUID("b") is refused operator-ineligible. The driver refuses it type-mismatch, because the literals are malformed (FR-091-AC-43) | filament-docs src/content/docs/fsl/types/identifiers.md:62 |
| FND-002 | low | Collections says sequence[1, 2, 1] fits a Set bounded by 2, but the linked case convert-sequence-to-set uses Set<Integer>[0, 3]. No case shows equal members merged before the bound is checked | filament-docs src/content/docs/fsl/types/collections.md:224 |
| FND-003 | low | The Text constructor text says the text is normalized before its length is counted. Both cases write a precomposed U+00E9, so no case exercises normalization | filament-docs src/content/docs/fsl/types/text.md:70 |
| FND-004 | low | Timestamp(n) is described as "an integer count" with no unit or epoch. FR-056 defines nanoseconds since 1970-01-01T00:00:00Z, so a reader cannot tell what Timestamp(5) denotes | filament-docs src/content/docs/fsl/types/identifiers.md:39 |
| FND-005 | low | some-out-of-bound passes on the released 0.1.4 driver for the wrong reason: Some(3) itself is refused there with the same ill_typed/type-mismatch. The PR body names only index [i] as needing a newer driver, but some-present also fails on 0.1.4 | filament-examples examples/language-values/example.toml:245 |
| FND-006 | low | protocol-reference cases say they serve "Types reference (protocol reference)", but the docs page that links them is protocols/declaration | filament-examples examples/lock/example.toml:199 |
| FND-007 | low | The convert grammar line convert<<Kind><<element>>[<min>, <max>]>(<c>) nests placeholder brackets inside type brackets and is hard to read | filament-docs src/content/docs/fsl/types/collections.md:213 |

## Coverage

Plan completion: not assessed

Diff-scoped review of both PRs. The driver dependence matches existing main: index-sequence and index-ordered-set already fail on the released 0.1.4. The examples check workflow runs manually and builds a driver revision from source by default. So the dependence is not a gate failure; FND-005 covers the reporting gap only. The style gate (Vale), `pnpm build` and the browser gates were not run, as the brief directs.
