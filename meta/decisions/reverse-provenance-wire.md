---
id: dec.reverse-provenance-wire
nodes: [cairn.kernel.artefacts, cairn.kernel.query]
status: accepted
ratification: binding
date: 2026-08-02
informed_by: [res.inversion-convergence-minutes]
related: [dec.todo-relationship-model]
---
# Computed reverse provenance is a wire contract

## Decision

Decision artefacts carry computed reverse provenance edges, `refined_by` and `superseded_by`, on the JSON contract at `SCHEMA_VERSION` 11. The edges are derived once at load from forward `refines:` and `supersedes:` references and are never authored frontmatter fields. Maintainer-signed; machine ratification is not granted. Accepted 2026-08-03 by maintainer ratification in session.

## Rubric

- Tier: Binding. The public JSON contract moves, so this decision requires a maintainer signature.
- Unblocks: This unblocks `todo.reverse-provenance-surfacing` to expose decision lineage consistently in `cairn rationale`, `cairn get`, `cairn pending`, and the webui decision panes.
- Alignment: Against `dec.cairn-mission` first: the computed edges make the graph maintainable by keeping forward and reverse provenance consistent.
  - Goal 1: the load-time computation makes decision authority investigable without asking authors to maintain duplicate fields.
  - Goal 2: the shared wire shape makes lineage extendable across CLI, MCP, and webui consumers.
  - Goal 3: the advisory and rendered edges keep the product fit for purpose when one decision qualifies another.
  - Goal 4: deterministic derivation and schema versioning keep the continuous loop safe to operate.
  - Goal 5: one computed representation gives every reader the same current decision chain.
- Options: (a) Local-only rendering without a wire change fails the todo's `--json` acceptance and leaves consumers without lineage; (b) Authored reverse fields would drift from the forward references and create a second source of truth; (c) Computed wire edges under this binding record preserve one source of truth and are recommended.

## Rationale

A reverse edge is a query of the loaded forward provenance graph, not a new claim. Computing it after all decisions load keeps authored decision files stable while making a qualified authority visible wherever the JSON contract is consumed.
