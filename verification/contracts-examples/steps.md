# Contract libraries: Rust examples

These are the programs of the contract libraries pages of the Verification documentation. The test runs every fenced block marked `step=<id>`, in order, in one shell, in a fresh copy of [project](project), and compares the output of each step with [expected](expected). Run it with `make contracts-examples-test`.

The project depends on filament-contract-runtime and filament-verification-contracts by git on `main`, and `Cargo.lock` records the commit of each. The `analyze` folder is a second package that depends on filament-analyze and filament-contract-ir. The steps need Rust (`cargo`) and network access to github.com and crates.io.

## Count verdicts in a campaign

```bash step=runtime
cargo run -q --example runtime
```

## Validate, compare and digest verification records

```bash step=contracts
cargo run -q --example contracts
```

## Lower a consistency request to the query a solver receives

This step is a separate package, `analyze` in the project folder, with its own lock file.

```bash step=analyze
(cd analyze && cargo run -q)
```
