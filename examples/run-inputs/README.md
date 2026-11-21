---
type: reference
---

# Run-inputs cases

Run-inputs is a corpus for what a caller supplies to `fsl run` and what the run returns. Each case submits a request and compares the outcome with a recorded result.

Run the cases from the repository root:

```sh
cargo run --quiet --locked --manifest-path tools/fsl-examples/Cargo.toml -- check run-inputs
```

Set `FSL` to the driver binary. Every record in this system was written by the driver at FSL f5977125 (`fsl-driver-aa71f20dd30bab9ae2ca66580ca407e434164cca`, version string 0.1.4).

## Cases

- `call-*` call a function of [rules/calls.fsl](rules/calls.fsl) with arguments. They send arguments in the FR-181 typed canonical form, `{"type": "integer", "value": "5"}`, which FR-550 requires for reading and writing. Only `call-zero-arguments` runs: it has no argument, and its recorded result is in the driver's legacy untagged form `{"integer": "5"}`, which is the one legacy-wire case. `call-unknown-function` also runs. The others are staged on QSL-2758.
- `call-tagged-*` supply rational, text and set forms and wait on QSL-2758.
- `unit-alias-only` and `unit-one-function` check units with only a type alias and with one function.
- `option-*` run one call under `--engine`, `--request-bytes`, `--dependent-bytes` and `--limit`. Those that settle before values are read, or refuse the option, run; two that complete a call are staged on QSL-2758.
- `monitor-empty-stream` monitors a stream that has no record.
- `plan-*` run a verification plan through the `verification_plan` member of a run request.
- `call-three-items-complete` selects three items in one run request.

## Staged

- `call-*` (except the zero-argument case), `option-engine-interpreter` and `option-request-bytes-large` wait on QSL-2758: `fsl run` reads and writes values in a legacy untagged form (`{"integer": "5"}`, `{"sequence": [...]}`) and refuses the FR-181 typed form as `wrong-value-kind`, exit 20. FR-550-AC-1 requires the typed canonical form for arguments and results.
- `call-tagged-*` wait on QSL-2758 for the same reason.
- `plan-passes`, `plan-fails`, `plan-blocks` and `plan-null-members-as-absent` wait on QSL-2103. QSL-2502 was cancelled as a duplicate of QSL-2511 (Done), and the newer driver still refuses the member: `invalid clause request member verification_plan: a verification plan is run by the plan coordinator route` (`invalid-request`, exit 20). FR-824 asks for the plan run outcome with the exits 0, 10 and 22.
- `call-three-items-complete` waits on QSL-2103: `invalid clause request member items: items is handled by the multi-selection runner`, exit 20, where FR-550-AC-3 asks for three dispositions.

## Coverage

| Criterion | Cases |
| --- | --- |
| FR-550-AC-1 values in and out in the typed canonical form | `call-zero-arguments` (legacy wire), `call-two-arguments`, `call-arguments-reordered`, `call-big-integer`, `call-negative-integer`, `call-zero-integer`, `call-boolean`, `call-sequence-*`, `call-digit-*` (staged) |
| FR-550-AC-1 typed canonical form of each kind | `call-tagged-*` (staged) |
| FR-550-AC-2 unknown function | `call-unknown-function` (names the function; there is no parameter) |
| FR-550-AC-3 multi-item results | `call-three-items-complete` (staged) |
| FR-746, FR-339 zero-declaration unit (W64: parses, refused at check) | `unit-alias-only`, `unit-one-function` |
| FR-306 engine selection | `option-engine-interpreter`, `option-engine-unknown` |
| FR-460, FR-461, FR-720 limits | `option-request-bytes-*`, `option-dependent-bytes-too-small`, `option-limit-*` |
| FR-161-AC-2 pending rather than proved | `monitor-empty-stream` |
| FR-824 plan run outcome and exit ordering | `plan-passes`, `plan-fails`, `plan-blocks`, `plan-null-members-as-absent` (FR-824-AC-6, staged) |

Not asserted, because the product answer cannot be told from the required one by exit code (a staged case needs a different exit): the refusals of FR-550-AC-2 (a value out of range, a sequence over its bound, a wrong element kind, a Boolean for an integer, integer strings that are not decimal, a missing and an unknown parameter) all exit 20 both today and when fixed, and the typed requests exit 20 for every value. The first-round cases pinned `invalid-value` or `wrong-value-kind` at `position 0` with an empty location, and `cause` null for the missing and unknown parameter, where FR-550-AC-2 asks for the parameter and the location in the value. They are removed until QSL-2758 lets a typed request reach the check. The same holds for the FR-824-AC-3 and AC-6 pointer refusals (`seed` `"7"` at `/request/verification_plan/seed`): the build refuses the whole member. The header-only and profile-only unit cases are removed: both recorded the message "a type alias has none" for a unit that has no alias.

Not covered here: reference and population arguments (the run request has no member that supplies a state document or population, so the tank model and `data/tank-0.json` cannot be given to a call; FR-550-AC-2 `dangling_reference`), `--engine aot` and `--engine jit` (this build does not link them; the shop system has the cases), `--clocks` and the zero-declaration parse (the grammar system).
