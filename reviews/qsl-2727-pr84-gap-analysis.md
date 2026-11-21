---
id: SR-2026111919340478491681441
title: PR 84 gap-analysis of QSL-2727 sound corpus repair
type: SpecReview
analysis: gap-analysis
scope: agent-ix/filament-examples; QSL-2727; complete PR 84 diff against current main
review_set: subset
examined_scope:
- id: README.md
  path: README.md
  role: examined
  excerpt: '# Filament examples


    filament-examples is a set of example systems written in the Filament Specification
    Language (FSL). Each system has a domain model, rules, data, and the recorded
    result of every check.


    You can read a rule, run it against a passing and a failing case, and compare
    your result with the recorded one. Copy a system folder as the starting point
    for a model of your own.


    ## The three families


    - **Shop (first steps).** Values, rules about data, and rules about operations,
    in an online shop.

    - **Enterprise and distributed systems.** Rules over a service architecture, and
    a protoc'
- id: examples/architecture/README.md
  path: examples/architecture/README.md
  role: examined
  excerpt: '# Architecture


    The architecture example is a microservice estate in three tiers, with teams that
    own services, dependencies between services, and data stores of different classifications.
    Rules over this model keep ownership, allowed dependencies and personal-data boundaries
    checkable whenever the architecture changes.


    You can run each rule against a good architecture and against a broken variant
    of it. A rolling deploy plan is checked step by step as a trace.


    ## The model


    - `Service` has a tier (edge, core or data), an owning team, an ordered list of
    dependencies, the stores it reads, and'
- id: examples/architecture/example.toml
  path: examples/architecture/example.toml
  role: examined
  excerpt: '# Service architecture: rules over a microservice model

    name = "architecture"

    summary = "Services owned by teams, in tiers, with dependencies and data stores.
    Rules keep ownership, allowed dependencies and personal-data boundaries; a rolling
    deploy plan keeps enough instances serving."

    model = "model/architecture.semantic-ir.json"


    [[case]]

    id = "owned-ok"

    rule = "EveryServiceOwned"

    statement = "Every service has an owning team."

    file = "rules/architecture.fsl"

    command = "run requests/owned-ok.json"

    status = "running"

    expect_exit = 0

    expect_verdict = "holds"

    serves = "Tutorial: architecture ru'
- id: examples/architecture/expected/staged-availability-missed.json
  path: examples/architecture/expected/staged-availability-missed.json
  role: examined
  excerpt: "{\n  \"category\": \"unsupported\",\n  \"last_stage\": \"S6b\",\n  \"\
    result\": null,\n  \"first_diagnostic\": null,\n  \"items\": [\n    {\n      \"\
    category\": \"unsupported\",\n      \"claim_name\": \"CheckoutAvailable\",\n \
    \     \"record\": {\n        \"diagnostic\": {\n          \"cause\": \"not-yet-implemented\"\
    ,\n          \"code\": \"unsupported_construct\",\n          \"detail\": {}\n\
    \        },\n        \"result\": \"unsupported\",\n        \"settled_by\": \"\
    disposition\"\n      },\n      \"request\": {\n        \"candidates\": {\n   \
    \       \"backends\": [\n            \"en-5\"\n          ],\n          \"type\"\
    : \"set\"\n        },\n        \"claim\": {\n          \"node"
- id: examples/architecture/expected/staged-availability-missed.messages.txt
  path: examples/architecture/expected/staged-availability-missed.messages.txt
  role: examined
  excerpt: 'random parameter `call(ok)` cannot be written to a checked-package/v2
    graph yet

    '
- id: examples/architecture/expected/staged-availability.json
  path: examples/architecture/expected/staged-availability.json
  role: examined
  excerpt: "{\n  \"category\": \"success\",\n  \"last_stage\": \"S4\",\n  \"result\"\
    : null,\n  \"first_diagnostic\": null,\n  \"items\": []\n}\n"
- id: examples/architecture/expected/staged-availability.messages.txt
  path: examples/architecture/expected/staged-availability.messages.txt
  role: examined
  excerpt: 'random-parameter `call(ok)` cannot be written to a checked-package/v2
    graph yet

    '
- id: examples/architecture/expected/staged-sampling-claim-missed.json
  path: examples/architecture/expected/staged-sampling-claim-missed.json
  role: examined
  excerpt: "{\n  \"category\": \"unsupported\",\n  \"last_stage\": \"S4\",\n  \"result\"\
    : null,\n  \"first_diagnostic\": {\n    \"code\": \"unsupported_construct\",\n\
    \    \"cause\": \"not-yet-implemented\"\n  },\n  \"items\": []\n}\n"
- id: examples/architecture/expected/staged-sampling-claim-missed.messages.txt
  path: examples/architecture/expected/staged-sampling-claim-missed.messages.txt
  role: examined
  excerpt: 'random parameter `call(ok)` cannot be written to a checked-package/v2
    graph yet

    '
