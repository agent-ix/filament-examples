---
id: SR-2026111919340478491681432
title: PR 71 gap-analysis review
type: SpecReview
analysis: gap-analysis
scope: agent-ix/filament-examples; QSL-2727/QSL-2268; complete PR 71 diff
review_set: subset
examined_scope:
- id: README.md
  path: README.md
  role: examined
  excerpt: '| [architecture](examples/architecture) | Enterprise | 35 | 7 |'
- id: examples/architecture/data/start-available.json
  path: examples/architecture/data/start-available.json
  role: examined
  excerpt: "{\n  \"format\": \"filament.state.snapshot/v1\",\n  \"identity\": {\n    \"authority\": \"example\"\
    ,\n    \"identity\": \"ix://example/architecture/data/start-available\"\n  },\n  \"model\": {\n    \"identity\"\
    : \"example/architecture\"\n  },\n  \"observation\": \"current\",\n  \"anchor\": {\n    \"kind\": \"initialization\"\
    ,\n    \"name\": \"start\"\n  },\n  \"populations\": [\n    {\n      \"population\": \"ix://example/architecture/teams\"\
    ,\n      \"complete\": true,\n      \"objects\": [\n        {\n          \"key\": \"storefront\",\n        \
    \  \"type\": \"ix://example/architecture/Team\",\n          \"fields\": {\n            \"size\": {\n        "
- id: examples/architecture/example.toml
  path: examples/architecture/example.toml
  role: examined
  excerpt: 'command = "analyze requests/probability/sampling/staged-sampling-claim-missed.json"

    engine = "probability"

    status = "staged"

    expect_exit = 21

    ready_exit = 10'
- id: examples/architecture/expected/claims-unbounded-event.json
  path: examples/architecture/expected/claims-unbounded-event.json
  role: examined
  excerpt: "{\n  \"category\": \"unsupported\",\n  \"last_stage\": \"S3\",\n  \"result\": null,\n  \"first_diagnostic\"\
    : {\n    \"code\": \"unsupported_construct\",\n    \"cause\": \"expression-form\"\n  },\n  \"items\": []\n}\n"
- id: examples/architecture/expected/claims-unbounded-event.messages.txt
  path: examples/architecture/expected/claims-unbounded-event.messages.txt
  role: examined
  excerpt: 'unsupported_construct (expression-form): the operator needs a closed interval under this profile

    '
- id: examples/architecture/expected/models-workload-zero-weight.json
  path: examples/architecture/expected/models-workload-zero-weight.json
  role: examined
  excerpt: "{\n  \"category\": \"success\",\n  \"last_stage\": \"S4\",\n  \"result\": null,\n  \"first_diagnostic\"\
    : null,\n  \"items\": []\n}\n"
- id: examples/architecture/requests/probability/models/models-workload-zero-weight.json
  path: examples/architecture/requests/probability/models/models-workload-zero-weight.json
  role: examined
  excerpt: "{\n  \"format\": \"native-compile/1\",\n  \"request\": {\n    \"models\": [\n      {\n        \"format\"\
    : \"semantic-ir/2.0.0\",\n        \"source\": {\n          \"file\": \"../../../model/architecture.semantic-ir.json\"\
    ,\n          \"authority\": \"example\",\n          \"identity\": \"example:architecture/model\",\n        \
    \  \"document\": \"model\",\n          \"formal_revision\": 1\n        }\n      }\n    ],\n    \"program\":\
    \ {\n      \"source\": {\n        \"file\": \"../../../rules/probability/models/models-workload-zero-weight.fsl\"\
    ,\n        \"authority\": \"example\",\n        \"identity\": \"example:architecture/models-workload"
