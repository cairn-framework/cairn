---
node: cairn.root
status: open
created: 2026-07-15
---

# Repo organisation cleanup

## Problem

The repo feels disorganised. It is hard to see what relates to what. Artefacts from prior states linger: stale or orphaned meta artefacts, superseded-but-open todos, old change directories, leftover files that no longer have a home in the graph.

## Approach: use cairn itself

Files are truth; the graph reconciles. Use cairn to find the mess rather than walking the tree by eye:

1. Surface orphaned research (research not linked from any decision).
2. Surface decisions with no node.
3. Surface superseded todos that are still open.
4. Surface archived or stale change directories.
5. Surface artefacts with broken or missing provenance links.
6. Tighten relate-what-to-what via `informed_by`, `related`, and `satisfies` edges.
7. Archive prior-state artefacts properly rather than deleting blindly (history and provenance stay intact).

## Acceptance

- A concrete cleanup pass with a before/after inventory of artefacts touched.
- `cairn lint` and `cairn scan` introduce no NEW orphan or provenance findings (the pre-existing deferred correlator finding, `dec.revisit-trigger-correlator-deferred`, is out of scope).
- Prior-state artefacts are archived, not lost.

## Non-goals

- No ad-hoc mass deletion.
- Nothing that loses git history or provenance.
