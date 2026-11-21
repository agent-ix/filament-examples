---
id: SR-2026111919340478491681436
title: "QSL-2214 filament-examples code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-examples; full bounded PR 73 diff and its existing readers"
review_set: subset
---

## Summary

Independent Rust and example review of the protocol documentation cases, actual generated records, manifest/index changes, and regression lane. The reviewer did not author these changes.

## Verdict

CONDITIONAL at initial review: add the repository-license header to the new Rust test.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new Rust integration test lacks its MIT SPDX license header required by the Rust style baseline. | tools/fsl-examples/tests/protocol_docs.rs:1 |

## Coverage

Reviewed all 23 changed files, including the six manifest cases, their request/source and expected-record families, composition imports, index/count updates and both integration tests. Existing CountingDriver was inspected before accepting the distinct real-process CapturingDriver decorator. Production execution remains the unit under test; the decorator records actual stdout. The supported test asserts five runs and four proved records; existing runner comparison validates the compile-only composition case. The separate ignored negative regression still requires exit 10 and a counterexample and will fail against the preserved product defect. Its source/request were not changed to pass.

Draco ran protocol_docs_examples_run_and_match_records on the frozen submitted source: exit 0, one passed, zero failed, zero ignored, one filtered. It executed the five intended cases through the preserved driver CLI meeting the assigned minimum revision. The subsequent change is a license comment only. No full suite/gate ran. New tests carry requirement references, but no engine-backed local criterion coverage is claimed because this repository has no spec root.

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
| FND-001 | fixed | Added the MIT SPDX header; verified the complete correction contains only that comment and leaves runtime assertions unchanged. |

## Final review state

PASS after the inspected correction. The existing partial-delivery boundaries remain explicit.

After excerpt: `// SPDX-License-Identifier: MIT`

