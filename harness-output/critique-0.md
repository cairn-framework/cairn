# Iteration 0 Evaluation — Cairn Graph Explorer (baseline)

The evaluator has never seen the source. All observations are from the rendered page at http://127.0.0.1:3737/ at 1440×900, plus a narrow-viewport simulation and direct DOM/API probes.

## Adversarial Gate

| Check | Result | Details |
|-------|--------|---------|
| Viewport boundary (1440px) | FAIL | Graph canvas lays modules in a single horizontal row extending to x≈2118. The graph-region clips at x≈1097. The rightmost 3 modules (ReconcilerInterface, Reconciliation, CodeReconciler) are entirely off-screen with no pan, no zoom-to-fit, no horizontal scroll inside the region. ~1021px of content is unreachable by default. |
| Viewport boundary (390px) | FAIL (by design inspection) | Only one responsive breakpoint exists (@max-width 860px), and it stacks the panel below the graph via `display:block`. There is no 390-specific bottom-sheet or full-width overlay treatment — the panel just drops beneath the clipped graph. The spec asks for a true mobile collapse; the implementation is a single stack rule. |
| Text readability | PASS (with caveats) | No sub-12px text observed. Node IDs render in Inter sans-serif rather than mono, so `cairn.kernel.artefacts` looks like body copy, not a reference identifier — the spec explicitly wants monospace for IDs and paths. Hierarchy is flat: the eyebrow "MODULE", the heading "Artefacts", and the ID are all similar in weight/size differences that feel template. |
| Interaction completeness | FAIL | (a) Zoom in / Zoom out / Reset are **no-ops** — the graph's transform stays `none` before and after clicks. (b) Layer-nav Back / Next are **no-ops** — counter stays stuck at `0 / 0` regardless of selected node. (c) Clicking a node fires TWO unrelated behaviors at once (collapse children AND select), with no visible signal distinguishing them. (d) There is no drag-to-pan on the canvas. (e) No filter toggles exist anywhere in the DOM — the whole `--include-todos / --include-research / --include-reviews / --include-deprecated-decisions / --include-changes` control surface from spec §5 is missing. |
| Overflow stress test | FAIL | Graph-region `overflow: hidden` silently clips the 3 rightmost modules. No horizontal scrollbar, no fade edge, no "3 more →" indicator. Users cannot tell content exists off-screen. |
| Console on load / click / layer-advance / filter | PASS | Zero errors or warnings in the console across load, node-click (including collapse toggle), zoom-button clicks, and layer-nav clicks. The code doesn't fail loudly — it just does nothing. |

**Gate impact:** Graph zone capped at **Craft 5 / Functionality 4** due to the clipped-nodes defect. Topbar zone capped at **Functionality 4** because 3 of its 3 buttons are non-functional. Detail-panel zone capped at **Functionality 5** because layer-nav is non-functional. Overall page Craft and Functionality are floored by these.

## Zone Evaluations

### Zone: Topbar (0, 0, 1477 × 89)
**Scores:** DQ: 4 | O: 3 | Craft: 5 | Func: 4

**Strengths:** The "CAIRN" eyebrow in a muted blue-grey over a large black "Graph Explorer" is legible. Buttons are outlined, not filled — consistent with "no loud CTAs".

**Issues:**
- Extremely generic SaaS header: small-caps eyebrow + big bold product name + 3 outlined button pills in the top-right. This is the default "app header" pattern on 100 dashboards. Nothing identifies this as a Cairn tool versus a Linear, Notion, or Vercel dashboard.
- "Graph Explorer" is a 40-ish-pixel tall black heading, heavier than needed, with no tension against the rest of the page. Feels like a landing-page H1 dropped into an app chrome.
- Three zoom buttons are non-functional — clicking them yields no visible change to the graph. A toolbar where the toolbar doesn't work is a severe craft defect.
- Zero project metadata in the topbar: no project name ("cairn"), no schema version, no "generated at" timestamp. The user has no idea which project they are looking at.
- Button sizing is uniform; no weighting distinguishes "Reset" (rare) from "Zoom in/out" (frequent).

