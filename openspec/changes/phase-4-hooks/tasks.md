# Tasks: Phase 4 Hooks

## 1. Hook Engine

- [x] 1.1 Implement hook kind types, hook report types, and exit decision logic.
- [x] 1.2 Reuse scanner and lint findings without duplicate filesystem scans.
- [x] 1.3 Add JSON and human output rendering for hook reports.

## 2. Structural, Interface, and Tension Hooks

- [x] 2.1 Implement structural hook blocking on structural errors.
- [x] 2.2 Implement interface hook blocking on unresolved interface contradictions.
- [x] 2.3 Implement tension hook reporting rationale tensions with exit code `0`.
- [x] 2.4 Implement `hook all` combined semantics.

## 3. Active Change Conflicts

- [x] 3.1 Detect overlapping blueprint node and edge operations.
- [x] 3.2 Detect artefact path operation collisions.
- [x] 3.3 Detect incompatible rename chains.
- [x] 3.4 Surface conflicts as structural hook failures.
- [x] 3.5 Reuse the same conflict detector inside `cairn archive <change>` before any snapshot or mutation, and add a direct-archive bypass regression test.

## 4. Entrypoints and Documentation

- [x] 4.1 Add CLI commands for all hook kinds.
- [x] 4.2 Add a committed script that invokes `cairn hook all` for Git and agent-task-end integration.
- [x] 4.2a Update the Phase 0 pre-commit hook from `cargo fmt --check` only to also enforce `cairn hook all`, so structural and interface integrity are gated on every commit.
- [x] 4.3 Document hook exit semantics and integration commands.

## 5. Required Verification

- [x] 5.1 `cargo build` passes with zero warnings.
- [x] 5.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [x] 5.3 `cargo fmt --check` passes.
- [x] 5.4 `cargo test` passes.
- [x] 5.5 `cargo test --locked` passes.
- [x] 5.6 `python3 .agents/skills/cflx-proposal/scripts/cflx.py validate phase-4-hooks --strict` passes.
