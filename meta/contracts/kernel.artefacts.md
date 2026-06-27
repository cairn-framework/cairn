---
node: cairn.kernel.artefacts
---

# Contract: cairn.kernel.artefacts

## Purpose

The typed artefact registry and Phase 2 loaders. Given a parsed blueprint AST, it
resolves the artefact pointers each node declares (contracts, decisions,
research, sources, reviews, todos), reads the referenced Markdown files, parses
their frontmatter and bodies into typed records, and validates cross-artefact
integrity (for example research-to-source links and SHA-256 source digests).

## Public interface

- `contract::load_contracts(root, ast)`: loads Phase 1 contract pointers into a
  `ContractSet` (`contracts: BTreeMap<String, Contract>` plus `findings`).
- `Contract`: a parsed contract with `path`, `declared_by`, `node`, and `body`.
- `registry::load_artefacts(root, ast, contracts)`: loads all non-contract Phase
  2 artefacts and returns an `ArtefactSet`.
- `ArtefactSet`: aggregate of `contracts`, `todos`, `decisions`, `reviews`,
  `research`, `sources`, and `findings`.
- Typed records: `Todo`, `Decision`, `Review`, `Research`, `Source`, plus the
  `Claims` / `ClaimsMode` re-exported from the crate root, and status enums
  (`TodoStatus`, `DecisionStatus`, `ReviewType`, `ResearchMethod`,
  `SourceVerification`).
- `ArtefactType`, `ArtefactLoader`, `ArtefactRecord`, `ArtefactLoadRequest`,
  `ArtefactError`: the generic loader surface.
- `frontmatter::parse(source)`: minimal `---` delimited frontmatter parser
  returning `Frontmatter` (`values`, `lists`, `body`).

## Invariants

- Frontmatter parsing requires a leading `---`; absent it the whole input is body
  with empty `values` and `lists`.
- Scalar values are stripped of trailing `#` comments and surrounding quotes.
- Loading is non-fatal: malformed or missing artefacts produce `Finding` entries
  rather than aborting the load.
- Contract and artefact pointers are deduplicated and sorted before loading.
- A contract may only reference a node ID that exists in the AST.

## Dependencies

Leaf module with no outgoing blueprint edges. At the code level it consumes types
from `cairn.kernel.blueprint` (`Ast`, `Node`, `Field`) and `Finding` /
`FindingSeverity` from `cairn.kernel.map::graph`. The loaded `ArtefactSet` is in
turn consumed by the scanner and downstream kernel modules.

## Tests

A `#[cfg(test)] mod tests` lives at the foot of `src/artefacts/registry/mod.rs`
covering decision, review, research, and source loading. Integrity validation has
a dedicated suite in `src/artefacts/registry/validate/tests.rs` exercising the
SHA-256 digest and cross-reference checks.
