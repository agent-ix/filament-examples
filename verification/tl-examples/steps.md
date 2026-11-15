# Temporal logic libraries: Rust examples

These are the programs of the temporal logic pages of the Verification documentation. The test runs every fenced block marked `step=<id>`, in order, in one shell, in a fresh copy of [project](project), and compares the output of each step with [expected](expected). Run it with `make tl-examples-test`.

The project depends on the four TL crates (tl-syntax, tl-parse, tl-rewrite and tl-mltl) at their 0.4.0 release commits, by git. The steps need Rust (`cargo`) and network access to github.com and crates.io.

## Build a formula and bind it to a signal

```bash step=bind
cargo run -q --example bind
```

## Parse and format text

```bash step=parse
cargo run -q --example parse
```

## Rewrite a formula and check the result

```bash step=rewrite
cargo run -q --example rewrite
```

## Evaluate a formula on a trace

```bash step=evaluate
cargo run -q --example evaluate
```

## From text to a verdict

```bash step=pipeline
cargo run -q --example pipeline
```
