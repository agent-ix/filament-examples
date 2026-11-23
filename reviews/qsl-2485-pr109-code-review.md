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

## New findings (disposition pass 1)

Found at head 45a21ed3ed989566a677809a6944bf3641bd6827 in the rewritten sed handling. Each sed behaviour below was confirmed with GNU sed 4.9 in a scratch directory: the command printed or copied the outside file. The provider's result for each was traced by reading the code; cargo was not run, because other cargo jobs were running on this host.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | high | `sed_script_words` marks the first word that is not an option as the script, unless it has already seen a script word. It does not recognise a script attached to `-e` (`-es/x/y/`) or a `--file=F` option, so a file operand after either is treated as a script. In `sed_script_fragments`, the leading `/` of an absolute operand then opens a "regex address" that swallows the first path component, and the rest is scanned as a relative, in-workspace path. `sed -es/x/y/ <abs>/evaluator/reference.fsl` and `sed -n --file=c.sed <abs>/evaluator/reference.fsl` print the reference answer with no violation. Before this round both were flagged as absolute reads. | agent-evals/src/provider.rs:743-781,805-815 |
| FND-007 | high | Execution is detected only when `e`, or an `s` command, starts a command. After an address modifier (`/x/I`, `/x/M`) or a custom-delimiter address (`\%x%`), the parser leaves command start, so a following `e` command or `s///e` flag is not seen, and `s` pattern and replacement text is scanned as plain words. Example: `sed '/x/I s/.*/cat ..\/evaluator\/reference.fsl/e' model.fsl` runs `cat` on the reference; the escaped slashes keep the path scan inside the workspace, and no "executes a command" violation is raised. The same loss happens when a script word is misread as an option argument: `-i<suffix ending in f>` (for example `-i.conf`) or `-l N` / `--line-length N`. `sed -i.conf 's/.*/cat ..\/evaluator\/reference.fsl/e' model.fsl` writes the reference into the model undetected. | agent-evals/src/provider.rs:768-774,861-869 |
| FND-008 | low | After an address modifier or a `\cREc` address, `w FILE` and `W FILE` fall into the plain fragment, which is scanned with `writes=false`. Read-only allowances then apply, so a write into the shipped docs (`/x/I w docs/shipped/fsl/x.md`) is not reported. The shipped docs are a symlink to a writable checkout. | agent-evals/src/provider.rs:865-869 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | r/R/w/W file arguments, the s///w flag, the e command and the s///e flag are now scanned or flagged when the script word is recognised. `s/`-prefixed operands are path-scanned. Covered by runtime_sed_file_commands_flags_and_path_looking_operands_stay_breaches. New bypasses in the rewrite are tracked separately as FND-006 and FND-007. |
| FND-002 | fixed | A new test asserts sandbox-breach for r, R, w, W, s///w, -e, --expression=, -f and s/../.. operands, and the exec violation for s///e, e and 1e. |
| FND-003 | fixed | The Claude `runs_n3` block now has pass_rate null, status unscored-harness-error, harness_errors 3 and scored_runs 0, with QSL-3008 as owner. The stale pending and authentication fields were removed. |
| FND-004 | fixed | probe_request and probe_outcome were extracted. probe_shapes_match_the_shipped_run_golden_and_fail_closed pins the request against examples/shop/expected/values-line-total.json and checks that the old shape, non-Boolean values, refusals and bad exits do not grade. |
| FND-005 | fixed | Both `runs_n3` blocks now carry `model` (gpt-6.1-sol and claude-opus-5-5) and per-run rows. |
| FND-006 | fixed | `sed_plan` parses attached `-eSCRIPT`, `--expression=`, `--file=`, `-fFILE`, `-iSUFFIX` and `-l N` / `--line-length` by an allow-list; unknown options fail closed. Operands after an attached script are path-scanned, and the test covers `sed -es/x/y/ <ref>` and `--file=c.sed <ref>`. |
| FND-007 | fixed | The `e` command, the `s///e` flag and every command outside the allow-list are refused wherever they appear, including after the `I`/`M` modifiers and `\c` addresses. `-i.conf` and `-l 5` no longer shift the script word, and tests cover each route. The new bypasses are FND-009 to FND-011. |
| FND-008 | fixed | The `w`/`W` commands and the `s///w` flag are not on the allow-list and are refused after any address. Covered by tests (`/x/I w`, `\cxc W`). |
| FND-009 | fixed | The regex parts of `s` and addresses now skip bracket expressions, and refuse a delimiter, `\]` or `\<delimiter>` inside a bracket and an unterminated bracket (including `[^]/]` and `[[:alpha:]/]`). The `[/]` exec and write exploits are now tests. |
| FND-010 | fixed | The routes named in the finding are now breaches: a line with `$?` or a backtick that contains a sed word, and `env`, `command`, `xargs`, `find -exec`, `timeout` and `busybox` wrappers, all covered by tests. The remaining spellings are FND-014. |
| FND-011 | fixed | `-f`, `-fFILE`, `--file` and `--file=` are refused outright (SED_FILE_REFUSAL), covering `-f -`, `/dev/stdin` and rewritten script files. |
| FND-012 | fixed | `-i` and `--in-place` operands are now scanned as writes, and symlinks resolve to their targets. Tests cover in-place edits through `docs/shipped` and `docs/native-grammar.md`, including the `.bak` and long forms. |

