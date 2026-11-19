---
id: SR-011
title: QSL-2106 numeric documentation cases code review
type: SpecReview
analysis: code-review
scope: 'agent-ix/filament-examples; examples/docs-values/**, README.md, index.json; PR 52 at 04fcb95ec531cedb0a03f0d7e7e4a3ea271c659e'
review_set: subset
---

## Summary

Ticket QSL-2106. Reviewed the 15 numeric example cases, five original FSL programs, their requests and recorded outcomes, Makefile integration, documentation and generated catalog changes. No defects found in this vertical slice.

## Verdict

PASS. Each signed division case selects the intended named function and division-law source. The functions compare both quotient and remainder with explicit constants, so a wrong arithmetic result changes the recorded Boolean outcome. Guarded remainder cases assert concrete integers 2 and 0. The unguarded-divisor case asserts refusal at S3 with undefined_expression/unproved-nonzero rather than accepting any failure.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

All changed files are covered by the case units below and the integration units: each case includes its source program, request and expected record. Request function selection, profile selection, signed-operand constants, manifest status/exit, expected category/stage/result/diagnostic, and generated paths were examined. Existing tools/fsl-examples/src/outcome.rs::evaluate_case was read as context: it compares exit and normalized outcome structure, including actual result values, with the recorded oracle. No modified runner or first-party Rust code exists in this diff; Rust review and Rust build/lint checks are inapplicable. No spec/plan artifacts changed; spec review is inapplicable. No spec tree or applicable AssuranceProfile exists in this repository. No copied sibling code, compatibility layer, workflow change, new helper or production semantic implementation is introduced.

Coder evidence accepted without rerun: 15 cases recorded successfully, 12 division comparisons true, guarded modulo values 2/0, unguarded modulo refusal as stated, and generated README/index passed. This evidence uses the disclosed reusable driver app0.1.4 at c6e7741 with an older FSL pin; it is not a claim of runtime execution against today's FSL main. The lead owns the final gate. This slice does not complete every documentation snippet in QSL-2106.

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
