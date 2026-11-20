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

PASS after the final runtime-fix disposition at 5addd7283ee6e1a14954644ad06e776ff8ec3bd0. FND-001 and FND-002 are fixed. The eight grader-owned inputs, absolute trusted executable, Integer-result checks and negative candidates provide meaningful repair checks. Ready for one retained command/boundary evidence grade; no successful retained grade or new live sample is claimed.

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

## New Findings: Runtime Fix Review

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-002 | medium | The new AST command-evidence parser accepts exit as an ordinary top-level command. Consequently, exit 0 followed by fsl check postage.fsl and fsl run postage-example.json can establish both required proofs even though the shell exits before either invocation. Reject or stop evidence collection at shell termination before counting later commands, and add the concrete early-exit regression. | agent-evals/src/provider.rs:375; agent-evals/src/provider.rs:405; agent-evals/src/provider.rs:431; agent-evals/fixtures/EV-002/task.md:11 | implementation-bug-despite-evidence |

### Runtime Disposition

| Finding | Round | Outcome | Reason |
| --- | --- | --- | --- |
| FND-002 | 2 | still-open | At 6dc18a9, exit is absent from the rejected command set; each later root command is still appended and can match the required public invocation. |
| FND-002 | 3 | fixed | At 5addd728, AST evidence collection stops at literal exit, exec or return and non-query command-builtin execution, preserving earlier reachable evidence without crediting later invocations. One new focused test covers the heredoc/termination tails, reachable prefix and valid commands before a final exit. |

The bounded runtime review inspected only ff7f82fc..6dc18a921b1b52f1370ace6235c857bb8b63b512: the tree-sitter dependency declarations/lock entries, AST recognizer, matcher call change and two new regressions. Published parser crates are declared registry dependencies; no parser code is copied into the repository. Heredoc bodies, comments, control-flow nodes, function definitions, assignments and the listed shell-state modifiers do not establish positive evidence. No script is executed or expanded by the parser. The concrete remaining input is `exit 0\nfsl check postage.fsl\nfsl run postage-example.json\n`; all three are simple root command nodes, but only the first actually executes.

Accepted the supplied two passing focused heredoc regressions and the actual live sample's public check, eight semantic executions and access-policy results. Original SDK report 0/1 remains unchanged; the failed command-evidence classification is not a fresh live failure. No tests, builds, semantic probes, live samples, authentication checks or prior framework reviews were repeated. The original documentation-policy finding was not reopened.

Final runtime disposition inspected only 6dc18a9..5addd7283ee6e1a14954644ad06e776ff8ec3bd0 and FND-002. Accepted the supplied new termination test's 1/1 pass, exit 0, in .agent/qsl-2486-termination-focused.log. No new findings or additional review sweep; all previous findings remain closed. No tests, builds, semantic probes, authentication checks or live samples were repeated. Record round 3 is the disposition of the finding introduced in record round 2, not a third coder fix round.
