# Cairn: Agent Orientation

Cairn is a graph-based architecture map for codebases. It models systems, containers, modules, and actors as nodes connected by dependency edges: a navigable structural graph. Each node has depth (code targets, contracts, artefacts like decisions/todos/research) and temporal history (changes, archive trail, decision lineage). The graph is the source of truth for what exists, how it connects, and why it's shaped that way.

## Your task context

Your change directory (`openspec/changes/<change-id>/`) contains everything you need: `proposal.md` (why), `design.md` (how), `tasks.md` (what), and `specs/` (acceptance criteria). Work from these files. The `apply_prompt` in `.cflx.jsonc` gives you the build/lint requirements.

## Where things live

| Path | What |
|---|---|
| `openspec/conventions.md` | Rust code conventions (error codes, module size, state versioning, testing, docs). Authoritative; do not duplicate. |
| `openspec/registries/` | `declared-items.md`, `error-codes.md`. Check when adding new public items or error codes to avoid collisions. |
| `openspec/changes/<other-phase>/specs/` | Other phases' acceptance criteria. Check only if your design.md references them. |
| `docs/spec.md` | Canonical Cairn spec. |
| `docs/design-system/` | Canonical design tokens, components, and live reference for any UI work. |

## Check if relevant, don't read by default

- **Conventions**: `openspec/conventions.md` covers cross-cutting rules (error codes, naming, module limits). Check when making structural or naming decisions.
- **Registries**: `openspec/registries/` covers declared items and error codes across all phases. Check when adding new public items or error codes to avoid collisions.
- **Specs from other phases**: `openspec/changes/<other-phase>/specs/` is only relevant if your design.md references another phase's requirements.

When implementing a feature phase, check `openspec/conventions.md` for the test-first pre-phase convention. If a paired `phase-<N>.0-tests` change exists, remove the matching `#[cflx_planned(phase = <N>)]` attribute as the feature lands rather than rewriting those tests from scratch. The attribute is structured (proc-macro), not a comment; do not parse the `#[ignore]` reason string.

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
- All Rust code must pass the gates in `.cflx.jsonc` `apply_prompt` before marking a task complete.
- No `unsafe` code unless your phase design document justifies it.
- No `#[allow(...)]` without a `// Reason:` comment.
