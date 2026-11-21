---
id: SR-2026111919340478491681437
title: PR 75 gap-analysis review
type: SpecReview
analysis: gap-analysis
scope: agent-ix/filament-examples; QSL-2234; PR 75 through narrow rebase review at
  cea30fc2d28ce56c042238a07be2f168c5eb2e9d
review_set: subset
examined_scope:
- id: README.md
  path: README.md
  role: examined
  excerpt: '| [rover](examples/rover) | Safety | 67 | 6 |'
- id: examples/rover/example.toml
  path: examples/rover/example.toml
  role: examined
  excerpt: '[[case]]

    id = "hybrid-systems-slow-enough"

    rule = "SlowEnough"

    statement = "The bounded braking model stays below speed 30 for 10 seconds, with
    a certified flowpipe proof."

    file = "rules/hybrid-systems/bounded-braking.fsl"

    command = "analyze requests/hybrid-systems/bounded-slow-enough.analyze.json"

    status = "running"

    expect_exit = 0

    expect_verdict = "holds"

    serves = "Advanced analyses reference (hybrid systems)"


    [[case]]

    id = "hybrid-systems-too-slow"

    rule = "TooSlow"

    statement = "The bounded braking model can start at speed zero, refuting a speed
    above 5 with a replayed counterexample."

    file'
- id: examples/rover/expected/hybrid-systems-slow-enough.json
  path: examples/rover/expected/hybrid-systems-slow-enough.json
  role: examined
  excerpt: "\"record\": {\n        \"backend\": \"fsl-hybrid-reference\",\n      \
    \  \"basis\": {\n          \"exhaustive\": [\n            \"time\"\n         \
    \ ],\n          \"horizon\": {\n            \"denominator\": \"1\",\n        \
    \    \"numerator\": \"10\"\n          },\n          \"jumps\": 1,\n          \"\
    method\": \"flowpipe-outward-rounded\",\n          \"type\": \"bounded-solver\"\
    \n        },\n        \"cause\": null,\n        \"certification\": \"certified\"\
    ,\n        \"result\": \"proved\",\n        \"settled_by\": \"result\"\n     \
    \ },\n      \"request\": {\n        \"candidates\": {\n          \"backends\"\
    : [\n            \"fsl-hybrid-reference\"\n          ],\n          "
- id: examples/rover/expected/hybrid-systems-too-slow.json
  path: examples/rover/expected/hybrid-systems-too-slow.json
  role: examined
  excerpt: "\"record\": {\n        \"backend\": \"fsl-hybrid-reference\",\n      \
    \  \"cause\": null,\n        \"result\": \"refuted\",\n        \"settled_by\"\
    : \"result\"\n      },\n      \"replay\": {\n        \"reproduced\": true,\n \
    \       \"target\": {\n          \"state_clause\": \"TooSlow\"\n        }\n  \
    \    },\n      \"request\": {\n        \"candidates\": {\n          \"backends\"\
    : [\n            \"fsl-hybrid-reference\"\n          ],\n          \"type\": \"\
    set\"\n        },\n        \"claim\": {\n          \"node\": {\n            \"\
    ordinal\": 0,\n            \"package\": \"example:rover/bounded-braking\",\n \
    \           \"path\": [\n              \"temporal\",\n             "
- id: examples/rover/requests/hybrid-systems/bounded-slow-enough.analyze.json
  path: examples/rover/requests/hybrid-systems/bounded-slow-enough.analyze.json
  role: examined
  excerpt: "{\n  \"format\": \"native-analyze/1\",\n  \"request\": {\n    \"models\"\
    : [],\n    \"program\": {\n      \"source\": {\n        \"file\": \"../../rules/hybrid-systems/bounded-braking.fsl\"\
    ,\n        \"authority\": \"example\",\n        \"identity\": \"example:rover/bounded-braking\"\
    ,\n        \"document\": \"boundedbraking\",\n        \"formal_revision\": 1\n\
    \      }\n    },\n    \"items\": [{\"claim\": \"SlowEnough\"}]\n  }\n}\n"
- id: examples/rover/requests/hybrid-systems/bounded-too-slow.analyze.json
  path: examples/rover/requests/hybrid-systems/bounded-too-slow.analyze.json
  role: examined
  excerpt: "{\n  \"format\": \"native-analyze/1\",\n  \"request\": {\n    \"models\"\
    : [],\n    \"program\": {\n      \"source\": {\n        \"file\": \"../../rules/hybrid-systems/bounded-braking.fsl\"\
    ,\n        \"authority\": \"example\",\n        \"identity\": \"example:rover/bounded-braking\"\
    ,\n        \"document\": \"boundedbraking\",\n        \"formal_revision\": 1\n\
    \      }\n    },\n    \"items\": [{\"claim\": \"TooSlow\"}]\n  }\n}\n"
