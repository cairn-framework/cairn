# Tasks: Phase 3 Changes

## 1. Change Discovery and Parsing

- [x] 1.1 Implement active change discovery under `meta/changes/`.
- [x] 1.2 Parse `proposal.md`, optional `design.md`, and `blueprint.delta`.
- [x] 1.3 Implement blueprint delta operations for added, modified, removed, and renamed nodes and edges.
- [x] 1.4 Parse artefact operation frontmatter in mirrored change directories.
- [x] 1.5 Add parser tests for every delta section and invalid operation.

## 2. Validation

- [x] 2.1 Validate all delta references against current truth.
- [x] 2.2 Validate artefact operation targets and required fields.
- [x] 2.3 Validate archive order conflicts inside a single change.
- [x] 2.4 Add tests for invalid references, missing files, duplicate operations, and malformed rename operations.

## 3. Archive

- [x] 3.1 Implement atomic archive with snapshot and rollback.
- [x] 3.2 Apply operations in renamed, removed, modified, added order.
- [x] 3.3 Run a validation scan after applying deltas with the archiving change excluded from active-change discovery and roll back on structural errors or unresolved interface contradictions.
- [x] 3.4 Move successful changes to `meta/changes/archive/YYYY-MM-DD-<change-id>/`.
- [x] 3.5 Run a final output scan after the move so generated status no longer lists the archived change as active.
- [x] 3.6 Append archive events to `.cairn/log.md`.
- [x] 3.7 Add integration tests for success, rollback, and active-change output after archive.

## 4. Rename

- [x] 4.1 Implement `cairn rename <old-id> <new-id>`.
- [x] 4.2 Generate a change directory with blueprint rename operations and edge updates.
- [x] 4.3 Copy and modify artefact frontmatter references into the change directory.
- [x] 4.4 Add tests proving the main tree is unchanged until archive.

## 5. Queries

- [x] 5.1 Implement `changes`, `show`, `archive`, and `rename` commands.
- [x] 5.2 Add `--include-changes` support to `neighbourhood`.
- [x] 5.3 Update `status` to include active changes.
- [x] 5.4 Add JSON schemas and human output snapshots for change-aware commands.

## 6. Documentation

- [x] 6.1 Document change directory layout and operation frontmatter.
- [x] 6.2 Document archive ordering and rollback guarantees.
- [x] 6.3 Document rename workflow and post-archive reference guarantees.

## 7. Required Verification

- [x] 7.1 `cargo build` passes with zero warnings.
- [x] 7.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [x] 7.3 `cargo fmt --check` passes.
- [x] 7.4 `cargo test` passes.
- [x] 7.5 `cargo test --locked` passes.
- [x] 7.6 `python3 .agents/skills/cflx-proposal/scripts/cflx.py validate phase-3-changes --strict` passes.
