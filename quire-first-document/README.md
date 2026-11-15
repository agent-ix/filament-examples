# Quire first document

A small project for the Quire "Get started" tutorial and how-tos: a team service that sends invitation links. It has one user story, one functional requirement with three acceptance criteria, a source file and tests. It is a Node.js project. No FSL is involved.

Two tests carry the ids of the first two criteria, so the test matrix shows the third as untagged. The tutorial adds a test for it, and a second requirement with no tests.

## Contents

- [project](project): the folder the steps start from.
  - `spec/usecase/US-001-invite-a-teammate.md` and `spec/functional/FR-001-invitation-expiry.md`: the documents. Each links to the other.
  - `src/invitation.ts` and `tests/`: the code and the tests. A test names the criteria it checks with `trace("FR-001-AC-1")`, a call that does nothing when the test runs and that `quire` reads from the source. The code names `FR-001-AC-2` in a comment, which `quire` reports as a mention, not as evidence.
  - `fixtures/`: files that steps copy in: an incomplete requirement and its repair, two tests (one tagged with the requirement id, which is a mistake the matrix warns about), and an incomplete runbook.
  - `modules/runbook`: a one-type spec module, the module of the "Author a module" how-to, with its document in `runbooks/`.
  - `.github/workflows/specs.yml`: the workflow of the CI how-to. The test does not run it; it runs the same commands.
- [steps.md](steps.md): every command, in order.
- [expected](expected): the recorded output of each step.

## What the recordings leave out

The outputs were recorded with `quire` 0.45.0 and `quoin` 0.30.1. They differ from a terminal in two ways:

- `quire` prints the full path of each file in its findings. The recordings show the path from the project folder.
- `quire` prints notices about the installed modules on standard error before its result. Steps marked `stderr=hide` or `stderr=findings` leave them out.

## Usage

Run every step on a fresh copy of the project and compare the output with the recorded output:

```
make quire-test
```

The run needs Node.js 22.18 or later, npm, git, and network access to npmjs.org and github.com. It installs `quire`, `quoin` and the default spec modules into a new temporary home folder, which it removes afterwards. Record the outputs again with `make quire-record`.

`make test` runs `quire-first-document.test.mjs`, which runs the same steps with the `quire` and `quoin` on your path and your installed modules. It skips itself when either tool is missing or older than the recorded release.
