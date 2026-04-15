# Phase 2 Deferrals

The kernel MVP deliberately stops at a hand-authored DSL, in-memory ontology, and CLI query layer.

Deferred work:

- Scanner and filesystem walking beyond parsing path strings.
- Contract artefact parsing, interface hashes, and code reconciliation.
- Opening or validating artefact files referenced by the DSL.
- Change directories, archive command, and rename propagation.
- Hooks and commit/task-boundary enforcement.
- Edge validation against source code.
- Docstring generation and drift detection.
- MCP, LSP, summariser, and brownfield extraction surfaces.

These are separate changes after the query layer proves useful in real coding sessions.
