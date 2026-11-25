---
type: reference
---

# Engines cases

Engines is a corpus for the analysis engines that `fsl analyze` routes to and for the command-line verbs around them. Each case submits a request and compares the outcome with a recorded result.

Run the cases from the repository root:

```sh
cargo run --quiet --locked --manifest-path tools/fsl-examples/Cargo.toml -- check engines
```

Set `FSL` to the driver binary. Expected records are compared with the output from that binary; regenerate changed records against the release candidate before shipping it.

## Cases

- `verb-*` run the verbs a build accepts but does not route (`prove`, `generate`, `render`), an option a verb does not have, and the `--integer-division` option of `check`.
- `hybrid-*` analyze a radio that harvests, transmits and idles in a six-second cycle. Each bound is tested on both sides of the value the charge reaches (8 and 2). One case shows that an equality the flowpipe cannot prove stays inconclusive. Three more declare a hybrid model that is refused or admitted: a clock-like rate of 1, the rate `2 - 1`, and an initial mode that does not exist.
- `timed-*` analyze a rover that brakes and halts in 50 to 100 ms each. The deadline is tested at 199, 200 and 201 ms, and the earliest stop at 99 and 150 ms.
- `probability-*` analyze a fair coin that starts as tails. Each bound is tested on both sides of the exact value 1/2 and 3/4, with `>=` and `<=`; the atom and always claims now settle exact outcomes. One case shows that a strict `>` has no spelling.
- `schedulability-*` state two task sets and a response bound with their exact boundaries (task a has priority 1 and task b priority 2; a smaller value is a higher priority, FR-419, so a responds in its own 5 ms).
- `deadlock-*` declare a tank that fills and cannot drain, with and without a `terminal` declaration, and ask for its deadlock-freedom item explicitly beside one authored temporal clause. Two more show that `terminal any` adds no deadlock item and that one is derived beside an authored claim.
- `refinement-*`, `hyperproperty-*` run a refinement whose strong abstract fairness is refuted and a copy-swap hyperproperty over the tank; `composition-*`, `planning-*` and `synthesis-*` are staged.
- `simulation-*` enumerate and sample traces of a tank and check the request members of a simulation item.

## Staged

- `composition-tank-bounds` waits on QSL-2739 (composition unsupported), `planning-tank-goal` and `synthesis-tank-controller` wait on QSL-1978 (the planning and synthesis item kinds are refused).

## Coverage

| Criterion | Cases |
| --- | --- |
| FR-522-AC-1, FR-301-AC-4 unrouted verbs exit 21 and name the verb | `verb-prove-not-routed`, `verb-generate-not-routed`, `verb-render-not-routed` |
| FR-301-AC-3 unknown options refuse | `verb-unknown-option` |
| FR-301-AC-6 integer-division option | `verb-integer-division-*` |
| FR-193-AC-6 proved by a bounded solver, no solver result settles refuted without a witness | `hybrid-at-most-eight`, `hybrid-at-least-two`, `hybrid-reach-point-inconclusive` |
| FR-193-AC-22 replayed refutation | `hybrid-below-eight`, `hybrid-above-two` |
| FR-193-AC-7 rate-one hybrid | `hybrid-rate-one-refused`, `hybrid-rate-expression-admitted` |
| FR-193-AC-11 initial mode | `hybrid-initial-mode-missing` |
| FR-415, FR-416 timed claims | `timed-*` |
| FR-407 claim form | `probability-strict-comparison-refused` |
| FR-411 exact probability, both comparators | `probability-within-*` |
| FR-407-AC-1, FR-411, FR-413 exact atom and always claims over the coin | `probability-atom-now`, `probability-atom-now-at-most`, `probability-tails-through-one` |
| FR-419 schedulability | `schedulability-*` |
| FR-366-AC-1, AC-2 deadlock-freedom item | `deadlock-terminal-covers-state`, `deadlock-no-terminal-refuted`, `deadlock-terminal-misses-state` |
| FR-366-AC-3 terminal any | `deadlock-terminal-any-adds-no-item` |
| FR-366-AC-4 one derived item | `deadlock-derived-item-beside-temporal` |
| FR-375 to FR-380 refinement liveness, strong abstract fairness | `refinement-strong-fairness-refuted` |
| FR-191, FR-395 hyperproperty | `hyperproperty-same-steps-same-level` |
| FR-348, FR-489 composition | `composition-tank-bounds` (staged) |
| FR-194, FR-800 to FR-810 synthesis and planning | `synthesis-tank-controller`, `planning-tank-goal` (staged) |
| FR-366-AC-6 two terminal declarations | `deadlock-two-terminals-refused` |
| FR-181, FR-520 simulation items | `simulation-*` |

Not covered here: timed refinement with a timed abstract side and hidden-field refinement (QSL-2243, no failing request was found), finite hyperproperty reductions (QSL-2248), verification plans (FR-192; area 7 covers the plan request), protocol analysis (the lock system), the `version` verb (FR-560: this driver build has no `version` verb) and the probability and model-checking verdicts of the claims system.
