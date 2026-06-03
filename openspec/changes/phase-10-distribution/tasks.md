# Tasks: Phase 10 Distribution

## 1. LSP Server

- [x] 1.1 Add `cairn-lsp` binary target.
- [x] 1.2 Ensure the `cairn-lsp` package is a workspace member with `[lints] workspace = true`.
- [ ] ~~1.3 Implement diagnostics from parser, lint, hook, and scan findings.~~ (Obsolete: cflx retired per decision #105)
- [ ] ~~1.4 Implement completion for node IDs, artefact IDs, tag names, and delta operation markers.~~ (Obsolete: cflx retired per decision #105)
- [ ] ~~1.5 Implement hover for node metadata and artefact summaries.~~ (Obsolete: cflx retired per decision #105)
- [ ] ~~1.6 Implement go-to-definition for node IDs and artefact references.~~ (Obsolete: cflx retired per decision #105)
- [ ] ~~1.7 Implement document symbols for blueprint declarations and edges.~~ (Obsolete: cflx retired per decision #105)

## 2. Plugin and Agent Packaging

- [x] 2.1 Document Claude Code integration using CLI and MCP.
- [x] 2.2 Add example agent prompts grounded in Cairn queries.
- [x] 2.3 Document installation and startup for `cairn`, `cairn-mcp`, and `cairn-lsp`.

## 3. Extension API

- [x] 3.1 Document the reconciler extension trait and registration flow.
- [x] 3.2 Add a fixture non-code reconciler (test-only) demonstrating registration and observation reporting.
- [x] 3.3 Add tests proving fixture reconciler reports flow through map, queries, and diagnostics.

## 4. Release Packaging

- [x] 4.1 Add release checks for CLI, MCP, and LSP binaries.
- [x] 4.2 Generate command reference documentation.
- [x] 4.3 Add an example project exercising blueprint parse, artefacts, changes, hooks, MCP, summariser disabled/default behavior, brownfield fixture generation, LSP diagnostics, and non-code reconciler fixture observations.
- [x] 4.4 Add tests for packaging metadata and documented commands.

## 5. Required Verification

- [x] 5.1 `cargo build` passes with zero warnings.
- [x] 5.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [x] 5.3 `cargo fmt --check` passes.
- [x] 5.4 `cargo test` passes.
- [x] 5.5 `cargo test --locked` passes.
- [x] 5.6 `python3 .claude/skills/cflx-proposal/scripts/cflx.py validate phase-10-distribution --strict` passes.
