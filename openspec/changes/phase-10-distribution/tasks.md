# Tasks: Phase 10 Distribution

## 1. LSP Server

- [ ] 1.1 Add `cairn-lsp` binary target.
- [ ] 1.2 Implement diagnostics from parser, lint, hook, and scan findings.
- [ ] 1.3 Implement completion for node IDs, artefact IDs, tag names, and delta operation markers.
- [ ] 1.4 Implement hover for node metadata and artefact summaries.
- [ ] 1.5 Implement go-to-definition for node IDs and artefact references.
- [ ] 1.6 Implement document symbols for DSL declarations and edges.

## 2. Plugin and Agent Packaging

- [ ] 2.1 Document Claude Code integration using CLI and MCP.
- [ ] 2.2 Add example agent prompts grounded in Cairn queries.
- [ ] 2.3 Document installation and startup for `cairn`, `cairn-mcp`, and `cairn-lsp`.

## 3. Extension API

- [ ] 3.1 Document the reconciler extension trait and registration flow.
- [ ] 3.2 Add a fixture non-code reconciler.
- [ ] 3.3 Add tests proving fixture reconciler reports flow through ontology, queries, and diagnostics.

## 4. Release Packaging

- [ ] 4.1 Add release checks for CLI, MCP, and LSP binaries.
- [ ] 4.2 Generate command reference documentation.
- [ ] 4.3 Add an example project exercising all major capabilities.
- [ ] 4.4 Add tests for packaging metadata and documented commands.

## 5. Required Verification

- [ ] 5.1 `cargo build` passes with zero warnings.
- [ ] 5.2 `cargo clippy -- -D warnings -D clippy::pedantic -D clippy::nursery` passes.
- [ ] 5.3 `cargo fmt --check` passes.
- [ ] 5.4 `cargo test` passes.
- [ ] 5.5 `cargo test --locked` passes.
- [ ] 5.6 `python3 /Users/george/repos/cairn/.agents/skills/cflx-proposal/scripts/cflx.py validate phase-10-distribution --strict` passes.
