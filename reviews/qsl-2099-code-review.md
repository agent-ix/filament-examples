---
id: SR-002
title: "code review of the Rust example runner (QSL-2099)"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-examples; tools/fsl-examples/**, tools/licences.mjs, tools/licences.test.mjs, tools/steps.test.mjs, tools/rename-ids.mjs, Makefile, config.mk, examples/*/Makefile, examples/*/expected/**, image/Dockerfile, image/licenses/**, install-test/steps.md, .github/workflows/check.yml, README.md, tools/README.md"
review_set: subset
---

## Summary

Ticket: QSL-2099. PR agent-ix/filament-examples#32 at ba36b565a3c85a9746c90cde0cce0fd984d4707a. Code review with the rust-review lane (Agent IX rust-style baseline) over `git diff origin/main...HEAD`.

Evidence: `cargo test --locked` 19/19, `cargo clippy --locked --all-targets` clean, `node --test tools/*.test.mjs` 33/33, `node tools/licences.mjs check` passes, and `make check staged` with the real fsl 0.1.3 driver gives check 282/0 and staged 21/0. Docker and install-test were not run (lead builds the image).

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | low      | The new runner stage repeats `ARG RUST_VERSION=1.98.1`, a second copy of the driver stage's toolchain default; a bump of one leaves the other behind. Declare it once before the first FROM and use bare `ARG RUST_VERSION` in both stages. | image/Dockerfile:19, image/Dockerfile:58 |

## Verdict

APPROVE. Structural comparison (category, result, last_stage, first diagnostic code and cause, items with witness fields) is correct and every AC-named field has a failing negative test; message text lives only in `.messages.txt` (text, no digests) and feeds only the README regions, which are generated from the snapshot, so a live rewording never fails the gate. The `expect_verdict` deviation loses no check: run.mjs never compared it to live output either (only exit plus bytes), and the verdict is covered by result and items. Staged, promote and record rules port run.mjs one for one, as do the README-region and index freshness checks (no README or index.json changed in the diff). Dropping only run.mjs and run.test.mjs is right: steps, licences, rename-ids and model are still used by other targets. The 8 empty-stdout staged records become `{"outcome": null}`, as before. Licence data lists the runner's 24 built crates, each with its texts in THIRD-PARTY-LICENSES.txt, and the lock drift check covers non-built lock entries via `notBuilt`. Rust follows rust-style: Copy code enum plus thiserror envelope, no unwrap or indexing on input, documented pub items, forbid(unsafe_code). No Python, no logic-bearing shell, no pins outside Cargo besides the existing toolchain ARG.
