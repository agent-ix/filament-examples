---
title: Generate and compile a library model
type: tutorial
---

Generate the model from [Write a model](https://github.com/agent-ix/filament-docs/blob/main/src/content/docs/fsl/guides/write-a-model.md), then compile its rule.
From the repository root, run `make tutorial-write-model-test FSL=/absolute/path/to/fsl`.
The steps run in a temporary copy of this repository.

## Generate the model

The library description declares bounded copy counts, loan status, books and their population.
Generate the model file that the compile request names.

```bash step=generate-library-model
node tools/model.mjs tools/models/library.mjs examples/library/model/library.semantic-ir.json
cd examples/library
```

## Compile the rule

The rule uses the model's declared `copies` field.

```bash step=compile-library
"$FSL" compile requests/compile-ok.json --format json | tee compile-library-result.json
```

```bash step=assert-compile-library
node --input-type=module <<'JS'
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
const result = JSON.parse(readFileSync('compile-library-result.json', 'utf8'));
assert.equal(result.format, 'filament-outcome/1');
assert.equal(result.category, 'success');
assert.equal(result.last_stage, 'S4');
assert.deepEqual(result.diagnostics, []);
assert.deepEqual(result.items, []);
console.log('OnLoanHasCopies compiles against the generated library model');
JS
```

## Refuse an undeclared field

The misspelled rule reads `copy`, which the model does not declare.
The tutorial expects a type refusal before any data is read.

```bash step=compile-misspelled expect-exit=20
"$FSL" compile requests/compile-misspelled.json --format json | tee compile-misspelled-result.json
```

```bash step=assert-compile-misspelled
node --input-type=module <<'JS'
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
const result = JSON.parse(readFileSync('compile-misspelled-result.json', 'utf8'));
assert.equal(result.format, 'filament-outcome/1');
assert.equal(result.category, 'refusal');
assert.equal(result.last_stage, 'S3');
assert.ok(result.diagnostics.length > 0, 'the undeclared field has a diagnostic');
assert.equal(result.diagnostics[0].code, 'ill_typed');
assert.deepEqual(result.items, []);
console.log('The misspelled field is refused before evaluation');
JS
```
