---
id: SR-1436
title: "QSL-2485 PR 62 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-examples; QSL-2485; agent-evals/ frozen d867fd71ad84ab24079765668618eaa757dec8c0"
review_set: subset
relationships:
  - { target: "ix://agent-ix/filament-examples/FR-001", type: references }
---

## Summary

Diff-scoped independent review of QSL-2485, filament-examples PR 62. Code review includes the Rust review lane. The grader uses the real public check and executes the extracted invariant with bounded inputs through the public CLI; generic agent execution remains in the declared SDK dependency. EV-101 remains pending QSL-1980.

## Verdict

**CONDITIONAL** — No additional semantic or stub defects found in the examined scenario. This is a manual, scoped acceptance-to-code review, not a repository matrix or whole-harness conformance claim. Live Codex/Claude evidence is still pending, so scenario acceptance remains conditional.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

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

## Disposition context

Round 1 adds no gap-analysis findings. SR-1435 records the two original findings: FND-001 fixed, FND-002 still open for command-evidence cwd binding. Live Codex/Claude acceptance remains pending. Accepted new focused checks and Clippy; none repeated.

### Round 2 context

SR-1435 FND-002 is now fixed; both original code findings have fixed latest dispositions. No new gap analysis was run. The integration is ready for the authorized single Codex then single Claude live gate. Live acceptance remains pending.

## Runtime integration correction

Inspected only ae74cf2..fa959c7712134b68b08a10eed3d79cd5e82d7c65. This addresses actual observed runtime failures, not another sweep or a reopening of either original finding.

The original Codex sample authored a correct model and passed all 156 public semantic probes, but its command evidence was zero because the parser missed the actual SDK typed CommandExecution events. The original Claude sample performed no task work because it timed out at the project-trust dialog. These original outcomes must remain in the scorecard.

The fix reads published typed event_msg/item_completed/CommandExecution argv and file-URI cwd via the declared url dependency. It ignores aggregated output and assistant prose for positive command evidence, does not evaluate opaque JavaScript, and retains custom tool inputs for boundary inspection. The retained-evidence subcommand reuses command/boundary checks without invoking model checking, the 156 semantic probes, or an agent.

Claude trust preparation changes only the isolated per-run home config, guards the configured home against the actual HOME, canonicalizes the SDK workspace and constrains it beneath the sibling private work root. It trusts that one workspace without changing operator settings.

No additional findings in this scoped runtime diff. Accepted reported typed-execution/context/space-URI/comment-output/invalid-URI tests, private trust positive/outside-rejection tests, and scoped all-target Clippy. No tests, builds, probes, live agents or original findings were rerun or re-reviewed. Ready for code-lead-directed offline retained Codex evidence grading and one affected Claude retry; no broader live rerun is warranted.

## Retained sed classification correction

Inspected only the actual sed-classification change in agent-evals/src/provider.rs through 72e949dde7089e06b718971d6983ec44854e2ffd. Generic equals-sign splitting no longer invents an absolute path from quoted sed replacement data. Nonempty shell-assignment and long-option prefixes still expose their path values for boundary inspection. The exact retained sed command and opaque custom-tool input are represented in the new focused test, alongside forbidden quoted absolute evaluator operands and SOURCE assignments. No additional findings in this bounded correction; prior findings remain closed and were not re-reviewed.

Accepted the reported new exact-input test without rerun. Existing retained Codex model correctness, all 156 probes, typed command count 11 and check evidence true remain accepted; this correction only addresses the boundary false positive. One retained Codex evidence grade may proceed. Claude retry reached the task trust boundary successfully but stopped at Not logged in before performing task work: authentication acceptance is pending, not an agent-task error. No more live runs or authentication probes are authorized here. Original reports and failure classifications must remain honest.
