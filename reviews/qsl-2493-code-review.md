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

**PASS** — both original findings are fixed at c716e73489d6a3e5ced43d2e47687a1a7b4c558c. The approved shared capture helper was accepted as a peer fact; only EV-009's integration was inspected. Live acceptance and the Claude authentication prerequisite remain pending.

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

## Dispositions

### Round 1

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | The manifest declares 120000 ms. `grade_with_budget` creates one absolute deadline before `assess`; validation, domain regeneration, all four public commands and final checks share that deadline. Calls use the independently approved bounded capture API, and capture failures become structured harness errors with timeout and reaped-child evidence. Artifact reads are bounded to one MiB plus one byte. |
| FND-002 | fixed | All five inaccurate `Trace: FR-001-AC-6` annotations were replaced with ticket provenance, preserving the tests without claiming sandbox-criterion evidence. |

Disposition scope was only EV-009's caller, manifest and trace changes from 1494e6b8 to c716e734. The shared helper's approval and API contract were accepted without reopening its implementation, registration or EV-003 wrapper review. The reported new own-grade deadline test, quota edge evidence and current all-target Clippy pass were accepted; no old grader, helper, public CLI, build or live checks were repeated. Merge of the approved shared helper first is delivery ordering, not an additional review gate for this caller.
