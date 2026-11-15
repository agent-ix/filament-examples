---
id: FR-001
title: "Expire password reset links"
type: FR
org: acme
relationships:
  - target: "ix://acme/widgets/US-001"
    type: "implements"
---

# FR-001: Expire password reset links

## Description

The system SHALL accept a password reset link for 30 minutes after issuing it and for one use only.

## Behavior

- When a user opens a reset link less than 30 minutes after it was issued, the system SHALL accept the new password.
- If a user opens a reset link 30 minutes or more after it was issued, then the system SHALL refuse it and report that it expired.
- If a user opens a reset link that was already used, then the system SHALL refuse it.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-001-AC-1 | Given a link issued 29 minutes ago, the new password is accepted. | Test |
| FR-001-AC-2 | Given a link issued 30 minutes ago, the link is refused as expired. | Test |
| FR-001-AC-3 | Given a link that was already used, the link is refused. | Test |
| FR-001-AC-4 | The reset email states the time at which the link expires. | Inspection |

## Dependencies

- **Upstream**: [US-001](../usecase/US-001-reset-link-expiry.md) reset links stop working after a short time