- id: examples/architecture/requests/probability/sampling/staged-sampling-claim-missed.json
  path: examples/architecture/requests/probability/sampling/staged-sampling-claim-missed.json
  role: examined
  excerpt: "{\n  \"format\": \"native-analyze/1\",\n  \"request\": {\n    \"models\": [\n      {\n        \"format\"\
    : \"semantic-ir/2.0.0\",\n        \"source\": {\n          \"file\": \"../../../model/architecture.semantic-ir.json\"\
    ,\n          \"authority\": \"example\",\n          \"identity\": \"example:architecture/model\",\n        \
    \  \"document\": \"model\",\n          \"formal_revision\": 1\n        }\n      }\n    ],\n    \"program\":\
    \ {\n      \"source\": {\n        \"file\": \"../../../rules/probability/sampling/staged-sampling-claim-missed.fsl\"\
    ,\n        \"authority\": \"example\",\n        \"identity\": \"example:architecture/staged-sampl"
- id: examples/architecture/rules/probability/models/models-workload-zero-weight.fsl
  path: examples/architecture/rules/probability/models/models-workload-zero-weight.fsl
  role: examined
  excerpt: 'language "ix:native" edition "1-draft";

    profile v = "filament.model.complete/v1";

    profile ev = "filament.temporal.event-position.false-extension/v1";

    profile inf = "filament.temporal.infinite-trace/v1";

    model Arch = "example/architecture";


    workload Quiet on Arch { weight Arch::Service::call = 0; }

    '
- id: examples/architecture/rules/probability/sampling/staged-sampling-claim-missed.fsl
  path: examples/architecture/rules/probability/sampling/staged-sampling-claim-missed.fsl
  role: examined
  excerpt: 'language "ix:native" edition "1-draft";

    profile v = "filament.model.complete/v1";

    profile ev = "filament.temporal.event-position.false-extension/v1";

    profile inf = "filament.temporal.infinite-trace/v1";

    model Arch = "example/architecture";


    // Each call to a service fails 1 time in 10,000, so each service stays up through 10 calls with

    // probability about 0.999.

    random Arch::Service::call(ok) ~ { true: 9999, false: 1 };

    workload Traffic on Arch { weight Arch::Service::call = 1; }


    // A call returns its outcome, and the service is up exactly when it succeeded.

    post UpFollow'
- id: examples/parts-saga/requests/protocols/retries-and-timeouts/timeouts-await.json
  path: examples/parts-saga/requests/protocols/retries-and-timeouts/timeouts-await.json
  role: examined
  excerpt: "{\n  \"format\": \"native-compile/1\",\n  \"request\": {\n    \"clocks\": {\n      \"line-clock\": {\n\
    \        \"kind\": \"timestamped_event\",\n        \"timestamp_unit\": \"s\"\n      }\n    },\n    \"models\"\
    : [\n      {\n        \"format\": \"semantic-ir/2.0.0\",\n        \"source\": {\n          \"file\": \"../../../model/parts-saga.semantic-ir.json\"\
    ,\n          \"authority\": \"example\",\n          \"identity\": \"example:parts-saga/model\",\n          \"\
    document\": \"model\",\n          \"formal_revision\": 1\n        }\n      }\n    ],\n    \"program\": {\n \
    \     \"source\": {\n        \"file\": \"../../../rules/protocols/retries-and-time"
- id: examples/rover/example.toml
  path: examples/rover/example.toml
  role: examined
  excerpt: 'command = "analyze requests/staged-vote-probability-missed.json"

    engine = "probability"

    status = "staged"

    expect_exit = 21

    ready_exit = 10'
- id: examples/rover/requests/staged-vote-probability-missed.json
  path: examples/rover/requests/staged-vote-probability-missed.json
  role: examined
  excerpt: "{\n  \"format\": \"native-analyze/1\",\n  \"request\": {\n    \"models\": [\n      {\n        \"format\"\
    : \"semantic-ir/2.0.0\",\n        \"source\": {\n          \"file\": \"../model/rover.semantic-ir.json\",\n\
    \          \"authority\": \"example\",\n          \"identity\": \"example:rover/model\",\n          \"document\"\
    : \"model\",\n          \"formal_revision\": 1\n        }\n      }\n    ],\n    \"program\": {\n      \"source\"\
    : {\n        \"file\": \"../rules/staged-vote-probability-missed.fsl\",\n        \"authority\": \"example\"\
    ,\n        \"identity\": \"example:rover/staged-vote-probability-missed\",\n        \"document\": \"stagedvote"
