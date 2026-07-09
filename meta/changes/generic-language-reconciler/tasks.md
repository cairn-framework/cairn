# Tasks: generic-language-reconciler

Deferred follow-up after `fix-dir-language-inference` + `reconcile-config-schema`
land. Depends on change A (the `Unknown` variant and the
`TargetReport.hash: Option<String>` semantics feed the registry's no-spec
and no-hash paths).

- [ ] Capture baseline AFTER change A lands: snapshot `ReconcileReport` per language (claimed_files, symbols, findings) + self-host interface hashes + a `cairn files --json` / scanner assertion covering `TargetReport.hash` `Some` vs `None` (the self-host guard only exercises non-empty hashes today)
- [ ] Define `LanguageSpec` { grammar, extensions, exportable_kinds, kind_to_symbol, interface_symbol }
- [ ] Implement shared `CodeReconciler` pipeline (discover/parse/collect/fingerprint) over `LanguageSpec`
- [ ] Move `eligible_owners` / `most_specific_owner` into shared pipeline (language-agnostic)
- [ ] Define four `LanguageSpec` instances; delete per-language `discover_*`/`walk`/`public_symbols`/`collect_public_symbols`/`symbol_kind`/`interface_symbol`
- [ ] Add static `LANGUAGES` registry; replace `match` in `reconcile_targets` with lookup
- [ ] Assert byte-identical `ReconcileReport` per language vs baseline; assert `Some`/`None` hash semantics preserved
- [ ] Add a test registering a spec without a new module (proves extension is cheap)
- [ ] Gates: `cargo test`, `cairn scan` (hashes unchanged), `cairn hook all`, `cargo clippy -D warnings --all-targets`, file-size gate, wire snapshots unchanged
