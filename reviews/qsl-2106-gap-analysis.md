---
id: SR-012
title: QSL-2106 numeric documentation cases gap analysis
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/filament-examples; examples/docs-values/**, README.md, index.json; PR 52 at 04fcb95ec531cedb0a03f0d7e7e4a3ea271c659e'
review_set: subset
---

## Summary

Ticket QSL-2106. Reviewed the claimed numeric documentation slice against its executable cases and semantic oracles. No hollow case or missing behavior within this slice was identified.

## Verdict

PASS for the PR's claimed slice. This is a diff-scoped manual requirement-to-case and oracle review. It is not a repository-wide matrix or language-completeness claim.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Plan completion: not assessed

The slice supplies each of the four dividend/divisor sign combinations under truncating, floor and Euclidean division, plus guarded nonzero remainder, an explicit zero-divisor branch and rejection of an unproved nonzero divisor. It routes real FSL programs through the established runner. Wrong quotients/remainders, a wrongly evaluated guarded branch or a wrong refusal code/cause can make a case fail. Original programs live in their authoritative examples repository; teaching documentation can reference them through the existing example mechanism without copied programs or a new bookkeeping detector.

The repository has no spec document root or declared criterion matrix for these examples. No matrix was run, and no criterion coverage percentage is claimed. Case statements are reviewed test intent, not trace-tag claims. No source/test stubs or pass-only placeholder fixtures were found. The broader docs conversion remains open; this PR makes no claim that all 21 historical snippets are complete. No tests/builds were repeated; reported evidence and driver-version limits are recorded in SR-011. No spec review or Rust implementation review applies to this case-only diff.

## Examined units

```json
{
  "clean": true,
  "scope": [
    {
      "id": "division-truncating-positive-positive",
      "path": "examples/docs-values/example.toml",
      "role": "examined",
      "excerpt": "Under truncating division, 7 divided by 2 gives quotient 3 and remainder 1."
    },
    {
      "id": "division-truncating-negative-positive",
      "path": "examples/docs-values/example.toml",
      "role": "examined",
      "excerpt": "Under truncating division, -7 divided by 2 gives quotient -3 and remainder -1."
    },
    {
      "id": "division-truncating-positive-negative",
      "path": "examples/docs-values/example.toml",
      "role": "examined",
      "excerpt": "Under truncating division, 7 divided by -2 gives quotient -3 and remainder 1."
    },
    {
      "id": "division-truncating-negative-negative",
      "path": "examples/docs-values/example.toml",
      "role": "examined",
      "excerpt": "Under truncating division, -7 divided by -2 gives quotient 3 and remainder -1."
    },
    {
      "id": "division-floor-positive-positive",
      "path": "examples/docs-values/example.toml",
      "role": "examined",
      "excerpt": "Under floor division, 7 divided by 2 gives quotient 3 and remainder 1."
    },
    {
      "id": "division-floor-negative-positive",
      "path": "examples/docs-values/example.toml",
      "role": "examined",
      "excerpt": "Under floor division, -7 divided by 2 gives quotient -4 and remainder 1."
    },
    {
      "id": "division-floor-positive-negative",
      "path": "examples/docs-values/example.toml",
      "role": "examined",
      "excerpt": "Under floor division, 7 divided by -2 gives quotient -4 and remainder -1."
    },
    {
      "id": "division-floor-negative-negative",
      "path": "examples/docs-values/example.toml",
      "role": "examined",
      "excerpt": "Under floor division, -7 divided by -2 gives quotient 3 and remainder -1."
    },
    {
      "id": "division-euclidean-positive-positive",
      "path": "examples/docs-values/example.toml",
      "role": "examined",
      "excerpt": "Under euclidean division, 7 divided by 2 gives quotient 3 and remainder 1."
    },
    {
      "id": "division-euclidean-negative-positive",
      "path": "examples/docs-values/example.toml",
      "role": "examined",
      "excerpt": "Under euclidean division, -7 divided by 2 gives quotient -4 and remainder 1."
    },
    {
      "id": "division-euclidean-positive-negative",
      "path": "examples/docs-values/example.toml",
      "role": "examined",
      "excerpt": "Under euclidean division, 7 divided by -2 gives quotient -3 and remainder 1."
    },
    {
      "id": "division-euclidean-negative-negative",
      "path": "examples/docs-values/example.toml",
      "role": "examined",
      "excerpt": "Under euclidean division, -7 divided by -2 gives quotient 4 and remainder 1."
    },
    {
      "id": "guarded-remainder-negative",
      "path": "examples/docs-values/example.toml",
      "role": "examined",
      "excerpt": "The guarded Euclidean remainder of -7 by 3 is 2."
    },
    {
      "id": "guarded-remainder-zero",
      "path": "examples/docs-values/example.toml",
      "role": "examined",
      "excerpt": "The zero-divisor branch returns its explicit default without evaluating mod."
    },
    {
      "id": "unguarded-remainder",
      "path": "examples/docs-values/example.toml",
      "role": "examined",
      "excerpt": "An unguarded divisor that can be zero is refused while compiling."
    },
    {
      "id": "docs-values-integration",
      "path": "examples/docs-values/Makefile",
      "role": "examined",
      "excerpt": "check staged:"
    },
    {
      "id": "docs-values-documentation",
      "path": "examples/docs-values/README.md",
      "role": "examined",
      "excerpt": "The division cases cover both signs of each operand under all three laws."
    },
    {
      "id": "generated-catalog",
      "path": "index.json",
      "role": "examined",
      "excerpt": "docs-values"
    },
    {
      "id": "systems-table",
      "path": "README.md",
      "role": "examined",
      "excerpt": "| [docs-values](examples/docs-values) | Language | 15 | 0 |"
    }
  ],
  "findings": []
}
```
