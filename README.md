# Cairn

Cairn is a structural graph query layer over a human-authored architecture DSL. It produces an ontology for a project: a map of what exists (modules, dependencies, contracts, decisions, research, sources) and how the pieces relate. Agents and humans query the ontology instead of scanning the repo.

## Status

Specification complete (v0.6). Implementation in progress — see `docs/spec.md` for the full specification and `openspec/changes/` for active work.

## Development Setup

Cairn is a Rust workspace. After cloning, install the local Git format hook:

```sh
scripts/install-pre-commit-hook.sh
```

The hook recreates `.git/hooks/pre-commit`, which is not committed by Git, and
runs `cargo fmt --check` before each commit.

Run the local quality suite before pushing:

```sh
make check
```

`make check` runs `cargo fmt --check`,
`RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features`,
`cargo test`, and `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps`.

The Conflux archive gate for this repository is
`scripts/pre-archive-rust-gates.sh`. It enforces formatting, strict Clippy, and
tests before a change is archived.

## Phase 0 Scope

Phase 0 intentionally provides only the Rust workspace, lint policy, hook
scripts, and smoke tests. It does not implement the DSL parser, ontology graph,
query commands, scanner, artefact handling, reconciler behavior, or future Cairn
domain modules.

## Reference

- `docs/spec.md` — Cairn v0.6 specification
- `docs/dsl.md` — DSL grammar reference
- `test/fixtures/cairn.dsl` — example DSL file
- `meta/campaigns/rust-full-spec.md` — implementation roadmap
