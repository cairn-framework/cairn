# Cairn Atomic Improvement Loop

## Current Status
- In-progress: none

## Last Completed
- Extended `tests/conventions.rs` to enforce `// Reason:` comments on inner `#![allow(...)]` attributes as well as outer `#[allow(...)]`.
  - Bead: cairn-b33
  - Commit: 7504b8f

## Result
- `scripts/check-file-sizes.sh` reports no oversized files in `src/`.
- `cairn lint` reports no findings.
- `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --locked` all pass.
- All `#[allow(...)]` and `#![allow(...)]` directives in `src/` and `tests/` now document their rationale.

## Next Candidates
1. Run `cargo clippy` with pedantic lints and address low-risk findings.
2. Audit `TODO`/`FIXME` comments or `unimplemented!()` stubs for quick fixes.
3. Investigate slow tests or add targeted tests for recently-split modules.
