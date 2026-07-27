---
id: dec.brownfield-init-round-trip
nodes:
  - cairn.brownfield
  - cairn.kernel.changes
status: accepted
date: 2026-06-26
---
# Brownfield init round-trip: canonical delta, seeded base blueprint, post-delta validation

## Decision

`cairn init --from-code` now produces a discover-to-archive round-trip that actually
lands discovered modules in `cairn.blueprint` (regression cairn-e12). Three coupled
choices make this work:

1. **Canonical delta emission.** The brownfield emitter (`blueprint_delta`) writes the
   canonical `## ADDED Nodes` and `## ADDED Edges` sections the change parser
   (`src/changes/delta.rs`) reads, replacing the earlier `+ Module ...` form that no
   parser accepted. Node names collapse to a single bareword (for example "api gateway"
   becomes "ApiGateway") because the grammar's name slot is a bareword, not a string.

2. **Seeded comment-only base blueprint.** When no `cairn.blueprint` exists,
   `init --from-code` seeds a comment-only file. It parses to an empty graph, so
   archiving the brownfield change merges the discovered modules in as top-level
   nodes. We deliberately do not seed a placeholder `System` root: an empty-bodied root
   would become an orphan island in `cairn islands`, whereas flat discovered modules
   plus their inferred edges form a faithful cold-start map. Nesting modules under
   systems and containers is left to refinement.

3. **Post-delta artefact validation.** `validate_change` resolves artefact `node:` and
   `nodes:` references against the post-delta node set (the existing graph plus the
   change's added nodes), not the pre-delta graph. A brownfield contract stub
   references a node the same change adds, so pre-delta validation wrongly rejected it.

Contract stubs carry `operation: added` frontmatter (closing P-05), enforced for both
the built-in stub and templated stubs so every brownfield contract passes archive
validation.

## Scope

`cairn refine` shares the same delta-format defect through its own emitter
(`blueprint_delta_with_renames`). That is tracked separately (cairn-h8o) and also needs
canonical `## REMOVED Nodes` and `## RENAMED Nodes` sections.
