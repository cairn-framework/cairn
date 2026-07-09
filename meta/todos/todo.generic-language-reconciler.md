---
node: cairn.reconcile
status: open
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
