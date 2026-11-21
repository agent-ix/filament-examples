---
id: SR-2026111919340478491681437
title: "QSL-2214 filament-examples gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-examples; full bounded PR 73 diff and its existing readers"
review_set: subset
---

## Summary

Repository-driven audit attempt and direct requirement-to-behavior inspection for the protocol examples. The repository lacks a local spec root; plan completion was not assessed.

## Verdict

FAIL for whole-repository assurance because engine-backed reconciliation cannot run against the missing spec root. This is a repository limitation, not a new example failure.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The examples repository has no spec document root; engine-backed matrix reconciliation and a whole-repository assurance PASS are unavailable. | spec/ |

## Coverage

Reconciliation: quire coverage --scope . --json (quire 0.52.0, engine 0.62.2) exited 3: no document root at <repository>/spec; documents live in spec under the repository root. No grep fallback, invented matrix or retrospective requirement tree was created. Criteria counts and complete reverse ownership are unavailable.

Plan completion: not assessed

Direct inspection covered the new corpus records, manifest/index consumers and the real-process Rust integration tests. The five supported cases actually run; the failed hidden-choice form remains staged, requires a future exit 10 with counterexample, and is not counted as a running documentation case. No new source/test stub, fabricated output or vacuous successful test was found. The new test license comment is fixed separately. Semantic review: skipped.

## Reviewed units

```yaml
{
  "scope": [
    {
      "id": "projection-cases",
      "path": "examples/lock/example.toml",
      "role": "examined",
      "excerpt": "id = \"protocol-project-asker\""
    },
    {
      "id": "realizability-cases",
      "path": "examples/lock/example.toml",
      "role": "examined",
      "excerpt": "id = \"protocol-realizable\""
    },
    {
      "id": "blocked-negative",
      "path": "examples/lock/example.toml",
      "role": "examined",
      "excerpt": "status = \"staged\"\nexpect_exit = 30\nready_exit = 10"
    },
    {
      "id": "composition-declaration",
      "path": "examples/lock/rules/protocols/handover-composition.fsl",
      "role": "examined",
      "excerpt": "compose HandoverSystem rule assume-guarantee {"
    },
    {
      "id": "runtime-tests",
      "path": "tools/fsl-examples/tests/protocol_docs.rs",
      "role": "examined",
      "excerpt": "//! The protocol documentation cases through the production driver and runner."
    },
    {
      "id": "generated-index",
      "path": "index.json",
      "role": "examined",
      "excerpt": "\"case\": \"protocol-unrealizable\","
    },
    {
      "id": "catalog-counts",
      "path": "README.md",
      "role": "examined",
      "excerpt": "| [lock](examples/lock) | Safety | 25 | 1 |"
    }
  ]
}
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | The missing document-root and repository-wide assurance work are outside the bounded examples delivery; no local spec tree was manufactured. |

