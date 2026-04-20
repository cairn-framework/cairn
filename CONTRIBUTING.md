# Contributing to Cairn

Cairn is a Rust workspace. This file covers the local development loop: building, running the quality gates, installing the Git hook, and understanding what the current kernel does and does not interpret. For high-level context and terminology state, see `CLAUDE.md` and `docs/spec.md`.

## Local setup

```sh
git clone https://github.com/George-RD/cairn.git
cd cairn
cargo build --release
scripts/install-pre-commit-hook.sh
```

`install-pre-commit-hook.sh` recreates `.git/hooks/pre-commit`, which is not checked in. The hook runs `cargo fmt --check` plus `cairn hook all` before every commit.

## Quality gates

Run the full suite before pushing:

```sh
make check
```

`make check` runs `cargo fmt --check`, `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features`, `cargo test`, and `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps`.

The Conflux archive gate for this repo is `scripts/pre-archive-rust-gates.sh`. It enforces formatting, strict Clippy, and tests before a change is archived. Conflux (cflx) drives the apply, accept, and archive lifecycle; see `CLAUDE.md` for repo-level workflow notes and `AGENTS.md` for codex-agent conventions.

## Kernel commands

The kernel parses `cairn.blueprint`, builds a queryable map graph, loads contract markdown artefacts, reconciles Rust source files against declared module paths, and exposes the first CLI query surface. Common commands:

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
cairn hook structural
cairn hook interface
cairn hook tension
cairn hook all --json
cairn ui
```

Every kernel command accepts `--file <path>` to select a blueprint file and `--json` to render the same typed response structs as stable machine-readable JSON.

Phase 1 implements contract artefacts only. A contract is a Markdown file with frontmatter containing `node: <id>`, and `cairn contract <node-id>` returns the parsed body. Other artefact pointers (todos, decisions, research, reviews, sources) are retained as raw blueprint metadata but are not interpreted until later phases.

`cairn scan` regenerates:

- `map.md` with generated frontmatter, synced nodes, ghost nodes, active changes, and findings.
- `.cairn/log.md` with an appended scan event.
- `.cairn/state/interface-hashes.json` with deterministic Rust interface hash state.

## Hooks

Hooks enforce the integrity classes from `docs/spec.md`:

- `cairn hook structural` exits `1` when structural errors or active-change conflicts exist.
- `cairn hook interface` exits `1` when the current interface hash differs from `.cairn/state/interface-hashes.json`.
- `cairn hook tension` prints advisory findings and always exits `0`.
- `cairn hook all` runs all classes. Structural and interface failures determine the exit code; tensions do not fail the hook.

Every hook accepts `--json`, `--file <path>`, and `--changes-dir <path>`. Use `scripts/cairn-hook-all.sh` from Git hooks or agent task-end hooks so the same engine runs in every boundary.

## Design system

All UI work grounds on `docs/design-system/`: tokens, fonts, components, and a single-page live reference. Colors, spacing, radius, and motion come from `docs/design-system/tokens.css`; nothing hardcodes hex values in components. See `docs/design-system/README.md` for consumption patterns for the marketing site, the embedded Rust web UI, and any future surface.

## Landing page

The marketing landing lives at `docs/landing/index.html`. It is static HTML consuming the design system. Deployment is wired through the GitHub Actions Pages workflow at `.github/workflows/pages.yml`. Once Pages is enabled in repo Settings, the site is live at `https://george-rd.github.io/cairn/`.

## Further reading

- `docs/spec.md`: Cairn v0.7 specification
- `docs/blueprint.md`: blueprint grammar reference
- `docs/design-system/README.md`: design system consumption patterns
- `test/fixtures/cairn.blueprint`: example blueprint file
- `AGENTS.md`: agent-facing conventions for working in this repo
- `CLAUDE.md`: repo-level notes, terminology state, and workflow conventions
