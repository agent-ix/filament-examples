---
title: Declare an intended terminal state for a protocol
type: how-to
---

EV-006 exercises FR-211's protocol deadlock-freedom item. The fixed Fill
protocol has two interleaved branches: `A` increments a cell and `B` sets it to
2 only when its value is 0. Taking `A` first reaches a terminal state with a
live protocol instance: `B` is blocked by `TwoPre`, and the root waits at the
join. Since the state is not declared intended, the baseline deadlock-freedom
item is refuted.

The requested `terminal when { k.v = 1 };` declaration makes that state an
intended end state. The other terminal state is reached after the protocol
instance finishes. The repaired deadlock-freedom item therefore proves
exhaustively.

The task retains both full `fsl analyze` outputs. The provider independently
runs the original fixed baseline and the submitted repaired source, checks the
counterexample and blocked-thread evidence, and requires the repaired
exhaustive proof to match the current source.

The protocol analysis route is still subject to release availability. An
unsupported or incomplete command result is a tool capability outcome and
never counts as a proof or as an agent failure.
