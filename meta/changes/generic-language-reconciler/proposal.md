# Proposal: Generic parameterized reconciler for extensible language support

> **Sequencing note (from review):** This is a follow-up refactor, NOT
> part of the issue #215 remediation milestone. Changes
> `fix-dir-language-inference` and `reconcile-config-schema` fully address
> the reported bug (discovery of `lib/main.ts`) and the documentation
> no-op. This refactor is deferred until after those land, so a
> user-visible fix is not blocked behind a large equivalence refactor.

## Motivation

Issue #215 also asks about broader language support (the reporter wanted
Dart). Today adding a language requires five touchpoints plus ~330-450
lines of near-duplicated code per language:

1. A `Language` variant plus four methods (`from_extension`,
   `from_language_str`, `as_str`, `reconciler_id`) in
   `src/reconcile/target.rs`, and an entry in `SUPPORTED_LANGUAGES`.
2. A new `tree-sitter-X` Cargo dependency.
3. A new reconciler module (~333-457 lines: code 457, typescript 378,
   python 333, go 333; ~1500 lines across four).
4. A new `match` arm in `src/scanner/mod.rs:129-143`.
5. `pub mod X;` in `src/reconcile/mod.rs`.

The four existing reconcilers are structurally near-identical: each
re-implements `discover_X_files` -> `walk` -> parallel chunks ->
`public_symbols` -> `collect_public_symbols` -> `symbol_kind` ->
`interface_symbol`. The only genuine per-language knowledge is the
tree-sitter grammar, the file extension set, and which AST node kinds
count as exported. There is no registry: dispatch is a hardcoded `match`
(a search for `Box<dyn Reconciler>` / `HashMap<ReconcilerId, ...>`
returns zero matches).

This makes the feature request for more languages gated on a refactor,
not just grunt-work. It also blocks the `tree_sitter_languages` config
documented in `docs/spec.md` from ever being real.

## Scope

- Extract a generic `CodeReconciler` parameterized by a `LanguageSpec`:
  grammar (`tree_sitter::Language`), file extensions, the set of
  "exportable" AST node kinds, and a `symbol_kind` map. The shared
  pipeline (discover, parse, collect, fingerprint) lives once.
- Reimplement the four existing reconcilers as `LanguageSpec` instances,
  deleting the duplicated pipeline code.
- Introduce a static registry (`LanguageSpec` table) so
  `reconcile_targets` dispatches by lookup instead of a `match` arm, and
  adding a language becomes "register a spec plus add a grammar dep".
- This unblocks (but does not itself add) Dart and other languages, and
  makes the `tree_sitter_languages` config implementable.

## Out of scope

- Adding Dart (or any new language). This change only refactors existing
  ones; new languages are follow-ups that become cheap.
- Implementing `tree_sitter_languages` dynamic selection (follow-up after
  the registry exists; tracked separately).

## Acceptance

- All four existing reconcilers produce byte-identical `ReconcileReport`
  output (claimed files, symbols, symbol records, findings) before and
  after the refactor, verified by the reconciler test suite and
  interface-hash stability on the self-host scan.
- `src/reconcile/` line count drops substantially (the ~1500 lines of
  duplicated pipeline collapse into one shared implementation plus four
  small specs).
- The post-change-A `TargetReport.hash` (`Option<String>`) `Some`/`None`
  semantics are preserved by the registry path (asserted via
  `cairn files --json` for normal and no-hash targets).
- Adding a hypothetical language is demonstrable by a test that registers
  a spec without a new reconciler module.
- `cargo test`, `cairn scan`, `cairn hook all`, `cargo clippy -D warnings
  --all-targets` pass; file-size gate satisfied.
