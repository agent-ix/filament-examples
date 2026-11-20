Use only the shipped `fsl` executable, its own help and errors, and these original
problem inputs. Work inside this directory. Do not read documentation, grammar,
source repositories, evaluator code or reference answers. Keep every supplied
source, request, domain and trace file unchanged.

Discover how to perform these four supported CLI tasks. Save the actual machine
readable command outputs, including the monitor's complete output stream:

| Task | Original input | Output to save |
| --- | --- | --- |
| Check that the quote function is well formed | `calculation.fsl` | `checked.json` |
| Evaluate the freight quote for five pallets | `run-request.json` | `run-result.json` |
| Check WithinLimit over every behavior of the supplied stock system | `analyze-request.json` | `analysis-result.json` |
| Check WithinThree against the recorded stock counts | `monitor-request.json`, `readings.jsonl` | `monitor-result.jsonl` |

The quote charges nine plus four per pallet. The inventory starts at zero and
adds one until three; its domain also admits four, so its claimed limit matters.
The recorded stream contains counts zero, two and four, with authoritative
closure of positions zero through two. Report the actual monitor violation,
including its evaluation and observed positions. A violation is the expected
result of that recording, not a reason to weaken its rule or alter its input.

Use the real public commands. A hand-written result, success marker, fake wrapper,
or changing a request to a different easy task cannot satisfy the evaluation.
This task covers check, run, analyze and monitor. Export is a separate pending
program requirement and is outside this runnable task.

Once all four tasks are complete, put `FSL-EVAL-DONE` only at the end of your
final response.
