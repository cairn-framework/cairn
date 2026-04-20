# Design Description — Iteration 2, Cairn Graph Explorer (refinement of iter-1)

This is a **refinement**, not a redesign. Iter-1 landed the visual identity at 7/7 for design and originality (weighted avg 6.50). The shortfall is Craft (6) and Functionality (5), and the critique attributed each failure to a shallow behavior defect, not to the aesthetic. This iteration fixes those behaviors and does not touch the look. If any decision here appears to contradict iter-1, read iter-1 as the source of truth and this as deltas layered over it.

## 0. Deltas at a glance

1. **Fit-to-viewport now fits the *content* bounding box, not the 3200×2000 SVG canvas.** This is the single visible composition fix.
2. **Reset view actually resets** — same math as fit, 160ms linear snap.
3. **Layer-nav is hidden (not disabled, not stubbed) whenever the neighbourhood has ≤ 1 layer.** The panel's vertical rhythm closes up above the artefact section when it is absent — no ghost spacing, no empty rule.
4. **Panel close collapses the shell grid column** — canvas reclaims the 380px instantly. The panel does not animate out; the column reflows in one frame.
5. **Esc closes the panel** when the panel is open. Close button receives a focus-ring on panel open as a subtle keyboard affordance.
6. **The literal text "collapse" is gone from system/container nodes.** Replaced by a square chevron glyph in the node's top-right, paired to the new corner-tab grammar the finding indicator already established.
7. **Interaction sweep polish**: hover state on node cards, zoom-button active/press state, long-generated_at truncation rule in topbar, drag cursor feedback.

## 1. Graph canvas — fit-to-content-bbox

### (a) Rule / spec

After layout, compute the axis-aligned bounding box of all rendered node rectangles (not of the SVG viewBox, not of the edge splines, not of labels — the node rectangles, which are the subject). Call this `contentBox = { minX, minY, maxX, maxY, w, h }`.

Define the **canvas region** as the visible area inside the dot-grid panel — i.e. after the topbar, before the detail-panel rule, with its own padding. Call this `viewRect = { w, h }` in CSS pixels.

The **fit transform** is:

- `padInner = 48px` on all four sides (was 32px in iter-1 — increase by 16px so the graph reads "matted" inside the canvas rather than "pressed to the edges" after centering).
- `availableW = viewRect.w − 2·padInner`
- `availableH = viewRect.h − 2·padInner`
- `fitScale = min(availableW / contentBox.w, availableH / contentBox.h, 1.0)` — the final `min(…, 1.0)` is the **small-content clamp**: if the rendered graph is smaller than the canvas, we do **not** scale up; we center it at 1.0× so nodes keep their native type size. An architect looking at a 4-module graph should see those 4 modules at their natural reading size, not blown up to fill the screen.
- `translateX = padInner + (availableW − contentBox.w·fitScale)/2 − contentBox.minX·fitScale`
- `translateY = padInner + (availableH − contentBox.h·fitScale)/2 − contentBox.minY·fitScale`

The transform applied is `translate(translateX, translateY) scale(fitScale)`. Zoom in/out multiply `fitScale`; pan adds to translate; zoom-out floor is `fitScale` (cannot zoom below fit); pan is clamped to 0 when the current scale equals `fitScale`.

### (b) Rationale from the critique

Critique §Graph Canvas: "fits the full SVG viewBox (3200×2000) not the content bbox. Result: on load, nodes cluster at the bottom-left quadrant of the canvas, with roughly two-thirds of the visible area empty dot-grid above and to the right of the content." This caps Craft at 6. The fix is arithmetic, not aesthetic. The composition is already correct — it is just anchored to the wrong rectangle.

### (c) Implementation-visible behavior

- On load: the graph appears centered in the canvas with 48px of dot-grid margin around the outermost nodes. No node sits flush against any canvas edge.
- On window resize: recompute `viewRect`, recompute `fitScale`, re-apply `translateX/translateY`. The transition on resize is **instant** (no animation — resize is a layout event, not an interaction). Debounce by one animation frame so we don't thrash during the resize drag.
- If the content is smaller than `availableW × availableH`, the graph centers at 1.0× — no upscale. The surrounding dot-grid negative space is intentional, not a bug.
- When the user zooms beyond fit and then resizes: keep the user's current user-scale relative to `fitScale` (i.e. preserve `userScale / fitScale`), recompute `fitScale`, reapply. Do not reset the user's zoom on resize.

## 2. Reset view — actually resets

