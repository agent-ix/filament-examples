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
