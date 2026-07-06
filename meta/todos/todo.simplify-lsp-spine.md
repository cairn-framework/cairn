---
node: cairn.lsp
status: open
created: 2026-07-06
---

# LSP Reuses watch and query_api Instead of Its Own Loop

Part of todo.simplify-architecture (wave 3).
Depends on: todo.simplify-ui-query-api (establishes the consume-the-spine
pattern and any shared cache layer the LSP can reuse).

`src/lsp/diagnostics.rs:101-123` calls `scanner::scan` directly and
reimplements finding-to-diagnostic transformation;
`src/lsp/server.rs:50` runs its own background rescan loop although
`src/watch.rs` is the shared periodic-scan-with-change-events utility
(already consumed by `cairn watch` and the query_api `watch` tool).

- Source findings from the `lint` operation via `query_api::execute`
  (or the same internal query the spine uses) instead of a private scan.
- Replace the bespoke background loop with `src/watch.rs`, translating
  finding-change events into diagnostic publishes.
- Keep the LSP-specific part small: protocol plumbing plus the
  Finding-to-LSP-Diagnostic mapping.

Acceptance: `src/lsp` no longer calls `scanner::scan` directly and has no
private rescan loop; existing LSP diagnostics tests green; manual smoke:
`cairn-lsp` publishes diagnostics for a blueprint edit in an editor
session or scripted stdio exchange.
