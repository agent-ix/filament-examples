---
type: reference
---

# Grammar cases

Grammar is a source-level corpus for the FSL parser. Each case submits a source file and compares its outcome with a recorded result.

Run the cases from the repository root:

```sh
cargo run --quiet --locked --manifest-path tools/fsl-examples/Cargo.toml -- check grammar
```

Set `FSL` to the driver binary. The manifest selects valid syntax and malformed input; the recorded result checks the parse report or refusal code.

The corpus covers identifiers, strings, comments, line endings, expression operators, calls, collections, type declarations, model selections, and claims. Temporal and protocol cases exercise their source forms. Parsing a declaration leaves its name resolution and type checking to the later stages.

Evaluation cases call model-independent functions to check operator precedence, associativity, comparisons, conditionals, and lexical bindings against concrete result values.

The source files carry requirement references. [example.toml](example.toml) lists each case and its expected exit code. [rules](rules) contains the submitted source, and [expected](expected) contains the recorded outcomes.
