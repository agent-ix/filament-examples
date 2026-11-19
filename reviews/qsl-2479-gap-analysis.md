---
id: SR-1426
title: "Scoped gap review of the current-main install journey"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-examples; QSL-2479; PR 57; tools/fsl-examples/tests/install_journey.rs"
review_set: subset
---

## Summary

Manual acceptance-to-test correspondence for the new QSL-2479 current-main local-source install journey. The dispatch explicitly limits this review to the PR diff and prohibits matrix, build and test reruns; this is not a repository-wide assurance audit.

## Verdict

**PASS** — the available install/version/check/sample acceptance in this slice has real failing assertions and actual public process execution. This does not assert release-asset coverage or completion of the wider hardening program.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

- Reconciliation: manual diff-scoped acceptance-to-assertion check; no computed matrix was run, and no matrix totals are claimed.
- Plan completion: not assessed
- The authoritative README command is selected at runtime, malformed or missing installation instructions produce a clear error, and Cargo must succeed and produce the installed fsl.
- Version must match the authoritative package version; checking must succeed without diagnostics; the original sample must complete with integer 42.
- A malformed original source must refuse with exit 20 and invalid_syntax/unexpected-token. No stand-in replaces FSL or Cargo.
- Reported focused live execution and scoped Clippy results are accepted. The explicit ignored live lane is not represented as ordinary/default CI execution.
- Existing release and revision-pinned install lanes are unchanged. No release workflow, old exclusion recheck, source-model-authoring feature or matrix-only gate was performed.
- Known QSL-2640 packaging work remains separate; no new duplicate ticket, owner request or priority change was made.
- No unowned changed production behavior, stubs, count-only gate or inflated passing claim was found. Specification/plan reviews do not apply.

## Examined Scope

```json
{
  "clean": true,
  "scope": [
    {
      "id": "QSL-2479",
      "path": "Linear QSL-2479",
      "role": "context_only",
      "excerpt": "Outcome: a test (container or clean temp prefix) installs the released fsl asset exactly as the docs say, then runs version and a check on a sample."
    },
    {
      "id": "current-main-target",
      "path": "Current user execution instruction",
      "role": "context_only",
      "excerpt": "then L4 (QSL-2479-2483) against FSL main"
    },
    {
      "id": "local-install-command",
      "path": "filament-driver/README.md:33-38",
      "role": "examined",
      "excerpt": "cargo install --path fsl-cli --locked"
    },
    {
      "id": "native-install-and-empty-prefix",
      "path": "tools/fsl-examples/tests/install_journey.rs:108-157",
      "role": "examined",
      "excerpt": "    install\n        .args(&arguments[1..])\n        .args([\"--debug\", \"--root\"])\n        .arg(&prefix)"
    },
    {
      "id": "isolated-installed-version",
      "path": "tools/fsl-examples/tests/install_journey.rs:69-77,158-170",
      "role": "examined",
      "excerpt": "assert_eq!(String::from_utf8(version.stdout)?.trim(), expected_version);"
    },
    {
      "id": "original-program-check",
      "path": "tools/fsl-examples/tests/install_journey.rs:172-190",
      "role": "examined",
      "excerpt": "let checked = outcome(&checked, \"check\", \"success\")?;"
    },
    {
      "id": "actual-integer-result",
      "path": "tools/fsl-examples/tests/install_journey.rs:191-211",
      "role": "examined",
      "excerpt": "json!({\"kind\": \"completed\", \"value\": {\"integer\": \"42\"}}),"
    },
    {
      "id": "malformed-source-refusal",
      "path": "tools/fsl-examples/tests/install_journey.rs:213-233",
      "role": "examined",
      "excerpt": ".any(|d| d[\"code\"] == \"invalid_syntax\" && d[\"cause\"] == \"unexpected-token\"),"
    },
    {
      "id": "explicit-live-lane",
      "path": "tools/fsl-examples/tests/install_journey.rs:1-6,94-99",
      "role": "examined",
      "excerpt": "#[ignore = \"QSL-2479 native install journey: requires current driver checkout and its warm target\"]"
    }
  ],
  "findings": [],
  "bindings": []
}
```

