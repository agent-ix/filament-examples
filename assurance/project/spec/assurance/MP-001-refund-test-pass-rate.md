---
id: MP-001
title: Refund test pass rate
type: MeasurementPlan
status: active
owner: shop-team
metric: refund_test_pass_rate
definition_version: shop.refund-pass-rate-v1
stage: baseline
statistical_design:
  population: refund test cases in the unit suite
  minimum_population: 2
  sampling: every refund test case
  repetitions: 1
  estimator: proportion
  error_model: none; the suite is deterministic
  uncertainty: report the count of passing and failing cases
  decision_rule:
    comparator: ge
    threshold: 1.0
relationships:
  - target: ix://example/shop/FR-001
    type: measures
---

# Refund test pass rate

## Decision Use

The result shows whether every refund test passes on the measured revision.

## Population

Every refund test case in the unit suite.

## Collection Procedure

Run the unit suite once and count the passing refund test cases.

## Interpretation

A rate below 1.0 means at least one refund test failed.