- id: examples/rover/rules/hybrid-systems/bounded-braking.fsl
  path: examples/rover/rules/hybrid-systems/bounded-braking.fsl
  role: examined
  excerpt: "language \"ix:native\" edition \"1-draft\";\nprofile p = \"filament.temporal.infinite-trace/v1\"\
    ;\n\n// The analysis model tracks speed and deceleration without a timer state.\n\
    hybrid Braking using p {\n  mode Moving invariant { v >= 0 and v <= 20 and a >=\
    \ 1 and a <= 2 }\n    flow { v' = -a; a' = 0; }\n    transition to Stopped when\
    \ { v = 0 } reset { v = 0 };\n  mode Stopped invariant { v = 0 }\n    flow { v'\
    \ = 0; a' = 0; }\n}\n\n// Every allowed initial speed is below this bound.\ntemporal\
    \ SlowEnough using p over (h: Braking) clock \"model-time\" on origin {\n  always\
    \ [0, 10] holds(h.v < 30)\n}\n\n// An allowed initi"
- id: index.json
  path: index.json
  role: examined
  excerpt: "\"case\": \"hybrid-systems-slow-enough\",\n      \"status\": \"running\"\
    ,\n      \"rule\": \"SlowEnough\",\n      \"statement\": \"The bounded braking\
    \ model stays below speed 30 for 10 seconds, with a certified flowpipe proof.\"\
    ,\n      \"rule_file\": \"examples/rover/rules/hybrid-systems/bounded-braking.fsl\"\
    ,\n      \"data_files\": [],\n      \"request_file\": \"examples/rover/requests/hybrid-systems/bounded-slow-enough.analyze.json\"\
    ,\n      \"expected_file\": \"examples/rover/expected/hybrid-systems-slow-enough.json\"\
    ,\n      \"verdict\": \"holds\",\n      \"exit\": 0,\n      \"ready_exit\": null,\n\
    \      \"engine\": null,\n      \"clock\": nu"
- id: examples/rover/rules/hybrid-systems/braking.fsl
  path: examples/rover/rules/hybrid-systems/braking.fsl
  role: examined
  excerpt: "language \"ix:native\" edition \"1-draft\";\nprofile p = \"filament.temporal.infinite-trace/v1\"\
    ;\n\n// A braking vehicle: in mode Moving the speed v falls at a deceleration\
    \ a\n// between 1 and 2 while a timer t runs. At speed 0 it jumps to Stopped.\n\
    hybrid Braking using p {\n  mode Moving invariant { v >= 0 and v <= 20 and a >=\
    \ 1 and a <= 2 }\n    flow { v' = -a; a' = 0; t' = 1; }\n    transition to Stopped\
    \ when { v = 0 } reset { v = 0 };\n  mode Stopped invariant { v = 0 }\n    flow\
    \ { v' = 0; a' = 0; t' = 1; }\n}\n\n// The speed stays below 30 during the first\
    \ 10 seconds.\ntemporal SlowEnough using p over (h: B"
- id: examples/rover/requests/hybrid-systems/braking.json
  path: examples/rover/requests/hybrid-systems/braking.json
  role: examined
  excerpt: "{\n  \"format\": \"native-compile/1\",\n  \"request\": {\n    \"models\"\
    : [],\n    \"program\": {\n      \"source\": {\n        \"file\": \"../../rules/hybrid-systems/braking.fsl\"\
    ,\n        \"authority\": \"example\",\n        \"identity\": \"example:rover/braking\"\
    ,\n        \"document\": \"braking\",\n        \"formal_revision\": 1\n      }\n\
    \    }\n  }\n}\n"
- id: FR-1019-AC-1
  path: agent-ix/filament-spec-language@529282e9dc/spec/functional/FR-1019-ship-a-first-party-reference-hybrid-provider.md
  role: context_only
  excerpt: With the reference provider registered and no other hybrid provider, `SlowEnough`
    over `Braking` settles `proved`, `bounded-solver` with `horizon` 10, `jumps` 1
    and `exhaustive` `["time"]`, method `flowpipe-outward-rounded`, certification
    `certified` after FR-751 accepts its certificate, backend `fsl-hybrid-reference`.
- id: FR-1019-AC-2
  path: agent-ix/filament-spec-language@529282e9dc/spec/functional/FR-1019-ship-a-first-party-reference-hybrid-provider.md
  role: context_only
  excerpt: '`TooSlow` settles `refuted` with one `HybridCounterexample` that exact
    replay reproduces, whose trace is the start state `(Moving, time 0, v = 0, a =
    1)` with no jump; two runs of the same request give equal counterexamples.'
