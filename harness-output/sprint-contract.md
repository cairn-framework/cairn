# Sprint Contract — Cairn Graph Explorer UI refresh

## Scope in

1. Visual fixes on `/` at widths 1440 and 390.
2. CSS design tokens in `src/ui_assets/style.css` (custom properties for color, type scale, spacing, radii, shadow/border weights). Tokens must be named so later phases (edges-docstrings, MCP panel, summariser) can extend without reshuffling.
3. Consistent component patterns for: topbar, graph canvas region, detail panel, node card, edge treatment, artefact cards (whatever `/api/node/<id>/<kind>` returns — decisions, contracts, etc. — rendered as stacked expandable cards in `#artefacts`), finding-badge, layer-nav, meta/schema footer if present.
4. Dark mode via `@media (prefers-color-scheme: dark)` using the same tokens.
5. Fix graph canvas overflow: current layout puts 9 modules in a single 2118px row that clips past the 1097px region. Need either auto fit-to-width on load, or functional pan/zoom, or a responsive layout pass. Pick one and make it work.
6. Wire the zoom buttons and layer-nav so they visibly change state (the DOM listeners exist but currently appear no-op).
7. Render the artefact cards that `/api/node/<id>/<kind>` returns — the current app.js fetches them but the empty detail panel suggests rendering is incomplete.

## Scope out

- Any change to `src/ui.rs` or other Rust files.
- New API endpoints, new data fields, renamed fields.
- Replacing the graph rendering technique in `app.js`.
- **Inventing new features not already in the Rust API or existing JS surface** — specifically: no new artefact-filter toggles (todos/research/reviews/deprecated/changes), no new panels, no new routes. The original brief listed these as "preserve" but they don't currently exist in the UI; adding them is later-phase work.
- npm / build tooling / external CDN.

## Scope out

- Any change to `src/ui.rs` or other Rust files.
- New API endpoints, new data fields, renamed fields.
- Replacing the graph rendering technique in `app.js`.
- Speculative panels for phases that haven't shipped.
- npm / build tooling / external CDN.

## Acceptance criteria (testable)

1. **Live regression protection** — the release binary still serves ≥3 sequential requests from a real browser (not just in-process loopback) without crashing. The EAGAIN fix in `src/ui.rs` is already in; this run must not break it.
2. **Backend contract preserved** — all `/api/*` fetches use the existing paths and field names.
3. **Feature preservation** — every feature listed in spec.md §Features still works end-to-end: graph loads, a node can be clicked, detail panel opens, layer nav advances, each artefact toggle filters content, lint badges render, zoom buttons work.
4. **No console errors** in the browser console on load, on node click, on layer advance, and after toggling each artefact filter.
5. **Large graph stays interactive** — with ≥200 nodes (matches the existing 200-node integration test), first paint < 1s on localhost, click-to-panel open < 150ms.
6. **Tokens used, not inline values** — search of `style.css` shows zero hardcoded hex colors, px font sizes, or rem spacing outside the `:root` / `[data-theme]` custom property blocks. All consumer rules use `var(--…)`.
7. **Dark mode** — toggling `prefers-color-scheme: dark` at the OS level swaps the palette without any layout shift and without any element becoming unreadable.
8. **Responsive** — at 390px width, the detail panel collapses to full-width overlay or a bottom sheet; the graph canvas remains usable.

## Quality scoring thresholds (drives SHIP decision)

- **Design quality** ≥ 7.0
- **Originality** ≥ 7.0 (no template SaaS aesthetic)
- **Craft** ≥ 7.0 (tokens actually wired, no clipping, no overlap, no console errors)
- **Functionality** ≥ 8.0 (this is a tool — functionality threshold is higher than design)

Weighted average ≥ 7.2 to SHIP.

## Evaluator must verify (adversarial gate)

- No horizontal scroll at any supported viewport.
- No text clipping in artefact panels with long content.
- No overlap between detail panel and graph canvas at 1024–1280 widths.
- Zoom buttons change the graph visibly (not just a no-op).
- Filter toggles change the artefact list (click, count before/after differs).
- Layer nav counter matches rendered content.
- Console clean at load and after a click-through workflow.
