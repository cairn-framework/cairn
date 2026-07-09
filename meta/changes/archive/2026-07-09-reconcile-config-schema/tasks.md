# Tasks: reconcile-config-schema

- [x] Rewrite docs/spec.md config example to match parser, using real YAML keys (`targets`, `ignore`, `multi_target`, `context`, `rules`, `artefact_types`)
- [x] Label `reconcilers:`/`tree_sitter_languages` as future, cross-ref `generic-language-reconciler`
- [x] Add known-top-level-key set to `parse_config` using the real YAML keys (not struct field names)
- [x] Add a config-warning carrier (`Config.findings` or `ConfigWarning`); emit `CAIRN_CONFIG_UNKNOWN_KEY` inline `Finding` on unknown top-level key (non-aborting)
- [x] Convert config warnings into `Finding`s in `load_project` before `build_graph` (so they reach `cairn scan` / `cairn lint`)
- [x] Add `[findings.codes]` heading/body/cta entry in `copy.toml` (follow `src/copy.rs` lookup pattern)
- [x] Tests: unknown key warns and does not abort; warning reaches scan/lint output; reporter's block warns on `reconcilers` with a `targets:` pointer
- [x] Gates: `cargo test`, `cairn scan`, `cairn hook all`, `cargo clippy -D warnings --all-targets`
