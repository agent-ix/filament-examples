---
type: reference
---

# Claims cases

Claims is a corpus for the claim and temporal forms of FSL. Each case submits a rule, a trace or a model and compares the outcome with a recorded result.

Run the cases from the repository root:

```sh
cargo run --quiet --locked --manifest-path tools/fsl-examples/Cargo.toml -- check claims
```

Set `FSL` to the driver binary.

## Cases

- `future-*` read the future operators (`eventually`, `always`, `until`, `release`, nested forms, bounded and unbounded intervals) on an infinite trace written as a prefix and a loop. Each case names the position where `p` and `q` hold, so the expected truth is stated in the caption. Interval endpoints are tested on both sides, including a zero-width interval and a lower bound with no upper bound.
- `future-activation-*` check a clause that activates on each position where a guard holds: every activation must hold, and a trace with no trigger is inactive and has no truth value.
- `past-*` read `once`, `historically`, `since` and `triggered` at position 3 of a trace, including a look-back that reaches before the first position.
- `window-*` read a clause under the timestamped finite-window profile: inclusive endpoints, a stamp just outside the interval, empty windows, completeness, pending and indeterminate results, tied stamps, listing order and the lower boundary of a look-back.
- `false-extension-*` read a clause over a closed finite stream under a false-extension profile, where a missing position reads every atom as false and `not holds(p)` differs from `holds(not p)`.
- `model-*` analyze invariants, temporal claims and state-graph claims (`possible`, `always possible`, `unique path`) over a dial that counts 0 to 3. `rules/dial.fsl` has the full dial, `rules/dial-acyclic.fsl` counts up only, and `rules/dial-branch.fsl` adds a step of two.
- `refusal-*` compile one claim declaration and record the refusal the standard names, or the successful compile for the boundary beside it: clocks over a model and over a trace, profiles, intervals, Boolean bodies, fairness and probability thresholds.

## Staged

`model-possible-trap-cyclic` asks whether the dial can reach n above 3. No state has it, so the claim is refuted. Because the dial can return to 0 the state graph has a cycle, and analyze does not settle the claim: the request's `deadline_ms` of 2000 stops it with `cancelled`/`deadline`, exit 22. FR-391-AC-2 requires `refuted`, closed-scope, exit 10. The same claim over the acyclic dial refutes in a fraction of a second (`model-possible-trap`).

## Coverage

| Criterion | Cases |
| --- | --- |
| FR-091 bounded future operators, inclusive endpoints, zero width, lower bounds, nesting (AC-1, AC-3) | `future-ev-0to2-*`, `future-ev-at1-*`, `future-al-0to2-*`, `future-until-1to2-*`, `future-release-1to2-*`, `future-ev-from2-*`, `future-al-from1-*` |
| FR-091-AC-2, AC-7 false extension and pointwise negation | `false-extension-always-one-position`, `false-extension-always-two-positions`, `false-extension-eventually-not-p-one-position`, `false-extension-always-not-p-one-position`, `false-extension-holds-not-p-*` |
| FR-091-AC-3 until lower-bound convention | `future-until-1to2-p-off-at-anchor`, `false-extension-until-1to2-p-off-at-anchor` |
| FR-091-AC-6 unbounded interval under a bounded profile | `refusal-unbounded-under-bounded`, `refusal-lower-only-under-bounded` |
| FR-091-AC-10, AC-11 window truth tables and empty windows | `window-always-*`, `window-eventually-*`, `window-stamp-*`, `window-single-*`, `window-empty-*` |
| FR-091-AC-12 pending and decisive early settlement | `window-pending-before-deadline`, `window-settled-at-deadline`, `window-open-scope-*` |
| FR-091-AC-14, AC-15 completeness and the inclusive deadline | `window-incomplete-*`, `window-complete-*`, `window-deadline-instant-participates`, `window-after-deadline-ignored` |
| FR-091-AC-16 unkeyed ties refuse order-dependent operators | `window-unkeyed-ties-until`, `window-conflicting-same-stamp`, `window-agreeing-same-stamp` |
| FR-091-AC-17 look-back boundary | `window-once-*` |
| FR-091-AC-22, AC-23 listing order and an empty trace | `window-listing-order-ignored`, `window-no-observations` |
| FR-092 bounded past operators; FR-161-AC-12 non-strict since and triggered | `past-*` |
| FR-161, FR-367 infinite-trace operators, loops | `future-recur-*`, `future-persist-*`, `future-ev-loop`, `future-al-loop-gap`, `future-activation-in-loop` |
| FR-093 activation (AC-5) | `future-activation-*` |
| FR-090-AC-2, AC-11, AC-12 profile and clock selection | `refusal-no-clock`, `refusal-*clock*`, `refusal-unknown-profile` |
| FR-019-AC-6 invariant anchor and Boolean body | `model-invariant-*`, `refusal-invariant-not-boolean` |
| FR-390, FR-391 state-graph claims (AC-1 to AC-4) | `model-possible-*`, `model-always-possible-*`, `model-unique-path-*` |
| FR-391-AC-2 trap over a cyclic graph | `model-possible-trap-cyclic` (staged) |
| FR-407 probability threshold strictly between 0 and 1 | `refusal-probability-*` |
| FR-520 analyze item naming | `model-unknown-claim` |

Not covered here: the absent-valuation cases of FR-091-AC-13 (a sample position cannot omit a field), the counter-cover cases of FR-091-AC-18, AC-20 and AC-25 (needs a cover configuration), the order-key cases of FR-091-AC-16 beyond the unkeyed tie, the progress-ledger criteria of FR-094, FR-160 uncertain time, FR-415 to FR-417 timed models, fairness (the lock system covers it), FR-361 to FR-369 counterexample structure and the probabilistic engines, which the architecture system covers.
