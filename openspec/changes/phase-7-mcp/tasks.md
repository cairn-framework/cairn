# Tasks: Phase 7 MCP

## 1. Shared Query API

- [x] 1.1 Extract CLI query request and response structs into reusable library modules.
- [x] 1.2 Ensure CLI JSON output and MCP responses use the same data model.
- [x] 1.3 Add tests proving CLI and library query outputs are equivalent.

## 2. MCP Server

- [x] 2.1 Add `cairn-mcp` binary target.
- [x] 2.2 Implement stdio transport.
- [x] 2.3 Implement the shared query tool registry with MCP name, schemas, and safety class metadata.
- [x] 2.4 Register read-only tools for core, docstring, artefact, status, and change queries.
- [x] 2.5 Add gated mutating tool exposure for mutation-capable tools including archive and rename.
- [x] 2.6 Ensure the `cairn-mcp` package is a workspace member with `[lints] workspace = true`.

## 3. Context and Rules

- [x] 3.1 Parse `cairn.config.yaml` context and rules blocks.
- [x] 3.2 Compose project context into every response.
- [x] 3.3 Compose artefact-specific rules into relevant query responses.
- [x] 3.4 Add tests for missing config and populated config.

## 4. Error Handling and Documentation

- [x] 4.1 Map Cairn errors to MCP errors with stable codes.
- [x] 4.2 Document server startup, tool list, mutating-tool safety, and response schema.
- [x] 4.3 Add integration tests for stdio requests and error responses.

## 5. Required Verification

- [x] 5.1 `cargo build` passes with zero warnings.
- [x] 5.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [x] 5.3 `cargo fmt --check` passes.
- [x] 5.4 `cargo test` passes.
- [x] 5.5 `cargo test --locked` passes.
- [x] 5.6 `python3 .agents/skills/cflx-proposal/scripts/cflx.py validate phase-7-mcp --strict` passes.
