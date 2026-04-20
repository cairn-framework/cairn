# Design Description — Iteration 1, Cairn Graph Explorer (full page)

## 1. Creative Vision Statement

This is a reading room for a software architect's own system, not a dashboard. The defining tension is **editorial precision with structural warmth**: read it as a well-set technical journal that happens to be interactive — think the quiet typographic confidence of Stripe's internal docs, the inhabited density of a good `man` page typeset in print, and the controlled asymmetry of a Swiss architectural review. The surface is built from paper-colored neutrals, ink-colored type, a single ochre signal, and lots of mono. Every identifier is a typographic citation; every edge is a drawn line, not a UI flourish; every panel boundary is a deliberate rule, not a shadow. Warmth comes from the off-white paper ground, the slight warmth baked into both the light and dark palettes, and from generous leading on prose. Precision comes from hairline rules, consistent baseline rhythm, monospace identifiers, and the refusal to round corners or blur edges. This tool should feel like reading an annotated architectural floor plan in a well-lit studio — not like clicking around a SaaS console.

The reader is an architect who already knows their system. The UI's job is to disappear into legibility: show the graph, name the parts, cite the artefacts, and get out of the way. The only ornament allowed is typography.

## 2. Design-Token System

All tokens live under a single namespace. Light is the default theme served on load (matches the spec preference and matches the "paper" mental model). Dark inverts to an ink-over-graphite reading room mood — same tokens, different values, no tonal reshuffling so that the visual vocabulary stays consistent between modes.

### 2.1 Color — Light mode (default on load)

Palette is warm-neutral with a single ochre signal and three severity tones tuned to the same warmth. No pure white, no pure black, no system blue anywhere.

- `cairn-paper` — warm off-white paper ground. Intent: HSL ≈ 40° 25% 97% (roughly `#FAF7F1`). This is the page.
- `cairn-paper-sunk` — very slightly cooler/darker paper for the graph canvas region. Intent: HSL ≈ 38° 18% 95% (roughly `#F4F1EA`). The canvas is inset into the page; this tone makes the inset read.
- `cairn-ink` — near-black body ink, warm. Intent: HSL ≈ 30° 12% 12% (roughly `#211D18`). Used for headings and primary prose.
- `cairn-ink-muted` — secondary prose and meta. Intent: HSL ≈ 30° 8% 36% (roughly `#5E574E`).
- `cairn-ink-faint` — captions, inline meta, timestamp strips. Intent: HSL ≈ 30° 6% 54% (roughly `#8A8278`).
- `cairn-rule` — hairline rule and border color, warm. Intent: HSL ≈ 35° 12% 82% (roughly `#D9D2C6`).
- `cairn-rule-strong` — 1.5–2px structural borders (panel divider, topbar underline). Intent: HSL ≈ 33° 14% 68% (roughly `#B7AC9A`).
- `cairn-accent` — the single signal color. Burnt ochre / Siena. Intent: HSL ≈ 24° 62% 42% (roughly `#B25A1C`). Used for: selected node stroke, focused edge, active layer indicator, and the one-in-a-page "current" affordance. Nothing else.
- `cairn-accent-ink` — an inked variant of the accent for text-on-paper use where the full accent would be too loud. Intent: HSL ≈ 22° 48% 28% (roughly `#6E3A17`).
- `cairn-severity-note` — lowest severity (info / note). Intent: HSL ≈ 200° 22% 40% (a muted slate-blue, roughly `#516C79`).
- `cairn-severity-warn` — medium severity (including `CAIRN_CONTRACT_MISSING`). Intent: HSL ≈ 38° 48% 38% (dry amber-brown, roughly `#8F6C22`). Specifically NOT hazard yellow, NOT Bootstrap warning, NOT amber-pill. This is a pigment, not a stoplight.
- `cairn-severity-err` — highest severity. Intent: HSL ≈ 8° 54% 40% (muted brick, roughly `#9E4234`). Not fire-engine red.
- `cairn-node-system` — container fill for the root/system kind. A parchment-tint, HSL ≈ 45° 30% 92%.
- `cairn-node-container` — container fill for kernel/container-like kinds. A cooler parchment, HSL ≈ 80° 15% 92% (warm sage).
- `cairn-node-module` — leaf module fill. Same as `cairn-paper`, i.e. modules are "on the page." Their identity comes from the border, not the fill.
- `cairn-node-reconciler` — specialisation fill for reconciler-like kinds. HSL ≈ 12° 20% 94% (faint blush).
- `cairn-selection-ink` — the color used for the 2px inset stroke on a selected node. Same value as `cairn-accent`.
- `cairn-focus-ring` — keyboard-focus ring. A 2px solid in `cairn-accent` offset 2px outside the element.

