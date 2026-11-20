---
id: SR-1439
title: "Code and Rust review of EV-002 refusal-only repair"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-examples; QSL-2486; fa959c7..b1ee66a7; agent-evals/src/scenarios/refusal_repair.rs, agent-evals/src/provider.rs, agent-evals/src/lib.rs, agent-evals/fixtures/EV-002/"
review_set: subset
---

## Summary

Ticket: QSL-2486. Reviewed the new refusal-only repair scenario, original inputs, independent public-CLI grader, catalogue entry and reference-exposure switch. PR opening is pending the repository queue. Shared framework changes through fa959c7 were excluded from review; its boundary function was read only to establish how the new scenario policy is applied.

## Verdict

PASS at disposition round 1, head 9a263ba6760d25b6731b1bd77f41290197903ad7. FND-001 is fixed. The eight grader-owned inputs, absolute trusted executable, Integer-result checks and negative candidates provide meaningful repair checks. Live acceptance remains pending; this is approval of the implementation fix, not a live success claim.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | EV-002 forbids documentation reads, but shipped_references=false only suppresses preparation links. Assert still applies the shared boundary policy that permits reads beneath tools.docs, so a recorded read of the configured documentation directory can receive a passing repair score. Apply the entry's reference policy during transcript grading and cover EV-002 rejection alongside EV-001 permission. | agent-evals/src/provider.rs:37; agent-evals/src/provider.rs:789; agent-evals/src/provider.rs:865; agent-evals/fixtures/EV-002/task.md:6 | correct-requirement-no-evidence |

## Coverage

Examined all six changed files in fa959c7..b1ee66a7. The task's charge rule, fixed function contract, refusal-only information boundary, public check/run commands and sentinel-independent result requirement were compared with preparation and grading. Reviewed the new Rust API visibility, recovery paths, filesystem use and real subprocess calls. No vendored fixture or substitute evaluator was added.

The concrete failure path for FND-001 is a valid repair plus a recorded `cat <configured-docs-root>/fsl/<document>` call: `apply_evidence` calls the shared boundary checker without the Entry, and its read-only predicate accepts paths beneath `tools.docs` regardless of `shipped_references`.

Accepted the coder's four new deterministic grader passes, authoring Clippy pass and two confined integration compile passes. No tests, builds, CLI calls, authentication checks, SDK registration or existing canaries were rerun. The task-specific documentation-policy regression is not present among those four tests: the existing docs-absence assertion calls scenario preparation directly, rather than provider preparation/assertion.

No applicable assurance profile was supplied for this bounded diff; no repository-wide profile or coverage inventory was performed.

## Dispositions

| Finding | Round | Outcome | Evidence |
| --- | --- | --- | --- |
| FND-001 | 1 | fixed | At 9a263ba6760d25b6731b1bd77f41290197903ad7, apply_evidence passes entry.shipped_references through transcript, command and path inspection. The path check rejects published documentation/grammar before the workspace allow rule when the entry forbids references. Two new retained-transcript tests reject EV-002 documentation/grammar reads, including workspace symlinks, preserve task-source and check/run evidence, and retain EV-001 reference access. |

Disposition scope was only b1ee66a7..9a263ba6760d25b6731b1bd77f41290197903ad7 and the original finding. Accepted the coder's two focused policy-test passes, exit 0, recorded in .agent/qsl-2486-reference-policy.log. No tests, builds, authentication probes, CLI executions, shared-framework reviews or live samples were repeated. No additional findings.
