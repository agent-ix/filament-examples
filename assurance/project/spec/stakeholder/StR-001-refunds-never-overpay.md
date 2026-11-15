---
id: StR-001
title: "Refunds never pay out more than the customer paid"
type: StR
relationships:
  - target: "ix://example/shop/FR-001"
    type: "satisfied_by"
---

# StR-001: Refunds never pay out more than the customer paid

## Stakeholder Need

Every refund the shop records shall be no larger than the amount the customer paid for the order.

## Rationale

A refund above the amount paid is a loss that the shop's accounts cannot explain.

## Validation Criteria

| ID | Criteria | Validation |
|----|----------|------------|
| StR-001-VC-1 | No refund recorded for an order exceeds the amount paid for that order. | Demonstration |
