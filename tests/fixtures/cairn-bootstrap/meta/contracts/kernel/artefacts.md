---
node: cairn.kernel.artefacts
informed_by:
  - type: decision
    id: dec.stable-ids
  - type: decision
    id: dec.two-chain-authority
---

# Contract: cairn.kernel.artefacts

The Artefacts module owns the typed artefact registry: it loads decision, research, todo, review, source, and contract files from the pointers the blueprint declares and validates each against its type schema.

## Interface

- **Input.** Artefact pointer paths from parsed blueprint nodes, plus the artefact files those pointers resolve to.
- **Output.** Typed artefact records (id, type, anchoring nodes, frontmatter fields) attached to graph nodes for downstream queries.
- **Errors.** A pointer that resolves to nothing, an unparseable frontmatter block, or a reference to an unknown node surfaces as a scan finding; loading never aborts the scan.

## Invariants

- Every artefact carries a stable dotted id whose typed prefix names its type.
- An artefact anchors to nodes only through its own frontmatter; the registry never invents an anchor.
- Validation is per file: one malformed artefact never hides findings from its siblings.

## Out of scope

- Deciding which pointers exist. The blueprint declares them; the parser records them.
- Reconciling code targets. Artefacts describe rationale and obligations, not source layout.
