# Tasks: Phase 3 Changes

## 1. Change Discovery and Parsing

- [ ] 1.1 Implement active change discovery under `meta/changes/`.
- [ ] 1.2 Parse `proposal.md`, optional `design.md`, and `dsl.delta`.
- [ ] 1.3 Implement DSL delta operations for added, modified, removed, and renamed nodes and edges.
- [ ] 1.4 Parse artefact operation frontmatter in mirrored change directories.
- [ ] 1.5 Add parser tests for every delta section and invalid operation.

## 2. Validation

- [ ] 2.1 Validate all delta references against current truth.
- [ ] 2.2 Validate artefact operation targets and required fields.
- [ ] 2.3 Validate archive order conflicts inside a single change.
- [ ] 2.4 Add tests for invalid references, missing files, duplicate operations, and malformed rename operations.

## 3. Archive

- [ ] 3.1 Implement atomic archive with snapshot and rollback.
- [ ] 3.2 Apply operations in renamed, removed, modified, added order.
- [ ] 3.3 Run a validation scan after applying deltas with the archiving change excluded from active-change discovery and roll back on structural errors or unresolved interface contradictions.
- [ ] 3.4 Move successful changes to `meta/changes/archive/YYYY-MM-DD-<change-id>/`.
- [ ] 3.5 Run a final output scan after the move so generated status no longer lists the archived change as active.
- [ ] 3.6 Append archive events to `.cairn/log.md`.
- [ ] 3.7 Add integration tests for success, rollback, and active-change output after archive.

## 4. Rename

- [ ] 4.1 Implement `cairn rename <old-id> <new-id>`.
- [ ] 4.2 Generate a change directory with DSL rename operations and edge updates.
- [ ] 4.3 Copy and modify artefact frontmatter references into the change directory.
- [ ] 4.4 Add tests proving the main tree is unchanged until archive.

## 5. Queries

- [ ] 5.1 Implement `changes`, `show`, `archive`, and `rename` commands.
- [ ] 5.2 Add `--include-changes` support to `neighbourhood`.
- [ ] 5.3 Update `status` to include active changes.
- [ ] 5.4 Add JSON schemas and human output snapshots for change-aware commands.

## 6. Documentation

- [ ] 6.1 Document change directory layout and operation frontmatter.
- [ ] 6.2 Document archive ordering and rollback guarantees.
- [ ] 6.3 Document rename workflow and post-archive reference guarantees.

## 7. Required Verification

- [ ] 7.1 `cargo build` passes with zero warnings.
- [ ] 7.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [ ] 7.3 `cargo fmt --check` passes.
- [ ] 7.4 `cargo test` passes.
- [ ] 7.5 `cargo test --locked` passes.
- [ ] 7.6 `python3 .agents/skills/cflx-proposal/scripts/cflx.py validate phase-3-changes --strict` passes.
