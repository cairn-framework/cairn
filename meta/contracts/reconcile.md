---
node: cairn.reconcile
---

# Contract: cairn.reconcile

## Purpose

Pluggable reconciler layer that compares blueprint intent against project
reality. Each reconciler discovers source files, attributes public symbols to
the owning node, computes a deterministic interface fingerprint, and emits
findings. Tree-sitter backed code reconcilers ship for Rust, TypeScript,
Python, and Go, alongside a fixture reconciler that demonstrates the extension
API for non-code domains.

## Public interface

- `Reconciler`: domain-agnostic trait with `id()` and
  `reconcile(ReconcileRequest) -> Result<ReconcileReport, ReconcileError>`.
- `ReconcilerId(String)`: stable reconciler identifier.
- `ReconcileRequest<'a>`: borrows the project `root` and `ignores` patterns.
- `ReconcileReport`: serde-serializable result carrying `claimed_files`,
  `symbols` (an `Arc<Vec<String>>` via the `serde_arc_vec` helper), per-node
  `node_symbols`, an `InterfaceFingerprint`, and `findings`.
- `ReconcileError`: `{ code, message }` with `Display` and `Error`.
- `code::RustCodeReconciler`, `typescript::TypeScriptReconciler`,
  `python::PythonReconciler`, `go::GoReconciler`: per-language reconcilers, each
  constructed from a blueprint `Ast`.
- `fixture::FixtureReconciler`: findings-only demonstration reconciler.
- `target`: `Language` enum, `Target`, `TargetId`, `SUPPORTED_LANGUAGES`
  (`rust`, `typescript`, `python`, `go`), and `DEFAULT_CONTRACT_ROLE`.
- `fingerprint::InterfaceFingerprint`: `from_symbols` and `from_sorted`.

## Invariants

- `InterfaceFingerprint` sorts symbols before hashing, so the hash is
  order-independent and always a 16-character lowercase hex string.
- Symbol attribution picks the most specific owner whose path prefixes the file.
- File discovery walks the root honouring `ignores`; only public or exported
  declaration kinds become interface symbols.
- The fixture reconciler claims no files and produces no symbols; it only emits
  findings.

## Dependencies

Outgoing blueprint edge: `cairn.reconcile -> cairn.kernel.map` (reports findings
to the graph). Reconcilers read the parsed blueprint `Ast` and emit
`map::graph::Finding` values; the scanner invokes them.

## Tests

Unit tests live in `#[cfg(test)]` modules within `src/reconcile/fingerprint.rs`
(deterministic, sorted, hex-format hashing) and `src/reconcile/code.rs`
(symbol collection and owner attribution). Per-language reconcilers carry their
own module-level test coverage.
