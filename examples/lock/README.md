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

## Staged

These cases are staged until the language supports them.

- `staged-two-locks-deadlock`: The two-lock system can reach a state where neither process can move. Needs deadlock-freedom items in `fsl analyze`.

`example.toml` records the exact reason and ticket for each.
