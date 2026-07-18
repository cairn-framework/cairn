# Cairn: Agent Orientation

Cairn is a graph-based architecture map for codebases. It models systems, containers, modules, and actors as nodes connected by dependency edges: a navigable structural graph. Each node has depth (code targets, contracts, artefacts like decisions/todos/research) and temporal history (changes, archive trail, decision lineage). The graph is the source of truth for what exists, how it connects, and why it's shaped that way.

Two chains meet at a hinge: the **provenance chain** (evidence flowing in: Source → Research → Decision) and the **authority chain** (rules flowing out: Decision → Blueprint → Contract → Code). The Decision carries obligations in both directions. Do not describe the architecture as a flat stack of layers; the two-chain topology is load-bearing, not decorative.

## Your task context

Your change directory (`meta/changes/<change-id>/`) contains everything you need: `proposal.md` (why), `design.md` (how), `tasks.md` (what), and `specs/` (acceptance criteria). Work from these files. The quality gates in `scripts/pre-archive-rust-gates.sh` gives you the build/lint requirements.

Two skill files are worth loading for any coding work in this repo, whether through a Skill tool or by reading them directly: `.claude/skills/karpathy-guidelines/SKILL.md` (think before coding, simplicity first, surgical changes, goal-driven execution) for any coding work, and `.claude/skills/cairn-dev/SKILL.md` (full cairn CLI surface, blueprint syntax, artefact schemas, development loop) for architecture navigation or artefact authoring.

## Where things live

| Path | What |
|---|---|
| `docs/conventions.md` | Rust code conventions (error codes, module size, state versioning, testing, docs). Authoritative; do not duplicate. |
| `docs/registries/` | `declared-items.md`, `error-codes.md`. Check when adding new public items or error codes to avoid collisions. |
| `archive/openspec/changes-archive/<other-phase>/specs/` | Other phases' acceptance criteria. Check only if your design.md references them. |
| `archive/openspec/specs/<area>/spec.md` | Consolidated per-area specs, distinct from the per-phase acceptance criteria above. |
| `docs/spec.md` | Canonical Cairn spec. |
| `docs/design-system/` | Canonical design tokens, components, and live reference for any UI work. |
| `docs/` | Marketing landing page (GitHub Pages target); pulls from the design system like any UI surface. |
| `cairn.blueprint` | Root blueprint: cairn describing itself (dogfood). The graph's source of truth. |
| `test/fixtures/cairn-bootstrap/` | Bootstrap fixture for tests; may lag behind the root blueprint, only smoke-parsed. |

## Check if relevant, don't read by default

- **Conventions**: `docs/conventions.md` covers cross-cutting rules (error codes, naming, module limits). Check when making structural or naming decisions.
- **Registries**: `docs/registries/` covers declared items and error codes across all phases. Check when adding new public items or error codes to avoid collisions.
- **Specs from other phases**: `archive/openspec/changes-archive/<other-phase>/specs/` is only relevant if your design.md references another phase's requirements.

When implementing a feature phase, check `docs/conventions.md` for the test-first pre-phase convention. If a paired `phase-<N>.0-tests` change exists, remove the matching `#[cairn_planned(phase = <N>)]` attribute as the feature lands rather than rewriting those tests from scratch. The attribute is structured (proc-macro), not a comment; do not parse the `#[ignore]` reason string.

## Terminology

CAIRN spec is v0.8. The phase 2.6 terminology rename is applied and archived (merge commit `3f15946`); use `blueprint`/`.blueprint` (not `DSL`/`.dsl`) and `map`/`map.md` (not `ontology`/`index.md`) in all new prose, code identifiers, and spec drafts. If you see `DSL` or `.dsl` string literals in `src/cli/mod.rs` or `src/blueprint/parser.rs`, that is intentional legacy-file detection with a migration warning; do not "fix" it.

Everything else is kept deliberately; do not propose flattening this taxonomy, it encodes distinctions the framework depends on:

