---
id: dec.stable-ids
nodes: [cairn.kernel.parser, cairn.kernel.artefacts]
status: accepted
date: 2026-04-13
revisited: 2026-04-13
revisit_triggers:
  - "ID convention proves too rigid in practice (e.g., IDs cannot be moved across containers)"
  - "A second reconciler surfaces a need for namespace-like ID semantics not covered by dotted notation"
informed_by:
  - type: source
    id: src.review-adversarial-1
---

# dec.stable-ids: All nodes and artefacts require stable dotted IDs

## Context

v0.4 referenced nodes by name and path. Both are unstable: names get renamed for clarity, paths get reorganized, and two containers can legitimately have a Module called Auth. References in artefact frontmatter that rely on names or paths break silently when either changes.

## Decision

Every node and every cross-referenced artefact carries a stable ID. IDs are dotted, lowercase, and unique across the project. Names remain for display and can change freely; IDs cannot. Edges reference IDs, not names. Artefact frontmatter uses typed ID references ({type: research, id: "res.crypto-sharing"}).

The CLI accepts either names or IDs for human convenience; internal representation is always ID.

## Consequences

- Renames are safe. Reorganizing the DSL does not break artefact references.
- Duplicate display names become legal as long as IDs differ.
- One more field to author per node. The grammar gets slightly heavier; the ID convention needs documentation.
- Research and decision linking unifies around typed ID references, eliminating the v0.4 inconsistency where research used `sources: [path]` and decisions used `informed_by: [typed objects]`.
