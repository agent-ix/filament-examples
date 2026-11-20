---
id: SR-1435
title: "QSL-2485 PR 62 code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-examples; QSL-2485; agent-evals/ frozen d867fd71ad84ab24079765668618eaa757dec8c0"
review_set: subset
relationships:
  - { target: "ix://agent-ix/filament-examples/FR-001", type: references }
---

## Summary

Diff-scoped independent review of QSL-2485, filament-examples PR 62. Code review includes the Rust review lane. The grader uses the real public check and executes the extracted invariant with bounded inputs through the public CLI; generic agent execution remains in the declared SDK dependency. EV-101 remains pending QSL-1980.

## Verdict

**CONDITIONAL** — Both original findings are fixed. The reviewed integration is ready for the authorized single Codex sample followed by the single Claude sample. Live scenario acceptance remains pending; deterministic checks are not agent success rates.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The assertion accepts source/reference-answer reads: it extracts commands only to find fsl invocations and never rejects sandbox breaches. Copying the reference model from the evaluator checkout, then checking it, can pass EV-001 despite violating the task. Reject forbidden evaluator/source paths and repository fetches from actual tool-call inputs before granting a pass. | agent-evals/src/provider.rs:318 |
| FND-002 | medium | Command evidence matches any whitespace token rather than the parsed check operand. For example, fsl check eval other other.fsl --format json # model.fsl is accepted although model.fsl is only a shell comment; redirection destinations and a different absolute executable named fsl also count. Parse actual shell commands/arguments and bind the required operand and executable identity to the trusted public invocation. | agent-evals/src/provider.rs:203 |

## Coverage

Frozen diff: all 17 changed paths under agent-evals/, including SDK/Cargo dependency declarations and locks, data config, agent model settings, original fixture/task, pending variant, shared library, provider, model grader and preparation test. No reviews additions were read. No applicable AssuranceProfile was supplied in the scoped agent-evals/spec tree.

Reconciliation: manual scoped requirement/task-to-assertion inspection; matrix intentionally not run under the review brief. No traceability totals or whole-repository assurance claim.

Plan completion: not assessed

Semantic review: original English object/field/population/operation/invariant obligations compared with real check, structural CST analysis and all-156-binding oracle loop. Exact names and direct self-field scope are explicit in the task; aliases and equivalent local expressions are accepted. Dummy declarations, Boolean shortcuts and wrong frames have meaningful negative assertions. Tests are opt-in because they require the configured public CLI; their reported execution is accepted. No vendored model, executable, schema, SDK implementation or docs snapshot was added.

Evidence accepted without rerun: eight distinct deterministic tests, affected structural-negative test, scoped Clippy -D warnings, SDK registration. One doctor command-probe exit 0 is not authentication/live-agent evidence. No tests, builds, gates, matrix, doctor or live samples were repeated. Full older FR-001/FR-002 scheduling, installer, scorecard and canary obligations are context only and outside the authorized minimal scenario integration.

SDK integration is minimal: configuration is data, scenario registry/setup/assertions are native, SDK owns process/transcript/metrics, temporary credentials are removed on ordinary result/error paths, and private raw reports are not committed. No global configuration or schedule edits appear.

## Examined units

