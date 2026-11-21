---
id: SR-2026111919340478491681434
title: PR 72 code-review review
type: SpecReview
analysis: code-review
scope: agent-ix/filament-examples; QSL-2735; complete seven-file PR 72 diff
review_set: subset
examined_scope:
- id: README.md
  path: README.md
  role: examined
  excerpt: '| [language-values](examples/language-values) | Language | 44 | 0 |'
- id: examples/language-values/data/native-operation-dispatch.json
  path: examples/language-values/data/native-operation-dispatch.json
  role: examined
  excerpt: "{\n  \"format\": \"filament.state.snapshot/v1\",\n  \"identity\": {\n\
    \    \"authority\": \"example\",\n    \"identity\": \"language-values/native-operation-dispatch/current\"\
    \n  },\n  \"observation\": \"current\",\n  \"anchor\": {\n    \"kind\": \"handler\"\
    ,\n    \"name\": \"validate\"\n  },\n  \"populations\": [\n    {\n      \"population\"\
    : \"orders\",\n      \"complete\": true,\n      \"objects\": [\n        {\n  \
    \        \"key\": \"sub1\",\n          \"type\": \"Sub\",\n          \"fields\"\
    : {\n            \"total\": {\n              \"integer\": \"4\"\n            }\n\
    \          }\n        }\n      ]\n    }\n  ]\n}\n"
- id: examples/language-values/example.toml
  path: examples/language-values/example.toml
  role: examined
  excerpt: 'name = "language-values"

    summary = "Original runnable value-language forms with independently stated semantic
    results."


    [[case]]

    id = "native-operation-dispatch"

    rule = "DispatchSelectsSubtype"

    statement = "A native Sub object conforms to Order, and self.size() in an Order
    invariant selects Sub''s redefining body (7) over Order''s body (1)."

    file = "rules/native-operation-dispatch.fsl"

    command = "run requests/native-operation-dispatch.json"

    status = "running"

    expect_exit = 0

    expect_verdict = "holds"


    [[case]]

    id = "float-specials-nan"

    rule = "example"

    statement = "NaN is unequal to itself."

    fil'
- id: examples/language-values/expected/native-operation-dispatch.json
  path: examples/language-values/expected/native-operation-dispatch.json
  role: examined
  excerpt: "{\n  \"category\": \"success\",\n  \"last_stage\": \"S6a\",\n  \"result\"\
    : null,\n  \"first_diagnostic\": null,\n  \"items\": [\n    {\n      \"axes\"\
    : {\n        \"activation\": \"active\",\n        \"execution\": \"completed\"\
    ,\n        \"settlement\": \"decisive-witness\",\n        \"truth\": \"true\"\n\
    \      },\n      \"category\": \"success\",\n      \"cause\": null,\n      \"\
    clause\": \"DispatchSelectsSubtype\",\n      \"request_index\": 0,\n      \"result\"\
    : \"tested\",\n      \"witness\": {\n        \"deciding_comparison\": {\n    \
    \      \"node_id\": \"{\\\"ordinal\\\":0,\\\"package\\\":\\\"language-values/native-operation-dispatch\\\
    \",\\\"path\\\":[\\\"expression\\\",\\\"binary\\\",{"
- id: examples/language-values/requests/native-operation-dispatch.json
  path: examples/language-values/requests/native-operation-dispatch.json
  role: examined
  excerpt: "{\n  \"format\": \"native-run/1\",\n  \"request\": {\n    \"models\":\
    \ [],\n    \"program\": {\n      \"source\": {\n        \"file\": \"../rules/native-operation-dispatch.fsl\"\
    ,\n        \"authority\": \"example\",\n        \"identity\": \"language-values/native-operation-dispatch\"\
    ,\n        \"document\": \"native-operation-dispatch\",\n        \"formal_revision\"\
    : 1\n      }\n    },\n    \"clause\": {\n      \"selection\": {\n        \"clause\"\
    : {\n          \"name\": \"DispatchSelectsSubtype\",\n          \"observation\"\
    : \"current\",\n          \"self\": {\n            \"population\": \"orders\"\
    ,\n            \"key\": \"sub1\"\n          },\n          \"anchor\": {\n"