- id: examples/rover/rules/staged-vote-probability-missed.fsl
  path: examples/rover/rules/staged-vote-probability-missed.fsl
  role: examined
  excerpt: "language \"ix:native\" edition \"1-draft\";\nprofile v = \"filament.model.complete/v1\";\nprofile ev\
    \ = \"filament.temporal.event-position.false-extension/v1\";\nmodel Rover = \"example/rover\";\n\nrandom Rover::sample(faultA)\
    \ ~ { true: 1, false: 999 };\nrandom Rover::sample(faultB) ~ { true: 1, false: 999 };\nrandom Rover::sample(faultC)\
    \ ~ { true: 1, false: 999 };\nworkload Driving on Rover {\n  weight Rover::sample = 1;\n  weight Rover::loseLidar\
    \ = 1;\n  weight Rover::brake = 1;\n  weight Rover::halt = 1;\n  weight Rover::enterFailsafe = 1;\n}\n\n// Each\
    \ operation acknowledges completion with true."
- id: index.json
  path: index.json
  role: examined
  excerpt: "\"case\": \"models-workload-zero-weight\",\n      \"status\": \"running\""
- id: FR-185-AC-10
  path: agent-ix/filament-spec-language@ab98ea12/spec/functional/FR-185-declare-random-parameters-workloads-and-rewards.md
  role: context_only
  excerpt: '`Steady` with `Server::reset: 0` and weight 1 on the other two operations checks and records weight
    0 for `Server::reset`, with no diagnostic; `Steady` with weight 0 on all three operations checks; `Steady` with
    `Server::reset` omitted is still refused `incomplete-workload`.'
- id: FR-192
  path: agent-ix/filament-spec-language@ab98ea12/spec/functional/FR-192-settle-a-statistical-result-on-the-measured-axis.md
  role: context_only
  excerpt: 'FSL''s settlement map in `fsl-replay` SHALL settle every

    `probabilistic-satisfaction` item that EN-4 runs as exactly one FR-331 terminal record with

    the result value `measured`, carried by `TerminalValue::Measured

    (StatisticalVerdict)`, and SHALL never settle it `proved` or `refuted`

    (ADR-024 SV-1 to SV-5, SV-9, SV-10). The record SHALL carry FSpec FR-241''s

    execution disposition and a separate `measured` axis with the values

    `accepted`, `rejected` and `undecided`, and no FR-242 truth and no FR-243

    basis. A Rejected measurement SHALL fail the pipeline as an O-16 violation

    does (ADR-024 RU-2), an'
- id: FR-407-AC-11
  path: agent-ix/filament-specification@7aab94cf/spec/functional/analysis/FR-407-state-probabilistic-claims.md
  role: context_only
  excerpt: Under the event-position false-extension profile, an event `always holds(n.healthy)` with no interval
    refuses `unsupported_construct` at the temporal frontend (FR-048-AC-9) and never reaches the claim check; under
    the timed profile a bounded-window event `eventually[0 ms, 4 ms] holds(P)` is admitted; under the infinite-trace
    profile `[eventually holds(P)]` is admitted as a reachability form while `[eventually[0, 5] holds(P)]` and `[eventually[2,
    *] holds(P)]` refuse `invalid_probabilistic_model`/`unbounded-event` and `[eventually eventually holds(P)]`
    and `[eventually true]` refuse `not-state-
- id: FR-407-AC-16
  path: agent-ix/filament-specification@7aab94cf/spec/functional/analysis/FR-407-state-probabilistic-claims.md
  role: context_only
  excerpt: Under a bounded profile, `probability >= 0.99 [always holds(n.healthy)]` and `probability >= 0.99 [eventually[2,
    *] holds(n.healthy)]` each refuse `unsupported_construct` at the operator or interval span with the owning claim
    and selected profile, exactly as the same formula in a temporal clause under that profile refuses, and neither
    refuses `unbounded-event`; under the timed profile `eventually holds(P)` with no interval refuses `unsupported_construct`
    and `eventually[0 ms, 4 ms] holds(P)` is admitted; under the infinite-trace profile `[eventually[2, *] holds(P)]`
    refuses `invalid_probabilis
