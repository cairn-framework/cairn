# Design: Phase 10 Distribution

## References

- `docs/spec.md` section 12 for query API stability.
- `docs/spec.md` section 14 for distribution phase.
- `docs/spec.md` section 17 for deliberately uncommitted downstream choices.

## LSP Server

The project SHALL add a `cairn-lsp` binary. The server SHALL reuse the same parser, ontology, query, and finding APIs as CLI and MCP.

Supported LSP features:

- Diagnostics from parser, lint, hook, and scan findings.
- Completion for node IDs, artefact IDs, tag names, and delta operation markers.
- Hover for node metadata, current state, paths, attached artefact counts, and recent findings.
- Go-to-definition for node IDs in edges and artefact frontmatter.
- Document symbols for systems, containers, modules, actors, and edges.

## Plugin Packaging

Distribution SHALL include documented integration for Claude Code using:

- CLI commands.
- MCP server startup.
- Project context and rules composition.
- Example prompts that query Cairn rather than scanning the repo.

Packaging SHALL not require a hosted service.

## Reconciler Extensions

The reconciler trait SHALL be documented as an extension API. A fixture non-code reconciler SHALL demonstrate how to register a reconciler that produces nodes or reality observations for a domain outside source code.

The fixture reconciler SHALL be test-only or example-only and SHALL NOT claim to implement a production org, BOM, or research reconciler.

## Release Checks

Release validation SHALL cover:

- CLI binary.
- MCP binary.
- LSP binary.
- Shell completions where supported.
- Manpage or markdown command reference.
- Example project exercising DSL, artefacts, changes, MCP, and LSP diagnostics.

## Testing

Tests SHALL cover LSP request/response flows, diagnostic parity with lint, completion contents, hover contents, go-to-definition, plugin docs examples, extension registration, and release packaging checks.
