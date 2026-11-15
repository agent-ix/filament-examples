# Quoin first spec

A small project with one user story, one functional requirement and its tests, as the Quoin "Get started" tutorial builds it. It is a Python project. No FSL is involved.

The requirement says that a password reset link works for 30 minutes and for one use. The functional requirement has four acceptance criteria. Two tests carry the ids of the first two, so the test matrix shows the third as untagged. The tutorial adds a third test and has a base review written, which finds that gap and two others.

## Folders

| Path | Contents |
| --- | --- |
| `project/` | The project as the tutorial reaches it before the matrix step: `spec/`, `src/` and `tests/` |
| `staged/` | The files the tutorial adds later: a third test, a test tagged with the requirement id (a wrong tag, shown on the concept page) and the review |
| `steps.md` | The commands the tutorial runs |
| `expected/` | What each command printed |

## Run it

Install the Quoin and Quire command lines (`npm install -g @agent-ix/quoin @agent-ix/quire-cli`), then validate the specification and read the test matrix:

```
cd quoin-first-spec/project
quire validate --scope . "spec/**/*.md" --summary
quire matrix --scope . --format tsv
```

The tests run with pytest (`pip install pytest`, then `pytest` in `project/` with `staged/tests/test_used_link.py` copied into `project/tests/`). They are not part of the matrix output: the matrix reads the trace tags in the test files, and nothing runs.

`make test` runs `quoin-first-spec.test.mjs`. It runs each step of `steps.md` in a copy of `project/` and compares what it prints with `expected/`. It skips itself when `quoin` or `quire` is not installed. The recorded output was made with Quoin 0.30.1 and Quire 0.45.0; a newer release can print something else, and `RECORD=1 node --test quoin-first-spec/` records it again.

The review in `staged/reviews/` was written by an agent that followed the `spec-review` skill. It is a fixture: nothing in the test asks an agent to write it again.
