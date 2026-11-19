---
id: SR-008
title: QSL-2184 function claim examples gap-analysis
type: SpecReview
analysis: gap-analysis
scope: agent-ix/filament-examples; examples/shop claim-function cases, README.md and
  index.json at e967bd91b388c4ee8368f9c5ab14be5e6f56bf7b
review_set: subset
evaluated_revision: e967bd91b388c4ee8368f9c5ab14be5e6f56bf7b
relationships:
- target: ix://agent-ix/filament-spec-language/FR-867
  type: references
---

## Summary

Six shop function selections were inspected against the merged FR-867 contract.

## Verdict

FAIL. The refusal diagnostics and the undefined outcome lack the required evidence.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The integer, bounded and unknown expected items omit diagnostic. The FR-867 contract requires type-mismatch details naming seven/Integer or boundedValue/Int[0, 3] with expected Boolean, and missing-name detail naming nope. A generic declined item with the right code passes these records. The language projection at 0b4bbfc still sets diagnostic to None. Fix that producer and record the three actual outputs. | examples/shop/expected/claim-function-integer.json:14 |
| FND-002 | high | The undefined expected file has an empty items array. After role binding, the current language serializer builds an undefined-evaluation cause with code = reason, omitting the required where and cause members. Promoting an exit-10 recording from that serializer would still miss FR-867-AC-3. Verify the complete refuted item with a driver that carries the corrected projection. | examples/shop/expected/claim-function-undefined.json:2 |

## Coverage

The six requests use native-run/1 and select their named function over the same program, model and snapshot. The undefined selection supplies an empty orders population and the target key from lookupTargets. All paths resolve. The holds and false files correctly encode tested/exit 0 and refuted/exit 10. The manifest, index, and README agree at 153 running and 3 staged. Static checks ran at the review head. No build was started and no live six-case pass is asserted.

Language PR #2958 merged at 0991d3c7. At that revision and at language main 0b4bbfc2822c7592b53800ffab9f9233f603609d, fsl-replay/src/outcome.rs:1463 sets diagnostic: None in from_clause_run and line 1645 stores the undefined reason under code. Driver #189 merged at bcdd9b96 with its FSL dependency still at a2f4ae87, before QSL-2462. Re-recording the current driver cannot demonstrate the required structures. The example runner copies whole items into the compared record (tools/fsl-examples/src/outcome.rs:65). Quoin 0.36.7 matrix on terra exited 2 (CORE_REFUSED / QQ-1002) because this repository has no root spec/ directory. Reviewed the ACs directly from the owning FSL spec at 25c129c0. No plan or optional semantic audit was requested.

Plan completion: not assessed

