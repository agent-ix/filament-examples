---
id: FR-003
title: "Scenario EV-005: write a requirement, validate it and link an FSL claim to its matrix row"
type: FR
relationships:
  - target: ix://agent-ix/filament-examples/agent-evals/FR-001
    type: depends_on
  - target: ix://agent-ix/filament-examples/agent-evals/FR-002
    type: depends_on
---

# FR-003: Link an FSL claim to a test matrix row

## Description

Scenario EV-005 asks an agent to write a functional requirement with `quire`,
validate it, and back one of its acceptance criteria with an FSL claim checked
by `fsl analyze`, so that the criterion's row in the test matrix shows the
claim's result. It runs in the FR-001 sandbox and is scored and triaged by
FR-002.

### Decision: the link is a quoin evidence record of the `fsl analyze` result

The test matrix binds a criterion to source through trace tags in test code,
and no `quire` language adapter reads FSL source. A requirement's FSL rule
written in place in the Markdown is documented as planned. What the released
tools support today is the evidence store: `quoin evidence record` stores a run
of a declared suite, binds each criterion a passing entry names, and
`quoin matrix` shows that binding as the criterion's evidence status. The link
from a claim to a matrix row is therefore a recorded run of an analysis suite
whose entry names the claim and the criterion. A criterion checked by FSL
declares the Verification method `Analysis`, which needs no test symbol, so its
static status in `quire matrix` is `method-without-symbol` and its evidence
status comes from the run.

### The record

- **Suite.** One row in `spec/evidence/suites.md` (a `SuiteRegistry`): id
  `SUITE-<n>`, Command `fsl analyze <request> --format json`, Tool `fsl`,
  Evidence Kind `Analysis`.
- **Results.** The `entries` form `quoin evidence record` reads without an
  adapter-specific format:
  `{"entries": [{"symbol": "<unit file>#<claim name>", "path": "<unit file>", "outcome": "<outcome>", "traceIds": ["<criterion id>"]}]}`,
  with one entry for every item of the analyze request, in request order. The
  path is relative to the repository root.
- **Outcome.** Taken from the item's settlement in the `analyze` document,
  three ways, as the FSL documentation's outcomes page defines them: `proved`
  gives `pass`; `refuted` gives `fail`; `measured` gives `pass` when its
  decision is `accepted`, `fail` when it is `rejected` (a rejected measurement
  fails a run exactly as a violated rule does) and `error` when it is
  `undecided`; every other result (`tested`, `inconclusive`, `unsupported`,
  `declined`, `incomplete`, `failed`) and a disposition with no result give
  `error`. Only a proof or an accepted measurement discharges the criterion.
- **Where it is emitted.** By whoever runs the analysis: run the suite's
  command, write the entries file from the `analyze` document, record it with
  `quoin evidence record --suite SUITE-<n> --tool fsl --adapter entries --results <file>`,
  and record the suite's mocks once with
  `quoin evidence inspect-mocks --suite SUITE-<n>`. The run record lands in
  `spec/evidence/runs/SUITE-<n>/` and the binding in
  `spec/evidence/bindings.json`.
- **How the row reads it.** `quoin matrix` shows the criterion with evidence
  status `bound` (bound by `SUITE-<n>`) and `quoin evidence audit` counts it
  healthy, when the latest run's entry for it passed.

### Refusal when the record is stale or missing

Quoin binds a criterion only through a passing entry; a failing, erroring or
skipped entry binds nothing. A criterion's evidence is refused in these cases,
as the released quoin reports them:

| Case | `quoin matrix` evidence status | `quoin evidence audit` finding |
| --- | --- | --- |
| An earlier run bound the criterion with a passing entry, and the latest run's entry for it is `fail` or `error` | `stale` | `stale-evidence` |
| An earlier run bound the criterion, and the latest run of the suite has no entry for the bound claim | `suspect` | `vacuous-evidence` |
| No run ever bound the criterion, and the store binds no criterion at all: no run is recorded, or every recorded run bound nothing (such as a first and only run whose entry is `fail`) | `no run evidence` | `undischarged` |
| No run ever bound the criterion, while other criteria are bound | `undischarged` | `undischarged` |