- id: examples/language-values/rules/native-operation-dispatch.fsl
  path: examples/language-values/rules/native-operation-dispatch.fsl
  role: examined
  excerpt: "language \"ix:native\" edition \"1-draft\";\nprofile P = \"filament.model.complete/v1\"\
    ;\n\nobject Order identity (total) {\n  total: Int[0, 9];\n  population orders;\n\
    \  operation size(): Integer;\n}\n\nobject Sub extends Order {\n  operation size():\
    \ Integer redefines Order::size;\n}\n\nbody SubSize using P on Sub::size() { 7\
    \ }\nbody OrderSize using P on Order::size() { 1 }\npre CanSize using P on Order::size\
    \ { self.total >= 0 }\n\n// The selected Sub receiver must use SubSize, which\
    \ differs from OrderSize.\ninvariant DispatchSelectsSubtype using P on Order at\
    \ current { self.size() = 7 }\n"
- id: index.json
  path: index.json
  role: examined
  excerpt: '{"system": "language-values", "case": "native-operation-dispatch", "status":
    "running", "rule": "DispatchSelectsSubtype", "statement": "A native Sub object
    conforms to Order, and self.size() in an Order invariant selects Sub''s redefining
    body (7) over Order''s body (1).", "rule_file": "examples/language-values/rules/native-operation-dispatch.fsl",
    "data_files": ["examples/language-values/data/native-operation-dispatch.json"],
    "request_file": "examples/language-values/requests/native-operation-dispatch.json",
    "expected_file": "examples/language-values/expected/native-operation-dispatch.json",
    "v'
- id: FR-081
  path: agent-ix/filament-spec-language@529282e9dc/spec/functional/FR-081-preserve-model-correspondence-and-declaration-identity.md
  role: context_only
  excerpt: 'When the model binder normalizes the original declarations

    [FR-056](FR-056-admit-domain-package-model-declarations.md) admits from one

    domain package, it SHALL derive one immutable correspondence between each

    original declaration and the effective declarations, where selected rules

    derive them. Every original

    declaration key SHALL survive unchanged in that correspondence. Every derived

    declaration SHALL carry a structural identity key built only from the

    correspondence''s own parts. No identity, key or ordering the binder

    produces SHALL depend on a display name, a registration order or any stat'
