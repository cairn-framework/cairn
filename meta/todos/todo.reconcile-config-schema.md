---
node: cairn.kernel.scanner
status: done
created: 2026-07-09
satisfies: reconcile-config-schema
---

# Reconcile cairn.config.yaml spec with the parser

`docs/spec.md:199-207` documents a `reconcilers:` / `tree_sitter_languages`
config block the parser (`src/scanner/config/mod.rs`) never implements;
it is silently dropped. Correct the spec to document the real `targets:`
override, and warn on unknown top-level config keys. See
`meta/changes/reconcile-config-schema/`.
