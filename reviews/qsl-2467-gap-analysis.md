---
id: SR-021
title: "QSL-2467 examples gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-examples; QSL-2467 PR #56 at 6ffe7f7c03cba791ce54b4459448fcafa61fa853; ticket-scoped Rover timed examples"
review_set: subset
---

## Summary

A ticket-scoped manual gap analysis checked the three new Rover timed-analysis cases against QSL-2467 and FSL FR-239/FR-632. Each request, case registration, expected output and index entry is present and matches the ticket's EN-6 outcome requirement; the repository-wide engine-backed traceability matrix could not run because this repository has no local `spec/` document root.

## Verdict

**CONDITIONAL** — no gap was found in the three changed cases, but Quoin could not produce a repository matrix, so this is not a repository-wide traceability pass.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Plan completion: not assessed

Quoin gap-analysis was invoked. `quoin matrix --repo /Users/peter/dev/qsl-2467-examples --json` exited 2 with `CORE_REFUSED` / `QQ-1002`: no document root at `/Users/peter/dev/qsl-2467-examples/spec`. No engine-backed matrix, repository-wide tagged-test sweep, or repository-wide source reverse-gap/stub audit is claimed. The code change adds example data and descriptions, not product implementation source; the changed cases were checked manually against the ticket and the owning FSL requirements.

FR-239 requires EN-6 to advertise bounded and unbounded temporal-satisfaction for timed subjects. FR-632 routes timed `native-analyze/1` items through the zone engine. The three requests select timed items on `model-time`; the expected records use backend `en-6`, with bounded hold `proved`, bounded refutation `refuted`, and the liveness output carrying `non_zeno`. The case table marks all three as running with exits 0, 10 and 0; `index.json` contains matching entries. Accepted coder evidence: all three outputs were recorded and the focused runner check passed 3/3. No aggregate gate was run.

Semantic review was not requested and was skipped. The ticket's review of `FR-239` and `FR-632` was limited to these three cases; no local specification criteria exist in this repository.

## Examined units

```yaml
clean: true
scope:
  - {id: FR-239, path: "filament-spec-language/spec/functional/FR-239-check-a-timed-claim-by-symbolic-zone-search.md", role: context_only, excerpt: "EN-6 registers a provider manifest advertising (temporal-satisfaction, bounded) and (temporal-satisfaction, unbounded) for timed subjects."}
  - {id: FR-632, path: "filament-spec-language/spec/functional/FR-632-request-timed-zone-items.md", role: examined, excerpt: "A native-analyze/1 item over a timed subject SHALL be decided by the zone engine."}
  - {id: timed-zone-deadline-holds, path: "examples/rover/example.toml", role: examined, excerpt: "Running timed model-time analyze case; expects holds and exit 0."}
  - {id: timed-zone-deadline-refuted, path: "examples/rover/example.toml", role: examined, excerpt: "Running timed model-time analyze case; expects violated and exit 10."}
  - {id: timed-liveness-holds, path: "examples/rover/example.toml", role: examined, excerpt: "Running timed model-time analyze case; expects holds and exit 0."}
  - {id: timed-analyze-requests, path: "examples/rover/requests/analyze/timed-*.json", role: examined, excerpt: "Each request selects its intended temporal claim and includes the correct starting snapshot as the initial state and universe."}
  - {id: timed-analyze-expected, path: "examples/rover/expected/timed-*.json", role: examined, excerpt: "Expected outputs record EN-6 outcomes; the liveness result includes the non_zeno member."}
  - {id: rover-index, path: "index.json", role: examined, excerpt: "The three new cases are indexed with the same request, expected-output, engine and verdict data as example.toml."}
findings: []
```
