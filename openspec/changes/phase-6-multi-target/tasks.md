# Tasks: Phase 6 Multi-Target and Languages

## 1. Target Model

- [ ] 1.1 Add target records for single-path and path-list nodes.
- [ ] 1.2 Update path ownership indexes to track node ID plus target path.
- [ ] 1.3 Preserve single-path behavior from earlier phases.

## 2. Reconcilers

- [ ] 2.1 Add language detection through optional explicit target config and file extensions, with config taking precedence.
- [ ] 2.2 Implement TypeScript reconciler.
- [ ] 2.3 Implement Python reconciler.
- [ ] 2.4 Implement Go reconciler.
- [ ] 2.5 Dispatch all supported languages through the shared `Reconciler` trait.
- [ ] 2.6 Implement canonical public interface extraction and sorting rules for Rust, TypeScript, Python, and Go.

## 3. State and Divergence

- [ ] 3.1 Store interface hashes by node ID and target path.
- [ ] 3.2 Migrate existing single-hash state into target-hash state.
- [ ] 3.3 Detect target interface divergence.
- [ ] 3.4 Implement and document intentional asymmetry markers.
- [ ] 3.5 Add tests proving private symbols, comments, formatting, and source order do not change interface hashes.
- [ ] 3.6 Add tests for contradiction and tension cases.

## 4. CLI and Output

- [ ] 4.1 Update `get`, `files`, `lint`, and `scan` to expose target-level state.
- [ ] 4.2 Update JSON schemas for target arrays and per-target findings.
- [ ] 4.3 Add human-readable output snapshots for multi-target modules.

## 5. Documentation

- [ ] 5.1 Document path-list reconciliation.
- [ ] 5.2 Document supported languages, explicit config override, and detection order.
- [ ] 5.3 Document interface hash state format and migration behavior.

## 6. Required Verification

- [ ] 6.1 `cargo build` passes with zero warnings.
- [ ] 6.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [ ] 6.3 `cargo fmt --check` passes.
- [ ] 6.4 `cargo test` passes.
- [ ] 6.5 `cargo test --locked` passes.
- [ ] 6.6 `python3 .agents/skills/cflx-proposal/scripts/cflx.py validate phase-6-multi-target --strict` passes.
