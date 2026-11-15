---
id: RB-001
title: Restart the job queue
type: Runbook
---
# RB-001: Restart the job queue

## Purpose

Bring the job queue back after it stops accepting work.

## Steps

- Drain the workers.
- Restart the queue service.
- Confirm that new jobs are accepted.
