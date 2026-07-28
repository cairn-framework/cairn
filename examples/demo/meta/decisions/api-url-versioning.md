---
id: dec.api-url-versioning
nodes:
  - tasks.api
status: accepted
date: 2026-05-22
informed_by: [src.api-design]
---

# Ship the task endpoints unversioned until a second client exists

`tasks.api` exposes `POST /tasks`, `GET /tasks`, and `GET /tasks/:id`, with no
version segment, as `meta/contracts/api.md` states.

The guidance in `src.api-design` is to version only when a breaking change is
unavoidable. The demo has one client and no released contract, so a `/v1` prefix
would carry cost with nothing depending on it. When a breaking change does
arrive, the version goes in the URL path, where a log line shows it, rather than
in a negotiated header.
