A meter starts at level 0. Its integer domain is 0 through 4. Each advance raises
its level by one while it is below 3, then leaves it unchanged. The supplied
model, starting snapshot, transition contract and analysis requests are the user's
fixed inputs. Do not change them or add any operations, assumptions or fairness.

Append two temporal declarations to counter.fsl, using the existing inf profile,
binder c of type Meter::Counter, clock "model-steps", and on origin:

- WithinLimit: always holds(c.level <= 3).
- TooSmall: always holds(c.level <= 2).

Use exactly these names, binder and predicates. Whitespace and declaration order
are your choice; do not add comments or other declarations to the appended part.
This keeps the requested safety properties distinct from a tautology or a different
claim. The model's domain permits 4, so the first property is about reachability,
not just the scalar type's bound.

Use the shipped documentation under docs/ to find the declaration and command
syntax. Run the public fsl analyze command with holding-request.json and
violating-request.json, --format json, retaining their complete standard output
in holding.json and violating.json. A violation may exit nonzero; retain its
output and inspect the counterexample. Do not count incomplete, budget-limited or
unsupported analysis as a proof.

Write result.json with exactly this shape and the actual values you obtained:

{
  "holding": { "claim": "WithinLimit", "verdict": "...", "exit_code": 0 },
  "violating": {
    "claim": "TooSmall", "verdict": "...", "exit_code": 10,
    "trace_position": 0,
    "counterexample_levels": [],
    "violating_level": 0
  }
}

The counterexample_levels array includes the initial level followed by the
post-state level of each prefix transition. trace_position is the counterexample's
reported position. violating_level is the level where TooSmall is false. Read
these values from the returned counterexample rather than treating a refutation
as an engine failure.

Only after the files and analysis are complete, make your final action:

echo '<<<EVAL-COMPLETE>>>'
