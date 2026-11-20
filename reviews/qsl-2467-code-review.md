---
id: SR-020
title: "QSL-2467 examples code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-examples; PR 56 at 6ffe7f7c03cba791ce54b4459448fcafa61fa853; Rover timed rules, requests, snapshots, expected outputs, manifest, README and index"
review_set: subset
---

## Summary

Ticket QSL-2467, examples PR #56: reviewed the three Rover timed-analysis cases and their requests, snapshots, expected outputs, catalog and count updates against the ticket ruling and FR-239/FR-632. The recorded outcomes route to EN-6 as required, but the new rule file duplicates the existing Rover operation model and two case descriptions promise stronger persistence than their formulas state.

## Verdict

**FAIL** — the diff adds a second copy of the Rover mode-machine operation contracts (high P0 duplication) and has a natural-language/formula mismatch in the two hold cases.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new timed rule file duplicates the existing Rover operation contracts and terminal predicate, creating two sources that can drift. | examples/rover/rules/timed-analyze.fsl:12-48 |
| FND-002 | medium | The two hold-case statements say the rover remains stopped, while their formulas require only eventual stop or eventual recurrence after each stopped state. | examples/rover/example.toml:358,384 |

## Coverage

Ticket acceptance and the current FSL FR-239/FR-632 text were read. The three case entries are running timed analyze cases on `model-time`; the expected hold outputs record EN-6 `proved` at S6c, the refutation records EN-6 `refuted` at S8, and the liveness output includes FR-632's `non_zeno` member. The coder's focused record/check evidence was accepted: three cases recorded, then check 3/3 (exit 0, 10, 0); no tests were rerun here.

No Rust, spec, or plan files changed. No applicable AssuranceProfile or repository-local `spec/` tree exists. The optional semantic fan-out was not requested. The Quoin matrix limitation and manual ticket-scope check are detailed in the separate gap-analysis artifact. No aggregate gate was run.

## Examined units

