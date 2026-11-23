The Fill protocol coordinates two operations on one Cell. The cell starts at
value 0 and its value is bounded from 0 through 3. Branch left can increment
the value while it is below 3. Branch right can set it to 2 only while the
value is 0. Both branches run before the join.

The supplied protocol currently has no terminal declaration. Analyze it with
the supplied request and save the complete output as `deadlock.json`. Read the
deadlock counterexample and the blocked threads. The prefix `fork(Both)`,
`attempt(A)` leaves the right branch waiting at `B` because `TwoPre` is false,
while the root waits at `join Both`.

This stop is intentional when the cell value is 1. Add exactly this member to
the Fill protocol, without changing the model, operation contracts, run, or
finish:

`terminal when { k.v = 1 };`

Run the same public `fsl analyze` request again and save its complete output as
`repaired.json`. Write `result.json` with exactly these members and the values
from the two commands:

```json
{
  "deadlock": { "result": "refuted", "exit_code": 10,
    "steps": ["fork(Both)", "attempt(A)"],
    "blocked": ["(0, Both, right, 0) at B: Precondition{TwoPre}",
      "(0) at join Both: JoinWaiting{[(0, Both, right, 0)]}"] },
  "repaired": { "result": "proved", "exit_code": 0,
    "basis": "Exhaustive" }
}
```

The recorded baseline is an actual deadlock, not an unsupported or incomplete
analysis. The repaired claim passes because value 1 is an intended terminal
state; do not use `terminal any` or change `TwoPre` to suppress the counterexample.

Only after both analyses and the result file are complete, make your final
action:

`echo '<<<EVAL-COMPLETE>>>'`
