---
title: Check business rules
type: tutorial
---

Run the shop's refund, charge and idempotency rules from [Tutorial: business rules for refunds and charges](https://github.com/agent-ix/filament-docs/blob/main/src/content/docs/fsl/guides/tutorial-business-rules.md).
From the repository root, run `make tutorial-business-rules-test FSL=/absolute/path/to/fsl`.
The steps run in a temporary copy of `examples/shop` and use its model, rules and requests.

## Check a small refund

```bash step=refund-small
"$FSL" run requests/refund-small.json | tee refund-small-result.txt
```

```bash step=assert-refund-small
grep -q 'clause RefundApproval$' refund-small-result.txt
grep -q 'tested (success)' refund-small-result.txt
```

## Check a large refund with one approver

```bash step=refund-large-one expect-exit=10
"$FSL" run requests/refund-large-one-approver.json | tee refund-large-one-result.txt
```

```bash step=assert-refund-large-one
grep -q 'clause RefundApproval$' refund-large-one-result.txt
grep -q 'refuted (violation)' refund-large-one-result.txt
```

## Check a large refund with two approvers

```bash step=refund-large-two
"$FSL" run requests/refund-large-two-approvers.json | tee refund-large-two-result.txt
```

```bash step=assert-refund-large-two
grep -q 'clause RefundApproval$' refund-large-two-result.txt
grep -q 'tested (success)' refund-large-two-result.txt
```

## Check a charge that completes the order

```bash step=charge-completes
"$FSL" run requests/charge-completes.json | tee charge-completes-result.txt
```

```bash step=assert-charge-completes
grep -q 'clause PaidWhenCovered$' charge-completes-result.txt
grep -q 'tested (success)' charge-completes-result.txt
```

## Check a charge that leaves the order open

```bash step=charge-leaves-open expect-exit=10
"$FSL" run requests/charge-leaves-open.json | tee charge-leaves-open-result.txt
```

```bash step=assert-charge-leaves-open
grep -q 'clause PaidWhenCovered$' charge-leaves-open-result.txt
grep -q 'refuted (violation)' charge-leaves-open-result.txt
```

## Check a retry with a new key

```bash step=retry-new-key
"$FSL" run requests/charge-once-retry-new-key.json | tee retry-new-key-result.txt
```

```bash step=assert-retry-new-key
grep -q 'clause ChargeAtMostOnce$' retry-new-key-result.txt
grep -q 'tested (success)' retry-new-key-result.txt
```

## Check a double charge

```bash step=charged-twice expect-exit=10
"$FSL" run requests/charge-once-charged-twice.json | tee charged-twice-result.txt
```

```bash step=assert-charged-twice
grep -q 'clause ChargeAtMostOnce$' charged-twice-result.txt
grep -q 'refuted (violation)' charged-twice-result.txt
```
