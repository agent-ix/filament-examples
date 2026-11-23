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

The eight EV-006 grader tests remain untagged because agent-evals has no
criterion for this scenario. The spec-first deferral is tracked by
[QSL-3018](https://linear.app/agent-ix/issue/QSL-3018/agent-evals-add-ev-006-protocol-deadlock-acceptance-criterion-spec).

Tool execution failures and timeouts use the `harness-error:` class. An
unsupported protocol analysis route is a `cli-gap:` result. Neither is counted
as an agent artifact failure. Live Codex/Claude acceptance runs and the
scorecard remain pending; QSL-2490 stays open until that acceptance work is
completed.
