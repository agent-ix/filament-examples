---
id: SR-004
title: "Code review of filament-examples PR 33 lock analyze cases (QSL-1976)"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-examples; examples/lock/**, examples/parts-saga/example.toml, examples/rover/example.toml, examples/shop/example.toml, examples/*/expected/**, examples/*/README.md, README.md, index.json at f72dd9ba161a1ce1fc4dba7db4f6a2d6a2ef76c8"
review_set: subset
evaluated_revision: "f72dd9ba161a1ce1fc4dba7db4f6a2d6a2ef76c8"
---

## Summary

Ticket: QSL-1976. Review of `git diff origin/main...HEAD`. This is one review across this PR and agent-ix/filament-driver#159. The driver half is in the driver's `reviews/qsl-1976-*.md`. No tooling changed. The PR adds data, requests, expected records and README regions. There is no Python and no shell.

Ruling 3 (re-recorded running records). All 103 modified `expected/*.json` files were compared with jq, old against new with every `axes` member deleted recursively. 97 are identical apart from `axes`. Their numstat shows additions only, and main had no `axes` member in any of them. The other 6 are the staged analyze cases. Ten were spot-checked by eye: architecture data-tier-calls-core and no-call-cycle-partial; cell estop-holds and setup-slow; rover in-time-small-budget, op-eventually-from-2 and recording-in-time; shop charge-frame-ok, limits-source-bytes-raised and profiles-count-under-queries. Each adds one FR-622 `axes` object per monitor item, with `activation`, `execution`, `settlement` and `truth`. Each `settlement` is consistent with its category: `closed-scope` for success, `decisive-counterexample` for violation, `unsettled` for pending, and `unavailable` for incomplete.

Ruling 4 (staged cases). On main, nine toml mentions of QSL-1976 belonged to seven staged cases plus two `why` strings. Every case is still present. `staged-one-served-strong` is replaced by the running `analyze-served-strong-each` and `analyze-served-weak-each`. Six cases are re-pointed, and each blocker exists in Linear with matching scope. QSL-2122 covers invariants, deadlock and time-lock. QSL-2127 covers refinement and names S-15 and S-19. QSL-2131 covers schedulability and names S-11. QSL-2224 covers protocol model checking and names S-8. QSL-1832 is the model-checker baseline in project "QSL model-checker scaling". No case was dropped. However, in five of the six re-pointed cases, the recorded refusal is a request-authoring error, not the named blocker (FND-001, FND-002).

## Verdict

REQUEST CHANGES. The two running lock cases are correct and well described. The weak-fairness counterexample is replayed and reproduced, and the strong-fairness item is proved. The axes-only re-recording is verified. Two medium defects make staged cases record the wrong cause. Those cases would stay at exit 20 after their blocker lands, so the staged suite would never report them ready.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `staged-two-locks-deadlock` names `Pair` as its claim. `Pair` is the model alias (`model Pair = "example/two-locks"`), not a checked claim, so the recorded exit 20 is `unknown-claim` for a request mistake, not the QSL-2122 gap. FSL FR-620 derives a model subject's deadlock-freedom item (FR-368) when a request has no `items`. The driver's `analyze_unsupported_item_is_typed` shows that derived item settling `unsupported`/`unsupported-requested-capability`, exit 21. Fix: drop `items` (or name the derived item as FR-620 allows), re-record, and set `expect_exit` to the real refusal (21 if it matches the driver test). Keep `ready_exit = 10` and QSL-2122. | examples/lock/requests/staged-two-locks-deadlock.json:26; examples/lock/example.toml:117-127 |
| FND-002 | medium | Five staged analyze cases still point `fsl analyze` at a `native-compile/1` request. Their recorded exit 20 is `format: native-compile/1 is not read here`, which no engine ticket changes. When QSL-2127, QSL-2131, QSL-2224 or QSL-1832 lands, these cases still exit 20, and their `ready_exit = 0` is never reached. Fix: add a `native-analyze/1` request under `requests/analyze/` for each case, as the lock did. Do not edit the compile requests, which the compile cases at rover/example.toml:645 and shop/example.toml:427 share. Re-record each case at the refusal the real blocker gives. Where no item spelling yet reaches the engine, record that per-item refusal and say so in `why`. | examples/shop/example.toml:387-398; examples/rover/example.toml:381-392; examples/rover/example.toml:409-420; examples/rover/example.toml:437-448; examples/parts-saga/example.toml:84-95 |
| FND-003 | low | `rover/staged-mode-machine` waits on QSL-1832, a profiling and baseline ticket that produces measurements, not a faster search. Closing it does not make the case finish in time. Fix: point `waits_on` at the scaling ticket in that project that removes the time-out, or at a new ticket for this model's search, so the case is re-checked when it can pass. | examples/rover/example.toml:417 |
| FND-004 | low | Two `why` strings still say the claim "would further need `quire analyze` (QSL-1976)". After this PR the verb is `fsl analyze` and QSL-1976 is the ticket that routes it. Fix: say `fsl analyze` and drop the QSL-1976 reference (or name the engine ticket the claim needs). | examples/architecture/example.toml:244; examples/rover/example.toml:432 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | The new mode-machine request caps the item at `temporal.limits.max_states` 200, so the case records `incomplete`, exit 22. When QSL-2287 lands, the same request still stops at 200 states and still exits 22, so `ready_exit = 0` is not reached until the request changes too. Fix: say in `why` that un-staging removes the 200-state cap (or use the published default limit once the search fits it), so that whoever closes QSL-2287 knows the request is part of the change. | examples/rover/requests/analyze/mode-machine.json:35; examples/rover/example.toml:415 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At 9ae461d, `items` is dropped and the case is re-recorded at what FSL does: `success`, empty `items`, exit 0. Two-locks has only pre and post clauses, so there is no model subject and FR-368 derives no deadlock-freedom item. The record is honest. The `why` states the vacuous success and its reason, the status stays staged, and `ready_exit = 10` under QSL-2122 means the staged check flags the case as soon as a deadlock item is derived. A zero-item `success` is vacuous, but it is what the tool prints, and the `why` says so. |
| FND-002 | fixed | Each of the five cases now has its own `requests/analyze/*.json` `native-analyze/1` request. The compile requests used by rover:645 and shop:427 are untouched. Each case is re-recorded at the refusal FSL really gives. The refinement claims `ControllerRefinesStop` and `ClientChargesOnce` are `declined` `unknown-claim` (QSL-2127). The schedulability claims `ControlFp` and `StopMonitorFast` are `unknown-claim` (QSL-2131). The `PartFlow` deadlock-freedom item is `unsupported-requested-capability`, exit 21 (QSL-2224). Mode-machine is `incomplete`, `max_states` 200 (QSL-2287). All of these claims are declared in their `.fsl` files, so each refusal is the engine gap and not a request error. |
| FND-003 | fixed | `waits_on = "QSL-2287"`, a lead-filed ticket for the mode-machine search finishing within budget. |
| FND-004 | fixed | Both `why` strings now say "checking the claim then runs through `fsl analyze`". No QSL-1976 reference remains in either toml. |

## Verdict (disposition pass 1)

APPROVE at f18153a. All four round-1 findings are fixed. FND-005 is low and does not block.