- id: FR-1019-AC-13
  path: agent-ix/filament-spec-language@529282e9dc/spec/functional/FR-1019-ship-a-first-party-reference-hybrid-provider.md
  role: context_only
  excerpt: '`examples/hybrid-braking/slow-enough.analyze.json` and `too-slow.analyze.json`,
    read through `fsl_command::command::analyze_with_providers` with `Engine::hybrid_reference()`
    as the one provider and no external provider configured, settle `proved`, `certified`
    and `refuted` respectively, each with backend `fsl-hybrid-reference`.'
- id: FSL-hybrid-braking-fixture
  path: agent-ix/filament-spec-language@529282e9dc/examples/hybrid-braking/braking.native
  role: examined
  excerpt: "// SPDX-License-Identifier: AGPL-3.0-only\n// Braking without `t`, with\
    \ one held and one violated claim (FR-1019)\nlanguage \"ix:native\" edition \"\
    1-draft\";\nprofile p = \"filament.temporal.infinite-trace/v1\";\nhybrid Braking\
    \ using p {\n  mode Moving invariant { v >= 0 and v <= 20 and a >= 1 and a <=\
    \ 2 }\n    flow { v' = -a; a' = 0; }\n    transition to Stopped when { v = 0 }\
    \ reset { v = 0 };\n  mode Stopped invariant { v = 0 }\n    flow { v' = 0; a'\
    \ = 0; }\n}\ntemporal SlowEnough using p over (h: Braking) clock \"model-time\"\
    \ on origin { always [0, 10] holds(h.v < 30) }\ntemporal TooSlow using p over\
    \ (h: Brak"
- id: QSL-2234-examples-acceptance
  path: QSL-2234 description
  role: examined
  excerpt: The existing `hybrid-systems-*` examples reach a verdict through `fsl analyze`
    once QSL-1976 lands (driver test in `filament-driver/fsl-cli/tests/frontend_cli.rs`).
- id: hybrid-systems-slow-enough
  path: examples/rover/example.toml
  role: examined
  excerpt: '[[case]]

    id = "hybrid-systems-slow-enough"

    rule = "SlowEnough"

    statement = "The bounded braking model stays below speed 30 for 10 seconds, with
    a certified flowpipe proof."

    file = "rules/hybrid-systems/bounded-braking.fsl"

    command = "analyze requests/hybrid-systems/bounded-slow-enough.analyze.json"

    status = "running"

    expect_exit = 0

    expect_verdict = "holds"

    serves = "Advanced analyses reference (hybrid systems)"'
- id: hybrid-systems-too-slow
  path: examples/rover/example.toml
  role: examined
  excerpt: '[[case]]

    id = "hybrid-systems-too-slow"

    rule = "TooSlow"

    statement = "The bounded braking model can start at speed zero, refuting a speed
    above 5 with a replayed counterexample."

    file = "rules/hybrid-systems/bounded-braking.fsl"

    command = "analyze requests/hybrid-systems/bounded-too-slow.analyze.json"

    status = "running"

    expect_exit = 10

    expect_verdict = "violated"

    serves = "Advanced analyses reference (hybrid systems)"'
- id: hybrid-systems-braking
  path: examples/rover/example.toml
  role: examined
  excerpt: '[[case]]

    id = "hybrid-systems-braking"

    rule = "Braking"

    statement = "A braking vehicle has a moving mode with a falling speed and a stopped
    mode."

    file = "rules/hybrid-systems/braking.fsl"

    command = "compile requests/hybrid-systems/braking.json"

    status = "running"

    expect_exit = 0

    expect_verdict = "compiled"

    serves = "Advanced analyses reference (hybrid systems)"'
- id: hybrid-systems-claim
  path: examples/rover/example.toml
  role: examined
  excerpt: '[[case]]

    id = "hybrid-systems-claim"

    rule = "SlowEnough"

    statement = "The braking vehicle''s speed stays below 30 during the first 10 seconds."

    file = "rules/hybrid-systems/braking.fsl"

    command = "compile requests/hybrid-systems/braking.json"

    status = "running"

    expect_exit = 0

    expect_verdict = "compiled"

    serves = "Advanced analyses reference (hybrid systems)"'
- id: Driver-FR-022-AC-11
  path: agent-ix/filament-driver@da01aa94cd4f65a8e9661f6af6d514556af4825e/spec/functional/FR-022-analyze-a-native-analyze-request.md
  role: context_only
  excerpt: 'With no `--provider-config`, `fsl analyze` over FSL''s `examples/hybrid-braking/slow-enough.analyze.json`
    with `--format json` writes one item settled `proved` with certification `certified`
    and backend `fsl-hybrid-reference`, category `success`, exit 0, and over `too-slow.analyze.json`
    one item settled `refuted` with backend `fsl-hybrid-reference`, category `violation`,
    exit 10; neither writes `HybridProviderAbsent`, and the document''s `registry`
    lists `fsl-hybrid-reference` and `en-1`. The process-provider half depends on
    FSL''s `Engine::hybrid_process` (FSL FR-750): with `--provider-config` s'
