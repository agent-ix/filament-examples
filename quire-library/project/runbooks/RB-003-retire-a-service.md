---
id: RB-003
title: Retire a service
type: Runbook
relationships:
  - target: ix://RB-009
    type: runs_after
---
# RB-003: Retire a service

## Purpose

Remove a service that no job uses. Finish RB-002 first, as FR-001 requires.

## Steps

- Stop the service.
- Remove its keys.
