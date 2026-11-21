---
id: SR-2026111919340478491681439
title: "QSL-2586 PR82 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-examples; QSL-2586; PR82 five-path promotion and unchanged native request/program/data/harness context"
review_set: subset
---

## Summary

Bounded manual acceptance/oracle coverage is clean; repository-wide gap assurance is unavailable because this examples repository has no owned root spec scope.

## Verdict

**CONDITIONAL** — mergeable bounded manual acceptance only; no engine-backed PASS or whole-repository reverse-gap completeness is claimed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Evidence

Frozen candidate: `f42dcf22a10aca847de904170096d02a5fa5a690`; base: `1fb4096f2627f7546347ce527c0e6d58a64753c3`. Ticket: QSL-2586. Exact GitHub head/base and the clean local worktree match. Only the five specified paths differ. No repository AGENTS.md or CLAUDE.md is present; supplied session instructions apply. No root spec/ or applicable AssuranceProfile is present.

The existing native-run/1 request selects the unchanged Boolean undefinedClaim over the current snapshot. Its zero-capacity orders population is complete and empty, while the reference names the real target in lookupTargets. The unchanged body uses lookup absent undefined. This is a meaningful absent-key evaluation, not an input refusal or a fabricated successful claim. The supplied immutable AA71/FSL f597 semantic result is rc10, violation/refuted, axes active/completed/closed-scope/false, with undefined-evaluation and the actual body location index 4/path [0]. The candidate retains that entire item, no top-level diagnostic, S6a and null top-level result. The actual execution was reported by the lead/service; this reviewer did not execute or independently recapture it.

Exact FSL f597 FR-867's examples-recording section and the undefined clause of AC-3 require refutation, exit10 and undefined-evaluation with W/R; FR-100 grounds absent-key. Only that AC-3 subclause is examined here; the other five cases are unchanged context, not independently re-executed. TC-2951 concerns select-refusal serialization under AC-4 and supplies context only; it is not claimed to test this undefined case.

The unchanged runner clones full items and compares the entire projected JSON plus exit. Thus it detects wrong cause, location, truth, result or settlement even if exit10 still matches. Message snapshots are removed by the existing writer when diagnostic text is empty; deleting the obsolete wrong-role-mapping text is consistent with no diagnostic. One manifest case changes staged20/ready10 to running10/violated and removes obsolete blocker metadata. The generated index differs only in that entry's status/verdict/exit/ready_exit/waits_on/needs; engine, clock, data/source paths and every other entry remain equal. Independent static JSON/manifest inspection counts shop 154 running and 2 staged, matching README; total case count is unchanged. Native programs, requests, model and data, architecture family and harness source are byte-unchanged versus base. git diff --check is clean.

The PR body distinguishes ARCH's reported corpus classification from a new baseline: absence of historical settlement mismatches from the reported exceptions is an inference. It accurately cites the already-landed #71 native-compile unsupported/S3 oracle, excludes item1 IR-1568 hardening and identifies item3 as driver #204. This review neither repeats ARCH's corpus measurement nor asserts an exhaustive semantic corpus pass. No blanket settlement updates or capability narrowing appear.

Accepted service evidence is exactly one focused case passed, zero failed, rc0, on immutable AA71 (sha256:5187190217b0edc053e14b8ff53c1de1e9d6e40a217ee289cef4e2a869e380c6), after existing-harness promote --verdict violated and check shop --case claim-function-undefined. No Cargo/build/runtime/test/baseline/full gate was run by this reviewer. No new implementation, helper, gate, copied producer spec, suppressed warning, weakened comparison or fake pass was introduced.

## Coverage

Plan completion: not assessed

Code-review and gap-analysis method instructions were loaded and applied to the authorized slice. Rust/React lanes are inapplicable because no Rust/React file changes; unchanged Rust harness code was inspected for oracle integrity only. spec-review and its subanalyses are inapplicable because no spec/plan changes occur. Optional whole-repository semantic review was not run.

