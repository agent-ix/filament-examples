---
id: SR-016
title: QSL-2585 saga attempt binders gap-analysis
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/filament-examples; examples/parts-saga/rules/**; PR 54 at 2a8358e6cda6f0189d25e7d06bf38e8a00705c85'
review_set: subset
---

## Summary

Ticket QSL-2585. Reviewed all 13 changed FSL programs and their 36 binder/predicate replacements against the operation result contracts and FR-218-AC-44. No defects found in this source repair.

## Verdict

PASS for the diff. Every changed binder matches its Boolean-returning operation; its predicate reads that same local binder, retaining the success condition. Choice, repeat, branch and join structure, role ownership, and contracts remain intact. EffectView, CommitView, compensation AttemptView/retry records, model and service contracts and expected outcomes are unchanged. Negative cases still contain their intended wrong-role, overlapping-guard, missing-contract, omitted-branch, missing-outstanding-policy and zero-attempt defects. No copied upstream source, compatibility layer, test removal, weakened expectation, helper or framework is introduced.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Plan completion: not assessed

Diff-scoped review only. Accepted coder evidence: all 12 affected focused checks passed, including protocol-compiles exit 0 and wrong-role/overlapping-verdict exit 20 for intended causes. No tests, builds, matrix, broken pre-fix baseline or aggregate gate were repeated. The lead owns the final affected-family gate. Timed saga and compensation end-to-end behavior remains owned by QSL-2225 and is not claimed supported.

No local spec tree or applicable AssuranceProfile exists. No Rust or spec/plan change occurs, so Rust and spec review are inapplicable. Quoin is installed; matrix execution is deliberately excluded by the review brief. This gap analysis instead checks changed source against the existing owning criterion and affected case contracts, with no repository-wide traceability claim. No new tests or trace tags are introduced, so no new test-to-AC binding is claimed. The FR-218-AC-44 scope excerpt is its examined positive clause; mismatched-result refusals are context, not newly tested here.

## Examined units

```json
{
  "clean": true,
  "scope": [
    {
      "id": "examples/parts-saga/rules/inspection.fsl",
      "path": "examples/parts-saga/rules/inspection.fsl",
      "role": "examined",
      "excerpt": "              as (again: Boolean) { again };\n          as (freed: Boolean) { freed };"
    },
    {
      "id": "examples/parts-saga/rules/part-flow-overlapping-verdict.fsl",
      "path": "examples/parts-saga/rules/part-flow-overlapping-verdict.fsl",
      "role": "examined",
      "excerpt": "      as (reserved: Boolean) { reserved };\n      as (processed: Boolean) { processed };\n          as (assembled: Boolean) { assembled };\n              as (again: Boolean) { again };\n          as (freed: Boolean) { freed };"
    },
    {
      "id": "examples/parts-saga/rules/part-flow-untimed.fsl",
      "path": "examples/parts-saga/rules/part-flow-untimed.fsl",
      "role": "examined",
      "excerpt": "      as (reserved: Boolean) { reserved };\n      as (processed: Boolean) { processed };\n          as (assembled: Boolean) { assembled };\n              as (again: Boolean) { again };\n          as (freed: Boolean) { freed };"
    },
    {
      "id": "examples/parts-saga/rules/part-flow-wrong-role.fsl",
      "path": "examples/parts-saga/rules/part-flow-wrong-role.fsl",
      "role": "examined",
      "excerpt": "      as (reserved: Boolean) { reserved };\n      as (processed: Boolean) { processed };\n          as (assembled: Boolean) { assembled };\n              as (again: Boolean) { again };\n          as (freed: Boolean) { freed };"
    },
    {
      "id": "examples/parts-saga/rules/part-flow.fsl",
      "path": "examples/parts-saga/rules/part-flow.fsl",
      "role": "examined",
      "excerpt": "      as (reserved: Boolean) { reserved };\n      as (processed: Boolean) { processed };\n          as (assembled: Boolean) { assembled };\n              as (again: Boolean) { again };"
    },
    {
      "id": "examples/parts-saga/rules/protocols/compensation/compensation-zero-attempts.fsl",
      "path": "examples/parts-saga/rules/protocols/compensation/compensation-zero-attempts.fsl",
      "role": "examined",
      "excerpt": "      as (reserved: Boolean) { reserved };\n      as (processed: Boolean) { processed };\n          as (assembled: Boolean) { assembled };\n              as (again: Boolean) { again };"
    },
    {
      "id": "examples/parts-saga/rules/protocols/control-flow/control-join-any-needs-outstanding.fsl",
      "path": "examples/parts-saga/rules/protocols/control-flow/control-join-any-needs-outstanding.fsl",
      "role": "examined",
      "excerpt": "      as (reserved: Boolean) { reserved };\n      as (assembled: Boolean) { assembled };"
    },
    {
      "id": "examples/parts-saga/rules/protocols/control-flow/control-join-omits-branch.fsl",
      "path": "examples/parts-saga/rules/protocols/control-flow/control-join-omits-branch.fsl",
      "role": "examined",
      "excerpt": "      as (reserved: Boolean) { reserved };\n      as (assembled: Boolean) { assembled };"
    },
    {
      "id": "examples/parts-saga/rules/protocols/control-flow/control-parallel-all.fsl",
      "path": "examples/parts-saga/rules/protocols/control-flow/control-parallel-all.fsl",
      "role": "examined",
      "excerpt": "      as (reserved: Boolean) { reserved };\n      as (assembled: Boolean) { assembled };"
    },
    {
      "id": "examples/parts-saga/rules/protocols/control-flow/control-parallel-any-cancel.fsl",
      "path": "examples/parts-saga/rules/protocols/control-flow/control-parallel-any-cancel.fsl",
      "role": "examined",
      "excerpt": "      as (reserved: Boolean) { reserved };\n      as (assembled: Boolean) { assembled };"
    },
    {
      "id": "examples/parts-saga/rules/protocols/control-flow/control-repeat-progress.fsl",
      "path": "examples/parts-saga/rules/protocols/control-flow/control-repeat-progress.fsl",
      "role": "examined",
      "excerpt": "        as (again: Boolean) { again };"
    },
    {
      "id": "examples/parts-saga/rules/protocols/steps/steps-all-kinds.fsl",
      "path": "examples/parts-saga/rules/protocols/steps/steps-all-kinds.fsl",
      "role": "examined",
      "excerpt": "      as (reserved: Boolean) { reserved };"
    },
    {
      "id": "examples/parts-saga/rules/protocols/steps/steps-unknown-contract.fsl",
      "path": "examples/parts-saga/rules/protocols/steps/steps-unknown-contract.fsl",
      "role": "examined",
      "excerpt": "      as (reserved: Boolean) { reserved };"
    },
    {
      "id": "FR-218-AC-44",
      "path": "agent-ix/filament-spec-language:spec/functional/FR-218-check-every-protocol-control-construct-at-s3.md",
      "role": "examined",
      "excerpt": "Over the Cell fixture, whose `flag` returns `Boolean` and whose `touch` returns nothing: `attempt T by w on M::Cell::flag contracts [FlagPre, FlagPost] as (t: Boolean) { true }` followed by `check Gate using v { t }` checks and the check reads the result;"
    },
    {
      "id": "parts-saga-model",
      "path": "examples/parts-saga/model/parts-saga.semantic-ir.json",
      "role": "context_only",
      "excerpt": "reserve, process, release and assemble return ix://filament/native/Boolean."
    },
    {
      "id": "affected-case-contracts",
      "path": "examples/parts-saga/example.toml",
      "role": "examined",
      "excerpt": "protocol-compiles; protocol-overlapping-verdict; protocol-wrong-role; inspection-compiles; steps-all-kinds; steps-unknown-contract; control-parallel-all; control-parallel-any-cancel; control-join-omits-branch; control-repeat-progress; compensation-zero-attempts; control-join-any-needs-outstanding."
    }
  ],
  "findings": [],
  "bindings": []
}
```
