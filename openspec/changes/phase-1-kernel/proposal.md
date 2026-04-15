# Proposal: Phase 1 Kernel

## Problem/Context

Phase 0 creates only the Rust foundation. Cairn now needs the first complete product capability: a kernel that turns an authored `cairn.dsl`, contract artefacts, and filesystem reality into a queryable ontology.

The kernel is the architectural spine described in `docs/spec.md` sections 6, 7, 10, and 12. It MUST establish the parser, graph, reconciler interface, scanner, generated outputs, and CLI query surface that every later phase extends.

## Proposed Solution

Implement the Rust kernel with:

- A hand-written DSL lexer and parser for `System`, `Container`, `Module`, optional `Actor`, stable IDs, tags, paths, artefact pointers, and edges.
- An in-memory ontology graph with node lookup, name lookup, parent/child links, inbound/outbound edge indexes, path ownership, and node states.
- A reconciler trait plus the first code reconciler implementation using Tree-sitter for Rust source discovery and interface fingerprints.
- Contract artefact loading and validation as the only artefact type implemented in Phase 1.
- Scanner orchestration that reconciles DSL, contracts, and code reality into the ontology.
- CLI commands: `get`, `neighbourhood`, `contract`, `files`, `dependents`, `depends`, `order`, `lint`, and `scan`.
- Generated `index.md` and `.cairn/log.md` outputs from `scan`.

## Acceptance Criteria

- Valid DSL fixtures parse into deterministic AST and ontology structures.
- Malformed DSL produces source-positioned errors.
- Duplicate IDs, path ties, invalid edge endpoints, missing required fields, broken contract pointers, and dependency cycles surface as structural errors.
- The code reconciler reports synced, ghost, and orphaned state for claimed Rust paths.
- `scan` writes `index.md`, `.cairn/log.md`, and `.cairn/state/` without requiring network access.
- Each required CLI command supports human-readable output and `--json` output.
- Contract-only artefact loading works; todos, decisions, reviews, research, and sources remain absent until Phase 2.
- All strict Rust gates pass.

## Out of Scope

- Todo, decision, review, research, and source artefact types.
- Change directories, archive, rename, and change-aware queries.
- Commit hooks beyond the Phase 0 Rust gates.
- Edge validation against imports, docstring drift detection, multi-target reconciliation, MCP, summariser, brownfield extraction, LSP, and plugin packaging.
