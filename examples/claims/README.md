---
type: reference
---

# Claims cases

Claims is a corpus for the claim and temporal forms of FSL. Each case submits a rule, a trace or a model and compares the outcome with a recorded result.

Run the cases from the repository root:

```sh
cargo run --quiet --locked --manifest-path tools/fsl-examples/Cargo.toml -- check claims
```

Set `FSL` to the driver binary. Every record in this system was written by the driver at FSL f5977125 (`fsl-driver-aa71f20dd30bab9ae2ca66580ca407e434164cca`, version string 0.1.4).

## Cases

- `future-*` read the future operators (`eventually`, `always`, `until`, `release`, nested forms, bounded and unbounded intervals) on an infinite trace written as a prefix and a loop. Each case names the position where `p` and `q` hold, so the expected truth is stated in the caption. Interval endpoints are tested on both sides, including a zero-width interval and a lower bound with no upper bound.
- `future-activation-*` check a clause that activates on each position where a guard holds: every activation must hold, and a trace with no trigger is inactive and has no truth value.
- `past-*` read `once`, `historically`, `since` and `triggered` at position 3 of a trace, including a look-back that reaches before the first position.
- `window-*` read a clause under the timestamped finite-window profile: inclusive endpoints, a stamp just outside the interval, empty windows, completeness, pending and indeterminate results, tied stamps, listing order and the lower boundary of a look-back.
- `false-extension-*` read a clause over a closed finite stream under a false-extension profile, where a missing position reads every atom as false and `not holds(p)` differs from `holds(not p)`.
- `model-*` analyze invariants, temporal claims and state-graph claims (`possible`, `always possible`, `unique path`) over a dial that counts 0 to 3. `rules/dial.fsl` has the full dial, `rules/dial-acyclic.fsl` counts up only, and `rules/dial-branch.fsl` adds a step of two.
- `refusal-*` compile one claim declaration and record the refusal the standard names, or the successful compile for the boundary beside it: clocks over a model and over a trace, profiles, intervals, Boolean bodies, fairness and probability thresholds.

The running case `model-possible-trap-cyclic` refutes a possible claim whose target no state satisfies, over a dial that can return to 0. Its request carries no deadline: the exploration of the four states ends on its own, so the verdict does not depend on how fast the machine is.

## Staged

- `window-absent-valuation` waits on QSL-2750: the stepping wire refuses a sample that omits a field (`invalid_runtime_input`/`missing-member`), where FR-091-AC-13 reads the absent atom as a missing valuation and settles indeterminate (exit 11).
- `window-order-keys-admit-until` waits on QSL-2750: two observations stamped 1 with distinct keys from one authority are read as one instant and give a conflicting valuation (exit 11), where FR-091-AC-16 orders them and until is true (exit 0).

Not assertable: FR-091-AC-12 says a pending result carries its deadline (5) and AC-16 says the refusal is at the third position. The recorded outcome carries neither, and a staged case needs an exit that differs from the required one, so no case can state them.

## Coverage

| Criterion | Cases |
| --- | --- |
| FR-091 bounded future operators, inclusive endpoints, zero width, lower bounds, nesting (AC-1, AC-3) | `future-ev-0to2-*`, `future-ev-at1-*`, `future-al-0to2-*`, `future-until-1to2-*`, `future-release-1to2-*`, `future-ev-from2-*`, `future-al-from1-*` |
| FR-091-AC-2, AC-7 false extension and pointwise negation | `false-extension-always-one-position`, `false-extension-always-two-positions`, `false-extension-eventually-not-p-one-position`, `false-extension-always-not-p-one-position`, `false-extension-holds-not-p-*` |
| FR-091-AC-3 until lower-bound convention | `future-until-1to2-p-off-at-anchor`, `false-extension-until-1to2-p-off-at-anchor` |
| FR-091-AC-6 unbounded interval under a bounded profile | `refusal-unbounded-under-bounded`, `refusal-lower-only-under-bounded` |
| FR-091-AC-10, AC-11 window truth tables and empty windows | `window-always-*`, `window-eventually-*`, `window-stamp-*`, `window-single-*`, `window-empty-*` |
| FR-091-AC-12 pending and decisive early settlement (the deadline value is not in the outcome) | `window-pending-before-deadline`, `window-settled-at-deadline`, `window-open-scope-*` |
| FR-091-AC-14, AC-15 completeness, the inclusive deadline and the half-open watermark | `window-half-open-watermark`, `window-incomplete-*`, `window-complete-*`, `window-deadline-instant-participates`, `window-after-deadline-ignored` |
| FR-091-AC-13 conflicting valuation; AC-16 unkeyed ties and order keys | `window-unkeyed-ties-until`, `window-conflicting-same-stamp`, `window-agreeing-same-stamp`, `window-order-keys-two-authorities`, `window-order-keys-admit-until` (staged) |
| FR-091-AC-13 absent valuation | `window-absent-valuation` (staged) |
| FR-091-AC-18, AC-20, AC-25 counter, exact and default cover | `window-cover-*` |
| FR-091-AC-17 look-back boundary | `window-once-*` |
| FR-091-AC-22, AC-23 listing order and an empty trace | `window-listing-order-ignored`, `window-no-observations` |
| FR-092 bounded past operators; FR-161-AC-12 non-strict since and triggered | `past-*` |
| FR-161, FR-367 infinite-trace operators, loops | `future-recur-*`, `future-persist-*`, `future-ev-loop`, `future-al-loop-gap`, `future-activation-in-loop` |
| FR-093 activation (AC-5) | `future-activation-*` |
| FR-090-AC-2, AC-11, AC-12 profile and clock selection | `refusal-no-clock`, `refusal-*clock*`, `refusal-unknown-profile` |
| FR-019-AC-6 invariant anchor and Boolean body | `model-invariant-*`, `refusal-invariant-not-boolean` |
| FR-390, FR-391 state-graph claims (AC-1 to AC-4) | `model-possible-*`, `model-always-possible-*`, `model-unique-path-*` |
| FR-391-AC-2 trap over a cyclic graph | `model-possible-trap-cyclic` (exit 10) |
| FR-407 probability threshold strictly between 0 and 1 | `refusal-probability-*` |
| FR-520 analyze item naming | `model-unknown-claim` |

Not covered here: FR-094 progress ledger, FR-160 uncertain time, FR-415 to FR-417 timed models, FR-361 to FR-369 counterexample structure, fairness (the lock system covers it) and the probabilistic engines (the architecture system covers them).
