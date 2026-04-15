# Tasks: Phase 7 MCP

## 1. Shared Query API

- [ ] 1.1 Extract CLI query request and response structs into reusable library modules.
- [ ] 1.2 Ensure CLI JSON output and MCP responses use the same data model.
- [ ] 1.3 Add tests proving CLI and library query outputs are equivalent.

## 2. MCP Server

- [ ] 2.1 Add `cairn-mcp` binary target.
- [ ] 2.2 Implement stdio transport.
- [ ] 2.3 Implement the shared query tool registry with MCP name, schemas, and safety class metadata.
- [ ] 2.4 Register read-only tools for core, docstring, artefact, status, and change queries.
- [ ] 2.5 Add gated mutating tool exposure for mutation-capable tools including archive and rename.
- [ ] 2.6 Ensure the `cairn-mcp` binary root uses the Phase 0 strict crate attributes.

## 3. Context and Rules

- [ ] 3.1 Parse `cairn.config.yaml` context and rules blocks.
- [ ] 3.2 Compose project context into every response.
- [ ] 3.3 Compose artefact-specific rules into relevant query responses.
- [ ] 3.4 Add tests for missing config and populated config.

## 4. Error Handling and Documentation

- [ ] 4.1 Map Cairn errors to MCP errors with stable codes.
- [ ] 4.2 Document server startup, tool list, mutating-tool safety, and response schema.
- [ ] 4.3 Add integration tests for stdio requests and error responses.

## 5. Required Verification

- [ ] 5.1 `cargo build` passes with zero warnings.
- [ ] 5.2 `cargo clippy -- -D warnings -D clippy::pedantic -D clippy::nursery` passes.
- [ ] 5.3 `cargo fmt --check` passes.
- [ ] 5.4 `cargo test` passes.
- [ ] 5.5 `cargo test --locked` passes.
- [ ] 5.6 `python3 /Users/george/repos/cairn/.agents/skills/cflx-proposal/scripts/cflx.py validate phase-7-mcp --strict` passes.
