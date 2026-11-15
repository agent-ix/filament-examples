# Assurance example

The assurance example is a small shop service whose refund rule is written as requirements, tested, and backed by recorded evidence. It is the example of the Assurance guides on the docs site.

You can check the requirements, read the test matrix, record test runs and a reviewed change, and record a measurement with `quire` and `quoin`.

## Contents

- [project](project): the repository the steps start from.
  - `spec/stakeholder/StR-001-refunds-never-overpay.md`: the stakeholder requirement.
  - `spec/functional/FR-001-refund-limit.md`: the functional requirement and its two acceptance criteria.
  - `spec/evidence/suites.md`: the test suite the evidence names.
  - `spec/assurance/MP-001-refund-test-pass-rate.md`: a measurement plan.
  - `src/refund.mjs` and `tests/refund.test.ts`: the code and the test of the first criterion. `fixtures/refund-refused.ts` is the test of the second criterion, which a step copies into `tests/`.
  - `results/`: the test results the evidence steps record, in the form `quoin evidence record` reads.
  - `change/`: the change record, the proof attestation, the test result file it describes, the review decision and the audit report of the change steps.
  - `measurements/collection.json`: one measurement against the plan.
- [steps.md](steps.md): every command, in order.
- [expected](expected): the recorded output of each step.

## Usage

Run every step on a fresh copy of the project and compare the output with the recorded output:

```
make assurance-test
```

The run needs Node.js 22.18 or later, npm, git, and network access to npmjs.org and github.com. It installs `quire`, `quoin` and the spec modules into a new temporary home folder. Record the outputs again with `make assurance-record`.
