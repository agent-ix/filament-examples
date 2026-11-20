---
id: SR-1441
title: "Code review of EV-003 safety analysis"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-examples; QSL-2487; 6fcc655..aaf1467; agent-evals/src/scenarios/safety_analyze.rs, agent-evals/src/provider.rs registration, agent-evals/src/lib.rs registration, agent-evals/fixtures/EV-003/"
review_set: subset
---

## Summary

Reviewed the new finite-safety agent scenario, its independent public-analyzer grader, original problem fixtures and provider registration. The subprocess-budget defect identified in the initial review is fixed at 8f5c0a25. The bounded actual-runtime ripgrep argument-classification defect is fixed at 76b1c21e. Shared provider implementation outside the assigned runtime diff remains excluded.

## Verdict

**PASS** — FND-001 and FND-002 are fixed. Unknown or combined ripgrep option arity now fails closed; the known actual pattern forms remain usable. Retained command/boundary grading and current-code gates remain pending.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | EV-003 declares a 30000 ms assertion budget, but both trusted analyzer invocations use blocking Command::output with no deadline or cancellation. A slow or stuck analysis exceeds the scenario budget; the suite timeout can kill only the provider while its analyzer remains running, instead of returning the required harness-error budget failure and reaping the child. Bound both invocations by one assertion deadline and observe child termination. | agent-evals/src/scenarios/safety_analyze.rs:330; agent-evals/fixtures/EV-003/scenario.toml:4 |

## Scope

- Examined EV-003 task: preserve the supplied model/start/requests/transition contract; append exactly the two named safety properties; run both public analyses; retain complete outcomes and interpret the counterexample in the closed result shape.
- Examined provider EV-003 entry and source module registration against FR-001-AC-2's active-scenario catalogue behavior.
- Examined FR-001's provider-time-budget decision: "The provider stops its own child commands (`fsl`, `quoin`, `quire`) when the budget runs out and returns a failed assertion whose failure starts `harness-error: assert budget exceeded`, so a slow tool is classified and never read as an agent failure."
- Inspected original reference model/source and emitted semantic model as this repository's user-problem fixture, not an agent-authored native domain or copied upstream artifact.
- Examined regular-file/size checks, unchanged inputs, exact requested-property constraints stated in the prompt, public executable selection, terminal proof/refutation checks, current-claim certificate identity, counterexample levels and summary parsing.
- Examined all eight new tests and accepted the author's reported passing results. Tests exercise nonproof/uncertified outcomes, changed inputs/tautology, false counterexamples/summaries, semantic certificate binding and command evidence. The explicit public-CLI reference lane remains clearly separate from synthetic transcript units.

## Evidence and limits

No tests, builds, public analyzer runs, matrix measurements, SDK probes or live agents were rerun. The previously accepted six grader/reference checks and two evidence checks are reused. Independent code and Rust review ran over the new-feature diff only. Live Codex acceptance remains pending; Claude authentication is an accepted external prerequisite, with zero task samples. No claim is made that the complete repository or unmodified framework was reviewed. The exact model identifier was not exposed to this reviewer; tracker markers record it as unavailable rather than inventing one.

## Dispositions

Round 1 inspected only aaf1467..8f5c0a25 for FND-001. One absolute deadline starts before artifact validation and is reused for both analyses and the final semantic checks. The capture helper checks expiry before spawning, uses file-backed output, and kills and waits for its child on expiry. A timeout returns the required `harness-error: assert budget exceeded` prefix. The three new focused deadline tests reported by the coder passed; no tests or other checks were rerun for disposition.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Shared 30-second deadline, expired-budget spawn prevention, file-backed output and owned-child termination/reaping are implemented; timeout classification is a harness error. |
| FND-002 | still-open | Combined ripgrep options such as -nf can consume an external pattern-file path, but the new classifier suppresses that operand as regex data. |
| FND-002 | fixed | Unknown or combined option arity returns no path-role interpretation and produces an explicit harness-error, so an external pattern-file operand cannot be silently exempted as regex data. |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | The new ripgrep classifier treats unrecognized combined options as value-free. For `rg -nf /outside/patterns docs`, ripgrep interprets -n plus -f and reads /outside/patterns, but rg_path_operands skips -nf then hides /outside/patterns as the implicit regex. This bypasses the evaluator/source-file access guard introduced by this fix. Recognize the relevant combined file option or conservatively retain path checks when option arity is unknown. | agent-evals/src/provider.rs:796-803 |

Runtime follow-up inspected only 23580ac..3e5df881 for PR65. Literal stdout redirection, exit-status captures and earlier unconditional calls are recognized without treating redirect targets, heredoc bodies, control flow or unreachable text as input invocations. Regex arguments are distinguished from direct -f/--file/--pre paths, but the combined-option edge above remains. The supplied one-line Mac lipo operand-order correction was also inspected; no additional method or sweep was added.

Accepted runtime evidence: one Codex task, 14 commands and 74876 ms; proof/refutation, fixed inputs, current counterexample and saved outcomes passed. The original SDK 0/1 remains recorded, and command/boundary-only retained grading is pending. Three new focused tests reportedly passed; no tests, analyzers, semantic checks, builds, retained grades or live agents were rerun by the reviewer.

Round 3 inspected only 3e5df881..76b1c21e for FND-002. `rg_path_operands` now returns `None` for unknown/combined option arity or a missing option argument, and the caller records `harness-error: unsupported rg option arity in boundary evidence`. Known -n/-L and simple pattern arguments retain their intended roles; explicit file/preprocessor operands retain path checks. The one new affected-arity test reportedly passed. No tests, prior semantic/reference checks, retained grades, builds, Clippy, Mac checks or agent runs were repeated.
