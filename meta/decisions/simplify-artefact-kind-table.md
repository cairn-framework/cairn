---
id: dec.simplify-artefact-kind-table
nodes:
  - cairn.kernel.artefacts
status: accepted
date: 2026-07-09
related: [dec.kernel-core, dec.changes-in-artefact-set]
---

# Collapse frontmatter artefact loaders into one ArtefactKind table

## Context

Todo, decision, review, research, and source each carried a bespoke loader
loop over blueprint pointers, frontmatter parse, required/optional fields,
and a typed push into `ArtefactSet`. The frontmatter parser was already
shared; the loop shell was not. The 2026-07-06 four-audit investigation
ratified in `todo.simplify-architecture` (wave 1,
`todo.simplify-artefact-kind-table`) estimated ~1,000 LOC of this
per-type duplication.

Contract loading (`src/artefacts/contract.rs`) and change loading
(`registry/changes.rs`, directory-based) are genuine special cases and
stay out of the table.

## Decision

Introduce `src/artefacts/registry/kinds.rs` with one `ArtefactKind`
descriptor table (pointer field name + `load_one` constructor). A single
`load_kinds` walk drives all five frontmatter-backed kinds.
`load_artefacts` becomes: load kinds, load changes, validate integrity.

`pub(crate) load_decisions` remains as a thin wrapper over the decisions
kind for the architecture-hook caller that only needs decisions.

CLI scaffold write path for todo and decision shares one
`write_new_artefact` helper. CLI spellings (`cairn todo new`,
`cairn decision`, `cairn gap`) are unchanged; gap keeps its collision
suffix logic. Dead unused generic loader types (`ArtefactLoader`,
`ArtefactLoadRequest`, `ArtefactRecord`, `ArtefactError`) are removed;
`ArtefactType` stays for export.

## Rationale

One table entry is the unit of extension for a new frontmatter-backed
kind. Typed public records (`Todo`, `Decision`, …) stay so callers and
serialisers do not change shape. Exact validation finding codes and
messages are preserved; existing registry/parse/validate tests are the
guard.

## Consequences

- Adding a sixth frontmatter-backed kind is a table row plus a
  constructor, not a new loop shell.
- Contract and change remain outside the table by design.
- CLI command spellings are deferred to the CLI-family simplify todos.