`quoin evidence audit --strict` exits 1 in each case. A claim that fails on its
first recorded run is therefore shown as never evidenced, not as stale; it
becomes `stale` only once a passing run had bound it. Quoin does not compare a
run with later commits, so it does not see a claim edited after its run. The
scenario closes that gap itself: `assert` runs the sandbox `fsl analyze` on the
final workspace, maps each item as above, and fails the sample as
`stale-record` when an outcome differs from the latest run record's entry, and
as `claim-edited` when the unit file differs from the fixture.

### Scenarios

**EV-005 (the claim holds).** Fixture: a git repository with one commit holding
`claims/door.fsl` (a unit with the claim `DoorHeld` and the model it needs),
`claims/analyze.json` (a `native-analyze/1` request with one item,
`DoorHeld`), and no `spec/` folder. With the release `fsl`, `DoorHeld` settles
`proved`; the provider's tests check it. Task: write
`spec/functional/FR-001-*.md` whose criterion `FR-001-AC-1` says the door is
held while it is open, is verified by analysis with `fsl` and names
`DoorHeld`; validate it with `quire`; declare the analysis suite; run the
analysis and record its result as evidence for `FR-001-AC-1`; show the test
matrix; and write `result.json` with `criterion` and `evidence_status` (the
status `quoin matrix` shows for it).

**EV-105 (a change breaks the claim).** Fixture: the EV-005 repository after a
correct EV-005 solution (the requirement, the suite row, one recorded run of
`2026-01-05T12:00:00Z` whose `DoorHeld` entry passed and bound `FR-001-AC-1`,
and its mock inspection), plus a second commit that changes the door model in
`claims/door.fsl` so that `DoorHeld` settles `refuted`. Task: the controller
changed in the last commit; run the analysis suite again, record the result as
evidence for `FR-001-AC-1`, show the test matrix, and write `result.json` with
`criterion` and `evidence_status`. The expected outcome is the refusal: a
second run recorded with outcome `fail`, the row `stale` with a
`stale-evidence` finding, and `result.json` saying `stale`.

**EV-205 (the rule written in the requirement).** The same task as EV-005, with
the rule written in the requirement's Markdown under a `### <clause id>`
heading and read in place by `fsl` through the package's extraction member, as
the documentation's "Rules in a Markdown specification" describes. The entry's
symbol is `<requirement file>#<clause id>`. It passes on the EV-005 criteria.
`agent-evals tools` probes the sandbox `fsl` with `fsl compile` on a fixture
compile request whose package holds an extraction member. While `fsl compile`
refuses it, `tools.json` marks EV-205 inactive with the refusal code (today
`extraction-requires-run`), `describe` leaves it out, and the scorecard lists
it as inactive with that code; any refusal code keeps it inactive, and a code
other than `extraction-requires-run` is shown as a changed reason. When a
release's `fsl compile` accepts the member, EV-205 runs with no change to the
harness.

### Assertion

For EV-005, `assert` passes when all of these hold:

1. `quire validate --scope . 'spec/**/*.md'` exits 0.
2. `quire matrix --format json` lists `FR-001-AC-1` with `method` `Analysis`
   and `status` `method-without-symbol`.
3. `quoin matrix --json` lists `FR-001-AC-1` with `evidence_status` `bound`.
4. `quoin evidence audit --json` lists `FR-001-AC-1` among `healthy`.
5. The latest run record of the suite has one entry per request item, with
   symbol `claims/door.fsl#DoorHeld`, trace id `FR-001-AC-1` and the outcome
   the provider's own `fsl analyze` gives (no `stale-record`), and
   `claims/door.fsl` equals the fixture (no `claim-edited`).
6. The transcript's tool-call inputs include `fsl analyze`,
   `quoin evidence record` and `quire validate`, so the run record came from
   the tools and not from a hand-written file.
