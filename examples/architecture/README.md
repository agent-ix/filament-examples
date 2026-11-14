# Architecture

The architecture example is a microservice estate in three tiers, with teams that own services, dependencies between services, and data stores of different classifications. Rules over this model keep ownership, allowed dependencies and personal-data boundaries checkable whenever the architecture changes.

You can run each rule against a good architecture and against a broken variant of it. A rolling deploy plan is checked step by step as a trace.

## The model

- `Service` has a tier (edge, core or data), an owning team, an ordered list of dependencies, the stores it reads, and the instances that serve it.
- `Team` has a size.
- `Store` has a classification (public, internal or personal) and an owning service.
- `Instance` has a serving flag and a release.

The model is in [model/architecture.semantic-ir.json](model/architecture.semantic-ir.json). Dependencies are an ordered sequence because the reachability check accepts only single, optional and ordered edges.

## Rules about the architecture

The rules in [rules/architecture.fsl](rules/architecture.fsl) check one service of a snapshot. The table lists each rule with its requirement and a passing and a failing case.

| Rule | Requirement | Passing | Failing |
| --- | --- | --- | --- |
| `EveryServiceOwned` | Every service has an owning team | `owned-ok` | `owned-orphan` |
| `NothingCallsTheEdge` | No service depends on an edge service | `edge-ok` | `edge-called` |
| `DataTierStaysDown` | A data service depends only on data services | `data-tier-ok` | `data-tier-calls-core` |
| `NoDependencyCycle` | Following dependencies never leads back to the start | `no-cycle-ok` | `no-cycle-loop` |
| `PersonalDataBoundary` | A reader of personal data is marked as handling it | `personal-data-ok` | `personal-data-leak` |

Each rule is checked for one service, the one the request names. The case `no-cycle-checked-for-web` checks `NoDependencyCycle` for the web service on the architecture where payments and ledger depend on each other. Web is on no cycle, so the rule holds.

## Rolling deploy

The rule `KeepServing` in [rules/deploy.fsl](rules/deploy.fsl) requires each service to keep its minimum number of instances serving. It checks one step. The case `deploy-step-ok` passes, and `deploy-step-too-many` takes too many instances out at once.

The rule `ServingThroughout` in [rules/deploy-plan.fsl](rules/deploy-plan.fsl) checks the whole plan as a trace of architecture snapshots. The plan in `deploy-plan-one-at-a-time` takes one instance out per step and passes. The plan in `deploy-plan-two-at-a-time` takes two out and is violated at step 1.

## Run a passing and a failing case

From this folder, with `FSL` set to the driver:

```
$FSL run requests/no-cycle-ok.json --format json
```

The recorded result is:

<!-- output: no-cycle-ok -->
```
exit 0
{"category":"success","clause":"NoDependencyCycle","result":"tested"}
```
<!-- /output -->

In `no-cycle-loop`, payments depends on ledger and ledger depends on payments:

```
$FSL run requests/no-cycle-loop.json --format json
```

The recorded result is:

<!-- output: no-cycle-loop -->
```
exit 10
{"category":"violation","clause":"NoDependencyCycle","result":"refuted"}
```
<!-- /output -->

## Staged

These cases are staged until the language supports them.

- `staged-deploy-bounded`: The deploy rule over a window of five steps. Needs clock selection.
- `staged-availability`: 99% availability over 10 calls with 1-in-1,000 failures. Needs packaging of random parameters.

`example.toml` records the exact reason and ticket for each.
