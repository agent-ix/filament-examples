---
id: SR-1445
title: "EV-009 blind-help scenario code review"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/filament-examples; QSL-2493; agent-evals/src/scenarios/blind_help.rs, agent-evals/src/lib.rs, agent-evals/src/provider.rs, agent-evals/fixtures/EV-009/, agent-evals/README.md"
---

## Summary

Reviewed the frozen EV-009 feature at 1494e6b86fd9191325e4e61d74ffb77e5afd70b9. The four active verbs require meaningful public results, while export remains explicitly pending under QSL-1979. The new grader lacks bounded subprocess lifecycle handling, and its five test trace tags name the sandbox criterion rather than the behavior they exercise.

## Verdict

**CONDITIONAL** — resolve FND-001 and FND-002. The authorized Codex sample and Claude authentication prerequisite remain pending; no live result is claimed.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | EV-009 declares no assertion budget and calls both the domain generator and all four public FSL commands with unbounded `Command::output()`. A stalled child can outlive the SDK's provider timeout without a `harness-error: assert budget exceeded` result or termination/reaping. Apply one declared assertion deadline across grading and its children, including domain regeneration, and classify exhaustion as a harness error. | agent-evals/src/scenarios/blind_help.rs:96; agent-evals/src/scenarios/blind_help.rs:267; agent-evals/src/scenarios/blind_help.rs:297; agent-evals/fixtures/EV-009/scenario.toml:1 | correct-requirement-no-evidence |
| FND-002 | low | All five new grader tests trace FR-001-AC-6, whose contract is transcript sandbox-breach detection. These tests call the scenario grader or semantic decoders directly and cannot fail when the transcript sandbox scan regresses. Remove the inaccurate bindings or bind only to actual owning criteria; do not count these semantic/input-integrity tests as sandbox evidence. | agent-evals/src/scenarios/blind_help.rs:374; agent-evals/src/scenarios/blind_help.rs:384; agent-evals/src/scenarios/blind_help.rs:394; agent-evals/src/scenarios/blind_help.rs:404; agent-evals/src/scenarios/blind_help.rs:426 | correct-requirement-no-evidence |

## Coverage

Rust review ran within code-review: real command construction, resource lifecycle, immutable regular-file inputs, semantic output checks, per-verb aggregation, test assertions and trace tags. Inspected the new registration's calls into the existing command-evidence seam; the monitor operand correctly binds its named `--input` file. Shared framework implementation was not re-reviewed.

Examined scope: the new `blind_help.rs`; its `lib.rs` module registration and provider entry; README usage; and all original EV-009 files (`analyze-request.json`, `calculation.fsl`, `domain.mjs`, `initial.json`, `inventory.fsl`, `monitor-request.json`, `monitor.fsl`, `readings.jsonl`, `run-request.json`, `scenario.toml`, `task.md`). FR-001's provider-budget description, AC-6 and AC-10 were read as contract context. No applicability claim is made for an assurance profile outside this changed scope.

Five distinct new grader checks and integrated scoped Clippy are accepted from the author. No tests, builds, CLI probes, matrix, repository inventory, SDK registration, doctor, authentication or live runs were repeated. The earlier stopped combined-filter incident is not counted as product evidence. No duplicate-helper search was performed.

## Review context

Ticket: QSL-2493. Repository: agent-ix/filament-examples, public. PR: pending. Reviewer run: f58865ae-df7e-4046-877b-64e6c29a586b. Exact model identifier is unavailable to this reviewer. Native full-domain authoring and a successful export are not claimed by this feature.
