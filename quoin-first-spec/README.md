# Quoin first spec

A small project for the Quoin "Get started" tutorial: one functional requirement that an agent wrote with the `specify` skill, and a few hand-written tests. It is a Python project. No FSL is involved.

The requirement says that a password reset link works for 30 minutes and for one use. It has seven acceptance criteria and two constraints. Two hand-written tests carry the ids of the first two criteria, so the test matrix shows the rest as untagged. The tutorial adds a third test.

## What was recorded and what was written by hand

| Path | Origin |
| --- | --- |
| `project/spec/functional/FR-001-password-reset-link-expiry.md` | Recorded from an agent run, not edited |
| `staged/reviews/2026-11-14-reset-link-base.md` | Recorded from an agent run, not edited |
| `project/src/`, `project/tests/`, `project/pytest.ini`, `staged/tests/` | Written by hand |
| `module/` | Written by hand: the module of the "Add your own module" how-to |

Provenance of the two agent runs, both on 2026-11-14:

- Specification: in an empty folder made with `git init`, after `quoin config set org acme`, the prompt `/specify password reset links expire 30 minutes after they are issued and work once` ran in Claude Code (`claude -p`, model Sonnet) with the `quoin` plugin 0.30.0, `@agent-ix/quoin` 0.30.1 and `@agent-ix/quire-cli` 0.45.0. The agent wrote one file, the requirement, and no user story. It asked at the end whether to run a review.
- Review: in a copy of `project/` made a git repository, the prompt `/spec-review base review of the reset link specification` ran with the same versions. The agent wrote the review file and validated it. It found no assurance profile and started no workflow.

An agent's output varies from run to run. A new run writes different wording and a different number of criteria and findings. Nothing in this folder asks an agent to write them again.

## Folders

| Path | Contents |
| --- | --- |
| `project/` | The project as the tutorial reaches it before the matrix step |
| `staged/` | Files the tutorial adds later: a third test, a test tagged with the requirement id (a wrong tag), and the review |
| `module/` | A one-type Quire module |
| `steps.md` | The commands the tutorial runs |
| `expected/` | What each command printed |

## Run it

Install the Quoin and Quire command lines (`npm install -g @agent-ix/quoin @agent-ix/quire-cli`), then validate the specification and read the test matrix:

```
cd quoin-first-spec/project
quire validate --scope . "spec/**/*.md" --summary
quire matrix --scope .
```

The matrix reads the trace tags in the test files, and nothing runs. The tests run with pytest (`pip install pytest`, then `pytest` in `project/`, with `staged/tests/test_used_link.py` copied into `project/tests/`).

`make test` runs `quoin-first-spec.test.mjs`. It runs each step of `steps.md` that is not marked `manual` in a copy of `project/` or `module/` and compares what it prints with `expected/`. It skips itself when `quoin` or `quire` is not installed, or is older than Quoin 0.30.1 or Quire 0.45.0, the releases the outputs were recorded with. `RECORD=1 node --test quoin-first-spec/quoin-first-spec.test.mjs` records them again.
