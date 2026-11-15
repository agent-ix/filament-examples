---
id: FR-001
title: "Invitations expire after seven days"
type: FR
relationships:
  - target: "ix://example/invites/US-001"
    type: "satisfies"
---

# FR-001: Invitations expire after seven days

## Description

The service shall accept an invitation only within seven days of the time it was sent, and only once.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-001-AC-1 | An invitation opened one day after it was sent is accepted. | Test |
| FR-001-AC-2 | An invitation opened seven days or more after it was sent is refused. | Test |
| FR-001-AC-3 | An invitation that was already accepted is refused when opened again. | Test |

## Dependencies

- **Upstream**: [US-001](../usecase/US-001-invite-a-teammate.md)