```yaml
clean: false
scope:
  - {id: FR-239, path: "filament-spec-language/spec/functional/FR-239-check-a-timed-claim-by-symbolic-zone-search.md", role: context_only, excerpt: "EN-6 registers a provider manifest advertising (temporal-satisfaction, bounded) and (temporal-satisfaction, unbounded) for timed subjects."}
  - {id: FR-632, path: "filament-spec-language/spec/functional/FR-632-request-timed-zone-items.md", role: examined, excerpt: "A native-analyze/1 item over a timed subject SHALL be decided by the zone engine."}
  - {id: README.md, path: "README.md", role: examined, excerpt: "The rover row increases from 62 to 65 running cases."}
  - {id: timed-lidar-lost-start, path: "examples/rover/data/timed-lidar-lost-start.json", role: examined, excerpt: "A complete Rover snapshot starts in Degraded mode with lidarOk false and stopped false."}
  - {id: timed-stopped-start, path: "examples/rover/data/timed-stopped-start.json", role: examined, excerpt: "A complete Rover snapshot starts in SafeStop mode with stopped true."}
  - {id: timed-analyze.fsl, path: "examples/rover/rules/timed-analyze.fsl", role: examined, excerpt: "The file declares the timed claims and repeats the existing LidarLoss, Brake, Halt, Failsafe, Sample and Parked model rules."}
  - {id: timed-zone-deadline-holds-request, path: "examples/rover/requests/analyze/timed-zone-deadline-holds.json", role: examined, excerpt: "The native-analyze/1 request selects StopsWithin200ms over the timed-analyze program and the stopped-start snapshot."}
  - {id: timed-zone-deadline-refuted-request, path: "examples/rover/requests/analyze/timed-zone-deadline-refuted.json", role: examined, excerpt: "The native-analyze/1 request selects StopsWithin50ms over the timed-analyze program and the lidar-lost-start snapshot."}
  - {id: timed-liveness-holds-request, path: "examples/rover/requests/analyze/timed-liveness-holds.json", role: examined, excerpt: "The native-analyze/1 request selects StoppedRoverRemainsStoppedInModelTime over the stopped-start snapshot."}
  - {id: timed-zone-deadline-holds-output, path: "examples/rover/expected/timed-zone-deadline-holds.json", role: examined, excerpt: "The item is proved by EN-6 with basis zone-certified and the document ends at S6c."}
  - {id: timed-zone-deadline-refuted-output, path: "examples/rover/expected/timed-zone-deadline-refuted.json", role: examined, excerpt: "The item is refuted by EN-6 and the document ends at S8."}
  - {id: timed-liveness-holds-output, path: "examples/rover/expected/timed-liveness-holds.json", role: examined, excerpt: "The item is proved by EN-6 with basis zone-certified at S6c and includes non_zeno: not-required."}
  - {id: rover-example-manifest, path: "examples/rover/example.toml", role: examined, excerpt: "The three cases are marked running, select engine timed and clock model-time, and expect exits 0, 10 and 0."}
  - {id: index, path: "index.json", role: examined, excerpt: "The three new Rover case entries match their request, expected-output, engine and verdict metadata."}
findings:
  - fnd: FND-001
    severity: high
    confidence: high
    method: code-review
    check: other
    artifact_id: examples/rover/rules/timed-analyze.fsl
    related_ids: [examples/rover/rules/model-checking/mode-machine.fsl]
    path: examples/rover/rules/timed-analyze.fsl
    lines: "12-48"
    excerpt: |-
      pre LidarLossWhileDriving using v on Rover::Rover::loseLidar { self.mode = Rover::Mode::Driving and self.lidarOk }
      post LidarLossDegrades using v on Rover::Rover::loseLidar { self.mode = Rover::Mode::Degraded and not self.lidarOk and self.speed <= 300 and self.speed <= pre(self.speed) }
      pre BrakeWhenDegraded using v on Rover::Rover::brake { self.mode = Rover::Mode::Degraded }
      post BrakeEntersSafeStop using v on Rover::Rover::brake { self.mode = Rover::Mode::SafeStop and self.speed <= 100 and self.speed <= pre(self.speed) }
      pre HaltInSafeStop using v on Rover::Rover::halt { self.mode = Rover::Mode::SafeStop and not self.stopped }
      post HaltStops using v on Rover::Rover::halt { self.speed = 0 and self.stopped }
      pre FailsafeAfterHalt using v on Rover::Rover::enterFailsafe { self.mode = Rover::Mode::SafeStop and self.stopped }
      post FailsafeEntered using v on Rover::Rover::enterFailsafe { self.mode = Rover::Mode::Failsafe }
      pre SampleUntilFailsafe using v on Rover::Rover::sample { self.mode != Rover::Mode::Failsafe }
      post SampleRecordsVote using v on Rover::Rover::sample { self.voteWrong = ((faultA and faultB) or (faultA and faultC) or (faultB and faultC)) }
      terminal Parked using v on Rover when { self.mode = Rover::Mode::Failsafe };
    finding: "The new timed rule file repeats the existing Rover mode-machine operation contracts and terminal predicate from examples/rover/rules/model-checking/mode-machine.fsl. The identical transition rules now have two owners and will drift. Reuse or extend the existing model rule file while adding the timed claims."
  - fnd: FND-002
    severity: medium
    confidence: high
    method: code-review
    check: test-intent
    artifact_id: examples/rover/example.toml
    related_ids: [examples/rover/rules/timed-analyze.fsl]
    path: examples/rover/example.toml
    lines: "355-386"
    excerpt: |-
      statement = "A stopped rover remains stopped within the 200 ms model-time deadline."
      statement = "A stopped rover remains stopped in model time."
    finding: "Both hold-case descriptions promise that the rover remains stopped. StopsWithin200ms is only eventually[0 ms, 200 ms] holds(r.stopped), and StoppedRoverRemainsStoppedInModelTime is only always (holds(r.stopped) implies eventually holds(r.stopped)); neither formula asserts continuous persistence. State what these formulas prove or strengthen the predicates to match the descriptions."
```

