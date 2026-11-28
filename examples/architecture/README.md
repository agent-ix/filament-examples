# Architecture

The architecture example is a microservice estate in three tiers, with teams that own services, dependencies between services, and data stores of different classifications. Rules over this model keep ownership, allowed dependencies and personal-data boundaries checkable whenever the architecture changes.

You can run each rule against a good architecture and against a broken variant of it. A rolling deploy plan is checked step by step as a trace.

## The model

- `Service` has a tier (edge, core or data), an owning team, an ordered list of dependencies, the stores it reads, and the instances that serve it.
- `Team` has a size.
- `Store` has a classification (public, internal or personal) and an owning service.
- `Instance` has a serving flag and a release.

The model is in [model/architecture.semantic-ir.json](model/architecture.semantic-ir.json). Dependencies are an ordered sequence field, which a reachability check over a field accepts. The relationship `calls` links a service to the services it calls at run time. It is a set of up to eight links, and a reachability check over a relationship end accepts it.

## Rules about the architecture

The rules in [rules/architecture.fsl](rules/architecture.fsl) check one service of a snapshot. The table lists each rule with its requirement and a passing and a failing case.

| Rule | Requirement | Passing | Failing |
| --- | --- | --- | --- |
| `EveryServiceOwned` | Every service has an owning team | `owned-ok` | `owned-orphan` |
| `NothingCallsTheEdge` | No service depends on an edge service | `edge-ok` | `edge-called` |
| `DataTierStaysDown` | A data service depends only on data services | `data-tier-ok` | `data-tier-calls-core` |
| `NoDependencyCycle` | Following dependencies never leads back to the start | `no-cycle-ok` | `no-cycle-loop` |
| `NoCallCycle` | Following calls at run time never leads back to the start | `no-call-cycle-ok` | `no-call-cycle-loop` |
| `NoCallsIntoTheEdge` | Nothing calls an edge service at run time | `no-call-edge-ok` | `no-call-edge-called` |
| `PersonalDataBoundary` | A reader of personal data is marked as handling it | `personal-data-ok` | `personal-data-leak` |

Each rule is checked for one service, the one the request names. The case `no-cycle-checked-for-web` checks `NoDependencyCycle` for the web service on the architecture where payments and ledger depend on each other. Web is on no cycle, so the rule holds.

The rule `NoCallCycle` reads the links of the `calls` relationship from the snapshot. `no-call-cycle-partial` lists only some links and marks them incomplete, so the run ends incomplete (exit 22) instead of guessing.

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
{"axes":{"activation":"active","execution":"completed","settlement":"closed-scope","truth":"true"},"category":"success","clause":"NoDependencyCycle","result":"tested"}
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
{"axes":{"activation":"active","execution":"completed","settlement":"closed-scope","truth":"false"},"category":"violation","clause":"NoDependencyCycle","result":"refuted"}
```
<!-- /output -->

The case `probability-as-percent` writes the threshold of a probabilistic claim as 99, meaning 99%. A threshold lies strictly between 0 and 1, so the checker refuses the claim.

## Staged

These cases are staged until the language supports them.

- `staged-deploy-bounded`: The deploy rule over a window of five steps. Needs clock selection.
- `schedulers-fair-weak`: exact analysis under weak fairness reaches the work-unit limit.

`example.toml` records the exact reason and ticket for each.

The unlisted [sampling-negative request](requests/probability/sampling/staged-sampling-claim-missed.json) has no reproducible unforced outcome yet.

## Probability reference cases

The folder `rules/probability` holds one subfolder for each page of the probability reference: `models`, `claims`, `schedulers`, `sampling` and `exact`. The probability cases declare `post UpFollowsCall`, so a service is up after a call exactly when the call succeeded, and a `random` line that makes a call fail 1 time in 10,000. `fsl analyze` decides them from a start snapshot, `data/start-service-up.json` or `data/start-service-down.json`; the two ComesUp cases start down and are proved. The missed unbounded-always case also starts down and is refuted under every scheduler. A model without a `random` line leaves each call argument to the scheduler, so analyze should refute an every-scheduler claim with value 0 and a replayed counterexample (`claims-every-scheduler-without-random`). Analyze decides the bounded always claim, the step quantile, every-scheduler reachability, the finite mean of a fraction (FR-197-AC-4; FSpec FR-411-AC-9), the long-run fraction (FR-199-AC-1) and unbounded always (FR-198). The weak-fairness case remains staged because its exact calculation reaches the work-unit limit.

## Probability tutorial cases

The cases `staged-availability`, `availability-holds`, `staged-availability-missed` and `availability-sampled` are the service of the probability tutorial: calls that fail 1 time in 1,000, and a claim about 10 calls in a row. `availability-holds` now reports incomplete for its workload window; `staged-availability-missed` refutes the claim at 0.999 with a replayed counterexample, and `availability-sampled` measures a claim at 0.98 by sampling. `recovery-expected-calls` now reports incomplete, while `recovery-quantile` proves that the service is up after one call on at least 95% of runs. `availability-long-run` and `availability-mean-fraction` prove a long-run fraction and a mean of a fraction under a workload. The sampled cases show every result of a sampled claim: `availability-sampled` accepts, `availability-sampled-rejected` rejects, `availability-sampled-undecided` stays inconclusive inside the indifference region, and `availability-sampled-long-run` measures a long-run fraction. `availability-sampled-every-scheduler` and `availability-sampled-reachability` record the two claims that a sampler refuses.
