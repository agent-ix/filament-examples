---
title: Check bounded trace evidence and an open prefix
type: how-to
---

Run EV-004 to measure whether an agent can monitor a temporal rule and explain
its actual settlement and counterexample positions.

The original record type accepts levels 0 through 4. Three supplied traces ask
different questions:

- `holding.jsonl` has levels 0, 1 and 2, then authoritative completeness through
  position 2 and a closed scope. The origin window `[0,2]` holds.
- `violating.jsonl` changes the last level to 3. The same window is violated.
- `open.jsonl` has levels 0, 1 and 2, followed only by the end of the input file.
  An unbounded `always` rule remains pending on that finite prefix.

The bounded rule evaluates at origin 0. Its monitor event settles after
observation 2, when the progress record arrives. The violating predicate is at
sample 2 with level 3. The task and report keep evaluation, observation and bad
sample positions distinct.

## Deterministic checks

From the repository root, check only the new monitor grader:

```sh
cargo test --locked --manifest-path agent-evals/Cargo.toml scenarios::trace_monitor::tests -- --test-threads=1
```

Run its explicit public-CLI reference lane with the accepted binary:

```sh
FSL=/absolute/path/to/fsl cargo test --locked --manifest-path agent-evals/Cargo.toml scenarios::trace_monitor::tests::public_monitor_reference_and_current_binding_rejection -- --exact --ignored --test-threads=1
```

The reference lane runs all three public monitor commands, grades their semantic
results independently, then rejects a saved closure ledger transplanted from
another rule. Local units reject wrong settlement positions, incomplete progress,
false unbounded proof claims, changed inputs, tautological rules, malformed output
and missing monitor-command evidence. Synthetic transcript and wire fixtures are
labeled deterministic units; they are not live-agent receipts.

## Agent-run acceptance

After independent review and shared provider registration, select EV-004 and run
the configured Codex and Claude models sequentially, once each. Record actual
runs, turns, commands, outcomes and failure classifications through the shared SDK.

The grader requires the fixed traces, requests and intended rules. It independently
invokes public `fsl monitor`, checks decisive window evidence, and compares the
current closure binding and covered sample fingerprints. Timing metadata does
not affect those semantic checks. The unbounded prefix must remain unsettled.
The shared provider requires actual monitor tool calls for all three input files.

The comparison preserves the complete ordered event stream and final summary,
including intermediate pending states and every input/closure binding. JSON
whitespace and object-key order are formatting. The only excluded timing
annotations are top-level `event.wall_ms` and `summary.elapsed_ms`; nested or
other fields remain part of semantic evidence.
