Repair `postage.fsl` so that `postage(units)` implements its stated charge rule:
7 base credits plus 3 credits per unit, with negative counts treated as zero.
Keep its name, one Integer parameter, and Integer return type. Write the repair
in that same file; do not change the task into another calculation.

Use only the source, the refusal text supplied below, and the public `fsl` CLI
for feedback. Do not read documentation, source checkouts, SDKs, other example
programs, or grader files. The `fsl` on PATH is the trusted tool: do not replace
it, create a wrapper, or change tool configuration.

Run `fsl check postage.fsl` to validate your repair. A source that merely parses
or checks is not enough: the repaired function must return the required charge
for negative, zero and positive unit counts.

When finished, run `echo '<<<EVAL-COMPLETE>>>'` as your final action, and only
then. The independent grader executes the repaired function on hidden boundary
inputs; your response or this sentinel cannot make an incorrect function pass.
