---
id: src.summariser-accept-path
file: src/summariser/
verification: tracked
type: in-repo code read
date: 2026-07-28
---

# The summariser draft-acceptance path, read at `00c212a`

In-repo code inspected while designing
`meta/changes/contract-node-shape-drift/`. The claims below are the state at
`00c212a` on `main`; the paths are cited `tracked` and read as they stand, so
re-read the files for the current behaviour.

- `src/summariser/accept.rs`: `accept()` installs a contract's canonical text,
  records `accepted_interface_hash` as a hash of that text into the draft store
  under `.cairn/state/summariser/`, and runs a post-write scan with rollback. It
  is the only writer of accepted contract state. The hash it stores is unrelated
  to `.cairn/state/interface-hashes.json`, which holds code-target hashes the
  scanner writes.
- `src/summariser/store.rs`: `validate_transition` treats `Accepted` and
  `Discarded` as terminal, returning `InvalidTransition` for every outgoing
  transition. An accepted draft can never be re-accepted.

Draft generation itself lives outside this path and is recorded separately as
`src.query-api-draft-generation`.
