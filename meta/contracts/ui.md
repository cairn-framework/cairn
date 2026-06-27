---
node: cairn.ui
---

# Contract: cairn.ui

## Purpose

Read-only embedded HTTP server and browser UI for exploring the architecture
graph. It serves a vendored Preact single-page app plus a JSON `/api/*` bridge
that runs scans on demand and exposes graph, node, lint, status, contract,
artefact, beads, and rationale data. The server is dependency-free (raw
`TcpListener`) and never mutates project state.

## Public interface

- `UiOptions`: `{ port, no_open, blueprint_path }` with `Default` and
  `from_args` argument parsing.
- `UiError`: enum over `Bind { port, source }`, `Io`, `Project`, and
  `ShutdownHandler`, implementing `Display`, `Error`, and `From<io::Error>`.
- `ServerHandle`: background-server handle exposing `url`, `address`, and `stop`.
- `serve_current_thread(UiOptions) -> Result<String, UiError>`: blocking server
  that runs until Ctrl+C.
- `start_background(UiOptions) -> Result<ServerHandle, UiError>`: spawns the
  server on a worker thread for tests and embedders.
- HTTP routes: `/`, `/assets/*`, `/vendor/*`, and the `/api/*` bridge
  (`/api/meta`, `/api/status`, `/api/graph`, `/api/lint`, `/api/node/<id>` with
  `contract`/`decisions`/`todos`/`research`/`sources`/`beads`/`rationale`
  suffixes, `/api/depends/<id>`, `/api/dependents/<id>`, `/api/blueprint`).

## Invariants

- Every `/api/*` JSON envelope is stamped with `schema_version` (currently `1`)
  as its first key via the single `versioned` helper.
- The server binds only to `127.0.0.1`; port `0` requests an OS-assigned port.
- Scans are cached and reloaded only when the blueprint or watched target and
  contract files change mtime.
- The surface is strictly read-only: handlers read scan and artefact data and
  never write project state.
- Connection-reset and broken-pipe stream errors are swallowed, not fatal.

## Dependencies

Outgoing blueprint edges: `cairn.ui -> cairn.kernel.scanner` (runs scans for API
responses), `cairn.ui -> cairn.kernel.map` (serves graph data), and
`cairn.ui -> cairn.state` (reads node-linked beads for the inspector). The UI
also calls `kernel.query` helpers to shape graph and node responses.

## Tests

Unit tests live in `#[cfg(test)]` modules in `src/ui/mod.rs` (server lifecycle
and routing over a background handle), `src/ui/api.rs` (endpoint JSON shaping and
artefact collection), and `src/ui/serialise.rs` (escaping and percent-decoding).
