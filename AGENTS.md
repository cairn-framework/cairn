# Cairn: Agent Orientation

Cairn is a graph-based architecture map for codebases. It models systems, containers, modules, and actors as nodes connected by dependency edges: a navigable structural graph. Each node has depth (code targets, contracts, artefacts like decisions/todos/research) and temporal history (changes, archive trail, decision lineage). The graph is the source of truth for what exists, how it connects, and why it's shaped that way.

## Your task context

Your change directory (`meta/changes/<change-id>/`) contains everything you need: `proposal.md` (why), `design.md` (how), `tasks.md` (what), and `specs/` (acceptance criteria). Work from these files. The quality gates in `scripts/pre-archive-rust-gates.sh` gives you the build/lint requirements.

## Where things live

| Path | What |
|---|---|
| `docs/conventions.md` | Rust code conventions (error codes, module size, state versioning, testing, docs). Authoritative; do not duplicate. |
| `docs/registries/` | `declared-items.md`, `error-codes.md`. Check when adding new public items or error codes to avoid collisions. |
| `archive/openspec/changes-archive/<other-phase>/specs/` | Other phases' acceptance criteria. Check only if your design.md references them. |
| `docs/spec.md` | Canonical Cairn spec. |
| `docs/design-system/` | Canonical design tokens, components, and live reference for any UI work. |

## Check if relevant, don't read by default

- **Conventions**: `docs/conventions.md` covers cross-cutting rules (error codes, naming, module limits). Check when making structural or naming decisions.
- **Registries**: `docs/registries/` covers declared items and error codes across all phases. Check when adding new public items or error codes to avoid collisions.
- **Specs from other phases**: `archive/openspec/changes-archive/<other-phase>/specs/` is only relevant if your design.md references another phase's requirements.

When implementing a feature phase, check `docs/conventions.md` for the test-first pre-phase convention. If a paired `phase-<N>.0-tests` change exists, remove the matching `#[cairn_planned(phase = <N>)]` attribute as the feature lands rather than rewriting those tests from scratch. The attribute is structured (proc-macro), not a comment; do not parse the `#[ignore]` reason string.

## Project state and artefacts

For project status, outstanding work, or the reasoning behind a decision, **query
cairn directly**. Do not infer state from markdown files, strongholds, or memory;
the graph is the source of truth.

```bash
cairn status              # project summary: nodes, findings, backlog. Start here.
cairn change list         # active change proposals.
cairn frontier            # buildable-now ghost nodes vs blocked, tiered by dependency depth.
cairn decisions <node>    # provenance chain for a node.
cairn research <node>     # research linked to a node.
cairn sources <node>      # external material a node cites.
```

If asked "what's next", start with `cairn status` and `cairn change list`, then
`cairn frontier` for buildable-now ghost nodes, then your issue tracker. Any
file under `docs/` or `archive/` is secondary context, never current state.

When **creating** a decision, research finding, or source, place it in `meta/` following
the convention in docs/conventions.md section 10 ("Artefact organization and provenance
links"):

- `meta/decisions/dec.<slug>.md` — requires `id`, `nodes:`, `status`, `date`.
  Chain to evidence via `informed_by: [res.X, src.Y]`.
- `meta/research/res.<slug>.md` — requires `id`, `nodes:`. Cite sources via
  `sources: [src.Z]`.
- `meta/sources/src.<slug>.md` — requires `id`, `file:`, `verification:`. No
  `nodes:` field; anchors transitively through citations.

Files are FLAT (no subfolders). Use slug namespacing for grouping
(`res.gas-city.analysis`, not `research/gas-city/analysis.md`).

## UI and visual work: use the design system

Any UI change (the webui at `src/ui_assets/`, any landing or marketing page, any new surface) pulls from the canonical design system at `docs/design-system/`. Do not re-invent styling.

- **Tokens are authoritative.** Colors, type, spacing, radius, shadow, and motion come from `docs/design-system/tokens.css`. Do not hardcode hex values or rem values in components, pages, or stylesheets.
- **Reuse components before inventing.** Classes defined in `docs/design-system/components.css` must be reused by class name before a new component is introduced. If something close already exists, extend it rather than parallel-building.
- **Font authority.** Source Serif 4 for headings and long-form copy. IBM Plex Mono for code and technical vocabulary. Both are wired up in `docs/design-system/fonts.css`.
- **When adding a new token or component**, update `docs/design-system/tokens.css` or `docs/design-system/components.css`, update the live reference at `docs/design-system/index.html`, and note the addition in `docs/design-system/README.md`. All four move in the same commit.
- **Live reference.** Open `docs/design-system/index.html` directly in a browser. It is the source of truth for visual output; if the page does not render as intended, the system is wrong, not the page.
- **Em-dashes are banned in user-facing copy.** Replace with a period, colon, comma, or parenthesis as context dictates. This applies to UI strings, marketing copy, and any prose that reaches a reader.

## Guardrails

- Implement only what your tasks.md specifies. Do not add features from other phases.
- Do not modify files outside your change scope unless your design.md explicitly requires it.
- If a task is ambiguous, prefer the simpler interpretation. Check `proposal.md` and `design.md` before guessing.
- All Rust code must pass the gates in `scripts/pre-archive-rust-gates.sh` `apply_prompt` before marking a task complete.
- No `unsafe` code unless your phase design document justifies it.
- No `#[allow(...)]` without a `// Reason:` comment.

## Task tracking: native Todo artefacts are the front door

This repo's own development uses cairn's native Todo artefact (`docs/spec.md`
§8.2, "Todo (authority)"), the same mechanism a fresh `cairn init` user gets.
Add work with `cairn todo new <slug> --node <id>`, which scaffolds
`meta/todos/todo.<slug>.md`; there is no separate claim/close verb, status
changes (`open`, `in_progress`, `done`, `blocked`) are plain file edits.
Inspect with `cairn todos <node>` or `cairn status`.

Projects that separately run beads (`bd`) get a read-only, per-node view
(`cairn backlog <node>`, `src/state/backlog.rs`); this repo's own development
no longer uses bd for new work (`dec.native-todos-first`, superseding
`dec.bd-upgrade-plan`'s "this repo tracks work in beads" and firing
`dec.beads-task-layer`'s revisit trigger 3, "cairn adopts a non-beads task
tracker"). Historical bd issues remain readable via `bd show <id>` for
archaeology; do not create new ones for this repo's own work.
