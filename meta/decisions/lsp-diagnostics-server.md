---
id: dec.lsp-diagnostics-server
nodes:
  - cairn.lsp
status: accepted
date: 2026-06-23
---

# LSP diagnostics server

## Context

OMP consumes language-server diagnostics for on-write feedback. cairn already produces findings via `cairn watch`; exposing them through a persistent LSP server lets orchestrators subscribe without parsing the CLI's JSON stream.

## Decision

Add a `cairn.lsp` module that owns `src/lsp/` and `src/bin/cairn-lsp.rs`. The server uses `lsp-server` + `lsp-types` synchronously over stdio and publishes Cairn findings as `textDocument/publishDiagnostics` notifications. A background watch loop rescans the project and pushes diagnostic deltas.

## Rationale

- Synchronous implementation matches cairn's existing architecture; no async runtime is required.
- `lsp-server` provides a blocking stdio transport and LSP handshake, while `lsp-types` gives portable diagnostic types.
- Reusing `cairn watch`'s scan loop keeps the server deterministic and aligned with CLI behavior.

## Consequences

- The public crate API gains `cairn::lsp` with `LspOpts` and `run`.
- New LSP-only diagnostics must be mapped from `Finding`; richer source-location mapping is future work.
- The module needs its own decision and interface-hash tracking like `cairn.mcp`.
