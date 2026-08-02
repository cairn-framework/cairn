---
node: cairn.kernel.artefacts
status: open
created: 2026-08-02
related: [dec.todo-relationship-model, todo.pending-queue-briefing]
---

# Decision Unblock Edge Amendment

`dec.todo-relationship-model` revisit trigger 2 fired 2026-08-02: the
pending-queue unblock sort (`dec.north-star-continuous-loop` goal 5)
needs a relationship kind the trio cannot express. `blocked_by` resolves
todo stems only (ruling 2) and `related:` is weak and non-directional
(rulings 1 and 3), so no typed edge states "this pending decision
unblocks that todo"; the live case is `dec.control-plane-programme`
gating `todo.overharness-console-ux`. Stretching `related:` to carry it
is the named anti-pattern; the reopening condition is a schema
amendment.

## Task

1. Author the schema-amendment decision for a typed decision-to-todo
   unblock edge (shape, direction, resolution rules), refining
   `dec.todo-relationship-model`; enqueue it for maintainer signature.
2. Implement parse, validation, and wire surfaces for the new edge under
   that signed amendment.
3. Deliver the transferred Task 4 of `todo.roadmap-derived-view`: feed
   `cairn pending`'s unblock sort from the new typed edges.

## Acceptance

- The amendment decision is signed; the edge parses and validates.
- `cairn pending` orders by what each decision unblocks, traceably from
  typed edges, replacing the age-only sort.
