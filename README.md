# Cairn

Cairn is a structural graph query layer over a human-authored architecture blueprint. It produces a map for a project: a map of what exists (modules, dependencies, contracts, decisions, research, sources) and how the pieces relate. Agents and humans query the map instead of scanning the repo.

## Status

Specification complete (v0.7). Implementation in progress — see `docs/spec.md` for the full specification and `openspec/changes/` for active work.

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

## Phase 1 Kernel

The Phase 1 kernel parses `cairn.blueprint`, builds a queryable map graph, loads
contract Markdown artefacts, reconciles Rust source files against declared
module paths, and exposes the first CLI query surface.

Common commands:

```sh
cairn init
cairn scan
cairn get <node-id> --json
cairn neighbourhood <node-id>
cairn files <node-id>
cairn contract <node-id>
cairn depends <node-id> --transitive
cairn dependents <node-id>
cairn order
cairn lint --json
```

Every Phase 1 command accepts `--file <path>` to select a blueprint file and `--json`
to render the same typed response structs as stable machine-readable JSON.

Phase 1 implements only contract artefacts. A contract is a Markdown file with
frontmatter containing `node: <id>`, and `cairn contract <node-id>` returns the
parsed body. Other artefact pointers such as todos, decisions, research,
reviews, and sources are retained as raw blueprint metadata but are not interpreted
until Phase 2.

`cairn scan` regenerates:

- `map.md` with generated frontmatter, synced nodes, ghost nodes, active
  changes, and findings.
- `.cairn/log.md` with an appended scan event.
- `.cairn/state/interface-hashes.json` with deterministic Rust interface hash
  state.

## Reference

- `docs/spec.md` — Cairn v0.7 specification
- `docs/blueprint.md` — blueprint grammar reference
- `test/fixtures/cairn.blueprint` — example blueprint file
- `meta/campaigns/rust-full-spec.md` — implementation roadmap
