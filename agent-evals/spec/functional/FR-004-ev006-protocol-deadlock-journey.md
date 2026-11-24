---
id: FR-004
title: "Scenario EV-006: model and repair a protocol deadlock"
type: FR
relationships:
  - target: ix://agent-ix/filament-examples/agent-evals/FR-001
    type: depends_on
  - target: ix://agent-ix/filament-examples/agent-evals/FR-002
    type: depends_on
---

# FR-004: Scenario EV-006 protocol deadlock journey

## Description

EV-006 asks an agent to model the Fill protocol from an English task, analyze
its deadlock, and repair it in one of two intentional ways: declare the
protocol's terminal state or relax the precondition of the blocked operation.
The grader independently runs the public `fsl analyze` and `fsl run` commands,
then checks structured results and the relationship between the baseline and
repaired source. It accepts either repair only when the result is a certified
exhaustive proof of deadlock freedom. This is agent-evaluation coverage for
one protocol task; it does not claim language-conformance coverage for
FSL FR-211.

The grader prepares fixed task inputs, records the trusted deadlock outcome,
and identifies the blocked operation from that outcome rather than source
branch order. The agent's own baseline and repair remain the artifacts under
evaluation. The grader compares structured CLI fields instead of prose or
reference identifiers, and treats the blocked thread list as unordered while
preserving the ordered execution prefix.

### Grader, harness, and CLI-gap boundary

A wrong or missing agent artifact is an agent failure. An analysis route that
the released CLI does not support, or a parser refusal for source accepted by
the trusted CLI, is a `cli-gap`. A tool execution failure or timeout while the
grader performs trusted analysis or bounded predicate probes is a
`harness-error`; it is not reported as an agent artifact failure. A predicate
probe refusal after trusted analysis accepts the source is also a
`harness-error`.

## Inputs

- The English EV-006 task and fixed model, start state, and analysis requests in
  `fixtures/EV-006/`.
- The agent-authored baseline and repaired protocol sources.
- The released `fsl analyze` and `fsl run` commands through the FR-001 tools.

## Outputs

- One independent assertion result for each EV-006 sample, with structured
  checks for the deadlock, source repair, predicate probes, and exhaustive
  proof.
- Failure names classified at the grader boundary as specified above; other
  failed samples are classified by FR-002 triage.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-004-AC-1 | The EV-006 grader prepares the fixed English task inputs and refuses changed inputs without requiring a hidden reference implementation. | Test (TC-004) |
| FR-004-AC-2 | The trusted baseline result is accepted as the Fill deadlock only when its structured outcome has the expected ordered fork-and-attempt prefix and both blocked thread paths with their causes. | Test (TC-004) |
| FR-004-AC-3 | The grader compares saved baseline and repaired summaries with fresh CLI structures, ignores ordering within the blocked list, and preserves ordering within the execution prefix. | Test (TC-004) |
| FR-004-AC-4 | The grader accepts a terminal repair only when the source preserves both parallel branches and their attempts and adds a non-vacuous terminal declaration after the protocol roles and before its run. | Test (TC-004) |
| FR-004-AC-5 | The grader accepts a precondition repair only when the source preserves both parallel branches and their attempts and changes the precondition of the operation blocked in the trusted deadlock result. | Test (TC-004) |
| FR-004-AC-6 | The grader evaluates the changed terminal or precondition predicate for every cell value in `0..3` and accepts it only when it is false for at least one value and true at the trusted deadlock value. | Test (TC-004) |
| FR-004-AC-7 | The repaired analysis is accepted only when the Fill item is proved with an exhaustive basis, certified result, and accepted certificate. | Test (TC-004) |
| FR-004-AC-8 | The grader accepts either intended repair and classifies unsupported analysis as `cli-gap`, while trusted tool execution failure or timeout is `harness-error`. | Test (TC-004) |

## Dependencies

- [FR-001](./FR-001-agent-eval-harness.md) for the sandbox, provider, and
  released tools.
- [FR-002](./FR-002-scheduled-runs-scorecard-and-triage.md) for committed run
  results and triage.
- FSL FR-211 concerns language conformance and is not traced by these
  scenario-grader criteria.