## New findings (disposition pass 2)

Found at head 4c6c3edc2d505ba5355965a509582736b0c85420 in the allow-list rewrite. Every sed behaviour below was confirmed with GNU sed 4.9 in a scratch directory: the command printed the outside reference file, or wrote outside the workspace. The provider's result for each was traced by reading the code; cargo was not run, because three other cargo jobs were running (load about 9.7).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-009 | high | `SedScan::delimited` ends an `s` or address regex at the first unescaped delimiter. GNU sed does not: it reads the delimiter inside a bracket expression as literal, so `s/[/]/X/` turns `a/b` into `aXb`. After `[/]` the checker is out of step with sed, so a `#` comment or `c` text in the checker's reading can swallow sed's real flags. Two cases pass the allow-list with no violation. (1) `s/^[/]*/#\` + newline + `cat ..\/evaluator\/reference.fsl;#/e` runs the cat and prints the reference. (2) `s/^[/]*/#/w ../evaluator/out.fsl` writes outside the workspace. The script word is then dropped from the path scan. | agent-evals/src/provider.rs:891-901,961 |
| FND-010 | high | The allow-list runs only when the whole command line parses and argv[0] is `sed`. Two routes skip it, leaving only the lexical path scan, where `..\/` escapes defeat path detection. (1) Any `$`, backtick, `(` or `)` outside single quotes makes `shell_tokens` return None, for example a trailing `; echo $?`. (2) A wrapper such as `env`, `command`, `xargs`, `find -exec`, `timeout`, `nice` or `busybox` hides argv[0]. Both `sed 's/.*/cat ..\/evaluator\/reference.fsl/e' model.fsl; echo $?` and `env sed 's/.*/cat ..\/evaluator\/reference.fsl/e' model.fsl` print the reference with no violation. Suggested fix: fail closed on any command that cannot be parsed but contains a `sed` word, and on `sed` in a wrapper position. | agent-evals/src/provider.rs:366,1215,1264-1320 |
| FND-011 | high | A script file named `-` (`-f -`, `--file=-`) is resolved as the workspace file `./-`. GNU sed reads `-` as stdin. With a harmless `./-` planted, `printf 'e cat ..\057evaluator\057reference.fsl\n' \| sed -f - model.fsl` runs the stdin script and prints the reference, while the checker approves the planted file. The printf octal escapes hide the path from the lexical scan. Script files are also read at grading time, not at run time, so a `c.sed` that is rewritten after use is judged on its final content. | agent-evals/src/provider.rs:1291-1305 |
| FND-012 | medium | Operands of `sed -i` and `--in-place` are path-scanned as reads (`writes=false`). Read-only allowances then accept an in-place edit of the shipped docs (`docs/shipped/...`, a symlink to a writable checkout) or the grammar link. `sed -i 's/doc/CHANGED/' docs/shipped/g.md` was confirmed to rewrite the symlink target. This existed before the PR, but sits inside the sed handling this PR rewrites. | agent-evals/src/provider.rs:1279-1320 |

## New findings (disposition pass 3)

Found at head 2e30cb07383832452c4a559aa0f4635a5e0cdd9c. The sed routes in FND-014 were confirmed with GNU sed 4.9 and bash in a scratch directory: each printed the outside reference. The provider's result was traced by reading the code; cargo was not run, because other cargo jobs were running (load about 10.4).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-013 | high | `contains_sed_word` is now applied to every tool-input string that is not itself a recorded command, so harmless text is graded `sandbox-breach`. (1) Codex code-mode transcripts pair a `custom_tool_call` JavaScript record (`tools.exec_command({cmd:"sed ..."})`) with a typed `CommandExecution`; this shape is the one the existing Codex test uses. The sed command is already allow-list-checked through the typed record, but the code record (or its corroborated `cmd_data`) contains the word `sed`, so every Codex run that runs even `sed -n '1,3p' model.fsl` now fails. (2) A Claude Bash call's `description` (for example "Use sed to fix the field"), or Write/Edit content containing the word `sed`, also breaches. The changed test documents (1) as intended, but it turns correct runs into failures. The committed codex 3/3 was graded under an earlier boundary and may not reproduce. Suggested fix: refuse only uncorroborated code, and run the scan on command strings, not descriptions or file content. | agent-evals/src/provider.rs:1556-1572,2248-2262 |
| FND-014 | medium | The wrapper refusal checks only whole words after argv[0], so it misses: (1) `gsed` as argv[0], which is neither parsed nor refused; (2) glob spellings such as `/usr/bin/s?d` or `/bin/s[e]d`; (3) sed inside a single string argument, as in `bash -c 'sed "s/.*/cat ..\/evaluator\/reference.fsl/e" model.fsl'`, `sh -c` and `eval '...'`, because the argument word is not split. All three run the exec unrefused. This is the general shell-indirection limit owned by QSL-3015, but the PR's refusal claims "any wrapper". Either apply `contains_sed_word` to argv[0] and to every word, or narrow the claim and point to QSL-3015. | agent-evals/src/provider.rs:757-765,1330-1335 |
