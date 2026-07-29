---
id: dec.order-containment-rule
nodes:
  - cairn.kernel.query
  - cairn.kernel.map
status: superseded
date: 2026-07-12
---

# Order query: containment is a hard children-first constraint

Superseded 2026-07-29 by `dec.brownfield-discovery-cycle-severity` (option A of
its ratification protocol; its clause 6 restates every surviving obligation).

## Context

`cairn order` (gh:#237, `todo.order-containment-fix`) sorted over dependency
edges only. Containment never participated, so whether children appeared
before their parent depended on lexicographic key order: `cairn.kernel.*`
happened to sort before `cairn.kernel`, but in a project with no dependency
edges the parent sorted first. The order was total but the containment
relationship was accidental.

## Decision

`topological_order` (src/map/integrity.rs) sorts over a combined precedence
graph via Kahn's algorithm:

1. Dependency edges: a node's dependencies sort before it.
2. Containment: children sort before their parent (a container follows its
   parts, matching the deps-first build order).
3. Ties break by node id, so the result is deterministic and independent of
   declaration or map key order.

Both edge kinds are hard constraints. A contradiction between them (for
example a node with a dependency edge to its own container) is reported as
`CAIRN_ORDER_CYCLE`, exactly like a pure dependency cycle, rather than
silently dropping one constraint and inventing an order.

## Rationale

A soft-containment fallback (drop containment edges on deadlock) was
considered and rejected: it would let `cairn order` return an order that
contradicts the declared blueprint while claiming success, hiding a real
structural contradiction. The blueprint is the declared intent; if its
containment and dependency declarations cannot both be satisfied, that is a
finding, not a tie to break.

## Consequences

- `cairn frontier` and lint reuse `order()`'s traversal and inherit the rule.
- Blueprints where a child declares a dependency on its own ancestor now fail
  `cairn order` with a cycle finding naming the stuck nodes; the fix is to
  remove or invert the contradictory edge.
- `cycle_findings` remains dependency-only and keeps its path-carrying
  message; `query::lint` routes cycle detection through
  `topological_order` (which runs `cycle_findings` first), so a
  combined-constraint contradiction fails lint and hooks, not just
  `cairn order`.
