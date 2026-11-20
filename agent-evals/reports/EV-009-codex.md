# EV-009: blind CLI journey

The single Codex sample passed all four runnable contracts using the shipped FSL
executable, CLI help/errors and original problem inputs. The transcript boundary
and all four public-command evidence checks passed. Export and Claude remain
pending, so program completion is partial.

| Measurement | Actual result |
| --- | --- |
| SDK report generated | 2026-11-20T03:57:59.564Z |
| Agent / model | Codex / gpt-6.1-sol |
| Samples / pass rate | 1 / 1/1 |
| SDK exit reason | complete |
| SDK tool calls | 15 |
| Recorded command strings | 15 |
| Assistant turns | Unavailable in the SDK report |
| SDK latency | 28568 ms |
| Configured whole-assertion budget | 120000 ms |
| Failure class | None for this sample |

| Active contract | Independently checked result | Public exit | Command evidence |
| --- | --- | --- | --- |
| check | Original quote function accepted | 0 | Passed |
| run | Five-pallet quote completed with integer 29 | 0 | Passed |
| analyze | WithinLimit proved with certified exhaustive exploration | 0 | Passed |
| monitor | WithinThree violated; evaluation origin 0, observed position 2, complete closed-scope stream | 10 | Passed |

The configured assertion budget covers validation, original-domain regeneration,
all four CLI children and final artifact checks under one absolute deadline.
The monitor comparison retains the complete ordered events and final summary.

The required export contract remains pending under QSL-1979; no export attempt or
successful-refusal substitute is included. Claude has zero attempts in this
scenario and retains its existing authentication prerequisite. The sample's
`program_status` is `partial/export-pending` and `overall_completion` is `pending`.
This is evidence for four runnable contracts, not completion of every FSL verb.

The run reused the accepted L5 inputs: driver 18d6fb0, FSL deac8870a,
codegen 889e9995 and contract IR e576f749. Raw SDK reports and transcripts are
retained privately. No FSL build, authentication probe, old canary or repeated
reference gate formed part of this sample.