disposition_1_scope:
- id: README.md
  path: README.md
  role: examined
  excerpt: '| [rover](examples/rover) | Safety | 67 | 6 |'
- id: examples/rover/example.toml
  path: examples/rover/example.toml
  role: examined
  excerpt: '[[case]]

    id = "hybrid-systems-radio-below-capacity"

    rule = "BelowStorageCapacity"

    statement = "The radio''s harvest, transmit and standby cycle stays below storage
    capacity for 12 seconds, with a certified proof."

    file = "rules/hybrid-systems/radio-energy-cycle.fsl"

    command = "analyze requests/hybrid-systems/radio-below-capacity.analyze.json"

    status = "running"

    expect_exit = 0

    expect_verdict = "holds"

    serves = "Advanced analyses reference (hybrid systems)"


    [[case]]

    id = "hybrid-systems-radio-exceeds-six"

    rule = "ChargeNeverExceedsSix"

    statement = "Harvesting raises the radio''s charge above six'
- id: examples/rover/expected/hybrid-systems-radio-below-capacity.json
  path: examples/rover/expected/hybrid-systems-radio-below-capacity.json
  role: examined
  excerpt: "\"record\": {\n        \"backend\": \"fsl-hybrid-reference\",\n      \
    \  \"basis\": {\n          \"exhaustive\": [\n            \"time\"\n         \
    \ ],\n          \"horizon\": {\n            \"denominator\": \"1\",\n        \
    \    \"numerator\": \"12\"\n          },\n          \"jumps\": 6,\n          \"\
    method\": \"flowpipe-outward-rounded\",\n          \"type\": \"bounded-solver\"\
    \n        },\n        \"cause\": null,\n        \"certification\": \"certified\"\
    ,\n        \"result\": \"proved\",\n        \"settled_by\": \"result\"\n     \
    \ },\n      \"request\": {\n        \"candidates\": {\n          \"backends\"\
    : [\n            \"fsl-hybrid-reference\"\n          ],\n          "
- id: examples/rover/expected/hybrid-systems-radio-exceeds-six.json
  path: examples/rover/expected/hybrid-systems-radio-exceeds-six.json
  role: examined
  excerpt: "\"record\": {\n        \"backend\": \"fsl-hybrid-reference\",\n      \
    \  \"cause\": null,\n        \"result\": \"refuted\",\n        \"settled_by\"\
    : \"result\"\n      },\n      \"replay\": {\n        \"reproduced\": true,\n \
    \       \"target\": {\n          \"state_clause\": \"ChargeNeverExceedsSix\"\n\
    \        }\n      },\n      \"request\": {\n        \"candidates\": {\n      \
    \    \"backends\": [\n            \"fsl-hybrid-reference\"\n          ],\n   \
    \       \"type\": \"set\"\n        },\n        \"claim\": {\n          \"node\"\
    : {\n            \"ordinal\": 0,\n            \"package\": \"example:rover/radio-energy-cycle\"\
    ,\n            \"path\": [\n              \"tempora"
- id: examples/rover/requests/hybrid-systems/radio-below-capacity.analyze.json
  path: examples/rover/requests/hybrid-systems/radio-below-capacity.analyze.json
  role: examined
  excerpt: "{\n  \"format\": \"native-analyze/1\",\n  \"request\": {\n    \"models\"\
    : [],\n    \"program\": {\n      \"source\": {\n        \"file\": \"../../rules/hybrid-systems/radio-energy-cycle.fsl\"\
    ,\n        \"authority\": \"example\",\n        \"identity\": \"example:rover/radio-energy-cycle\"\
    ,\n        \"document\": \"radioenergycycle\",\n        \"formal_revision\": 1\n\
    \      }\n    },\n    \"items\": [{\"claim\": \"BelowStorageCapacity\"}]\n  }\n\
    }\n"
- id: examples/rover/requests/hybrid-systems/radio-exceeds-six.analyze.json
  path: examples/rover/requests/hybrid-systems/radio-exceeds-six.analyze.json
  role: examined
  excerpt: "{\n  \"format\": \"native-analyze/1\",\n  \"request\": {\n    \"models\"\
    : [],\n    \"program\": {\n      \"source\": {\n        \"file\": \"../../rules/hybrid-systems/radio-energy-cycle.fsl\"\
    ,\n        \"authority\": \"example\",\n        \"identity\": \"example:rover/radio-energy-cycle\"\
    ,\n        \"document\": \"radioenergycycle\",\n        \"formal_revision\": 1\n\
    \      }\n    },\n    \"items\": [{\"claim\": \"ChargeNeverExceedsSix\"}]\n  }\n\
    }\n"
