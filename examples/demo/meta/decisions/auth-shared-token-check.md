---
id: dec.auth-shared-token-check
nodes:
  - tasks.auth
status: accepted
date: 2026-05-22
---

# Token verification lives in tasks.auth, not in tasks.api

Every request arrives through `tasks.api`, so the check could live there. It does
not: `tasks.auth` owns credential lookup against `tasks.db` already, and one
owner for both halves of authentication keeps the rule in a single place.

The demo therefore declares the `tasks.api -> tasks.auth` edge rather than
inlining a token check in the API module.
