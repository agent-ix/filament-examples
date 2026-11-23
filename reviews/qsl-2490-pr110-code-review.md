---
id: SR-1524
title: "QSL-2490 PR 110 code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-examples; QSL-2490; PR 110 head b8dfba61aad6e399703a52900120b667ce298e5c; agent-evals/src/scenarios/protocol_deadlock.rs, agent-evals/src/provider.rs, agent-evals/src/lib.rs, agent-evals/Cargo.toml, agent-evals/Cargo.lock, agent-evals/fixtures/EV-006/*"
review_set: subset
relationships:
  - { target: "ix://agent-ix/filament-examples/FR-001", type: references }
---

## Summary

Ticket: QSL-2490. This is a diff-scoped review of filament-examples PR 110, which adds scenario EV-006 (declare an intended protocol terminal state and prove deadlock freedom). The Rust review lane is folded in. The author is a codex coder, and both commits carry the identity `Peter Krenesky <kreneskyp@gmail.com>`.

## Verdict

**FAIL.** There is one high finding: the grader's expected `result.json` disagrees with the task text, so an agent that follows the task exactly always fails.

The provider change only registers the new entry. It does not touch the transcript boundary or the rg/sed rules. This branch is based on main without PR 109, so expect a merge conflict in provider.rs, not a weakening. No scorecard or result claims a live run.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The task tells the agent to write `blocked` as `["(0, Both, right, 0) at B: Precondition{TwoPre}", "(0) at join Both: JoinWaiting{[(0, Both, right, 0)]}"]`. `check_summary` requires exactly `["B: Precondition{TwoPre}", "join Both: JoinWaiting"]`. An agent that copies the task's own example fails with WrongSummary every time. No test exercises `check_summary`. | agent-evals/src/scenarios/protocol_deadlock.rs:315-336; agent-evals/fixtures/EV-006/task.md:24-30 |
| FND-002 | medium | The task gives away the whole answer: the exact member to add (`terminal when { k.v = 1 };`) and the complete expected `result.json`. The prefix, blocked threads, both exit codes and the basis are all stated. The summary and protocol checks therefore test copying, not reading the counterexample. The grader also accepts only that exact line (whitespace-insensitive), which shuts out the repair the ticket also allows: the precondition fix, `TwoPre: self.v <= 1` per FR-211-AC-3. | agent-evals/fixtures/EV-006/task.md:12-31; agent-evals/src/scenarios/protocol_deadlock.rs:132-163 |
| FND-003 | medium | A protocol route that is unavailable or incomplete, a tool failure, or a timeout all come back as plain `UnsupportedAnalysis`, `ToolFailure` or `Timeout` failures. None is mapped to the `harness-error:` prefix, which the sibling `safety_analyze` uses for an exceeded budget, or to a CLI-gap class. So they count as failed agent samples. That contradicts the fixture README ("never counts as a proof or as an agent failure"). The failure text also doubles the code (`{code:?}: {fault}`, where `Display` already prints the code). | agent-evals/src/scenarios/protocol_deadlock.rs:165-171,370-383; agent-evals/fixtures/EV-006/README.md:23-25 |
| FND-004 | medium | The tests cover only three pure helpers with synthetic JSON. There is no test of `check_summary` (FND-001 would have failed it), `compare_items` or `assess`/`grade`, either with a stub `fsl` or with a recorded release outcome. The expected release output was never checked against the real `fsl analyze`: exit 10 and 0, claim name `Fill`, `Main/Both/left/A`, a certified exhaustive basis. The reference lane was not run. | agent-evals/src/scenarios/protocol_deadlock.rs:386-449 |
| FND-005 | low | The counterexample check is loose: `encoded.contains("B")` is satisfied by `Both`, and `contains("right")` and `contains("fork")` match anywhere in the JSON. It is weaker than the precise prefix and blocked-thread shape FR-211-AC-2 states. `compare_items` against the trusted run is the real check, but a changed FSL counterexample would still pass `check_deadlock` itself. | agent-evals/src/scenarios/protocol_deadlock.rs:226-245 |

## Coverage

