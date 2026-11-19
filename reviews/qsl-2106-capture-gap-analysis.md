---
id: SR-014
title: QSL-2106 capture examples gap-analysis
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/filament-examples; examples/docs-captures/**, README.md, index.json; PR 53 at 5c93741651195fcd035516c2b8f747f31068cdfa'
review_set: subset
---

## Summary

Ticket QSL-2106. Reviewed the seven original capture cases, both source programs, eight requests, seven expected outcomes, integration and catalog changes. No defects found in this partial slice.

## Verdict

PASS for the changed capture slice. The origin drive-to-brake/no-stop violation catches recomputing a fast capture after slowing. The slow-origin always rule catches replacing a slow capture when speed rises. Reactivation brake/drive/brake violates at position 1, catching both sharing the first slow capture and replacing the new fast capture on the following brake frame. Drive/stop/brake holds, catching reuse of the first fast capture for later slow activations. The stop controls provide nonconstant positive outcomes. Each request selects the matching clause/source and references canonical Rover model/data; no sibling files are copied. Recorded outcomes assert exit, category, S6a stage, clause, completed/closed-scope active truth, and negative witness positions through the existing full normalized-structure comparison.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Accepted coder evidence without rerun: seven runnable cases pass, four holds and three violations, all closed-scope, using driver c6e7741 with FSL pin 13b0b11e. This is not current-main runtime evidence. No tests, build, matrix, baseline, or full gate was repeated. The lead owns the final changed-system gate. Broader QSL-2106 remains open; docs companion #93 is outside this review.

All changed files examined; each case scope unit includes its request and expected outcome. The eighth false-guard request is explicitly excluded from the runnable manifest and catalog, has no incorrect expected output, and is not counted as passing coverage. Its dependency disposition remains with the lead. FR-093-AC-3 is the relevant mutable-state obligation; distinct semantic-trigger identities, duplicate receipts, capture faults and retained cross-call ledger semantics are not claimed by these position-based source examples. No local spec tree or applicable AssuranceProfile exists. No Rust/source runner or spec/plan change occurs, so Rust and spec reviews are inapplicable. Existing tools/fsl-examples/src/outcome.rs::evaluate_case is context only.

Manual diff-scoped requirement/source/oracle alignment used because this examples repository has no spec root; no computed repository traceability claim is made. Quoin is installed, but a repository matrix is inapplicable here. No stubs, reverse ownership gaps, hollow success or copied upstream content found in this slice.

Plan completion: not assessed

## Examined units

```json
{
  "clean": true,
  "scope": [
    {
      "id": "origin-fast-stops",
      "path": "examples/docs-captures/example.toml",
      "role": "examined",
      "excerpt": "An initially fast rover stops within three steps even as its speed changes."
    },
    {
      "id": "origin-fast-only-slows",
      "path": "examples/docs-captures/example.toml",
      "role": "examined",
      "excerpt": "Slowing down after activation does not erase the captured obligation to stop."
    },
    {
      "id": "origin-slow-then-fast",
      "path": "examples/docs-captures/example.toml",
      "role": "examined",
      "excerpt": "A captured slow-at-origin value stays true after the current speed rises."
    },
    {
      "id": "origin-fast-then-slow",
      "path": "examples/docs-captures/example.toml",
      "role": "examined",
      "excerpt": "A later low speed cannot repair a false slow-at-origin capture."
    },
    {
      "id": "reactivate-slow-then-fast-stops",
      "path": "examples/docs-captures/example.toml",
      "role": "examined",
      "excerpt": "A new fast activation captures its own value and is followed by a stop."
    },
    {
      "id": "reactivate-slow-then-fast-misses",
      "path": "examples/docs-captures/example.toml",
      "role": "examined",
      "excerpt": "A later fast activation cannot borrow the first slow activation's exemption."
    },
    {
      "id": "reactivate-fast-then-slow",
      "path": "examples/docs-captures/example.toml",
      "role": "examined",
      "excerpt": "A later slow activation does not inherit an earlier fast activation's stopping requirement."
    },
    {
      "id": "capture-source-origin",
      "path": "examples/docs-captures/rules/origin.fsl",
      "role": "examined",
      "excerpt": "capture wasFast: Boolean = vehicle.speed > 300;\n  eventually[0,3] holds(vehicle.stopped or not wasFast)"
    },
    {
      "id": "capture-source-reactivation",
      "path": "examples/docs-captures/rules/reactivation.fsl",
      "role": "examined",
      "excerpt": "on each (trigger: Fleet::Rover) when (trigger.speed > 0) {\n  capture beganFast: Boolean = trigger.speed > 300;\n  eventually[0,1] holds(vehicle.stopped or not beganFast)"
    },
    {
      "id": "inactive-control",
      "path": "examples/docs-captures/README.md",
      "role": "examined",
      "excerpt": "The required result is inactive with no truth value; no output is recorded for that control."
    },
    {
      "id": "runner-integration",
      "path": "examples/docs-captures/Makefile",
      "role": "examined",
      "excerpt": "check staged:\n\t@$(FSL_EXAMPLES) $@ ."
    },
    {
      "id": "system-catalog",
      "path": "README.md",
      "role": "examined",
      "excerpt": "| [docs-captures](examples/docs-captures) | Language | 7 | 0 |"
    },
    {
      "id": "case-catalog",
      "path": "index.json",
      "role": "examined",
      "excerpt": "\"system\": \"docs-captures\""
    },
    {
      "id": "FR-093-AC-3",
      "path": "agent-ix/filament-specification:spec/functional/temporal/FR-093-bind-temporal-activation-and-captures.md",
      "role": "examined",
      "excerpt": "Changing mutable source state after activation cannot change an already captured amount, reference or predicate valuation environment."
    },
    {
      "id": "FR-093-AC-6",
      "path": "agent-ix/filament-specification:spec/functional/temporal/FR-093-bind-temporal-activation-and-captures.md",
      "role": "context_only",
      "excerpt": "False guards create no obligation, unestablished guards remain incomplete/refused, and conflicting payloads under one semantic trigger identity refuse without replacing retained captures."
    }
  ],
  "findings": [],
  "bindings": []
}
```
