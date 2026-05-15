<p align="center">
  <img src="docs/assets/screenshots/landing-hero.png" alt="Cairn landing page hero" width="820">
</p>

<h1 align="center">Cairn</h1>

<p align="center">
  <em>Your agent gets lost in your repo every session. Give it the map.</em><br>
  <strong>The declarative constraint layer for AI-assisted codebases.</strong>
</p>

<!-- badges placeholder: build, crates.io, license once published -->

## Why Cairn

Existing tools either *describe* or *act*. Knowledge graphs describe your codebase but never enforce anything. Coding agents act on your codebase but have no architectural guardrails. Static analysis checks syntax, not intent.

Cairn is the missing constraint layer. You declare architectural truth in a `cairn.blueprint`. Cairn reconciles that declaration against the code you actually shipped, gates commits when they drift, and gives every agent a queryable map grounded in reality, not inference.

| Gap | What exists today | What Cairn adds |
|---|---|---|
| **Knowledge graphs** | Describe structure. No enforcement. | Declares structure *and* gates against drift from it. |
| **Coding agents** | Act on code. No architectural memory. | Persistent map agents query instead of re-scanning. |
| **Static analysis** | Checks syntax and style. | Checks architectural intent: dependencies, contracts, decisions. |

## How it works

```
blueprint  -->  reconcile  -->  gate  -->  query
(declare)      (scan code)    (enforce)   (serve agents)
```

1. **Declare.** Author a `cairn.blueprint` naming your systems, modules, contracts, and the decisions that shaped them.
2. **Reconcile.** `cairn scan` walks the code, computes interface hashes, and flags every node as `synced`, `ghost`, or `orphaned`.
3. **Gate.** Pre-commit hooks block on `interface contradictions` (breaking drift) and surface `rationale tensions` (advisory warnings).
4. **Query.** `cairn get`, `cairn neighbourhood`, `cairn context` return typed JSON so agents ground on structure, not guesswork.

## The Kubernetes analogy

Cairn follows the same pattern as Kubernetes: declare desired state, reconcile continuously, and reject mutations that violate it.

| Cairn | Kubernetes | Role |
|---|---|---|
| `cairn.blueprint` | Manifests (YAML) | Declared desired state |
| Scanner | Controllers | Reconciliation loop |
| Drift gate / hooks | Admission webhooks | Reject invalid mutations |
| Artefact types | CRDs | Typed extensions |
| Map (graph) | etcd | Reconciled state store |
| CLI (`cairn get`, `cairn scan`) | kubectl | Operator interface |
| Reconcilers (tree-sitter, etc.) | Operators | Pluggable domain logic |

## What it does

- Parses a human-authored `cairn.blueprint` into a typed graph (systems, containers, modules, contracts, decisions, research, sources, todos, reviews).
- Reconciles declared nodes against real files on disk and flags `synced`, `ghost`, and `orphaned` state.
- Produces `map.md` with generated frontmatter, active changes, and ranked findings agents can consume.
- Computes deterministic Rust interface hashes and detects contract drift between revisions.
- Surfaces `interface contradictions` (blocking) and `rationale tensions` (advisory) so commits that break the authority chain never land silently.
- Exposes every result as machine-readable JSON so coding agents ground on typed responses, not prose.

## See it

<table>
  <tr>
    <td width="33%" valign="top">
      <a href="docs/landing/index.html"><img src="docs/assets/screenshots/landing-full.png" alt="Cairn landing page"></a>
      <p><strong>Landing page</strong><br>
      <code>docs/landing/index.html</code><br>
      Hosted at <a href="https://george-rd.github.io/cairn/">george-rd.github.io/cairn</a> once GitHub Pages is enabled.</p>
    </td>
    <td width="33%" valign="top">
      <a href="docs/design-system/README.md"><img src="docs/assets/screenshots/design-system.png" alt="Cairn design system showcase"></a>
      <p><strong>Design system</strong><br>
      <code>docs/design-system/</code><br>
      Tokens, fonts, components, and a live reference page every Cairn surface grounds on.</p>
    </td>
    <td width="33%" valign="top">
      <img src="docs/assets/screenshots/webui-graph.png" alt="Cairn Graph Explorer webui">
      <p><strong>Graph Explorer</strong><br>
      <code>cairn ui</code><br>
      Local browser UI for walking the reconciled map. Runs against the current scan.</p>
    </td>
  </tr>
</table>

## Status

Specification complete (v0.7). Phase 1 kernel shipped. Phase 2 and later are ongoing under `openspec/changes/`. See `docs/spec.md` for the full specification.

## Quickstart

```sh
cargo install --git https://github.com/George-RD/cairn.git   # install
cairn init                                                     # create a blueprint
cairn scan                                                     # reconcile against code
```

See [docs/quickstart.md](docs/quickstart.md) for prerequisites, alternative install methods, and a full first-run walkthrough.

## Development

Cairn is a Rust workspace. After cloning, install the local Git format hook:

```sh
scripts/install-pre-commit-hook.sh
```

The hook recreates `.git/hooks/pre-commit`, which is not committed by Git, and runs `cargo fmt --check` plus `cairn hook all` before each commit.

Run the local quality suite before pushing:

```sh
make check
```

`make check` runs `cargo fmt --check`, `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features`, `cargo test`, and `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps`.

The Conflux archive gate for this repository is `scripts/pre-archive-rust-gates.sh`. It enforces formatting, strict Clippy, and tests before a change is archived.

Agent-side conventions live in `AGENTS.md`. For any UI, landing, or visual work, start at `docs/design-system/README.md`.

## Phase 1 Kernel

The Phase 1 kernel parses `cairn.blueprint`, builds a queryable map graph, loads contract Markdown artefacts, reconciles Rust source files against declared module paths, and exposes the first CLI query surface.

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
cairn hook structural
cairn hook interface
cairn hook tension
cairn hook all --json
```

Every Phase 1 command accepts `--file <path>` to select a blueprint file and `--json` to render the same typed response structs as stable machine-readable JSON.

Phase 1 implements only contract artefacts. A contract is a Markdown file with frontmatter containing `node: <id>`, and `cairn contract <node-id>` returns the parsed body. Other artefact pointers (todos, decisions, research, reviews, sources) are retained as raw blueprint metadata but are not interpreted until Phase 2.

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

## Reference

- `docs/spec.md`: Cairn v0.7 specification
- `docs/blueprint.md`: blueprint grammar reference
- `docs/design-system/README.md`: design system consumption patterns
- `test/fixtures/cairn.blueprint`: example blueprint file
- `meta/campaigns/rust-full-spec.md`: implementation roadmap
- `AGENTS.md`: agent-facing conventions for working in this repo
- `CLAUDE.md`: repo-level notes, terminology state, and workflow conventions