```yaml
scope:
- id: FR-867-AC-3
  path: filament-spec-language/spec/functional/FR-867-name-the-function-and-its-result-type-when-a-non-boolean-function-is-selected-as-a-claim.md
  role: examined
  excerpt: '`claim-function-undefined` exit 10, item `refuted`, `cause` `{"kind": "undefined-evaluation", "where": W, "cause": R}` with `R` its FR-100 undefined reason; `claim-function-integer`
    (`seven`) exit 20, item `declined`, diagnostic `ill_typed`/`type-mismatch` with detail `function` `seven`, `expected` `Boolean`, `actual` `Integer`; `claim-function-bounded` (`boundedValue`)
    the same with `actual` `Int[0, 3]`; `claim-function-unknown` (`nope`) exit 20, item `declined`, diagnostic `missing_declaration`/`missing-name` with detail `name` `nope`. Each has
    its recorded `expected/<case>.json`, and the ex'
- id: FR-867-AC-4
  path: filament-spec-language/spec/functional/FR-867-name-the-function-and-its-result-type-when-a-non-boolean-function-is-selected-as-a-claim.md
  role: examined
  excerpt: '`diagnostic` `{"code": "ill_typed", "cause": "type-mismatch", "detail": {"function": "seven", "expected": "Boolean", "actual": "Integer"}}`; over the `boundedValue` run,
    `detail.actual` `Int[0, 3]`; over the `nope` run, `diagnostic` `{"code": "missing_declaration", "cause": "missing-name", "detail": {"name": "nope"}}`; over the `yes` run, an item
    `tested` with `cause` null and no `diagnostic`.'
- id: claim-functions
  path: examples/shop/rules/claim-functions.fsl
  role: examined
  excerpt: "language \"ix:native\" edition \"1-draft\";\nprofile v = \"filament.model.complete/v1\";\nmodel Shop = \"example/shop-outcomes\";\n\nfunction yes using v(): Boolean pure\
    \ { true }\nfunction no using v(): Boolean pure { false }\nfunction seven using v(): Integer pure { 7 }\nfunction boundedValue using v(): Int[0, 3] pure { 3 }\n\nfunction undefinedClaim\
    \ using v(\n  p: Population<Shop::Order>[0],\n  r: Shop::Order\n): Boolean pure {\n  lookup<Shop::Order>(p, r) absent undefined = r\n}\n"
- id: Systems counts
  path: README.md
  role: examined
  excerpt: '| [shop](examples/shop) | Shop | 153 | 3 |'
- id: claim-function-holds
  path: examples/shop/example.toml
  role: examined
  excerpt: 'id = "claim-function-holds"

    rule = "yes"

    statement = "A Boolean function selected as a claim is tested against the current snapshot."

    file = "rules/claim-functions.fsl"

    command = "run requests/claim-function-holds.json"

    status = "running"

    expect_exit = 0

    expect_verdict = "holds"

    serves = "Function claims"'
- id: request-holds
  path: examples/shop/requests/claim-function-holds.json
  role: examined
  excerpt: "\"selection\": {\"function\": {\"name\": \"yes\", \"arguments\": [], \"snapshot\": \"current\"}},\n      \"observations\": {\"current\": \"../data/outcomes-undefined-call.json\"\
    }\n    }\n  }\n}"
- id: expected-holds
  path: examples/shop/expected/claim-function-holds.json
  role: examined
  excerpt: "{\n  \"category\": \"success\",\n  \"last_stage\": \"S6a\",\n  \"result\": null,\n  \"first_diagnostic\": null,\n  \"items\": [\n    {\n      \"axes\": {\n        \"activation\"\
    : \"active\",\n        \"execution\": \"completed\",\n        \"settlement\": \"closed-scope\",\n        \"truth\": \"true\"\n      },\n      \"category\": \"success\",\n      \"\
    cause\": null,\n      \"clause\": \"yes\",\n      \"request_index\": 0,\n      \"result\": \"tested\"\n    }\n  ]\n}"
- id: index-holds
  path: index.json
  role: examined
  excerpt: "\"case\": \"claim-function-holds\",\n      \"status\": \"running\",\n      \"rule\": \"yes\",\n      \"statement\": \"A Boolean function selected as a claim is tested against\
    \ the current snapshot.\","
- id: claim-function-false
  path: examples/shop/example.toml
  role: examined
  excerpt: 'id = "claim-function-false"

    rule = "no"

    statement = "A Boolean function returning false is refuted as a claim."

    file = "rules/claim-functions.fsl"

    command = "run requests/claim-function-false.json"

    status = "running"

    expect_exit = 10

    expect_verdict = "violated"

    serves = "Function claims"'
- id: request-false
  path: examples/shop/requests/claim-function-false.json
  role: examined
  excerpt: "\"selection\": {\"function\": {\"name\": \"no\", \"arguments\": [], \"snapshot\": \"current\"}},\n      \"observations\": {\"current\": \"../data/outcomes-undefined-call.json\"\
    }\n    }\n  }\n}"
- id: expected-false
  path: examples/shop/expected/claim-function-false.json
  role: examined
  excerpt: "{\n  \"category\": \"violation\",\n  \"last_stage\": \"S6a\",\n  \"result\": null,\n  \"first_diagnostic\": null,\n  \"items\": [\n    {\n      \"axes\": {\n        \"activation\"\
    : \"active\",\n        \"execution\": \"completed\",\n        \"settlement\": \"closed-scope\",\n        \"truth\": \"false\"\n      },\n      \"category\": \"violation\",\n    \
    \  \"cause\": null,\n      \"clause\": \"no\",\n      \"request_index\": 0,\n      \"result\": \"refuted\"\n    }\n  ]\n}"
- id: index-false
  path: index.json
  role: examined
  excerpt: "\"case\": \"claim-function-false\",\n      \"status\": \"running\",\n      \"rule\": \"no\",\n      \"statement\": \"A Boolean function returning false is refuted as a claim.\"\
    ,"
- id: claim-function-undefined
  path: examples/shop/example.toml
  role: examined
  excerpt: 'id = "claim-function-undefined"

    rule = "undefinedClaim"

    statement = "A function claim with an undefined evaluation is refuted."

    file = "rules/claim-functions.fsl"

    command = "run requests/claim-function-undefined.json"

    status = "staged"

    expect_exit = 20

    ready_exit = 10

    waits_on = "QSL-2462"

    needs = "Population<Order>[0] function-claim argument role mapping"

    why = "the population argument is refused invalid_runtime_input/wrong-role-mapping at argument 0; QSL-2462 must map the selected population and reference before this claim can be
    refuted as undefined"

    serves = "Function claims"'
- id: request-undefined
  path: examples/shop/requests/claim-function-undefined.json
  role: examined
  excerpt: "\"selection\": {\"function\": {\n        \"name\": \"undefinedClaim\",\n        \"arguments\": [\n          {\"parameter\": \"p\", \"value\": {\"population\": \"ix://example/shop-outcomes/orders\"\
    }},\n          {\"parameter\": \"r\", \"value\": {\"reference\": {\n            \"population\": \"ix://example/shop-outcomes/lookupTargets\", \"key\": \"target\"\n          }}}\n\
    \        ],\n        \"snapshot\": \"current\"\n      }},\n      \"observations\": {\"current\": \"../data/outcomes-undefined-call.json\"}\n    }\n  }\n}"
- id: expected-undefined
  path: examples/shop/expected/claim-function-undefined.json
  role: examined
  excerpt: "{\n  \"category\": \"refusal\",\n  \"last_stage\": \"S6a\",\n  \"result\": null,\n  \"first_diagnostic\": {\n    \"code\": \"invalid_runtime_input\",\n    \"cause\": \"wrong-role-mapping\"\
    \n  },\n  \"items\": []\n}"
- id: index-undefined
  path: index.json
  role: examined
  excerpt: "\"case\": \"claim-function-undefined\",\n      \"status\": \"staged\",\n      \"rule\": \"undefinedClaim\",\n      \"statement\": \"A function claim with an undefined evaluation\
    \ is refuted.\","
- id: claim-function-integer
  path: examples/shop/example.toml
  role: examined
  excerpt: 'id = "claim-function-integer"

    rule = "seven"

    statement = "A non-Boolean function selected as a claim is refused with its result type."

    file = "rules/claim-functions.fsl"

    command = "run requests/claim-function-integer.json"

    status = "running"

    expect_exit = 20

    expect_verdict = "refused: type-mismatch"

    serves = "Function claims"'
- id: request-integer
  path: examples/shop/requests/claim-function-integer.json
  role: examined
  excerpt: "\"selection\": {\"function\": {\"name\": \"seven\", \"arguments\": [], \"snapshot\": \"current\"}},\n      \"observations\": {\"current\": \"../data/outcomes-undefined-call.json\"\
    }\n    }\n  }\n}"
- id: expected-integer
  path: examples/shop/expected/claim-function-integer.json
  role: examined
  excerpt: "{\n  \"category\": \"refusal\",\n  \"last_stage\": \"S4\",\n  \"result\": null,\n  \"first_diagnostic\": null,\n  \"items\": [\n    {\n      \"axes\": {\n        \"activation\"\
    : \"unavailable\",\n        \"execution\": null,\n        \"settlement\": \"unavailable\",\n        \"truth\": null\n      },\n      \"category\": \"refusal\",\n      \"cause\":\
    \ {\n        \"code\": \"ill_typed\",\n        \"kind\": \"type-mismatch\"\n      },\n      \"clause\": \"seven\",\n      \"request_index\": 0,\n      \"result\": \"declined\"\n\
    \    }\n  ]\n}"
- id: index-integer
  path: index.json
  role: examined
  excerpt: "\"case\": \"claim-function-integer\",\n      \"status\": \"running\",\n      \"rule\": \"seven\",\n      \"statement\": \"A non-Boolean function selected as a claim is refused\
    \ with its result type.\","
- id: claim-function-bounded
  path: examples/shop/example.toml
  role: examined
  excerpt: 'id = "claim-function-bounded"

    rule = "boundedValue"

    statement = "A bounded non-Boolean function claim names its declared result type."

    file = "rules/claim-functions.fsl"

    command = "run requests/claim-function-bounded.json"

    status = "running"

    expect_exit = 20

    expect_verdict = "refused: type-mismatch"

    serves = "Function claims"'
- id: request-bounded
  path: examples/shop/requests/claim-function-bounded.json
  role: examined
  excerpt: "\"selection\": {\"function\": {\"name\": \"boundedValue\", \"arguments\": [], \"snapshot\": \"current\"}},\n      \"observations\": {\"current\": \"../data/outcomes-undefined-call.json\"\
    }\n    }\n  }\n}"
- id: expected-bounded
  path: examples/shop/expected/claim-function-bounded.json
  role: examined
  excerpt: "{\n  \"category\": \"refusal\",\n  \"last_stage\": \"S4\",\n  \"result\": null,\n  \"first_diagnostic\": null,\n  \"items\": [\n    {\n      \"axes\": {\n        \"activation\"\
    : \"unavailable\",\n        \"execution\": null,\n        \"settlement\": \"unavailable\",\n        \"truth\": null\n      },\n      \"category\": \"refusal\",\n      \"cause\":\
    \ {\n        \"code\": \"ill_typed\",\n        \"kind\": \"type-mismatch\"\n      },\n      \"clause\": \"boundedValue\",\n      \"request_index\": 0,\n      \"result\": \"declined\"\
    \n    }\n  ]\n}"
- id: index-bounded
  path: index.json
  role: examined
  excerpt: "\"case\": \"claim-function-bounded\",\n      \"status\": \"running\",\n      \"rule\": \"boundedValue\",\n      \"statement\": \"A bounded non-Boolean function claim names\
    \ its declared result type.\","
- id: claim-function-unknown
  path: examples/shop/example.toml
  role: examined
  excerpt: 'id = "claim-function-unknown"

    rule = "nope"

    statement = "A function claim naming no declaration is refused with that name."

    file = "rules/claim-functions.fsl"

    command = "run requests/claim-function-unknown.json"

    status = "running"

    expect_exit = 20

    expect_verdict = "refused: missing-name"

    serves = "Function claims"'
- id: request-unknown
  path: examples/shop/requests/claim-function-unknown.json
  role: examined
  excerpt: "\"selection\": {\"function\": {\"name\": \"nope\", \"arguments\": [], \"snapshot\": \"current\"}},\n      \"observations\": {\"current\": \"../data/outcomes-undefined-call.json\"\
    }\n    }\n  }\n}"
- id: expected-unknown
  path: examples/shop/expected/claim-function-unknown.json
  role: examined
  excerpt: "{\n  \"category\": \"refusal\",\n  \"last_stage\": \"S4\",\n  \"result\": null,\n  \"first_diagnostic\": null,\n  \"items\": [\n    {\n      \"axes\": {\n        \"activation\"\
    : \"unavailable\",\n        \"execution\": null,\n        \"settlement\": \"unavailable\",\n        \"truth\": null\n      },\n      \"category\": \"refusal\",\n      \"cause\":\
    \ {\n        \"code\": \"missing_declaration\",\n        \"kind\": \"missing-name\"\n      },\n      \"clause\": \"nope\",\n      \"request_index\": 0,\n      \"result\": \"declined\"\
    \n    }\n  ]\n}"
- id: index-unknown
  path: index.json
  role: examined
  excerpt: "\"case\": \"claim-function-unknown\",\n      \"status\": \"running\",\n      \"rule\": \"nope\",\n      \"statement\": \"A function claim naming no declaration is refused\
    \ with that name.\","
- id: recorded-undefined-message
  path: examples/shop/expected/claim-function-undefined.messages.txt
  role: examined
  excerpt: 'the argument at position 0 is refused: wrong-role-mapping at ""'
```

