---
id: SR-1525
title: "QSL-2490 PR 110 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-examples; QSL-2490; PR 110 head b8dfba61aad6e399703a52900120b667ce298e5c; agent-evals/"
review_set: subset
relationships:
  - { target: "ix://agent-ix/filament-examples/FR-001", type: references }
---

## Summary

Ticket: QSL-2490, "Model a protocol and check deadlock-freedom", in project FSL L5: agent journey evals.

- **Outcome:** a scenario with a task prompt, a sandbox, deterministic pass criteria and a scorecard row.
- **Acceptance:** runs for codex and claude, with every failure classified and ticketed.
- **Readiness sweep:** the fixture is the Fill protocol from FR-211-AC-2/3, "described in English". The pass criteria accept "the intended terminal declaration or the precondition fix".

This audit checked the diff against those terms and against the agent-evals spec.

## Verdict

**FAIL.** No test binds to an agent-evals criterion, and no agent-evals requirement owns EV-006. The scenario also departs from the ticket: the protocol is given as source, not described in English. And no live runs exist, which is correctly not claimed anywhere.

Plan completion: not assessed

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Reverse gap: no agent-evals FR or AC specifies EV-006. FR-001 to FR-003 do not name it, and this PR changes no spec. The new tests are tagged `#[trace("TC-656", "FR-211-AC-2")]`, which is the FSL repository's own language criterion and test case. These grader tests run no `fsl`, so tagging them to FR-211-AC-2 claims FSL coverage they do not provide. The proof test is also tagged AC-2, although "basis Exhaustive" is AC-3's wording. Add an agent-evals criterion for EV-006 and bind the tests to it. | agent-evals/src/scenarios/protocol_deadlock.rs:392,428,437 |
| FND-002 | medium | The scenario does not do what the ticket asks. The ticket title and readiness ask the agent to model the Fill protocol from an English description. Here the full protocol source is supplied, and the agent only adds one dictated line. The precondition-fix route the readiness note allows is refused by the exact-source check. | agent-evals/fixtures/EV-006/task.md:1-31; agent-evals/fixtures/EV-006/given.native |
| FND-003 | medium | Acceptance is not met yet: there are no codex or claude runs, no scorecard row and no classified failures. Nothing claims runs, and the author's ticket comment says no live run was made, which is correct. The release protocol-analysis reference lane was not run either, so whether the protocol route is reachable is unknown. Per the readiness note, an unreachable route must be classified as a CLI gap or FSL bug and ticketed. QSL-2490 cannot close on this PR. | agent-evals/fixtures/EV-006/README.md:23-25 |

## Coverage

- **Computed matrix:** I did not re-run it. Agent-evals criteria FR-001 to FR-003 have no binding from this PR's tests, which are tagged only to the FSL ids FR-211-AC-2 and TC-656. Those ids live in filament-spec-language: TC-656 is `spec/test-cases/TC-656-protocol-terminal-states-and-deadlocks.md`.
- **Stubs:** none. `prepare` and `grade` are implemented.
- **Leak check:** no reference file ships into the workspace, but the expected outputs are in `task.md` (SR-1524 FND-002).
- **Scorecard and run claims:** none in the diff.
- **Semantic review:** not requested.
- Plan completion: not assessed

## Examined units

- QSL-2490 outcome and acceptance: examined.
- QSL-2490 readiness sweep pass criteria: examined.
- FSL FR-211-AC-2 and FR-211-AC-3 (filament-spec-language origin/main): context_only.
- agent-evals FR-001 to FR-003 (no EV-006 mention): examined.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | still-open | The false FR-211-AC-2/TC-656 trace tags and the ix-trace-rs dev-dependency were removed, so no test claims FSL coverage any more. But still no agent-evals FR or AC specifies EV-006, and the six new tests bind to no criterion. Add an EV-006 criterion and tag the tests to it. |
| FND-002 | still-open | The task now asks the agent to model Fill from English and allows either repair. But the grader requires the baseline to match the hidden reference text (SR-1524 FND-006), so modelling from English cannot pass. |
| FND-003 | deferred | Live codex and claude runs are post-merge acceptance work on QSL-2490 itself. Nothing in the diff claims a run, and QSL-2490 must stay open until N=3 runs per agent are scored and every failure is classified and ticketed. |
| FND-001 | still-open | Round 2: the diff still changes no `agent-evals/spec` file. No agent-evals criterion specifies EV-006, and the seven protocol_deadlock tests carry no trace tag. |
| FND-002 | still-open | Round 2: the agent now models Fill from English, and the grader analyzes the agent's own model. But the grader still requires unstated reference identifiers (`Main`, `Both`, `left`, `right`, `TwoPre`; SR-1524 FND-006), and accepts any deadlock-free `protocol.native` as the repair (SR-1524 FND-010). |
| FND-001 | still-open | Round 3: still no `agent-evals/spec` change. No agent-evals criterion specifies EV-006, and the eight protocol_deadlock tests carry no trace tag. Add the criterion in this PR, or file a ticket owning it and record a deferral. |
| FND-002 | fixed | Round 3: the agent models Fill from English with its own names. The grader checks the counterexample structurally without reference identifiers, the task states the result schema, and both allowed repairs pass while structural wrong repairs fail. Vacuous repairs remain as SR-1524 FND-012. |
| FND-001 | deferred | QSL-3018 (FSL L5: agent journey evals, milestone Scenario triage, related to QSL-2490) owns it spec-first: author an agent-evals requirement and test case for EV-006, then tag the eight tests, keeping them separate from FR-211 conformance. The EV-006 README links it. This is an acceptable owner. |
