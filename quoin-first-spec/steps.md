# Steps of the Quoin first-spec tutorial

The commands the tutorial runs, in order. The test in this folder runs each step in a copy of `project/` and compares what it prints with the file of the same name in `expected/`. The documentation site shows these commands and outputs, so the tutorial cannot drift from what was run.

A fence is `<shell> step=<id>` followed by optional words:

- `stage=<path>,<path>` copies those files from `staged/` into the project before the step runs.
- `expect-exit=<n>` is the exit status the step must give (the default is 0).
- `manual` marks a step the test does not run, such as an install. Its recorded output was captured once, by hand, in a scratch home directory. The Copilot step was not run at all, and its recorded output is empty.

The recorded output is what the command prints on standard output. When the command fails, it also holds the lines on standard error that name a document or count the failed documents. Quire prints notices about its installed modules on standard error, and they change from release to release, so they are left out. The project folder is shortened to nothing in paths.

## Install

```bash step=new-project manual
mkdir reset-links
cd reset-links
git init
```

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

```bash step=add-plugin-codex manual
codex plugin marketplace add agent-ix/agent-plugins
codex plugin add quoin@agent-ix
```

```bash step=add-skills-opencode manual
for skill in assurance-status-report gap-analysis spec-app-review \
  spec-architecture-evaluation spec-correctness spec-criterion-strength-analysis \
  spec-dependency-analysis spec-ears-analysis spec-evidence-analysis \
  spec-failure-domain-analysis spec-fuzz spec-ideation spec-integrity-analysis \
  spec-matrix spec-object-review spec-review spec-risk-complexity-analysis \
  spec-scope-boundary-analysis spec-security-analysis spec-to-plan specify
do
  gh skill install agent-ix/quoin "$skill" --scope user --agent opencode
done
```

```bash step=add-plugin-copilot manual
copilot plugin marketplace add agent-ix/quoin
copilot plugin install quoin@quoin
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
quire matrix --scope .
```

A test tagged with the requirement id instead of a criterion id binds to no criterion.

```bash step=matrix-requirement-tag stage=tests/test_used_link_by_requirement.py
quire matrix --scope .
```

```bash step=matrix-after stage=tests/test_used_link.py
quire matrix --scope .
```

## Review

```bash step=validate-review stage=reviews/2026-11-14-reset-link-expiry-base.md
quire validate --scope . "reviews/*.md" --summary
```
