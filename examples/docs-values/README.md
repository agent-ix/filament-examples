---
type: reference
---

# Numeric documentation cases

Numeric documentation cases are executable programs for the FSL number reference. They select an integer division law explicitly and compare quotients and remainders with concrete values.

Run the cases from the repository root:

```sh
cargo run --quiet --locked --manifest-path tools/fsl-examples/Cargo.toml -- check docs-values
```

Set `FSL` to the driver binary. The division cases cover both signs of each operand under all three laws. The remainder cases exercise a nonzero divisor and an explicit zero-divisor branch.