### 2.2 Color — Dark mode

Triggered by `prefers-color-scheme: dark`. Same token names, different values. Same warm bias — this is a dimmed reading room, not a black terminal.

- `cairn-paper` → HSL ≈ 32° 8% 11% (warm graphite, roughly `#1F1C18`).
- `cairn-paper-sunk` → HSL ≈ 30° 9% 8% (deeper, for the canvas inset, roughly `#171410`).
- `cairn-ink` → HSL ≈ 38° 20% 92% (warm bone, roughly `#EDE6D8`).
- `cairn-ink-muted` → HSL ≈ 36° 12% 72% (roughly `#BCB2A0`).
- `cairn-ink-faint` → HSL ≈ 34° 8% 54% (roughly `#8C847A`).
- `cairn-rule` → HSL ≈ 32° 8% 22% (roughly `#3A342D`).
- `cairn-rule-strong` → HSL ≈ 32° 8% 34% (roughly `#574F45`).
- `cairn-accent` → HSL ≈ 28° 72% 62% (warm ochre-orange, roughly `#D89156`). Brighter than the light-mode accent because it sits on darker ground; same family.
- `cairn-accent-ink` → HSL ≈ 28° 54% 78% (roughly `#E8C19A`).
- `cairn-severity-note` → HSL ≈ 200° 28% 66% (roughly `#8DB0C0`).
- `cairn-severity-warn` → HSL ≈ 38° 44% 62% (roughly `#C69E5A`).
- `cairn-severity-err` → HSL ≈ 10° 50% 64% (roughly `#C4735F`).
- Node fills in dark mode are subtle tints on `cairn-paper` — never lighter than `#2A2620`-ish — so that nodes read as debossed panels on graphite, not as bright cards floating on black.

### 2.3 Type

System stack only (spec forbids external fonts). Two families:

- **Sans family (prose, UI chrome, node labels, section headings)** — `ui-sans-serif`, then `system-ui`, then `-apple-system`, then `Segoe UI`. Explicitly not Helvetica, not Arial. On macOS this resolves to SF; on modern Windows to Segoe UI Variable; on Linux to whatever ui-sans resolves. This is acceptable.
- **Mono family (all identifiers, paths, contract names, file references, kinds, schema version, timestamp, and the layer counter)** — `ui-monospace`, then `SFMono-Regular`, then `JetBrains Mono`, then `IBM Plex Mono`, then `Menlo`, then `Consolas`. The mono is load-bearing: it is what makes the UI feel technical-journal instead of SaaS-dashboard. Any identifier rendered in sans is a design bug.

**Type scale** — 8px base, 1.25 modular ratio, but with two deliberate breaks (display step below, and micro step above) so prose has room to breathe at the top and meta has room to live at the bottom.

- `cairn-size-caption` — 11px / 0.6875rem. Used for meta strip timestamps, micro labels.
- `cairn-size-meta` — 12px / 0.75rem. Used for kind eyebrows, finding code labels, schema version, layer counter.
- `cairn-size-body` — 14px / 0.875rem. Default prose, node labels, artefact body.
- `cairn-size-body-lg` — 16px / 1rem. Detail-panel description, artefact summary.
- `cairn-size-h3` — 18px / 1.125rem. Artefact titles in the expandable cards.
- `cairn-size-h2` — 22px / 1.375rem. Detail-panel node name.
- `cairn-size-h1` — 28px / 1.75rem. Product mark in topbar. Explicitly not billboard — this is chrome, not a hero.
- `cairn-size-display` — reserved, unused this iteration.

**Line heights**: prose (body and larger) uses 1.5; meta and caption use 1.35; mono identifiers use 1.4 so they don't crowd vertically. Letter-spacing is default for sans; +0.5% tracking on all-caps eyebrow meta; default for mono.

**Weights**: sans uses 400 for prose, 500 for UI labels and section headings, 600 for the product mark and node names, 700 is never used. Mono uses 400 only. No italic except in one place — the description text in the detail panel is roman; the inline quotation of an external source (if one ever appears) is italic. In this iteration there is no italic in the UI.

### 2.4 Spacing

