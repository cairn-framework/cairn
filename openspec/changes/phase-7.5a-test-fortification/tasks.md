# Tasks: Phase 7.5a Test Fortification

## 1. Snapshot Infrastructure

- [ ] 1.1 Add `insta = { version = "1", features = ["json", "yaml"] }` to `[dev-dependencies]` in `Cargo.toml`.
- [ ] 1.2 Create `tests/snapshots/.gitignore` permitting committed `*.snap` files and ignoring `*.snap.new`.
- [ ] 1.3 Create `docs/testing.md` documenting the `cargo insta review` workflow, snapshot file layout, and when to use inline vs file-based snapshots.

## 2. Wire-Format Snapshots

- [ ] 2.1 Snapshot test for `GET /api/meta`.
- [ ] 2.2 Snapshot test for `GET /api/status`.
- [ ] 2.3 Snapshot test for `GET /api/graph` against the bootstrap fixture.
- [ ] 2.4 Snapshot test for `GET /api/lint`.
- [ ] 2.5 Snapshot test for `GET /api/blueprint`.
- [ ] 2.6 Snapshot test for `GET /api/node/:id` against a representative node in the bootstrap fixture.
- [ ] 2.7 Snapshot tests for each per-artefact endpoint: `contract`, `decisions`, `todos`, `research`, `sources`, `rationale`.
- [ ] 2.8 Snapshot tests for `GET /api/depends/:id` and `GET /api/dependents/:id`.

## 3. God-Module Unit Tests

Each sub-task is an atomic commit group (one commit per module; file + inline test block + allow-list comment):

- [ ] 3.1 `src/ui.rs`: add `// cairn:allow-large-module reason: scheduled-for-phase-7.5b-split` at the top. Add inline `#[cfg(test)] mod tests` covering route dispatch, 404 behaviour, content-type, `schema_version` field presence, unsupported-method handling.
- [ ] 3.2 `src/cli/mod.rs`: same allow-list comment. Tests per top-level command verb, both human and `--json` output, primary error surfacing.
- [ ] 3.3 `src/changes.rs`: same allow-list comment. Tests for delta parse/serialise round-trip for ADDED/MODIFIED/REMOVED/RENAMED plus one conflict-detection case.
- [ ] 3.4 `src/query_api.rs`: same allow-list comment. Public-function boundary tests with at least one positive and one negative case per exported function.
- [ ] 3.5 `src/artefacts/registry.rs`: same allow-list comment. Register / lookup / unknown-kind / duplicate-kind cases.

## 4. File-Size Gate

- [ ] 4.1 Author `scripts/check-file-sizes.sh` with the 500-line ceiling and `// cairn:allow-large-module reason: <text>` allow-list mechanism.
- [ ] 4.2 Wire the script into `scripts/pre-archive-rust-gates.sh` as the final check.
- [ ] 4.3 Add a self-test under `tests/scripts/` (or a Rust integration test invoking the shell script) verifying: exactly-500 passes, 501 fails, allow-list honoured, missing reason rejected.

## 5. CFLX Script and Convention Updates

- [ ] 5.1 Extend `scripts/cflx-analyze-cairn-phases.py` regex to `^phase-(\d+)(?:\.(\d+))?([a-z]?)-`. Update sort key to the tuple `(major, minor, suffix)`.
- [ ] 5.2 Add a `python -m unittest` or doctest covering the new ordering for plain integer, decimal, and suffix cases.
- [ ] 5.3 Add "Test-first pre-phase" section to `openspec/conventions.md` defining the `phase-<N>.0-tests` pattern and the `#[ignore = "awaits phase-<N>"]` convention.
- [ ] 5.4 Add to the testing section of `openspec/conventions.md`: "Public JSON wire formats SHALL be pinned via `insta` snapshot tests."
- [ ] 5.5 Cross-reference the new convention from `AGENTS.md` (one paragraph, with pointer to the convention entry).

## 6. Required Verification

- [ ] 6.1 `cargo build` passes with zero warnings.
- [ ] 6.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [ ] 6.3 `cargo fmt --check` passes.
- [ ] 6.4 `cargo test` passes; all new snapshots reviewed and accepted via `cargo insta review` before commit.
- [ ] 6.5 `scripts/pre-archive-rust-gates.sh` passes end-to-end including the new file-size check.
- [ ] 6.6 The five god modules still exceed 500 lines but carry the allow-list comment; pre-archive gate passes.