- id: examples/architecture/expected/staged-sampling-claim.json
  path: examples/architecture/expected/staged-sampling-claim.json
  role: examined
  excerpt: "{\n  \"category\": \"success\",\n  \"last_stage\": \"S4\",\n  \"result\"\
    : null,\n  \"first_diagnostic\": null,\n  \"items\": []\n}\n"
- id: examples/architecture/expected/staged-sampling-claim.messages.txt
  path: examples/architecture/expected/staged-sampling-claim.messages.txt
  role: examined
  excerpt: 'random-parameter `call(ok)` cannot be written to a checked-package/v2
    graph yet

    '
- id: examples/architecture/requests/probability/sampling/staged-sampling-claim-missed.json
  path: examples/architecture/requests/probability/sampling/staged-sampling-claim-missed.json
  role: examined
  excerpt: "{\n  \"format\": \"native-analyze/1\",\n  \"request\": {\n    \"models\"\
    : [\n      {\n        \"format\": \"semantic-ir/2.0.0\",\n        \"source\":\
    \ {\n          \"file\": \"../../../model/architecture.semantic-ir.json\",\n \
    \         \"authority\": \"example\",\n          \"identity\": \"example:architecture/model\"\
    ,\n          \"document\": \"model\",\n          \"formal_revision\": 1\n    \
    \    }\n      }\n    ],\n    \"program\": {\n      \"source\": {\n        \"file\"\
    : \"../../../rules/probability/sampling/staged-sampling-claim-missed.fsl\",\n\
    \        \"authority\": \"example\",\n        \"identity\": \"example:architecture/staged-sampling-claim-missed\"\
    ,\n "
- id: examples/architecture/requests/staged-availability-missed.json
  path: examples/architecture/requests/staged-availability-missed.json
  role: examined
  excerpt: "{\n  \"format\": \"native-analyze/1\",\n  \"request\": {\n    \"models\"\
    : [\n      {\n        \"format\": \"semantic-ir/2.0.0\",\n        \"source\":\
    \ {\n          \"file\": \"../model/architecture.semantic-ir.json\",\n       \
    \   \"authority\": \"example\",\n          \"identity\": \"example:architecture/model\"\
    ,\n          \"document\": \"model\",\n          \"formal_revision\": 1\n    \
    \    }\n      }\n    ],\n    \"program\": {\n      \"source\": {\n        \"file\"\
    : \"../rules/staged-availability-missed.fsl\",\n        \"authority\": \"example\"\
    ,\n        \"identity\": \"example:architecture/staged-availability-missed\",\n\
    \        \"document\": \"stagedavailabilit"
- id: examples/architecture/rules/probability/sampling/staged-sampling-claim-missed.fsl
  path: examples/architecture/rules/probability/sampling/staged-sampling-claim-missed.fsl
  role: examined
  excerpt: 'language "ix:native" edition "1-draft";

    profile v = "filament.model.complete/v1";

    profile ev = "filament.temporal.event-position.false-extension/v1";

    profile inf = "filament.temporal.infinite-trace/v1";

    model Arch = "example/architecture";


    // Each call to a service fails 1 time in 10,000, so each service stays up through
    10 calls with

    // probability about 0.999.

    random Arch::Service::call(ok) ~ { true: 9999, false: 1 };

    workload Traffic on Arch { weight Arch::Service::call = 1; }


    // A service is up after a call exactly when the call succeeded.

    post UpFollowsCall using v on Arch::Service::cal'
- id: examples/architecture/rules/probability/sampling/staged-sampling-claim.fsl
  path: examples/architecture/rules/probability/sampling/staged-sampling-claim.fsl
  role: examined
  excerpt: 'language "ix:native" edition "1-draft";

    profile v = "filament.model.complete/v1";

    profile ev = "filament.temporal.event-position.false-extension/v1";

    profile inf = "filament.temporal.infinite-trace/v1";

    model Arch = "example/architecture";


    // Each call to a service fails 1 time in 10,000, so each service stays up through
    10 calls with

    // probability about 0.999.

    random Arch::Service::call(ok) ~ { true: 9999, false: 1 };

    workload Traffic on Arch { weight Arch::Service::call = 1; }


    // A service is up after a call exactly when the call succeeded.

    post UpFollowsCall using v on Arch::Service::cal'
- id: examples/engines/example.toml
  path: examples/engines/example.toml
  role: examined
  excerpt: 'name = "engines"

    summary = "Analysis engines and CLI verbs: hybrid, timed, probability, schedulability,
    deadlock, simulation and the verbs a build does not route."


    [[case]]

    id = "verb-prove-not-routed"

    statement = "The prove verb is accepted but this build does not route it: the
    outcome names the verb under unsupported_operation and exits 21."

    file = "rules/hybrid.fsl"

    command = "prove requests/hybrid-at-most-eight.json"

    status = "running"

    expect_exit = 21

    expect_verdict = "unsupported: operation-not-routed"

    serves = "FR-522-AC-1, FR-301-AC-4 (unrouted verbs)"


    [[case]]

    id = "verb-generate-no'
