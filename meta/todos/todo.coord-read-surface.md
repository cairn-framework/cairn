---
node: cairn.kernel.query
status: done
created: 2026-08-07
blocked_by: [todo.coord-fact-store]
related: [dec.rung-three-coordination-substrate, res.parallel-dispatch-rung-3]
---

# Coordination store: the raw read surface

Implements `res.parallel-dispatch-rung-3` Part 2, the read half. The core
evaluates nothing: this surface returns raw facts and the reader derives every
projection.

## Task

1. `src/query_api/handlers/coordination.rs`, registered in `TOOL_REGISTRY` and
   dispatched through `execute_data_with_scan`, stamping `schema_version`.
2. Verbs: `cairn ruling list|show`, `cairn lease list`, all with `--json`,
   `--at <RFC3339>`, and `--since <filename>`.
3. The response carries no `active`, no `expired`, no `stale`, no `status`. It
   echoes `observed_at` exactly as supplied and echoes `null` when none is
   supplied, consulting no clock. It carries `store_state`
   (`uninitialised|ready`, and a read never creates the store), `cursor`,
   `truncated`, and `conflicts: []`. A read that cannot fully resolve the store
   fails closed.
4. There is no `--unresolved` flag on the wire. Resolution is a renderer-side
   join so the console and the driver share one set of reader predicates.
5. The three reader predicates as shared helpers: `held(unit, at)`,
   `stale(unit, at)` carrying holder, `expires_at` and residue, and
   `no_lease(unit)` as a distinct peer.
6. Read cost: every read lists and parses `facts/` in full. There is no
   incremental fold above a high-water mark and no parsed-envelope cache.
   Filenames are second-precision and atomic creation can land after listing
   starts, so a same-second fact can sort below an already-taken mark and be
   lost by a high-water reader. The full listing avoids that gap; the lost fact
   would be a `lease.grant` and the driver would dispatch over a held claim.

## Acceptance

- A regression test writes two facts in the same second whose names sort in the
  opposite order to their rename order, and asserts both are returned.
- A test asserts the response has no derived verdict field and that `observed_at`
  is `null` when the caller supplies none.
- A test asserts a read against a missing store returns
  `store_state: "uninitialised"` and creates nothing.
- Wire snapshots rebased; `schema_version` bumped if the shape requires it.
- Follow-up: `ruling show --json` still omits the contract's `cursor` and
  `truncated` fields.
- Follow-up (`todo.coord-archive-read-surface`): after `coord compact`, ruling
  list/show and lease projections read live facts only, so archived history
  drops out of the read surface; route readers through the validated
  live+archive helper or amend the rung-3 history-channel wording.
