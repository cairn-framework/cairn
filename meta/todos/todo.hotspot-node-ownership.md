---
node: cairn.kernel.artefacts
status: done
created: 2026-08-07
blocked_by: [todo.parallel-dispatch-granularity]
related: [dec.rung-three-coordination-substrate, res.parallel-dispatch-rung-3]
---

# Hotspot node ownership and the workflow serialises list

Implements `res.parallel-dispatch-rung-3` Part 3, the hotspot half, and
`dec.rung-three-coordination-substrate` clause 3.

Two moves, deliberately separate, because conflating them is how earlier drafts
went wrong.

## Task

1. Declare blueprint nodes owning `docs/registries/`, `cairn.blueprint`, and
   `docs/design-system/copy.toml`, using the existing `path` and `owns_files`
   keywords. No grammar change. This repairs a real ownership gap: those files
   are declared by no node today (`cairn.blueprint:13-139`), so they are
   invisible to ownership entirely, independent of dispatch. Check the effect on
   `most_specific_owner`, on `CAIRN_RECONCILE_ORPHANED_FILE`, and on `cairn map`
   output before landing; a node whose path is `cairn.blueprint` owns the file
   that declares it, and the scanner must be shown to handle that.
2. Add `serialises:` to the inert workflow artefact as a list of **path
   prefixes**, not node ids: a node id cannot name a path no node owns, which is
   the whole problem. Cairn validates that each prefix exists and evaluates
   nothing further, per `dec.orchestration-placement` clause 3.

Declaring the nodes does not attribute a hotspot to a unit, and the
implementation must not pretend otherwise. A unit anchored at
`cairn.kernel.scanner` that appends an error code to
`docs/registries/error-codes.md` has no registry node in its closure, and that
fact does not exist in committed state at composition time.

## Acceptance

- `cairn scan --strict` stays green after the three nodes are declared.
- No per-unit authoring is introduced anywhere.

Note (2026-08-07): task 1 landed via `dec.rung-three-node-declarations`; the
three owner nodes are declared and the scanner effect is gate-asserted by
`tests/rung3_node_declarations.rs`. Task 2 (`serialises:`) is carved out to
`todo.workflow-serialises-validation`, blocked on `todo.driver-in-repo`,
because no workflow artefact type exists yet; acceptance above shrank to
task 1's clauses accordingly.
