# Design: Generic parameterized reconciler

## Approach

Define a `LanguageSpec` that captures everything that currently varies
per reconciler:

    struct LanguageSpec {
        language: Language,
        grammar: tree_sitter::Language,
        extensions: &'static [&'static str],       // ["ts", "tsx"]
        exportable_kinds: &'static [&'static str], // ["export_statement", ...]
        kind_to_symbol: fn(&str) -> SymbolKind,
        interface_symbol: fn(tree_sitter::Node, &[u8]) -> String,
    }

A single `CodeReconciler<'a>` implements `Reconciler` by running the
shared pipeline against a `&LanguageSpec`:

- `discover_files` walks the root, keeping files whose extension is in
  `spec.extensions` (replaces `discover_rust_files` /
  `discover_ts_files` / ...).
- parallel chunk parsing sets `parser.set_language(&spec.grammar)`.
- `collect_public_symbols` is generic over `spec.exportable_kinds` and
  `spec.kind_to_symbol`.

A `static LANGUAGES: &[LanguageSpec]` registry replaces the
`match target.language` block in `reconcile_targets`
(`src/scanner/mod.rs:129-143`): lookup by `Language`, fall back to a
warning if absent. The existing owner-assignment logic
(`eligible_owners`, `most_specific_owner`) is language-agnostic and moves
into the shared pipeline unchanged.

The four `*.rs` reconciler modules become small `LanguageSpec`
definitions (grammar plus extension/kind tables), and the duplicated
`discover_*` / `walk` / `public_symbols` / `collect_public_symbols` /
`symbol_kind` / `interface_symbol` functions are deleted.

### Sequencing

Depends on `fix-dir-language-inference` landing first: the `Unknown`
variant and inference helper are natural inputs to the registry's "no
spec for this language" path. `reconcile-config-schema` is independent
but its `tree_sitter_languages` future-block becomes implementable once
this registry exists.

## Changes

ADDED:
- `LanguageSpec` plus shared `CodeReconciler` in `src/reconcile/` (likely a
  new `generic.rs`, or folded into `code.rs`).
- `static LANGUAGES: &[LanguageSpec]` registry.

MODIFIED:
- `src/scanner/mod.rs:120-144`: dispatch by registry lookup.
- `src/reconcile/mod.rs`: re-export the generic reconciler.

REMOVED:
- Per-language `discover_*_files`, `walk`, `public_symbols`,
  `collect_public_symbols`, `symbol_kind`, `interface_symbol` duplicates
  in `code.rs` / `typescript.rs` / `python.rs` / `go.rs`. The four modules
  shrink to spec definitions.

## Guards

- Reconciler equivalence tests: snapshot each language's
  `ReconcileReport` (claimed_files, symbols, findings) before refactor;
  assert byte-identical after.
- Self-host: `cairn scan` interface hashes unchanged.
- `tests/wire_format_snapshots.rs` unchanged.
- File-size gate (`scripts/check-file-sizes.sh`) satisfied for the new
  shared module (use `// cairn:allow-large-module` only if justified).
- `cargo clippy -D warnings --all-targets`, `cargo test`, `cairn hook all`.