- id: FR-082
  path: agent-ix/filament-spec-language@529282e9dc/spec/functional/FR-082-resolve-conformance-subsetting-and-redefinition.md
  role: context_only
  excerpt: 'When checking a specialization, a subsetting feature or a redefining member

    against the declaration it narrows, the model checker SHALL decide conformance

    over the closed ancestor graph the admitted domain package declares. The model

    checker SHALL check every independent variance axis the redefined member kind

    defines. The model checker SHALL report every failing axis rather than

    stopping at the first.


    This requirement is FSL''s own binding of filament-specification

    [FR-151](ix://agent-ix/filament-specification/FR-151)''s conformance and

    redefinition rules into the compiler''s behavior. It does '
- id: FR-083
  path: agent-ix/filament-spec-language@529282e9dc/spec/functional/FR-083-resolve-unique-most-specific-dispatch.md
  role: context_only
  excerpt: 'When linking a dispatch table for an operation''s redefinition family,
    the

    model checker SHALL select, for each concrete conforming subtype of the

    operation''s receiver type, the unique undominated redefinition applicable to

    it. The model checker SHALL produce a complete dispatch table only when every

    subtype in the family resolves to exactly one such candidate. An ambiguous or

    incomplete family SHALL produce no dispatch table, including no entry for a

    subtype that itself resolved cleanly.

    '
- id: FR-569
  path: agent-ix/filament-spec-language@529282e9dc/spec/functional/FR-569-declare-object-types-populations-and-operations-in-fsl-source.md
  role: context_only
  excerpt: 'When a complete-V1 unit declares an object type in its own source (its

    fields, its object identity, its supertypes, the populations of its objects

    and its operations with their signatures and frames), S1, S2, the assembler,

    S3 and S4 SHALL admit, check and emit it as below. A

    source-declared object type SHALL check, lower and evaluate exactly as an

    object type that a selected domain package declares (FR-056, FR-094), except

    that it is owned by the unit''s source rather than by a domain package. Its

    operations take the unit''s existing `pre`, `post` and `body` declarations

    (FR-104, FSpec FR-151 `'
- id: FR-302
  path: agent-ix/filament-spec-language@529282e9dc/spec/functional/FR-302-link-dispatch-during-unit-compile.md
  role: context_only
  excerpt: 'When the spine `compile` checks a unit that contains a dispatched call,
    the

    S3 checker SHALL link the dispatch table of the called operation''s

    redefinition family by [FR-083](FR-083-resolve-unique-most-specific-dispatch.md),

    and SHALL produce a checked package only when every called family links. If

    any called family fails to link, the compile SHALL refuse at S3 and produce no

    checked package.

    '
- id: FSpec-FR-151
  path: agent-ix/filament-specification/spec/functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md
  role: context_only
  excerpt: 'Type `S` conforms to type `T` exactly when `S` and `T` are the same effective

    type or a chain of declared supertypes leads from `S` to `T`. `S`

    is strictly narrower than `T` exactly when it is a proper descendant of `T` in

    that closed graph; equal shapes of distinct declarations never make one

    narrower. Redefinition retains correspondence and cannot weaken inherited

    contracts. Dispatch selects, for each closed subtype, the unique undominated

    method; registration, source or declaration order never resolves ambiguity,

    and no linearization is applied. An abstract type has no direct instances.

    '
- id: QSL-2735-native-case
  path: QSL-2735 comment 690b62bb-a902-4f9e-8d64-143b7da15a44
  role: examined
  excerpt: Remaining evidence scope is ONE bounded native example in filament-examples
    showing self.op() with a supertype and a redefined operation, authored in native
    source and run through the driver.
---

## Summary

Bounded example code review found no defect.

## Verdict

PASS — no scoped code-review findings; mergeable for the one native subtype-dispatch case.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Frozen candidate: agent-ix/filament-examples PR #72, head e601f1ea918c20b70fdb2337c1dcbb68152f63cd, base 92a8bc17b27edc21c15f60815cd05251b2f23a51; ticket QSL-2735. All seven changed files examined. No worktree edits or runtime/build/test execution performed. Retained runtime evidence was produced by the lead's draco service using the production CLI from driver219, tested da01 / merged 2e92e4f, version 0.1.4, FSL 529282e9dc and IR 20a4.

Native Order::size returns 1; Sub extends Order and explicitly redefines size with body 7. The invariant on static Order requires self.size() = 7. The request has models: [], explicitly selects current/orders/sub1, and supplies one complete nonempty population containing dynamic Sub with total 4. A base-body selection would produce 1 = 7 and fail. This establishes one positive subtype-dispatch example, not general dispatch coverage. The CanSize precondition is satisfied by the fixture; it is not the dispatch oracle.

Read retained raw stdout and empty stderr at /home/peter/.cache/handoff/qsl-2735-b2c6/logs/. Actual operation monitor is the existing public run clause route: category success, stage S6a, result null, one tested item, active/completed/truth true, deciding Boolean true, operands 7/7, dynamic Sub/sub1 and static Order in the witness. Every projected expected value equals raw output. Existing outcome.rs project/evaluate_case compares all item and witness fields, so a merely successful empty response cannot match. Retained regeneration reports 1 recorded, README generation 2 files, and focused check 1 passed / 0 failed. These checks are supplied evidence, not independent reruns.

Static JSON/TOML checks passed. Four authored files match authoring commit b2c6df4316f25608b3ee1b6aae3c6ae19f302e00 byte for byte. Index differs from base only by the one new case; corpus 607, language-values 44. README changed only its generated count from 43 to 44. Manual docs-style review of that changed row found no prose or navigation issue; broader README lint/rendering was outside this numeric generated edit. git diff --check passed. No implementation, pin, tools/model.mjs, JSON operation body or new syntax changes.

Reviewed producer context at pinned FSL 529282e9dc: FR-081 declaration identity, FR-082 conformance, FR-083 unique most-specific dispatch, FR-569 source objects and snapshot route, and FR-302 native bodies/dispatch joins. Read source_objects and operation_bodies fixture tests as context, not execution evidence. No producer code or specification text is copied into the product patch. The authored minimal example defines its own data and case oracle; generated expected/index/README are the existing runner's consumed outputs. No new helper, gate deletion, warning suppression, stub or vacuous assertion was introduced. No applicable AssuranceProfile exists because the repository has no root spec/.

Rust and React review lanes skipped: no changed Rust or React files. spec-review and its sub-analyses skipped: no spec/plan changes. Python-specific checks do not apply to FSL/JSON/TOML example authoring. No full corpus, staged gate, broad assurance, optional repository-wide semantic audit, new ticket, local fsl or Cargo command was run.
