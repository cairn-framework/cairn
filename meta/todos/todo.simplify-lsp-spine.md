---
node: cairn.lsp
status: done
created: 2026-07-06
---

# LSP Reuses the Shared Scan Loop and query_api

Part of todo.simplify-architecture (wave 3).
Depends on: todo.simplify-ui-query-api (establishes the consume-the-spine
pattern and any shared cache layer the LSP can reuse).
Follow the shared rules in todo.simplify-architecture.

`src/lsp/diagnostics.rs:101-123` calls `scanner::scan` directly and
reimplements finding-to-diagnostic transformation;
`src/lsp/server.rs:50` runs its own background rescan loop.

The shared pieces today: `src/watch.rs` exports `WatchOpts`,
`WatchEvent`, and `diff_findings` (a finding-set differ), while the
actual periodic scan loop lives in `src/cli/commands/watch.rs`
(`run_watch_command`); the query_api `watch` tool is a one-shot diff.
There is no ready-made loop to call, so:

- Extract the periodic scan loop from `src/cli/commands/watch.rs` into
  `src/watch.rs` (or a shared helper) and drive both the CLI `watch`
  command and the LSP from it, translating `WatchEvent`s into
  diagnostic publishes.
- Source findings from the `lint` operation via `query_api::execute`
  (or the same internal query the spine uses) instead of a private scan.
- Keep the LSP-specific part small: protocol plumbing plus the
  Finding-to-LSP-Diagnostic mapping.

Acceptance: `src/lsp` no longer calls `scanner::scan` directly and has no
private rescan loop; the extracted loop is the single implementation used
by both CLI watch and LSP; existing LSP diagnostics and watch tests
green; manual smoke: `cairn-lsp` publishes diagnostics for a blueprint
edit in an editor session or scripted stdio exchange.


## Resolution

Done 2026-07-10. `src/watch.rs::run_watch_loop` is the single shared periodic
scan loop; both `src/cli/commands/watch.rs::run_watch_command` (CLI watch) and
`src/lsp/diagnostics.rs::start_watch_thread` (LSP background rescan) drive it.
The LSP sources findings from the spine's `lint` operation via the new
`crate::query_api::lint_findings` instead of calling `scanner::scan`.
`src/lsp` now contains no `scanner::scan` call and no private rescan loop; the
watch thread translates `WatchEvent`s into `publishDiagnostics` through
`DiagnosticPublisher::apply_events`. Existing LSP and watch tests pass; a
scripted stdio smoke of `cairn-lsp` confirmed diagnostics publish for the
project's findings (see `meta/todos/lsp-smoke-evidence.log`).
