---
id: SR-20261121145558
title: PR 74 code-review review
type: SpecReview
analysis: code-review
scope: agent-ix/filament-examples; QSL-2159; PR 74 diff at 06aef6083c4efd5d13fe8c9e476303754adf6750 (examples/architecture probability cases, README.md, index.json)
review_set: subset
examined_scope:
- id: examples/architecture/model/architecture.semantic-ir.json
  path: examples/architecture/model/architecture.semantic-ir.json
  role: context_only
  excerpt: '{"identity":"ix://example/architecture/Service/call","name":"call","params":[{"name":"ok","typeRef":"ix://filament/native/Boolean"}],"pre":[],"post":[],"frame":{"modifies":["ix://example/architecture/Service/up"]}}'
- id: examples/architecture/rules/probability/claims/claims-probability-every-scheduler.fsl
  path: examples/architecture/rules/probability/claims/claims-probability-every-scheduler.fsl
  role: examined
  excerpt: 'random Arch::Service::call(ok) ~ { true: 9999, false: 1 }; // Each call fails 1 time in 10,000, so a service stays up through the next 10 steps with probability about 0.999'
- id: examples/architecture/rules/probability/claims/claims-every-scheduler-without-random.fsl
  path: examples/architecture/rules/probability/claims/claims-every-scheduler-without-random.fsl
  role: examined
  excerpt: '// No random line: every call argument belongs to the scheduler. The scheduler that always passes false makes the service fail at once'
- id: examples/architecture/rules/probability/claims/claims-unbounded-reachability.fsl
  path: examples/architecture/rules/probability/claims/claims-unbounded-reachability.fsl
  role: examined
  excerpt: 'probabilistic ComesUp using inf on Arch over (s: Arch::Service) under every scheduler on origin { probability >= 0.9 [ eventually holds(s.up) ] }'
- id: examples/architecture/rules/probability/exact/staged-exact-reachability.fsl
  path: examples/architecture/rules/probability/exact/staged-exact-reachability.fsl
  role: examined
  excerpt: '// A service that is up already satisfies eventually-up, so the claim holds with probability 1.'
- id: examples/architecture/rules/probability/schedulers/schedulers-fair-weak.fsl
  path: examples/architecture/rules/probability/schedulers/schedulers-fair-weak.fsl
  role: examined
  excerpt: 'probability >= 0.9 [ eventually[0,10] holds(s.up) ] // the service is up within 10 steps with probability about 0.9999'
- id: examples/architecture/rules/probability/claims/claims-mean-of-fraction.fsl
  path: examples/architecture/rules/probability/claims/claims-mean-of-fraction.fsl
  role: examined
  excerpt: 'mean of (fraction holds(s.up) over [0, 10]) >= 0.9'
- id: examples/architecture/rules/probability/claims/claims-long-run-fraction.fsl
  path: examples/architecture/rules/probability/claims/claims-long-run-fraction.fsl
  role: examined
  excerpt: 'long_run_fraction holds(s.up) >= 0.99'
- id: examples/architecture/rules/probability/claims/claims-unbounded-event.fsl
  path: examples/architecture/rules/probability/claims/claims-unbounded-event.fsl
  role: examined
  excerpt: 'random Arch::Service::call(ok) ~ { true: 9999, false: 1 }; probability >= 0.9 [ always holds(s.up) ]'
- id: examples/architecture/example.toml
  path: examples/architecture/example.toml
  role: examined
  excerpt: 'claims-probability-every-scheduler: staged expect_exit 21 ready_exit 0 waits_on QSL-2158; claims-every-scheduler-without-random: staged ready_exit 10 waits_on QSL-2158; schedulers-fair-weak: staged ready_exit 0; staged-exact-reachability-missed: staged ready_exit 10; claims-unbounded-reachability and staged-exact-reachability: running, holds'
- id: examples/architecture/example.toml#claims-quantile-steps
  path: examples/architecture/example.toml
  role: examined
  excerpt: 'statement = "... a service that is up is down within 5 steps on at least 95% of runs only under a scheduler that makes it fail. The claim is compiled, not decided." command = "compile ..." expect_verdict = "checked: quantile claim"'
- id: examples/architecture/README.md
  path: examples/architecture/README.md
  role: examined
  excerpt: 'The claims that quantify over every scheduler declare a `random` line for the call argument, with a call failing 1 time in 10,000, and `fsl analyze` decides them from the start snapshot `data/start-service-up.json`.'
