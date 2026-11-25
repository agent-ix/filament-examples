---
title: Model a protocol and repair its deadlock
type: how-to
---

EV-006 asks the agent to model Fill from English, analyze the deadlock, and
repair the protocol using either the intended terminal state or a corrected
precondition. The grader runs `fsl analyze` against the agent's own baseline and
repaired source and compares the saved outcomes with those independent runs.
It checks the deadlock's structured prefix, both blocked thread paths and
causes, and the repaired certified exhaustive proof without requiring
reference identifiers. It structurally compares the two parsed sources so the
repair preserves both branches and changes only an intended terminal member
or the blocked branch's precondition predicate. The blocked branch is
identified from the precondition clause in the trusted counterexample,
independent of source branch order. The protocol terminal member follows the
role declarations and precedes the run.

The grader evaluates the changed terminal or precondition predicate with the
public `fsl run` command for every cell value in the bounded `0..3` domain. The
predicate must be false for at least one value, and it must hold at the cell
value recorded in the trusted deadlock state. This rejects semantic tautologies
such as `self.v = self.v` as well as literal `true` predicates.

The grader's `fsl-cst` and `fsl-foundation` dependencies pin to FSL default
branch revision `0443d782436d33bd0a178d2745dca158e8e4acf5`, so the structural
check uses the current FSL parser. If that parser refuses source accepted by
the trusted CLI, grading reports `cli-gap:`.

Offline release verification used FSL version 0.2.0, revision
`5275e5086fec403c4d1a6bce25ab7c3660ea5716`. The baseline analysis exited 10
and contained the expected `Main/Both` fork, `Main/Both/left/A` attempt,
`TwoPre` block at `Main/Both/right/B`, and join wait. With the protocol terminal
member after the role, the intended-terminal repair exited 0; the `TwoPre`
precondition repair also exited 0. Each returned a certified exhaustive proof.
The stored CLI outcomes support offline stub tests; grading uses fresh trusted
CLI results.

The offline FSL 0.2.0 release at the same revision evaluated both
`probeValue = probeValue` and `not ( probeValue < 0 )` to `true` at every value
from 0 through 3. `reference/predicate-run-true.json` is the captured
`fsl run --format json` output replayed by the grader test. A refusal while
evaluating the generated predicate probe is a grader `harness-error:`, because
the trusted analysis has already accepted the agent's source.

The eight EV-006 scenario grader tests carry trace tags to FR-004. The
standalone `compare_items_checks_trusted_fields` unit test is not one of the
scenario grader tests and remains untagged. The requirement and test case are
tracked by
[QSL-3018](https://linear.app/agent-ix/issue/QSL-3018/agent-evals-add-ev-006-protocol-deadlock-acceptance-criterion-spec).

Tool execution failures and timeouts use the `harness-error:` class. An
unsupported protocol analysis route is a `cli-gap:` result. Neither is counted
as an agent artifact failure. Live Codex/Claude acceptance runs and the
scorecard remain pending; QSL-2490 stays open until that acceptance work is
completed.
