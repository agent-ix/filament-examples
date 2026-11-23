---
id: SR-1523
title: "QSL-2485 PR 109 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-examples; QSL-2485; PR 109 head 5c6839aac354d01b83219b42f9f6b13d42d94fd5; agent-evals/"
review_set: subset
relationships:
  - { target: "ix://agent-ix/filament-examples/FR-001", type: references }
  - { target: "ix://agent-ix/filament-examples/FR-002", type: references }
---

## Summary

Ticket: QSL-2485, project "FSL L5: agent journey evals". The ticket's acceptance reads: "runs for codex and claude; every failure is classified (doc gap, CLI gap, FSL bug, agent error) and ticketed in the owning project". The ticketing rules are in FR-002's triage decision: each ticket gets a project, a milestone and a `related` relation to the scenario ticket, and doc gaps get the `docs` label. This audit checked PR 109's runs and tickets against those rules and ran the computed Test Matrix for agent-evals/.

## Verdict

**FAIL** under the skill's verdict rule. Every `Test` criterion in agent-evals/ is untagged in the computed matrix (35 of 37). This is not new in this PR: the repo uses `/// Trace:` doc lines, which the engine does not bind.

Findings this PR itself introduced or left open are medium:
- Claude has no usable EV-001 result.
- The failure tickets lack the FR-002 milestone, relation and label.
- A known grader defect in two other scenarios is left as a note inside a ticket titled "fixed".

What holds:
- Each failure class is named and ticketed in the right project. Doc gaps (QSL-3005, QSL-3006) are in "FSL example findings". Harness errors (QSL-3007, QSL-3008) are in "FSL L5: agent journey evals".
- The Claude 0/3 is called a harness error and no FSL, CLI or doc result is claimed for it.

Plan completion: not assessed

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | QSL-2485's outcome asks for a scorecard row (pass rate over N runs) for codex and claude. Claude's three runs ended after 1 to 2 tool calls on a harness fault (QSL-3008, open), so there is no Claude result. The acceptance "runs for codex and claude" is met in letter only. QSL-2485 should stay open until QSL-3008 is fixed and Claude is re-run, or record explicitly that Claude acceptance moves to QSL-3008. | agent-evals/scorecards/EV-001-claude.json:78 |
| FND-002 | medium | The failure tickets QSL-3005, QSL-3006, QSL-3007 and QSL-3008 have no milestone and no labels; the doc-gap pair also lacks `docs`. None has a `related` relation to QSL-2485 or to any other ticket. FR-002's triage decision requires a project, a milestone and a `related` relation to the scenario ticket, plus the `docs` label for a doc gap. | agent-evals/spec/functional/FR-002-scheduled-runs-scorecard-and-triage.md:150-158 |
| FND-003 | medium | QSL-3007 is titled "fixed in QSL-2485 PR" but adds an open follow-up: EV-002 refusal_repair and EV-009 blind_help still build and compare `{"integer":...}` values. The new shape is `{"type":"integer","value":...}` (QSL-3006). If those graders hit the same refusal, they fail every valid model. No ticket owns that defect on its own, so it will be closed with QSL-3007. | agent-evals/src/scenarios/refusal_repair.rs:135,168; agent-evals/src/scenarios/blind_help.rs:192 |
| FND-004 | low | Pre-existing, not introduced here. The computed Test Matrix (`quire matrix --scope agent-evals`) reports 35 of 37 FR-001..FR-003 criteria `untagged` and 2 `method-without-symbol`. The repo tags tests with `/// Trace:` doc lines, which the engine reports as `trace-id-outside-marker`. The new test continues that convention, so it does not bind to FR-001-AC-6 in the matrix. | agent-evals/src/provider.rs:1525 |
| FND-005 | low | The N=3 run evidence (raw SDK reports and transcripts) is not committed and is not reachable at the paths the runner printed. FR-002 expects the scorecard to come from run folders committed under `agent-evals/reports/`. The committed `runs_n3` numbers are hand-copied and cannot be re-derived from the repo. They agree with the coder's runner logs (pass/fail and rounded latency). | agent-evals/scorecards/EV-001-codex.json:70 |

## Coverage

- Computed matrix: `quire matrix --scope agent-evals --format json` (quire 0.52.1, quoin 0.36.10). Result: 37 criteria, 35 `untagged`, 2 `method-without-symbol`. The engine warned `no-symbol-bound` for 61 Rust symbols, which use the doc-line convention.
- Reverse gap: the sed and rg boundary rules map to FR-001-AC-6. The probe maps to the EV-001 invariant check in the fixture task. TMUX_TMPDIR maps to the FR-001 run isolation. No unowned behaviour was found.
- Stubs: none in the diff.
- Classification audit against the run logs in the coder's scratchpad: codex runs5 gave 3 passes at 70 s, 62 s and 46 s. Claude runs5 gave 3 failures at 7 s each with runner exit 1. The earlier attempts (runs to runs4) were stale-tmux startup failures and the pre-fix grader failure, all recorded under `earlier_attempts` as harness-error. An earlier draft (`claude-n3.json`) had round estimated values. The committed file has the corrected values.
- Semantic review: skipped (not requested).
- Plan completion: not assessed

## Examined units

- QSL-2485 acceptance: "runs for codex and claude; every failure is classified (doc gap, CLI gap, FSL bug, agent error) and ticketed in the owning project". Examined.
- FR-002 triage decision (ticketing rules): examined.
- FR-001-AC-6 (transcript boundary): context_only. The breach finding is in SR-1522.
- QSL-3005, QSL-3006, QSL-3007, QSL-3008: examined (project, labels, milestone, relations).

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The Claude scorecard now records that QSL-2485's remaining Claude acceptance (an N=3 re-run) is owned by QSL-3008, and QSL-3008 is related to QSL-2485. |
| FND-002 | fixed | Each ticket now has its milestone and labels. QSL-3005: Usability and tooling, docs. QSL-3006: Confirmed defects, docs and defect. QSL-3007 and QSL-3008: Scenario triage, defect. All four are related to QSL-2485. |
| FND-003 | fixed | QSL-3010 was filed in FSL L5, milestone Scenario triage, related to QSL-3007. It owns the EV-002/EV-009 value-shape defect and was re-measured: fsl run refuses EV-009's request with wrong-value-kind. |
| FND-004 | deferred | This is a repo-wide convention that predates this PR: every agent-evals test uses `/// Trace:` doc lines. Moving to #[trace] attributes needs an ix-trace-rs dependency and touches every test, which is out of scope for an EV-001 run PR. No owning ticket exists yet; one should be filed in FSL L5. |
| FND-005 | deferred | Raw SDK reports and transcripts are excluded from commits on purpose (scorecard note: they may hold workstation paths and credentials). FR-002's committed run folders and the `agent-evals scorecard` command are not implemented yet (main.rs has no scorecard subcommand), so this belongs with that work, not this PR. The hand-copied numbers agree with the runner logs. |
