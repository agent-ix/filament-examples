# Parts saga

The parts saga is an example of a manufacturing flow across services. A work order releases a part to machining. The machining station reserves a machine, processes the part and hands it to inspection. Inspection passes the part to assembly, sends it back for rework at most twice, or scraps it. Scrapping releases the machine.

You can check the protocol that connects the stations, and run invariants over the jobs it moves. The checker finds two design faults in two variants of the protocol.

## The model

- `Job` has a part, a status and a rework count. The status is released, machined, passed, rework, scrapped or assembled.
- The stations are `PlanningService`, `MachiningStation`, `InspectionStation` and `AssemblyStation`.
- `WorkOrder` and `Handoff` are the messages between the stations.

The model is in [model/parts-saga.semantic-ir.json](model/parts-saga.semantic-ir.json).

## Rules about jobs

The rules in [rules/jobs.fsl](rules/jobs.fsl) check one job of a snapshot.

| Rule | Requirement | Passing | Failing |
| --- | --- | --- | --- |
| `ReworkAtMostTwice` | A part goes back for rework at most twice | none | `jobs-third-rework` |
| `ReworkedPartsAreTracked` | A part in rework has a rework count | `jobs-rework-tracked` | `jobs-rework-untracked` |

## The protocol

[rules/part-flow-untimed.fsl](rules/part-flow-untimed.fsl) declares four roles, three channels with at-least-once delivery, a choice by inspection, and a bounded rework loop. The case `protocol-compiles` checks it and exits 0.

[rules/inspection.fsl](rules/inspection.fsl) holds the inspection part of the flow alone, from the handoff to the choice; the case `inspection-compiles` checks it.

Two variants each contain one fault, and the checker refuses both with exit 20.

| Case | Fault the checker finds |
| --- | --- |
| `protocol-overlapping-verdict` | The pass and rework guards can both be true |
| `protocol-wrong-role` | Machining decides the loop without seeing the result |

## Run a passing and a failing case

From this folder, with `FSL` set to the driver:

```
$FSL compile requests/protocol-compiles.json --format json
```

The recorded result is:

<!-- output: protocol-compiles -->
```
exit 0
success
```
<!-- /output -->

The same command on `requests/protocol-overlapping-verdict.json` is refused:

<!-- output: protocol-overlapping-verdict -->
```
exit 20
refusal: undefined_expression: undefined_expression (unproved-exhaustiveness): the guards of `choice Verdict` are not exhaustive and exclusive (duplicate-arm)
```
<!-- /output -->

## Deadlock freedom

`staged-saga-deadlock-free` checks that the untimed protocol never gets stuck: every job ends assembled, scrapped or given up. The request supplies a starting state (`data/saga-start.json`, one job and one of each station), and `fsl analyze` explores all 8 reachable states and proves it.

## Protocol reference cases

The folder `rules/protocols` holds one subfolder for each page of the protocol reference: `declaration`, `roles-and-channels`, `steps`, `control-flow`, `retries-and-timeouts` and `compensation`. Each case is a small variation of the protocol above, and most of them show one form that compiles or one refusal. Case ids start with the topic of their page: the page `retries-and-timeouts` uses `timeouts-*`, and `roles-and-channels` uses `roles-*` and `channels-*`.
