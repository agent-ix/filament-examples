---
id: SR-96053
title: "QSL-2287 filament-examples release-build README code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-examples; QSL-2287; README.md; PR 131 head 544fb17d93642cef27100595b3ea79a8b5be7ff1"
review_set: subset
---

## Summary

Ticket: QSL-2287. The README's from-source steps now build the driver with `--release` and point `make check staged` at `filament-driver/target/release/fsl`; the sentence about `fsl-plugin-launch` names the same directory. The three edited lines agree with each other and sit outside every generated recorded-output region. No other file in the repository asserts the README text.

## Verdict

**PASS** at head 544fb17d93642cef27100595b3ea79a8b5be7ff1. Two low findings; neither blocks this README-only change.

Checks: the diff was read line by line; a repository search found no test or generator that reads the from-source commands. No build, example run or `make check` was run (README only, per brief).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The repository's CI source-driver step still builds the driver in debug and passes `target/debug/fsl` to `make check staged`, so CI and the README now differ. That step is the examples check, not the alpha journey gate, and has no 300 s per-command timeout, so it is slower but not failing. It is a workflow file, so changing it needs the repository owner; track it as a follow-up rather than in this PR. | .github/workflows/check.yml:82; .github/workflows/check.yml:83 |
| FND-002 | low | `--release` selects the driver's `release` profile (thin LTO, one codegen unit, about 6 GB peak rustc memory per the driver's Cargo.toml), while published driver archives use `--profile dist` (no LTO, 16 codegen units). For a reader building on a small machine, and for "time what users get", `dist` would be the closer and lighter match. Both are far inside the timeout. | README.md:84; README.md:85; README.md:88 |

## Examined units

```json
[
  {"id":"readme-build","path":"README.md","role":"examined","excerpt":"cargo build --release --locked -p fsl-cli -p filament-plugin-host --manifest-path filament-driver/Cargo.toml"},
  {"id":"readme-check","path":"README.md","role":"examined","excerpt":"make check staged FSL=$PWD/filament-driver/target/release/fsl"},
  {"id":"readme-launcher","path":"README.md","role":"examined","excerpt":"The build places `fsl-plugin-launch` beside `fsl` in `filament-driver/target/release`."},
  {"id":"ci-source-driver","path":".github/workflows/check.yml","role":"examined","excerpt":"CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 cargo build --locked -p fsl-cli --manifest-path \"$RUNNER_TEMP/filament-driver/Cargo.toml\""}
]
```
