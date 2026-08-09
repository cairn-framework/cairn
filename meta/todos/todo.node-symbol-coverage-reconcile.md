---
node: cairn.reconcile
status: open
created: 2026-08-09
---

# Node Symbol Coverage Reconcile


## Goal

After `todo.node-symbol-coverage-ruling` is accepted, split extraction into
exported and query-visible streams without changing interface hashes. Use
`res.node-symbol-coverage.investigation` as the seam map, including the Rust
`pub ` pre-parse shortcut.

## Scope

Update `src/reconcile/generic.rs`, `src/reconcile/mod.rs`,
`src/reconcile/code.rs`, `src/reconcile/typescript.rs`, and the reconciler
cache and scanner report assembly needed to carry both streams. Keep
`LanguageSpec::is_exportable`, `ReconcileReport.node_symbols`,
`ReconcileReport.node_symbol_records`, `TargetReport.symbols`, and
`TargetReport.symbol_records` as exported interface data. Add a named
query-visible record stream, ensure the query path parses Rust files with no
`pub ` marker, bump the cache schema, and cover sequential and parallel paths.

## Acceptance

- A failing test first demonstrates that private Rust and TypeScript items are
  present in the query stream but absent from the exported stream.
- The test passes after implementation and proves private-only edits do not
  alter interface fingerprints or target hashes.
- Fresh and cached scanner reports carry identical exported and query-visible
  records.
- `scripts/pre-archive-rust-gates.sh` and the strict scan and hook gates pass.