- `reconciler` (pluggable interface), `scanner` (engine), `scan` (verb/CLI): three distinct concepts.
- `artefact`: typed-schema kernel primitive (umbrella kept; direct types are contract, decision, todo, research, review, source).
- `rationale tension`: advisory non-blocking finding class, distinct from `interface contradiction` (blocking).
- `change` / `changes/`: carries delta semantics (ADDED/MODIFIED/REMOVED/RENAMED); `proposal.md` lives inside it.
- `neighbourhood`: graph-theoretic query primitive.
- `provenance chain` / `authority chain`: spec §3 spine (see above).
- `interface hash`, `ghost`/`synced`/`orphaned`, `drift`, `divergence`, `verified`/`external`/`unverified`, `hinge`: all kept.

## Project state and artefacts

For project status, outstanding work, or the reasoning behind a decision, **query
cairn directly**. Do not infer state from markdown files, strongholds, or memory;
the graph is the source of truth.

```bash
cairn status              # project summary: nodes, findings, backlog. Start here.
cairn context              # structured project overview; alternate agent entry point.
cairn change list          # active change proposals.
cairn frontier             # buildable-now ghost nodes vs blocked, tiered by dependency depth.
cairn get <id>             # inspect a module, e.g. cairn get cairn.kernel.scanner.
cairn neighbourhood <id>   # a module's dependencies and dependents.
cairn decisions <node>     # provenance chain for a node.
cairn research <node>      # research linked to a node.
cairn sources <node>       # external material a node cites.
cairn scan                 # check for orphaned files or drift; run before committing.
cairn scan --strict        # CI/agent verification gate; exits non-zero on Error or Warning findings.
cairn lint --json          # structured findings output for scripts or agents.
cairn onboard               # group orphaned files by directory with ignore/node suggestions.
cairn feedback "<msg>"      # record friction in .cairn/feedback.md for triage into native todos (via `cairn todo new`) or upstream issues.
cairn ui --port 3000        # browse the graph in a browser (human use).
```

Module IDs follow dotted notation rooted at `cairn` (e.g. `cairn.kernel.map`, `cairn.reconcile`, `cairn.ui`); run `cairn get <id>` to verify a node exists, or open `cairn.blueprint` for the full list.

When adding new source files or directories, check whether they fall under an existing module's `path` declaration in `cairn.blueprint`; if not, add them to an existing module or declare a new one. A clean `cairn scan` (zero findings) is the target state.

If asked "what's next", start with `cairn status` and `cairn change list`, then
`cairn frontier` for buildable-now ghost nodes, then your issue tracker. Any
file under `docs/` or `archive/` is secondary context, never current state.