Reconciliation: bounded manual manifest/native request/program/data/oracle comparison. Static quire matrix --scope /home/peter/dev/worktrees/terra-qsl2586-closeout --format json emitted missing-document-root diagnostic for the absent root spec/. No repository-local traceability config or owned FR criteria exist for a computed matrix. This is the established examples75 coverage limitation, not evidence of missing producer test tags. No grep fallback, local test-to-AC bindings, matrix counts, producer test coverage or repository-wide PASS is invented. Whole-repository matrix, reverse-gap completeness and stub inventory remain unassessed. Changed-slice reverse-gap/stub inspection covers one existing executable declarative case with QSL-2586 and contextual FR-867 ownership; the concrete lookup, admitted observations and full structural comparison are not hollow. No new runtime implementation is introduced.

Installed tools: /home/peter/.npm-global/bin/quoin 0.36.8; /home/peter/.npm-global/bin/quire 0.52.0 (engine 0.62.2); spec-artifacts-process@v0.36.0. These current versions are newer than the supplied Nov10 release notice and were reported to the lead. Current authoring contract loaded successfully. Python's default interpreter has neither tomllib nor toml; static manifest inspection therefore parsed the reviewed simple scalar case fields directly and is not represented as a complete TOML schema check.

## Examined units

```json
{
  "clean": true,
  "scope": [
    {
      "id": "shop-undefined-manifest",
      "path": "examples/shop/example.toml",
      "role": "examined",
      "excerpt": "id = \"claim-function-undefined\"\nrule = \"undefinedClaim\"\nstatement = \"A function claim with an undefined evaluation is refuted.\"\nfile = \"rules/claim-functions.fsl\"\ncommand = \"run requests/claim-function-undefined.json\"\nstatus = \"running\"\nexpect_exit = 10\nexpect_verdict = \"violated\""
    },
    {
      "id": "shop-undefined-oracle",
      "path": "examples/shop/expected/claim-function-undefined.json",
      "role": "examined",
      "excerpt": "\"cause\": {\n        \"cause\": \"absent-key\",\n        \"kind\": \"undefined-evaluation\",\n        \"where\": {\n          \"declaration\": {\n            \"function\": \"undefinedClaim\",\n            \"index\": 4,\n            \"kind\": \"body\"\n          },\n          \"path\": [\n            0\n          ]\n        }\n      },\n      \"clause\": \"undefinedClaim\",\n      \"request_index\": 0,\n      \"result\": \"refuted\""
    },
    {
      "id": "shop-obsolete-message",
      "path": "examples/shop/expected/claim-function-undefined.messages.txt",
      "role": "examined",
      "excerpt": "the argument at position 0 is refused: wrong-role-mapping at \"\""
    },
    {
      "id": "shop-readme-count",
      "path": "README.md",
      "role": "examined",
      "excerpt": "| [shop](examples/shop) | Shop | 154 | 2 |"
    },
    {
      "id": "shop-index",
      "path": "index.json",
      "role": "examined",
      "excerpt": "\"case\": \"claim-function-undefined\",\n      \"status\": \"running\",\n      \"rule\": \"undefinedClaim\",\n      \"statement\": \"A function claim with an undefined evaluation is refuted.\","
    },
    {
      "id": "shop-undefined-body",
      "path": "examples/shop/rules/claim-functions.fsl",
      "role": "examined",
      "excerpt": "function undefinedClaim using v(\n  p: Population<Shop::Order>[0],\n  r: Shop::Order\n): Boolean pure {\n  lookup<Shop::Order>(p, r) absent undefined = r\n}"
    },
    {
      "id": "shop-undefined-request",
      "path": "examples/shop/requests/claim-function-undefined.json",
      "role": "examined",
      "excerpt": "\"name\": \"undefinedClaim\",\n        \"arguments\": [\n          {\"parameter\": \"p\", \"value\": {\"population\": \"ix://example/shop-outcomes/orders\"}},\n          {\"parameter\": \"r\", \"value\": {\"reference\": {\n            \"population\": \"ix://example/shop-outcomes/lookupTargets\", \"key\": \"target\"\n          }}}\n        ],\n        \"snapshot\": \"current\""
    },
    {
      "id": "shop-undefined-data",
      "path": "examples/shop/data/outcomes-undefined-call.json",
      "role": "examined",
      "excerpt": "\"population\": \"ix://example/shop-outcomes/orders\",\n      \"complete\": true,\n      \"objects\": []"
    },
    {
      "id": "FR-867-AC-3-undefined-clause",
      "path": "agent-ix/filament-spec-language@f5977125981288ea1bd36fb67f71780182055aac:spec/functional/FR-867-name-the-function-and-its-result-type-when-a-non-boolean-function-is-selected-as-a-claim.md",
      "role": "examined",
      "excerpt": "`claim-function-undefined` exit 10, item `refuted`, `cause` `{\"kind\": \"undefined-evaluation\", \"where\": W, \"cause\": R}` with `R` its FR-100 undefined reason;"
    },
    {
      "id": "FR-100-family-undefined",
      "path": "agent-ix/filament-spec-language@f5977125981288ea1bd36fb67f71780182055aac:spec/functional/FR-100-run-a-named-function-through-the-spine.md",
      "role": "examined",
      "excerpt": "| `FamilyResult::Undefined(u)` | `undefined`, `reason` the catalog's `UndefinedReason` spelling (`precondition-false`, `absent-key`) | 10 |"
    },
    {
      "id": "TC-2951",
      "path": "agent-ix/filament-spec-language@f5977125981288ea1bd36fb67f71780182055aac:spec/test-cases/TC-2951-function-claim-refusal-in-the-outcome-document.md",
      "role": "context_only",
      "excerpt": "Verify that the `filament-outcome/1` `monitor` document of a `function` claim run refused at stage `select` carries the refusal's `diagnostic` with its FR-272 payload. Scope: FR-867-AC-4."
    },
    {
      "id": "harness-full-structure",
      "path": "tools/fsl-examples/src/outcome.rs",
      "role": "examined",
      "excerpt": "    let live = project(&run.stdout).structure;\n    if &live != expected {"
    },
    {
      "id": "harness-message-removal",
      "path": "tools/fsl-examples/src/runner.rs",
      "role": "examined",
      "excerpt": "    if recorded.messages.is_empty() {\n        if snapshot.exists() {\n            fs::remove_file(&snapshot).map_err(|e| Error::io(&snapshot, &e))?;\n        }"
    },
    {
      "id": "harness-running-index",
      "path": "tools/fsl-examples/src/generated.rs",
      "role": "examined",
      "excerpt": "                Status::Running { expect_verdict } => (Some(expect_verdict.clone()), None, None),"
    },
    {
      "id": "current-harness-config",
      "path": "config.mk",
      "role": "context_only",
      "excerpt": "FSL ?= fsl\nexport FSL"
    },
    {
      "id": "pr82-history-claim",
      "path": "https://github.com/agent-ix/filament-examples/pull/82",
      "role": "examined",
      "excerpt": "For QSL-2586 item 2, the historical settlement mismatches were not among the exceptions in ARCH's reported current 1,402-case classification; this is an inference from that reported classification, not a new corpus run."
    },
    {
      "id": "architecture-existing-compile-oracle",
      "path": "examples/architecture/expected/claims-unbounded-event.json",
      "role": "context_only",
      "excerpt": "{\n  \"category\": \"unsupported\",\n  \"last_stage\": \"S3\",\n  \"result\": null,\n  \"first_diagnostic\": {\n    \"code\": \"unsupported_construct\",\n    \"cause\": \"expression-form\"\n  },\n  \"items\": []\n}"
    }
  ],
  "findings": [],
  "bindings": []
}
```

## Custody and deferred tracker actions

Artifacts exist only in reviewer scratch. No source edits, pushes, GitHub comments, Linear calls or labels were performed. The lead must retain these latest two validated SRs, then deduplicate/post the prepared private method comments and apply/check has-review after the 11:39 PT Nov21 embargo. Tracker read-back has not occurred; local prepared JSON/YAML-compatible data parses and matches these clean placeholders.
