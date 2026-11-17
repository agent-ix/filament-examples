---
id: SR-006
title: "Code review — QSL-2351 selected example records"
type: SpecReview
analysis: code-review
scope: "filament-examples; selected expected records, system manifests, index.json, QSL-2351 and QSL-2372 context"
review_set: subset
---

## Summary

Reviewed PR #42 at `d795441f1919ac58c1232ad3567abad616625863`, including the selected record diff, pass-three ticket context, PR description, and recorded runner evidence. The 83 changed case records match the planner-authorized non-FR-865 batch, and the temporary selected-manifest checks pass.

## Verdict

**PASS** — the records, staged expectations, and manifest wait-on values match the requested batch; FR-865-dependent cases are excluded.

## Findings

| ID      | Severity | Summary                   | Refs |
| ------- | -------- | ------------------------- | ---- |
| FND-001 | low      | No findings (placeholder) | -    |

## Review Notes

- Manual acceptance-to-record mapping: 63 value-shape records, 13 item/witness/limit records, three first-diagnostic records, and four staged clock records; the four staged cases use exit 21 and wait on QSL-2240.
- The changed case records are exactly 83 JSON outputs. The additional changed paths are the two system manifests, the generated index, and message snapshots for the diagnostic and staged-clock cases.
- The PR body cites QSL-2372 for the whole-directory runner limitation and states that checks used a temporary manifest containing only the selected cases.
- Focused evidence from the PR logs: record 83/83, check 79 passed/0 failed, staged 4 passed/0 failed, `fsl-examples readme` exited 0, and `git diff --check` exited 0. The driver binary was built from `895d2f727c4236b36ae48087d91fb14a95de4401`.
- No production code or specification files changed, so Rust review and Quoin gap analysis were not applicable. The record-level gap check was manual against QSL-2351's pass-three inventory.

## Review Data

```yaml
clean: true
scope:
  - id: QSL-2351
    path: Linear/QSL-2351
    role: examined
    excerpt: "QSL-2351 refreshes pass-two output shape drift and resolves mod-by-zero cases; the pass-three comment adds value shapes, item drift, unknown-name diagnostics, and staged clock records."
  - id: QSL-2351-pass3
    path: Linear/QSL-2351/comments
    role: examined
    excerpt: "Pass 3 (driver main 895d2f7): 13 item cases add witnesses or bound/counter/field limit causes; four staged clock cases now give exit 21 unsupported_construct/declaration-form; two unknown-name cases now give missing_declaration/missing-name."
  - id: PR-42
    path: https://github.com/agent-ix/filament-examples/pull/42
    role: examined
    excerpt: "The first split contains 83 non-FR-865 records and checks a temporary selected-case manifest because the runner only accepts whole system directories."
findings: []
bindings: []
```
