---
title: Run a first rule
type: tutorial
---

Run `PaidInFull` on three orders from [A first rule](https://github.com/agent-ix/filament-docs/blob/main/src/content/docs/fsl/learn/first-rule.md).
From the repository root, run `make tutorial-first-rule-test FSL=/absolute/path/to/fsl`.
The steps run in a temporary copy of `examples/shop` and use its model, rule and requests.

## Check a fully paid order

The order is paid and its charged amount equals its total. The rule holds.

```bash step=paid-in-full
"$FSL" run requests/paid-in-full.json --format json | tee paid-in-full-result.json
```

```bash step=assert-paid-in-full
node --input-type=module <<'JS'
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
const result = JSON.parse(readFileSync('paid-in-full-result.json', 'utf8'));
assert.equal(result.format, 'filament-outcome/1');
assert.equal(result.category, 'success');
assert.equal(result.last_stage, 'S6a');
assert.deepEqual(result.diagnostics, []);
assert.equal(result.items.length, 1);
const [item] = result.items;
assert.equal(item.clause, 'PaidInFull');
assert.equal(item.request_index, 0);
assert.equal(item.category, 'success');
assert.equal(item.result, 'tested');
assert.deepEqual(item.axes, {
  activation: 'active', execution: 'completed', settlement: 'closed-scope', truth: 'true',
});
console.log('PaidInFull: tested on the fully paid order');
JS
```

## Check an order paid short

The order is still paid, but 4000 was charged against a total of 4200. The rule is violated.

```bash step=paid-short expect-exit=10
"$FSL" run requests/paid-short.json --format json | tee paid-short-result.json
```

```bash step=assert-paid-short
node --input-type=module <<'JS'
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
const result = JSON.parse(readFileSync('paid-short-result.json', 'utf8'));
assert.equal(result.format, 'filament-outcome/1');
assert.equal(result.category, 'violation');
assert.equal(result.last_stage, 'S6a');
assert.deepEqual(result.diagnostics, []);
assert.equal(result.items.length, 1);
const [item] = result.items;
assert.equal(item.clause, 'PaidInFull');
assert.equal(item.request_index, 0);
assert.equal(item.category, 'violation');
assert.equal(item.result, 'refuted');
assert.deepEqual(item.axes, {
  activation: 'active', execution: 'completed', settlement: 'closed-scope', truth: 'false',
});
console.log('PaidInFull: refuted on the underpaid order');
JS
```

## Check an open order

The open order has an unpaid balance. The implication holds because the order is not paid.

```bash step=open-order
"$FSL" run requests/paid-in-full-open.json --format json | tee open-order-result.json
```

```bash step=assert-open-order
node --input-type=module <<'JS'
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
const result = JSON.parse(readFileSync('open-order-result.json', 'utf8'));
assert.equal(result.format, 'filament-outcome/1');
assert.equal(result.category, 'success');
assert.equal(result.last_stage, 'S6a');
assert.deepEqual(result.diagnostics, []);
assert.equal(result.items.length, 1);
const [item] = result.items;
assert.equal(item.clause, 'PaidInFull');
assert.equal(item.request_index, 0);
assert.equal(item.category, 'success');
assert.equal(item.result, 'tested');
assert.deepEqual(item.axes, {
  activation: 'active', execution: 'completed', settlement: 'closed-scope', truth: 'true',
});
console.log('PaidInFull: tested on the open order');
JS
```
