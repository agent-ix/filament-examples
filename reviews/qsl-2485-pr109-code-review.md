---
id: SR-1522
title: "QSL-2485 PR 109 code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-examples; QSL-2485; PR 109 head 5c6839aac354d01b83219b42f9f6b13d42d94fd5; agent-evals/README.md, agent-evals/scorecards/EV-001-{codex,claude}.json, agent-evals/src/main.rs, agent-evals/src/provider.rs, agent-evals/src/scenarios/author_model.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/filament-examples/FR-001", type: references }
---

## Summary

Ticket: QSL-2485. Diff-scoped review (git diff origin/main...HEAD, one commit) of filament-examples PR 109: EV-001 N=3 live runs for codex and claude, and three harness fixes. The Rust review lane is folded in. The fixes are: the invariant probe's call-argument shape and result path, the transcript boundary's rg flag list and sed script handling, and a per-run TMUX_TMPDIR.

## Verdict

**FAIL**: one high finding. The sed fix skips the whole sed script word from the boundary path scan whenever the word starts with `s/` or `:` or contains `;s/`. A sed script can name files with `r`, `w`, `R`, `W` and the `e` command or `s///w` / `s///e` flags. So `sed -i 's/a/a/;r <abs>/evaluator/reference.fsl' model.fsl`, which copies the reference answer into the agent's model, now passes a scan that flagged it before the change. That weakens FR-001-AC-6 and could let a copied model pass EV-001.

Points that held up:
- The probe change matches the shipped result shape in `examples/shop/expected/values-line-total.json`. It still fails closed: `as_bool` refuses any non-Boolean value, and the run must return `category: success` and `kind: completed`.
- The added rg flags all take no argument, so the parser still reads the right words as paths. Combined short flags still fail closed.
- The TMUX_TMPDIR socket directory is private to the run.
- The scorecard medians follow from the committed per-run arrays: codex 17 calls and 62487 ms, claude 1 call and 6820 ms. The coder's run logs show codex 1/1 at 70 s, 62 s and 46 s, and claude 0/1 at 7 s for each run, which agree with the committed values.
- No credentials, login state, HOME paths or host paths appear in the committed diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new sed rule (`sed_script`) drops the whole script word from the path scan when it starts with `s/` or `:` or contains `;s/`. File operands inside the script are no longer checked: the `r`/`R` read, `w`/`W` write and `e` exec commands, and the `s///w FILE` and `s///e` flags. Example: `sed -i 's/a/a/;r <abs>/evaluator/reference.fsl' model.fsl` copies the reference into model.fsl. Before this change, splitting on whitespace and `;` exposed that absolute path and it was flagged as `sandbox-breach`. A real path operand that starts with `s/` is skipped the same way, for example `sed -n p s/../../x` after `mkdir s`. Suggested fix: skip only data fragments inside `s` commands (for example a lone `/`), and keep checking file arguments of r/R/w/W/e and the w flag. | agent-evals/src/provider.rs:968-972 |
| FND-002 | medium | The new test `runtime_plain_rg_flags_and_sed_replacement_slashes_are_not_breaches` only checks that commands are accepted. No test checks that a sed script with `r`/`w`/`e` and an absolute or `../` path is still a `sandbox-breach`, so the regression in FND-001 goes unnoticed. FR-001-AC-6 requires a breach for reading an absolute path in the source tree. | agent-evals/src/provider.rs:1525-1543 |
| FND-003 | medium | The Claude scorecard's `runs_n3.pass_rate` is `"0/3"`, the same field and shape the codex block uses for a real 3/3 score. The harness-error label is only in the `failure_class` prose, so a reader or aggregator can take 0/3 as a model result. The top-level `status: pending` and `pending_prerequisite` (no usable Claude authentication) are now stale: the N=3 runs started tasks and made tool calls. Record the Claude result as unscored (for example `pass_rate: null`, `harness_error: 3`) and update or remove the stale top-level fields. | agent-evals/scorecards/EV-001-claude.json:14-20,78 |
| FND-004 | low | The probe request and response shape changed, and no test pins it. A test that checks `value_run`'s request shape and its reading of `result.value.value` against the shipped golden (`examples/shop/expected/values-line-total.json`) would have caught the drift that caused this PR's earlier harness failure. | agent-evals/src/scenarios/author_model.rs:105,121 |
| FND-005 | low | The `runs_n3` blocks do not name the model. The top-level `model` and `cohort` (fsl deac8870, cli_agent_evals) describe the earlier attempt, while `runs_n3.cohort` gives fsl 4b743f67. The N=3 block is not self-describing about which model id produced 3/3 (codex gpt-6.1-sol according to agents.toml). | agent-evals/scorecards/EV-001-codex.json:70-78 |

## Coverage

- Diff: the 6 changed files, 160 insertions and 5 deletions, in one commit, 5c6839aa.
- Commit identity: author and committer are `Peter Krenesky <kreneskyp@gmail.com>`, with a trailer `Co-Authored-By: Claude Sonnet 5.5 <noreply@anthropic.com>`. Recorded as found; no policy judgement.
- Trace: the new test carries `/// Trace: FR-001 AC-6`, the repo's existing doc-line convention. The `quire matrix` engine does not bind doc lines (see the gap-analysis SR).
- Leak scan: the diff was grepped for `/home/`, `peter`, `token`, `secret`, `auth`, `.claude`, `.codex`, `worktrees` and `/tmp/`. The only hits are pre-existing note text and a `total_tokens` metric. Nothing leaked.
- Tests were not run. Three other cargo jobs were running on this host, which runs one heavy build at a time. FND-001 was traced through the code, not executed.
- The raw SDK reports (`/tmp/fsl-agent-eval-*`) were not readable from this reviewer's sandbox. The per-run tool-call counts and exact milliseconds were checked only against the runner logs' rounded latencies and pass/fail.

## Examined units

- agent-evals/src/provider.rs `rg_path_operands` option list: examined.
- agent-evals/src/provider.rs `inspect_command` sed_script rule: examined.
- agent-evals/src/provider.rs `inspect_paths`: context_only.
- agent-evals/src/scenarios/author_model.rs `value_run`: examined.
- agent-evals/src/main.rs `run` TMUX_TMPDIR: examined.
- agent-evals/scorecards/EV-001-codex.json `runs_n3`: examined.
- agent-evals/scorecards/EV-001-claude.json `runs_n3` and top level: examined.
- agent-evals/README.md tmux note: examined.