- id: examples/rover/rules/hybrid-systems/radio-energy-cycle.fsl
  path: examples/rover/rules/hybrid-systems/radio-energy-cycle.fsl
  role: examined
  excerpt: "language \"ix:native\" edition \"1-draft\";\nprofile energy = \"filament.temporal.infinite-trace/v1\"\
    ;\n\n// A solar-powered radio repeats a six-second energy cycle. Harvesting adds\n\
    // three units per second; transmission consumes two; standby consumes one.\n\
    hybrid RadioEnergyCycle using energy {\n  initial { mode Harvest when { charge\
    \ = 2 }; }\n  mode Harvest invariant { charge >= 2 and charge <= 8 }\n    flow\
    \ { charge' = 3; }\n    transition to Transmit when { charge = 8 } reset { charge\
    \ = 8 };\n  mode Transmit invariant { charge >= 4 and charge <= 8 }\n    flow\
    \ { charge' = -2; }\n    transition to Standby w"
- id: index.json
  path: index.json
  role: examined
  excerpt: "\"case\": \"hybrid-systems-radio-below-capacity\",\n      \"status\":\
    \ \"running\",\n      \"rule\": \"BelowStorageCapacity\",\n      \"statement\"\
    : \"The radio's harvest, transmit and standby cycle stays below storage capacity\
    \ for 12 seconds, with a certified proof.\",\n      \"rule_file\": \"examples/rover/rules/hybrid-systems/radio-energy-cycle.fsl\"\
    ,\n      \"data_files\": [],\n      \"request_file\": \"examples/rover/requests/hybrid-systems/radio-below-capacity.analyze.json\"\
    ,\n      \"expected_file\": \"examples/rover/expected/hybrid-systems-radio-below-capacity.json\"\
    ,\n      \"verdict\": \"holds\",\n      \"exit\": 0,\n      \"ready"
- id: hybrid-systems-radio-below-capacity
  path: examples/rover/example.toml
  role: examined
  excerpt: '[[case]]

    id = "hybrid-systems-radio-below-capacity"

    rule = "BelowStorageCapacity"

    statement = "The radio''s harvest, transmit and standby cycle stays below storage
    capacity for 12 seconds, with a certified proof."

    file = "rules/hybrid-systems/radio-energy-cycle.fsl"

    command = "analyze requests/hybrid-systems/radio-below-capacity.analyze.json"

    status = "running"

    expect_exit = 0

    expect_verdict = "holds"

    serves = "Advanced analyses reference (hybrid systems)"'
- id: hybrid-systems-radio-exceeds-six
  path: examples/rover/example.toml
  role: examined
  excerpt: '[[case]]

    id = "hybrid-systems-radio-exceeds-six"

    rule = "ChargeNeverExceedsSix"

    statement = "Harvesting raises the radio''s charge above six after its initial
    state, producing a replayed counterexample."

    file = "rules/hybrid-systems/radio-energy-cycle.fsl"

    command = "analyze requests/hybrid-systems/radio-exceeds-six.analyze.json"

    status = "running"

    expect_exit = 10

    expect_verdict = "violated"

    serves = "Advanced analyses reference (hybrid systems)"'
- id: hybrid-systems-braking
  path: examples/rover/example.toml
  role: examined
  excerpt: '[[case]]

    id = "hybrid-systems-braking"

    rule = "Braking"

    statement = "A braking vehicle has a moving mode with a falling speed and a stopped
    mode."

    file = "rules/hybrid-systems/braking.fsl"

    command = "compile requests/hybrid-systems/braking.json"

    status = "running"

    expect_exit = 0

    expect_verdict = "compiled"

    serves = "Advanced analyses reference (hybrid systems)"'
- id: hybrid-systems-claim
  path: examples/rover/example.toml
  role: examined
  excerpt: '[[case]]

    id = "hybrid-systems-claim"

    rule = "SlowEnough"

    statement = "The braking vehicle''s speed stays below 30 during the first 10 seconds."

    file = "rules/hybrid-systems/braking.fsl"

    command = "compile requests/hybrid-systems/braking.json"

    status = "running"

    expect_exit = 0

    expect_verdict = "compiled"

    serves = "Advanced analyses reference (hybrid systems)"'
- id: RadioEnergyCycle-model
  path: examples/rover/rules/hybrid-systems/radio-energy-cycle.fsl
  role: examined
  excerpt: "hybrid RadioEnergyCycle using energy {\n  initial { mode Harvest when\
    \ { charge = 2 }; }\n  mode Harvest invariant { charge >= 2 and charge <= 8 }\n\
    \    flow { charge' = 3; }\n    transition to Transmit when { charge = 8 } reset\
    \ { charge = 8 };\n  mode Transmit invariant { charge >= 4 and charge <= 8 }\n\
    \    flow { charge' = -2; }\n    transition to Standby when { charge = 4 } reset\
    \ { charge = 4 };\n  mode Standby invariant { charge >= 2 and charge <= 4 }\n\
    \    flow { charge' = -1; }\n    transition to Harvest when { charge = 2 } reset\
    \ { charge = 2 };\n}"
