# CAIRN: Claude Code Working Notes

Repo-level context for Claude Code sessions working in this codebase. For the OpenSpec codex agent's instructions, see `AGENTS.md`.

## What CAIRN is

Connective tissue between past decisions, present code structure, and future intent. You author a declarative file describing your system; CAIRN parses it, reconciles against actual code, surfaces the graph + artefacts for coding agents to consume as reliable context, and gates commits when code drifts from the declaration. Extending beyond code to non-code domains (orgs, research, processes) is in-scope for future phases.

The framework's role (spec §3.4): *"a fence around the authority chain and a navigator for the provenance chain."*

## Where things live

| Path | What |
|---|---|
| `docs/spec.md` | Canonical spec. Read this first for any architecture question |
| `docs/design-system/` | Canonical design tokens, components, and live reference for any UI work |
| `docs/landing/` | Marketing landing page (GitHub Pages target) |
| `openspec/changes/<phase>/` | Active phase proposals (proposal.md + design.md + tasks.md + specs/) |
| `openspec/changes/archive/` | Archived phases (historical record; do NOT rewrite) |
| `openspec/specs/<area>/spec.md` | Consolidated per-area specs |
| `openspec/conventions.md` | Rust code conventions (error codes, module size, state versioning) |
| `openspec/registries/` | `declared-items.md`, `error-codes.md` |
| `src/` | Rust implementation (phase 1+ kernel and onwards) |
| `src/ui_assets/` | Embedded web UI (styled via `docs/design-system/`) |
| `test/fixtures/cairn-bootstrap/` | Bootstrap fixture: CAIRN describing itself |
| `AGENTS.md` | Instructions read by the codex agent during cflx runs |

## Architecture: two chains meeting at a hinge

CAIRN models **two chains**, not a flat six-layer stack:

- **Provenance chain** (evidence flowing in): Source → Research → Decision
- **Authority chain** (rules flowing out): Decision → Blueprint → Contract → Code
- **Hinge:** the Decision carries obligations in both directions

Describing CAIRN as "six layers" flattens the topology and loses the distinction between evidence and norms. v0.5 explicitly rejected the flat framing.

## Terminology state (as of 2026-04-20)

CAIRN spec is v0.7. **Phase 2.6 terminology rename has been applied and archived** (merge commit `3f15946`).

| Legacy (pre-phase 2.6) | Current (v0.7) |
|---|---|
| `DSL` / `.dsl` | `blueprint` / `.blueprint` |
| `ontology` | `map` |
| `index.md` (generated snapshot) | `map.md` |

If you see `DSL` or `.dsl` string literals in `src/cli/mod.rs` or `src/blueprint/parser.rs`, that is intentional legacy-file detection with a migration warning. Do not "fix" it.

**Everything else in v0.6 is kept deliberately.** Do NOT propose flattening the taxonomy; it encodes distinctions the framework depends on. Specifically:

- `reconciler` (pluggable interface), `scanner` (engine), `scan` (verb/CLI): three distinct concepts
- `artefact`: typed-schema kernel primitive (umbrella kept; direct types are contract, decision, todo, research, review, source)
- `rationale tension`: advisory non-blocking finding class, distinct from `interface contradiction` (blocking)
- `change` / `changes/`: carries delta semantics (ADDED/MODIFIED/REMOVED/RENAMED); `proposal.md` lives inside it
- `neighbourhood`: graph-theoretic query primitive
- `provenance chain` / `authority chain`: spec §3 spine
- `interface hash`, `ghost`/`synced`/`orphaned`, `drift`, `divergence`, `verified`/`external`/`unverified`, `hinge`: all kept

Use `blueprint` / `map` / `map.md` in all new prose, code identifiers, and spec drafts. The archived plan is in `openspec/changes/archive/phase-2.6-terminology-rename/`; consolidated spec at `openspec/specs/terminology-rename/spec.md`.

## Voice and audience

CAIRN's audience is broadening from career developers to "people building with AI tools," including non-devs. User-facing vocabulary should prefer plain concise English. But accuracy is the floor: do not flatten load-bearing technical taxonomy (see above). The bar is *"would a non-dev feel nervous typing this command or reading this doc?"*, not *"what's the simplest possible word."*

Em-dashes are banned in user-facing copy. Replace with period, colon, comma, or parenthesis as context dictates. Applies to spec prose, marketing copy, and this file.

