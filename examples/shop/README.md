# Shop

The shop is an example system for an online shop that takes orders, charges payments and issues refunds. Staff approve refunds, and a large refund needs a second person.

The shop teaches values first, then rules about data, then rules about operations. You can run each rule against a passing and a failing snapshot, and see what the driver reports for data that is wrong, missing or incomplete.

## The model

- `Order` has a total, an amount charged, a status, its charges and its refunds.
- `Charge` has an idempotency key and an amount.
- `Refund` has an amount, an approver and an optional second approver.
- `Staff` has a badge number.
- The operation `charge(amount)` changes the amount charged and the status. The operation `refund(amount, approvers)` adds to the refunds.

The model is in [model/shop.semantic-ir.json](model/shop.semantic-ir.json).

## Values

The functions in [rules/values.fsl](rules/values.fsl) compute a line total and add shipping. Quantities are limited to 1 to 99, and prices to 0 to 1,000,000 cents.

| Case | What it shows | Exit |
| --- | --- | --- |
| `values-line-total` | 1250 cents times 3 is 3750 | 0 |
| `values-with-shipping` | A 3750 cent subtotal costs 4249 with shipping | 0 |
| `values-free-shipping` | A 3750 cent subtotal does not ship free | 0 |
| `values-qty-zero` | Quantity 0 is outside its type | 20 |
| `values-too-narrow` | A result type too small for price times quantity | 20 |

## Rules about data

The rules in [rules/orders.fsl](rules/orders.fsl) check one object of a snapshot. The table lists each rule with its requirement and a passing and a failing case.

| Rule | Requirement | Passing | Failing |
| --- | --- | --- | --- |
| `PaidInFull` | A paid order was charged its full total | `paid-in-full` | `paid-short` |
| `RefundsWithinCharge` | Refunds never exceed what was charged | `refunds-within-charge` | `over-refunded` |
| `ChargeAtMostOnce` | No two charges share an idempotency key | `charge-once-retry-new-key` | `charge-once-charged-twice` |
| `TwoPeople` | The second approver is a different person | `two-people-ok` | `two-people-same` |

Two more cases show data the driver cannot judge. `dangling-charge` refers to a charge that is not in the data and exits 20. `partial-data` has an incomplete population and exits 22. Neither is a failed rule.

## Rules about operations

The rules in [rules/operations.fsl](rules/operations.fsl) check an operation call with the state before and after it.

| Rule | Requirement | Passing | Failing |
| --- | --- | --- | --- |
| `RefundApproval` | A refund over 500 dollars needs two approvers | `refund-large-two-approvers` | `refund-large-one-approver` |
| `PaidWhenCovered` | A charge that covers the total leaves the order Paid | `charge-completes` | `charge-leaves-open` |
| Frame of `charge` | A charge changes only the amount charged and the status | `charge-frame-ok` | `charge-changes-total` |

The case `refund-small` is a refund under the limit with one approver, and it passes. The case `charge-changes-total-post` exits 20 because the charge breaks its frame, and the driver declines to judge the postcondition.

## Run a passing and a failing case

From this folder, with `FSL` set to the driver:

```
$FSL run requests/paid-in-full.json --format json
```

The driver exits 0 and reports the rule as tested:

```
{"category":"success","clause":"PaidInFull","request_index":0,"result":"tested"}
```

Run the same rule against an order that is paid but short of its total:

```
$FSL run requests/paid-short.json --format json
```

The driver exits 10 and reports the rule as refuted:

```
{"category":"violation","clause":"PaidInFull","request_index":0,"result":"refuted"}
```

The two request files differ only in the data file they name. Run all cases with `make check FSL=$FSL`.

## Staged

The case `staged-charge-adds-amount` states that a charge adds exactly its amount. The checker cannot prove the arithmetic yet, so the case stays staged. See `example.toml` for the reason.
