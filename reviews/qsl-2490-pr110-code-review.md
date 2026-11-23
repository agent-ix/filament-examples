---
id: SR-1524
title: "QSL-2490 PR 110 code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-examples; QSL-2490; PR 110 head b8dfba61aad6e399703a52900120b667ce298e5c; agent-evals/src/scenarios/protocol_deadlock.rs, agent-evals/src/provider.rs, agent-evals/src/lib.rs, agent-evals/Cargo.toml, agent-evals/Cargo.lock, agent-evals/fixtures/EV-006/*"
review_set: subset
relationships:
  - { target: "ix://agent-ix/filament-examples/FR-001", type: references }
---

## Summary

Ticket: QSL-2490. This is a diff-scoped review of filament-examples PR 110, which adds scenario EV-006 (declare an intended protocol terminal state and prove deadlock freedom). The Rust review lane is folded in. The author is a codex coder, and both commits carry the identity `Peter Krenesky <kreneskyp@gmail.com>`.

## Verdict

**FAIL.** There is one high finding: the grader's expected `result.json` disagrees with the task text, so an agent that follows the task exactly always fails.

The provider change only registers the new entry. It does not touch the transcript boundary or the rg/sed rules. This branch is based on main without PR 109, so expect a merge conflict in provider.rs, not a weakening. No scorecard or result claims a live run.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The task tells the agent to write `blocked` as `["(0, Both, right, 0) at B: Precondition{TwoPre}", "(0) at join Both: JoinWaiting{[(0, Both, right, 0)]}"]`. `check_summary` requires exactly `["B: Precondition{TwoPre}", "join Both: JoinWaiting"]`. An agent that copies the task's own example fails with WrongSummary every time. No test exercises `check_summary`. | agent-evals/src/scenarios/protocol_deadlock.rs:315-336; agent-evals/fixtures/EV-006/task.md:24-30 |
| FND-002 | medium | The task gives away the whole answer: the exact member to add (`terminal when { k.v = 1 };`) and the complete expected `result.json`. The prefix, blocked threads, both exit codes and the basis are all stated. The summary and protocol checks therefore test copying, not reading the counterexample. The grader also accepts only that exact line (whitespace-insensitive), which shuts out the repair the ticket also allows: the precondition fix, `TwoPre: self.v <= 1` per FR-211-AC-3. | agent-evals/fixtures/EV-006/task.md:12-31; agent-evals/src/scenarios/protocol_deadlock.rs:132-163 |
| FND-003 | medium | A protocol route that is unavailable or incomplete, a tool failure, or a timeout all come back as plain `UnsupportedAnalysis`, `ToolFailure` or `Timeout` failures. None is mapped to the `harness-error:` prefix, which the sibling `safety_analyze` uses for an exceeded budget, or to a CLI-gap class. So they count as failed agent samples. That contradicts the fixture README ("never counts as a proof or as an agent failure"). The failure text also doubles the code (`{code:?}: {fault}`, where `Display` already prints the code). | agent-evals/src/scenarios/protocol_deadlock.rs:165-171,370-383; agent-evals/fixtures/EV-006/README.md:23-25 |
| FND-004 | medium | The tests cover only three pure helpers with synthetic JSON. There is no test of `check_summary` (FND-001 would have failed it), `compare_items` or `assess`/`grade`, either with a stub `fsl` or with a recorded release outcome. The expected release output was never checked against the real `fsl analyze`: exit 10 and 0, claim name `Fill`, `Main/Both/left/A`, a certified exhaustive basis. The reference lane was not run. | agent-evals/src/scenarios/protocol_deadlock.rs:386-449 |
| FND-005 | low | The counterexample check is loose: `encoded.contains("B")` is satisfied by `Both`, and `contains("right")` and `contains("fork")` match anywhere in the JSON. It is weaker than the precise prefix and blocked-thread shape FR-211-AC-2 states. `compare_items` against the trusted run is the real check, but a changed FSL counterexample would still pass `check_deadlock` itself. | agent-evals/src/scenarios/protocol_deadlock.rs:226-245 |

## Coverage

- **Diff:** 13 files, 866 insertions, 1 deletion, in 2 commits (c45a9a00, b8dfba61).
- **Boundary:** `provider.rs` only adds an `ENTRIES` row with `shipped_references: true` and two `analyze` evidence commands. No boundary, allow-list or path rule changed.
- **Fixtures:** no reference answer file is shipped into the workspace. `prepare` writes only the given inputs. The answer leaks through `task.md` instead (FND-002).
- **Dependencies:** the `ix-trace-rs` dev-dependency is pinned by git tag v0.1.2. Its lock entry already exists, and `Cargo.lock` only adds the dependency edge.
- **Tests:** not run by the reviewer (titan load). The author reports protocol_deadlock::tests 3 passed, provider::tests 20 passed, and fmt and clippy clean.

## Examined units

- protocol_deadlock.rs prepare, check_inputs, analyze, check_deadlock, check_proof, compare_items, check_summary, assess and grade, plus tests: examined.
- fixtures/EV-006 task.md, README.md, given.native, both request files, start.json and scenario.toml: examined.
- provider.rs ENTRIES row: examined.
- FSL FR-211-AC-2 and FR-211-AC-3: context_only.
