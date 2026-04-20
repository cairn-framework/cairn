# Design Description — Iteration 3, Cairn Graph Explorer (refinement of iter-2)

This is a tight delta pass, not a redesign. Iter-1 established the reading-room aesthetic at weighted 6.50. Iter-2 regressed to 5.20 because five of seven intended fixes did not land — every one of them a shallow behavioural defect on the same aesthetic that the critique already calls "right." This brief is six deltas. Nothing else moves.

Read iter-1 as the source of truth, iter-2 as the intent layer, and this document as the correction pass. If any statement here conflicts with iter-1's palette, typography, or motion policy, iter-1 wins.

## Deltas at a glance

1. Fit-to-content-bbox engages on initial load — no clipped nodes on first paint.
2. Reset view lands on the current fit-scale even after a panel-close reflow.
3. Layer-nav is fully absent (no flow, no whitespace) when there are 0 or 1 artefacts.
4. Section headers (FINDINGS, PATHS, CONTRACTS, DEPENDS-ON, DEPENDENTS, FILES, TAGS, STATE, ARTEFACTS) never render over empty content.
5. Chevron tab polish — leaves have none, rotation animates smoothly, pigment is neutral, the tab does not open the panel.
6. Close-and-re-open cycle is state-clean — no stale findings, no stale artefact index leaking across nodes.

---

## 1. Fit-to-content-bbox engages on initial load

### (a) Rule / intended behavior

The first composed frame the user sees — the frame that exists before any click, any resize, any keypress — must already be fit-to-content. The graph does not start at natural 1.0× and then correct itself. It starts correct. The viewer lands on a balanced composition: all fixture modules visible inside the canvas region, with approximately 48px of dot-grid margin around the outermost nodes, centered on both axes.

