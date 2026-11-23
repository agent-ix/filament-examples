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
or the second branch's precondition predicate.

Offline release verification used FSL version 0.2.0, revision
`5275e5086fec403c4d1a6bce25ab7c3660ea5716`. The baseline analysis exited 10
and contained the expected `Main/Both` fork, `Main/Both/left/A` attempt,
`TwoPre` block at `Main/Both/right/B`, and join wait. Both repairs were run:
the intended-terminal repair exited 0, and the `TwoPre` precondition repair
exited 0. Each returned a certified exhaustive proof. The stored CLI outcomes
support offline stub tests; grading uses fresh trusted CLI results.

Tool execution failures and timeouts use the `harness-error:` class. An
unsupported protocol analysis route is a `cli-gap:` result. Neither is counted
as an agent artifact failure. Live Codex/Claude acceptance runs and the
scorecard remain pending; QSL-2490 stays open until that acceptance work is
completed.
