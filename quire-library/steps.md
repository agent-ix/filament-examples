# Steps of the Quire library pages

The Rust and WebAssembly programs of the Quire library pages of the documentation. The test runs every fenced block marked `step=<id>`, in order, in one shell, in a fresh copy of [project](project), and compares the output of each step with [expected](expected). Run it with `make quire-library-test`.

The project depends on `quire-rs` at its `v0.60.0` tag, by git. The steps need Rust (`cargo`), Node.js and npm, and network access to github.com, crates.io and npmjs.org. They use no `quire` command and no installed spec module: the modules are in [project/modules](project/modules), the documents are in [project/runbooks](project/runbooks), and [project/broken](project/broken) holds manifests that fail to load in different ways. The Python page has its own steps in [python/steps.md](python/steps.md).

The test starts in a copy of the project, so it skips this step.

```bash step=get-project
git clone https://github.com/agent-ix/filament-examples
cd filament-examples/quire-library/project
```

## Validate a document

```bash step=validate
cargo run -q --example validate
```

## See what validation checks

```bash step=validate-checks
cargo run -q --example validate_checks
```

## Load modules, and what happens when one is broken

```bash step=load-rules
cargo run -q --example load_rules
```

## Check a requirement against the grammar

```bash step=grammar
cargo run -q --example grammar
```

## Read a document

```bash step=parse
cargo run -q --example parse
```

## Extract records and edit a section

```bash step=extract-edit
cargo run -q --example extract_edit
```

## Extract one record for each section

```bash step=iterate
cargo run -q --example iterate
```

## Query a corpus

```bash step=corpus
cargo run -q --example corpus
```

## Use the WebAssembly package

```bash step=wasm-install
npm install --silent --no-audit --no-fund @agent-ix/quire-wasm@0.6.0
```

```bash step=wasm
node examples/wasm.mjs
```
