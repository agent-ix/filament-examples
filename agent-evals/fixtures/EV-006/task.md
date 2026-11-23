# Model and repair a protocol deadlock

The supplied model and initial state describe one finite cell whose integer
value starts at zero. From the behavior below, author a complete `Fill`
protocol in `baseline.native`:

- Branch A increments the cell by one when its current value is below 3.
- Branch B sets the cell's value to 2 when the current value is 0.
- Run both branches in parallel, join them, then finish.
- The baseline has no declared terminal state.

Place any terminal member inside the protocol after the role declarations and
before the run.

Run `fsl analyze deadlock-request.json --format json` and save its complete
output as `deadlock.json`. Repair the protocol in `protocol.native` so the
behavior is deadlock-free, either by declaring an intended terminal state or
by correcting Branch B's precondition so it permits the state reached after
Branch A. Keep the model, requests, and initial state unchanged.

Run `fsl analyze repaired-request.json --format json` and save its complete
output as `repaired.json`. Write `result.json` using exactly this schema:

```json
{
  "deadlock": {
    "exit_code": <integer>,
    "result": "<string>",
    "prefix": [
      { "step": "<string>", "node": "<string>" }
    ],
    "blocked": [
      { "thread": <object>, "rest": <object>, "cause": <object> }
    ]
  },
  "repaired": {
    "exit_code": <integer>,
    "result": "<string>",
    "basis": <object>,
    "certification": "<string>"
  }
}
```

Copy the structured values from the corresponding CLI outcomes. Keep `prefix`
steps in their reported order. The two `blocked` entries may appear in either
order.
