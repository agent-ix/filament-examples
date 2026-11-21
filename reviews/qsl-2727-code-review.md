---
id: SR-2026111919340478491681431
title: PR 71 code-review review
type: SpecReview
analysis: code-review
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

FAIL — the delivered sampling and vote analysis commands do not satisfy their retained staged contracts. No waiver is inferred from partial scope or pending runtime. QSL-2268 and the two non-probability authoring fixes have no scoped source defect.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The sampling command now analyzes the claim, but its staged record still requires an S4 unsupported/not-yet-implemented result. The supplied fresh-driver run reaches EN4 at S6c and refuses not-markov instead, so evaluate_case rejects the changed outcome even though exit 21 matches. Its S4 writer explanation and QSL-2000 blocker are also stale. Keep this pending analysis change outside the mergeable partial delivery, or resolve the authoring/engine cause and obtain the required measured/rejected oracle before recording a justified staged or running result. | examples/architecture/example.toml:466-472 |
| FND-002 | high | The vote command now performs exact analysis, but the staged manifest and record still assert exit 21 and an S4 unsupported/not-yet-implemented result. The supplied run remained active for 15 minutes and was cancelled, producing incomplete/22; it establishes neither that record nor the required false-claim verdict. The staged runner has no per-case timeout and waits for child completion, so this change can stall the staged gate. Keep this pending analysis change outside the partial delivery until a stable outcome and its required oracle are established; do not record cancellation as the intended result. | examples/rover/example.toml:816-822 |

## Coverage

The complete 15-file diff was inspected, including request path resolution, snapshot populations, writable frames and Boolean returns, random rates, workload weights, profiles, clocks, result normalization, expected structures, README and corpus index. No copied producer implementation, new helper, deleted gate, suppressed warning, or stub is introduced. The zero-weight case is an original workload example; the existing zero-weight random-distribution example is unchanged. Supplied focused regeneration/check evidence supports the bounded-event unsupported/21 record, timeout compile/0, and all-zero workload S4 compile/0. Index and architecture README count agree at 35 running and 7 staged. Global hardcoded count tests are pre-existing and were not run or changed.

Producer context: pinned FSL ab98ea12d8a4e66a4bce4ac0008703bb5cec2527 FR-185 Workloads and AC-10 admit zero and all-zero weights. FR-192 Description requires EN4 measured/rejected, never a refuted counterexample. FSpec FR-407-AC-11/16 require bounded-profile interval-less operators to refuse unsupported_construct, consistent with the changed expected record. The new posts specify every frame-modified field and Boolean result; source inspection does not identify the remaining EN4 refusal's precise cause. Diagnostics retains that triage.

Sampling still returned EN4/not-markov at S6c. Vote ended only after owned cancellation: incomplete/22, not a natural verdict or counterexample. Neither probability record/status was promoted. Availability and exact reachability source/request bytes remain unchanged from base, with product gaps on QSL-2158; they are not accepted or waived. PR title/body correctly describe partial delivery and leave QSL-2727 open. QSL-2268's scoped gate is evidenced on the fresh driver; the separate released-driver wording in its ticket is not independently established by this review. A partial merge is supportable after pending probability analysis edits are kept outside the delivery, or after both cases obtain justified evidence and consistent contracts.