rebase_review_scope:
- id: README.md
  path: README.md
  role: examined
  excerpt: '| [rover](examples/rover) | Safety | 67 | 6 |'
- id: index.json
  path: index.json
  role: examined
  excerpt: '"case": "hybrid-systems-radio-below-capacity"'
- id: index.json-radio-exceeds-six
  path: index.json
  role: examined
  excerpt: '"case": "hybrid-systems-radio-exceeds-six"'
---

## Summary

Latest narrow rebase review confirms unchanged semantic inputs and refreshed counts/index. The bounded manual acceptance check remains clean; full repository gap assurance remains unavailable without root spec scope.

## Verdict

CONDITIONAL — bounded manual acceptance remains clean at cea30fc2d28ce56c042238a07be2f168c5eb2e9d; no full repository gap assurance is claimed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Historical initial-review coverage follows. Current disposition-1 coverage is recorded later in this document.

Frozen candidate: agent-ix/filament-examples PR #75 at 4d18c551bfa6ada7d4b4a1a69667b0e08c3f08fc, base 49ff6cf5873d953c114681a22b35256ffc7aeeba. Ticket QSL-2234. All eight changed paths inspected. No source edits, build, test execution, solver invocation, fetch or push occurred. No applicable AssuranceProfile exists: this repository has no root spec/.

Three authored runtime files match c8189e1c8cfb0b73694173fc46ed6bb6092995b1 byte for byte; all four authored paths including the manifest match a685b84796d3879051dfec095dc5a7e8717d8464. Retained runtime stdout comes from that exact source, supplied under the preserved driver main DA01 cohort da01aa94cd4f65a8e9661f6af6d514556af4825e / CLI 0.1.4 / FSL 529282e9dc. This cohort identification is supplied evidence, not a fresh execution. Both retained stderr files are empty.

The native-analyze/1 requests have models: [], select exactly SlowEnough or TooSlow, resolve source relative to their own document and name no backend, provider, solver option, horizon override or budget override. Both claims use the source horizon 10. The finite Moving invariant bounds v to [0,20] and a to [1,2]; rates reduce v and preserve a. Stopped preserves v=0. There are allowed initial states, so SlowEnough is not an empty-domain success: its accepted interval certificate contains Moving and Stopped dwell boxes. TooSlow has a real permitted violating initial point, time 0 / Moving / v=0 / a=1. This example deliberately refutes immediately and does not demonstrate a later flow-induced violation.

Read retained qsl-2234-c8189e1-bounded-slow.stdout.json: success at S6c, one SlowEnough item, result proved, backend fsl-hybrid-reference, bounded-solver horizon 10 / jumps 1 / exhaustive [time] / flowpipe-outward-rounded, certification certified, certificate.accepted true. Read retained qsl-2234-c8189e1-bounded-too-slow.stdout.json: violation at S8, one TooSlow item, result refuted, native hybrid-counterexample with the initial point and no jumps, replay.reproduced true targeting TooSlow. All five structural fields projected by tools/fsl-examples/src/outcome.rs (category, last_stage, result, first_diagnostic, full items) equal each generated expected JSON exactly. Its existing evaluate_case compares complete projected structure and exit, so success alone cannot satisfy these records. Runner passes the manifest command plus --format json and does not inject a provider.

Retained a685 derived logs report regeneration 2 recorded / 0 not recorded, README/index 2 generated files, and focused check 2 passed / 0 failed (exits 0 and 10). These are retained results, not independent reruns. Patch SHA-256 independently matches f53f6e5aa4093661cba63c1dcba474ca63702d1acc516a23adb1e093b6a2ced2. Index is exactly base plus the two new cases (607 to 609 overall); README changes only rover running count 65 to 67, staged remains 6. The original timer-bearing braking.fsl, request, both compile manifest cases and their expected JSON are byte-identical to base. git diff --check passed. No new helper, gate, implementation, dependency, warning suppression, fake response or coverage threshold is introduced.

Producer context: FR-1019 Examples owns the exact timer-free Braking model and both predicates; AC-1, AC-2 and AC-13 explain expected outcomes. Driver FR-022-AC-11 at DA01 names the default proof/refutation route and its external-provider half. Those are contextual producer requirements, not locally owned traceability obligations discharged by this PR. This slice supplies only examples evidence. Configured pure-hybrid provider integration, timeout/cancellation acceptance and combined-engine support remain separate unfinished QSL-2234 work. Do not close the umbrella from this slice.

Rust and React lanes skipped because no Rust or React files changed; Python-specific checks do not apply to FSL/JSON/TOML authoring. spec-review and sub-analyses skipped because no spec/plan artifact changed. No aggregate, full corpus, all-target, local Cargo or new runtime check was run.