**Screenshot evidence:** iter-0-desktop (topbar strip).

### Zone: Graph Canvas (0, 89, 1097 × 996)
**Scores:** DQ: 5 | O: 5 | Craft: 4 | Func: 3

**Strengths:** The faint dot-grid background is a quiet, confident choice — it reads as "this is a canvas for graph work" without being loud. Edge routing uses curved lines and there is a light visual distinction between ownership (solid) and dependency (dashed) edges. The selected-node green fill signals "this is the active one" reasonably well.

**Issues:**
- **Off-screen content**: 3 of 9 module nodes sit past the right edge of the clipped graph-region. The only visible sign is that "Parser" is cut mid-word on the right. A user has no way to reach ReconcilerInterface, Reconciliation, or CodeReconciler from the default view — and since Zoom out is broken, they cannot reveal them by zooming either.
- **Identical-looking nodes**: the five leaf modules (Artefacts, Changes, CLI, Hooks, Parser, …) all have the SAME amber/tan border, SAME yellow `CAIRN_CONTRACT_MISSING` pill, SAME layout. Visual hierarchy collapses — the eye cannot distinguish them at a glance because every node shouts the same warning in the same color.
- **Badge typography**: `CAIRN_CONTRACT_MISSING` is uppercase sans-serif in a pale amber rounded pill. It looks like a stock Bootstrap warning badge dragged in — not the restrained severity treatment the spec asks for.
- **Edge labels are inconsistent**: only the Kernel→Artefacts edge has a text label ("owns"). The other 4 ownership edges from Kernel to siblings are unlabeled despite being the same relation. Half-labeled edges read as work-in-progress rather than intentional.
- **Node IDs are set in sans-serif** (Inter). IDs like `cairn.kernel.artefacts` should be monospace per the spec and per the fact that they ARE identifiers — setting them in a proportional body face erases their "this is a reference, not prose" meaning.
- **Node cards are rounded soft rectangles with subtle drop-shadows on hover-ish looking padding**. Spec explicitly rejects "rounded cards, soft shadows" as the SaaS dashboard antipattern. The nodes are exactly that shape.
- **Layout engine is naive**: every leaf module sits at y=539 in a single horizontal row regardless of viewport width. With 9 modules and 230px horizontal spacing, this was guaranteed to overflow. There is no wrap, no force-directed relaxation, no "fit to viewport" pass.
- **Click semantics are overloaded and fragile**: clicking Cairn or Kernel COLLAPSES children AND opens the detail panel in a single gesture, but clicking a leaf module only opens the panel. Re-clicking the root collapses it to a single "system · 2 hidden" card with no explicit way to expand other than a second click in the same spot — and "Reset" does not restore.

**Screenshot evidence:** iter-0-graph-zone (saved as zoom output 1 above).

### Zone: Detail Panel (1097, 89, 380 × 996)
**Scores:** DQ: 4 | O: 3 | Craft: 4 | Func: 5

**Strengths:** The eyebrow (kind) → title (name) → ID → description rhythm is a reasonable information order. The close button has a clear affordance.

