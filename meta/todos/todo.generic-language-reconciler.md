---
node: cairn.reconcile
status: done
created: 2026-07-09
satisfies: generic-language-reconciler
---

# Generic parameterized reconciler for extensible language support

The four language reconcilers are ~1500 lines of near-duplicated pipeline.
Extract a `LanguageSpec`-parameterized `CodeReconciler` plus a static
registry so dispatch is a lookup, not a `match` arm, and adding a language
is a spec plus a grammar dep rather than a new ~350-line module.
Unblocks broader language support (e.g. Dart) and the `tree_sitter_languages`
config. Depends on `fix-dir-language-inference`. See
`meta/changes/generic-language-reconciler/`.

## Resolution (2026-07-09)

Implemented on `feat/generic-language-reconciler`. `LanguageSpec` plus a
shared `CodeReconciler` pipeline live in `src/reconcile/generic.rs` (456
lines); the four language modules shrank to spec definitions and the
scanner dispatch is a `LANGUAGES` registry lookup with a warning path for
unknown languages. Equivalence proven by `tests/reconcile_baseline.rs`:
byte-identical `ReconcileReport` per language against baselines captured
with the ORIGINAL reconcilers (Rust source-text signatures, TS
export_statement unwrapping, python/go kind:name formats all replicated
exactly), plus `TargetReport.hash` Some/None semantics and an
alternate-spec test proving the spec-only pipeline boundary. Self-host
`cairn scan` interface hash unchanged vs main; wire snapshots unchanged.
Review-gate fix: committed Rust fixtures carry a `.fixture` suffix and are
materialised into a tempdir at test runtime so the self-host scan cannot
claim them.
