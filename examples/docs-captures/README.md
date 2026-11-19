---
type: reference
---

# Capture and activation cases

Capture and activation cases are temporal programs that retain values at their activation anchors. Their requests reference the existing Rover model and observations directly.

The speed changes between 800 and 100. A fast rover that only slows still owes a stop; an initial slow value stays true after the current speed rises. Reactivation cases distinguish independent values from a value shared by all activations.

Run from the repository root:

```sh
cargo run --quiet --locked --manifest-path tools/fsl-examples/Cargo.toml -- check docs-captures
```

Set `FSL` to the driver binary. The all-false guard case requires an inactive result rather than a healthy obligation.

