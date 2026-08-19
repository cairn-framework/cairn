---
node: cairn.root
status: open
created: 2026-08-11
---

# Public Error Boundary Convention Divergence

## Scope

`docs/conventions.md` Error Types says that subsystem errors surfaced to users
or returned from a public API MUST convert to `CairnError` via `From`, and the
Result Convention says every public API MUST return `Result<T, CairnError>`.
The correctness receipt for `dec.conventions-error-types` found direct
counterexamples: public `artefacts::frontmatter::{set_field, upsert_field,
remove_field}` return `SetFieldError`, and public UI APIs return `UiError`, with
no `From<SetFieldError>` or `From<UiError>` implementation for `CairnError`.

This is separate from `todo.conventions-thiserror-divergence`, which owns only
the derive-macro formatting rule. Resolve this remaining rule/code divergence
without expanding that completed unit.

Evidence: `res.conventions-error-types-alignment`; correctness receipt for
`dec.conventions-error-types`.

## Dependencies

None.

## Acceptance

- Either the public error boundaries are converted to `CairnError`, or the two
  universal convention bullets are narrowed to the public boundary the
  codebase actually provides.
- `docs/conventions.md` and every affected public API agree.
- A regression test protects the selected boundary.
- `cargo clippy --all-targets --all-features -- -D warnings` and
  `cargo test --workspace` pass.

## Non-goals

Do not revisit the hand-written formatting rule decided by
`dec.conventions-error-types`.
