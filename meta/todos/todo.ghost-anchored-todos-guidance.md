---
node: cairn.root
status: open
created: 2026-07-31
related: [res.inversion-convergence-minutes]
---

# Ghost-anchored todos: guidance rule

`res.inversion-convergence-minutes` row R3. Frontier is empty (zero ghost
nodes) while 21+ todos are open: the forward programme is invisible to
the graph's own buildability view because new-capability work is authored
as todos on existing nodes, never as declared future structure.

## Task

Land the discipline as guidance in the agent pack and the cairn-dev
references: a todo whose work implies new structure (a new module,
container, or dependency edge) anchors to declared ghost node(s) at
authoring time, so `cairn frontier` tiers the structural future while the
todo DAG (`dec.todo-relationship-model`) carries the work ordering. Todos
on existing behaviour anchor to existing nodes, as today. Ghost nodes
remain a process step (declared before build, materialised at build,
gated by scan); this rule makes them also the roadmap's structure axis.

Pack content is binding surface (`dec.decision-ratification-tiers`), so
the guidance edit lands with maintainer signature; enqueue via
`cairn pending` if authored by the loop.

## Acceptance

- The rule appears in the agent pack source
  (`tools/agent-pack/content/`) and rendered assets, and in the cairn-dev
  reference the router points at for todo authoring.
- The next new-capability todo authored after the rule lands cites a
  ghost node, and `cairn frontier` shows it.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves investigable. It fills the empty frontier lane with guidance for declaring future structure.
