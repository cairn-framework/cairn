---
node: cairn.kernel.query
status: in_progress
created: 2026-07-31
blocked_by: [todo.todo-relationship-schema-implementation]
related: [dec.todo-relationship-model]
---

# Roadmap as a derived view over todo edges

`dec.todo-relationship-model` ruling 5: the roadmap is a computed
projection, never an authored artefact. This todo owns the projection and
its surfaces. Blocked on the schema implementation (the edges must parse
before a view can traverse them); the `blocked_by:` entry above is
forward-declared in the new syntax and becomes typed when that unit lands.

## Task

1. Compute the projection per ruling 5 of `dec.todo-relationship-model`
   (the single normative copy): todos whose status is not `done`,
   topological tiers from `blocked_by` only (`parent` groups, it never
   orders), grouped by `parent`, WorkItem rank within a tier.
2. Surface it in the CLI with `--json` (either a new verb or folded into
   an existing one such as `frontier`; follow the command-reference
   consistency tests either way and document the choice against
   `dec.frontier-query`, which tiers ghost nodes the same way).
3. Upgrade the webui backlog channel from a flat list to the projection
   (tiers and parent grouping legible; design-system tokens only).
4. Feed `cairn pending`'s unblock sort, the consumer
   `dec.north-star-continuous-loop` goal 5 names for these edges.

## Acceptance

- The projection renders this repository's own backlog with at least the
  forward-declared edges resolving into tiers.
- `--json` output ships with a schema entry per `todo.wire-format-schemas`
  conventions; wire snapshots regenerated if versions bump.
- Webui backlog channel shows tiers and parents; token gate and biome
  pass.

## Task 4 gap (2026-08-02)

The pending unblock sort cannot ship from typed edges: `blocked_by`
resolves todo stems only (ruling 2) and `related:` is weak and
non-directional (rulings 1 and 3), so no typed edge expresses "this
pending decision unblocks that todo". Counting `related:` would fabricate
unblock semantics and score the live case (dec.control-plane-programme
gating the console) at zero. This is the relationship kind the trio
cannot express; reopening condition is revisit trigger 2 of
`dec.todo-relationship-model` (a schema amendment, not a stretch of
`related:`). The pending queue keeps its age sort;
`todo.pending-queue-briefing` owns the queue surface.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves fit-for-purpose. It is campaign step 3 for the live work projection.
