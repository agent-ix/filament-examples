---
id: SR-2026111919340478491681429
title: "code-review of QSL-2481 examples PR 59"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-examples; Makefile; tutorials/first-rule/steps.md; tutorials/write-a-model/steps.md"
review_set: subset
---

## Summary

Ticket: QSL-2481. Reviewed frozen PR #59 at a483ac4246a935dc2557aae1d2b2e3e65766eba3, excluding review artifacts. The Make targets reuse the native Steps runner and exported FSL setting. Its pipefail preserves expected CLI exit statuses through tee. The first-rule cases assert truth, activation and execution without imposing a settlement state. The model generator and request files remain local authoritative examples; the diff adds no copied binary, model, schema or workflow.

## Verdict

**PASS** — No implementation or integration defect found in the reviewed diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Plan completion: not assessed

Diff-scoped review only; no matrix, tests, builds or prior artifacts rerun. Accepted coder evidence: 6 first-rule steps / 3 public runs and 5 generated-model steps / 2 public compiles passed against the shared current-main CLI (driver18d/FSLdeac/codegen889/IRe576). QSL-1980-dependent authoring and guide facets remain pending and uncounted; this is not acceptance of the whole ticket. Rust review is inapplicable: no Rust/Cargo diff. Repository-wide traceability and unrelated assurance context were outside the assigned scope.

## Examined Units

```yaml
{
  "scope": [
    {
      "id": "Makefile-tutorial-targets",
      "path": "Makefile",
      "role": "examined",
      "excerpt": "tutorial-test: tutorial-first-rule-test tutorial-write-model-test"
    },
    {
      "id": "first-rule-paid",
      "path": "tutorials/first-rule/steps.md",
      "role": "examined",
      "excerpt": "The order is paid and its charged amount equals its total. The rule holds."
    },
    {
      "id": "first-rule-short",
      "path": "tutorials/first-rule/steps.md",
      "role": "examined",
      "excerpt": "The order is still paid, but 4000 was charged against a total of 4200. The rule is violated."
    },
    {
      "id": "first-rule-open",
      "path": "tutorials/first-rule/steps.md",
      "role": "examined",
      "excerpt": "The open order has an unpaid balance. The implication holds because the order is not paid."
    },
    {
      "id": "write-model-generate",
      "path": "tutorials/write-a-model/steps.md",
      "role": "examined",
      "excerpt": "Generate the model file that the compile request names."
    },
    {
      "id": "write-model-compile",
      "path": "tutorials/write-a-model/steps.md",
      "role": "examined",
      "excerpt": "The rule uses the model's declared `copies` field."
    },
    {
      "id": "write-model-missing-name",
      "path": "tutorials/write-a-model/steps.md",
      "role": "examined",
      "excerpt": "The misspelled rule reads `copy`, which the model does not declare."
    },
    {
      "id": "existing-steps-runner",
      "path": "tools/steps.mjs",
      "role": "context_only",
      "excerpt": "const parts = [\"set -eu -o pipefail\", \"exec 2>&1\"];"
    },
    {
      "id": "existing-fsl-config",
      "path": "config.mk",
      "role": "context_only",
      "excerpt": "FSL ?= fsl\nexport FSL"
    },
    {
      "id": "misspelled-source",
      "path": "examples/library/requests/compile-misspelled.json",
      "role": "context_only",
      "excerpt": "\"file\": \"../rules/misspelled.fsl\""
    }
  ]
}
```

## Disposition Round 1 Context

The sole gap-analysis finding was resolved at f24f908c79ed2eae8ee3913eb74c50a1907c77e9 by the diagnostic-locus assertion. The original code-review PASS remains. Only that six-line fix was inspected; no tests, builds or additional review sweep ran.
