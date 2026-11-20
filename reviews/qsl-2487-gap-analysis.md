---
id: SR-1442
title: "Gap analysis of EV-003 safety analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-examples; QSL-2487; 6fcc655..aaf1467; EV-003 task, fixtures, independent grader and provider registration"
review_set: subset
---

## Summary

Compared the requested safety-property/proof/counterexample journey with the new task, fixed original model, public analyzer assertions and eight accepted tests. No additional semantic or artifact-integrity gap was found beyond the subprocess-budget defect recorded in SR-1441.

## Verdict

**CONDITIONAL** — the actual Codex task's semantics passed; retained command/boundary acceptance and current-code gates remain pending. SR-1441 FND-001 and FND-002 are fixed. Claude task acceptance remains pending usable authentication, not an FSL or agent defect.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Plan completion: not assessed

This is a diff-scoped gap analysis, not a repository-wide matrix audit. No matrix, tests or accepted reference runs were repeated. Scope examined:

- EV-003 fixed inputs and requested properties: byte-preserved model/state/requests, unchanged transition prefix, explicit property/binder/clock constraints and rejection of tautologies or changed input.
- EV-003 proof: independently invoked public analyzer must return an unrefused certified exhaustive proof; saved outcome semantics and proof identity bind to the current submitted claim rather than a stale certificate. Harmless timing metadata is not a grading input.
- EV-003 refutation: independently invoked public analyzer must refute TooSmall, identify meter's finite prefix and report levels [0,1,2,3], position3 and violating level3. A bounded state domain allows4, so the holding property tests reachability rather than the scalar bound alone.
- EV-003 result interpretation: exact closed result shape, required verdicts/exits and counterexample values; malformed JSON, extra fields and invented summaries reject.
- FR-001-AC-2 integration: catalogue entry, module registration and preparation expose the new scenario. Its two required analyze operands are registered for observed-command checks.
- Evidence tests cover absent/wrong/echoed commands, wrong or stale certificate, incomplete results and artifact mutations. The supplied original model is problem data; full native-domain authoring is not claimed.

At the initial review, actual Codex task acceptance had not yet run. The subsequent actual Codex task and bounded runtime follow-up are recorded below. Claude's known authentication prerequisite remains pending; this review neither retries it nor records a failed or passed Claude task. Shared framework behavior already reviewed in PR62 is excluded except for the explicitly assigned actual-runtime fix. Semantic inspection was the assigned review scope; no additional optional semantic-review campaign or whole-repository assurance claim was added.

## Disposition evidence

The bounded-runtime gap recorded separately in SR-1441 is fixed at 8f5c0a25. Accepted coder evidence consists of three new short-budget tests covering terminated/reaped child, both output streams beyond pipe capacity, and exhausted shared budget preventing another spawn with harness-error classification. Original reference/evidence checks were not repeated; no additional gap was found in this focused fix.

Runtime follow-up 23580ac..3e5df881 accepted the reported single Codex task's semantic results: proved/refuted claims, current counterexample levels [0,1,2,3], fixed-input binding and saved outcomes all passed. Original SDK 0/1 is preserved. The command/boundary fix adds valid stdout-capture/status handling and regex-data classification; its combined-option path defect is recorded only as SR-1441 FND-002. No additional semantic gap was found, and no accepted checks or live runs were repeated.

Focused follow-up 3e5df881..76b1c21e fixes FND-002 by rejecting unresolved ripgrep option arity rather than assigning its next operand a regex role. Accepted evidence is one new affected test covering rejection of combined/unknown option arity and permission for the known actual pattern forms. The original task semantics and prior tests were not rerun; the next authorized action is command/boundary-only retained grading, not a new agent sample.
