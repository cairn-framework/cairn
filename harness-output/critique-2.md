# Iteration 2 Evaluation — Cairn Graph Explorer

Evaluator has never seen src/. All observations are from the rendered page at http://127.0.0.1:3737/ on tab 1121946586 at OS-clamped 1477×1084 viewport (target was 1440×900 but Chrome will not shrink further). Dark mode is in force (`prefers-color-scheme: dark`) and dark tokens resolve cleanly.

A note about tooling: the `claude-in-chrome` `left_click` automation does not appear to fire click handlers on several of this page's controls (the topbar `Reset`, `Zoom in`, `Zoom out` buttons, and the node cards). A raw `MouseEvent('click')` dispatched against the same DOM target works correctly, and `left_click_drag` pan also works. So every functional verdict below that depends on a click is validated by DOM-dispatched click; interactions that inherently need real pointer events (pan via drag, finger-touch) are validated by the automation drag. Real human users are not expected to hit the automation quirk.

## Fix-Verification Checklist

| # | Delta | Result | Evidence |
|---|-------|--------|----------|
| 1 | Fit-to-content-bbox on load (48px inset, all nodes visible, graph fills most of canvas) | ✗ FAIL | On fresh navigation, `.graph` style.transform is `translate(0px, 0px) scale(1)` — the fit-transform does NOT engage at page load. Nodes render at natural 2105×385 coordinates inside a 1477-wide viewport with `overflow: hidden`, so ReconcilerInterface is clipped mid-word and both Reconciliation and CodeReconciler are off-screen right. The fit math DOES run later (on resize or on panel close), but the user's first frame is broken. Screenshot ss_87366gktx shows 8 modules visible, Reconciler* clipped, ~55% of canvas below is dead dot-grid. |
| 2 | Reset view animates to fit over ≤200ms linear | ✗ FAIL (tooling-tainted but also logically broken) | When invoked via DOM synth click the handler fires and the transform snaps back to the fit value over the 160ms linear transition on `.graph`. Transition rule is `transform 0.16s linear` ✓. HOWEVER Reset computed from a post-close state produced `scale(0.467)` where the expected fit is ~0.655 — the recompute pipeline occasionally lands on a wrong scale after a panel-close cycle. Combined with Delta #1 (no fit at load), the user's experience is: "Reset button does not reliably return to a composed fit frame." |
| 3 | Layer-nav hidden on ≤1-layer nodes | ✗ FAIL | The `hidden` attribute IS set (`layerNavHiddenAttr: true`), but the CSS rule `.layer-nav { display: grid }` OVERRIDES the browser default `[hidden] { display: none }` because of specificity. The element computes `display: grid`, `visibility: visible`, is 316×31px, and still renders the text `← BACK 0 / 0 NEXT →` on every single node (confirmed on Cairn, Kernel, Artefacts via DOM probes; confirmed visually in screenshot ss_2856ns0b3). Same defect as iter-1 — design intent not achieved. |
| 4 | Close-panel grid reflow (380px column disappears, canvas reclaims width) | ✓ PASS (structurally) / ✗ FAIL (composition) | `getComputedStyle(.shell).gridTemplateColumns` correctly switches from `1097px 380px` → `1477px` on close. The column DOES collapse. But the re-fit triggered by the wider viewport produces a scale of `0.467` instead of the expected `0.655` — the graph shrinks when the canvas grows. Wrong direction. |
| 5 | Esc closes panel; focus ring on close `×` on open | ✓ PASS | Opening Cairn's panel sets `.panel-close` outline to `rgb(228 153 88)` (ochre accent) immediately; outline fades to transparent within ~500ms. Pressing Escape with the panel open sets `detail-panel.hidden = true`. Panel reopens cleanly on next node click. |
| 6 | No literal "collapse" word; 16×16 chevron tab on SYSTEM / CONTAINER only | ✓ PASS | Every node body probed with `/\bcollapse\b/i` returns false. Only `Cairn` (SYSTEM) and `Kernel` (CONTAINER) carry a `.node-chevron-tab` element with glyph `›` and `title="Collapse"` (tooltip — the only valid place the word may appear). All 9 leaf modules (Artefacts, Changes, CLI, Hooks, Parser, Query, ReconcilerInterface, Reconciliation, CodeReconciler) have no chevron element. Clicking Kernel's chevron hides its 8 kernel.* children (confirmed visually screenshot ss_6803kizzo — Cairn and Kernel remain, only CodeReconciler which is `cairn.reconcilers.code` not `cairn.kernel.*` correctly persists). |
| 7 | Sweep polish: hover pointer cursor, :active fill on zoom press, drag grabbing cursor, no orphan ARTEFACTS header | ✗ PARTIAL | No `.artefacts`-prefixed empty header observed (no fixture nodes have artefacts, so test is degenerate). BUT the detail-panel renders an orphan `FINDINGS` section header with a hairline rule even when `findings.length === 0` (Cairn has 0 findings, section still renders — see screenshot ss_2856ns0b3). This IS the orphan-header class of defect. Cursor and :active states not meaningfully verifiable through the scripted sweep — best-effort pass on CSS inspection. |