```json
[
  {
    "id": "EV-001-object",
    "path": "agent-evals/fixtures/EV-001/task.md",
    "role": "examined",
    "excerpt": "- Exactly one object declaration named StorageBin. It is concrete, without inheritance, and its identity is the field key."
  },
  {
    "id": "EV-001-fields",
    "path": "agent-evals/fixtures/EV-001/task.md",
    "role": "examined",
    "excerpt": "- Exactly four required scalar fields: key is an integer from 1 through 16; limit is an integer from 1 through 12; occupied is an integer from 0 through 12; locked is Boolean. You may use native bounded integer types or aliases to them."
  },
  {
    "id": "EV-001-population",
    "path": "agent-evals/fixtures/EV-001/task.md",
    "role": "examined",
    "excerpt": "- The object declares its closed population bins."
  },
  {
    "id": "EV-001-operation",
    "path": "agent-evals/fixtures/EV-001/task.md",
    "role": "examined",
    "excerpt": "- Its sole operation reserve takes one parameter amount, an integer from 1 through 12, returns Boolean, and declares its write frame as exactly the receiver's occupied field. It creates/deletes nothing."
  },
  {
    "id": "EV-001-invariant",
    "path": "agent-evals/fixtures/EV-001/task.md",
    "role": "examined",
    "excerpt": "- Its sole state clause is the current invariant WithinCapacity applies to StorageBin: occupied must not exceed limit. Write that relationship directly over the two self fields; it must hold for every allowed pair, independent of key/locked. Equivalent Boolean/arithmetic formulations are welcome; comments or a trivially true predicate are not a model of this requirement."
  },
  {
    "id": "EV-001-isolation",
    "path": "agent-evals/fixtures/EV-001/task.md",
    "role": "examined",
    "excerpt": "Do not inspect the evaluator, reference answers, or other source repositories."
  },
  {
    "id": "EV-001-check",
    "path": "agent-evals/fixtures/EV-001/task.md",
    "role": "examined",
    "excerpt": "Author a genuine native, source-owned object model in `model.fsl`, then check it with `fsl check` and save that command's JSON outcome as `check.json`."
  },
  {
    "id": "provider-boundary",
    "path": "agent-evals/src/provider.rs",
    "role": "examined",
    "excerpt": "//! Thin Rust consumer of the SDK's published external-provider protocol."
  },
  {
    "id": "native-setup",
    "path": "agent-evals/src/main.rs",
    "role": "examined",
    "excerpt": "//! Project setup and the thin Rust provider for the published cli-evals SDK."
  },
  {
    "id": "shared-types",
    "path": "agent-evals/src/lib.rs",
    "role": "examined",
    "excerpt": "//! FSL-specific scenarios behind the published cli-evals external provider."
  },
  {
    "id": "prepare-test",
    "path": "agent-evals/tests/prepare.rs",
    "role": "examined",
    "excerpt": "//! The new scenario is prepared through the real external-provider executable."
  },
  {
    "id": "pending-package",
    "path": "agent-evals/fixtures/EV-101/scenario.toml",
    "role": "examined",
    "excerpt": "waits_on = \"QSL-1980\""
  },
  {
    "id": "sdk-dependency",
    "path": "agent-evals/package.json",
    "role": "examined",
    "excerpt": "\"@agent-ix/cli-agent-evals\": \"0.3.0\""
  },
  {
    "id": "sdk-config",
    "path": "agent-evals/cli-agent-evals.config.mjs",
    "role": "examined",
    "excerpt": "timeoutMs: 300000,"
  },
  {
    "id": "readme-semantics",
    "path": "agent-evals/README.md",
    "role": "examined",
    "excerpt": "The invariant is evaluated through public FSL execution for all 156 occupied/limit pairs. Recorded agent tool calls must also include checking the authored file. A success marker, comment or dummy declaration cannot satisfy those checks."
  }
]
```

## Dispositions

Round 1 inspected only d867fd71ad84ab24079765668618eaa757dec8c0..253fd2d5883a0e559902d9bd0ab1a627f998b5d4. Accepted four new integrity checks and scoped Clippy without rerun. This is transcript integrity checking, not an operating-system sandbox.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Assert now enforces actual shell and non-shell tool-call boundaries, rejects reference/source paths and repository fetches, resolves symlink ancestors and tracks boundary-scan cwd. |
| FND-002 | still-open | Literal parsing fixes comments, redirects and trusted executable matching, but Assert flattens tool calls into command strings and seeds each evidence match with the workspace, discarding call workdir and Claude persistent cwd. A separate cd docs followed by fsl check eval x model.fsl incorrectly counts docs/model.fsl as the workspace model; a valid call with workdir docs and operand ../model.fsl can incorrectly fail. Bind evidence using the same recorded-input cwd tracking as the boundary scan. |

### Round 2

Inspected only the FND-002 fix in 253fd2d5883a0e559902d9bd0ab1a627f998b5d4..5f85f57781df6f8443e495110cd8e7d600e536ce. FND-001 remains fixed from round 1 and was not reopened or rechecked. Accepted the new retained-transcript context test and scoped Clippy without rerun.

| FND | outcome | reason |
| --- | --- | --- |
| FND-002 | fixed | Assert now retains ordered RecordedInput contexts. Evidence uses per-call Codex workdir and Claude persistent cwd while binding the expected operand to the exact workspace file. The targeted fixture rejects docs/model.fsl, accepts ../model.fsl from docs, and verifies that a Codex workdir override does not leak into the next call. |