4px base unit. A 4-8-12-16-24-32-48-64-96 scale. Tokens:

- `cairn-space-0` = 0
- `cairn-space-1` = 4px (tight — between a mono ID and its trailing meta)
- `cairn-space-2` = 8px (default between stacked lines in a meta strip)
- `cairn-space-3` = 12px (between label + field)
- `cairn-space-4` = 16px (default between artefact card rows)
- `cairn-space-5` = 24px (between panel sections, e.g. description to findings)
- `cairn-space-6` = 32px (panel content inset from panel edge on desktop)
- `cairn-space-7` = 48px (between major panel regions, e.g. findings block to artefacts block)
- `cairn-space-8` = 64px (topbar height allowance, footer height allowance on desktop)
- `cairn-space-9` = 96px (reserved; not used this iteration)

Detail-panel horizontal inset is `cairn-space-6` (32px) on desktop and `cairn-space-4` (16px) on mobile. Vertical rhythm inside the panel alternates `cairn-space-5` (between sections) with `cairn-space-3` (between label and value inside a section). Artefact cards have `cairn-space-4` between each, not more — the density is the point.

### 2.5 Radii

Aggressively restrained. The spec rejects rounded cards. Tokens:

- `cairn-radius-0` = 0. Used on: nodes, artefact cards, detail panel, topbar, footer, finding markers.
- `cairn-radius-1` = 2px. Used on: the keyboard focus ring's outer edge softening, input/button hit areas, nothing else.

Nothing in this UI has a `border-radius` greater than 2px. Rounded pills are banned.

### 2.6 Border weights

- `cairn-border-hair` = 1px. Rules between artefact cards, panel dividers, node outlines for unselected state.
- `cairn-border-rule` = 1.5px. The one horizontal rule under the topbar and above the footer.
- `cairn-border-strong` = 2px. Selected node's inset stroke, focus ring, the active layer underline.

Borders use `cairn-rule` by default, `cairn-rule-strong` for structural dividers, `cairn-accent` for selection and focus only.

### 2.7 Shadow policy

**There are no box-shadows anywhere in this UI.** Not on nodes, not on cards, not on the panel, not on hover. Depth is conveyed by `cairn-paper-sunk` (the canvas is visibly inset from the page because it is a slightly cooler tone with a 1px top-and-bottom rule) and by border weight. Any shadow value in the stylesheet is a design defect.

## 3. Topbar (Desktop, 1440)

Replace the eyebrow + big H1 + three outlined pills with a **masthead strip**. The topbar reads left to right as:

**Left block** — the product mark. "Cairn" set in sans, weight 600, 28px. Immediately to its right, on the same baseline, a thin vertical rule (1px, `cairn-rule-strong`, 60% of the cap height tall, not full height) then the sub-mark "Graph Explorer" in sans 500 at 14px, `cairn-ink-muted`. No all-caps eyebrow. This reads as a fixed masthead, not a marketing headline.

**Center block (this is where project meta lives)** — a mono meta strip. One line, three fields separated by a middle-dot character (" · ") in `cairn-ink-faint`:

- project name, mono 12px, `cairn-ink` (e.g. `cairn`)
- schema version, mono 12px, `cairn-ink-muted`, prefixed by the literal text `schema ` in sans 12px `cairn-ink-faint` (so: `schema v0.6`)
- generated timestamp, mono 12px, `cairn-ink-muted`, prefixed by `generated ` in sans 12px `cairn-ink-faint` (relative if ≤ 24h, absolute ISO otherwise)

This center block is horizontally centered in the topbar only at ≥ 1200px; below that it sits left-aligned next to the mark. The meta *lives in the topbar*, not in a footer. There is no separate schema footer.

**Right block** — three canvas controls, but redesigned from pills into a single connected control strip. A horizontal row of three mono glyphs: `−`, `+`, `⌾` (reset), set 16px in mono, 40px hit targets each, separated by 1px `cairn-rule` hairlines, enclosed by a 1px `cairn-rule` outer rule. No background fill. Hover state: the hovered segment fills with `cairn-rule` at 40% opacity. Active state (e.g. while the user holds the zoom-in key): segment fills with `cairn-accent-ink` at 15% opacity. The buttons are labelled by title attributes for accessibility: "Zoom out", "Zoom in", "Reset view". They are *not* the primary affordance — the graph has direct pan/zoom — so they are visually quiet.

