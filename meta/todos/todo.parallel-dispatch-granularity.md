---
node: cairn.root
status: open
created: 2026-07-31
related: [res.inversion-convergence-minutes, todo.node-overlap-conflicts-query]
---

# Parallel dispatch granularity: name the three rungs, design the third

`res.inversion-convergence-minutes` row R2. For concurrent units to land
as mergeable PRs, the dispatcher needs computable disjointness, and the
repository currently offers none of it in typed form.

## The three rungs (design constraint, not implementation order)

1. **Order**: typed `blocked_by` edges (`dec.todo-relationship-model`)
   give topological waves. Owned by
   `todo.todo-relationship-schema-implementation`.
2. **Advisory overlap**: the one-hop conflicts query
   (`todo.node-overlap-conflicts-query`), committed state only. This is
   a warning precursor, explicitly NOT merge-safety: it cannot see
   shared files, registries, blueprint edits, generated assets, or
   unpushed claims.
3. **Merge-safety**: a write-set/lease model plus a shared multi-ref
   derived index (`res.overharness-design-threads` thread c; the B-queue
   md5 ledger is the acknowledged single-writer prototype). Canonicity
   never moves: the index is derived and disposable.

## Task

Research and design rung 3 under the driver-v2 umbrella: how a unit
declares or derives its write-set, how leases are granted and observed
across worktrees, and how the serialisation hotspots get explicit
ownership (docs/registries/, cairn.blueprint,
docs/design-system/copy.toml, wire snapshots: the files every unit
touches). Ratified slate constraint the eventual decision must carry
(res.inversion-convergence-minutes fork note): start with derived
node-closure over committed state (zero new authoring burden), promote
to declared write-sets only on measured false-overlap evidence.
Unratified candidate from the slate's post-ratification intake:
every derived fact carries source, extractor plus version, observed_at,
freshness, and completeness; deterministic, attested, and observed are
distinct evidence classes and never blur. Output is a design plus an
enqueued decision, not code.

## Acceptance

- A design document (research artefact) covering derivation, grant,
  observation, and hotspot ownership, with the derived-first ruling and
  its promotion trigger stated.
- The rung vocabulary above appears verbatim, so no consumer mistakes
  rung 2 for rung 3.
- Follow-up implementation todos filed with `blocked_by` edges.
