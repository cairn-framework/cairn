# Tasks: fix-dir-language-inference

Sequenced so each step is independently verifiable.

- [ ] Ratify inference policy (infer-from-contents vs require-declaration); author `dec.language-inference-policy` if non-default chosen
- [ ] Add `Language::Unknown` variant + `as_str`/`reconciler_id` (target.rs)
- [ ] Add `Language::infer_from_directory(root, path, ignores)` (target.rs)
- [ ] Replace `unwrap_or(Language::Rust)` at scanner/mod.rs:96 with extension fast-path -> infer -> Unknown
- [ ] Skip `Unknown` in `reconcile_targets`; emit `CAIRN_RECONCILE_LANGUAGE_UNKNOWN` warning (inline `Finding`)
- [ ] Suppress hash + emit `CAIRN_RECONCILE_EMPTY_TARGET` when `claimed_files` empty
- [ ] Make `TargetReport.hash` `Option<String>` and propagate to ALL readers:
  - `cairn files` renderer (`src/cli/render/node.rs`) + JSON serialiser
  - cache reconstruction (`src/scanner/cache.rs` `build_reports_from_cache`)
  - `.cairn/state/interface-hashes.json` persistence + aggregate `interface_hash` (`src/scanner/mod.rs`)
  - `detect_divergence` (`src/scanner/mod.rs:176`)
- [ ] Add `[findings.codes]` heading/body/cta entries in `copy.toml` for both new codes (no central registry; follow `src/copy.rs` lookup pattern)
- [ ] Route both new codes in `src/query_api/handlers/remediate.rs:173-179`; test remediation output
- [ ] Tests: .ts dir, mixed dir, empty/unsupported dir, targets-override-wins, self-host hashes unchanged, fresh + cached scan round-trip preserves `None`
- [ ] Update wire-format snapshot for `cairn files --json` as a recorded change
- [ ] Gates: `cargo test`, `cairn scan`, `cairn hook all`, `cargo clippy -D warnings --all-targets`
