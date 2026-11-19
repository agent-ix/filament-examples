---
id: SR-018
title: "spec-review of QSL-2494 scheduled runs, history, scorecard and triage (FR-002, TC-002)"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-examples; PR #55 at 87769de91cc8b526185cb8f6f8286b2619b610e2; agent-evals/spec/functional/FR-002-scheduled-runs-scorecard-and-triage.md, agent-evals/spec/test-cases/TC-002-scheduled-runs-scorecard-and-triage.md"
review_set: subset
---

## Summary

Ticket: QSL-2494. Spec review of FR-002 (the weekly schedule, committed run history, scorecard and triage) and TC-002 in PR #55. The report claims were checked against `src/report.ts` and `src/types.ts` of `@agent-ix/cli-agent-evals` 0.3.0. filament-examples is a public repository, and CI there is manual-only (`.github/workflows/check.yml` is `workflow_dispatch`). `quire validate` exits 0.

## Examined scope

- Report contract: `reportVersion`, `model`, `repeats`, `results[].passRate`, `aggregate.toolCalls.p50` (null when unavailable), `aggregate.latencyMs.p50` and `runs[].workDir` all exist in `cli-agent-evals.report/v1` as FR-002 uses them. (examined)
- Schedule: the launchd `StartCalendarInterval` weekday 1 / hour 3 and the crontab `0 3 * * 1` both mean Monday 03:00. A per-user LaunchAgent runs only in a logged-in session, which FR-002 assumes. Installing replaces the entry and does not add one, which is testable. (examined)
- The scheduled run's git handling and the PR-per-run loop: FND-001. (examined)
- Redaction and the public repository: FND-002. (examined)
- Scorecard determinism and the trend rule (same agent, scenario and model id; `new` otherwise): consistent, and AC-4's golden fixture exercises up, same and new. (examined)
- Triage checklist, routing to Linear only and the `[A-Z]+-[0-9]+` key check: consistent with the standing rule that tickets live in Linear. (examined)
- FR-002-AC-1..7 and TC-002: each criterion can fail. The ids are unique. (examined)

## Verdict

Mergeable once FND-001 and FND-002 are addressed. Neither is high: the schedule, scorecard and triage design is sound. The loop does not define what happens when the previous run's PR is still open, and the merge gate has no mechanism behind it in a repository whose CI is manual-only.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | No rule for a still-open previous run PR: each run branches from origin/main, so stacked PRs all rewrite SCORECARD.md (conflicts, and trends that ignore unmerged runs), the `-2` run id computed from main collides with an unmerged same-date branch, PRs accumulate weekly, and "can merge only when triage --check and scorecard --check pass" has no enforcing mechanism | agent-evals/spec/functional/FR-002-scheduled-runs-scorecard-and-triage.md:41 |
| FND-002 | medium | Redaction replaces only the literal run root and home folder in reports: macOS /var/folders vs /private/var/folders aliases, other operator paths quoted in failure text, and triage.md (which carries the first failure) are not covered, in a public repository | agent-evals/spec/functional/FR-002-scheduled-runs-scorecard-and-triage.md:62 |
| FND-003 | low | `agent-evals clean` is used but is missing from FR-001's subcommand table and has no AC. Transcripts kept under the OS temp folder for triage may be purged by macOS temp cleanup before a slow triage | agent-evals/spec/functional/FR-002-scheduled-runs-scorecard-and-triage.md:65 |
| FND-004 | low | The schedule entry runs `cargo run --manifest-path <checkout>/agent-evals/Cargo.toml`, so the `scheduled` driver is built from whatever branch or uncommitted state the checkout has at 03:00, which contradicts "uses the harness on main" for that step | agent-evals/spec/functional/FR-002-scheduled-runs-scorecard-and-triage.md:30 |