- id: examples/engines/expected/deadlock-no-terminal-refuted.json
  path: examples/engines/expected/deadlock-no-terminal-refuted.json
  role: examined
  excerpt: "{\n  \"category\": \"violation\",\n  \"last_stage\": \"S8\",\n  \"result\"\
    : null,\n  \"first_diagnostic\": null,\n  \"items\": [\n    {\n      \"axes\"\
    : {\n        \"activation\": \"active\",\n        \"execution\": \"completed\"\
    ,\n        \"settlement\": \"decisive-counterexample\",\n        \"truth\": \"\
    false\"\n      },\n      \"category\": \"violation\",\n      \"claim_name\": \"\
    K\",\n      \"counterexample\": {\n        \"fairness\": [],\n        \"initial_state\"\
    : 0,\n        \"interval\": null,\n        \"kind\": \"deadlock\",\n        \"\
    loop\": [],\n        \"over_binding\": null,\n        \"prefix\": [\n        \
    \  {\n            \"post_state\": {\n              \"key\": "
- id: examples/engines/expected/deadlock-terminal-misses-state.json
  path: examples/engines/expected/deadlock-terminal-misses-state.json
  role: examined
  excerpt: "{\n  \"category\": \"violation\",\n  \"last_stage\": \"S8\",\n  \"result\"\
    : null,\n  \"first_diagnostic\": null,\n  \"items\": [\n    {\n      \"axes\"\
    : {\n        \"activation\": \"active\",\n        \"execution\": \"completed\"\
    ,\n        \"settlement\": \"decisive-counterexample\",\n        \"truth\": \"\
    false\"\n      },\n      \"category\": \"violation\",\n      \"claim_name\": \"\
    K\",\n      \"counterexample\": {\n        \"fairness\": [],\n        \"initial_state\"\
    : 0,\n        \"interval\": null,\n        \"kind\": \"deadlock\",\n        \"\
    loop\": [],\n        \"over_binding\": null,\n        \"prefix\": [\n        \
    \  {\n            \"post_state\": {\n              \"key\": "
- id: examples/grammar/example.toml
  path: examples/grammar/example.toml
  role: examined
  excerpt: 'name = "grammar"

    summary = "Source grammar: lexical boundaries, expressions, declarations, claims,
    and syntax refusals."


    [[case]]

    id = "header-only"

    statement = "A source containing only its header parses; package completeness
    is checked later."

    file = "rules/header-only.fsl"

    command = "parse example grammar/header-only rules/header-only.fsl"

    status = "running"

    expect_exit = 0

    expect_verdict = "parses"


    [[case]]

    id = "unit-no-declarations"

    statement = "A source containing a header and profile parses without a declaration."

    file = "rules/unit-no-declarations.fsl"

    command = "parse example gramm'
- id: examples/grammar/expected/synthesis-objective.json
  path: examples/grammar/expected/synthesis-objective.json
  role: examined
  excerpt: "{\n  \"category\": \"success\",\n  \"last_stage\": \"S2\",\n  \"result\"\
    : {\n    \"bytes\": \"222\",\n    \"kind\": \"parsed\",\n    \"path\": \"rules/synthesis-objective.fsl\"\
    ,\n    \"source\": {\n      \"authority\": \"example\",\n      \"identity\": \"\
    grammar/synthesis-objective\"\n    },\n    \"usage\": [\n      {\n        \"bound\"\
    : \"1048576\",\n        \"setting\": \"s1.input_bytes\",\n        \"used\": \"\
    222\"\n      },\n      {\n        \"bound\": \"100000\",\n        \"setting\"\
    : \"s1.tokens\",\n        \"used\": \"52\"\n      },\n      {\n        \"bound\"\
    : \"50000\",\n        \"setting\": \"s1.nodes\",\n        \"used\": \"39\"\n \
    \     },\n      {\n        \"bound\": \"288\",\n      "
- id: examples/grammar/expected/synthesis-objective.messages.txt
  path: examples/grammar/expected/synthesis-objective.messages.txt
  role: examined
  excerpt: 'expected `satisfies`

    '
- id: examples/lock/example.toml
  path: examples/lock/example.toml
  role: examined
  excerpt: '# Two processes and a lock: fairness and deadlock

    name = "lock"

    summary = "Two processes take turns on a lock. Which process is served depends
    on fairness, and two locks taken in opposite orders can deadlock."

    model = "model/lock.semantic-ir.json"


    [[case]]

    id = "fairness-without-fairness"

    rule = "ServedWithoutFairness"

    statement = "Process 1 holds the lock again and again."

    file = "rules/fairness/one-served.fsl"

    command = "compile requests/fairness/one-served.json"

    status = "running"

    expect_exit = 0

    expect_verdict = "compiled"

    serves = "Model checking reference (fairness)"


    [[case]]

    id = "fai'
