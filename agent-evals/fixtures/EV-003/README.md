---
title: Prove the meter's safety bound
type: how-to
---

Run EV-003 to measure whether an agent can state a safety property, prove it with
public `fsl analyze`, and read a meaningful counterexample.

The original user problem has one meter, initially at level 0. Its declared
integer domain is 0 through 4. The supplied operation advances to 3 and then
stays there. `WithinLimit` requires the level always to be at most 3;
`TooSmall` requires it always to be at most 2. The reachable levels are 0, 1, 2
and 3, so analysis must prove the first claim and refute the second at position 3.

The semantic model was generated from the original `reference/counter.model.mjs`
through this repository's public `tools/model.mjs` model owner. The agent receives
that model as fixed user input. All proof and refutation results come from the
public analyzer. The reference source stays outside prepared agent workspaces.

## Deterministic checks

From the repository root, run the new grader's local artifact checks:

```sh
cargo test --locked --manifest-path agent-evals/Cargo.toml safety_analyze::tests -- --test-threads=1
```

Run the explicit public-CLI reference lane with the accepted binary:

```sh
FSL=/absolute/path/to/fsl cargo test --locked --manifest-path agent-evals/Cargo.toml scenarios::safety_analyze::tests::reference_proves_and_refutes_through_public_cli -- --exact --ignored --test-threads=1
```

The reference lane creates original inputs in a temporary workspace, runs both
requests, and grades the saved artifacts with independent analyzer invocations.
Artifact tests reject changed inputs, different properties, unsupported or
incomplete results, false counterexample values and invented summary fields.

## Agent-run acceptance

Select only EV-003 through the shared suite. After review, run the configured
Codex and Claude models sequentially, once each. The scorecard records the actual
samples, turns, commands, result and failure classification.

A passing artifact set preserves the fixed inputs, adds the two requested
properties, saves both complete analyzer outputs, and reports the independently
verified verdicts and counterexample in `result.json`. The grader requires a
certified exhaustive `proved` record and a `refuted` record with levels
`[0, 1, 2, 3]`. It compares semantic result fields and binds the proof certificate
to the current submitted claim, independently of timing metadata. The shared
provider also requires observed public analyze commands for both requests.

The command-evidence unit uses an original synthetic SDK transcript. It exercises
both required commands and rejects a missing command, another verb or request,
and an echoed command. This fixture is deterministic assertion coverage and
does not represent a live agent run. The final success sentinel alone cannot
satisfy the artifact and command checks.

## Assertion budget

One 30-second deadline covers artifact checks and both independent analyzer
commands. A timed-out analyzer is terminated and reaped. Deadline exhaustion is
reported as a harness error, so it cannot become an agent failure or a proof.
Stdout and stderr use temporary file-backed capture to retain complete output
without pipe-buffer blocking.
