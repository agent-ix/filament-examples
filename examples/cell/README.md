# Work cell

The work cell is an example of a manufacturing cell with a hydraulic press behind a guard and a robot arm that shares the floor with people. The press may move only with the guard closed and the emergency stop released, and it runs slowly in setup mode. The robot keeps a protective distance that grows with its speed.

You can check each interlock as a rule over a snapshot of the press or the robot. Every rule has a passing and a failing case.

## The model

- `Press` has a guard state, a ram state, a mode (stopped, setup or run), a ram speed in mm/s and an emergency stop. Its operations are `start` and `openGuard`.
- `Robot` has a speed in mm/s, a separation from the nearest person in mm, and a flag for a person in the cell.

The model is in [model/cell.semantic-ir.json](model/cell.semantic-ir.json).

## Rules

The table lists each rule with its requirement and a passing and a failing case.

| Rule | Requirement | Passing | Failing |
| --- | --- | --- | --- |
| `GuardClosedWhileMoving` | The ram moves only while the guard is closed | `guard-closed-running` | `guard-open-running` |
| `EstopHolds` | With the emergency stop pressed, the ram does not move | `estop-holds` | `estop-moving` |
| `SetupIsSlow` | In setup mode the ram moves at 10 mm/s or less | `setup-slow` | `setup-fast` |
| `StartNeedsGuard` | The press starts only with the guard closed | `start-guard-closed` | `start-guard-open` |
| `OpeningGuardStops` | Opening the guard leaves the ram stopped | `open-guard-stops` | `open-guard-keeps-moving` |
| `SpeedAndSeparation` | A person in the cell is kept at a safe distance | `separation-ok` | `separation-too-close` |

The first three rules are in [rules/press.fsl](rules/press.fsl) as invariants. `StartNeedsGuard` and `OpeningGuardStops` in the same file are a precondition and a postcondition of two operations. `SpeedAndSeparation` is in [rules/robot.fsl](rules/robot.fsl).

The case `interlock-backwards` checks the interlock written the wrong way round, in [rules/interlock-backwards.fsl](rules/interlock-backwards.fsl): "a closed guard means the ram moves". It holds on the state where the ram moves with the guard open.

The separation rule requires the distance to be at least 500 ms of travel plus 200 mm. The source works in thousandths of a millimeter because integer division is not available in FSL source yet.

## Run a passing and a failing case

From this folder, with `FSL` set to the driver:

```
$FSL run requests/guard-closed-running.json --format json
```

The recorded result is:

<!-- output: guard-closed-running -->
```
exit 0
{"category":"success","clause":"GuardClosedWhileMoving","result":"tested"}
```
<!-- /output -->

In `guard-open-running` the ram moves with the guard open:

```
$FSL run requests/guard-open-running.json --format json
```

The recorded result is:

<!-- output: guard-open-running -->
```
exit 10
{"category":"violation","clause":"GuardClosedWhileMoving","result":"refuted"}
```
<!-- /output -->

All twelve cases run today, and none is staged.
