---
id: SR-1441
title: "Code review of EV-003 safety analysis"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-examples; QSL-2487; 6fcc655..aaf1467; agent-evals/src/scenarios/safety_analyze.rs, agent-evals/src/provider.rs registration, agent-evals/src/lib.rs registration, agent-evals/fixtures/EV-003/"
review_set: subset
---

## Summary

Reviewed the new finite-safety agent scenario, its independent public-analyzer grader, original problem fixtures and provider registration. The subprocess-budget defect identified in the initial review is fixed at 8f5c0a25. Shared provider implementation previously reviewed for PR62 is outside this review.

## Verdict

**PASS** — one shared assertion deadline now bounds both analyzer invocations; timeout terminates and reaps the owned child and is classified as a harness failure. Live acceptance remains pending.

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
