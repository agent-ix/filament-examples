# Steps of the Quoin first-spec tutorial

The commands the tutorial runs, in order. The test in this folder runs each step in a copy of `project/` and compares what it prints with the file of the same name in `expected/`. The documentation site shows these commands and outputs, so the tutorial cannot drift from what was run.

A fence is `<shell> step=<id>` followed by optional words:

- `stage=<path>,<path>` copies those files from `staged/` into the project before the step runs.
- `expect-exit=<n>` is the exit status the step must give (the default is 0).
- `manual` marks a step the test does not run, such as an install. Its recorded output was captured once, by hand.

The recorded output is what the command prints on standard output, followed by what it prints on standard error without the notices Quire gives about installed modules (lines that start with `semantic.` or `warning: [`). The project folder is shortened to nothing in paths.

## Install

```bash step=install manual
npm install -g @agent-ix/quoin @agent-ix/ix-flow @agent-ix/quire-cli
```

```bash step=check-install manual
quoin --version
quire --version
quoin catalog validate
```

```bash step=add-plugin manual
claude plugin marketplace add agent-ix/agent-plugins --scope project
claude plugin install quoin@agent-ix --scope project
claude plugin list
```

## Validate

```bash step=validate
quire validate --scope . "spec/**/*.md" --summary
```

Change the header of the criteria table, then validate again.

```bash step=break-fr
sed -i.bak 's/| Verification |/| Method |/' spec/functional/FR-001-reset-link-expiry.md
```

```bash step=validate-broken expect-exit=3
quire validate --scope . "spec/**/*.md"
```

```bash step=restore-fr
mv spec/functional/FR-001-reset-link-expiry.md.bak spec/functional/FR-001-reset-link-expiry.md
```

## Test matrix

```bash step=matrix
quire matrix --scope . --format tsv
```

```bash step=matrix-after stage=tests/test_used_link.py
quire matrix --scope . --format tsv
```

## Review

```bash step=validate-review stage=reviews/2026-11-14-reset-link-expiry-base.md
quire validate --scope . "reviews/*.md" --summary
```