- **Diff:** 13 files, 866 insertions, 1 deletion, in 2 commits (c45a9a00, b8dfba61).
- **Boundary:** `provider.rs` only adds an `ENTRIES` row with `shipped_references: true` and two `analyze` evidence commands. No boundary, allow-list or path rule changed.
- **Fixtures:** no reference answer file is shipped into the workspace. `prepare` writes only the given inputs. The answer leaks through `task.md` instead (FND-002).
- **Dependencies:** the `ix-trace-rs` dev-dependency is pinned by git tag v0.1.2. Its lock entry already exists, and `Cargo.lock` only adds the dependency edge.
- **Tests:** not run by the reviewer (titan load). The author reports protocol_deadlock::tests 3 passed, provider::tests 20 passed, and fmt and clippy clean.

## Examined units

- protocol_deadlock.rs prepare, check_inputs, analyze, check_deadlock, check_proof, compare_items, check_summary, assess and grade, plus tests: examined.
- fixtures/EV-006 task.md, README.md, given.native, both request files, start.json and scenario.toml: examined.
- provider.rs ENTRIES row: examined.
- FSL FR-211-AC-2 and FR-211-AC-3: context_only.

## New findings (disposition pass 1)

Found at head 3a9653225fa4ca95040064a60f7f6afc27e77dc1. This branch is rebased on main with PR 109 merged, and `provider.rs` still changes only the EV-006 entry, so the #109 boundary is not weakened.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | high | `check_inputs` requires `baseline.native` to equal the hidden reference source, ignoring only whitespace. The task's English does not give the identifiers that comparison needs: the profile aliases and URIs (`v`, `pp`, `filament.model.complete/v1`, `filament.protocol.finite-global/v1`), the model alias `M`, the contract names `IncPre`/`IncPost`/`TwoPre`/`TwoPost` and their exact predicates, the `(a: Boolean)`/`(b: Boolean)` binders, `finish Done as (d: Boolean) { true }`, and `on origin`. A correct model written from the English cannot match. The repair is just as rigid. Only `terminal when{k.v=1};` placed immediately before `finish Done`, or the literal `self.v<=1`, is accepted; an equivalent predicate or another member position is refused. | agent-evals/src/scenarios/protocol_deadlock.rs:143-161; agent-evals/fixtures/EV-006/task.md:3-13 |
| FND-007 | high | `check_summary` expects FR-211's prose notation: steps `fork(Both)` and `attempt(A)`, and blocked `(0, Both, right, 0) at B: Precondition{TwoPre}` and `(0) at join Both: JoinWaiting{[(0, Both, right, 0)]}`. The task now only says "the value observed in your files", and the JSON that `fsl analyze --format json` emits uses a different form. It has `{"step":"fork","node":"Main/Both"}`, thread paths `{"instance":"0","steps":[{"branch":"right","node":"Main/Both","ordinal":"0"}]}`, `rest.node` `Main/Both/right/B` and `cause.type` `precondition`, as the captured `reference/deadlock-outcome.json` shows. No FSL renderer on origin/main formats `JoinWaiting{`/`Precondition{`. An agent that faithfully reports what it observed fails WrongSummary. | agent-evals/src/scenarios/protocol_deadlock.rs:317-350; agent-evals/fixtures/EV-006/task.md:17-36 |
| FND-008 | medium | `CliGap` is also raised on the agent's own saved files: `one_item`, `check_deadlock` and `check_proof` run on `deadlock.json`/`repaired.json` in `assess` and `compare_items`. A wrong, empty or hand-written saved output is therefore reported as `harness-error:` and excused as not an agent failure. Separately, a genuine CLI gap (the route is unavailable) is also reported as `harness-error:`. The readiness note and FR-002 triage class that as `cli-gap` and file it in the FSL CLI project. Keep `harness-error:` for the trusted run's tool or timeout failures only. | agent-evals/src/scenarios/protocol_deadlock.rs:203-223,251-263,354-361,389-397 |
| FND-009 | low | The fixture README hard-codes the host path `/home/peter/fsl-rc3/fsl` as verification evidence and names no fsl version or commit. Only the terminal-declaration repair was verified offline; the precondition-fix route the grader now accepts was not run against a release `fsl`. | agent-evals/fixtures/EV-006/README.md:12-18 |