### (a) Rule / spec

Clicking the reset glyph (`⌾` in the zoom strip) animates the graph's transform back to the fit-transform computed in §1. The animation is **160ms linear** on `transform`, no easing curve, no bounce. On animation end, the transform must equal what §1 would produce right now (re-read `viewRect` at animation start, do not cache from page load — the window may have resized since).

Keyboard equivalent: pressing `0` (zero) while focus is on the canvas or nowhere-in-particular also triggers reset. (Mirrors the common "fit to screen" shortcut; not a requirement, but consistent with pro tools.)

### (b) Rationale from the critique

Critique §Topbar: "Reset is a no-op. Dead toolbar button. Hard-caps Functionality at 5." The handler exists; it just doesn't call the fit-transform function. Wire it.

### (c) Implementation-visible behavior

- Zoom in twice → click reset → graph glides back to centered fit over 160ms. Glide is linear, not eased. No overshoot.
- Pan the graph off-center → click reset → graph glides to centered fit. Translate and scale animate together on the same 160ms.
- Click reset while already at fit → no visible motion (transform doesn't change). No flicker, no "snap to self" animation.
- The reset button gets a 120ms color deepen on hover (consistent with the other zoom-strip segments). On click, the segment's background fills with `cairn-accent-ink` at 15% opacity for the duration of the 160ms transition, then releases — this gives the dead-button defect from iter-1 a *visible* affirmative response.

## 3. Layer-nav visibility

### (a) Rule / spec

Render the entire `.layer-nav` block **only when** `neighbourhood.layers > 1` **and** `neighbourhood.hasArtefacts === true`. Otherwise, the block is fully removed from flow (`display: none`, not `visibility: hidden`, not `opacity: 0`, not a disabled-stub). No horizontal rule, no container padding, no "0 / 0" counter — the DOM has nothing in that position.

The condition is not `count !== 0`; it is `count > 1`. A single-layer neighbourhood with one artefact still has nothing to navigate between, so the control would mislead.

### (b) Rationale from the critique

Critique §Detail Panel: "Layer-nav renders `← BACK 0 / 0 NEXT →` for every node — design explicitly bans this rendering when there are 0 artefacts. It should be fully hidden, not shown disabled. This is the single most-visible spec violation on the page." Hard-caps Functionality at 6.

### (c) Implementation-visible behavior — and vertical rhythm

When hidden, the panel's vertical stack closes up. The order in iter-1 was:

```
[header block]
  24px
[description]
  24px  (space-5)
[findings — if any]
  24px  (space-5)
[layer-nav — always rendered, even when empty]   ← the defect
  16px
[artefact-card blocks]
  …
```

When layer-nav is hidden, the space between the preceding section (findings, or description if no findings) and the artefact-card block collapses to a single `cairn-space-5` (24px). There must not be a 48px double-gap where the nav used to sit, and there must not be an orphaned hairline rule above the artefact section. The rule that previously closed the layer-nav block is *rendered as part of the layer-nav block itself*, so removing the block removes its rule.

If there are no artefact-card blocks either (the common case in the current fixture), the panel terminates after findings (or after the metadata tail blocks — tags / files / contracts / paths / dependents / depends-on). No trailing empty regions.

## 4. Close-panel grid reflow

### (a) Rule / spec

The shell uses `grid-template-columns`. iter-1 wires `1097px 380px`; when `.detail-panel[hidden]` is present, apply `grid-template-columns: 1fr` (or equivalently `1fr 0`). The canvas column expands to reclaim the 380px. The detail-panel column goes to zero width.

Transition: **instant** — not animated. The panel itself is already display:none'd by the hidden attribute, so there is nothing visual to animate. The canvas snapping wider on close is fine and preferred — it reads as "the column closed," not as "the canvas stretched." On panel *reopen* (node click), the grid returns to `1097px 380px` (or whatever the desktop track is), again instantly; the panel's own internal slide/fade is not part of this spec — the panel appears in place.

At 1440 viewport, the column values are `1fr 380px` when open and `1fr` when closed. The 1fr is implicit; do not hardcode the 1097px.

### (b) Rationale from the critique

Close-panel flow test, critique §Close-Panel Flow: "`.shell { grid-template-columns: 1097px 380px }` stays, so a 380px dead column remains to the right of the graph." The panel hides; the space does not return. Fix the grid.

### (c) Implementation-visible behavior

- Open panel → `×` close → canvas expands to full width in one frame. No horizontal scroll appears during the transition (there is no transition).
- After close, the graph's fit transform should **re-fit** to the new, wider `viewRect` — same math as §1, same small-content clamp. So the close action visibly recomposes the graph. This is the payoff for fixing both §1 and §4 together: close-to-open cycles produce well-composed frames at both widths.
- Reopen via node click → column snaps back to `1fr 380px` instantly, panel populates with the selected node's content, graph re-fits to the narrower `viewRect`.
- At ≤480px (mobile), the column reflow rule does not apply — the panel is a bottom sheet, not a grid column. Keep iter-1's mobile CSS untouched.

## 5. Esc closes the panel

### (a) Rule / spec

A single `keydown` listener on `document`. When the panel is *not hidden* **and** the pressed key is `Escape` **and** no text input is focused (guard against interrupting future filter/search inputs), fire the close-button's click handler. The listener is always installed; the conditional gates it. Do not attach/detach per panel open — that is fragile.

Visual cue: when the panel opens, the close `×` receives a **focus ring** (the standard 2px `cairn-accent` ring offset 2px) for the first 400ms, then the ring fades to `cairn-rule-strong` over 200ms linear. This telegraphs "Esc is a valid way out" without requiring the user to Tab into the panel. After 600ms total, the close button looks normal; the keyboard affordance was announced and then got out of the way.

The ring on panel-open is suppressed if the user's previous interaction was via keyboard (detected by a `:focus-visible` style toggle, or by a `keydown` flag set shortly before node selection). Keyboard users already get focus rings; don't double-announce.

### (b) Rationale from the critique

Critique §Close-Panel Flow: "Esc closes panel — FAIL (nice-to-have). `keydown Escape` did not close." Not blocking, but trivially cheap and improves the tool's keyboard parity. The orchestrator explicitly pulled this up to a numbered priority.

### (c) Implementation-visible behavior

- Click any node, panel opens. Close `×` shows a brief ochre focus ring.
- Press Esc. Panel hides, grid reflows, canvas re-fits. Same effect as clicking `×`.
- Press Esc again with panel already hidden. Nothing happens. No console error.
- Focus an input (future-proofing — there are none in this iteration), press Esc. Panel does not close. Input loses focus per browser default.

## 6. Container/system node — remove "collapse" label, replace with chevron tab

### (a) Rule / spec

The literal text `collapse` currently rendered inside system- and container-kind nodes is leaked developer wording and must not appear. The node body in those kinds stays the same as iter-1 (kind eyebrow + name + ID) — nothing is added to the body.

In place of the text label, add a **square corner affordance** in the top-right of the node, mirroring the grammar of the finding tab but in the opposite corner and with different semantics:

- 16×16px square.
- Zero radius.
- Background: `cairn-paper` (matches the node fill for module-style, or the kind-fill for system/container — it blends in with the node, it does not stand off).
- Border: 1px `cairn-rule-strong` on the left and bottom edges only (so it reads as "continuous with the node's border," same trick the finding tab uses on the opposite corner).
- Glyph: a single mono chevron `›` rotated 90° when the container is expanded (pointing down, showing "click to collapse"), 0° when collapsed (pointing right, showing "click to expand"). Glyph color `cairn-ink-muted`. 10px mono.
- Hit target: the full 16×16 square.
- Tooltip (title attribute): `Collapse` when expanded, `Expand` when collapsed. This is the ONLY place the word "collapse" appears in the UI — inside a native tooltip, not as visible body copy.
- On hover: glyph color deepens to `cairn-ink`. 120ms linear.
- On focus (keyboard Tab): the standard 2px `cairn-accent` ring offset 2px outside the 16×16 tab — exactly the same focus treatment as the panel close button.

If a node is of a kind that has no children at all (leaf modules), this corner affordance is **not rendered**. A leaf module with an expand chevron would lie to the reader.

### (b) Rationale from the critique

Critique §Graph Canvas: "Kernel's `collapse` / system's `collapse` text sits inside the node as plain label copy, not as an affordance. It reads as metadata noise — design description §4.2 lists only kind-eyebrow + name + ID for node content." The orchestrator flagged this as "leaked developer wording."

Also addresses a usability question iter-1 did not: how does a user discover that containers are collapsible? A chevron in a consistent corner answers this visually, matching the grammar we already established for finding tabs.

### (c) Implementation-visible behavior

- Kernel (container) renders its kind eyebrow + name + ID + the small `›`-rotated-90° tab in top-right. No text label.
- Clicking the tab hides Kernel's children. The tab glyph rotates to 0° (no animation — this is a state change, not a motion event). Edges reroute to terminate at Kernel.
- Clicking the tab again re-shows children. Glyph back to 90°.
- Hovering the container body anywhere else (not the tab) does not suggest collapse — hover stays on the node-border-darken rule from iter-1.
- Collapsing a container re-fires the fit-to-content-bbox transform from §1, because the content bbox has changed. The re-fit animates at 160ms linear, same as reset, so the composition "settles" after a collapse rather than snapping.
- When a container is selected (clicked, panel opens), the corner chevron tab stays visible and the node's selection stroke (2px inset `cairn-accent`) still paints. The chevron tab sits *on top of* the inset stroke at its corner — the stroke appears to run under the tab (because the tab's left/bottom borders continue the node's border logic).

## 7. Interaction-sweep polish

These are small touches surfaced when I imagine the iter-2 evaluator's mandated full sweep. Each is a one-liner behaviorally, none of them are aesthetic pivots.

### (a) Node hover state — visible, quiet

iter-1 has node border darkening from `cairn-rule-strong` → `cairn-ink-faint` over 120ms on hover. Confirmed in critique. Add one more quiet cue to distinguish "hovering a clickable node" from "hovering dead canvas": the cursor becomes `pointer` on node hover. The dot-grid canvas itself stays on `default` cursor for pan-ready regions and switches to `grab` / `grabbing` during drag-pan (see §7d).

### (b) Zoom-button stacking and press feedback

Zoom in and zoom out can be clicked rapidly. Each click multiplies scale by 1.15× (in) or 0.87× (out); cap at 2.5× (in) or floor at `fitScale` (out). Iter-1 handles the arithmetic. The missing piece is *visible* confirmation of the press.

- On `:active` (mouse down, before release), the segment fills with `cairn-accent-ink` at 15% opacity. Releases on mouse up.
- If the user holds the button (continuous press), the fill stays. This matches iter-1's active-state spec.
- When at the cap (2.5× for zoom-in, `fitScale` for zoom-out), subsequent clicks on the capped button produce **no** fill and no transform change — the button visibly does nothing, because there is nothing to do. This is correct; do not pretend a click worked. A tooltip on the capped state could say `At maximum zoom` / `At minimum zoom` but this is optional.

Do not stack animated transitions visibly — each click runs its own 120ms linear transform transition. If a user mashes zoom-in 5×, the resulting transform is the composed 5× worth of scaling with 5 overlapping 120ms transitions, which is fine — the last one wins and the graph rests at 2.5×.

### (c) Topbar — long generated_at handling

The mono meta strip reads `Cairn · schema v1 · generated 2026-04-19T11:27:14Z`. If `generated_at` is a longer string (future-proofing against full ISO with timezone offsets, e.g. `2026-04-19T11:27:14.125+01:00`), and the strip would push past 60% of the topbar width:

- The strip's `max-width` is `60vw` at ≥1200px; contents use `overflow: hidden` and `text-overflow: ellipsis` on the `generated` field specifically (not on the whole strip — we always want project name and schema version fully visible). The `generated` field truncates from the right with `…` if needed.
- On hover of the truncated field, a native `title` attribute exposes the full ISO string. No tooltip chrome, no popover — just the browser's built-in behavior.
- At <1200px, the strip wraps to a second line as iter-1 already handles; no truncation needed there.

This is belt-and-braces — the current fixture's timestamp fits. But the critique's sweep mandate will probe this and a graceful degradation is cheap.

### (d) Drag cursor and pan feedback

During click-drag on empty canvas (pan): cursor becomes `grabbing` while mouse is down and moving. On mouse-up, returns to `grab` when hovering empty canvas. On mouse-up over a node, returns to `pointer`. These are CSS-only and cost nothing but communicate "this region is draggable" which the iter-0 and iter-1 evaluators both had to discover by experiment.

Kinetic pan / inertia is **not** added. This is a reading tool; the graph comes to rest on mouse-up immediately. Linear truth beats simulated physics here.

### (e) Artefact panel empty state (micro-fix)

The artefact-panel zone scored 5/5/5/5 because artefacts are out of sprint scope and no fixture node exercises the expandable-card shape. Leave the card design unchanged. One belt-and-braces item: if a kind header would render with `(0)` — e.g. `DECISIONS (0)` — suppress the whole header. No kind header should appear above zero rows. This is already implied by iter-1's "hide empty blocks" rule but not explicitly called out for artefact kinds.

### (f) Mobile belt-and-braces CSS

The iter-1 evaluator couldn't verify mobile live due to OS clamp. Belt-and-braces additions:

- `.detail-panel` at ≤480px uses `position: fixed; inset: 30vh 0 0 0;` when open. This is equivalent to `bottom: 0; height: 70vh` but tolerates iOS Safari's dynamic viewport quirks better.
- `overscroll-behavior: contain` on the bottom sheet — prevents scroll chaining from inside the sheet to the page behind it. Keeps the canvas still while the user reads a long artefact.
- The 3px × 32px drag handle is presentation-only in this iteration (no drag-to-dismiss handler yet). It should still be visually present — the spec is about the mood of the sheet, not about physics.
- `pointer-events: none` on the canvas behind the open sheet, so an accidental tap through the sheet doesn't select a new node. Re-enable on close.

## 8. Do not change

A hedge against implementation over-editing. Every item below is from iter-1 and **must survive iter-2 unchanged**. If a change in this doc seems to contradict any of these, this doc is wrong; the list below wins.

- The full token system (colors, type, spacing, radii, borders, shadow policy) from iter-1 §2. Every HSL value stays, every token name stays.
- Light as the default theme, dark via `@media (prefers-color-scheme: dark)`, same warm-bias palette on both.
- The **ink-on-paper palette**: warm off-white ground, warm graphite in dark mode, burnt-ochre accent used *only* on selection/focus. No new colors. No system blue. No pastels.
- **Mono for all identifiers.** Node IDs, panel IDs, paths, contract names, kind labels, schema version, the layer counter (when present), the generated timestamp. An identifier in sans is a defect.
- **Zero border-radius** on nodes, panels, topbar, zoom buttons, artefact cards, finding tabs, the new chevron tab (§6), everything. `cairn-radius-1` = 2px is reserved for focus-ring outer edge softening only.
- **Zero box-shadow** anywhere. Depth via `cairn-paper-sunk` inset tone and border weight.
- **Corner finding tab** (16×16 square, continuous with node border, single mono glyph `!` / `×` / `i`, paper-colored glyph on severity-colored fill). Keep exactly as iter-1 rendered. The new chevron tab in §6 is its mirror in top-right; both grammars coexist.
- **Masthead topbar** with mark + hairline sub-rule + sub-mark on the left, mono meta strip in the center, connected 3-segment zoom strip on the right. Do not split into two rows on desktop. Do not pill-ify the zoom controls.
- **No footer.** Project meta lives in the topbar.
- **Typographic rectangular nodes** with kind eyebrow (mono 12px) + name (sans 16px 600) + ID (mono 12px muted). Fill by kind (parchment / sage / paper / blush). Selected = 2px inset `cairn-accent` stroke.
- **Edges unlabeled.** Solid = ownership, dashed = dependency. Selected node's edges redraw 1.5px `cairn-accent`; non-connected edges step down to `cairn-rule`. Do not restore partial labeling.
- **8px base spacing** (4-8-12-16-24-32-48-64-96). Every gap uses a token.
- **Motion policy**: ≤200ms linear only. No easings with overshoot. No shimmers. No fades on page load. No continuous motion.
- **Detail panel information order**: header → description → findings → layer-nav (when shown) → artefact-card blocks → metadata tail (tags/files/contracts/paths/dependents/depends-on). Hide empty blocks entirely.
- **Dot-grid background** in the canvas region, dots at `cairn-rule` 40% opacity.
- **Bottom sheet on mobile** (≤480px) — 70vh, 180ms linear slide, 32×3px square drag handle, close `×` in header.

## Opinion Statement

This iteration MUST feel like the iter-1 design with the controls finally believed. The reading-room identity is right; what was off was that the buttons whispered promises the DOM didn't keep — Reset was dead, layer-nav lied about its state, fit-to-viewport pinned the composition to the wrong rectangle, and the close button left a ghost column. Every delta in this document is about making the tool *trustworthy under sweep* — every button does what its glyph says, every empty block is absent (not disabled), every composition is centered in the rectangle the user is actually looking at. The tool MUST NOT start to "feel polished" in a way that drifts toward SaaS — no new micro-interactions, no springs, no accent-color creep onto secondary controls. Polish here means *quieter and more correct*, not *louder and smoother*. If at any point in implementation iter-2 starts adding motion, colors, or controls that weren't in iter-1 or in this delta list, it is fighting the aesthetic — revert, re-read iter-1.