Below the topbar sits one `cairn-border-rule` (1.5px) horizontal divider in `cairn-rule-strong`. Nothing else separates the topbar from the canvas. The topbar's total height is 64px content + 1.5px rule = 65.5px.

## 4. Graph Canvas

### 4.1 Layout and overflow

The off-screen clipping is solved by a **fit-to-viewport pass on initial load and on window resize**. After the existing layout engine places nodes, compute the bounding box of all placed nodes, and scale the whole graph uniformly so the bbox fits inside the available canvas region with a 32px inset on all four sides. The existing CSS `transform: scale()` technique (which Zoom in/out/Reset will now also drive) is the mechanism — no rendering-engine swap. Once fit, the user can zoom in (the `+` button increases scale by 1.15×, capped at 2.5×), zoom out (0.87×, floored at the fit-scale so you cannot zoom beyond fit), and reset (snap back to fit-scale with a 160ms linear transition). Click-drag on empty canvas translates the graph; this translate is additive on top of the scale transform. At fit-scale, translate is clamped to 0 (you cannot pan away from a fitted view). At any scale > fit, translate is permitted.

The canvas region is tinted with `cairn-paper-sunk` and separated from the detail panel by a 1.5px vertical rule in `cairn-rule-strong`. The dot-grid background stays — it is one of the few things the baseline got right — but its dot color moves to `cairn-rule` at 40% opacity so it sits quieter under the new palette.

### 4.2 Node styling (all rounded-card SaaS defaults explicitly rejected)

Nodes are typographic rectangles. Zero radius. Zero shadow. A 1px border in `cairn-rule-strong` by default. Interior padding 12px horizontal, 10px vertical. Fill color by kind (see `cairn-node-*` tokens above): system gets parchment, container gets warm sage, module gets paper, reconciler gets blush. This is the main way node kinds differ — not by badge, by substrate.

Inside the node, stacked vertically:

- **Kind eyebrow** — small-caps sans, `cairn-size-meta` (12px), weight 500, `cairn-ink-faint`, tracking +0.5%. E.g. `MODULE`.
- **Name** — sans, `cairn-size-body-lg` (16px), weight 600, `cairn-ink`. E.g. `Artefacts`.
- **ID** — mono, `cairn-size-meta` (12px), `cairn-ink-muted`. E.g. `cairn.kernel.artefacts`. This is the change that single-handedly shifts the feel from SaaS to reference.

No other content inside the default node. Width is content-driven with a 180px minimum and 260px maximum (breaking long names across two lines is allowed; breaking IDs is not — IDs overflow with a visual hairline fade if they exceed max width).

### 4.3 Selected / unselected / container states

- **Unselected**: 1px `cairn-rule-strong` border.
- **Selected**: 2px **inset** stroke in `cairn-accent`, not outset. This is the load-bearing detail — the inset stroke makes the node feel "picked up and held," not "highlighted with a glow." No shadow, no scale transform, no outer glow. The kind-fill stays.
- **Container nodes** (system, kernel) get a second concentric 1px rule in `cairn-rule` offset 3px inside the outer border — this is the only place we use a double rule, and it reads as "this is a container" without needing a label. When a container is selected, the inner rule stays `cairn-rule` and the outer becomes the 2px inset `cairn-accent`.
- **Hover** on an unselected node: the border color deepens from `cairn-rule-strong` to `cairn-ink-faint` over 120ms. No scale, no shadow, no background change.
- **Focus** (keyboard): a 2px solid `cairn-accent` ring offset 2px outside the node border.

### 4.4 Edges