## New findings (disposition pass 2)

Found at head e68ff4a42f66eb670c2eb68fba87fb988478eb10.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-010 | high | Nothing ties `protocol.native` to the baseline or to the repairs the task allows. The repaired side checks only that the trusted run of `protocol.native` settles `Fill` as proved, with a certified exhaustive basis. A "repair" that deletes branch B, serializes the two attempts, or swaps in any trivial deadlock-free `Fill` passes, so a wrong model slips through as a pass. Suggested check: the repaired run's exploration must still contain both attempts. Or diff the two sources' parsed structure so that only a `terminal` member or the `TwoPre` predicate may differ. | agent-evals/src/scenarios/protocol_deadlock.rs:95-111,251-273,398-410 |
| FND-011 | medium | `assess` checks the agent's saved `deadlock.json` and `repaired.json` before it runs the trusted analysis. When the protocol route is unavailable, the agent faithfully saves the unsupported outcome, `check_deadlock` fails `WrongDeadlock`, and the result counts as an agent failure, never reaching the `cli-gap:` path. The `cli-gap:` test only passes because it plants a valid captured `deadlock.json` first. Fix: run the trusted analysis first, or apply `is_cli_gap` to the saved files. | agent-evals/src/scenarios/protocol_deadlock.rs:398-406 |

## New findings (disposition pass 3)

Found at head c6e2ba03f475b2e080fc45f859f3ac37cb23fcb1. The reviewer ran the crate's 7 protocol_deadlock tests (all pass). It also ran a scratch probe of `check_protocol_repair` and `parse_protocol` against the pinned `fsl-cst`, using a copy of the crate outside the worktree; nothing was committed.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-012 | medium | Vacuous repairs pass the structural check. `terminal when { true };` and `terminal when { k.v >= 0 };` added after the role, and a second-branch precondition replaced by `{ true }`, all return `Ok` from `check_protocol_repair` (probed). The first two declare every end state intended, which is `terminal any` in effect; the third deletes the guard in all but name. Each removes the deadlock rather than repairing it. The grader runs nothing that would refuse them, and FSL should prove all three. `terminal any` itself is still refused at the behavior level, because FR-211-AC-2 says the request then carries no deadlock-freedom item. Suggested fix: require that the repaired trusted analysis still refutes with the same counterexample when its terminal predicate is replaced by `true`. A simpler fix is to refuse a terminal predicate or precondition body that is a constant. | agent-evals/src/scenarios/protocol_deadlock.rs:347-397 |
| FND-013 | low | The precondition repair is positional: `protocol_shape` takes the second parallel branch as the one to repair. An agent that writes Branch B first gets a correct `TwoPre` repair refused as WrongProtocol (probed with the branches swapped). Identify the branch by the blocked thread in the trusted counterexample, or by the operation whose precondition changed. | agent-evals/src/scenarios/protocol_deadlock.rs:333-345,350-351 |
| FND-014 | low | The structural check parses with `fsl-cst` pinned at deac8870, which is 560 commits behind the release revision 5275e508 the README cites for grading. It runs before the trusted analysis, so syntax the release accepts but the pinned parser refuses becomes an agent WrongProtocol failure. For example, the pinned parser refuses a protocol-member `terminal` placed after `run` (`expected finish`) and accepts it after the role. Align the pin with the release, or classify a parse refusal of a source the trusted `fsl` accepts as `cli-gap:`/`harness-error:`. | agent-evals/Cargo.toml:14; agent-evals/src/scenarios/protocol_deadlock.rs:199-232,724-728 |

## New findings (disposition pass 4)

