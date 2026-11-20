---
id: SR-1438
title: "QSL-2483 Mac main-build gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-examples; QSL-2483; .github/workflows/install-test.yml, tools/macos-main-smoke.sh, tools/README.md; frozen 9cfac97"
review_set: subset
---

## Summary

Diff-scoped acceptance-to-implementation inspection of the native Mac source-build route. The change supplies the missing workflow hook and invokes the existing meaningful native quickstart test. It does not claim a native run has passed.

## Verdict

**CONDITIONAL** — Resolve SR-1437 FND-001, then collect the actual native arm64 result through this workflow. No additional requirement or oracle defect was found in this scoped pass.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Manual reconciliation covers all three changed files and the invoked existing platform oracle. No matrix or repository-wide inventory was run under the explicit brief. This review makes no whole-repository traceability or assurance claim. No spec artifacts or Rust code changed.

The user-authorized behavior is a main-build option for the native Mac test, without requiring a new released asset. The workflow supplies explicit graph inputs; the script obtains authoritative public sources outside the product checkout and builds one native FSL; the existing oracle enforces Darwin arm64, runs the selected driver's README source/request/command, and checks version, successful execute outcome, completed Boolean true and empty diagnostics. Build/test failure propagates. Cleanup and retained logs are explicit. Native Mac execution is pending, and the older accepted Linux result was not rechecked.

Code review's runner budget defect is recorded once in SR-1437 rather than duplicated here. No copied models, binaries, schemas or source snapshots enter this repository. Temporary dependency checkouts are build dependencies, not vendored content. The source selection remains explicit; no migration shim or compatibility layer is introduced.

Accepted checks: coder's bash syntax and diff checks. No test/build/live gate was executed by this reviewer. Optional whole-repository semantic analysis and computed-matrix audit were not run; scoped code-to-task/oracle faithfulness is part of this review.

Plan completion: not assessed

## Examined units

```json
[
  {
    "id": "mac-main-runner",
    "path": ".github/workflows/install-test.yml",
    "role": "examined",
    "excerpt": "name: README smoke (native macOS arm64, source)"
  },
  {
    "id": "mac-main-build",
    "path": "tools/macos-main-smoke.sh",
    "role": "examined",
    "excerpt": "cargo build --config \"$config\" -p fsl-cli --bin fsl"
  },
  {
    "id": "mac-main-smoke",
    "path": "tools/macos-main-smoke.sh",
    "role": "examined",
    "excerpt": "FSL_SMOKE_PLATFORM=macos-arm64"
  },
  {
    "id": "mac-main-usage",
    "path": "tools/README.md",
    "role": "examined",
    "excerpt": "No release asset\nis needed, and a Linux run supplies no Mac pass evidence."
  },
  {
    "id": "existing-platform-oracle",
    "path": "tools/fsl-examples/tests/platform_smoke.rs",
    "role": "context_only",
    "excerpt": "assert_eq!(outcome[\"category\"], \"success\", \"{platform}: {outcome}\");"
  }
]
```

## Disposition round 1 coverage

Fix head ec32952 replaces local FSL compilation with consumption of an exact native main-build artifact. SR-1437 FND-001 is fixed. The fix preserves the actual Darwin-arm64 guard and the existing meaningful README quickstart oracle, binds the executable artifact to a successful manual main run and exact driver commit/digest, and fetches that commit's README. No newly introduced requirement/oracle gaps were found in this fix diff.

The scoped code review is now clear. This analysis remains **CONDITIONAL** only on the actual main-build producer result and native Mac gate. Neither a downloaded artifact nor a native Mac pass is claimed. No broad analysis, tests, builds, producer workflow dispatch, artifact download or platform journey was performed by this reviewer.
