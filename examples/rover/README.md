# Rover

The rover is an example of a delivery vehicle that drives on a lidar and three range sensors whose readings are voted two out of three. Without its lidar it slows down and must stop. In failsafe mode it stays still. Its control loop is a periodic task set.

You can check the mode rules as invariants over a snapshot, call the voting function directly, and check the safe-stop rule over a trace of control steps. A violation of the trace rule comes back with a counterexample.

## The model

- `Rover` has a mode (driving, degraded, safe stop or failsafe), a speed and a stopped flag. It also has a lidar flag, three range readings, a clock in ms and a flag for a wrong vote.
- Its operations give the mode a shape. `loseLidar` takes a driving rover to degraded mode and slows it. `brake` takes a degraded rover to safe-stop mode. `halt` stops a rover in safe-stop mode. `enterFailsafe` takes a stopped rover to failsafe mode, where it stays. `sample` reads the three sensors in every mode but failsafe. Statements about the mode machine start from a driving rover with a working lidar that has not stopped.
- A second model, `rover-spec`, declares a `Vehicle` that can be stopped. It is used by the refinement example.

The models are in [model](model). The preconditions and postconditions of the operations are in [rules/model-checking/mode-machine.fsl](rules/model-checking/mode-machine.fsl).

## Rules

The rules in [rules/modes.fsl](rules/modes.fsl) check one snapshot of the rover. The rule `SafeStopWithin3Steps` is in [rules/safe-stop-steps.fsl](rules/safe-stop-steps.fsl) and checks a trace. The table lists each rule with its requirement and a passing and a failing case.

| Rule | Requirement | Passing | Failing |
| --- | --- | --- | --- |
| `FailsafeIsStill` | In failsafe mode the rover is stopped | `failsafe-still` | `failsafe-rolling` |
| `DegradedIsSlow` | Without lidar the speed is 300 mm/s or less | `degraded-slow` | `degraded-fast` |
| `ObstacleSlows` | With two of three sensors near an obstacle, crawl | `obstacle-crawl` | `obstacle-fast` |
| `SafeStopWithin3Steps` | After losing lidar, stopped within three steps | `safe-stop-in-time` | `safe-stop-late` |

The case `obstacle-outvoted` has one sensor reading near and two reading far. The vote outvotes the one sensor, and the rover may drive at normal speed.

## The voting function

The function `twoOfThree` returns true when at least two of its three arguments are true. The case `vote-two-of-three` calls it with the values 1, 1 and 0 and gets true. The case `vote-one-of-three` calls it with 1, 0 and 0 and gets false. A call passes a Boolean as 1 for true or 0 for false.

## The control loop

[rules/control-loop.fsl](rules/control-loop.fsl) declares sensor fusion every 10 ms, the stop monitor every 20 ms and path planning every 50 ms. The case `control-loop-compiles` checks the declaration and exits 0.

## Run a passing and a failing case

From this folder, with `FSL` set to the driver, check the rule over a trace that stops in time:

```
$FSL run requests/safe-stop-in-time.json --format json
```

The recorded result is:

<!-- output: safe-stop-in-time -->
```
exit 0
{"axes":{"activation":"active","execution":"completed","settlement":"closed-scope","truth":"true"},"category":"success","clause":"SafeStopWithin3Steps","result":"tested"}
```
<!-- /output -->

In `safe-stop-late` the lidar is lost at the second step and the rover stops four steps later:

```
$FSL run requests/safe-stop-late.json --format json
```

The recorded result is below. The full counterexample, with the trace prefix, the loop and the decisive position, is in the recorded file.

<!-- output: safe-stop-late -->
```
exit 10
{"axes":{"activation":"active","execution":"completed","settlement":"closed-scope","truth":"false"},"category":"violation","clause":"SafeStopWithin3Steps","position":0,"result":"refuted","counterexample":"recorded in expected/safe-stop-late.json"}
```
<!-- /output -->

## More rules over the trace

These cases back the time pages of the FSL docs.

- `recording-cut-short`, `recording-in-time` and `recording-late` check `SafeStopWithin3Steps` over recordings with no loop. The first two are pending: the recording ends before the rule is decided. The third is violated, because positions 1 to 4 are recorded and none is stopped.
- `safe-stop-late-without-always` checks the rule written without `always`, in [rules/safe-stop-without-always.fsl](rules/safe-stop-without-always.fsl). It is read only at the first state, where the lidar works, so it holds even on the late run.
- [rules/time-operators.fsl](rules/time-operators.fsl) holds one rule per temporal operator and interval form, each checked over the in-time trace (`op-*`), and `StopAfterLoss`, read at every position where the lidar is lost (`stop-after-loss-in-time`, `stop-after-loss-late`).
- `never-stops` checks `EventuallyStopped` over a lasso whose loop is a driving state. The rover never stops, so the rule is violated, and the counterexample is the lasso.
- `in-time-small-budget` gives the check a work budget of 5 units and stops incomplete.
- `keeps-sampling` has a fairness constraint. A recorded trace does not say when `sample` was enabled, so the check is unsupported.
- `timed-safe-stop-compiles` checks a timed rule over model time, in [rules/timed-safe-stop.fsl](rules/timed-safe-stop.fsl).

## The mode machine

[rules/model-checking/mode-machine.fsl](rules/model-checking/mode-machine.fsl) holds the rules that give the operations their meaning, and the rules the model-checking pages show.

- `model-subjects-mode-machine` compiles the whole file.
- `model-subjects-lidar-loss-degrades` and `model-subjects-brake-from-degraded` check a postcondition on one call of an operation. `model-subjects-brake-while-driving` checks a precondition and is violated, because a driving rover cannot brake.
- `model-subjects-parked` and `model-subjects-terminal-any` compile a `terminal` declaration.
- `fairness-stops-after-lidar-loss` compiles a rule with two fairness constraints.
- `state-graph-queries-possible`, `state-graph-queries-always-possible` and `state-graph-queries-unique-path` compile the three state-graph queries.
- `reductions-constraint` compiles a state constraint.
- `hyperproperties-vote-ignores-lidar` and `hyperproperties-sample-is-deterministic` compile a hyperproperty and a step relation.
- `schedulability-control-loop-mixed` compiles a task set with a high-criticality task and three claims.
- `hybrid-systems-braking` and `hybrid-systems-claim` compile a hybrid model and a claim over it.
- `refinement-controller` compiles the refinement of the rover controller. Halting is the abstract stop.

## Staged

These cases are staged until the language supports them.

- `staged-safe-stop-200ms`: Stopped within 200 ms. Needs clock selection.
- `staged-stops-in-four-events`: Stopped within four recorded events. Needs clock selection.
- `staged-stops-in-four-samples`: Stopped within four fixed-rate samples. Needs clock selection.
- `staged-control-loop-schedulable`: The task set fits and the monitor responds in 10 ms. Needs schedulability analysis in the `analyze` command.
- `staged-vote-probability` and `staged-vote-probability-missed`: The vote is rarely wrong, and a stricter bound that the vote misses. Needs packaging of random parameters.

The step rule also runs on a live feed: `staged-safe-stop-monitor` watches a feed in which the rover stops in time and stays pending, since a rule that must hold always is never settled by a finite feed, and `safe-stop-monitor-late` watches a feed in which the rover is still driving three steps after the lidar was lost, a violation.

`example.toml` records the exact reason and ticket for each.
