---
node: tasks.auth
status: open
created: 2026-05-22
---

# Add middleware for auth

Implement middleware that validates tokens on every API request.

## Acceptance

- All protected endpoints require a valid token
- Invalid tokens return 401 Unauthorized