If the content bounding box is smaller than the canvas, the graph rests at 1.0× and centers without upscaling (iter-1's small-content clamp is preserved). If the content is larger than the canvas, the graph scales down uniformly so it fits inside the canvas minus the 48px inset. In the current fixture at a 1440-wide viewport, the computed scale is below 1.0 and the graph fills the majority of the canvas width.

### (b) What the critique saw

Iter-2 renders a fresh-navigation frame at `translate(0,0) scale(1)` — the fit computation never runs before the user's first observation. The natural 2105-wide content overflows a 1477-wide viewport; ReconcilerInterface is clipped mid-word and two other modules are off-canvas entirely. Craft and Functionality are both capped at 4 on this single defect.

### (c) Implementation-visible success signal

At 1440×900, on first paint (no interactions):
- The graph transform is NOT `translate(0,0) scale(1)`. It is a real fit-transform with a scale value below 1.0 (or at 1.0 with a positive centering translate if the content is smaller than the canvas).
- Every fixture node's centerpoint lies inside the visible canvas region.
- No node's bounding box extends past the canvas edges.
- The visible composition shows roughly 48px of dot-grid between the outermost nodes and the canvas edges, on all four sides.

---

## 2. Reset view lands on the post-reflow fit-scale

### (a) Rule / intended behavior

Reset always computes its target from the current canvas dimensions at the moment Reset is pressed — never from a cached value. The result is that Reset is idempotent relative to *right now*: whatever the canvas width is when Reset fires, the graph snaps to the fit-transform that matches that width, over a ≤200ms linear transition. No easing curves, no overshoot.

The payoff case: a user zooms in twice, pans, opens the detail panel, closes the detail panel (canvas reclaims the 380px column), then clicks Reset. The graph settles on the fit that matches the now-wider canvas — not the fit that was correct a moment ago with the panel open. "Reset" must mean "fit to what you are currently looking at," not "fit to what used to be here."

### (b) What the critique saw

After a panel-close reflow, Reset occasionally produced `scale(0.467)` where the correct value was ~`0.655`. The ratio — roughly 0.71 — matches the narrower open-panel width divided by the closed width, which strongly suggests the recompute pipeline is reading a stale rectangle somewhere. From the user's seat, Reset becomes untrustworthy: sometimes it fits, sometimes it shrinks the graph. A Reset button that is usually-but-not-always right is worse than one that is always wrong, because the user cannot build a mental model.

### (c) Implementation-visible success signal

- Zoom in twice, pan off-center, open the detail panel, close the detail panel, click Reset. The graph animates to a transform whose scale equals the fit-scale for the current (post-close) canvas width, within a ≤200ms linear transition. No intermediate overshoot, no second correction.
- Immediately invoking Reset again produces no visible motion (idempotent when already at fit).
- The same cycle at a different viewport width produces the appropriate fit for *that* width. Reset does not carry state across resizes.

---

## 3. Layer-nav absent when artefacts ≤ 1

### (a) Rule / intended behavior

When the currently-selected node has 0 or 1 artefacts in its neighbourhood, the layer-nav block is not in the document flow at all. Not hidden-and-still-taking-space. Not visually empty but occupying 31px of vertical room. Not present-but-transparent. Gone.

That means: no heading, no counter, no back/next glyphs, no surrounding rule, no vertical whitespace where the nav used to be. The detail panel's vertical rhythm closes up as if the nav were never part of the schema. The preceding section (description, or findings if present) flows directly into the next (metadata tail) with a single `cairn-space-5` gap — not a doubled gap.

This is true for every node in the current fixture (every fixture node has zero artefacts), so on this iteration the layer-nav never renders anywhere. The fix is categorical, not per-node.

### (b) What the critique saw

The hide logic is present in intent — a `hidden` attribute is set when artefact count is 0 — but the base rule for the block uses a display value that overrides the browser default for `[hidden]`, so the row still paints `← BACK 0 / 0 NEXT →` on every single node. The critique flagged this as "the single most visible spec violation on the page." It is the same defect iter-1 shipped, recurring in a slightly different guise.

### (c) Implementation-visible success signal

- Click any node in the fixture (every node has 0 artefacts). The string `BACK` does not appear anywhere in the detail panel. The string `NEXT` does not appear. The counter `0 / 0` does not appear. No hairline rule sits where the nav would have.
- Measuring the vertical distance from the bottom of the description (or findings, when present) to the top of the next rendered block shows a single `cairn-space-5` (24px), not 48px or more. No ghost gap.
- A click on a hypothetical multi-artefact node would render the full nav with its heading, counter, and buttons — the block is conditional, not deleted.

---

## 4. No orphan section headers over empty content

### (a) Rule / intended behavior

Every labeled block inside the detail panel — FINDINGS, PATHS, CONTRACTS, DEPENDS-ON, DEPENDENTS, FILES, TAGS, STATE, ARTEFACTS — renders its header *only if* that block has at least one row of content. If the content array is empty, the block does not render: no label, no hairline rule under the label, no trailing whitespace, no "(0)" count. The metadata tail collapses section-by-section depending on what the selected node actually has.

This is a single rule applied to every section, not a per-section retrofit. The principle from iter-1 — "hide empty blocks entirely" — must be enforced for every labeled region that the panel can render, not just some of them.

### (b) What the critique saw

On Cairn (zero findings), a `FINDINGS` header and its hairline rule render underneath the description, floating above no content. Iter-2 added this suppression for the metadata tail but missed findings. The defect is small individually but signals inconsistency: the panel's information architecture says "a label means there is something to say here," and an orphan label breaks that contract.

### (c) Implementation-visible success signal

- Select Cairn (no findings, no paths, no contracts, no depends-on, no files beyond the root). The strings `FINDINGS`, `PATHS`, `CONTRACTS`, `DEPENDS-ON` do not appear anywhere in the panel. The panel's vertical extent matches the actual content.
- Select Artefacts (has one finding, no paths, no contracts). `FINDINGS` renders with its one row. `PATHS` does not appear. `CONTRACTS` does not appear.
- Select any leaf with at least one tag. `TAGS` renders. The labels that apply render; the ones that don't, don't. No label in the panel ever sits above zero rows.

---

## 5. Chevron tab polish on parent nodes

### (a) Rule / intended behavior

Iter-2 landed the basic chevron on system- and container-kind nodes; iter-3 tightens four loose threads:

(i) **Leaves have no chevron.** A module with no children must not render the corner tab. An expand/collapse affordance on an unexpandable node is a lie. The tab renders only where collapse/expand is meaningful: SYSTEM and CONTAINER kinds.

(ii) **Smooth rotation.** When the parent is collapsed, the chevron points right (0°). When expanded, it points down (90°). The rotation between states animates over ≤200ms linear — no snap, no bounce, no fade-through. The motion is small and quiet, matching the overall motion policy. This is the one animated state-change in the node vocabulary; it should feel like a turning page, not a flipping switch.

(iii) **Neutral pigment.** The chevron tab is a wayfinding affordance, not a severity indicator. Its fill is `cairn-paper` (matching the node body, continuous with the border). Its glyph is `cairn-ink-muted`. It must not use `cairn-severity-*` tones and must not use `cairn-accent`. The accent color is reserved for selection and focus; using it on a chevron would put two competing ochres on the same node the moment the user selects an expanded container.

(iv) **Click isolation.** Clicking the chevron toggles the container's expansion and does *nothing else*. It does not propagate to the node's selection handler, does not open the detail panel, does not redraw edges to an ochre-selected state. The user clicks the chevron to restructure the view; they click the node body to inspect it. Two distinct gestures, two distinct outcomes.

### (b) What the critique saw

Iter-2 hit the fundamentals cleanly — glyph present on SYSTEM/CONTAINER only, absent from leaves, collapse-click does hide children, the literal word "collapse" is gone. But the rotation is state-swapped (instantaneous), the tab pigment was not audited against the accent/severity palette, and click-propagation was not validated.

### (c) Implementation-visible success signal

- Every leaf module (Artefacts, Changes, CLI, Hooks, Parser, Query, ReconcilerInterface, Reconciliation, CodeReconciler) has no chevron tab. Only Cairn (system) and Kernel (container) have one.
- Clicking Kernel's chevron: the glyph rotates smoothly from 90° to 0° (or vice versa) over ≤200ms linear. No frame-snap, no overshoot. Kernel's children hide/show in sync with the rotation completion.
- The chevron's fill color is a neutral paper tone (sampled against the node body, they match). The glyph color is a muted ink. Neither matches `cairn-accent` nor any `cairn-severity-*` hue.
- Clicking Kernel's chevron while Kernel is *not* selected: Kernel does NOT become selected. The detail panel does NOT open. Edges do NOT redraw in accent. Only the expansion state changes.
- Clicking Kernel's chevron while Kernel *is* selected: Kernel stays selected, panel stays open, only the expansion state changes. The chevron does not act as a deselect.

---

## 6. Close-and-re-open is state-clean

### (a) Rule / intended behavior

Closing the detail panel resets the state that drives panel content: the selected-node identifier clears, the artefact index resets, any loaded artefact payloads are discarded. Opening the panel on a new node — by click, by keyboard selection, by any path — re-seeds from scratch: the fresh node's identifier, a fresh artefact index at zero, a fresh fetch of that node's data.

The user-facing effect is that every panel-open is a clean render. No content from the previous node ever "bleeds" into the next view. A user clicking Artefacts (which has a finding) then closing and clicking CLI (which has no finding) must see no finding in the CLI panel, even for a frame. The close verb must actually clear, not merely hide.

Close paths covered: the close `×` button, the Escape key, a click on canvas background. All three clear state identically — they are the same verb with different triggers, not three separate code paths with different semantics.

### (b) What the critique saw

Iter-2's state hygiene is mostly right — the critique's rotation test (Cairn → Kernel → Parser → Artefacts with close cycles) passed for panel content and reported "no state leak between swaps." But the re-fit math after close still misbehaved, which hints that *some* state is surviving close-and-reopen in ways it should not. The critique flagged this as the same defect-class as the layer-nav bug: "added the hide logic but didn't verify the cascade." Belt-and-braces: clear everything the panel reads from, not just the DOM representation.

### (c) Implementation-visible success signal

- Click a node with a finding (Artefacts shows `CAIRN_CONTRACT_MISSING`). Close the panel. Click a leaf with no finding (CLI). The panel shows CLI's content with no FINDINGS block. Not even for a render frame. No stale severity glyph.
- Same sequence but observed via the artefact counter: on a multi-artefact node, advance to layer 2, close the panel, open an unflagged leaf. The layer-nav is hidden (per delta #3), so the previous "2 / N" counter cannot be re-exposed when it becomes applicable again — the state behind the nav is reset, not merely its visibility.
- Rapid clicking across Cairn → close → Kernel → close → Parser → close → Artefacts → close → CLI produces no intermediate frame where the panel shows data from a prior node. Each open is a fresh read.

---

## Do not change

Every item below comes from iter-1 or landed cleanly in iter-2. They must survive iter-3 unchanged. If a delta above appears to contradict any of these, this document is wrong and the list below wins.

- The warm-neutral palette. `cairn-paper` grounds, `cairn-ink` body, `cairn-accent` burnt-ochre signal, three pigment-warm severity tones. No system blue. No pastels. No gradients.
- **Mono for all identifiers.** Node IDs, panel IDs, paths, contract names, kind eyebrows, schema version, generated timestamp, layer counters when rendered. An ID set in sans is a defect.
- **Zero border-radius** on nodes, panels, topbar, zoom strip segments, artefact cards, finding tabs, chevron tabs. `cairn-radius-1` (2px) is reserved for focus-ring softening only.
- **Zero box-shadow** anywhere. Depth comes from `cairn-paper-sunk` inset tone and border weight alone.
- **Ochre accent used only for selection and focus.** Not on chevrons, not on hover, not on secondary controls, not on the layer-nav buttons when they render.
- **Zoom-strip geometry** from iter-1: three connected segments (`−`, `+`, `⌾`), mono glyphs, 1px hairline dividers, outer rule, no background fill, no pill rounding.
- **Escape closes the panel.** Iter-2's keydown listener passed the critique's sweep — keep it.
- **Chevron on parent nodes only.** Leaves do not render the tab. (Sharpened in delta #5 above; the underlying behaviour from iter-2 is preserved.)
- **Grid reflow on panel close.** The shell's `grid-template-columns` collapses the 380px panel column on close and restores it on reopen, instantly. The canvas reclaims the width. Delta #2 builds on top of this — it does not replace it.
- **Motion policy:** ≤200ms linear only. No easings with overshoot. No shimmers. No load-in fades.
- **Detail panel information order:** header → description → findings → layer-nav (when shown) → artefact-cards → metadata tail. Empty blocks hidden. Delta #4 enforces this for the remaining sections that iter-2 missed.

## Opinion Statement

Iter-3 MUST feel like iter-1 with the buttons finally believed. Every affordance the UI offers should do what its glyph says, every region that announces itself with a label should have something to say, every Reset should land on the frame the user is actually looking at. Trust is built from repetition without surprise — clicking the same node twice produces the same panel, closing the panel twice produces the same empty grid, pressing Reset after any sequence of interactions produces the same fit. This is NOT where we add motion, pigment, or new controls. If at any point implementation introduces a new color, a new easing curve, or a new piece of chrome, it is drifting away from the direction — revert, re-read iter-1, and try again with less.
