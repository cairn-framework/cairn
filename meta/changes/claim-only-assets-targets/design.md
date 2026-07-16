# Design: claim-only-assets-targets

## Approach

1. Add `Language::Assets` variant to `Language` enum in `src/reconcile/target.rs`.
2. Allow `assets` in `cairn.config.yaml` target overrides by separating configuration validation from inference language constraints.
3. Path-match in `build_targets` when matching overrides to allow single nodes (e.g. `cairn.ui`) to register multiple path-distinct targets.
4. Add Assets targets walk handling in `reconcile_targets` (scanner mod.rs) and `build_reports_from_cache` (scanner cache.rs) to collect files and return an empty symbols set and `hash: None`.
