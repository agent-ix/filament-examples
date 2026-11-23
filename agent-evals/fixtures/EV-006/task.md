# Model a protocol, diagnose its deadlock, and repair it

The files describe a finite cell model and its initial state. From the English
behavior below, write `baseline.native` as a complete `Fill` protocol. It has
one worker role and two parallel branches. Branch A increments the cell when
its value is below 3. Branch B sets the cell to 2 only when its value is 0.
Both branches join, then the protocol finishes. Do not declare terminal states
in the baseline.

Run `fsl analyze deadlock-request.json --format json` and save the complete
output as `deadlock.json`. Then repair the protocol in `protocol.native` so
that its intended behavior is deadlock-free. Either declare the reached
intended end state or correct the precondition so it permits the state reached
after A. Keep the model, requests, and initial state unchanged.

Run `fsl analyze repaired-request.json --format json` and save the complete
output as `repaired.json`. Write `result.json` with the observed result and
exit code, the ordered protocol steps, both blocked-thread paths and causes,
and the repaired proof basis. Use this shape, replacing each placeholder with
the value observed in your files:

```json
{
  "deadlock": {
    "result": "<observed result>",
    "exit_code": <observed integer>,
    "steps": ["<step 1>", "<step 2>"],
    "blocked": ["<full blocked path and cause 1>", "<full blocked path and cause 2>"]
  },
  "repaired": {
    "result": "<observed result>",
    "exit_code": <observed integer>,
    "basis": "<observed proof basis>"
  }
}
```
