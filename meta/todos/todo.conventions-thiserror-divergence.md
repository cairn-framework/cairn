---
node: cairn.root
status: done
created: 2026-08-09
---

# Conventions Require thiserror And Nothing Uses It

## Scope
`docs/conventions.md`, section "Error Types", states that all Cairn error types
MUST use `thiserror::Error` for derivation. `thiserror` appears in neither
`Cargo.toml` nor `Cargo.lock`, and `CairnError` hand-writes its `Display`
implementation. The rule has zero implementations in the repository.

A MUST that nothing satisfies is worse than no rule: every new error type has
to decide whether to obey a convention the codebase contradicts, and each
session relitigates it. Resolve it in one direction.

Evidence: `res.authoreval-instrument-evidence` section 3.

## Dependencies
None.

## Acceptance
- Either `thiserror` is adopted (dependency added, `CairnError` and the
  subsystem error types converted) or the rule is removed from
  `docs/conventions.md` and replaced by the convention the codebase actually
  follows.
- Whichever direction is taken, `docs/conventions.md` and the code agree, and
  a reader can tell which by reading either one.
- `cargo clippy --all-targets --all-features -- -D warnings` and
  `cargo test --workspace` pass.

## Sizing
S if the rule is removed. M if `thiserror` is adopted.

## Non-goals
Do not change any error's message text, code, or variant set while converting.
