---
id: SR-1446
title: "EV-009 acceptance and test gap analysis"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/filament-examples; QSL-2493; EV-009 task, manifest, original fixtures, blind_help.rs and new registration"
---

## Summary

Mapped the four runnable EV-009 tasks to their immutable inputs, independent public command checks, semantic artifact comparisons and new tests. Export remains a required pending program facet rather than a successful refusal or a fifth passed verb. The monitor oracle does not yet verify the complete saved event stream required by the task.

## Verdict

**CONDITIONAL** — FND-001 is fixed at c716e73489d6a3e5ced43d2e47687a1a7b4c558c; only authorized live acceptance and the known provider prerequisites remain pending. No full-program completion or Claude task result is claimed.

Plan completion: not assessed

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | The task requires the monitor's complete output stream, but `monitor_result` reduces it to the first pending event, one decisive event and selected summary fields. Duplicating the first pending event, or removing/changing a non-first pending event, leaves the returned semantic value unchanged; `grade_case` can therefore accept a saved stream that differs from the real command output. Compare the ordered stable event content/completeness as well as the decisive verdict, and cover an omitted, added or altered pending event. | agent-evals/fixtures/EV-009/task.md:8; agent-evals/src/scenarios/blind_help.rs:190; agent-evals/src/scenarios/blind_help.rs:283; agent-evals/src/scenarios/blind_help.rs:407 | correct-requirement-no-evidence |

## Coverage

The check task rejects a refusal in place of successful checking; run requires the original quote of 29; analyze requires the original certified exhaustive proof and preserves claim, subject and certificate identity in the comparison; monitor verifies the expected violation, evaluation origin, observed position, binding and authoritative closure. Immutable source, request, domain and trace files are checked before public execution, including rejection of symlinked inputs. The provider entry disables shipped references and requires all four public commands. Every active per-verb result must pass; required export remains pending QSL-1979, `program_status` is partial and `overall_completion` is pending.

Examined all new fixtures, the task and manifest, README instructions, the five new test bodies, the scenario grader and the new module/provider registration. The deadline and trace-binding issues are recorded once in SR-1445 and are not duplicated here.

The author reports five new grader checks and integrated all-target Clippy passing. Those results are accepted. Live Codex acceptance is pending review; Claude remains pending the known usable-authentication prerequisite. No matrix, repository-wide audit, test, build, reference run, doctor, authentication probe or old canary was executed: the user's diff-only and no-remeasurement instructions supersede the gap skill's broader default workflow.

## Review context

Ticket: QSL-2493. Repository: agent-ix/filament-examples, public. PR: pending. Frozen feature: 1494e6b86fd9191325e4e61d74ffb77e5afd70b9. Reviewer run: f58865ae-df7e-4046-877b-64e6c29a586b. Exact model identifier is unavailable. Scope is this feature's acceptance, not a repository-wide coverage or plan-completion claim.

## Dispositions

### Round 1

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | `monitor_result` now returns the entire ordered event array and summary as parsed JSON after its semantic checks. `grade_case` therefore compares every event and summary field between the saved artifact and the independent public result, ignoring JSON whitespace/key order rather than discarding intermediate stream data. New adversarial checks cover omitted, inserted and altered pending events. |

The two reported new complete-stream tests are accepted without rerunning them. Four active verb contracts still must all pass; the required export oracle remains pending QSL-1979 with program completion explicitly pending. No shared-helper re-audit, new matrix, public CLI probe, live run or broader acceptance sweep was performed during disposition.
