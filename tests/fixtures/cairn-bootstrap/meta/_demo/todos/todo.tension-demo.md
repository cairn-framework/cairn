---
node: cairn.demo.nonexistent
status: open
created: 2026-05-08
---

# DEMO: rationale-tension trigger (advisory, non-blocking)

This todo deliberately references a node ID (`cairn.demo.nonexistent`) that does not exist in `cairn.blueprint`. When the scanner loads it (via `cairn --file cairn-with-demo.blueprint scan`), it produces a `CAIRN_TODO_ORPHAN_NODE` finding at Warning severity.

`cairn hook all` classifies Warning-severity findings as *rationale tensions*: advisory only, never block a commit. This artefact is the canonical bootstrap example of that channel.

It is NOT a real backlog item. Do not action.