- id: examples/lock/expected/staged-two-locks-deadlock.json
  path: examples/lock/expected/staged-two-locks-deadlock.json
  role: examined
  excerpt: "{\n  \"category\": \"unsupported\",\n  \"last_stage\": \"S8\",\n  \"result\"\
    : null,\n  \"first_diagnostic\": null,\n  \"items\": [\n    {\n      \"category\"\
    : \"unsupported\",\n      \"claim_name\": \"FirstLockIsFree\",\n      \"record\"\
    : {\n        \"diagnostic\": {\n          \"cause\": \"unbounded-extent\",\n \
    \         \"code\": \"unsupported_projection\",\n          \"detail\": {\n   \
    \         \"candidate\": \"contract-checker\",\n            \"kind\": \"operation-contract\"\
    ,\n            \"modes\": [\n              \"bounded\"\n            ]\n      \
    \    }\n        },\n        \"result\": \"unsupported\",\n        \"settled_by\"\
    : \"disposition\"\n      },\n      \"request"
- id: examples/lock/expected/two-locks-ordered.json
  path: examples/lock/expected/two-locks-ordered.json
  role: examined
  excerpt: "{\n  \"category\": \"unsupported\",\n  \"last_stage\": \"S6c\",\n  \"\
    result\": null,\n  \"first_diagnostic\": null,\n  \"items\": [\n    {\n      \"\
    category\": \"unsupported\",\n      \"claim_name\": \"FirstLockIsFree\",\n   \
    \   \"record\": {\n        \"diagnostic\": {\n          \"cause\": \"unbounded-extent\"\
    ,\n          \"code\": \"unsupported_projection\",\n          \"detail\": {\n\
    \            \"candidate\": \"contract-checker\",\n            \"kind\": \"operation-contract\"\
    ,\n            \"modes\": [\n              \"bounded\"\n            ]\n      \
    \    }\n        },\n        \"result\": \"unsupported\",\n        \"settled_by\"\
    : \"disposition\"\n      },\n      \"reques"
- id: examples/parts-saga/example.toml
  path: examples/parts-saga/example.toml
  role: examined
  excerpt: '# Parts-processing saga across services

    name = "parts-saga"

    summary = "A work order releases a part; machining, inspection and assembly stations
    reserve, process and hand it on. Inspection passes it, sends it to rework or scraps
    it; scrapping releases the machine upstream."

    model = "model/parts-saga.semantic-ir.json"


    [[case]]

    id = "jobs-rework-tracked"

    rule = "ReworkedPartsAreTracked"

    statement = "A scrapped part never reaches assembly; an assembled part was never
    scrapped."

    file = "rules/jobs.fsl"

    command = "run requests/jobs-rework-tracked.json"

    status = "running"

    expect_exit = 0

    expect_ver'
- id: examples/parts-saga/expected/staged-saga-deadlock-free.json
  path: examples/parts-saga/expected/staged-saga-deadlock-free.json
  role: examined
  excerpt: "{\n  \"category\": \"unsupported\",\n  \"last_stage\": \"S6b\",\n  \"\
    result\": null,\n  \"first_diagnostic\": null,\n  \"items\": [\n    {\n      \"\
    axes\": {\n        \"activation\": \"unavailable\",\n        \"execution\": null,\n\
    \        \"settlement\": \"unavailable\",\n        \"truth\": null\n      },\n\
    \      \"category\": \"unsupported\",\n      \"claim_name\": \"PartFlow\",\n \
    \     \"record\": {\n        \"diagnostic\": {\n          \"cause\": \"unbounded-extent\"\
    ,\n          \"code\": \"unsupported_projection\",\n          \"detail\": {\n\
    \            \"candidate\": \"en-1\",\n            \"kind\": \"temporal-satisfaction\"\
    ,\n            \"modes\": [\n              \"bound"
- id: examples/parts-saga/requests/analyze/saga-deadlock-free.json
  path: examples/parts-saga/requests/analyze/saga-deadlock-free.json
  role: examined
  excerpt: "{\n  \"format\": \"native-analyze/1\",\n  \"request\": {\n    \"models\"\
    : [\n      {\n        \"format\": \"semantic-ir/2.0.0\",\n        \"source\":\
    \ {\n          \"file\": \"../../model/parts-saga.semantic-ir.json\",\n      \
    \    \"authority\": \"example\",\n          \"identity\": \"example:parts-saga/model\"\
    ,\n          \"document\": \"model\",\n          \"formal_revision\": 1\n    \
    \    }\n      }\n    ],\n    \"program\": {\n      \"source\": {\n        \"file\"\
    : \"../../rules/part-flow-untimed.fsl\",\n        \"authority\": \"example\",\n\
    \        \"identity\": \"example:parts-saga/part-flow-untimed\",\n        \"document\"\
    : \"partflowuntimed\",\n        \"formal_r"