When **creating** a decision, research finding, or source, place it in `meta/` following
the convention in docs/conventions.md section 10 ("Artefact organization and provenance
links"):

- `meta/decisions/<slug>.md` (id `dec.<slug>`): requires `id`, `nodes:`, `status`, `date`.
  Chain to evidence via `informed_by: [res.X, src.Y]`.
- `meta/research/<slug>.md` (id `res.<slug>`): requires `id`, `nodes:`. Cite sources via
  `sources: [src.Z]`.
- `meta/sources/<slug>.md` (id `src.<slug>`): requires `id`, `file:`, `verification:`. No
  `nodes:` field; anchors transitively through citations.

Filenames are slug-only; the typed prefix lives only in the `id:` frontmatter
(docs/conventions.md section 10). Files are FLAT (no subfolders). Use slug
namespacing for grouping (id `res.gas-city.analysis`, filename
`gas-city.analysis.md`, not `research/gas-city/analysis.md`).

## UI and visual work: use the design system

Any UI change (the webui at `src/ui_assets/`, any landing or marketing page, any new surface) pulls from the canonical design system at `docs/design-system/`. Do not re-invent styling.

- **Tokens are authoritative.** Colors, type, spacing, radius, shadow, and motion come from `docs/design-system/tokens.css`. Do not hardcode hex values or rem values in components, pages, or stylesheets.
- **Reuse components before inventing.** Classes defined in `docs/design-system/components.css` must be reused by class name before a new component is introduced. If something close already exists, extend it rather than parallel-building.
- **Font authority.** Source Serif 4 for headings and long-form copy, IBM Plex Mono for code and technical vocabulary, IBM Plex Sans for UI chrome. All three are wired up in `docs/design-system/fonts.css`.
- **When adding a new token or component**, update `docs/design-system/tokens.css` or `docs/design-system/components.css`, update the live reference at `docs/design-system/index.html`, and note the addition in `docs/design-system/README.md`. All four move in the same commit.
- **Live reference.** Open `docs/design-system/index.html` directly in a browser. It is the source of truth for visual output; if the page does not render as intended, the system is wrong, not the page. See `docs/design-system/README.md` for consumption patterns (marketing via `<link>`, Rust webui via `include_str!`).
- **Em-dashes are banned in user-facing copy.** Replace with a period, colon, comma, or parenthesis as context dictates. This applies to UI strings, marketing copy, and any prose that reaches a reader. Full guidance: `docs/agent/voice.md`.

## Guardrails

- Implement only what your tasks.md specifies. Do not add features from other phases.
- Do not modify files outside your change scope unless your design.md explicitly requires it.
- If a task is ambiguous, prefer the simpler interpretation. Check `proposal.md` and `design.md` before guessing.
- All Rust code must pass the gates in `scripts/pre-archive-rust-gates.sh` `apply_prompt` before marking a task complete.
- No `unsafe` code unless your phase design document justifies it.
- No `#[allow(...)]` without a `// Reason:` comment.
- Never bypass hooks: `git commit --no-verify`, `git push --no-verify`, and the `SKIP=hookid` env var are forbidden. If a hook fails, fix the underlying issue.
- Archived phases under `archive/openspec/changes-archive/` are historical record; do not rewrite them.
- No em-dashes in any prose in this repository (docs, decisions, code comments, commit messages included); the pre-commit hook enforces this across all `*.md` files (`archive/`, `docs/research/`, and `graphify-out/` excepted). Replace with a period, colon, comma, or parenthesis.

See `docs/agent/principles.md` for the positive-form counterpart to these guardrails: typed artefacts encode obligations, authoring is template-driven and tag-extensible, and AI assists authoring but never substitutes for the reconciler's deterministic enforcement.

## Pre-submit review: mandatory

Before submitting any PR, run a self-review simplification pass (remove dead code, fix naming) followed by an adversarial review pass (catches bugs, logic errors, convention violations); in Claude Code these are `/reforge` then `/debate` (or `/palantir-debate`), in other harnesses run the equivalent read-only reviewer subagents in sequence. Fix anything surfaced before submitting. This applies to every PR in a stack, not just the top; skip only for a single-line documentation change.

When asked for a `/debate`, or a sign-off question merits one, structure the response as three self-contained paragraphs: **For** (steel-man the strongest argument in favour), **Against** (steel-man the strongest counter-argument), **Verdict** (the decision, and why it outweighs the opposing view, ending on a forced decision line, not a hedge).

## Task tracking: native Todo artefacts are the front door

This repo's own development uses cairn's native Todo artefact (`docs/spec.md`
§8.2, "Todo (authority)"), the same mechanism a fresh `cairn init` user gets
(`dec.native-todos-first`). Add work with `cairn todo new <slug> --node <id>`,
which scaffolds `meta/todos/todo.<slug>.md`; there is no separate claim/close
verb, status changes (`open`, `in_progress`, `done`, `blocked`) go through the
sanctioned write verb `cairn todo set <slug> <status>` (surgical frontmatter
edit; `dec.todo-write-surface`), not ad-hoc file edits. Inspect with `cairn
todos <node>` or `cairn status`.

## Developing cairn itself: the dev loop

To develop cairn itself, run the Cairn Dev Loop via `/cairn-loop`
(`.claude/commands/cairn-loop.md` plus the skills it loads). That command
(plus the skills it loads) is the sole normative orchestrator: one unit per
session, fail-closed recovery, single squash commit (`dec.loop-command-harness-model`).
A short descriptive overview lives in `docs/agent/cairn-dev-workflow.md` and
is never normative; where the two disagree, the command wins.
