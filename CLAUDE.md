# CAIRN — Claude Code Working Notes

Repo-level context for Claude Code sessions working in this codebase. For the OpenSpec codex agent's instructions, see `AGENTS.md`.

## What CAIRN is

Connective tissue between past decisions, present code structure, and future intent. You author a declarative file describing your system; CAIRN parses it, reconciles against actual code, surfaces the graph + artefacts for coding agents to consume as reliable context, and gates commits when code drifts from the declaration. Extending beyond code to non-code domains (orgs, research, processes) is in-scope for future phases.

The framework's role (spec §3.4): *"a fence around the authority chain and a navigator for the provenance chain."*

## Where things live

| Path | What |
|---|---|
| `docs/spec.md` | Canonical spec — read this first for any architecture question |
| `openspec/changes/<phase>/` | Active phase proposals (proposal.md + design.md + tasks.md + specs/) |
| `openspec/changes/archive/` | Archived phases (historical record; do NOT rewrite) |
| `openspec/specs/<area>/spec.md` | Consolidated per-area specs |
| `openspec/conventions.md` | Rust code conventions (error codes, module size, state versioning) |
| `openspec/registries/` | `declared-items.md`, `error-codes.md` |
| `src/` | Rust implementation (phase 1+ kernel and onwards) |
| `test/fixtures/cairn-bootstrap/` | Bootstrap fixture — CAIRN describing itself |
| `AGENTS.md` | Instructions read by the codex agent during cflx runs |

## Architecture — two chains meeting at a hinge

CAIRN models **two chains**, not a flat six-layer stack:

- **Provenance chain** (evidence flowing in): Source → Research → Decision
- **Authority chain** (rules flowing out): Decision → DSL → Contract → Code
- **Hinge:** the Decision — obligations in both directions

Describing CAIRN as "six layers" flattens the topology and loses the distinction between evidence and norms. v0.5 explicitly rejected the flat framing.

## Terminology state (as of 2026-04-20)

CAIRN is under a spec v0.6 → v0.7 terminology rename. **Phase 2.6 is committed as a proposal but NOT yet applied.**

| Old (v0.6 — current codebase) | New (v0.7 — after phase 2.6 runs) |
|---|---|
| `DSL` / `.dsl` | `blueprint` / `.blueprint` |
| `ontology` | `map` |
| `index.md` (generated snapshot) | `map.md` |

**Everything else in v0.6 is kept deliberately.** Do NOT propose flattening the taxonomy — it encodes distinctions the framework depends on. Specifically:

- `reconciler` (pluggable interface), `scanner` (engine), `scan` (verb/CLI) — three distinct concepts
- `artefact` — typed-schema kernel primitive (umbrella kept; direct types are contract, decision, todo, research, review, source)
- `rationale tension` — advisory non-blocking finding class, distinct from `interface contradiction` (blocking)
- `change` / `changes/` — carries delta semantics (ADDED/MODIFIED/REMOVED/RENAMED); `proposal.md` lives inside it
- `neighbourhood` — graph-theoretic query primitive
- `provenance chain` / `authority chain` — spec §3 spine
- `interface hash`, `ghost`/`synced`/`orphaned`, `drift`, `divergence`, `verified`/`external`/`unverified`, `hinge` — kept

When working in this repo: if you see `DSL` or `ontology` in code, docs, or phase files, that's the pre-phase-2.6 state — it's not an error. Do NOT rename ahead of phase 2.6. The full plan is in `openspec/changes/phase-2.6-terminology-rename/`.

## Voice and audience

CAIRN's audience is broadening from career developers to "people building with AI tools," including non-devs. User-facing vocabulary should prefer plain concise English. But accuracy is the floor: do not flatten load-bearing technical taxonomy (see above). The bar is *"would a non-dev feel nervous typing this command or reading this doc?"* — not *"what's the simplest possible word."*

## Workflow — cflx (Conflux)

Phases execute via `cflx` (Conflux). Lifecycle: **apply → accept → archive**. cflx runs in a worktree with a codex-driven implementation. Verification happens before merge; intermediate broken states on a feature branch are acceptable. When a phase specifies atomic-commit groupings (e.g., phase 2.6 task 2.1–2.5 + 3.1 together), the codex agent enforces that boundary — no special action needed from the orchestrator.

Pre-commit hook runs `cargo fmt --check`. Verification gate battery for every phase: `cargo build` (zero warnings), `cargo clippy --all-targets --all-features` with `-D warnings`, `cargo fmt --check`, `cargo test`, `cargo test --locked`, plus `cflx.py validate <phase> --strict`.

## What to avoid

- Rewriting archived phases under `openspec/changes/archive/` — they are historical record.
- Inventing new terminology for concepts already named in spec v0.6 beyond the three approved renames.
- Describing the architecture as a flat six-layer stack.
- Renaming ahead of phase 2.6; let the rename phase land first.
- Calling `cflx` "cairn" — they're different tools; cflx is the workflow runner, cairn is the framework.

## Further reading

- `docs/spec.md` — canonical spec
- `openspec/changes/phase-2.6-terminology-rename/` — terminology rename plan and full rationale
- Archived phases for prior-art patterns: `openspec/changes/archive/phase-2-artefacts/`, `openspec/changes/archive/phase-2.5-graph-explorer/`
