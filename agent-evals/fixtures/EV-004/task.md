Check a meter's recorded samples with public fsl monitor. The fixed record type
permits levels 0 through 4. Keep all supplied profile/record declarations,
requests and trace bytes unchanged.

Append exactly two temporal declarations to monitor.fsl. Use binder m of type
MeterReading, clock "ticks", activation on origin, and these names and forms:

- WindowBelowThree using win: always[0,2] holds(m.level <= 2).
- ForeverBelowThree using inf: always holds(m.level <= 2).

Whitespace and declaration order are your choice. Add no other declarations,
comments, assumptions or alternative properties to the appended portion. Use
the shipped docs under docs/ for the surrounding declaration and CLI syntax.

Run these three public monitor checks and save each complete JSON-lines standard
output under the indicated name:

- window-request.json with --input holding.jsonl -> holding-result.jsonl
- window-request.json with --input violating.jsonl -> violating-result.jsonl
- open-request.json with --input open.jsonl -> open-result.jsonl

Use --format json. Retain a violation's output even when its command exits
nonzero. The first two traces have authoritative progress/completeness through
position 2 and a closed scope/execution. The last is a finite prefix without an
unbounded closure certificate. Distinguish a bounded window holding from a
system-wide or unbounded proof.

Read the summary, settlement and counterexample. Write result.json with exactly
these members and the values obtained from the commands:

{
  "holding": {
    "clause": "WindowBelowThree", "kind": "...", "exit_code": 0,
    "settled": true, "evaluation_position": 0, "observed_position": 0
  },
  "violating": {
    "clause": "WindowBelowThree", "kind": "...", "exit_code": 10,
    "settled": true, "evaluation_position": 0, "observed_position": 0,
    "bad_sample": 0, "bad_level": 0
  },
  "open": {
    "clause": "ForeverBelowThree", "kind": "...", "exit_code": 0,
    "settled": false, "unbounded_proof": false
  }
}

The evaluation_position is the origin at which the rule is evaluated, reported by
settlement.position. The observed_position is the event's position when it
settles. bad_sample is the zero-based observation whose level makes the
predicate false; bad_level is that supplied level. These positions describe
different parts of the trace, so do not substitute one for another.

Only when the files and checks are complete, make your final action:

echo '<<<EVAL-COMPLETE>>>'
