---
id: RB-002
title: Rotate the signing keys
type: Runbook
relationships:
  - target: ix://RB-001
    type: runs_after
---
# RB-002: Rotate the signing keys

## Purpose

Replace the signing keys without refusing a job. Run RB-001 first when the queue is stopped.

## Steps

- Create the new key pair.
- Publish the new public key.
- Switch the signer to the new private key.
