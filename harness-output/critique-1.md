# Iteration 1 Evaluation — Cairn Graph Explorer

Evaluator has never seen src/. All observations are from the rendered page at http://127.0.0.1:3737/ on tab 1121946586 (dedicated evaluation tab), plus adversarial DOM/transform probes. Dark mode was served (OS `prefers-color-scheme: dark`), which is acceptable — spec §aesthetic says "dark mode ready via CSS tokens (preferred starting theme: light, with `@media (prefers-color-scheme: dark)` override)". Dark was the one actually rendered.

## Adversarial Gate

| Check | Result | Details |
|-------|--------|---------|
| Viewport boundary (1440px, page horizontal scroll) | PASS | `scrollWidth === clientWidth`, no page overflow. |
| Click-to-select shows content in panel | PASS | Clicking Artefacts populates `.panel-name`, `.panel-id`, description "Pluggable artefact type registry", a CAIRN_CONTRACT_MISSING finding in the `FINDINGS` block, plus PATHS, CONTRACTS, DEPENDENTS, STATE labeled sections. |
| Zoom in changes transform | PASS | `matrix(0.482…)` → `matrix(0.554…, …)` (1.15× as spec'd). Translate shifts to recenter. |
| Zoom out changes transform | PASS | Zoom out from 0.554 → 0.482 (fit floor correctly clamps at initial fit-scale). |
| Reset view returns to fit-scale | **FAIL** | Clicked Reset after zooming in twice. Transform stayed at `matrix(0.637…, -499, 248)` — no change, no 160ms snap-back. Reset is a no-op. Hard-caps topbar Functionality at 5. |
| Drag-pan (`left_click_drag`) moves nodes | PASS | Translate went from `(-155, 363)` to `(-499, 248)` — 344px horizontal, 115px vertical. |
| All 9 fixture modules visible on load | PARTIAL | 9 module nodes ARE rendered in the DOM (Artefacts, Changes, CLI, Hooks, Parser, Query, ReconcilerInterface, Reconciliation, CodeReconciler). They cluster tightly at the bottom-left of the canvas (~68% of canvas vertical space is empty dot-grid above them) because the fit transform fits the full SVG viewBox (3200×2000) not the content bbox. Every node IS reachable — but the visual "fit" is fighting the reading-room intent. |
| Layer-nav hidden when 0 artefacts | **FAIL** | Panel for Artefacts (0 linked artefacts) still renders `← BACK 0 / 0 NEXT →`. Design §5.3 item 4 says: "only render if the neighbourhood has more than one layer. If the backend reports 0/0, the entire control is hidden." This is a direct spec violation that also regresses from iter-0 language (iter-0 had the same defect; iter-1 did not fix it). |
| Topbar mono meta strip present | PASS | Reads `Cairn · schema v1 · generated 2026-04-19T11:27:14Z`. Project name mono, "schema" sans label + `v1` mono, "generated" sans label + ISO mono. Spec §3 compliant. |
| Empty metadata blocks hidden | PASS (structural) | For Artefacts the panel only rendered PATHS, CONTRACTS, DEPENDENTS, STATE — each with the data it had. TAGS, FILES, DEPENDS-ON did not render empty headings. Design §5.3 item 6 compliant. |
| Console clean (load, click, zoom, layer-nav, close) | PASS | `error|warn|exception` filter returns zero messages across the whole interaction loop. |
| Mobile 390×844 (bottom sheet, no horizontal scroll) | NOT DIRECTLY VERIFIABLE | Chrome's OS minimum window-width clamp prevented the viewport from actually shrinking below ~1100px. Static CSS inspection confirms `@media (max-width: 480px)` block exists with `.detail-panel { position: fixed; bottom: 0; height: 70vh; transform: translateY(...)}`, `.sheet-handle { ... 32×3px drag handle }`, and zoom strip moved into grid-area 1/2. Structurally present but visual behavior not confirmed live. |
| Dark-mode block exists in CSS | PASS | `@media (prefers-color-scheme: dark)` rule block present with the spec'd warm-graphite palette (`--cairn-paper: hsl(32 8% 11%)`, `--cairn-ink: hsl(38 20% 92%)`, etc.). Rendered page uses these tokens — body bg `rgb(30, 28, 26)` matches the HSL intent. |
| Typography: IDs in mono | PASS | Panel IDs render in `ui-monospace, SFMono-Regular, "JetBrains Mono", "IBM Plex Mono", Menlo, Consolas`. Node IDs (e.g. `cairn.kernel.artefacts`) render in the same mono stack. Fixes the single biggest iter-0 typographic defect. |
| No rounded cards | PASS | `.zoom-button`, `.node`, `.detail-panel` all compute to `border-radius: 0px`. |
| No drop shadows | PASS | `box-shadow: none` on every element queried (`.node`, `.detail-panel`, `.panel-inner`, `.topbar`). |
| Edge labels removed | PASS | Zero edge-text labels in the SVG. Inconsistent "owns"-only labeling from iter-0 is gone. Ownership conveyed by solid-vs-dashed as designed. |
| Finding badge is corner tab | PASS | Nine `.node-finding-tab` elements render as small square tabs with `!` glyph, bg `rgb(201, 169, 115)` (dry-amber severity-warn token). Not a Bootstrap pill. Spec §4.5 compliant. |

**Gate impact:**
- Topbar zone: Reset is non-functional → caps **Functionality at 5**.
- Detail-panel zone: layer-nav renders a stub "0 / 0" on every node (no neighbourhood has artefacts in the fixture), directly violating design §5.3 item 4 → caps **Functionality at 6**.
- Graph zone: fit-to-viewport technically fits, but fits the wrong bbox (the whole 3200×2000 SVG space, not the content). Nodes cluster in bottom-left; ~68% of canvas is dead empty dot-grid. → caps **Craft at 6**.

## Close-Panel Flow

Tested per coordinator addendum.

| Step | Result |
|------|--------|
| Click close `×` after selecting Artefacts | PASS — panel gets `hidden` attribute; `display: none`; `offsetParent: null`; rect width 0. |
| Graph canvas reclaims space after close | **FAIL** — `.shell { grid-template-columns: 1097px 380px }` stays, so a 380px dead column remains to the right of the graph. The close visibly hides the panel content but leaves a reserved empty column. No horizontal scrollbar, no overt breakage, but the canvas doesn't reflow to use the freed real estate. |
| Click empty canvas after close | PASS — no error, no double-close, panel stays hidden. |
| Open node B (Parser) after close | PASS — panel reopens. Name "Parser", ID `cairn.kernel.parser`, its own `CAIRN_CONTRACT_MISSING` finding. No stale Artefacts content bleeds through. |
| Node A → close → Node B → close → Node C rotation | PASS (functional) — each open rebuilds the panel from the newly selected node's data. State-leak check clean. Same layer-nav `0/0` defect on every node though. |
| Esc closes panel | FAIL (nice-to-have) — `keydown Escape` did not close. Not blocking; noted for iter-2. |
| Console errors during close/reopen cycles | PASS — zero errors, zero warnings across the full rotation. |
| Mobile bottom-sheet close slides away | NOT DIRECTLY VERIFIABLE — OS clamp. Static CSS shows `.detail-panel { transition: transform 180ms linear; transform: translateY(...) }` with `:not([hidden]) { transform: translateY(0) }`. Structurally the slide-away is wired; live confirmation blocked. |

## Zone Evaluations

### Zone: Topbar
**Scores:** DQ: 7 | O: 7 | Craft: 6 | Func: 5

**Strengths:** Masthead reads exactly like a fixed product chrome, not a landing hero. "Cairn" sans 28px + hairline 60%-height rule + "Graph Explorer" sub-mark at 14px muted — this is the design description implemented faithfully. The center mono meta strip (`Cairn · schema v1 · generated …`) is the biggest iter-1 win: no iter-0 had project context; now it's in eye-line, mono as specified, middle-dot separators in faint ink. Right-block zoom strip is three hairline-divided mono glyphs (`−` `+` `◎`) in a single enclosed 1px-ruled container — exactly the "connected control strip, not three free pills" the spec demanded.

**Issues:**
- **Reset view is a no-op.** Clicking `◎` after zooming in leaves the transform at the zoomed value. The topbar's tertiary affordance is dead chrome. In a reading-room tool where zoom is the third-most-used interaction, a non-functional Reset is a real defect — the user zooms in, cannot get back without double-tapping zoom-out, and loses trust in the toolbar.
- Zoom strip visual weight at 40px hit targets with 16px mono glyph is almost right, but in the live render the glyphs sit slightly high within the cell (baseline not centered). Minor craft nit.

### Zone: Graph Canvas
**Scores:** DQ: 6 | O: 6 | Craft: 6 | Func: 7

**Strengths:** Typographic rectangular nodes — zero radius, no shadow, kind eyebrow in small-caps mono, name in sans 600, ID in mono muted — exactly the "citation-style" node the design specified. Finding indicator is a square corner tab with `!` glyph in warm amber (severity-warn token), not a pill. Dot grid reads quiet at `cairn-rule` 40% opacity. Edges are unlabeled (iter-0's half-labeled "owns" problem is resolved). Ownership edges solid, dependency edges dashed. Selected-node stroke is a 2px inset `cairn-accent` on the active node (`rgb(228, 153, 88)` — the warm ochre from the design). All 9 modules are present in the DOM and reachable via pan/zoom.

**Issues:**
- **Fit-to-viewport fits the wrong thing.** The SVG element is 3200×2000, and the implementation scales the whole SVG to fit the canvas region rather than computing the node-bounding-box and fitting THAT. Result: on load, nodes cluster at the bottom-left quadrant of the canvas, with roughly two-thirds of the visible area empty dot-grid above and to the right of the content. The design explicitly says "compute the bounding box of all placed nodes, and scale the whole graph uniformly so the bbox fits inside the available canvas region with a 32px inset on all four sides." This is wired but wrong. The same content displayed centered with proper inset would look dramatically more composed.
- **Kernel container's children-row is a linear row of 9 tight rectangles.** They fit, but the visual rhythm is dense — a force-directed or wrapped grid would read more like "a cluster of modules owned by Kernel" than "a bar of 9 modules waiting for their turn." Not a defect against design, which preserves existing layout; but the composition would benefit from relaxation.
- **Kernel's `collapse` / system's `collapse` text** sits inside the node as plain label copy, not as an affordance. It reads as metadata noise — design description §4.2 lists only kind-eyebrow + name + ID for node content; "collapse" is not in that list.
- **Selected ochre stroke is 2px inset** as specified, and when Artefacts is selected the connected edges redraw in the accent ochre — nice. But the un-selected edges step down to a tone very close to the paper-sunk background; in dark mode they nearly vanish, which overshoots "the unselected neighborhood recedes."

### Zone: Detail Panel
**Scores:** DQ: 7 | O: 7 | Craft: 6 | Func: 6

**Strengths:** Major iter-0 → iter-1 rebuild. Panel on clicking Artefacts now shows: MODULE eyebrow in mono tracking, close `×` at top-right, "Artefacts" in 22px sans 600, `cairn.kernel.artefacts` in mono 14px muted, 24px gap, prose description, FINDINGS (1) labeled section with a 2px ochre-brick left margin mark + `CAIRN_CONTRACT_MISSING` code in mono + rule-text in sans muted + the missing contract path in mono below. Then PATHS (1), CONTRACTS (1), DEPENDENTS (1), STATE (1) as stacked labeled blocks — each showing only what the node has, no empty headings. This is the single biggest qualitative improvement from iter-0 and it matches the design description almost line-for-line.

**Issues:**
- **Layer-nav renders `← BACK 0 / 0 NEXT →` for every node** — design explicitly bans this rendering when there are 0 artefacts. It should be fully hidden, not shown disabled. On every node I clicked (Artefacts, Parser, Changes), the counter is always 0/0. This is the single most-visible spec violation on the page.
- **Panel doesn't reflow the shell grid on close.** After `×` close, `.shell { grid-template-columns: 1097px 380px }` is unchanged. The panel correctly gets `hidden` + `display:none`, but the 380px column persists as empty canvas. Minor UX: no horizontal scroll, no broken layout — but a wide empty gutter appears where the panel was. Ideal behavior: the shell grid should collapse to `1fr` when the panel is hidden, letting the canvas reclaim the space.
- **Esc does not close the panel.** Minor. Should be a 2-line keyboard handler.
- **Artefact expandable cards not exercisable in fixture.** The Artefacts node has 0 decisions, 0 contracts in the artefact sense — only the "CONTRACTS" label showing a single contract path, not an expandable card. The full artefact-card design (§5.3 item 5 with chevron / Row 1 header / Row 2 meta / expandable body / files footer) could not be verified because no fixture node has multi-artefact neighbourhoods. Recommend iter-2 evaluator find one by probing `/api/node/X` for a richer case.

### Zone: Meta / Schema Footer
**Scores:** DQ: 8 | O: 8 | Craft: 8 | Func: 9

**Strengths:** Correctly merged into the topbar per design §6 ("There is no footer… data lives in the topbar center block"). Project name `Cairn`, `schema v1`, `generated 2026-04-19T11:27:14Z` all mono 12px, middle-dot separators in faint ink. Eye-line, quiet, uncompetitive with panel bottom meta. The missing-footer complaint from iter-0 is resolved cleanly by moving the information, not by inventing a second strip.

## Fidelity Check vs. design-description-1.md

| Design spec | Implementation |
|-------------|----------------|
| Two-theme token system with `cairn-paper`/`cairn-ink`/`cairn-accent`/severity tokens | Implemented. Dark mode block present; light-mode rule block is the `:root` fallback with the paper/ink/accent values. |
| Sans stack `ui-sans-serif, system-ui, -apple-system, Segoe UI` (no Helvetica/Arial) | Implemented. Body computed font-family matches exactly. |
| Mono stack `ui-monospace, SFMono-Regular, "JetBrains Mono", "IBM Plex Mono", Menlo, Consolas` for identifiers | Implemented. Panel IDs, node IDs, topbar meta strip, layer counter, zoom glyphs all use this stack. |
| Zero border-radius on nodes/panel/topbar | Implemented. |
| Zero box-shadow anywhere | Implemented. |
| Masthead topbar: mark + hairline + sub-mark; center mono meta; right connected zoom strip | Implemented. |
| Fit-to-viewport on load with 32px inset on all four sides | **DEVIATION** — fits full SVG viewBox, not the content bbox. Canvas appears ~68% empty above/right of the nodes. |
| Layer-nav hidden when 0 artefacts | **DEVIATION** — renders `0 / 0` always. |
| Reset snaps to fit-scale with 160ms linear transition | **DEVIATION** — Reset is a no-op. |
| Edges: unlabeled; selected node's edges redraw 1.5px ochre; others fade | Implemented. |
| Finding badge: 16×16 square corner tab, single mono glyph in paper color | Implemented. |
| Detail-panel information order (header → description → findings → layer-nav → artefact cards → metadata blocks) | Implemented (with the layer-nav-never-hidden defect noted). |
| No gradients, no emoji-as-UI | Implemented. |
| Mobile 480px: bottom sheet slide-up, 32×3px drag handle, 70vh | Implemented in CSS. Could not verify live due to OS clamp. |

Overall: the implementation tracks the design description closely on typography, color tokens, zero-radius/zero-shadow rules, node composition, and topbar. The three material deviations are **Reset no-op**, **layer-nav 0/0 showing**, and **fit-to-viewport fits wrong bbox**.

## Whole-Page Scores

| Criterion | Raw Score | Zone Floor | Final Score | Trend vs iter-0 |
|-----------|-----------|------------|-------------|-----------------|
| Design Quality | 7 | — | **7** | +3 ↑ |
| Originality | 7 | — | **7** | +4 ↑ |
| Craft | 7 | 6 (graph — fit-to-viewport fits wrong bbox) | **6** | +2 ↑ |
| Functionality | 7 | 5 (topbar — Reset no-op) | **5** | +2 ↑ |
| **Weighted average** `(2·D + 2·O + 1·C + 1·F) / 6` | — | — | **6.50** | +3.00 ↑ |

## What Works (iter-1 wins)

1. The entire visual identity landed. Square typographic nodes with mono IDs, warm graphite ground, ochre signal-color discipline, hairline rules, zero decoration. The "well-set technical journal" mood the design description asked for is present and legible.
2. Topbar is a masthead, not a hero. Project meta is in the chrome, in eye-line, mono.
3. Detail panel went from 5%-filled to 70-80%-filled with real data — description, findings with severity mark, paths, contracts, dependents, state — all hidden when absent.
4. Finding indicator moved from Bootstrap-style pill to a square margin tab with a single glyph. Single biggest individual-element improvement.
5. Zoom in/out and pan actually work.
6. Console is silent across a multi-node click/close/reopen rotation.
7. Click-to-select populates cleanly; no stale state between node swaps.

## What Fails (iter-1 blockers for SHIP)

1. **Reset view is a no-op.** Dead toolbar button.
2. **Layer-nav shows `0 / 0` on every node.** Direct spec violation; explicit design instruction ignored.
3. **Fit-to-viewport fits the SVG viewBox, not the content bbox.** The graph looks sparse and bottom-left-weighted because of it. 32px-inset framing the design asked for is not what is on screen.
4. **Panel close leaves a 380px dead column.** Grid doesn't reflow.
5. **Esc does not close the panel.** Nice-to-have, not a blocker.
6. **Node "collapse" label text sits inside system/container nodes** as plain copy. Either make it an interactive affordance with its own styling (chevron + hit target) or remove it from the default node layout per design §4.2.
7. **Artefact expandable cards (§5.3 item 5) not exercisable** against the current fixture — can't confirm they render correctly because no fixture node has the multi-artefact shape. Iter-2 needs a node with richer `/api/node/<id>` to verify.

## Direction: **REFINE**

Iter-1 jumped from 3.50 → 6.50 (+3.00). Design-quality and originality both hit 7.0, the spec's SHIP threshold. Craft (6) and Functionality (5) are below threshold and the four defects above are all shallow — each is a ~10–50 line fix, not a re-architecture. One more focused pass should put everything over 7.0 (Functionality needs to reach 8.0 per sprint contract).

## Fix Items for Iteration 2 (ordered by impact)

1. **Wire Reset correctly.** On `◎` click, animate the `#graph` transform to the computed fit-scale over 160ms linear. Shouldn't be hard — the fit-scale is already computed; Reset just has to call back to it.
2. **Hide the layer-nav when the neighbourhood has 0 artefacts.** Toggle a `hidden` attribute on `.layer-nav` (or `display: none`) when the count is zero. If ANY fixture node has artefacts, verify the reveal works; otherwise leave hidden across the board.
3. **Fix the fit-to-viewport to fit the content bbox, not the SVG viewBox.** After layout, walk the rendered node positions, compute `min(x), min(y), max(x+w), max(y+h)`, then scale+translate so that bbox fits the canvas region with a 32px inset all around. This will center the graph instead of pinning it bottom-left.
4. **Release the panel column on close.** When `.detail-panel[hidden]`, set `.shell { grid-template-columns: 1fr }` (or add a modifier class to the shell). Canvas reclaims the 380px.
5. **Escape-to-close.** Single keyboard handler on `document`: if panel is open and key is Escape, fire the close-button click.
6. **Decide the fate of the "collapse" label** inside system/container nodes. Either promote it to a visible chevron affordance in the top-right with its own hit target and tooltip, or drop it from the default node body per design §4.2 (kind + name + ID only).
7. **Verify artefact-card render** against a node that actually has artefacts. Either extend the fixture or hit `/api/node/X` on a richer node; if no such fixture exists, this is a blocker for confirming §5.3 item 5 compliance.
8. **Light-mode verification.** OS-level dark mode forces dark rendering in this environment; iter-2 evaluator should toggle DevTools emulation to `prefers-color-scheme: light` and confirm the warm paper palette renders and that no hardcoded dark values leak through.

## Instructions for Iter-2 Evaluator: Full Interaction Sweep (mandatory)

Per user instruction, no iteration may be marked SHIP without at least one full interaction sweep. Iter-2 evaluator must exercise:

1. **Every topbar control**: Zoom out, Zoom in, Reset — click each at least twice, verify transform matrix shifts to distinct values on each click and that Reset returns to fit-scale. Capture before/after matrix values in the critique.
2. **Zoom stacking**: click Zoom in 5 times to confirm it approaches the 2.5× cap cleanly; click Zoom out 5 times to confirm it floors at fit-scale and doesn't go further.
3. **Pan in two directions**: drag-right, drag-up. Both must change the translate.
4. **Click every node kind**: system (Cairn), container (Kernel), leaf module (Artefacts or another). Confirm kind-eyebrow, fill tone, border treatment differ.
5. **Collapse/expand a container**: if the affordance exists, click it on Kernel — verify children hide/reveal and the layout reflows without clipping.
6. **Close-and-reopen the panel**: node A → `×` → node B → `×` → node C. No stale findings, correct ID in each open, no console errors, grid reflows after each close.
7. **Layer-nav with REAL artefacts**: probe `/api/node/X` to find a node with >0 layers, select it, verify Back/Next advance the counter and change panel content.
8. **Mobile bottom sheet at ≤480px**: use DevTools emulation (Chrome's OS resize clamp blocks true 390px). Verify sheet slides up on node-click, `×` slides it down with the 180ms linear transition, 3px drag handle is visible.
9. **Light mode**: DevTools → prefers-color-scheme: light. Verify the warm paper palette renders and no color token is missing/black-on-black.
10. **Focus ring**: Tab to a node, confirm 2px accent ring offset 2px outside.

Only after that sweep passes end-to-end should a SHIP decision be considered.
