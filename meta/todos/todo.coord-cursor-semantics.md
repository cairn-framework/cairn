---
node: cairn.kernel.query
status: done
created: 2026-08-07
related: [res.chatgpt-architecture-review, todo.coord-fact-store-hardening]
---

# Coordination cursor and `since` semantics

Owner for defects 4 and 5 of res.chatgpt-architecture-review: the
same-second cursor gap in the coordination list filter, and `since`
meaning a fact filename for coordination lists but an RFC 3339
timestamp for `wave stats` under one request schema.

## Task

1. Pin same-second cursor behaviour with a test, then fix, or
   explicitly document the cursor as non-incremental.
2. Remove the `since` ambiguity: distinct field names, or typed
   per-family request fields. The smallest fix that removes the
   ambiguity wins.

## Acceptance

- A test pins same-second cursor behaviour; the two `since` meanings
  are no longer expressed through one identically named field, or the
  schema documents both explicitly.
