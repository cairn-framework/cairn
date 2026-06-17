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

## Quickstart

```sh
cargo install --git https://github.com/cairn-framework/cairn.git   # install
cairn init                                                          # scaffold blueprint + config + agent guide
cairn scan                                                          # reconcile against code
```

Onboarding an existing codebase? `cairn init --from-code` discovers modules from your source tree and writes a reviewable proposal instead of a starter blueprint. `cairn onboard` then groups any leftover orphaned files with suggestions.

See [docs/quickstart.md](docs/quickstart.md) for prerequisites, alternative install methods, and a full first-run walkthrough. The blueprint grammar is in [docs/blueprint.md](docs/blueprint.md), and the command reference in [docs/commands.md](docs/commands.md).

## Using Cairn with coding agents

Cairn is built to be an agent's source of architectural truth, in both directions:

- **Guidance in.** `cairn init` writes `.cairn/AGENTS.md`, a ready-made section for your project's `CLAUDE.md` or `AGENTS.md` that teaches agents the orientation commands (`cairn context`, `cairn get`, `cairn neighbourhood`), the keep-the-blueprint-in-sync rule, and the pre-commit gate.
- **Typed answers out.** Every command takes `--json` and returns a stable envelope (`{"command", "status", "data"}`), so agents parse structure instead of prose. `cairn-mcp` exposes the same query API as MCP tools (see [docs/mcp.md](docs/mcp.md) and [docs/claude-code.md](docs/claude-code.md)).
- **Friction back upstream.** When Cairn itself misbehaves on your project (a confusing message, a wrong finding, a missing capability), `cairn feedback "<what happened>"` records it in `.cairn/feedback.md` and prints a prefilled issue link for [this repo's tracker](https://github.com/cairn-framework/cairn/issues). The generated agent guide tells agents to do this instead of silently routing around problems, so every adopting project helps dogfood Cairn.

A greenfield pattern that works well: declare your intended modules in the blueprint before any code exists. They show up as `ghost` nodes, agents treat them as a to-do list, and `cairn scan` confirms each one as it becomes real code.

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
- Reconciles declared nodes against real files on disk and flags `synced`, `ghost`, and `orphaned` state. The code reconciler speaks Rust, TypeScript, Python, and Go via tree-sitter.
- Produces `map.md` with generated frontmatter, active changes, and ranked findings agents can consume.
- Computes deterministic interface hashes and detects contract drift between revisions.
- Surfaces `interface contradictions` (blocking) and `rationale tensions` (advisory) so commits that break the authority chain never land silently.
- Tracks structured changes (`meta/changes/`) with delta semantics and an acceptance gate (`cairn accept`).
- Onboards brownfield codebases: `cairn init --from-code` extraction, `cairn refine` re-discovery, `cairn onboard` orphan triage, `cairn islands` for disconnected components.
- Exposes every result as machine-readable JSON, as MCP tools (`cairn-mcp`), and in a local web explorer (`cairn ui`).

## Commands at a glance

| Need | Command |
|---|---|
| Orientation for a session | `cairn context` |
| Reconcile blueprint against code | `cairn scan` (`--strict` for CI) |
| Inspect a node / its surroundings | `cairn get <id>`, `cairn neighbourhood <id>` |
| Dependency questions | `cairn depends <id>`, `cairn dependents <id>`, `cairn order`, `cairn islands` |
| Provenance questions | `cairn rationale <id>`, `cairn decisions <id>`, `cairn research <id>`, `cairn sources <id>` |
| Findings | `cairn lint`, `cairn check` |
| Commit gates | `cairn hook structural\|interface\|tension\|all` |
| Changes | `cairn change new <id>`, `cairn changes`, `cairn show <id>`, `cairn accept` |
| Brownfield | `cairn init --from-code`, `cairn refine`, `cairn onboard` |
| Export | `cairn export --format json\|md\|mermaid` |
| Web explorer | `cairn ui --port 3000` |
| Report Cairn friction | `cairn feedback "<message>"` |

Run `cairn --help` for the full list; commands accept `--file <path>` and `--json` (`cairn init` is the exception: it always scaffolds the current directory). Full reference: [docs/commands.md](docs/commands.md).

## Hooks

Hooks enforce the integrity classes from `docs/spec.md`:

- `cairn hook structural` exits `1` when structural errors or active-change conflicts exist.
- `cairn hook interface` exits `1` when the current interface hash differs from `.cairn/state/interface-hashes.json`.
- `cairn hook tension` prints advisory findings and always exits `0`.
- `cairn hook all` runs all classes. Structural and interface failures determine the exit code; tensions do not fail the hook.

Every hook accepts `--json`, `--file <path>`, and `--changes-dir <path>`. Use `scripts/cairn-hook-all.sh` from Git hooks or agent task-end hooks so the same engine runs in every boundary.

## See it

<table>
  <tr>
    <td width="33%" valign="top">
      <a href="docs/landing/index.html"><img src="docs/assets/screenshots/landing-full.png" alt="Cairn landing page"></a>
      <p><strong>Landing page</strong><br>
      <code>docs/landing/index.html</code><br>
      Hosted at <a href="https://cairn-framework.github.io/cairn/">cairn-framework.github.io/cairn</a> once GitHub Pages is enabled.</p>
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

Specification v0.7 ([docs/spec.md](docs/spec.md)). The kernel, artefact registry, change tracking, brownfield onboarding, hooks, MCP server, and web explorer have all shipped; Cairn is not yet on crates.io and the CLI surface may still move. This repository dogfoods Cairn: the root `cairn.blueprint` describes Cairn itself, and the commit gate runs `cairn hook all`.

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

Active change proposals live under `meta/changes/`; archived phases under `archive/openspec/` are historical record. Agent-side conventions live in `AGENTS.md`. For any UI, landing, or visual work, start at `docs/design-system/README.md`.

## Design system

All UI work grounds on `docs/design-system/`: tokens, fonts, components, and a single-page live reference. Colors, spacing, radius, and motion come from `docs/design-system/tokens.css`; nothing hardcodes hex values in components. See `docs/design-system/README.md` for consumption patterns for the marketing site, the embedded Rust web UI, and any future surface.

## Landing page

The marketing landing lives at `docs/landing/index.html`. It is static HTML consuming the design system. Deployment is wired through the GitHub Actions Pages workflow at `.github/workflows/pages.yml`. Once Pages is enabled in repo Settings, the site is live at `https://cairn-framework.github.io/cairn/`.

## Reference

- `docs/spec.md`: Cairn v0.7 specification
- `docs/quickstart.md`: install and first-run walkthrough
- `docs/blueprint.md`: blueprint grammar reference
- `docs/commands.md`: CLI command reference
- `docs/mcp.md` and `docs/claude-code.md`: agent and MCP integration
- `docs/design-system/README.md`: design system consumption patterns
- `test/fixtures/cairn.blueprint`: example blueprint file
- `AGENTS.md`: agent-facing conventions for working in this repo
- `CLAUDE.md`: repo-level notes, terminology state, and workflow conventions