Reconciliation: bounded manual acceptance/source/request/retained-runtime comparison only. quire matrix --scope /home/peter/dev/worktrees/qsl-2234-hybrid-examples --format json exited 3 with no matrix: "member '--scope': no document root at '/home/peter/dev/worktrees/qsl-2234-hybrid-examples/spec'". Root spec/ and repository-local traceability configuration are absent. Installed tools are /home/peter/.npm-global/bin/quire 0.52.0 (engine 0.62.2), /home/peter/.npm-global/bin/quoin 0.36.8, spec-artifacts-process@v0.36.0. Missing spec scope is expected for this examples corpus, not evidence of untagged producer criteria. No grep fallback, fabricated spec root, matrix counts or local test-to-AC bindings were invented. Whole-repository criterion reconciliation and reverse-gap completeness are unassessed. Reverse-gap and stub inspection covered the changed slice only: two declarative native claims, two requests and two executable harness cases, owned by QSL-2234 examples acceptance and contextual producer FR-1019/driver FR-022. Neither new case is hollow; both structural oracles retain decisive proof/replay fields. No new runtime implementation is introduced. No engine-backed PASS, full audit completion or producer test coverage is asserted.

Plan completion: not assessed

Semantic review: the optional whole-repository semantic audit was not run; the requested bounded intent/oracle check is included above.

## Disposition 1 coverage

Current disposition-1 candidate: PR #75 exact bd95aba848158fadff21ba31380b5c53ade40b5c, fresh base e48d4355684aa97dc53c97d58440b66312a974bf. This section supersedes the initial head and runtime evidence for current acceptance; the initial finding and historical evidence below remain unchanged. All eight current changed paths were reviewed. No source edits, builds, runtime repeats, fetch, push, aggregate or full-corpus checks occurred.

FND-001 is fixed by removal: bounded-braking.fsl, its two requests and its two expected JSON consumers are absent; the old case IDs are absent from manifest/index. The original timer-bearing braking.fsl, its request, two compile manifest cases and both expected JSON files remain byte-identical to the fresh base. The independent RadioEnergyCycle has a three-node closed cycle (Harvest -> Transmit -> Standby -> Harvest), one charge variable, an explicit Harvest/charge=2 initial state, rates +3/-2/-1, and guards at 8/4/2. Each mode lasts exactly two seconds. This differs substantively from the producer-owned two-mode Braking/Stopped model with v/a, deceleration interval, absorbing stopped state, unconstrained initial mode and immediate refutation. It is not that fixture renamed or reformatted. Scoped pinned-FSL search for the radio model/claim/mode names in examples, fsl-analyze/tests and fsl-hybrid-reference/tests returned no matches; direct comparison with the pinned Braking source establishes the structural difference. The shared native hybrid syntax is language use, not copied fixture ownership.

BelowStorageCapacity requires charge < 9 throughout [0,12]; both complete six-second cycles stay in [2,8], and the explicit initial state establishes a nonempty behavior. ChargeNeverExceedsSix requires charge <= 6 throughout [0,12]. It starts true (charge 2) and becomes false during the first Harvest phase after time 4/3, before reaching Transmit. Neither claim is a hardcoded response or empty-domain oracle. Requests are native-analyze/1 with models: [], their own relative radio source file, exactly one corresponding claim, and no backend/provider/options/budget override. Original compiler coverage remains separate and intact.

New retained actual evidence is qsl-2234-79b31b8-radio-capacity.stdout.json and qsl-2234-79b31b8-radio-six.stdout.json, both with empty stderr. All three source/request files are byte-identical to actual execution source head 79b31b8b4ffc1becf6a548750c30292ec71f358a. Final manifest is byte-identical to 8fb254766c7f25b294c38589dda1f3038f5b66a1. Runtime cohort is supplied preserved main DA01 / FSL 529282e9dc, not a new reviewer execution. Raw archive independently verified: 10968 bytes, SHA-256 2d00ef3f6d308be245d006b4b46f7afad5641ca8911efb9551b59553d8dc8210. Old c818/a685 records are historical only and do not validate this radio model.

Actual capacity outcome: success, S6c, one BelowStorageCapacity item, backend fsl-hybrid-reference, proved/certified, bounded-solver horizon 12 / jumps 6 / exhaustive [time] / flowpipe-outward-rounded, accepted hybrid-interval-certificate with 198 steps. Actual six outcome: violation, S8, one ChargeNeverExceedsSix item, refuted, native hybrid-counterexample, replay.reproduced true targeting ChargeNeverExceedsSix. Trace starts Harvest/charge 2/time 0 with no jumps. Exact rational endpoint time is 12297829382473034411/9223372036854775808, and charge is 55340232221128654849/9223372036854775808. Independently checked charge = 2 + 3*time, time - 4/3 = 1/27670116110564327424 > 0, charge - 6 = 1/9223372036854775808 > 0, and time < 2. This is a later flow-induced violation, not initial-state failure.

