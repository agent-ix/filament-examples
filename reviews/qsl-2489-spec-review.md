---
id: SR-019
title: "spec-review of QSL-2489 FSL claim to matrix row via quoin evidence (FR-003, TC-003)"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-examples; PR #55 at 87769de91cc8b526185cb8f6f8286b2619b610e2; agent-evals/spec/functional/FR-003-fsl-claim-evidence-link-scenario.md, agent-evals/spec/test-cases/TC-003-fsl-claim-evidence-link-scenario.md"
review_set: subset
---

## Summary

Ticket: QSL-2489. Spec review of FR-003 (scenario EV-005 and its variants EV-105 and EV-205) and TC-003 in PR #55. The quoin and quire claims were checked empirically with the released `quoin` 0.36.7 and `quire` 0.52.0 on scratch git repositories outside the checkout. Those repositories held an FR with Verification `Analysis`, a SuiteRegistry row with Evidence Kind `Analysis`, and hand-written `entries` results recorded with `--adapter entries`. The fsl claims were checked against filament-driver v0.1.4 and the filament-spec-language and filament-docs sources on main.

## Examined scope

- `quire matrix` shows an `Analysis` criterion as `method-without-symbol`, and the suite row with Evidence Kind `Analysis` validates. Confirmed. (examined)
- The `entries` shape `{entries:[{symbol,path,outcome,traceIds}]}` is accepted, and an unknown outcome is refused. A passing entry binds the criterion: `quoin matrix --json` `evidence_status` is `bound` and `quoin evidence audit --json` lists it under `healthy`. `inspect-mocks` records an empty inspection. Confirmed. (examined)
- After a bound pass, a later `fail` or `error` entry gives `stale` / `stale-evidence`, and `audit --strict` exits 1. A later run with no entry for the symbol gives `suspect` / `vacuous-evidence`. Confirmed. (examined)
- A commit that edits the claim file after the run leaves the criterion `bound`, so the spec is right that quoin does not detect a claim edited after its run, and `claim-edited` is needed. Confirmed. (examined)
- A first run that records only a `fail` entry: FND-001. (examined)
- The `analyze` document `items[].record.result` and its labels `proved`, `refuted`, `tested`, `measured`, `inconclusive`, `unsupported`, `declined`, `incomplete` and `failed` match fsl-command's `plan/outcome.rs`. (examined)
- The mapping of `measured` to pass: FND-002. (examined)
- EV-205: the `extraction-requires-run` code exists in fsl-foundation and the docs, and "Rules in a Markdown specification" is marked planned. (examined)
- FR-003-AC-1..7 and TC-003: each criterion can fail. The ids are unique. (examined)

## Verdict

Not mergeable. There are two high findings. First, the EV-105 expectation contradicts the released quoin: a run whose only entry is `fail` binds nothing, so the row reads `no run evidence` with an `undischarged` finding, never `stale` with `stale-evidence`, and FR-003-AC-5 cannot pass as written. Second, the outcome map sends every `measured` result to `pass`, although the docs say a rejected measurement fails a run. The rest of the evidence-link design matches the released tools closely.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | quoin 0.36.7 binds only from passing entries: a first and only run with outcome fail leaves FR-001-AC-1 at `no run evidence` with an `undischarged` finding, not `stale`/`stale-evidence`; EV-105, the refusal table's first row and AC-5 assume a prior binding that EV-105 never makes. The table's `no run evidence` "when the store holds no run" is also inexact | agent-evals/spec/functional/FR-003-fsl-claim-evidence-link-scenario.md:152 |
| FND-002 | high | The outcome map turns every `measured` into pass, but a measured result carries accepted/rejected/undecided and the docs say a rejected measurement fails a run, so a rejected statistical claim would discharge the criterion | agent-evals/spec/functional/FR-003-fsl-claim-evidence-link-scenario.md:149 |
| FND-003 | low | The EV-205 probe names no fsl command: the docs say only a `compile` request refuses the extraction member with `extraction-requires-run`, while `run` refuses it as an unknown member. Nothing says what a different refusal code means for activation | agent-evals/spec/functional/FR-003-fsl-claim-evidence-link-scenario.md:104 |
| FND-004 | low | The relationships use `ix://agent-ix/filament-examples/FR-001`/`FR-002`, but the repository already holds other FR-001 and FR-002 documents (assurance, quoin-first-spec, quire-first-document projects, which use ix://example/... namespaces), so the repo-level id is ambiguous | agent-evals/spec/functional/FR-003-fsl-claim-evidence-link-scenario.md:6 |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
| FND-004 | fixed | - |
