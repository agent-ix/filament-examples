---
type: reference
---
# EV-002: repair from refusal text

The agent repairs an Integer postage function using its source contract, the
public refusal and `fsl`. The charge is 7 plus 3 per positive unit; zero and
negative counts cost 7. Preparation captures the actual text refusal from the
configured trusted FSL binary. Documentation and reference solutions are not
provided to the agent.

The grader checks the submitted `postage.fsl`, then invokes the trusted absolute
`fsl` path with grader-owned run requests. Eight fixed inputs cover negative,
zero, positive and large counts. Each execution must return the required Integer.
The completion marker, a checking source, a constant answer and a changed rate
cannot substitute for those results.

Run the four deterministic grader tests from the repository root:

```sh
FSL=/absolute/path/to/fsl cargo test --manifest-path agent-evals/Cargo.toml --target-dir target --lib scenarios::refusal_repair::tests -- --ignored --test-threads=1
```

The tests accept an original correct repair and reject the unchanged Boolean
body, a constant 7 with a completion marker, and a charge with the wrong rate.
Live runs select `EV-002` through the shared cli-evals suite. Actual Codex and
Claude scorecard rows are recorded after independent review; deterministic grader
tests are not live-agent results.
