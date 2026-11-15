---
id: FR-001
title: "Password reset links expire after 30 minutes and are single-use"
type: FR
---
# FR-001: Password reset links expire after 30 minutes and are single-use

## Description

The system SHALL accept a password reset link only once and only within 30 minutes of the time the link was issued, rejecting any link that is expired or already used.

## Inputs

- Password reset token from the reset link
- Issue timestamp recorded when the link was generated

## Outputs

- Password updated and token consumed, or
- Rejection response stating the link is invalid, expired, or already used

## Behavior

- When a password reset is requested, the system shall record the issue time of the generated reset link.
- When a reset link is presented within 30 minutes of its issue time and has not been used, the system shall accept it and mark it as used in the same atomic operation as the password change.
- If a reset link is presented 30 minutes or more after its issue time, then the system shall reject it and leave the password unchanged.
- If a reset link that has already been used is presented, then the system shall reject it and leave the password unchanged.
- If a new reset link is issued for an account, then the system shall invalidate every earlier unused reset link for that account.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-001-CON-1 | The reset service SHALL compute expiry from the server clock, never from a client-supplied value | Security | Test |
| FR-001-CON-2 | Rejection responses SHALL NOT reveal whether the token belongs to an existing account | Security | Test |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-001-AC-1 | Given a link presented 29 minutes 59 seconds after issue and unused, the password is changed and the link is marked used | Test |
| FR-001-AC-2 | Given a link presented 30 minutes 0 seconds or more after issue, the request is rejected with HTTP 410 and the password is unchanged | Test |
| FR-001-AC-3 | Given a link that was already used successfully, a second presentation is rejected with HTTP 410 and the password is unchanged | Test |
| FR-001-AC-4 | Given two concurrent presentations of the same unused link, exactly one succeeds and the other is rejected | Test |
| FR-001-AC-5 | Given a link expired before first use, the link remains rejected and a later presentation never changes the password | Test |
| FR-001-AC-6 | Given a newer link issued for the same account, presenting the older unused link is rejected with HTTP 410 | Test |
| FR-001-AC-7 | Given an unknown, malformed, or expired token, the rejection response is identical in status and body | Test |

## Dependencies

- **Upstream**: none (no driving user story authored yet)
- **Downstream**: none
