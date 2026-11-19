---
id: SR-1425
title: "Code and Rust review of the current-main clean-prefix install journey"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-examples; QSL-2479; PR 57; tools/fsl-examples/tests/install_journey.rs"
review_set: subset
---

## Summary

Reviewed the sole new 234-line integration module at frozen head a25fef3d1c3c83b658f9010a69e40aace5df19ba. The test runs the driver's authoritative local-checkout Cargo command into a new prefix and exercises that installed executable without ambient runtime configuration.

## Verdict

**PASS** — no code or Rust findings in the PR diff. The lead-owned final applicable gate remains to be run; this review does not claim a full repository or release-asset gate pass.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

The installer is real Cargo, not a fake installer or a copied executable. The README supplies the install argv; test-only root and debug-profile arguments select an isolated destination and reuse the accepted current-main build. The expected version is read from the authoritative package manifest. Each installed-binary invocation uses its absolute path with a cleared child environment. The original model-free program checks and computes integer 42, and malformed source must produce the specified exit, category and diagnostic.

No production API, model, schema, binary, external fixture or documentation snapshot is copied into this repository. The temporary prefix is the requested installed-user environment. Existing ProcessDriver does not supply this test's cleared-environment installer seam; shared support outcome_document is a stand-in serializer, not an equivalent installed-process decoder. No applicable AssuranceProfile, local Rust idiom override, CI workflow change or narrowed gate occurs in the reviewed scope.

Accepted the reported focused live pass and scoped Clippy pass, including the 1.06-second warm installation, without rerunning tests, builds or inspecting installed artifacts. The live test has an explicit named ignore lane and required inputs; it is not claimed as default-test coverage. Known packaging defect QSL-2640 remains separately owned and is neither fixed nor claimed by this PR. Release downloads/checksums and bare-container prerequisites remain outside this current-main local-source slice.

Rust style, error handling, borrowing, deterministic scratch resources, lifecycle, public-process boundaries and meaningful assertions were examined. There is no unsafe or async code. Specification/plan review is inapplicable because neither changed. Plan completion: not assessed.

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
    },
    {
      "id": "existing-install-scope",
      "path": "install-test/steps.md",
      "role": "context_only",
      "excerpt": "The FSL driver comes from a prebuilt release for Linux on x86_64 and arm64 and for macOS on Apple silicon and Intel. A second path builds the driver from source."
    }
  ],
  "findings": [],
  "bindings": []
}
```