**Issues:**
- **95% empty**: after the 4 content lines and the Back/0-0/Next row, ~800 vertical pixels are pure white. The panel is a hollow shell.
- **Layer-nav shows `0 / 0`** for every node. Back and Next buttons produce no change. The whole layer-navigation feature is wired into the UI but not wired into the data — non-functional UI chrome is worse than no UI chrome.
- **No findings**: the Artefacts node has a `CAIRN_CONTRACT_MISSING` finding from `/api/lint` for `cairn.kernel.artefacts`. The badge is rendered on the graph node but does NOT surface in the detail panel. A user who clicks a flagged node to learn WHY it's flagged sees… no explanation.
- **No artefact sections**: decisions, todos, research, reviews, changes — zero of these sections exist in the panel or elsewhere on the page. Spec §5 (artefact panels) is simply not implemented.
- **No files / paths / contracts / tags / dependents / depends-on** — all fields promised by the `/api/node/<id>` contract are missing from the render.
- **Typography is flat**: "Artefacts" and "cairn.kernel.artefacts" have only subtle weight difference; nothing signals "this is an identifier, not another heading."
- **Close button is at the top right in the empty region above the content**, not paired with the heading — eye has to jump to find it. It could just as well sit inline with the eyebrow.
- **Panel has no visual boundary from the canvas** other than a 1px hairline — in the full-page screenshot the graph and the panel read as one empty field because the panel is empty and they share the same near-white background.

**Screenshot evidence:** iter-0-detail-zone (zoom output 2 above).

### Zone: Meta / Schema Footer (missing)
**Scores:** DQ: 1 | O: 1 | Craft: 0 | Func: 0

No footer exists at all. Spec §7 requires schema_version, project name, and generated_at to be visible. The `/api/meta` response contains this data, but nothing in the rendered DOM surfaces it. The `<body>` ends with the `<main.shell>` and an embedded script — no `<footer>`, no meta strip.

### Zone: Artefact Panels (missing)
**Scores:** DQ: 0 | O: 0 | Craft: 0 | Func: 0

No decisions panel, no todos panel, no research panel, no reviews panel, no changes panel, no filter toggles. Zero instances of the word "decision", "todo", "research", "review" or "change" in the whole rendered DOM (aside from the Changes module node). Spec §5 and §6 are entirely unbuilt.

## Whole-Page Scores

| Criterion | Raw Score | Zone Floor | Final Score | Trend |
|-----------|-----------|------------|-------------|-------|
| Design Quality | 4 | — (lowest zone DQ is 1 for missing footer; but missing-zone DQ is structural, not visual, so we apply floor from present zones: min=4) | **4** | — |
| Originality | 3 | — (min present zone O is 3) | **3** | — |
| Craft | 4 | 4 (graph zone) | **4** | — |
| Functionality | 4 | 3 (graph zone — off-screen modules) | **3** | — |
| **Weighted average** (2·DQ + 2·O + 1·Craft + 1·Func) / 6 | — | — | **3.50** | — |

