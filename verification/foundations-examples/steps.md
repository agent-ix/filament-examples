# Foundation libraries: Rust examples

These are the programs of the foundation libraries pages of the Verification documentation. The test runs every fenced block marked `step=<id>`, in order, in one shell, in a fresh copy of [project](project), and compares the output of each step with [expected](expected). Run it with `make foundations-examples-test`.

The project depends on filament-exact, filament-walk, quire-canonical, filament-semantic-value, filament-eval, filament-checked (a package of the filament-eval repository) and ix-cli-kit by git on `main`, and `Cargo.lock` records the commit of each. The `js` folder of the project installs `@agent-ix/quire` 0.0.9 from the public npm registry. The steps need Rust (`cargo`), Node.js, and network access to github.com, crates.io and registry.npmjs.org.

## Exact arithmetic and the four outcomes

```bash step=exact
cargo run -q --example exact
```

## Walk a deep tree and compute over an arena

```bash step=walk
cargo run -q --example walk
```

## Canonical bytes and digests

```bash step=canonical
cargo run -q --example canonical
```

## Declare a union and admit a value

```bash step=semantic-value
cargo run -q --example semantic_value
```

## Parse a Markdown document with the quire React library

```bash step=quire-react
(cd js && npm install --silent --registry https://registry.npmjs.org && node parse.mjs)
```

## Exit statuses, canonical JSON and color decisions

```bash step=cli-kit
cargo run -q --example cli_kit
```
