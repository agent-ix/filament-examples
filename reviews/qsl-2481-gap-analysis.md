---
id: SR-2026111919340478491681430
title: "gap-analysis of QSL-2481 examples PR 59"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-examples; Makefile; tutorials/first-rule/steps.md; tutorials/write-a-model/steps.md"
review_set: subset
---

## Summary

Ticket: QSL-2481. Reviewed frozen PR #59 at a483ac4246a935dc2557aae1d2b2e3e65766eba3, excluding review artifacts. The scoped check maps the five new tutorial cases to their actual public CLI assertions. It does not claim native FSL model authoring or the outdated guide diagnostic as covered: QSL-1980 remains their existing owner. Add a source identity/path and token-span assertion to the existing missing-name case; preserve the standard missing_declaration/missing-name result. No duplicate bug ticket is needed.

## Verdict

**CONDITIONAL** — One missing diagnostic-location assertion leaves a real user-facing regression undetected.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The missing-name oracle never asserts the diagnostic source or span, so a compiler that reports the same code and cause for the wrong file or token still passes this user journey. | tutorials/write-a-model/steps.md:61 |

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

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The existing oracle now asserts the complete regional locus, binding the diagnostic to the misspelled program source and token interval 301..310. Assessed at f24f908c79ed2eae8ee3913eb74c50a1907c77e9. |

## Current Verdict

**PASS** — disposition round 1 resolves the sole finding. Original findings remain preserved.

```yaml
{
  "dispositions": [
    {
      "fnd": "FND-001",
      "outcome": "fixed",
      "after_excerpt": "assert.deepEqual(result.diagnostics[0].locus, {\n  kind: 'region',\n  source: { authority: 'example', identity: 'example:library/misspelled' },\n  start: 301,\n  end: 310,\n});"
    }
  ]
}
```
