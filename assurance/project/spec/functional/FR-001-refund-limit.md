---
id: FR-001
title: "Refuse a refund above the amount paid"
type: FR
relationships:
  - target: "ix://example/shop/StR-001"
    type: "satisfies"
---

# FR-001: Refuse a refund above the amount paid

## Description

When a refund is requested for an order, the shop shall refuse the refund if its amount is larger than the amount paid for the order.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-001-AC-1 | A refund of at most the amount paid is accepted. | Test |
| FR-001-AC-2 | A refund above the amount paid is refused. | Test |

## Dependencies

- **Upstream**: [StR-001](../stakeholder/StR-001-refunds-never-overpay.md)
