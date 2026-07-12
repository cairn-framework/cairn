---
id: res.webui-review-audit
nodes: [cairn.ui]
date: 2026-07-12
method: primary
---

# Webui review audit (Track B, design-studio Review lane)

## Method

Review lane per design-studio `references/review/polish.md` (audit only, no Studio loop, no code edits). The current webui was served deterministically with the frozen replay server (`startReplayServer`, root `/Users/george/repos/cairn`, fixtures `harness/fixtures`, port 8765) and driven in headless Chromium via Playwright. Surface classified as `interactive` (selectable nodes, command palette, drawer, overlays), so all four lenses were applied: AI slop, hierarchy and rhythm, interaction states, accessibility. Every finding is grounded in a captured screenshot plus DOM or computed-style checks; contrast ratios were computed from live computed colours, not estimated.

Fixture data: 24 nodes, 50 edges, 0 findings. The findings surface was therefore auditable only in its empty state.

## Screenshots

- /tmp/trackb-review/desktop-1440-default.png (default view)
- /tmp/trackb-review/desktop-1440-node-selected.png (cairn.ui selected, inspector open)
- /tmp/trackb-review/desktop-1440-changes-drawer.png (findings drawer open, empty state)
- /tmp/trackb-review/desktop-1440-command-palette.png (query palette open)
- /tmp/trackb-review/desktop-1440-palette-query.png (palette with "ghost" typed)
- /tmp/trackb-review/desktop-1440-blueprint.png (blueprint overlay)
- /tmp/trackb-review/mobile-390-default.png (390 px viewport)

## Findings (ordered by severity)

### Quality

1. **Selection state is nearly invisible on the canvas and there is no scroll-to-selection.** (desktop-1440-node-selected.png; DOM check) Selecting `cairn.ui` changes only the node's border token from `--seam-thin` (1px) to `--seam-carved` (1.5px). In the screenshot the selected node is off-viewport entirely; the zoom reset does not bring it into view, and no class, glow, or tether marks it. The inspector announces the selection but the canvas does not corroborate it. The user must trust the label rather than see the link. Confidence: high.

2. **Canvas node cards are near-identical templates.** (desktop-1440-default.png) Module cards (Artefacts, Blueprint, Changes, CLI...) share the same layout, the same green dot, and the same small footer bar. Scannability is low: nothing besides the label differentiates 22 modules. This is the classic identical-card monotony pattern; the eye reads the column as texture, not information. Confidence: high.

3. **Ambiguous micro-affordances.** (desktop-1440-default.png) The green status dots, the `2 dep / 3 dep / 1 dep` footer bars, and the minimap dots carry no visible explanation. The legend covers synced/planned/orphaned swatches only. Whether footer bars are progress, health, or dependency counts is unguessable from the frame. Confidence: high.

4. **Findings drawer empty state is a dead end.** (desktop-1440-changes-drawer.png) "All nodes reconciled. No drift detected." is reassuring but offers no next action and no freshness signal (no last-reconciled timestamp). The collapse chevron is small and low contrast. With fixture data at 0 findings the populated state could not be audited; that remains an open gap. Confidence: high for empty state, unknown for populated state.

5. **Two competing primary zones and duplicated summary.** (desktop-1440-default.png) The eye lands on the inspector headline "Cairn" and its stat blocks, then ping-pongs to the card stack. The summary "24 nodes, 50 edges, 0 findings" appears in both the topbar and the inspector. One surface should own it. Confidence: medium.

### Polish

6. **Command palette lacks a visible ready-to-type state and a submit affordance.** (desktop-1440-command-palette.png) The `esc` chip is present but there is no `↵` hint and no visible caret in the idle frame; the empty input reads as decoration. The `module · container` separators read as decorative rather than "either/or". Confidence: medium.

7. **Uneven canvas density.** (desktop-1440-default.png) The PROVENANCE column is a dense vertical stack while the flanks under HINGE and AUTHORITY hold large dead zones. Layout is legible but the whitespace is inert rather than rhythmic. Confidence: medium.

8. **Mobile header is cramped and edge labels clip.** (mobile-390-default.png) No horizontal overflow (scrollWidth 390 = clientWidth 390, verified), and cards stack correctly, but the topbar packs `.BLUEPRINT`, the report chip, and the avatar tightly with one control partially occluded, and `owns` edge labels truncate behind cards. Tap chips near the ⌘K hint look under 44 px. Confidence: medium (static frame; targets not measured).

9. **11 px eyebrow capitals are at the floor of comfortable legibility.** (computed styles) `.chain-banner .label` and `.ins-eyebrow` are 11 px amber (#C4864A on #141310, 6.07:1) and muted labels are 12 px (#908673, 5.17:1). Both pass WCAG AA numerically, so this is a comfort nit, not a violation. Earlier per-image reads flagged these as "likely below AA"; the measured ratios supersede that. Confidence: high (measured).

### Notes (not defects)

- **Slop check passes.** No gradients, no glassmorphism, no pure black or white (ground is #141310 warm charcoal), no default sans stack (Source Serif 4 display with monospace ids), single amber accent with semantic green. The identical-card pattern (finding 2) is the only slop-adjacent trait.
- **Accessibility statics are healthy**: measured contrasts pass AA; semantic state colours are used consistently; the a11y static gate already runs in CI.
- **Genuine strength to preserve**: the disciplined warm-dark palette, the serif-plus-mono voice, and the stone/cairn vocabulary ("Click any stone to consult it") give the UI a crafted, opinionated tone that no generic dashboard has.

## Proposed polish direction

Refine, do not redesign. The identity (warm charcoal ground, single amber accent, serif display, geological vocabulary) is an asset and should stay. Spend the polish budget on making state legible at the canvas:

1. Give selection a real presence: stronger carved-seam treatment plus an accent glow or corner mark, and pan or scroll the selected stone into view when selection originates elsewhere (palette, inspector links).
2. Differentiate the module cards: let status, dep weight, or provenance depth modulate the card (footer bar length already exists; give it a labelled meaning and let the dot vary), so the column reads as information rather than texture.
3. Convert dead ends into guidance: findings empty state gains a last-reconciled timestamp and one quiet action; the palette gains an `↵` hint and visible caret.
4. Tidy the mobile topbar (collapse the report chip behind the avatar or overflow menu) and keep edge labels from clipping under cards.

These are all token- and component-level refinements inside the existing system; none require the full geological-metaphor rebuild that Track A explores.

## Zone rubric scores

Scale 1 to 5, higher is better. Grounded in the screenshots and DOM checks above.

| Zone | Hierarchy | Density | Colour discipline | State clarity |
|---|---|---|---|---|
| Header | 4 | 4 | 5 | 3 |
| Graph canvas | 3 | 2 | 4 | 2 |
| Inspector | 5 | 4 | 4 | 4 |
| Command surface | 4 | 4 | 4 | 3 |

Verdict: ready_with_nits. No blockers; the quality findings are selection visibility, card monotony, and affordance ambiguity, all fixable within the current design system.