- id: examples/rover/example.toml
  path: examples/rover/example.toml
  role: examined
  excerpt: '# Delivery rover: failsafe modes, sensor voting and safe stop

    name = "rover"

    summary = "A delivery rover drives on a lidar and three range sensors voted two-out-of-three.
    Without its lidar it slows down and must stop; in failsafe mode it stays still."

    model = "model/rover.semantic-ir.json"


    # Boolean arguments are passed as 1 (true) and 0 (false).

    [[case]]

    id = "vote-two-of-three"

    rule = "twoOfThree"

    statement = "The vote is true when at least two of its three inputs are true."

    file = "rules/modes.fsl"

    command = "run requests/vote-two-of-three.json"

    status = "running"

    expect_exit = 0

    expect_ve'
- id: examples/rover/expected/staged-vote-probability-missed.json
  path: examples/rover/expected/staged-vote-probability-missed.json
  role: examined
  excerpt: "{\n  \"category\": \"unsupported\",\n  \"last_stage\": \"S6c\",\n  \"\
    result\": null,\n  \"first_diagnostic\": null,\n  \"items\": [\n    {\n      \"\
    category\": \"unsupported\",\n      \"claim_name\": \"VoteAlmostNeverWrong\",\n\
    \      \"record\": {\n        \"backend\": \"en-5\",\n        \"cause\": {\n \
    \         \"kind\": \"not-markov\"\n        },\n        \"result\": \"unsupported\"\
    ,\n        \"settled_by\": \"result\"\n      },\n      \"request\": {\n      \
    \  \"candidates\": {\n          \"backends\": [\n            \"en-5\"\n      \
    \    ],\n          \"type\": \"set\"\n        },\n        \"claim\": {\n     \
    \     \"node\": {\n            \"ordinal\": 0,\n            \"package\": \"ex"
- id: examples/rover/expected/staged-vote-probability-missed.messages.txt
  path: examples/rover/expected/staged-vote-probability-missed.messages.txt
  role: examined
  excerpt: 'random parameter `sample(faultA)` cannot be written to a checked-package/v2
    graph yet

    '
- id: examples/rover/expected/staged-vote-probability.json
  path: examples/rover/expected/staged-vote-probability.json
  role: examined
  excerpt: "{\n  \"category\": \"success\",\n  \"last_stage\": \"S4\",\n  \"result\"\
    : null,\n  \"first_diagnostic\": null,\n  \"items\": []\n}\n"
- id: examples/rover/expected/staged-vote-probability.messages.txt
  path: examples/rover/expected/staged-vote-probability.messages.txt
  role: examined
  excerpt: 'random-parameter `sample(faultA)` cannot be written to a checked-package/v2
    graph yet

    '
- id: examples/rover/requests/staged-vote-probability-missed.json
  path: examples/rover/requests/staged-vote-probability-missed.json
  role: examined
  excerpt: "{\n  \"format\": \"native-analyze/1\",\n  \"request\": {\n    \"models\"\
    : [\n      {\n        \"format\": \"semantic-ir/2.0.0\",\n        \"source\":\
    \ {\n          \"file\": \"../model/rover.semantic-ir.json\",\n          \"authority\"\
    : \"example\",\n          \"identity\": \"example:rover/model\",\n          \"\
    document\": \"model\",\n          \"formal_revision\": 1\n        }\n      }\n\
    \    ],\n    \"program\": {\n      \"source\": {\n        \"file\": \"../rules/staged-vote-probability-missed.fsl\"\
    ,\n        \"authority\": \"example\",\n        \"identity\": \"example:rover/staged-vote-probability-missed\"\
    ,\n        \"document\": \"stagedvoteprobabilitymissed\",\n"
