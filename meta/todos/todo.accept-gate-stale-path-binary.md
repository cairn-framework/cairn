---
node: cairn.kernel.cli
status: done
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
- `cairn change accept` runs its lint leg against the binary already running the
  gate (`std::env::current_exe`), never a `cairn` resolved from `PATH`.
- A regression test asserts the accept gate does not resolve `cairn` from
  `PATH`, in the shape `tests/dogfood_gate.rs` already uses.
- Running `cairn change accept` with a deliberately stale `cairn` earlier on
  `PATH` produces the same verdict as running it without one.

## Acceptance amendment, 2026-08-10
Clause 1 originally required "the same mechanism `scripts/dogfood.sh` uses",
namely `cargo run --bin cairn`. That mechanism is repo-local and cannot ship:
`cairn change accept` runs in the adopter's project root, which holds no cairn
crate, so adopter-local `cargo run` either fails or grades an unrelated binary
of the same name. `std::env::current_exe` carries the same intent in shippable
form. It never consults `PATH` and grades the binary the user invoked.

The measured commands and results are recorded in
`res.authoreval-instrument-evidence` section 2, subsection "Resolution,
2026-08-10", which also states the residual limit. Clauses 2 and 3 are
unchanged and are the behavioural test of the fix.

## Sizing
S.

## Non-goals
Do not change which legs the battery runs, or their order.
