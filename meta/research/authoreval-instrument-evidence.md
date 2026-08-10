---
id: res.authoreval-instrument-evidence
nodes:
  - cairn.authoreval
  - cairn.kernel.cli
  - cairn.kernel.query
date: 2026-08-09
method: primary
---

# Building the authorability eval instrument: what the construction showed

Recorded while delivering `todo.authorability-eval-instrument`. Three
observations changed the design or the plan; each is reproduced below with the
command that produced it.

## 1. `cairn remediate <finding-code>` ignores its argument

The failure taxonomy was first designed to attribute a hotspot to a missing
repair affordance whenever `cairn remediate <code>` produced no plan for that
code. That design is not implementable against the shipped surface.

At `e0e017ae`, from the repository root:

```
cairn remediate CAIRN_RESEARCH_ORPHAN --json
cairn remediate CAIRN_DECISION_ACCUMULATION --json
cairn remediate CAIRN_NOT_A_REAL_CODE --json
```

All three return byte-identical plans, exit 0, and lead with the same
`cairn todos` action. A code that does not exist is not distinguished from one
that does, so the output carries no per-code signal at all. `cairn --help`
documents the argument as "Optional finding code to focus the plan".

Consequence for the instrument: `missing_repair_affordance` is defined by
persistence instead. A finding earns that class when the same code was present
in the immediately preceding failed scan and survived a repair attempt, which
needs no remediation query and is decidable from the run itself. The class is
only assignable from attempt 2 onward, because a first-shot failure has no
preceding scan it could have survived.

Filed as `todo.remediate-code-filter-ignored` against `cairn.kernel.query`.

## 2. `cairn change accept` grades a PATH binary, not the working tree

`src/cli/accept/mod.rs:60` runs the lint leg with
`run_command("cairn", &["lint", "--strict", id], project_root, json)`. That
resolves `cairn` from `PATH`.

Observed in the loop worktree at `e0e017ae` plus this unit's changes:

```
./target/debug/cairn lint --strict authorability-eval-instrument   # exit 0
cairn lint --strict authorability-eval-instrument                  # exit 1
```

Both binaries report `cairn 0.9.0`. The `PATH` copy at
`~/.cargo/bin/cairn` is an older build whose ratification manifest computation
differs, so it reported
`CAIRN_DECISION_CONVERGENCE_UNMET` against `dec.bootstrap-fixture-corpus-split`,
a decision untouched by this unit. `cairn change accept driver-v2-selection`
failed identically, which is what established the failure as environmental
rather than caused by this unit. With the working-tree binary first on `PATH`,
all six legs pass.

This is the same defect class that `scripts/dogfood.sh` and
`tests/dogfood_gate.rs` already close for the pre-push gate: a gate that grades
a stale binary can pass a broken tree or fail a correct one. The acceptance gate
was not converted.

Filed as `todo.accept-gate-stale-path-binary` against `cairn.kernel.cli`.

### Resolution, 2026-08-10

Delivered on `todo.accept-gate-stale-path-binary`. The paragraph above proposed
converting acceptance to the mechanism `scripts/dogfood.sh` uses; measurement
while delivering the fix showed that mechanism cannot ship. `cairn change
accept` runs in the adopter's project root, while `scripts/dogfood.sh` reaches
cairn through the cairn crate's own manifest. In an adopter tree:

```
cargo run --manifest-path <rust-adopter>/Cargo.toml --bin cairn -- lint --strict demo
# exit 101: no bin target named `cairn` in default-run packages
cargo run --manifest-path <non-rust-adopter>/Cargo.toml --bin cairn -- lint --strict demo
# exit 101: manifest path does not exist
```

A Rust adopter that happens to declare its own `cairn` binary target is worse
again: the command succeeds and grades that unrelated target. Adopter-local
`cargo run` cannot reliably select cairn's binary in either direction.

Acceptance therefore resolves the lint leg through `std::env::current_exe`,
which never consults `PATH` and grades the binary the user invoked. The two
mechanisms differ in form and agree in intent: a gate reaches cairn through the
build it is part of, never through whatever `PATH` offers. Residual limit: a
path is not an image handle, so a binary replaced between process start and the
lint spawn would run the replacement. That window is seconds wide against a
persistent stale-install condition, and closing it means running lint in
process, which the todo's non-goals rule out. The todo's acceptance clause was
amended in the same commit.

## 3. `docs/conventions.md` requires `thiserror` and nothing uses it

Section "Error Types" states that all Cairn error types MUST use
`thiserror::Error` for derivation. `thiserror` appears in neither `Cargo.toml`
nor `Cargo.lock`, and `CairnError` implements `Display` by hand. The rule has no
implementations anywhere in the repository.

This unit added a `CairnError` variant rather than a new error type, so it
introduced no new divergence, and adopting the rule would mean a new dependency
plus a refactor of a stable module: out of scope for one loop unit.

Filed as `todo.conventions-thiserror-divergence` against `cairn.root`.

## What this does not show

Nothing here measures authoring quality. The instrument now exists and its
offline path is exercised end to end, but no model has been run against it. That
is `todo.authorability-eval-prompt-corpus`, and the numbers it produces are the
parent's actual result.
