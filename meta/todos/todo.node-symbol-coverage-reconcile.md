---
node: cairn.reconcile
status: blocked
created: 2026-08-09
blocked_by: [todo.node-symbol-coverage-ruling]
parent: todo.node-symbol-coverage
---

# Node Symbol Coverage Reconcile


## Goal

After `todo.node-symbol-coverage-ruling` is accepted, factor a shared
exported/query-visible tree walk without changing interface hashes or adding
persisted query state. Use `res.node-symbol-coverage.investigation` as the seam
map, including the Rust `pub ` pre-parse shortcut.

## Scope

Update `src/reconcile/generic.rs`, `src/reconcile/mod.rs`,
`src/reconcile/code.rs`, and `src/reconcile/typescript.rs` to expose a
crate-internal transient query extractor that reuses item kinds, name and kind
resolution, and signature construction. Keep
`LanguageSpec::is_exportable`, `ReconcileReport.node_symbols`,
`ReconcileReport.node_symbol_records`, `TargetReport.symbols`,
`TargetReport.hash`, and `TargetReport.symbol_records` as exported interface
data. The query extractor must parse Rust files with no `pub ` marker, apply
the query policy for Rust and TypeScript, and never add query records to the
reconciler report, cache serialization, map snapshot, or graph node. Cover
both sequential and parallel exported paths without changing their hashes.

Keep Python and Go query behavior unchanged with regression coverage. For
TypeScript, resolve `variable_declaration` through its declarator or leave it
excluded by the ruling, keep `lexical_declaration` excluded unless explicitly
added, and ensure an `export_statement` wrapper plus its child yields one
record. Sort transient records with the same deterministic ordering as
`src/reconcile/generic.rs:284-289`.

## Acceptance

- A failing test first demonstrates that private Rust and TypeScript items are
  present in the query stream but absent from the exported stream.
- The test passes after implementation and proves private-only edits do not
  alter interface fingerprints or target hashes.
- Fresh and cached scanner runs produce the same transient query results, while
  the report and cache schema remain exported-only.
- `scripts/pre-archive-rust-gates.sh` and the strict scan and hook gates pass.