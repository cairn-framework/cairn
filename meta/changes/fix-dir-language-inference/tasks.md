# Tasks: fix-dir-language-inference

Sequenced so each step is independently verifiable.

- [x] Ratify inference policy (infer-from-contents vs require-declaration); default chosen, no artefact needed
- [x] Add `Language::Unknown` variant + `as_str`/`reconciler_id` (target.rs)
- [x] Add `Language::infer_from_directory(root, path, ignores)` (target.rs)
- [x] Replace `unwrap_or(Language::Rust)` at scanner/mod.rs:96 with extension fast-path -> infer -> Unknown
- [x] Skip `Unknown` in `reconcile_targets`; emit `CAIRN_RECONCILE_LANGUAGE_UNKNOWN` warning (inline `Finding`)
- [x] Suppress hash + emit `CAIRN_RECONCILE_EMPTY_TARGET` when `claimed_files` empty
- [x] Make `TargetReport.hash` `Option<String>` and propagate to ALL readers:
  - `cairn files` renderer (`src/cli/render/node.rs`)
  - query API/MCP files endpoint (`src/query_api/handlers/node.rs:84` `files_json`); `None` represented by omitting `hash` key
  - cache reconstruction (`src/scanner/cache.rs` `build_reports_from_cache`)
  - `.cairn/state/interface-hashes.json` persistence + aggregate `interface_hash` (`src/scanner/mod.rs`)
  - `detect_divergence` (`src/scanner/mod.rs:176`)
- [x] Add `[findings.codes]` heading/body/cta entries in `copy.toml` for both new codes
- [x] Route both new codes in `src/query_api/handlers/remediate.rs:173-179`
- [x] Tests: .ts dir, mixed dir, empty/unsupported dir, targets-override-wins, self-host hashes unchanged, fresh + cached scan round-trip preserves `None`
- [x] Wire-format snapshot: no existing snapshot covers the `files` endpoint hash field; `interface_hash` on bootstrap fixture changed from `bd60acb658c79e45` to empty (the fix)
- [x] Gates: `cargo test` (1540 passed), `cairn scan` (0 new findings), `cairn hook all` (exit 0), `cargo clippy -D warnings --all-targets` (clean)