**Landed cleanly: 2 of 7 (#5, #6).**
**Landed partially: 0.**
**Failed: 5 (#1, #2, #3, #4 composition-side, #7 orphan FINDINGS).**

## Full Interaction Sweep

| Step | Result | Detail |
|------|--------|--------|
| (a) Zoom in ×3, out ×3, Reset | PARTIAL | `.click()` on Zoom in stacks scale 0.645 → 0.741 → 0.853 → 0.981 (×1.15³ ✓). Zoom out 3× floors back to 0.645. Reset snaps to fit ✓. But see #2 — sometimes Reset lands at 0.467 after a panel-close perturbation. |
| (b) Drag-pan left/right/up/down | PASS | Real-pointer `left_click_drag` at scale 0.853 shifts translate (-203,315) → (-433,86). Pan clamps to 0 when scale == fitScale (spec-correct). |
| (c) System → container → leaf → flagged node click rotation (Cairn → Kernel → Parser → Artefacts) | PASS (content) / FAIL (layer-nav) | Each click populates panel-name, panel-id, description correctly. No state leak between swaps. Artefacts shows its `CAIRN_CONTRACT_MISSING` finding in the panel. Every single one of the four nodes also renders a spurious "← BACK 0 / 0 NEXT →" row — Delta #3 regression. |
| (d) Close rotation (A → close → B → close → C) | PASS | Panel closes cleanly each time. Reopening on a different node replaces all content. No stale data. But the post-close re-fit sometimes lands on `scale(0.467)` (see Delta #4). |
| (e) Esc / canvas-click / close-button all close the panel | PASS | Esc handled by document keydown listener. Click on canvas background hides the panel (confirmed when `name.dispatchEvent(click)` → open, then `viewport.dispatchEvent(click)` → close). Close-button click works. No double-fire. |
| (f) Layer-nav with ≥2 artefacts | NOT REACHABLE | `/api/node/X` probed against every node in graph; every single node returns `decisions=0, todos=0, research=0, reviews=0, changes=0`. Fixture contains no multi-artefact node, so the Back/Next handler cannot be exercised against real data. Spec §3 says hide this block whenever layers ≤ 1 — meaning in this fixture, the block should NEVER render. It renders everywhere. |
| (g) Mobile 390×844 bottom sheet | NOT LIVE VERIFIABLE | Chrome OS minimum-width clamp prevents the window from shrinking below ~1477px on this machine. Static CSS inspection: `@media (max-width: 480px)` block includes `.detail-panel { position: fixed; inset: 30vh 0 0 0; transform: translateY(100%); transition: transform var(--cairn-dur-sheet); }`, `.sheet-handle { 36×4px, centered }`, topbar collapses to 2 rows, zoom strip moves to grid-area 1/2. Structurally compliant; cannot be confirmed live this iteration. |
| (h) Dark mode tokens | PASS | `--cairn-paper: hsl(32 8% 11%)`, `--cairn-ink: hsl(38 20% 92%)`, `--cairn-accent: hsl(28 72% 62%)`, `--cairn-rule: hsl(32 8% 22%)`, `--cairn-severity-warn: hsl(38 44% 62%)`. Warm-graphite, warm ochre, restrained severity. Spec §aesthetic compliant. |
| (i) Console scrape `error|warn|Uncaught` across full flow | PASS | Zero messages returned from `read_console_messages` across load, zoom, pan, click, close, reopen cycle. |

## Zone Evaluations

### Zone: Topbar / Masthead
**Scores:** DQ: 7 | O: 7 | Craft: 6 | Func: 6

Masthead composition (Cairn mark + hairline + sub-mark + mono meta strip + connected 3-segment zoom) still reads well. Typography is right: `Cairn` in sans 28px, `Graph Explorer` in 14px muted, center strip is all-mono with middle-dot separators. Zoom-button glyphs (−, +, ◎) are crisp.

**Issues:**
- Reset is not consistently idempotent. A Reset-click after a close-reopen cycle can produce `scale(0.467)` instead of the canonical fit. This is the same class of defect iter-1 had (no-op Reset) in a different form — it now sometimes does the wrong thing rather than nothing.
- Zoom buttons are functional but the `:active` press-fill spec wasn't observable; reported as not implemented.

### Zone: Graph Canvas
**Scores:** DQ: 5 | O: 6 | Craft: 4 | Func: 4

The typographic node vocabulary (square, zero-radius, kind eyebrow + name + mono ID, finding corner tab with `!` glyph, optional chevron corner tab on containers) is still an eight-or-better design idea — it continues to read like a well-set technical journal rather than a SaaS dashboard. The new chevron tab in the top-right corner of Cairn and Kernel is clean and mirrors the finding-tab grammar precisely. Edges unlabeled, ownership solid, dependency dashed, selected-node edges redraw ochre.

**Issues (hard caps):**
- Initial-load fit is broken. With `translate(0 0) scale(1)` the natural 2105×385 content overflows the 1477-wide viewport. ReconcilerInterface, Reconciliation, CodeReconciler are clipped / off-screen and a fresh-load user has no immediate visual cue that they exist. This alone caps Craft at 4 and Functionality at 4.
- After any re-fit (window resize, panel close) the math can land on `scale(0.467)` where the correct value is `0.655`. The graph visibly shrinks when the viewport grows.
- Leaf module row is a dense horizontal bar of 9 tight rectangles — fine for 9, but the vertical half of the canvas below the nodes stays permanently empty in every observed state. The composition is still bottom-heavy-looking even when the math does center.

### Zone: Detail Panel
**Scores:** DQ: 7 | O: 7 | Craft: 5 | Func: 5

Header block (MODULE/SYSTEM/CONTAINER eyebrow + name + mono ID + close ×) lands. Description prose is clean. Findings block on Artefacts shows a single `CAIRN_CONTRACT_MISSING` row with the severity margin mark and the missing contract path in mono. Metadata tail (TAGS, STATE, PATHS, CONTRACTS, DEPENDENTS, FILES) renders only non-empty blocks.

**Issues (hard caps):**
- "← BACK 0 / 0 NEXT →" renders on every single node. The intended `hidden` attribute is applied but CSS `display: grid` on `.layer-nav` overrides the browser default `[hidden] { display: none }` because of specificity. This is the single most visible spec violation on the page — the same design note iter-1 failed, failed again here, as a CSS-specificity bug on top of the same kind of "added the hide logic but didn't verify the rule cascade" oversight. Hard-caps Func at 5 and Craft at 5.
- FINDINGS header renders as an orphan section when there are no findings (seen on Cairn, which has 0 findings — a horizontal rule and a FINDINGS label appear below the description anyway). Delta #7 claimed empty-block suppression; it doesn't land on findings.
- Close `×` focus ring on panel open works and fades correctly — the positive delta here is real.

### Zone: Meta / Schema strip
**Scores:** DQ: 7 | O: 7 | Craft: 7 | Func: 8

Unchanged from iter-1. `Cairn · schema v1 · generated 2026-04-19T12:48:02Z` in mono with middle-dot separators. The one subtle loss from iter-1 to iter-2 is that the timestamp changed from `T11:27:14Z` to `T12:48:02Z` — which is correct behavior (per-build timestamp) but I'd expect a cache-bust-safe static test-double at some point. Non-blocking.

### Zone: Mobile bottom sheet
**Scores:** DQ: 6 | O: 6 | Craft: 6 | Func: 6

Not live-verifiable (OS window clamp). CSS rules are structurally right: `position: fixed; inset: 30vh 0 0 0; transform: translateY(100%)` flipped to `translateY(0)` when open; `36×4px` sheet handle; overscroll-contain on panel-inner. Scoring held at 6 since I cannot independently confirm behavior vs iter-1.

## Adversarial Gate

| Check | Result | Detail |
|-------|--------|--------|
| Horizontal overflow at 1477 | PASS | `documentElement.scrollWidth === clientWidth === 1477`. No page-level horizontal scroll. |
| Visible node clipping at 1477 on first load | **FAIL** | ReconcilerInterface clipped mid-word; Reconciliation & CodeReconciler off-canvas. Traced to Delta #1 fit not engaging. |
| Text readability | PASS | All probed text ≥ 12px; contrast of mono ID against paper-sunk within WCAG AA. |
| Layer-nav spec compliance | **FAIL** | Renders `0 / 0` on every node. |
| Reset idempotence | **FAIL** | Post-close Reset lands at `scale(0.467)` rather than `0.655`. |
| Fit recompute on panel close | **FAIL** | Triggers wrong fit scale (panel-close reflow produces smaller-than-expected graph). |
| Dark mode rendering | PASS | Tokens resolve; all computed paint colors are from the tokens. |
| Console clean | PASS | Zero errors / warnings across full flow. |
| Escape closes panel | PASS | Keyboard handler works. |
| Container chevron replaces "collapse" text | PASS | Clean. |

**Gate impact:** Graph-canvas zone floored at Craft 4 / Func 4 (fit-to-viewport broken). Detail-panel zone floored at Craft 5 / Func 5 (layer-nav always renders `0 / 0`).

## Whole-Page Scores

| Criterion | Raw Score | Zone Floor | Final | vs iter-1 |
|-----------|-----------|------------|-------|-----------|
| Design Quality | 7 | — | **6** | −1 ↓ |
| Originality | 7 | — | **7** | 0 → |
| Craft | 5 | 4 (graph — no fit on load) | **4** | −2 ↓ |
| Functionality | 5 | 4 (graph — clipped nodes) | **4** | −1 ↓ |

Design Quality lowered by one because the initial-load compositional breakage (clipped nodes on top of an otherwise strong typography+layout identity) makes the page's first impression contradict its own reading-room intent. The craft-on-generic trap in reverse: high originality of vocabulary is not redeeming a broken fundamental.

**Weighted average** (weights: design 0.3, originality 0.2, craft 0.25, functionality 0.25):
`0.3·6 + 0.2·7 + 0.25·4 + 0.25·4 = 1.80 + 1.40 + 1.00 + 1.00 = 5.20`

Weighted average: **5.20** (vs iter-1 6.50) — a **regression of 1.30**.

## Decision: **REFINE** (strong)

SHIP thresholds: all four ≥ 7.0, weighted ≥ 7.2. We are below on every axis except Originality (tied with iter-1).

The decision border with PIVOT: iter-1 held Design=7, Originality=7, Craft=6, Func=5 (weighted 6.50). Iter-2 regressed on three of the four axes. Does that mean the aesthetic direction is wrong? No — Originality held at 7, the chevron affordance and the panel-close focus ring are genuine positive deltas, and the failures are all bugs in the same class (fit-transform pipeline + CSS specificity). The design description is NOT fighting the aesthetic; the implementation introduced regressions while fixing other things. So **REFINE**, with sharper emphasis on verification this time.

## What Works (iter-2 positive deltas over iter-1)

1. The literal word "collapse" is gone from node bodies. Chevron tab is present on SYSTEM/CONTAINER only, correctly absent from leaf modules, clicking it hides the container's children, glyph flips right-to-down per state. Clean execution of Delta #6.
2. Esc closes the panel. Close-`×` focus ring on panel-open lands with the specified ochre color and fades within ~500ms. Clean execution of Delta #5.
3. Shell grid DOES reflow on panel close (1097px 380px → 1477px) — structural half of Delta #4 landed.

## What Fails (must fix to SHIP)

1. **Fit-to-content-bbox does not engage at initial page load.** Users land on a broken frame with 3 modules clipped. This is the single most-visible defect on the page and the one a first-time viewer cannot miss.
2. **Layer-nav still renders `← BACK 0 / 0 NEXT →` on every node.** The `hidden` attribute is set but CSS specificity overrides the browser default. The fix is a one-line CSS rule: `.layer-nav[hidden] { display: none !important; }` or (cleaner) change the `.layer-nav` base rule so its display is applied only when NOT hidden.
3. **Post-close Reset/re-fit lands on wrong scale** (observed `0.467` vs expected `~0.655`). Whatever preserves-user-scale / re-reads-viewport logic is computing the wrong rectangle. The spec text in §1(c) is precise — re-read `viewRect` at transition time. Some path in the pipeline is reading a stale or wrong rectangle.
4. **Orphan FINDINGS section header** when node has zero findings. Delta #7 claimed this was covered; it isn't for this kind.
5. Some evaluator-tooling interactions against the topbar buttons fail to dispatch click events. This might indicate the zoom/reset handlers are attached to `mousedown`-style custom listeners rather than straightforward `click`. Not a user-facing bug but an implementation fragility worth reviewing.

## Fix Priorities for Iteration 3 (ordered by impact)

1. **Run the fit-transform on `DOMContentLoaded` AND `load` AND `window.resize`, verify the initial transform is never `translate(0, 0) scale(1)` after the first frame.** Add a `requestAnimationFrame` pass after the node layout step so the fit re-measures after layout is committed.
2. **Fix layer-nav hide.** One-line CSS rule: `.layer-nav[hidden] { display: none !important; }` (or restructure the rule cascade so `[hidden]` wins without `!important`).
3. **Audit the re-fit math after panel close.** Something is feeding the fit function the old 1097px canvas width while the new width is 1477px — or the opposite. Whatever it is, the produced fitScale is ~71% of the expected one. `0.467 / 0.655 ≈ 0.712 ≈ 1097 / 1477 × ?` — looks like the rectangle math is using container `clientWidth` from a stale layout pass.
4. **Suppress the FINDINGS section header when findings array is empty.** Applies the same "hide empty blocks" logic the metadata tail already applies.
5. Verify initial-load, zoom, close-reopen, resize cycles compose well at both 1280 and 1920 viewport widths — this iteration introduced regressions that were invisible at the viewport used during implementation.
