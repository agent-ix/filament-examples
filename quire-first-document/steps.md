# Steps of the Quire documentation

The commands of the Quire "Get started" tutorial and how-tos on the docs site. The test runs every fenced block marked `step=<id>`, in order, in one shell, in a fresh copy of [project](project), and compares the output of each step with [expected](expected). Run it with `make quire-test`.

Steps marked `stderr=hide` keep the command's standard output: `quire` also prints warnings about the installed module set on standard error, with paths that differ on every machine. Steps marked `stderr=findings` keep only the lines of standard error that name a document or count the failed ones. A path under the working folder is shown relative to it.

## Install

`quire` is the command line. It carries no document types, so the tutorial installs `quoin` as well, which installs the default spec modules.

```bash step=install
npm install --global --silent @agent-ix/quire-cli @agent-ix/quoin
```

```bash step=check-install
quire --version
```

The command installs the default modules under `~/.ix/filament/modules`. Its output lists each module with local paths and times, and is not compared.

```bash step=modules output=ignore
quoin module ensure-defaults
```

```bash step=list-modules
ls ~/.ix/filament/modules
```

The test starts in a copy of the project, so it skips this step.

```bash step=get-project
git clone https://github.com/agent-ix/filament-examples
cd filament-examples/quire-first-document/project
```

## Read a document

```bash step=parse
quire --pretty parse spec/usecase/US-001-invite-a-teammate.md
```

```bash step=lookup stderr=hide
quire lookup spec/functional/FR-001-invitation-expiry.md --heading "Acceptance Criteria" --content
```

## Validate

```bash step=validate stderr=hide
quire validate --scope . 'spec/**/*.md' --summary
```

```bash step=add-draft
cp fixtures/FR-002-draft.md spec/functional/FR-002-withdraw-invitation.md
```

```bash step=validate-draft expect-exit=3 stderr=findings
quire validate --scope . 'spec/**/*.md'
```

```bash step=fix-draft
cp fixtures/FR-002.md spec/functional/FR-002-withdraw-invitation.md
```

```bash step=validate-fixed stderr=hide
quire validate --scope . 'spec/**/*.md' --summary
```

## Extract links

```bash step=extract stderr=hide
quire --pretty extract spec/functional/FR-001-invitation-expiry.md --module ~/.ix/filament/modules/spec-artifacts-iso
```

## Trace requirements

```bash step=matrix stderr=hide
quire matrix
```

```bash step=trace-prefix stderr=hide
quire trace --id FR-001 --prefix
```

Without `--prefix` the id is matched exactly, and no test names `FR-001` itself.

```bash step=trace-exact stderr=hide
quire trace --id FR-001
```

```bash step=trace-test-file stderr=hide
quire trace --file tests/invitation.test.ts
```

```bash step=trace-source-file stderr=hide
quire trace --file src/invitation.ts
```

```bash step=trace-tsv stderr=hide
quire trace --id FR-001-AC-2 --format tsv
```

```bash step=add-test
cp fixtures/invitation-reuse.ts tests/invitation-reuse.test.ts
node --test --test-reporter=dot
```

```bash step=matrix-after stderr=hide
quire matrix
```

## Check in continuous integration

`quire matrix --strict` exits with status 1 while a criterion has no test. FR-002 has none.

```bash step=ci-strict expect-exit=1 stderr=hide
quire matrix --strict
```

## Write a document

```bash step=schema-fr stderr=hide
quire --pretty schema FR --module ~/.ix/filament/modules/spec-artifacts-iso
```

```bash step=write-fr
cat > spec/functional/FR-003-invitation-reminder.md <<'DOC'
---
id: FR-003
title: "Remind a teammate before an invitation expires"
type: FR
relationships:
  - target: "ix://example/invites/US-001"
    type: "satisfies"
---

# FR-003: Remind a teammate before an invitation expires

## Description

The service shall send one reminder to the invitee one day before the invitation expires.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-003-AC-1 | A reminder is sent six days after the invitation was sent. | Test |
| FR-003-AC-2 | No reminder is sent for an invitation that was already accepted. | Test |

## Dependencies

- **Upstream**: [FR-001](FR-001-invitation-expiry.md)
DOC
```

```bash step=validate-fr stderr=hide
quire validate --scope . spec/functional/FR-003-invitation-reminder.md --summary
```

## Select modules

`--module` names the modules to use and replaces the search. A module that does not define `FR` leaves the documents without a type.

```bash step=module-explicit stderr=hide
quire validate --scope . --module ~/.ix/filament/modules/spec-artifacts-iso 'spec/**/*.md' --summary
```

```bash step=module-wrong expect-exit=3 stderr=findings
quire validate --scope . --module ~/.ix/filament/modules/spec-objects-business 'spec/functional/*.md'
```

`IX_FILAMENT_MODULES_PATH` names a folder that holds module folders. This run uses a new home folder, so the default install is not searched.

```bash step=module-env stderr=hide
modules=~/.ix/filament/modules
HOME=$(mktemp -d) IX_FILAMENT_MODULES_PATH=$modules quire validate --scope . 'spec/**/*.md' --summary
```

## Author a module

```bash step=runbook-schema
quire --pretty schema Runbook --module modules/runbook
```

```bash step=runbook-validate stderr=hide
quire validate runbooks/RB-001-restart-the-queue.md --module modules/runbook
```

```bash step=runbook-draft expect-exit=3 stderr=findings
cp fixtures/RB-002-draft.md runbooks/RB-002-draft.md
quire validate runbooks/RB-002-draft.md --module modules/runbook
```

```bash step=runbook-install stderr=hide
rm runbooks/RB-002-draft.md
mkdir -p .ix/modules
cp -R modules/runbook .ix/modules/runbook
quire validate --scope . 'runbooks/*.md' --summary
```
