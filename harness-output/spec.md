# Spec — Cairn Graph Explorer UI

## Product

A single-page browser UI served by a Rust binary (`cairn ui`) for navigating Cairn architecture graphs. The UI consumes a JSON API from the same origin and renders nodes, edges, artefacts (decisions, TODOs, research notes, reviews), and lint findings derived from a `cairn.dsl` project file.

This is an **in-progress refresh**, not a greenfield build. The backend and feature surface are fixed. The implementation files are `src/ui_assets/{index.html,style.css,app.js}` which are embedded into the Rust binary at compile time.

## Audience

Software architects and senior engineers who maintain architecture decisions as code. They are comfortable in a terminal, read code daily, and use tools like Linear, Stripe Docs, and dev-oriented GitHub. They value information density over whitespace-as-decoration, and precise typography over loud color.

## Features (fixed; do not remove)

1. **Graph canvas** — nodes and edges rendered in the left region; zoom (`+/-/reset`) and pan; keyboard-focusable.
2. **Node selection** — clicking a node opens the right detail panel.
3. **Detail panel** — shows node kind, name, ID, description, and a finding list when lint issues touch the node.
4. **Layer navigation** — `Back` / `Next` plus a `N / M` counter to traverse neighbourhood layers.
5. **Artefact panels** — sections for decisions, todos, research, reviews, and changes, each with filter toggles (`--include-todos`, `--include-research`, `--include-reviews`, `--include-deprecated-decisions`, `--include-changes` style).
6. **Lint / finding badges** — per-node indicators plus a top-level list of findings with severity.
7. **Schema / meta footer** — schema version, project name, generated timestamp.
8. **Large-graph performance** — one fixture contains 200+ nodes and must stay interactive.

## API contract (read-only, do not change)

- `GET /` — this HTML shell.
- `GET /assets/style.css` — stylesheet.
- `GET /assets/app.js` — client JS.
- `GET /api/meta` — `{ schema_version, name, generated_at, ... }`.
- `GET /api/graph` — `{ nodes: [...], edges: [...] }`. Nodes have `id`, `kind`, `name`, `description`, `tags`, `parent`, `children`, `paths`, `contracts`, `state`, `files`. Edges have `kind` (`ownership` | `dependency`) and endpoints.
- `GET /api/node/<id>` — node detail.
- `GET /api/neighbourhood/<id>` — node + expanded layer with artefact filters as query params.
- `GET /api/lint` — structural findings with severities.

## Technical constraints

- Vanilla HTML + CSS + JS only. No build step, no framework, no bundler. Files must remain importable as raw strings via Rust `include_str!`.
- No external network fetches (no Google Fonts CDN, no CDN JS). System font stack is fine; self-hosted woff2 is allowed if trivial but the current build doesn't ship fonts so prefer system stack to avoid adding assets.
- No npm, no node. No build tooling.
- Graph rendering is currently canvas/SVG based in `app.js` — preserve whatever technique is already there; do not swap rendering engines.

## Aesthetic direction

**Creative tension: editorial precision WITH structural warmth.**

A single label ("brutalist", "minimal", "dev-tool dark mode") would land on template output. The tension: this is a serious analytical tool — it needs the precision and density of a technical reference (think Stripe Docs, Rauno Freiberg's site, Linear's graph views, the SerenityOS man pages) — BUT it is for a human reading about *their own system*, so the surface should feel inhabited, not clinical. Warmth comes from deliberate typography, restrained but confident color, and spacing that breathes without becoming a dashboard.

Concrete anchors:
- Typography: monospaced for all IDs, paths, file references, contract names. A well-tuned sans (system-ui / Inter-like fallback) for prose. Clear type scale. No Helvetica or Arial defaults.
- Color: neutral base (warm off-white or deep slate), one signal color for selection / active, severity colors that look restrained (no hazard-tape red, no Bootstrap warning yellow).
- Negative space: generous around headers, compact within artefact lists. Clear hierarchy between canvas, panel, and artefacts.
- No gradients as decoration. No drop shadows as decoration. Border, rhythm, and type weight do the work.
- Dark mode ready via CSS tokens (preferred starting theme: light, with `@media (prefers-color-scheme: dark)` override).

## Anti-patterns (explicitly reject)

- Generic SaaS dashboard look (rounded cards, soft shadows, pastel palette, system blue CTA).
- Bootstrap / Material / shadcn default aesthetic.
- Full-width hero with centered marketing copy (this is a tool, not a landing page).
- Over-animated interactions — a subtle transition is fine; bouncy springs are not.
- Emoji icons or emoji-as-UI.
- Single giant sidebar that pushes the graph to 60% width; the graph is the subject.
