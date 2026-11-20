# FSL agent evals

FSL agent evals runs model-authoring tasks through real coding-agent CLIs and independently checks the files they produce.

The public cli-evals SDK captures agent execution and metrics. The native provider prepares the FSL task, checks its concrete model, and invokes the public FSL command for validation.

## Getting started

Run these commands from `agent-evals/`. You need Rust, pnpm, the configured Codex and Claude CLIs, and authenticated sessions.

```sh
pnpm install --frozen-lockfile --ignore-scripts
cargo build --locked
```

Set `FSL` to the accepted current-main executable, `FSL_DOCS_DIR` to the authoritative shipped documentation tree, and `FSL_NATIVE_GRAMMAR` to the published native grammar file. The provider references the documentation without copying it.

The model IDs and sample count are in [agents.toml](agents.toml). Each new reviewed scenario runs once per agent.

## Run a model-authoring task

The task asks the agent to create an identified StorageBin object with bounded fields, an operation frame, and a capacity invariant.

```sh
./target/debug/agent-evals run EV-001 codex
./target/debug/agent-evals run EV-001 claude
```

Each command invokes the SDK for that scenario and agent. It creates a private run directory outside the checkout, isolates the agent home, and removes the temporary credentials after execution. The result names the private SDK report for triage.

Summarize one actual report:

```sh
./target/debug/agent-evals summarize /absolute/path/to/private/raw-report.json
```

The summary includes actual pass rate, available command/turn metrics and independent checks. Keep transcripts and workspaces private. Classify a failure as a doc gap, CLI gap, FSL bug or agent error before committing its compact scorecard.

## Scenario checks

The model grader checks object identity, field types and inclusive bounds, population, operation signature and frame. It accepts aliases, source order changes and equivalent invariant expressions. It uses the public syntax API for structure alongside a real public `fsl check`.

The invariant is evaluated through public FSL execution for all 156 occupied/limit pairs. Recorded agent tool calls must also include checking the authored file. A success marker, comment or dummy declaration cannot satisfy those checks.

The full native domain-package variant is recorded separately in [its pending scenario](fixtures/EV-101/scenario.toml).

## Build and test

The deterministic grader tests require the same accepted FSL executable. Run only the new model grader during authoring:

```sh
cargo test --locked scenarios::author_model::tests -- --ignored --test-threads=1
cargo test --locked provider::tests -- --test-threads=1
cargo test --locked --test prepare -- --ignored --test-threads=1
```

Live runs follow independent review. Use the individual scenario command rather than a whole-suite run.

## Add a scenario

A scenario module exports `ID`, `TITLE`, `prepare` and `grade`. Shared `Tools`, `ScenarioResult` and `Assertion` types are in [the library](src/lib.rs). Add its module and one entry in the provider's `ENTRIES` table, including its required public command operands. The SDK owns process execution and metrics.

## License

MIT. See [LICENSE](../LICENSE).