Every expected JSON value equals the existing outcome.rs projection of the new raw stdout, including all item certificate, request and replay fields. Existing runner/evaluate_case code is unchanged; it compares full projected structure and exit, so success alone is insufficient. Retained final-manifest scoped logs show regeneration 2 recorded/0 not recorded, README/index 2 generated files, and check 2 passed/0 failed at exits 0 and 10. These are supplied focused test results, not reviewer reruns. Derived patch independently verified: 278901 bytes, SHA-256 51edb6d250ea2c1dfcf65b15c4408e1063c338d3d9b5da81740040b184ccf1b4. Derived archive independently verified: 13999 bytes, SHA-256 e9b77e58fac39a6578acfee80884917ae51e9c804c8428cc337c6a6a09e349ce.

Index equals fresh base plus exactly the two radio cases (608 to 610 overall); existing architecture entries are preserved and architecture files are unchanged. README changes only rover running count 65 to 67; staged count remains 6. git diff --check passed. No production code, gate, dependency, provider or test harness changed. All language and spec-method applicability decisions remain unchanged: no changed Rust/React/Python/spec/plan files.

Current bounded manual gap reconciliation remains CONDITIONAL: no root spec/, no local requirement/traceability scope, no engine-backed repository matrix PASS, and no full reverse-gap audit. Initial matrix exit-3 missing-root diagnostic remains applicable; it was not repeated. Both new declarative claims/requests and harness cases have the QSL-2234 examples slice as their owner. FSL FR-1019 and driver FR-022 supply contextual hybrid routing/certification/replay contracts; their producer tests and literal Braking-specific ACs are not discharged by this independent radio example. No additional bounded acceptance or stub gap found.

Plan completion: not assessed

No new findings in disposition 1. QSL-2234 remains open for configured-provider/driver acceptance, timeout/cancellation and combined-engine work; examples alone cannot close it. These two latest validated SR files require lead/coder custody before merge.

## Narrow rebase review coverage

Narrow post-rebase review at exact PR #75 head cea30fc2d28ce56c042238a07be2f168c5eb2e9d, base 49a2d2445fba24ffb6a0d70d82108dda431171b7. This is the latest reviewed source head. The original review and disposition-1 finding text, evidence and outcome remain preserved below; FND-001 remains fixed. No new or open findings.

Compared all six semantic inputs directly with independently reviewed bd95aba848158fadff21ba31380b5c53ade40b5c: radio native source, both requests, manifest and both expected JSON files are byte-identical. Both committed SR copies also matched the reviewer's prior disposition-1 scratch artifacts before this update. The brief's abbreviated custody object 55cd2 was not available locally; no fetch was attempted and the stronger direct comparison with reviewed bd95 plus prior reviewer artifacts supplies continuity. This review did not repeat unchanged model, oracle or solver analysis.

Fresh-base README differs only at rover running count 65 -> 67; staged count stays 6. Index equals fresh main plus exactly the two radio entries, 614 -> 616 overall. All pre-existing records, including current main protocol and architecture additions, are preserved. Existing production runner/generator files are unchanged between the prior base e48d4355684aa97dc53c97d58440b66312a974bf and new base; the only tools/fsl-examples change is tests/protocol_docs.rs. git diff --check passed.

Retained static generation qsl-2234-08d0310-readme.log reports 2 generated files. Independently verified returned archive: 1245 bytes, SHA-256 21e0eea7458d9f8dd840a569a21c51546b3195a6abfdd857133584a9ba9c8363. Independently verified derived patch: 2323 bytes, SHA-256 2f3e1962527ea76006d5fe74993e8e3f066fabbb0eb01de70224678a08d028bf. These are counts/index generation evidence only, not a fresh runtime run or focused behavioral check.

Because every semantic input and the production harness are unchanged, the independently inspected 79b radio actual proof/replay outputs and 8fb final-manifest focused 2-case check remain applicable. They are not claimed as executions at cea30fc. The capacity proof remains certified/accepted with horizon 12 and six jumps; the native counterexample still replays a later violation after a true initial state. No build, runtime, scoped check, matrix, full gate, fetch, source edit or push was performed for this narrow review.

One remote PR query returned head cea30fc2d28ce56c042238a07be2f168c5eb2e9d, base 49a2d2445fba24ffb6a0d70d82108dda431171b7, mergeable MERGEABLE and mergeStateStatus BLOCKED. The reviewed source has no review blocker, but this is not a claim that all GitHub gates are satisfied. Latest two SR artifacts still need custody on the branch before merge.

Code-review remains PASS. Gap review remains CONDITIONAL, bounded manual acceptance only: root spec/ is absent, and no engine-backed repository assurance or whole-repository traceability/reverse-gap PASS is claimed. No matrix rerun. Plan completion: not assessed. QSL-2234 stays open for remaining driver/provider acceptance and timeout/cancellation work; examples do not close the umbrella.