Decision: **REFINE** (would be PIVOT if the implementation hadn't shown the working data path through `/api/graph` and `/api/lint` — the engine is fine; the UI on top of it is ~25% built.)

## What Works

- Backend contract is alive: `/api/meta`, `/api/graph`, and `/api/lint` all return structured data cleanly.
- Graph engine correctly wires ownership edges (solid arrows) and dependency edges (dashed curves) with directional arrowheads.
- Click-to-select correctly populates the detail panel with the selected node's kind, name, ID, and description.
- Container nodes (Kernel) render with a distinct green tint, so there IS at least one kind-dependent styling decision in place.
- Dot-grid background is a quietly tasteful canvas indicator — keep it.
- Typography choice in the body (Inter) is clean and readable (though wrong for IDs — see below).
- Console is silent on every interaction path tested.

## What Fails

1. **Three modules are literally unreachable** from the default desktop view (off-screen right, no pan, no zoom, no scroll).
2. **Toolbar buttons are decorative** — Zoom out, Zoom in, Reset all compute identical transforms. A user clicking them has no feedback that anything happened.
3. **Layer-nav is decorative** — Back / Next / 0-of-0 are rendered but not wired.
4. **Detail panel is a 4-line summary in a 996px-tall column.** The right half of the app is white space.
5. **Artefact panels do not exist** at all. This is ~60% of the product surface area from the spec.
6. **Filter toggles do not exist.** The entire `--include-*` UX from spec §5 is absent.
7. **Schema / meta footer missing.** No project name, no schema version, no timestamp anywhere on the page.
8. **Typography rule violated for identifiers**: node IDs and paths render in proportional sans-serif instead of monospace, erasing the "this is a reference token" signal.
9. **Visual redundancy at the module row**: 5+ identical amber-bordered cards with identical yellow `CAIRN_CONTRACT_MISSING` pills. The eye has nothing to rest on and cannot differentiate scale/severity across nodes.
10. **Generic topbar aesthetic**: eyebrow + big black H1 + three pill buttons is the "every dashboard" layout. Nothing identifies this as a Cairn tool for architects, not a Vercel or Linear chrome.

## Direction: REFINE (aggressively — near-rewrite of the surface)

The backend and the graph engine are sound. The visible UI is 25% of the spec at best, and what IS there reads as template SaaS. A refinement pass must:

## If Refining (ordered by impact)

1. **Build the missing structure first** (functionality > aesthetics):
   - A real meta/schema footer showing project name, schema version, generated timestamp in mono.
   - Artefact rail or bottom drawer containing the 5 sections (decisions, todos, research, reviews, changes), each with a visible toggle that actually filters its list.
   - Surface the lint findings for the selected node inside the detail panel (not just on the graph badge).
   - Wire Back / Next to actually walk the neighbourhood layers — and if no layers exist, hide the control instead of showing `0 / 0`.
   - Make Zoom in / out / Reset actually transform the graph (even just a CSS `transform: scale()` on `#graph` would be a start).
   - Fix the click semantics: separate the collapse affordance (e.g. a chevron on container nodes) from the select affordance (the whole card).

2. **Solve the graph overflow**:
   - Lay out leaf modules in a wrapped grid or a force-directed cluster instead of a single 9-column row.
   - Give the graph canvas a "fit to region" initial zoom so every node is visible at default load, with Zoom in to inspect detail.
   - Add horizontal pan (click-drag on empty canvas) so users can reach clipped content.

3. **Rebuild the visual identity to match spec "editorial precision with structural warmth"**:
   - Replace the rounded-corner, soft-shadow node cards with something more like a field of typographic rectangles — clear borders, almost no rounding, no shadow. Use border weight and color shifts to differentiate node kinds (system / container / module / reconciler), not just Kernel's one green variant.
   - Set all IDs, paths, file references, and contract names in a real monospace (JetBrains Mono, IBM Plex Mono, or the system `ui-monospace` stack). This alone will shift the feel from "SaaS dashboard" to "technical reference."
   - Replace the pale amber `CAIRN_CONTRACT_MISSING` pill with something closer to an editorial margin note — a small mono letterform + a severity glyph, not a Bootstrap-style pill.
   - Give the topbar a second row or inline meta: project name + schema version + generated timestamp in small mono next to the product name, so the chrome actually tells you WHAT you're looking at.
   - Make the detail panel dense: stack eyebrow, name (big), ID (mono, subdued), description (prose), then immediately follow with findings list, tags, paths, contracts, files, and neighbourhood (dependents / depends-on) as stacked labeled sections with tight vertical rhythm. Fill the 996px, don't leave it blank.
   - Reduce the heading weight on "Graph Explorer" — the app is the subject, not the title. A slightly smaller, lighter-weight product mark frees visual bandwidth for the actual content.

4. **Color discipline**:
   - Replace the default-looking amber node border + yellow badge combination. Pick a single quiet accent (one muted teal, one warm ink, one restrained ochre) and use it across selected node, linked edge, and finding severity consistently. The current state shows too many near-but-not-matching warm tones (rust border, amber badge) that don't read as one system.

## If Pivoting

Not recommended. The graph engine and API work. A pivot would throw away functional plumbing. Refine aggressively instead.

## Is the page functional enough to iterate on?

**Yes**. The server serves, the API returns, the graph lays out, the click path populates the panel. The implementation agent can reshape nodes, build new sections, and wire controls without fighting broken infrastructure. The work ahead is additive-and-restyling, not rescue-from-broken-state.
