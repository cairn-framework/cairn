---
node: cairn.kernel.cli
status: open
created: 2026-08-09
---

# Accept Gate Grades A Stale PATH Binary

## Scope
`src/cli/accept/mod.rs` runs the lint leg with
`run_command("cairn", &["lint", "--strict", id], project_root, json)`, which
resolves `cairn` from `PATH`. A developer with an older `cairn` installed has
`cairn change accept` grading that binary instead of the working tree, so the
gate can fail a correct tree or pass a broken one.

This is the defect class `scripts/dogfood.sh` and `tests/dogfood_gate.rs`
already close for the pre-push gate, using `cargo run --bin cairn`. The
acceptance gate was not converted.

Evidence and reproduction: `res.authoreval-instrument-evidence` section 2,
where a stale `~/.cargo/bin/cairn` reporting the same `0.9.0` version string
failed `cairn change accept` on two unrelated changes.

## Dependencies
None.

## Acceptance
- `cairn change accept` runs its lint leg against the working-tree binary, by
  the same mechanism `scripts/dogfood.sh` uses.
- A regression test asserts the accept gate does not resolve `cairn` from
  `PATH`, in the shape `tests/dogfood_gate.rs` already uses.
- Running `cairn change accept` with a deliberately stale `cairn` earlier on
  `PATH` produces the same verdict as running it without one.

## Sizing
S.

## Non-goals
Do not change which legs the battery runs, or their order.
