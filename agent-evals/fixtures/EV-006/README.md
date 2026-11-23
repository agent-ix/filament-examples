---
title: Model a protocol and repair its deadlock
type: how-to
---

EV-006 asks the agent to model Fill from the supplied English behavior, analyze
the baseline, and repair it by either declaring the intended end state or
correcting the precondition so it permits the state reached after A. The task
asks for both full `fsl analyze` outputs and a concise report of the observed
results, ordered steps, blocked thread paths/causes, and proof basis.

The reference source and captured outcomes are provider-only test fixtures.
They are not copied into the agent workspace. Both captured release outcomes
were verified with `/home/peter/fsl-rc3/fsl analyze` offline: the baseline
exited 10 and reported the `Main/Both` fork, `Main/Both/left/A` attempt,
`TwoPre` block at `Main/Both/right/B`, and join wait; the repaired source with
the intended terminal declaration exited 0 with a certified exhaustive proof.
These captures support offline stub tests; grading always compares the agent's
files with fresh trusted CLI results.

An unavailable protocol route, unsupported/incomplete result, tool failure, or
timeout is reported with a `harness-error:` prefix and never counts as an agent
failure or as proof.
