# Tools

This folder holds the example runner, the interim model generator and the install-test runner.

## Runner

[run.mjs](run.mjs) reads each `example.toml`, runs the cases through the FSL driver named by the `FSL` variable and compares the exit code and output with the manifest and `expected/`. The Makefile calls it as `make check`, `make staged`, `make record`, `make promote`, `make readme` and `make index`. [run.test.mjs](run.test.mjs) holds its unit tests, which run with `make test`.

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

Regenerate all eight models with `make models`. The output is identical to the committed files, so `git diff` shows nothing when a description has not changed.

- shop: `models/shop.mjs` writes `examples/shop/model/shop.semantic-ir.json`.
- architecture: `models/architecture.mjs` writes `examples/architecture/model/architecture.semantic-ir.json`.
- parts-saga: `models/parts.mjs` writes `examples/parts-saga/model/parts-saga.semantic-ir.json`.
- cell: `models/cell.mjs` writes `examples/cell/model/cell.semantic-ir.json`.
- rover: `models/rover.mjs` writes `examples/rover/model/rover.semantic-ir.json`.
- rover-spec: `models/rover-spec.mjs` writes `examples/rover/model/rover-spec.semantic-ir.json`.
- library: `models/library.mjs` writes `examples/library/model/library.semantic-ir.json`.
- payment-spec: `models/payment-spec.mjs` writes `examples/shop/model/payment-spec.semantic-ir.json`.
- payment-client: `models/payment-client.mjs` writes `examples/shop/model/payment-client.semantic-ir.json`.
