---
id: SR-003
title: "acceptance-criteria check of the Rust example runner (QSL-2099)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-examples; tools/fsl-examples/tests/**, tools/fsl-examples/src/**"
review_set: subset
---

## Summary

Ticket: QSL-2099. The repository has no spec tree, so quoin gap-analysis has no requirements to read; this is the manual AC-to-test check the reviewer role requires instead. Each ticket AC was mapped to its named test and the test body read: manifest.rs::parses_every_example_toml (282 running, 21 staged, compared to the run.mjs-written index.json), runner.rs::check_passes_on_all_running_cases, staged_fails_when_a_case_reaches_ready_exit, staged_fails_when_a_staged_case_changes, stale_index_or_readme_region_fails, record_refreshes_the_message_snapshot_and_keeps_the_structure, compare.rs::reworded_message_still_passes and the seven negative cases, manifest.rs::staged_without_waits_on_is_refused. run.mjs and run.test.mjs are deleted; the other tools/*.test.mjs stay as a documented deviation, which is correct because those tools are still used.

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | low      | No findings (placeholder) | - |
