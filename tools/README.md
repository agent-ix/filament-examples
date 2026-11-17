# Tools

This folder holds the example runner, the interim model generator, the install-test runner and the identifier rewrite.

## Runner

[fsl-examples](fsl-examples) is the example runner, a Rust program. It reads each `example.toml`, runs the cases through the FSL driver named by the `FSL` variable and compares the exit code and the structure of each outcome with the manifest and `expected/`: the category, the result value, the last stage, the code and cause of the first diagnostic, and the items with their witness fields. The diagnostic message text goes to a separate snapshot, `expected/<id>.messages.txt`, that `make record` rewrites and that never fails a case. The Makefile calls the runner as `make check`, `make staged`, `make record`, `make promote`, `make readme` and `make index`, through the `FSL_EXAMPLES` variable in [config.mk](../config.mk); the default builds it with cargo. Its tests run with `make runner-test`.

Run `fsl-examples --help` for the command summary. `check` accepts repeated `--case` selectors: a selector can be a case id or `<system>/<id>`, with `*` and `?` wildcards. A bare id selects that id in the chosen system folders, or across all systems when no folder is given. For example:

```
fsl-examples check shop --case 'paid-*'
fsl-examples check --case 'shop/paid-*' --case 'rover/safe-stop-late'
```

`regenerate` refreshes the selected cases' expected records; `record` remains a backward-compatible alias. Add `--dry-run` to print the proposed expected-file changes without writing them. Both commands use the `FSL` driver path:

```
fsl-examples regenerate shop --case 'paid-*' --dry-run
fsl-examples record shop --case paid-short
```

[`fr865-text.test.mjs`](fr865-text.test.mjs) runs three direct CLI commands without `--format` and compares their FR-865 stdout and exit status with the transcript fixtures under `verification/fr865-text/expected/`. Run it with `FSL=/path/to/fsl node --test tools/fr865-text.test.mjs`.

## Steps runner

[steps.mjs](steps.mjs) reads a Markdown file such as [install-test/steps.md](../install-test/steps.md), takes every fenced block marked `step=<id>`, and runs the blocks in order in one shell, in a bare container or on the host, in bash or zsh. A block can add `expect-exit=N` for a step that must fail. The runner reports the duration of each step, names the step that failed, and can compare each step's output with recorded files (`--expected`, `--record`). `make install-test` calls it, and [steps.test.mjs](steps.test.mjs) holds its unit tests. The project template uses an unmodified copy of this file for its tutorial.

## Model generator

[model.mjs](model.mjs) is an interim tool. The driver reads domain models as Semantic IR JSON, and nothing else can author that format yet. A model of one object type with one field is about 1.7 KB of JSON. The generator writes that JSON from a short description. It will be removed when the language has a way to declare a model.

The generated models are committed under `examples/<system>/model/`. You do not need the generator to run an example.

### Input

A description is a JavaScript module whose default export has these members.

| Member | Contents |
| --- | --- |
| `identity` | The model identity, such as `example/shop` |
| `values` | Bounded integer types, as `{ Cents: [0, 1000000] }` |
| `enums` | Enumerations, as `{ Status: ["Open", "Paid"] }` |
| `objects` | Object types with `fields` and `ops` |
| `populations` | Named collections of objects, as `{ orders: ["Order"] }` |

A field is a type name, or an object with `optional: true` or `many: [lower, upper]`. The header comment of `model.mjs` lists every form. The descriptions for the examples are in [models](models).

### Regenerate

Write one model with the description and the output path:

```
node tools/model.mjs tools/models/shop.mjs examples/shop/model/shop.semantic-ir.json
```

Regenerate all models with `make models`. The output is identical to the committed files, so `git diff` shows nothing when a description has not changed.

- shop: `models/shop.mjs` writes `examples/shop/model/shop.semantic-ir.json`.
- architecture: `models/architecture.mjs` writes `examples/architecture/model/architecture.semantic-ir.json`.
- parts-saga: `models/parts.mjs` writes `examples/parts-saga/model/parts-saga.semantic-ir.json`.
- cell: `models/cell.mjs` writes `examples/cell/model/cell.semantic-ir.json`.
- rover: `models/rover.mjs` writes `examples/rover/model/rover.semantic-ir.json`.
- rover-spec: `models/rover-spec.mjs` writes `examples/rover/model/rover-spec.semantic-ir.json`.
- lock: `models/lock.mjs` writes `examples/lock/model/lock.semantic-ir.json`.
- two-locks: `models/two-locks.mjs` writes `examples/lock/model/two-locks.semantic-ir.json`.
- library: `models/library.mjs` writes `examples/library/model/library.semantic-ir.json`.
- payment-spec: `models/payment-spec.mjs` writes `examples/shop/model/payment-spec.semantic-ir.json`.
- payment-client: `models/payment-client.mjs` writes `examples/shop/model/payment-client.semantic-ir.json`.

## Identifier rewrite

[rename-ids.mjs](rename-ids.mjs) rewrites the identifiers that the language rename changed (`quire.<family>/v1` profile ids, `quire-outcome/1`, `ix://quire/` identities and the like) in every text file of the repository, and skips `.git`, the licence data and this README, which quotes the old ids. It is repeatable: a second run changes nothing, and `--check` reports without writing. After it runs, `make models record readme index` has the driver write the models and recorded results again. [rename-ids.test.mjs](rename-ids.test.mjs) holds its tests.