- id: examples/architecture/data/start-service-up.json
  path: examples/architecture/data/start-service-up.json
  role: examined
  excerpt: '"up": {"boolean": true}'
- id: examples/architecture/expected
  path: examples/architecture/expected
  role: examined
  excerpt: 'staged cases record category unsupported at S6b; claims-unbounded-reachability and staged-exact-reachability record success, exact value 1/1 at the single start state'
- id: README.md
  path: README.md
  role: examined
  excerpt: '| [architecture](examples/architecture) | Enterprise | 34 | 9 |'
- id: index.json
  path: index.json
  role: examined
  excerpt: 'generated entries for the changed architecture cases'
- id: rules/staged-availability.fsl
  path: examples/architecture/rules/staged-availability.fsl
  role: context_only
  excerpt: 'post UpFollowsCall using v on Arch::Service::call { self.up = ok }'
- id: FR-876
  path: filament-spec-language spec/functional/FR-876-settle-every-scheduler-claims-over-a-model-without-random-parameters.md
  role: context_only
  excerpt: 'An operation argument with no `random` declaration SHALL be read as a choice of the scheduler ... the item SHALL be `proved` exactly when the event holds on every behaviour (for `>=`), and otherwise `refuted` with a counterexample'
---

## Summary

Ticket: QSL-2159. Review of the every-scheduler probability examples in PR 74. The case bookkeeping is right (counts, staging fields, QSL-2158 routing, no Python or shell, no running case blessed with a wrong answer), but the restored `random` lines do not make the captions true: none of the PR's units links `ok` to `up`, and two running cases and the fair-weak case are trivially decided at the start state.

## Verdict

FIX ROUND NEEDED. Clean: README count 34/9 matches the case moves; staged `ready_exit` 10 for `claims-every-scheduler-without-random` and `staged-exact-reachability-missed` matches FR-876 and the model; QSL-2158 is the right `waits_on` for the routed gap; staged expected files record the current `unsupported` output, which is the staging convention; no Python or logic-bearing shell added. `data_files: []` for analyze `documents` is a pre-existing generator limitation (tools/fsl-examples/src/generated.rs reads only `clause.observations`), out of scope.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | No unit links `ok` to `up`: the model's `call` has `post: []` and `modifies up`, so the `random` line constrains nothing and every "fails 1 time in 10,000, so..." caption and `ready_exit = 0` rests on a missing `post UpFollowsCall` line | examples/architecture/rules/probability/claims/claims-probability-every-scheduler.fsl:7 |
| FND-002 | medium | ComesUp cases run "proved" only because the start snapshot is already up; the eventual claim is decided at position 0 and shows nothing | examples/architecture/example.toml:378-389 |
| FND-003 | medium | schedulers-fair-weak caption gives "about 0.9999", but from an up start the bounded eventual holds at step 0 with probability 1 and fairness never matters | examples/architecture/example.toml:422-437 |
| FND-004 | medium | claims-mean-of-fraction and claims-long-run-fraction state values but stay `compile` "checked", not compared with the analyze verdict; README:86 says analyze decides them | examples/architecture/example.toml:357-376 |
| FND-005 | medium | claims-quantile-steps is an every-scheduler claim the caption itself says is false, left as `compile` "checked" with no verdict | examples/architecture/example.toml:346-354 |
| FND-006 | low | Running analyze probability cases carry no `engine = "probability"` (staged-exact-reachability dropped it), so index.json reports engine null | examples/architecture/example.toml:383,458 |

## Coverage

Frozen candidate agent-ix/filament-examples PR #74 head 06aef6083c4efd5d13fe8c9e476303754adf6750, base origin/main 49ff6cf. Read the full non-generated diff and the decision-bearing parts of every expected file. Worktree /Users/peter/dev/qsl-2727-code used read-only; no builds or tests run. Context: model JSON operation `call` (pre/post empty, frame modifies up), the existing `post UpFollowsCall` in rules/staged-availability.fsl and rules/probability-as-percent.fsl, the FR-876 Link fixture (keeps pre/post, drops only `random`), FR-876 spec text, fsl-eval FR-120 domain enumeration of `modifies` fields. Method: code-review only; rust-review, review-react, gap-analysis and spec-review skipped: no Rust, React, production code or spec changes in the diff.
