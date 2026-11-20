---
id: SR-1437
title: "QSL-2483 Mac main-build code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-examples; QSL-2483; .github/workflows/install-test.yml, tools/macos-main-smoke.sh, tools/README.md; frozen 9cfac97"
review_set: subset
---

## Summary

Independent review of the frozen main-build Mac workflow feature. The script selects an explicit coherent source graph, builds one native driver, and reuses the existing real platform oracle. One runner-capacity mismatch needs resolution before relying on the new route.

## Verdict

**CONDITIONAL** — The selected standard runner does not provide the script's documented minimum storage budget. Native Mac execution remains pending; no platform pass is claimed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The script requires at least 20 GiB free, while the selected standard macos-15 runner is documented with 14 GB storage. A runner satisfying that published specification exits before checkout/build, so this workflow does not provide a capacity-compatible route to the promised native test. Select an available runner with the required capacity, or explicitly resolve the remote runner's build-space policy and budget before dispatch. Do not silently relax the user's local disk rule or count a preflight refusal as a platform pass. | tools/macos-main-smoke.sh:17; .github/workflows/install-test.yml:80 |

## Coverage

Reviewed all three changed files against origin/main. The existing platform_smoke test was read as the invoked oracle, not changed or rerun. The accepted shared build receipt/config was used as build-input context without repeating its build, hashes, or tests. No applicable AssuranceProfile was supplied for this scoped workflow feature.

Clean examined behavior: native Darwin-arm64 guard; explicit driver/language/codegen/IR refs; authoritative repositories cloned into an owned runner temporary directory; native Cargo path patches matching the accepted graph; one driver build; reuse of that binary and selected driver's README; exact ignored platform test with one thread; pipefail propagation; owned cleanup; logs retained outside cleanup; manual dispatch choices; no published release/tag, copied binary/model/schema, secret output or private dependency workaround. Existing release matrix remains an ordinary explicit workflow choice.

Storage basis: [GitHub's standard public runner specifications](https://docs.github.com/en/actions/reference/runners/github-hosted-runners#standard-github-hosted-runners-for-public-repositories) document macos-15 arm64 with 14 GB SSD. This is a capacity-contract finding, not a measurement of an actual runner; a runner may have more space, but the workflow cannot rely on more than its selected class provides.

Accepted authoring evidence: bash syntax check and git diff check passed. No tests, builds, gates, probes, matrix, Linux journeys, or native Mac runs were repeated. No Rust files changed, so the Rust review lane does not apply. Workflow authorization came directly from Peter's session reply.

Plan completion: not assessed

## Examined units

```json
[
  {"id":"mac-main-selection","path":".github/workflows/install-test.yml","role":"examined","excerpt":"options: [macos-main, install-matrix]"},
  {"id":"mac-main-runner","path":".github/workflows/install-test.yml","role":"examined","excerpt":"runs-on: macos-15"},
  {"id":"mac-main-capacity","path":"tools/macos-main-smoke.sh","role":"examined","excerpt":"if (( free_kib < 20 * 1024 * 1024 )); then\n  echo 'Native build blocked: less than 20 GiB is free on the runner.' >&2\n  exit 1\nfi"},
  {"id":"mac-main-native","path":"tools/macos-main-smoke.sh","role":"examined","excerpt":"if [[ \"$(uname -s)-$(uname -m)\" != Darwin-arm64 ]]; then"},
  {"id":"mac-main-inputs","path":"tools/macos-main-smoke.sh","role":"examined","excerpt":"checkout filament-driver \"$DRIVER_REF\" \"$source_root/driver\"\ncheckout filament-spec-language \"$LANGUAGE_REF\" \"$source_root/language\"\ncheckout filament-contract-codegen \"$CODEGEN_REF\" \"$source_root/codegen\"\ncheckout filament-contract-ir \"$IR_REF\" \"$source_root/ir\""},
  {"id":"mac-main-build","path":"tools/macos-main-smoke.sh","role":"examined","excerpt":"cargo build --config \"$config\" -p fsl-cli --bin fsl"},
  {"id":"mac-main-cleanup","path":"tools/macos-main-smoke.sh","role":"examined","excerpt":"trap 'rm -rf \"$source_root\"' EXIT"},
  {"id":"mac-main-smoke","path":"tools/macos-main-smoke.sh","role":"examined","excerpt":"--test platform_smoke quickstart_runs_on_the_selected_native_platform"},
  {"id":"mac-main-usage","path":"tools/README.md","role":"examined","excerpt":"A failed checkout or build\nfails the job; it cannot count as a platform pass."},
  {"id":"existing-platform-oracle","path":"tools/fsl-examples/tests/platform_smoke.rs","role":"context_only","excerpt":"fn quickstart_runs_on_the_selected_native_platform() -> TestResult {"}
]
```
