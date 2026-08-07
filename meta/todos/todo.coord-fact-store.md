---
node: cairn.root
status: blocked
created: 2026-08-07
blocked_by: [todo.parallel-dispatch-granularity, todo.coord-common-dir-helper]
related: [dec.rung-three-coordination-substrate, res.parallel-dispatch-rung-3]
---

# Coordination store: the append-only fact store

Implements `res.parallel-dispatch-rung-3` Part 2 and
`dec.rung-three-coordination-substrate` clauses 2 and 4. Blocked on that
signature: the store's placement, its cross-clone scope, and the required
`evidence_class` field are all maintainer rulings.

Lineage the implementation must not appear to revive (`dec.change-format-only`):
the generic `StateBackend` abstraction was deleted as production-dead, and the
beads backend's create, claim, and sequence methods were deleted because
claiming and sequencing are workflow. No atomic claim path exists to extend.
This store appends a fact when a sanctioned verb runs and does nothing else.

## Task

1. Store layout under `<git-common-dir>/cairn/coord/`: `format`, `facts/`,
   `leases/<unit-id>/epoch-NNNNNN.json`, `singleton/epoch-NNNNNN.json`,
   `cache/`, `archive/`. Writes go through `persist::atomic_write`.
2. The fact envelope: `format`, `fact_id`, `kind`, `recorded_at`, `recorded_by`,
   `commit`, `evidence_class` (required), `supersedes`, `payload`. Payload shapes
   for the ruling, lease, outcome, and singleton families as the design states,
   including the lease `residue` object.
3. Epoch succession for the two records needing mutual exclusion (driver
   singleton, unit lease grant): read the highest existing epoch, then
   `create_new(true)` on the successor. Tokens are never deleted; release is a
   fact.
4. The appender barrier: refuse `lease.*` and `driver.singleton.*` when
   `recorded_by.kind == "console"`.
5. `cairn coord verify` (fact set is a superset of every prior observation, no
   `supersedes` chain has a missing antecedent, no unmatched `ruling.park` and no
   live-chain antecedent has been compacted) and `cairn coord compact --before`,
   which moves to `archive/` and deletes nothing.

## Acceptance

- A test proves two concurrent epoch acquisitions produce exactly one winner.
- A test proves a console-actor `lease.grant` is refused.
- A test proves `verify` fails on a removed antecedent and on a compacted
  unmatched park.
- No new dependency. No `flock`. No git object or ref writes.