Found at head e4a49d4bc88f781659de78c77fa7b98d616468e7 by tracing the new heuristics. The scratch probe was not re-run: load was about 16 with three other cargo jobs, and the new `fsl-cst` pin needs a fresh dependency build.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-015 | low | The vacuity checks are syntactic. `is_trivial_true` catches only `true` and constant comparisons. `terminal_is_false_at_start` evaluates only `<path> <op> <integer>` at v=0 and accepts anything else. So semantic tautologies still pass: `terminal when { k.v = k.v }`, `{ k.v = 1 or true }`, and a negation such as `{ not (k.v < 0) }`; and preconditions such as `{ self.v >= 0 }` or `{ self.v = self.v }`, which are true over the whole 0..3 domain. The terminal forms need deliberate disguise. `self.v >= 0` is a plausible lazy repair that the task wording arguably allows. For a sound check, evaluate the predicate over the bounded domain with a public `fsl run` probe, as EV-001's invariant probe does. Require it to be false somewhere and, for a terminal, true at the counterexample's deadlock state. | agent-evals/src/scenarios/protocol_deadlock.rs:488,501-575 |

## New findings (disposition pass 5)

Found at head 2b6b2ef6559175b859de124661b05ace7db924e6 by reading the code. Tests were not run by the reviewer this round: two other cargo jobs were running, and the new `base64` dependency and `fsl-cst` pin need a fresh build after a host reboot.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-016 | low | When the public `fsl run` probe cannot evaluate the agent's repair predicate, `evaluate_predicate` reports `CliGap` (`cli-gap:`), and the sample is excused rather than failed. That happens when the predicate names something other than `<binder>.v` or `self.v`, or uses syntax the pure-function probe rejects. The trusted `fsl analyze` has already accepted the source by then, so this is a limit of the grader's probe translation, not a CLI gap. Report it as `harness-error:`, or as a named grader limitation, so it is triaged to the eval project. Separately, the tautology cases in the test feed `[true; 4]` straight to `validate_predicate_samples`; whether a real `fsl run` evaluates `probeValue = probeValue` or `not ( probeValue < 0 )` to all-true is not exercised. | agent-evals/src/scenarios/protocol_deadlock.rs:563-630,1218-1290 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | `check_summary` now expects the full blocked paths `(0, Both, right, 0) at B: Precondition{TwoPre}` and `(0) at join Both: JoinWaiting{[(0, Both, right, 0)]}`, compared ignoring whitespace. A test covers exactly that result.json. The task no longer states the strings, which raises FND-007. |
| FND-002 | fixed | `task.md` no longer states the terminal line or the result values; it gives placeholders. The precondition repair (`TwoPre` to `self.v <= 1`) is accepted next to the terminal declaration. The new rigidity of the source check is FND-006. |
| FND-003 | fixed | Route, tool and timeout faults (`ToolFailure`, `Timeout`, `CliGap`) are now reported with a `harness-error:` prefix, `Display` prints only the detail, and the stub test asserts the prefix. Over-mapping is FND-008. |
| FND-004 | fixed | New tests: `summary_accepts_the_exact_full_paths_requested_by_task`, `compare_items_checks_trusted_fields`, and `full_grade_uses_stub_fsl_and_real_captured_outcomes`. The last one grades a full workspace through a stub `fsl` that replays the captured release outcomes, and asserts a pass and a `harness-error:` failure. The README states that the baseline (exit 10) and terminal repair (exit 0, certified exhaustive) were verified offline. |
| FND-005 | fixed | `check_deadlock` now checks structured fields: prefix steps and nodes (`fork` `Main/Both`, `attempt` `Main/Both/left/A`), a `precondition` `TwoPre` block at `Main/Both/right/B`, and a `join-waiting` at `Main/Both`. A substring-trap test covers it. |
| FND-006 | still-open | Source equality is gone; the grader now runs `fsl analyze` on the agent's own model, and both repairs pass the stub test. But `check_deadlock` still hard-codes the reference identifiers in the agent's analysis: node paths `Main/Both`, `Main/Both/left/A` and `Main/Both/right/B`, the branch name `right` in the thread path, and the clause `TwoPre`. `task.md` names none of them (it says "Branch A" and "Branch B"), so a correct model written with its own names fails `WrongDeadlock` as an agent error. |
| FND-007 | still-open | The prose notation is gone, and the summary is now built from the trusted CLI JSON (prefix `{step,node}`, blocked `{thread,rest,cause}` in any order, repaired `basis` and `certification`). But `task.md` never names the `result.json` keys the grader reads: top-level `deadlock`/`repaired`, and `exit_code`, `result`, `prefix`, `blocked`. They exist only in the provider-only `reference/result-summary.json`, so a faithful summary under other key names (`baseline`, `exitCode`, `steps`) fails `WrongSummary`. |
| FND-008 | fixed | A wrong or empty saved artifact, or an unexpected trusted outcome, is now an agent failure (`WrongDeadlock`/`WrongProof`, with no prefix). A tool failure, no exit code, non-JSON output or a timeout is `harness-error:`. An unsupported or incomplete outcome is `cli-gap:`. The stub test asserts each class. The ordering gap is FND-011. |
| FND-009 | fixed | The README names FSL 0.2.0 at revision 5275e5086fec403c4d1a6bce25ab7c3660ea5716, says both repairs were run (exit 0, certified exhaustive), and has no host path. `reference/precondition-outcome.json` holds the captured precondition repair. Its registry lists the `fsl-analyze` tool at 0.3.0, which is the analyzer component's version, not the CLI's. |
| FND-006 | fixed | `check_deadlock` no longer hard-codes reference identifiers. It checks the structure: a two-step `fork`/`attempt` prefix with non-empty nodes, one precondition block at a node rest whose thread is a one-step thread path, and one join waiter at a join rest that waits on exactly that thread. Fixture tests use different names (`Outer/ForkGroup`, `alpha`/`beta`, `GuardBeforeSecond`). |
| FND-007 | fixed | `task.md` now states the exact `result.json` schema (`deadlock`/`repaired`; `exit_code`, `result`, `prefix` `{step,node}`, `blocked` `{thread,rest,cause}`, `basis`, `certification`), which matches what `check_summary` builds from the trusted CLI JSON. |
| FND-010 | fixed | `check_protocol_repair` parses both sources and requires one `Fill` with one parallel of two branches, each with one attempt and an unchanged second-branch operation. Every token outside either one added `terminal` node or the second branch's precondition block must be identical. Deleted branch, serialized attempts, trivial Fill, changed `IncPre` and changed postcondition are refused (tested, and probed by the reviewer). Vacuous repairs are FND-012. |
| FND-011 | fixed | `assess` runs both trusted analyses before reading the saved files, so an unsupported outcome reaches `cli-gap:`. The test now writes an unsupported `deadlock.json` (`{"category":"unsupported","items":[]}`) instead of planting a valid capture. |
| FND-012 | fixed | The reported vacuous shapes are refused, as tested: `terminal when { true }`, `terminal when { k.v >= 0 }` (true at the v=0 start), and a blocked-branch precondition of `{ true }`. Constant comparisons such as `{ 1 = 1 }` are refused by `is_trivial_true`. The terminal must be a direct `ProtocolTerminal` member of `Fill`. Semantic tautologies remain and are recorded as low FND-015. |
| FND-013 | fixed | `blocked_precondition_clause` reads the one precondition clause from the trusted counterexample, and the repaired branch is the one whose contract list names it. Every branch's operation and contracts must be unchanged. A test passes the precondition repair with the branches in reversed source order. |
| FND-014 | fixed | `fsl-cst` and `fsl-foundation` are pinned to FSL default-branch revision 0443d782, 84 commits after the release 5275e508. `assess` now runs both trusted analyses before the structural parse, so a grader-parser refusal of source the CLI accepted is reported as `cli-gap:`. `task.md` and the README both place the terminal member after the role declarations and before the run, which matches the test. |
| FND-015 | fixed | The syntactic heuristics were replaced by a bounded semantic probe. `check_protocol_repair` returns the changed predicate (a terminal with the protocol's own binder, or the blocked-branch precondition). `probe_expression` rewrites `<binder>.v`/`self.v` to `probeValue`, and `evaluate_predicate` runs the public `fsl run` on `checkPredicate(probeValue: Int[0,3])` for v = 0..3. `validate_predicate_samples` refuses a predicate that is true throughout the domain or false at the deadlock value, which is decoded from the trusted counterexample's final `post_state` key (1 here). All five semantic tautologies and `true` produce probes that the all-true rule rejects, and both real repairs (`probeValue = 1`, `probeValue <= 1`) pass through the stub `fsl` grade. |
