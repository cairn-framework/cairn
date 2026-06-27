---
node: cairn.lsp
---

# Contract: cairn.lsp

## Purpose

A synchronous, stdio-based language server that publishes Cairn findings as
`textDocument/publishDiagnostics` notifications, so OMP and other orchestrators
get on-write feedback. A background thread re-scans the project on a fixed
interval and republishes diagnostics whenever the finding set changes, while
the main loop handles the LSP lifecycle (initialize, shutdown, exit).

## Public interface

- `LspOpts`: optional root override and `interval_secs` between background
  scans. `from_args(args)` parses flags, returning a message on an unknown flag
  or missing value.
- `run(connection, opts)`: drives the initialize handshake, resolves the
  workspace root, spawns the watch thread, and services requests until exit.
- `help_text()`: the `--help` text for the binary.
- `DiagnosticPublisher`: holds the message sender and previously published
  URIs; `scan_and_publish(root, blueprint)` scans and emits diagnostic deltas,
  returning whether the client is still connected.
- `start_watch_thread(sender, root, interval, stop)`: background scan loop.
- `MIN_INTERVAL_SECS`: scan interval floor (1 second).

## Invariants

- Root resolution order: explicit `--root`, then the first workspace folder,
  then the deprecated `root_uri`, then the current directory.
- The effective interval is `max(interval_secs, MIN_INTERVAL_SECS)`, so scans
  never run faster than once per second.
- Cleared findings are published as empty diagnostic lists for any URI that
  appeared in the previous scan but not the current one (delta semantics).
- A failed scan logs to stderr and retries on the next tick rather than
  aborting; a send failure (disconnected client) stops the watch loop.
- The `exit` notification breaks the loop and signals the watch thread to stop
  via the shared `AtomicBool`.

## Dependencies

Leaf node with no outgoing blueprint edges. It reads findings produced by a
project scan (`map::graph::Finding`, `FindingSeverity`) and speaks LSP via the
`lsp_server` and `lsp_types` crates over stdio.

## Tests

Unit tests live in `#[cfg(test)] mod tests` in `src/lsp/server.rs` (root
resolution, capabilities, request handling) and `src/lsp/diagnostics.rs`
(finding-to-diagnostic conversion, URI grouping, severity mapping, and delta
publishing), plus the `from_args` tests in `src/lsp/mod.rs`.
