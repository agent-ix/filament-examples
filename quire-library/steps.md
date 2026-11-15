# Steps of the Quire library pages

The programs of the Rust, Python and WebAssembly pages of the Quire documentation. The test runs every fenced block marked `step=<id>`, in order, in one shell, in a fresh copy of [project](project), and compares the output of each step with [expected](expected). Run it with `make quire-library-test`.

The project depends on `quire-rs` at its `v0.60.0` tag, by git. The steps need Rust (`cargo`), Node.js and npm, and network access to github.com, crates.io and npmjs.org. They use no `quire` command and no installed spec module: the module is [modules/runbook](project/modules/runbook), and the documents are in [runbooks](project/runbooks).

The Python page is not part of this test. Its wheel is not published on a public index, so the page shows a program that was run by hand against a wheel built from the same tag.

## Validate a document

```bash step=validate
cargo run -q --example validate
```

## Read a document

```bash step=parse
cargo run -q --example parse
```

## Extract records and edit a section

```bash step=extract-edit
cargo run -q --example extract_edit
```

## Query a corpus

```bash step=corpus
cargo run -q --example corpus
```

## Use the WebAssembly package

```bash step=wasm-install
npm install --silent --no-audit --no-fund @agent-ix/quire-wasm
```

```bash step=wasm
node examples/wasm.mjs
```