7. `result.json` has `criterion` `FR-001-AC-1` and `evidence_status` `bound`.
8. The FR-001 sandbox-breach scan is clean.

For EV-105, item 3 expects `evidence_status` `stale`; item 4 expects a
`stale-evidence` finding for `FR-001-AC-1`; item 5 expects a latest run newer
than the fixture's run, with outcome `fail`, while the fixture's passing run is
still in the store; item 6 needs `fsl analyze` and `quoin evidence record`; and
item 7 expects `evidence_status` `stale`.

## Inputs

- The FR-001 sandbox and the fixtures under `agent-evals/fixtures/EV-005/`,
  `EV-105/` and `EV-205/`.

## Outputs

- One assertion result per sample, with a `checks` member per numbered item
  and failures prefixed by their class where the provider can decide it
  (`sandbox-breach`, `harness-error`); `stale-record` and `claim-edited` are
  failure names, classified by triage (FR-002).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-003-AC-1 | With the release `fsl`, the EV-005 fixture request settles `DoorHeld` `proved`, and the same request over the EV-105 fixture's second commit settles it `refuted`. | Test (TC-003) |
| FR-003-AC-2 | The outcome map turns `proved` into `pass`, `refuted` into `fail`, `measured` with decision `accepted` into `pass`, `measured` with decision `rejected` into `fail`, `measured` with decision `undecided` into `error`, and `tested`, `inconclusive`, `unsupported`, `declined`, `incomplete`, `failed` and a disposition with no result into `error`. Recording a `rejected` measurement's entry with the released quoin leaves its criterion unbound (`no run evidence` or `undischarged`), never `bound`. | Test (TC-003) |
| FR-003-AC-3 | On a reference solution workspace for EV-005 (the requirement, the suite row, one entries run recorded with `--adapter entries` and one mock inspection), `assert` passes and its `checks` hold one member for each of the eight items. | Test (TC-003) |
| FR-003-AC-4 | `assert` on EV-005 fails: as `stale-record` when the recorded entry says `pass` but the provider's analysis gives `fail`; as `claim-edited` when `claims/door.fsl` differs from the fixture; when the criterion's Verification is `Test` (item 2); when the only recorded run's entry is `fail` (item 3, `no run evidence`); when no run is recorded (item 3); and when the transcript has no `quoin evidence record` tool call (item 6). | Test (TC-003) |
| FR-003-AC-5 | On a reference solution workspace for EV-105 (the fixture's passing run, then a second run recorded with outcome `fail`), `quoin matrix` shows `stale` and `quoin evidence audit` a `stale-evidence` finding for `FR-001-AC-1`, and `assert` passes. `assert` fails when the second run's entry was recorded as `pass`, when no second run was recorded (item 5), and when the fixture's passing run was deleted from the store (item 5). | Test (TC-003) |
| FR-003-AC-6 | While the sandbox `fsl compile` refuses the probe request's extraction member with `extraction-requires-run`, `tools.json` marks EV-205 inactive with that code and `describe` omits it; with a stub `fsl` whose `compile` refuses with another code, EV-205 stays inactive with that code; with a stub `fsl` whose `compile` accepts the probe, `describe` lists EV-205. | Test (TC-003) |
| FR-003-AC-7 | EV-005 and EV-105 run three times for `codex` and for `claude` in a scheduled run, and every failed sample has a class and a Linear key in the run's `triage.md`. | Demonstration (TC-003) |

## Dependencies

- [FR-001](./FR-001-agent-eval-harness.md) and
  [FR-002](./FR-002-scheduled-runs-scorecard-and-triage.md).
- `quoin evidence record` with the `entries` form, `quoin evidence audit` and
  `quoin matrix` as released in `@agent-ix/quoin`; `quire validate` and
  `quire matrix` as released in `@agent-ix/quire-cli`.
- `fsl analyze` writing the `analyze` document, as released in
  `agent-ix/filament-driver`.