## UI and visual work

All UI changes (webui at `src/ui_assets/`, landing at `docs/landing/`, any new surface) must pull colors, type, spacing, radius, shadow, and motion from `docs/design-system/tokens.css`. Do not hardcode hex or rem values. Reuse classes from `docs/design-system/components.css` before introducing new components. Font authority is Source Serif 4 (headings, long-form) plus IBM Plex Mono (code, technical vocabulary) plus IBM Plex Sans (UI chrome). See `docs/design-system/README.md` for consumption patterns (marketing via `<link>`, Rust webui via `include_str!`). AGENTS.md carries the same directive for codex-driven phases.

## Workflow: cflx (Conflux)

Phases execute via `cflx` (Conflux). Lifecycle: **apply → accept → archive**. cflx runs in a worktree with a codex-driven implementation. Verification happens before merge; intermediate broken states on a feature branch are acceptable. When a phase specifies atomic-commit groupings (e.g., phase 2.6 task 2.1–2.5 + 3.1 together), the codex agent enforces that boundary with no special action needed from the orchestrator.

Pre-commit hook runs `cargo fmt --check`. Verification gate battery for every phase: `cargo build` (zero warnings), `cargo clippy --all-targets --all-features` with `-D warnings`, `cargo fmt --check`, `cargo test`, `cargo test --locked`, plus `cflx.py validate <phase> --strict`.

## Workflow: Graphite (gt)

This repo uses Graphite (the `gt` CLI) for stacked PRs. The graphite-pr skill activates whenever you work in this repo. `gt` owns branch state. Every branch, commit, and push goes through `gt create` / `gt modify` / `gt submit`. Plain `git status`, `git log`, `git diff`, `git add`, `git reset`, `git stash` stay fine. Raw `git commit`, `git push`, `git checkout -b`, `git branch -D` bypass Graphite's metadata and corrupt the stack.

The 90% loop:

```bash
gt sync --no-interactive --force                     # Sync trunk
git add <files-for-this-unit>                        # Stage selectively
gt create -m "<type>(<scope>): <subject>"            # New branch + commit
gt submit --stack --publish --no-interactive         # Publish (auto-review fires)
```

Amend on review: `git add <files>; gt modify -a; gt submit --publish --no-interactive`. New scope on top: `git add <files>; gt create -m "..."`. After submit and review, run `~/.claude/skills/graphite-pr/scripts/gt-merge-cascade.sh` to merge with review-thread gating.

Sizing: one commit equals one logical unit, target under 250 lines added+removed, hard cap 400. See `~/.claude/skills/graphite-pr/SKILL.md` for full rules.

## What to avoid

- Rewriting archived phases under `openspec/changes/archive/`; they are historical record.
- Inventing new terminology for concepts already named in the spec beyond the three applied renames.
- Describing the architecture as a flat six-layer stack.
- Calling `cflx` "cairn"; they're different tools. cflx is the workflow runner, cairn is the framework.
- Em-dashes in any prose in this repository.
- Hardcoded colors, sizes, or fonts in UI code when a design-system token exists.
- Skipping hooks. Forbidden bypass paths include `git commit --no-verify`, `git push --no-verify`, and the `SKIP=hookid` env var. If a hook fails, fix the underlying issue.

## Further reading

- `docs/spec.md`: canonical spec
- `docs/design-system/README.md`: UI token + component reference
- `openspec/changes/archive/phase-2.6-terminology-rename/`: terminology rename plan and full rationale
- Archived phases for prior-art patterns: `openspec/changes/archive/phase-2-artefacts/`, `openspec/changes/archive/phase-2.5-graph-explorer/`

## graphify

This project has a graphify knowledge graph at graphify-out/.

Rules:
- Before answering architecture or codebase questions, read graphify-out/GRAPH_REPORT.md for god nodes and community structure
- If graphify-out/wiki/index.md exists, navigate it instead of reading raw files
- For cross-module "how does X relate to Y" questions, prefer `graphify query "<question>"`, `graphify path "<A>" "<B>"`, or `graphify explain "<concept>"` over grep; these traverse the graph's EXTRACTED + INFERRED edges instead of scanning files
- After modifying code files in this session, run `graphify update .` to keep the graph current (AST-only, no API cost)
