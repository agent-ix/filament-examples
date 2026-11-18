# Lock

The lock is an example of two processes that share one lock, and of two processes that take two locks in opposite orders.

You can read the rules that say when a lock can be acquired and released, and the rules about fairness that say which process is served. The two-lock units show a deadlock: a state in which neither process can take a step.

## The models

- `lock` has one `Lock` object with an owner: 0 when the lock is free, and 1 or 2 when a process holds it. The operation `acquire` takes the lock for a process, and `release` frees it.
- `two-locks` has one `Locks` object with two locks, A and B. Process 1 takes A and then B. Process 2 takes B and then A. The operations are `lockFirst`, `lockSecond` and `unlock`.

The models are in [model](model).

## Fairness

[rules/fairness/one-served.fsl](rules/fairness/one-served.fsl) holds the rule "process 1 holds the lock again and again" four times, with no fairness, with weak fairness on `acquire`, with weak fairness on each acquire and with strong fairness on each acquire.

- `fairness-without-fairness`, `fairness-weak-whole`, `fairness-weak-each` and `fairness-strong-each` compile the four rules.
- `fairness-counted-window` shows that a rule with a counted window carries no fairness constraint.
- `fairness-unknown-operation` shows that a fairness constraint names an operation of the model.

## Model checking

`fsl analyze` decides a rule over every behavior of the lock, starting from the free lock in [data/lock-free.json](data/lock-free.json). The requests are in [requests/analyze](requests/analyze).

- `analyze-served-strong-each` checks "process 1 holds the lock again and again" with strong fairness on each acquire. It holds on every behavior, so the result is `proved` and `fsl analyze` exits 0.
- `analyze-served-weak-each` checks the same rule with weak fairness on each acquire. It is `refuted`: the counterexample is a loop in which process 2 takes every turn, and `fsl analyze` replays it before it reports it. It exits 10.

Run one with `fsl analyze requests/analyze/served-weak-each.json --format json`, or with `--format text` for one line per rule.

## Deadlock

`model-subjects-two-locks` compiles the rules of the two-lock system. When process 1 holds A and process 2 holds B, each waits for the other's lock, and no operation is enabled.

`staged-two-locks-deadlock` asks `fsl analyze` whether the two-lock system can get stuck. It is `refuted` with a counterexample of two steps: process 1 takes A, then process 2 takes B. `fsl analyze` replays the counterexample before it reports it, and it exits 10.

`two-locks-ordered` is the same system with both processes taking lock A first. No state leaves each process holding the lock the other waits for, so `fsl analyze` proves it deadlock-free and exits 0.

## Protocols

The protocol `Handover` has an asker send one request to a holder over a channel. `protocol-handover-heard` checks that every run reaches the finish, and `fsl analyze` proves it. In `protocol-handover-unheard` the holder's receive has a guard that never holds, so the request stays on the channel; the counterexample names the blocked receive and the case exits 10.

A recorded run is checked against the same protocol. `protocol-recording-conforms` replays a recording of the send, the receive and the finish, and every obligation is satisfied. `protocol-recording-unasked` replays a recording in which the holder hears a request nobody sent: the send it needs is missing, and the case exits 10.