---

## Summary

Frozen review: agent-ix/filament-examples PR 71 at 8076ef95a1058d0030646b11213da973bf826b26, base 2518f065969678e816700aef1ebcd2c931c50f55. Tickets: QSL-2727 and QSL-2268. No branch edits, local builds, tests, full gate or new tickets. Runtime evidence is supplied by the lead from fresh driver a34862bf3cc3c6a04e6468d87f1155bd3fe86f8f, not independently rerun. No applicable AssuranceProfile exists for the changed examples. No Rust, React, spec or plan file changes, so language-specific and spec-review sub-analyses are inapplicable.

## Verdict

CONDITIONAL — manual diff inspection completed; the absent root spec/ prevents the required computed matrix and a full repository assurance verdict. Concrete changed gate defects are reported by code-review.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Plan completion: not assessed

The gap-analysis skill was loaded and its matrix, reverse-gap and stub procedures were attempted. `quire matrix --scope /home/peter/dev/worktrees/qsl-2727-examples-gate --format json` exits 3 because this examples repository has no root spec/ document tree. This is the documented two-roots diagnostic, not a missing skill or an engine failure. No provisional grep index can supply absent local requirements; no engine-backed repository PASS is claimed.

Manual, diff-scoped acceptance-to-fixture inspection only: every added behavior has a ticket owner and existing upstream semantics; the zero-weight case belongs to QSL-2268 and FR-185-AC-10, the closed-interval outcome to QSL-2727 and FSpec FR-407-AC-11/16, timeout clock selection to QSL-2727 and its existing protocol, and the two pending probability analyses to QSL-2727 with FR-187/FR-196 operation-determinacy context. No new test-to-AC binding is claimed; no tests or trace tags changed. New source and requests contain real declarations and paths, and all snapshot object keys/populations were inspected; no source or test stub, copied gate, inflated coverage, or unowned new behavior was identified in the diff. Root-wide requirement completeness and computed traceability remain unassessed because the spec root is absent.

The code-review artifact records two concrete delivery defects in the changed sampling/vote gate contracts; this artifact does not duplicate their findings. Optional semantic fan-out was not requested and was not run. No plan was supplied. Runtime acceptance remains bounded to the lead-supplied three green cases; the sampling measured/rejected and vote false-claim verdict oracles remain unpassed.

## Disposition round 1 coverage

Disposition round 1 reviewed frozen d80bf4f001bd24ca0e6f2e57e081b861ad0e1c18 against the original reviewed 8076ef95a1058d0030646b11213da973bf826b26 and base 2518f065969678e816700aef1ebcd2c931c50f55. Both findings are fixed by exclusion from this partial delivery: sampling/vote source, requests and manifest case blocks are restored byte-for-byte to base, the newly added start-available snapshot is removed, and their expected records are unchanged. Availability and exact reachability source/request/manifest/expected bytes also match base. No probability expected record, oracle, rate, threshold, weight, enabled operation or status was weakened.

The pending-probability remote branch was independently checked with git ls-remote: codex/qsl-2727-pending-probability resolves to the original reviewed 8076ef95a1058d0030646b11213da973bf826b26. Pending work is preserved, not accepted. The final nine product files retain the three previously evidenced changes; aside from the restored sampling manifest block, every retained product file is byte-identical to the prior reviewed head. All three committed original review artifacts match the prior scratch artifacts exactly. No branch writes, local build, tests, runtime execution or regeneration were performed for this disposition.

The PR is mergeable as a partial delivery of timeout clock selection, bounded-event unsupported/21 correction, and the zero-weight workload example, with no new finding. This does not clear the pre-existing probability gate failures, accept any probability oracle, or complete QSL-2727/QSL-2158. QSL-2268's released-driver wording remains unverified: supplied focused checks used fresh driver a34862bf3cc3c6a04e6468d87f1155bd3fe86f8f, not an independently verified released binary. Keep that ticket open until its release acceptance is established or explicitly resolved by the owner. The missing root spec/ still prevents a full engine-backed gap-analysis PASS; the manual diff review is complete.