Keep the curved Bezier routing already in place. Stroke `cairn-ink-muted` at 1px. Ownership edges are solid. Dependency edges are dashed (4px dash, 4px gap — the existing dashed style is fine if it's in that neighborhood). Arrowheads are solid filled triangles 6×8px in `cairn-ink-muted`. When a node is selected, all edges *connected* to that node redraw at 1.5px in `cairn-accent`; all other edges step down to `cairn-rule` (the unselected neighborhood recedes). This is how the user reads relationships — by a focused pencil-darkening of the selected node's edges, not by a glow.

Edge labels: label all ownership edges or label none. This iteration: label none. The existing inconsistent half-labeling (only Kernel→Artefacts has "owns") must be removed — it reads as work-in-progress. Ownership is already conveyed by solid-vs-dashed.

### 4.5 Finding badges on nodes

Replace the pale amber rounded pill with a **margin mark** — a small rectangular tab attached to the node's top-right corner, same border as the node (continuous with it, not floating), filled with `cairn-severity-warn` for warn, `cairn-severity-err` for error, `cairn-severity-note` for note. Inside the tab: a single mono character in `cairn-paper` — `!` for warn, `×` for err, `i` for note — at 10px. The tab is 16×16px. No rounded corners. No text label on the node itself. The full finding code (`CAIRN_CONTRACT_MISSING`) lives in the detail panel, not on the node. The node says "something's wrong here"; the panel says what.

The five identical amber-bordered leaf modules problem is solved primarily by (a) moving the amber color off the border entirely (borders are now `cairn-rule-strong` for all kinds), (b) moving the finding indicator to a small corner tab instead of a full-width pill, and (c) the new kind-by-fill system giving modules one uniform substrate that reads as "a group" rather than five competing cards.

## 5. Detail Panel

### 5.1 Overall shape

The panel is a 380px-wide column at 1440 width, separated from the canvas by a 1.5px `cairn-rule-strong` vertical rule. Background: `cairn-paper` (matches the page, NOT the canvas — the panel reads as "on the page," the canvas reads as "inset into the page"). Internal inset: 32px horizontal, 32px vertical-top, 48px vertical-bottom (because the panel ends with a thin meta strip). Vertical scroll inside the panel is allowed; the topbar does not scroll; the panel does.

### 5.2 Header row

Top of the panel, one line:

- **Kind eyebrow** — left — mono, 12px, `cairn-ink-faint`, all-caps with tracking, e.g. `MODULE`. Not sans. Mono signals this is a taxonomy token, not a prose label.
- **Close** — right — a 1em character `×` in mono 18px, `cairn-ink-muted`, 24×24px hit target. No circle, no button chrome. Hover: color → `cairn-ink`.

Below the eyebrow row, 12px of space, then the **name** in sans 22px weight 600 `cairn-ink`. Below the name, 4px, the **ID** in mono 14px `cairn-ink-muted`. Below the ID, 24px of space before the next block.

### 5.3 Information order (top to bottom, this is load-bearing)

1. **Header block**: eyebrow + close; name; ID. (above)
2. **Description**: prose, sans 16px, `cairn-ink`, 1.5 line height, max 58 characters per line — even if the panel is wider, the prose measure is capped for readability by applying a max-width to the paragraph. If description is missing, this block is hidden — no "No description" placeholder.
3. **Findings** (only if the selected node has findings from `/api/lint`): a labeled section titled `FINDINGS` in mono 12px `cairn-ink-faint` tracking, followed by a hairline rule in `cairn-rule`, followed by a stack of finding rows. Each finding row is: [2px-wide colored left margin mark in the severity token — this is the panel-side echo of the node tab] + [finding code in mono 13px `cairn-ink`, e.g. `CAIRN_CONTRACT_MISSING`] + [on a second line, below, sans 14px `cairn-ink-muted`, a one-line description of the rule]. Each finding row has 12px vertical padding. Rows are separated by a 1px `cairn-rule` hairline.
4. **Layer navigation** — **only render if the neighbourhood has more than one layer**. If the backend reports 0/0, the entire control is hidden, not rendered as disabled. When present: a single horizontal row inset 0 from the panel edge, with three elements on one baseline: [`← BACK` in mono 12px `cairn-ink-muted`, left aligned], [`N / M` in mono 13px `cairn-ink`, centered], [`NEXT →` in mono 12px `cairn-ink-muted`, right aligned]. Disabled state (e.g. Back at layer 0): `cairn-ink-faint`. Hover on enabled: color deepens to `cairn-accent-ink` over 120ms. Below the row, a 1px `cairn-rule` rule spans the full panel width.
5. **Artefact cards** — this is the section the baseline forgot to render. For each kind returned by `/api/node/<id>/<kind>` (decisions, contracts, and whatever else the endpoint surfaces for this node — we render what the API returns, we do not invent kinds), render a **kind header** then a stack of **artefact cards**.
   - **Kind header**: mono 12px `cairn-ink-faint` tracking, all-caps, e.g. `DECISIONS`, followed inline by a mono 12px `cairn-ink-muted` count in parentheses, e.g. `(3)`. Below the header, a 1px `cairn-rule` hairline. 16px above, 8px below.
   - **Artefact card**: a rectangular block, no border-radius, no background fill, separated from the next card by a 1px `cairn-rule` hairline. Internal padding: 12px vertical, 0 horizontal (the card inherits the panel inset). Inside the card:
     - **Row 1 (header, always visible)**: on one line, left: artefact ID in mono 13px `cairn-ink` (e.g. `DEC-0041`); center-left: artefact title in sans 14px weight 500 `cairn-ink`, truncated with ellipsis on overflow; right: a single chevron character `›` in mono 14px `cairn-ink-faint` rotated 90° when expanded, 0° when collapsed. The whole row is the expand/collapse hit target.
     - **Row 2 (meta, always visible)**: on one line, sans 12px `cairn-ink-muted`, a middle-dot-separated list of whatever light meta the API exposes for that artefact (status, date, author — render only what's present; if none, hide this row entirely). E.g. `accepted · 2026-03-12 · george`.
     - **Body (shown only when expanded)**: 12px of top padding, then the artefact body prose in sans 14px `cairn-ink` 1.5 line height. If the body contains mono spans (contract names, file paths, code identifiers), those are rendered inline in mono 13px. Max prose measure: 58 characters.
     - **Files footer (shown only when expanded and when files are present)**: 12px of top padding, a small mono 12px list of file paths, one per line, `cairn-ink-muted`. Prefixed by a sans 12px `cairn-ink-faint` label `files:`.
   - The expand/collapse transition is a 140ms linear height animation with no easing curve (no spring, no ease-out-back — this is a reading UI, not a bouncy one).
6. **Tags**, **files**, **contracts**, **paths**, **dependents**, **depends-on** — each as its own small labeled block at the bottom of the panel, in the same kind-header style: mono label, hairline, content. Content is a mono list, one per line where the values are identifiers/paths; an inline comma-separated list where the values are tags. If a block has no content, the whole block is hidden (not rendered empty).
7. **End of panel**: 24px of quiet space, nothing else. The panel does *not* have its own footer strip — the page footer is the topbar's project meta, which is already shown at top.

The net effect: on a typical selected node, the panel fills 70–100% of its vertical space with structured content, instead of the baseline's 5% fill.

## 6. Meta / Schema Footer

**There is no footer.** Project name, schema version, and generated timestamp live in the topbar center block (see §3). The spec asks for these three values to be visible; it does not mandate they live below. Putting them in the topbar keeps them in eye-line, frees 48px of vertical real estate at the bottom, and avoids a second mono strip that would compete with the detail panel's bottom meta. The visual trade-off is that the topbar becomes slightly denser. That is fine for this audience.

If an implementer feels pulled toward also adding a footer: don't. The data is already shown above.

## 7. Mobile (390px)

At 390 and narrower, the layout collapses to a single column with a **bottom sheet** for the detail panel. Specifically:

- **Topbar** compresses to a single row. Product mark on the left: "Cairn" at 20px. The sub-mark "Graph Explorer" is dropped — the mark alone is enough. The mono meta strip (project · schema · generated) moves to a second row immediately below the mark, full-width, 12px mono, still separated by middle-dots but wrapping is allowed if any individual field is long. The zoom control strip moves to the right of the first row, same three glyphs but at 32px hit targets.
- **Canvas** occupies the full page width below the topbar, fixed aspect height of `60vh`. It still auto-fits on load. Pinch-zoom (if native) and touch-drag pan are supported by the existing transform plumbing.
- **Detail panel** becomes a **bottom sheet**. Hidden by default until a node is selected. When a node is selected, the sheet slides up from the bottom edge of the viewport over 180ms linear (no spring) to cover 70% of viewport height. The top edge of the sheet has a 1.5px `cairn-rule-strong` rule and, centered 8px below that rule, a 32px-wide 3px-tall `cairn-rule-strong` drag handle (a single horizontal bar with no radius — even the drag handle is square). The sheet has the same content order as the desktop panel, with horizontal inset reduced to 16px. The close `×` sits top-right in the sheet header row. The sheet scrolls independently of the canvas behind it. Tapping outside the sheet does not close it (too easy to mis-tap on mobile); only the close button closes it.
- **Artefact cards** on mobile: expand/collapse still works row-1-is-the-hit-target. Meta row may wrap to two lines. Everything else identical.

At 391px–860px (tablet range): same as desktop single-column stacking the baseline already has, but with the graph canvas getting a minimum height of 480px so it does not collapse to nothing, and the detail panel rendering as a full-width inline block below the canvas with the same internal layout as desktop. No bottom sheet at this width.

## 8. Motion Policy

Motion is quiet on purpose. This is a reading tool.

**Allowed:**
- 120ms linear color transitions on hover (node border darkens, detail panel link underline appears, zoom-strip segment background fills).
- 140ms linear height transitions on artefact card expand/collapse.
- 160ms linear transform transition on the graph canvas when Reset is pressed (so the snap-back reads as intentional).
- 180ms linear translate transition on the mobile bottom sheet slide-up.
- Keyboard focus rings appear instantly (0ms) — focus is a state, not an animation.

**Banned:**
- Any `cubic-bezier` with an overshoot (no bouncy springs).
- `ease-out-back`, `ease-in-out-back`, any "fun" curves.
- Scale transforms on hover (no "lift" effect on nodes or cards).
- Fade-on-load animations for the whole page. The page appears; it does not "present itself."
- Shimmer / skeleton shimmers while data loads — loading states use a plain `cairn-ink-faint` text row saying `loading…` in mono, not animated bars.
- Continuous motion of any kind (no subtle pulse on the accent, no scan-lines on the canvas).

All durations ≤ 200ms. All easings are `linear`. If an implementer is tempted to reach for `ease-out` to "make it feel smoother," they are fighting the aesthetic — linear is the correct choice for this reading-room mood.

## 9. Explicit Anti-Patterns — Do Not Do These Even If They Look Easier

1. **No rounded cards**. Zero `border-radius` above 2px anywhere. Nodes, artefact cards, the detail panel, the topbar, buttons — all square. If something feels harsh because of this, the fix is more breathing room, not rounder corners.
2. **No drop shadows**. Not on nodes, not on the panel, not on hover, not on the mobile sheet. Depth is done with `cairn-paper-sunk`, border weight, and rules. A shadow anywhere in this CSS is a defect.
3. **No pastel palette**. The severity tones are pigment-warm, not candy. If `cairn-severity-warn` looks like Bootstrap yellow, it's wrong — it should read as aged amber on parchment.
4. **No Helvetica / Arial default**. The sans stack is `ui-sans-serif, system-ui, -apple-system, Segoe UI`. Never fall back to Helvetica or Arial — omit them from the stack entirely so the browser goes through system UI before reaching them.
5. **No system blue CTA**. There is no primary CTA in this UI. There is no blue anywhere. The one signal color is the warm ochre `cairn-accent`, used only on selection and focus.
6. **No Bootstrap-style badges**. Finding indicators are 16×16 square margin tabs with a single mono glyph, not rounded pills with uppercase words.
7. **No IDs in sans-serif**. Any string that matches an identifier or path or contract name or kind — mono, always. This is the single most important typographic rule in the whole design. An ID in sans breaks the entire aesthetic.
8. **No gradients**. Not on backgrounds, not on node fills, not on hover. Every color is a flat token.
9. **No emoji-as-UI**. No emoji icons anywhere. Severity glyphs are ASCII (`!`, `×`, `i`). The reset glyph is `⌾` (Unicode geometric, acceptable). No emoji.
10. **No centered marketing hero**. The topbar is chrome, not a headline. `Graph Explorer` is 14px sub-mark next to the product mark, not a 40px H1 with an eyebrow above it.
11. **No full-width sidebar** that eats 40% of the viewport. Detail panel is 380px fixed at 1440, which is ~26%. The graph is the subject.
12. **No animated springs**. All motion is linear, ≤ 200ms. If the implementer reaches for a spring, they are building the wrong product.
13. **No loud labels on edges**. Label all or label none. This iteration: none. Let solid-vs-dashed do the work.
14. **No filled node backgrounds with high saturation**. Node fills are all within a ≈5% tonal range of the paper color. You should only be able to tell modules from reconcilers after looking for a beat — the information is there, but it does not shout.
15. **No rounded pill zoom buttons**. The zoom control is a connected strip with hairline dividers, not three free-floating pills.

## Opinion Statement

This page MUST feel like opening a well-printed technical journal on a clean studio desk under a warm lamp — ink on paper, mono for names, quiet ochre for the one thing you're currently holding. It MUST NOT feel like a Vercel or Linear dashboard, a Bootstrap admin, or a Figma template. If at any point in implementation it starts to feel like "a nice clean SaaS tool," something has drifted — check for: a shadow that crept in, a radius above 2px, an identifier set in sans, or the accent being used on something other than selection. Fix the drift; do not compromise the aesthetic.
