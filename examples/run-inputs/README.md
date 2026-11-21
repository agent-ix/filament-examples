---
type: reference
---

# Run-inputs cases

Run-inputs is a corpus for what a caller supplies to `fsl run` and what the run returns. Each case submits a request and compares the outcome with a recorded result.

Run the cases from the repository root:

```sh
cargo run --quiet --locked --manifest-path tools/fsl-examples/Cargo.toml -- check run-inputs
```

Set `FSL` to the driver binary.

## Cases

- `call-*` call a function of [rules/calls.fsl](rules/calls.fsl) with arguments. The valid cases cover no argument, several arguments matched by name, an integer of 23 digits, a negative integer and zero, a Boolean, and a sequence at its declared maximum and minimum. The invalid cases put each argument one step outside its declared form: one above and one below a range, one element too many, a wrong element kind, a missing and an unknown parameter, a Boolean for an integer, and integer strings that are not decimal strings (`05`, `-0`, `+5`, `5.0`, empty). Each refusal names its cause.
- `call-tagged-*` supply the same kinds in the typed canonical form of FR-181, `{"type": ..., ...}`.
- `unit-*` check units with no declaration, only a profile, only a type alias, and one function.
- `option-*` run one call under `--engine`, `--request-bytes`, `--dependent-bytes` and `--limit`.
- `monitor-empty-stream` monitors a stream that has no record.
- `plan-*` run a verification plan through the `verification_plan` member of a run request.

## Staged

- `call-tagged-*` wait on QSL-2103. This driver build reads an integer as `{"integer": "5"}` and a sequence as `{"sequence": [...]}`, and refuses the FR-181 tagged form `{"type": "integer", "value": "5"}` and every rational, decimal, text and set form it tried, as `wrong-value-kind`. FR-550-AC-1 asks for every FR-181 kind in its typed canonical form.
- `plan-*` wait on QSL-2502. This build refuses the `verification_plan` member (`invalid-request`, "a verification plan is run by the plan coordinator route"), exit 20. FR-824 asks for the plan run outcome with one disposition per step and the exits 0, 10 and 22.

## Coverage

| Criterion | Cases |
| --- | --- |
| FR-550-AC-1 values in and out, integers at any magnitude, collections | `call-zero-arguments`, `call-two-arguments`, `call-arguments-reordered`, `call-big-integer`, `call-negative-integer`, `call-zero-integer`, `call-boolean`, `call-sequence-*` |
| FR-550-AC-1 typed canonical form of each kind | `call-tagged-*` (staged) |
| FR-550-AC-2 refused arguments name the parameter and the cause | `call-digit-*`, `call-sequence-above-max`, `call-sequence-wrong-element-kind`, `call-wrong-kind-*`, `call-integer-*`, `call-missing-argument`, `call-unknown-parameter`, `call-unknown-function` |
| FR-746, FR-339 zero-declaration unit (W64: parses, refused at check) | `unit-header-only`, `unit-profile-only`, `unit-alias-only`, `unit-one-function` |
| FR-306 engine selection | `option-engine-interpreter`, `option-engine-unknown` |
| FR-460, FR-461, FR-720 limits | `option-request-bytes-*`, `option-dependent-bytes-too-small`, `option-limit-*` |
| FR-161-AC-2 pending rather than proved | `monitor-empty-stream` |
| FR-824 plan run outcome and exit ordering | `plan-passes`, `plan-fails`, `plan-blocks` (staged) |

Not covered here: the multi-item results of FR-550-AC-3 (this driver's run request selects one function), reference and population arguments (FR-550-AC-2 `dangling_reference`; they need a model and a state environment, which the shop system exercises), the rejected members of a plan request (FR-824 `unknown-member`, `wrong-value-kind`, `invalid-value` pointers: the build refuses the whole member, so no case can tell them apart), `--engine aot` and `--engine jit` (this build does not link them; the shop system has the cases), `--clocks` and the zero-declaration parse (the grammar system).