## Dispositions

Round 1 (posted to QSL-2467 by the earlier disposition pass at head ea39bc4d70aeecb2243111cbc06477622e7962eb).

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | timed-analyze.fsl is gone; the timed claims and the time declaration now live in examples/rover/rules/model-checking/mode-machine.fsl:4-11,78-88 next to the one copy of the operation contracts. |
| FND-002 | fixed | example.toml statements now say "is stopped at some point within the next 200 ms" and "Whenever the rover is stopped, it is stopped at some point", which is what the formulas state. |
| FND-003 | fixed | Round 3, head c654a4e (PR #66): `git diff 7d2c190^ HEAD -- examples/rover/rules/model-checking/mode-machine.fsl` is empty; the time declaration and timed claim moved to mode-machine-timed.fsl with a QSL-2654 merge-back comment, its operation contracts byte-identical to mode-machine.fsl. |
| FND-004 | fixed | Round 3: the tautology is replaced by LidarLossEventuallyStopsInModelTime, `always (holds(not r.lidarOk) implies eventually holds(r.stopped))` under weak fairness of brake and halt, from a Degraded lidar-lost start. Without the fairness the rover can loop on `sample` forever in Degraded, so an engine that dropped or mis-applied the fair weak assumptions (or the brake/halt contracts) would report violated. |
| FND-005 | fixed | Round 3: new clocked model example/rover-deadline (generated by `make models`; regenerated byte-identical). From the same Degraded start, StopsWithin200ms holds and StopsWithin150ms is refuted (counterexample brake at x=75 ms, halt 87.5 ms later = 162.5 ms, both within the [50, 100] ms guard/invariant windows); the two claims differ only in the bound, so an engine that ignored the interval would fail the refuted case. |
| FND-006 | fixed | Round 3: the claim StoppedRoverRemainsStoppedInModelTime is removed; the new name LidarLossEventuallyStopsInModelTime says what the formula states. |
| FND-007 | rejected | Round 3: the profile is required. Removing `profile tc` from stop-deadline.fsl makes `fsl analyze` exit 21 with "`Rover` is a timed declaration, which needs the `filament.temporal.complete/v1` layer"; mode-machine-timed.fsl now says so in a comment. |

## New findings (disposition pass 2)

Round 2, at head ea39bc4d70aeecb2243111cbc06477622e7962eb (merged as 7d2c190 while this round ran).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | high | Adding the `time Rover tick ... period 50 ms` declaration to the shared mode-machine.fsl makes the existing running case rover/mode-machine (StopsAfterLidarLoss, model-steps, en-1) at least 5x slower: 225 s before the PR, not finished after 32 min CPU at the PR head; the coder ran only the three new cases. | examples/rover/rules/model-checking/mode-machine.fsl:4-11 |
| FND-004 | medium | timed-liveness-holds checks `always (holds(r.stopped) implies eventually holds(r.stopped))`, a tautology (p implies F p holds at the current position on every trace), so the case passes for any model and any timed semantics; it can only fail on routing, refusal or record-shape bugs. | examples/rover/rules/model-checking/mode-machine.fsl:86-88 |
| FND-005 | medium | timed-zone-deadline-holds starts in an already-stopped state, so `eventually[0 ms, 200 ms]` is met at time 0; with the refuted case (which a loop of `sample` would also refute for any bound) no case depends on the deadline value, so an engine that ignored the interval would pass all three. | examples/rover/rules/model-checking/mode-machine.fsl:78-84 |
| FND-006 | low | The claim name StoppedRoverRemainsStoppedInModelTime still promises persistence that the formula does not state (FND-002 fixed the statement text only). | examples/rover/rules/model-checking/mode-machine.fsl:86 |
| FND-007 | low | `profile tc = "filament.temporal.complete/v1"` is declared but no clause uses it. | examples/rover/rules/model-checking/mode-machine.fsl:4 |
