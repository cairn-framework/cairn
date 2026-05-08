---
node: cairn.kernel.scanner
informed_by:
  - type: decision
    id: dec.contradiction-classes
  - type: decision
    id: dec.blueprint-as-current-state
  - type: decision
    id: dec.module-path-mapping
---

# Contract: cairn.kernel.scanner

The Scanner is the engine that runs all registered reconcilers against a parsed blueprint and assembles their output into a single map. It is the meeting point between declared structure (from the parser) and actual structure (from the reconcilers).

## Interface

- **Input.** An `Ast` (from the parser), a registered set of reconcilers, and the loaded artefact set (contracts, decisions, todos, reviews, research, sources).
- **Output.** A reconciled map plus a sorted finding list. The map is rendered to `map.md`; findings are rendered to stdout (and to JSON when `--json` is set).
- **Exit.** Zero if no findings of `Error` severity. Non-zero otherwise. Severity follows the contradiction-class taxonomy (structural error, interface contradiction, rationale tension).

## Invariants

- Every blueprint Module reports exactly one of `synced`, `ghost`, or `orphaned` per reconciler output.
- A finding's severity reflects its contradiction class. Structural errors and interface contradictions block; rationale tensions surface but never block.
- The scanner ignores change directories under `meta/changes/`. Queries default to current truth; change-aware queries are opt-in.
- Output ordering is stable: findings are sorted by severity, then code, then node, then path. Two scanner runs over identical input produce byte-identical output.

## Out of scope

- Authoring contracts, decisions, or any other artefact. The scanner is read-only against authority artefacts.
- Modifying blueprint or contract files. Drift detection produces findings; resolution is the project's responsibility.
- Cross-blueprint queries. The scanner operates over one project root at a time.
