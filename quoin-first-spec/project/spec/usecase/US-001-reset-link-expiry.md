---
id: US-001
title: "Reset links stop working after a short time"
type: US
org: acme
relationships: []
---

# US-001: Reset links stop working after a short time

## Story

**As a** registered user who forgot my password
**I want** the reset link I was emailed to stop working after a short time
**So that** nobody can use the link later if my email is exposed.

## Context

Support has seen reset emails forwarded and left in shared inboxes. A link that works forever turns an old email into a way into the account.

## Acceptance Examples (Illustrative)

### US-001-EX-1: A fresh link works

- **Given** a reset link that was emailed ten minutes ago
- **When** the user opens it and chooses a new password
- **Then** the password is changed

### US-001-EX-2: An old link is refused

- **Given** a reset link that was emailed two hours ago
- **When** the user opens it
- **Then** the user is told the link has expired and can ask for a new one
