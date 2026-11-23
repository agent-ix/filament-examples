# Model and repair a protocol deadlock

The supplied model and initial state describe one finite cell whose integer
value starts at zero. From the behavior below, author a complete `Fill`
protocol in `baseline.native`:

- Branch A increments the cell by one when its current value is below 3.
- Branch B sets the cell's value to 2 when the current value is 0.
- Run those two operations in parallel, join both branches, then finish.
- The baseline has no declared terminal state.

Run `fsl analyze deadlock-request.json --format json` and save its complete
output as `deadlock.json`. Repair the protocol in `protocol.native` so the
behavior is deadlock-free, either by declaring an intended terminal state or
by correcting the precondition that blocks the second operation after the
increment. Keep the model, requests, and initial state unchanged.

Run `fsl analyze repaired-request.json --format json` and save its complete
output as `repaired.json`. In `result.json`, summarize the CLI's structured
output fields: the exit code and item result; each prefix transition's `step`
and `node`; each blocked entry's `thread`, `rest`, and `cause`; and the repaired
item's `basis` and `certification`. Preserve the JSON values and object shapes
from the CLI output. The prefix is ordered; the blocked entries may be in
whatever order the CLI emits.