## Dispositions

Round 1 at `e91ac5933c5f0682e09d2079f07f62df08e3cf72`.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | deferred | QSL-2500 explicitly owns diagnostic.detail for the integer, bounded and unknown select refusals. The three running cases match the current driver, and their recorded items will need refreshing when the corrected serializer is available. |
| FND-002 | deferred | QSL-2500 owns the undefined item with where and cause. Both the manifest and regenerated index stage claim-function-undefined for exit 10. Its current exit-20 refusal remains the expected baseline for the pinned driver. |

Round 2 at `e4a78e2c66ef2d63ce2d04624b31ac61b5dcac41`.

Both findings were deferred to QSL-2500 in round 1.

## Verdict (disposition pass 1)

PASS for the approved partial merge. Both upstream gaps are explicitly deferred to QSL-2500. Full FR-867 acceptance remains pending.

Both workflow source defaults and their checkout/build inputs select driver c6e7741ac632074126c8e7fd8a8108581e22f0c0. The examples branch is based on main ddb1baa3. All six manifest entries match their index statuses, exit codes and owners. The dispatch reports five running claim cases and three staged shop cases passed at that driver pin. Those gate results verify the current recorded outputs. QSL-2500 will require new recordings to verify the FR-867 payloads.

## Verdict (disposition pass 2)

PASS for the approved partial merge. The manifest and PR body agree that the current pin yields exit 20 and that promotion follows QSL-2500 and the aligned driver pin.
