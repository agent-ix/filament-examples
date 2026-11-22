# Pump and valve plant

The plant is an example of a process plant with a pump behind a discharge valve and two storage tanks. The pump moves liquid from tank T-101 to tank T-102 through valve V-2. A pump that runs against a closed valve deadheads: it heats the liquid and wears its seals. A tank that fills past its high-high alarm level overflows.

You can check each rule over a snapshot of the plant. Every rule has a passing and a failing case.

## The model

- `Valve` has a flag `open`.
- `Pump` has a flag `running` and a reference `discharge` to its valve.
- `Tank` has a `level`, an integer percent from 0 to 100.

The model is in [model/plant.semantic-ir.json](model/plant.semantic-ir.json). Each snapshot holds the valve `V-2`, the pump `P-101` and the tanks `T-101` and `T-102`.

## Rules

The table lists each rule with its requirement and the cases that check it.

| Rule | Requirement | Passing | Failing |
| --- | --- | --- | --- |
| `NoDeadhead` | The pump runs only while its discharge valve is open | `pump-open-valve`, `pump-off-valve-closed` | `pump-closed-valve` |
| `TankBelowHighHigh` | A tank stays below its high-high alarm level of 90 percent | `level-below-limit` | `level-at-limit`, `level-above-limit` |

`NoDeadhead` is in [rules/pump.fsl](rules/pump.fsl) and `TankBelowHighHigh` is in [rules/tank.fsl](rules/tank.fsl). Both are invariants, so a request names the one pump or tank it checks.

The case `pump-off-valve-closed` holds because a stopped pump may sit behind a closed valve. The case `level-at-limit` is violated because a level of exactly 90 is not below 90.

## Run a passing and a failing case

From this folder, with `FSL` set to the driver:

```
$FSL run requests/pump-open-valve.json --format json
```

The recorded result is:

<!-- output: pump-open-valve -->
```
exit 0
{"axes":{"activation":"active","execution":"completed","settlement":"closed-scope","truth":"true"},"category":"success","clause":"NoDeadhead","result":"tested"}
```
<!-- /output -->

In `pump-closed-valve` the pump runs and the valve is closed:

```
$FSL run requests/pump-closed-valve.json --format json
```

The recorded result is:

<!-- output: pump-closed-valve -->
```
exit 10
{"axes":{"activation":"active","execution":"completed","settlement":"closed-scope","truth":"false"},"category":"violation","clause":"NoDeadhead","result":"refuted"}
```
<!-- /output -->

## Check every behavior

The invariants above check one snapshot. The model [model/plant-line.semantic-ir.json](model/plant-line.semantic-ir.json) describes a line with one valve and one pump as four operations, so that the model checker can explore every order in which they run, starting from [data/line-idle.json](data/line-idle.json). The rules in [rules/model-checking](rules/model-checking) differ in one precondition: whether the valve may close while the pump runs.

| Rule | Requirement | Passing | Failing |
| --- | --- | --- | --- |
| `NeverDeadheads` | On every behavior, the pump never runs against a closed valve | `close-guarded` | `close-unguarded` |

```
$FSL analyze requests/analyze/close-guarded.json --format json
```

All eight cases run today, and none is staged.