- id: examples/run-inputs/example.toml
  path: examples/run-inputs/example.toml
  role: examined
  excerpt: 'name = "run-inputs"

    summary = "Run inputs and results: call arguments in their value forms, refusals
    of bad arguments, zero-declaration units, run options and limits, and verification
    plan runs."


    [[case]]

    id = "call-zero-arguments"

    statement = "LEGACY WIRE: A function with no parameter runs with no argument and
    completes with 5. The result is recorded in the driver''s legacy untagged form
    {\"integer\": \"5\"}, not the FR-181 typed form, and is kept as the single legacy
    case."

    file = "rules/calls.fsl"

    command = "run requests/call-zero-arguments.json"

    status = "running"

    expect_exit = 0

    expect_ve'
- id: examples/run-inputs/expected/call-three-items-complete.messages.txt
  path: examples/run-inputs/expected/call-three-items-complete.messages.txt
  role: examined
  excerpt: 'invalid clause request member items: items is handled by the multi-selection
    runner

    '
- id: examples/run-inputs/requests/call-three-items-complete.json
  path: examples/run-inputs/requests/call-three-items-complete.json
  role: examined
  excerpt: "{\n  \"format\": \"native-run/1\",\n  \"request\": {\n    \"models\":\
    \ [],\n    \"program\": {\n      \"source\": {\n        \"file\": \"../rules/calls.fsl\"\
    ,\n        \"authority\": \"example\",\n        \"identity\": \"example:run-inputs/calls\"\
    ,\n        \"document\": \"calls\",\n        \"formal_revision\": 1\n      }\n\
    \    },\n    \"items\": {\n      \"selections\": [\n        {\n          \"call\"\
    : {\n            \"function\": \"IdInt\",\n            \"arguments\": [\n    \
    \          {\n                \"parameter\": \"a\",\n                \"value\"\
    : {\n                  \"type\": \"integer\",\n                  \"value\": \"\
    3\"\n                }\n              }\n     "
- id: examples/shop/example.toml
  path: examples/shop/example.toml
  role: examined
  excerpt: '# Shop: first steps

    name = "shop"

    summary = "An online shop takes orders, charges payments and issues refunds. Values,
    rules about data, rules about operations."

    model = "model/shop.semantic-ir.json"


    [[case]]

    id = "claim-function-holds"

    rule = "yes"

    statement = "A Boolean function selected as a claim is tested against the current
    snapshot."

    file = "rules/claim-functions.fsl"

    command = "run requests/claim-function-holds.json"

    status = "running"

    expect_exit = 0

    expect_verdict = "holds"

    serves = "Function claims"


    [[case]]

    id = "claim-function-false"

    rule = "no"

    statement = "A Boolean function r'
- id: examples/shop/expected/expr-guard-two-names.json
  path: examples/shop/expected/expr-guard-two-names.json
  role: examined
  excerpt: "{\n  \"category\": \"success\",\n  \"last_stage\": \"S4\",\n  \"result\"\
    : null,\n  \"first_diagnostic\": null,\n  \"items\": []\n}\n"
- id: examples/shop/expected/expr-guard-two-names.messages.txt
  path: examples/shop/expected/expr-guard-two-names.messages.txt
  role: examined
  excerpt: 'the checker refused the package: undefined_expression (unproved-range)

    '
- id: examples/shop/expected/limits-observation-document-bytes.json
  path: examples/shop/expected/limits-observation-document-bytes.json
  role: examined
  excerpt: "{\n  \"category\": \"incomplete\",\n  \"last_stage\": \"S4\",\n  \"result\"\
    : null,\n  \"first_diagnostic\": null,\n  \"items\": [\n    {\n      \"axes\"\
    : {\n        \"activation\": \"unavailable\",\n        \"execution\": null,\n\
    \        \"settlement\": \"unavailable\",\n        \"truth\": null\n      },\n\
    \      \"category\": \"incomplete\",\n      \"cause\": {\n        \"bound\": \"\
    10\",\n        \"code\": \"stage_limit_exceeded\",\n        \"counter\": \"2047\"\
    ,\n        \"kind\": \"input-bytes-exceeded\",\n        \"limit\": \"input-bytes-exceeded\"\
    ,\n        \"locus\": {\n          \"document\": {\n            \"authority\"\
    : \"example\",\n            \"identity\": \"ix://exampl"
- id: examples/shop/expected/limits-observation-document-bytes.messages.txt
  path: examples/shop/expected/limits-observation-document-bytes.messages.txt
  role: examined
  excerpt: 'observation.document_bytes limit 10 exceeded; raise --limit observation.document_bytes

    '
- id: examples/shop/expected/staged-optional-parameter.json
  path: examples/shop/expected/staged-optional-parameter.json
  role: examined
  excerpt: "{\n  \"category\": \"success\",\n  \"last_stage\": \"S4\",\n  \"result\"\
    : null,\n  \"first_diagnostic\": null,\n  \"items\": []\n}\n"
- id: examples/shop/expected/staged-optional-parameter.messages.txt
  path: examples/shop/expected/staged-optional-parameter.messages.txt
  role: examined
  excerpt: 'the checker refused the package: ill_typed (type-mismatch)

    '
- id: examples/shop/requests/limits/observation-document-bytes.json
  path: examples/shop/requests/limits/observation-document-bytes.json
  role: examined
  excerpt: "{\n  \"format\": \"native-run/1\",\n  \"request\": {\n    \"models\":\
    \ [\n      {\n        \"format\": \"semantic-ir/2.0.0\",\n        \"source\":\
    \ {\n          \"file\": \"../../model/shop.semantic-ir.json\",\n          \"\
    authority\": \"example\",\n          \"identity\": \"example:shop/model\",\n \
    \         \"document\": \"model\",\n          \"formal_revision\": 1\n       \
    \ }\n      }\n    ],\n    \"program\": {\n      \"source\": {\n        \"file\"\
    : \"../../rules/paid-in-full.fsl\",\n        \"authority\": \"example\",\n   \
    \     \"identity\": \"example:shop/paid-in-full\",\n        \"document\": \"paid-in-full\"\
    ,\n        \"formal_revision\": 1\n      }\n    },\n    \"cla"
- id: index.json
  path: index.json
  role: examined
  excerpt: "{\n  \"format\": \"filament-examples-index/1\",\n  \"cases\": [\n    {\n\
    \      \"system\": \"architecture\",\n      \"case\": \"owned-ok\",\n      \"\
    status\": \"running\",\n      \"rule\": \"EveryServiceOwned\",\n      \"statement\"\
    : \"Every service has an owning team.\",\n      \"rule_file\": \"examples/architecture/rules/architecture.fsl\"\
    ,\n      \"data_files\": [\n        \"examples/architecture/data/good-architecture.json\"\
    \n      ],\n      \"request_file\": \"examples/architecture/requests/owned-ok.json\"\
    ,\n      \"expected_file\": \"examples/architecture/expected/owned-ok.json\",\n\
    \      \"verdict\": \"holds\",\n      \"exit\": 0,\n      \"ready_exit\": "
- id: config.mk
  path: config.mk
  role: context_only
  excerpt: '# FSL names the FSL command-line driver, the `fsl` binary. Override it
    on the command line to use another

    # build: make check FSL=/path/to/driver

    FSL ?= fsl

    export FSL


    # FSL_EXAMPLES runs the example runner (tools/fsl-examples). The default builds
    and runs it with cargo;

    # set it to an installed fsl-examples binary to skip the build, as the toolchain
    image does.

    EXAMPLES_ROOT := $(patsubst %/,%,$(dir $(abspath $(lastword $(MAKEFILE_LIST)))))

    FSL_EXAMPLES ?= cargo run --quiet --locked --manifest-path $(EXAMPLES_ROOT)/tools/fsl-examples/Cargo.toml
    --

    '
- id: tools/fsl-examples/src/outcome.rs
  path: tools/fsl-examples/src/outcome.rs
  role: context_only
  excerpt: '//! The structure of a `filament-outcome/1` document, and the comparison
    of a run with its record.

    //!

    //! A case is recorded in two files. `expected/<id>.json` holds the structure:
    the category, the

    //! result value, the last stage, the code and cause of the first diagnostic,
    and the items with

    //! their witness fields. `expected/<id>.messages.txt` holds the diagnostic message
    text. The gate

    //! compares the structure only, so rewording a message never fails a case.


    use crate::manifest::{Case, Status};

    use serde_json::{Map, Value, json};


    /// The format name of the driver''s outcome documents'
- id: examples/run-inputs/rules/calls.fsl
  path: examples/run-inputs/rules/calls.fsl
  role: context_only
  excerpt: 'language "ix:native" edition "1-draft";

    profile v = "filament.value.complete/v1";


    // FR-550: functions a run request calls with supplied arguments and reads a completed
    value from.

    function Five using v(): Integer pure { 5 }

    function Add using v(a: Integer, b: Integer): Integer pure { a + b }

    function IdInt using v(a: Integer): Integer pure { a }

    function IdBool using v(a: Boolean): Boolean pure { a }

    // A digit is an integer from 0 to 9 inclusive.

    function Digit using v(a: Int[0, 9]): Int[0, 9] pure { a }

    // A sequence holds at most three integers.

    function IdSeq using v(a: Sequence<Integer>'
- id: examples/parts-saga/rules/part-flow-untimed.fsl
  path: examples/parts-saga/rules/part-flow-untimed.fsl
  role: context_only
  excerpt: 'language "ix:native" edition "1-draft";

    profile v = "filament.model.complete/v1";

    profile p = "filament.protocol.finite-global/v1";

    model Parts = "example/parts";


    // A machine is reserved only when it is free.

    pre MachineFree using v on Parts::MachiningStation::reserve { not self.reserved
    }


    // Releasing a machine leaves it free and idle.

    post MachineReleased using v on Parts::MachiningStation::release { not self.reserved
    and not self.busy }


    // A work order releases a part to machining; machining reserves a machine,

    // processes the part and hands it to inspection. Inspection passes the part'
---

## Summary

Frozen independent review of agent-ix/filament-examples PR 84 at 8a7ce1e9b32ff24a9b2c9bd271f13e7b24ef1f7a against main 10c937d71625ec11c561a18bfc5ac1ded4ef1187. Ticket: QSL-2727. All 45 changed paths were inspected. Reviewer writes only these scratch artifacts; no source changes, runtime, builds, Cargo, tests, new measurements, baselines or whole gate.

## Verdict

CONDITIONAL — manual scoped inspection is clean; absent root spec/ prevents a computed traceability/repository assurance PASS. This limitation is pre-existing and does not block the bounded corpus repair.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Inspection

Three probability positive cases explicitly promise compilation, and expected records are success/S4 with no proof items. Negative availability and vote use native-analyze/1, explicit claim analysis items and existing initial snapshots; their records truthfully preserve unsupported/not-yet-implemented and unsupported/not-markov. Negative sampling is absent from manifest/index and its obsolete expected JSON/messages are removed. The real request/source remains unlisted with a README line explaining the absent reproducible unforced outcome. Forced cancellation is not stored as an expected passing verdict. Both sampling programs add only UpFollowsCall; weights, thresholds, workload, alpha, beta and indifference remain unchanged.

Both lock records retain all seven items: six unsupported operation-contract records (FirstLockIsFree, FirstLockTaken, SecondLockIsFree, SecondLockTaken, HoldsBoth, BothReleased) with unsupported_projection/unbounded-extent, plus the derived deadlock item. The bad-order item is refuted with native counterexample and replay.reproduced=true; ordered is proved with accepted closure certificate. Aggregate unsupported/21 is honest and the requests were not narrowed. The two engine cases retain K request/source bytes from main and record violation/refuted, native deadlock counterexamples and replay.reproduced=true.

The saga request binds PartFlow over jobs/j1 with all four model-binding roles. The returned request reports bounded extent with empty bounds; only the diagnostic says unbounded-extent. Manifest wording describes protocol analysis without falsely declaring the source extent unbounded. Shop's wire limit is document_bytes=10; the internal outcome setting observation.input_bytes is preserved with counter 2047 and incomplete/stage_limit_exceeded/input-bytes-exceeded. The guarded subtraction source supports its corrected statement. Grammar parse and optional compile promotions match their success records.

The multi-call request uses a closed items object containing selections, with IdInt(a: Integer) calls using canonical tagged 3, 5, 7 in order, matching calls.fsl. Its staged exit 20/ready 0 remains explicit pending QSL-2760; it does not claim old CLI adoption. The existing runner compares exact projected structures and exit codes, so descriptive expect_verdict strings do not bypass item comparisons.

## Coverage

Plan completion: not assessed

Manual diff-scoped coverage only, as authorized. This examples repository has no root spec/ and no applicable root AssuranceProfile. `quire matrix --scope /home/peter/dev/filament-examples-qsl2727 --format json` exited 3: `member '--scope': no document root at '/home/peter/dev/filament-examples-qsl2727/spec'`. Nested tutorial spec trees are fixtures, not a substitute root specification. No engine-backed coverage or repository-wide PASS is claimed; no invented local requirements or grep coverage count. Optional semantic fan-out was not requested and was not run. Root-wide reverse-gap completeness is outside this bounded delivery review.

The 15 selected case behaviors were manually inventoried against their manifests, requests, source and expected records: grammar/synthesis-objective; shop/staged-optional-parameter, expr-guard-two-names, limits-observation-document-bytes; architecture/staged-availability, staged-sampling-claim, staged-availability-missed; rover/staged-vote-probability, staged-vote-probability-missed; lock/staged-two-locks-deadlock, two-locks-ordered; parts-saga/staged-saga-deadlock-free; run-inputs/call-three-items-complete; engines/deadlock-no-terminal-refuted, deadlock-terminal-misses-state. These are corpus-owned repair behaviors under QSL-2727; requirements remain upstream (FR-577-AC-31, FR-651/FR-550-AC-3 and FR-366 references are context, not locally minted test bindings). No new unowned executable behavior, source/test stub, mock-only evidence, vendored implementation or coverage inflation was identified in this diff. No trace tags, harness, thresholds or suppression settings changed.

Changed JSON parses and every file reference in changed requests resolves. All example TOML parses with /usr/bin/python3.13. The full manifest/index case sets, statements, statuses, exit/verdict, readiness, waits_on and needs fields agree; root README running/staged counts agree. `git diff --check` passes. Engine and lock source/request files are unchanged from base main, preserving PR 83 bytes and all lock clauses. No runtime verification was performed by this reviewer.

The lead-supplied service evidence is 15/15 guards comprising 11 running checks plus four staged dry-run guards, on the known development-service cohort (old AA71/FSL f597), not 15 running successes or a fresh released pin. Root must retain latest review custody, then perform the mandatory final current-main scoped 15-case gate and 10-minute merge window. QSL-2727 remains open for negative sampling; QSL-2158 tracks that missing unforced result. Saga follow-up ownership remains with architecture; its current placeholder waits_on QSL-2727 is not a claim of delivery completion.
