# Proposal: Phase 10 Distribution

## Problem/Context

Phases 1-9 complete the core Cairn framework. The final campaign phase makes Cairn usable from editors, agent environments, and additional domains without changing the ontology semantics.

Phase 10 implements the distribution scope from `docs/spec.md` section 14: LSP server, Claude Code plugin packaging, and extension points for non-code reconcilers.

## Proposed Solution

Add:

- `cairn-lsp` server for editor autocomplete, hover, diagnostics, and jump-to-definition over DSL IDs, artefacts, and edges.
- Packaging metadata and install documentation for Claude Code integration using the existing CLI and MCP server.
- Reconciler extension registration for non-code domains such as org structure, product BOMs, and research programmes.
- Release packaging checks for binaries, completions, docs, and examples.

## Acceptance Criteria

- LSP diagnostics match `cairn lint` findings.
- LSP autocomplete suggests node IDs, artefact IDs, and command-relevant symbols.
- Hover on DSL IDs returns node metadata and attached artefact summary.
- Jump-to-definition works for edges and artefact references.
- Plugin packaging documents CLI, MCP, and project context integration.
- Reconciler extension APIs are documented and tested with a fixture non-code reconciler.
- All strict Rust gates pass.

## Out of Scope

- A visual graph dashboard.
- Hosted services.
- A mandated summariser provider.
- Domain-specific production reconcilers beyond one fixture extension.
