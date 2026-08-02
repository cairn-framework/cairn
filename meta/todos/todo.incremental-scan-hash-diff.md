---
node: cairn.kernel.scanner
status: done
created: 2026-07-11
---

# Scanner: content-hash incremental re-scan

Borrowed concept from AutoDocs/Sita and graphify (2026-07-11): hash-diff incremental updates so only changed parts are re-processed. Cairn's reconciler re-scans the full tree each run. Add a per-node (or per-file-set) content hash so an unchanged subtree can be skipped, keeping large-repo and brownfield scans cheap.

Scope:

- Hash derived from node-owned file contents; stored in the derived cache, never as canonical state (files remain truth, cache is rebuildable).
- `cairn scan` skips nodes whose hash is unchanged; a `--full` flag forces a complete re-scan.
- Reconciliation results (synced/ghost/orphaned) must remain a pure function of code and intent; the hash only gates recomputation, never changes results.

Acceptance: scan on an unchanged repo does measurably less work (timed or counted); a mutated file invalidates only its node; results identical with and without the cache on a test fixture.

## Priority (added 2026-07-11 after backlog review)

DEFER until there is a real driver. This is a performance optimisation borrowed
from a competitor; cairn has no large repos yet and full re-scan is not a reported
pain (scans run in well under a second). Keep the design, but do not schedule
ahead of user-facing work; revisit when a real repo makes scan latency felt.

## Mission disposition

2026-08-02: close against dec.cairn-mission. Serves none. A full re-scan is under a second and there is no large-repository evidence for this optimisation.
