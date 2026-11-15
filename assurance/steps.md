# Assurance steps: requirements, tests and evidence for shop refunds

These are the commands of the Assurance guides on the docs site. The test runs every fenced block marked `step=<id>`, in order, in one shell, in a fresh copy of [project](project), and compares the output of each step with [expected](expected). Run it with `make assurance-test`.

The project is a small shop service with one stakeholder requirement, one functional requirement with two acceptance criteria, a measurement plan, and two unit tests. The steps need `quire` and `quoin` from npm (`@agent-ix/quire-cli`, `@agent-ix/quoin`), Node.js 22.18 or later, and git. Steps marked `stderr=hide` keep the command's standard output: `quire` also prints warnings about the installed module set on standard error, with paths that differ on every machine.

## Install the tools and the spec modules

`quire` checks Markdown requirements and computes the test matrix. `quoin` records evidence. Both install from npm.

```bash step=install
npm install --global --silent @agent-ix/quire-cli @agent-ix/quoin
```

The modules define the requirement and assurance document types. The command installs the default set under `~/.ix/filament/modules`. Its output lists each module with local paths and times, and is not compared.

```bash step=modules output=ignore
quoin module ensure-defaults
```

The project is a git repository. Quoin reads the current commit when it audits evidence.

```bash step=repo
git init -q
git add -A
git -c user.name=Shop -c user.email=shop@example.com commit -qm "Shop refunds"
```

## Check the requirements and the test matrix

```bash step=validate stderr=hide
quire validate --scope . 'spec/**/*.md'
```

```bash step=matrix stderr=hide
quire matrix
```

```bash step=trace-id stderr=hide
quire trace --id FR-001 --prefix
```

Without `--prefix` the id is matched exactly, and no test names `FR-001` itself.

```bash step=trace-exact stderr=hide
quire trace --id FR-001
```

Add the test for the second criterion and run the tests. The dot reporter prints one dot per passing test.

```bash step=add-test
cp fixtures/refund-refused.ts tests/refund-refused.test.ts
node --test --test-reporter=dot
```

```bash step=matrix-after stderr=hide
quire matrix
```

```bash step=trace-id-after stderr=hide
quire trace --id FR-001 --prefix
```

```bash step=trace-file stderr=hide
quire trace --file tests/refund-refused.test.ts
```

```bash step=commit-test
git add -A
git -c user.name=Shop -c user.email=shop@example.com commit -qm "Test the refused refund"
```

## Record test evidence

Run the tests with Node's JUnit reporter. Node's report does not say which criteria each test checks, so `tools/junit-trace.mjs` adds a `trace` property to each test case from the tags `quire` reads, and makes the report the same on every machine.

```bash step=run-tests
node --test --test-reporter=junit --test-reporter-destination=results/node.xml
node tools/junit-trace.mjs results/node.xml results/junit.xml
```

```bash step=junit-file
cat results/junit.xml
```

```bash step=evidence-record
quoin evidence record --suite SUITE-001 --tool 'node --test' --adapter junit --results results/junit.xml --timestamp 2026-11-14T12:00:00Z
```

```bash step=inspect-mocks
quoin evidence inspect-mocks --suite SUITE-001 --timestamp 2026-11-14T12:05:00Z
```

```bash step=evidence-audit
quoin evidence audit
```

```bash step=assurance-case
quoin assurance
```

A change that breaks the refund rule makes the second test fail. Its run is recorded the same way, and the audit reports it.

```bash step=break-rule
cp fixtures/refund-broken.mjs src/refund.mjs
```

```bash step=run-tests-failing expect-exit=1
node --test --test-reporter=junit --test-reporter-destination=results/node-failing.xml
```

```bash step=evidence-record-failing
node tools/junit-trace.mjs results/node-failing.xml results/junit-failing.xml
quoin evidence record --suite SUITE-001 --tool 'node --test' --adapter junit --results results/junit-failing.xml --timestamp 2026-11-14T13:00:00Z
```

```bash step=evidence-audit-failing
quoin evidence audit
```

```bash step=evidence-audit-strict expect-exit=1
quoin evidence audit --strict
```

Undo the breaking change before the next section.

```bash step=restore-rule
git checkout -- src/refund.mjs
```

## Record a change and its proof

```bash step=change-record
quoin change-assurance write-record --input change/record.json
```

```bash step=change-attest
quoin change-assurance prepare-attestation --input change/attestation.json --output results/junit.xml --media-type application/xml | quoin change-assurance intake --attestation - --output results/junit.xml
```

```bash step=change-receipt
quoin change-assurance receipt --record refund-limit --revision 1 --candidate-revision candidate-1 --decisions change/decisions.json --audits change/audits.json --select unit-tests=unit-tests-1
```

```bash step=change-save
quoin change-assurance receipt --record refund-limit --revision 1 --candidate-revision candidate-1 --decisions change/decisions.json --audits change/audits.json --select unit-tests=unit-tests-1 --json > receipt.json
```

```bash step=change-verify
quoin change-assurance verify-receipt --input receipt.json --record refund-limit --revision 1 --candidate-revision candidate-1 --decisions change/decisions.json --audits change/audits.json --select unit-tests=unit-tests-1
```

## Record a measurement

```bash step=measure-record
quoin measurement record --input measurements/collection.json
```

```bash step=measure-report
quoin report
```
