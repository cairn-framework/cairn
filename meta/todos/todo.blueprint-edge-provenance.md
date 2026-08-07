---
node: cairn.kernel.blueprint
status: open
created: 2026-08-07
parent: todo.brownfield-nested-package-scan-clean
related: [dec.brownfield-discovery-cycle-severity, dec.brownfield-init-round-trip]
---

# Make an edge cairn inferred distinguishable from an edge a human wrote

Implementation unit split out of `todo.brownfield-nested-package-scan-clean`
under the sizing rule. It carries clause 4 of
`dec.brownfield-discovery-cycle-severity`: "edges written by `blueprint_delta`
must be distinguishable from edges a human wrote. That marker is the
implementation prerequisite."

Nothing about severity changes here, and no finding gains or loses a code. This
unit only makes provenance representable and round-trippable.

## Verified facts

1. There is no provenance today. `blueprint::ast::Edge`
   (`src/blueprint/ast.rs:94-103`) holds `from`, `to`, `description`, `span`,
   and `map::graph::EdgeRef` (`src/map/graph.rs:133-140`) holds `from`, `to`,
   `description`. An inferred edge and a hand-written one are byte-identical.
2. `blueprint_delta` (`src/brownfield/mod.rs:153-167`) writes discovered edges
   as `{from} -> {to} {description:?}` under a `## ADDED Edges` header, which is
   the same canonical form `write_edge` (`src/changes/apply/preserve.rs:353-355`)
   emits when applying a change.
3. The delta pipeline needs no separate grammar. `parse_edge_section`
   (`src/changes/delta.rs:67-79`) parses the section with the ordinary
   blueprint parser (`parse_str`), so an edge-level marker in the blueprint
   grammar reaches the delta path for free. Apply then merges through
   `preserve.rs:137-141` and re-serialises with `write_edge`, so the writer must
   round-trip the marker or apply silently strips it.
4. Node-level tag syntax already exists (`@tag`, `ast::Node.tags`), so the
   grammar has a precedent to follow rather than a new concept to invent.

## Task

- Choose and land an edge-level provenance marker in the blueprint grammar.
  Follow the existing `@tag` precedent unless there is a reason not to; state
  the reason in the PR if you deviate. The marker must be legible to a human
  reading `cairn.blueprint`, since the blueprint is an auditable intent IR.
- Thread it through: `blueprint::ast::Edge`, the lexer and parser
  (`src/blueprint/`), `map::graph::EdgeRef` and the map builder, and
  `write_edge` in `src/changes/apply/preserve.rs` so apply round-trips it.
- Emit it from `blueprint_delta` (`src/brownfield/mod.rs`) for every edge
  discovery derived, and only for those.
- Absence means hand-declared. Every existing blueprint keeps parsing unchanged
  and every existing edge keeps its current meaning; this is an additive
  grammar change, not a migration.
- Update the grammar reference in
  `.claude/skills/cairn-dev/references/blueprint-syntax.md` and any other
  surface that documents edge syntax.

## Acceptance

- A parser test pins that an edge carrying the marker parses, that an edge
  without it parses unchanged, and that the marker survives a
  parse-serialise-parse round trip through `write_edge`.
- A test pins an end-to-end `cairn change apply` of a brownfield-shaped
  `## ADDED Edges` section: the marker reaches `cairn.blueprint` and then the
  built graph's `EdgeRef`, not just the AST.
- A test pins that both observed edge directions survive discovery unmodified,
  so a future edge-suppression shortcut fails loudly. This is acceptance
  bullet 6 of the parent todo.
- The dogfood `cairn.blueprint`, whose edges are all hand-declared, is
  unchanged, and `cairn scan --strict` exits 0.
- `cargo test` and `cargo clippy --all-targets --all-features -- -D warnings`
  pass.

## Non-goals

Any change to which edges discovery emits (clause 1 of the decision is that it
keeps emitting all of them), any nesting change (clause 2), and any severity
change. Severity is `todo.order-cycle-discovery-severity`.
