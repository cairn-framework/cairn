# OQ2: edge-label render verification

Open question from cross-check integration: does `app.js` already render edge
labels on selection in the graph-explorer? This stronghold answers that with
file:line citations and converts the answer into an LOC estimate for Bundle C
sub-component C5.b.

## Method

Files read in full or in the relevant range:

- `/Users/george/repos/cairn/src/ui_assets/app.js` (1474 lines; targeted reads
  around 285-300, 535-700, 1260-1300, 1430-1455).
- `/Users/george/repos/cairn/src/ui_assets/style.css` (54 KB; targeted reads
  around 730-770, plus full grep for `edge|label|verb|relation`).
- `/Users/george/repos/cairn/openspec/specs/graph-explorer/spec.md` (172 lines,
  full read; the `Dependency edge labels` scenario sits at lines 55-59).
- `/Users/george/repos/cairn/src/ui/serialise.rs` (full).
- `/Users/george/repos/cairn/src/ui/api.rs` (lines 25-50; the JSON emitter for
  `/api/graph` edges).
- `/Users/george/repos/cairn/src/map/query.rs` (lines 40-90; `GraphEdgeResponse`
  type definition).
- `/Users/george/repos/cairn/src/map/graph.rs` (lines 70-90; `EdgeRef`).
- `/Users/george/repos/cairn/src/ui_assets/api/graph` (the static fixture
  shipped under `ui_assets/api/`, distinct from the live endpoint).

Searches run:

- `grep -n -i "edge|label|link|relation|verb"` in `app.js` (returned 70 hits).
- `grep -n "text|<text"` in `app.js` (every SVG `<text>` element is inside a
  node group; **none** sit inside the edge-render branch).
- `grep -n "hover|Hover|hovered|onHover"` in `app.js`.
- `grep -n "selection|onSelect|setSelection|selectedId"` in `app.js`.
- `grep -n "edge|label|verb|relation"` in `style.css` (six edge-related rules:
  `.edge`, `.edge.dependency`, `.edge.traced`, `.edge.dimmed`, plus three
  unrelated `.label` selectors for chain-banner labels).
- `git log --oneline -20 -- src/ui_assets/app.js` (four commits total, latest
  `403582e webui: rebuild graph explorer as v2 app`, 2026-04-21).

## Findings

### Rendering library and pattern

Custom SVG rendered via Preact + htm tagged templates (the `html` template tag
is imported from `vendor/`). The graph explorer is **not** D3, **not** vis.js,
**not** Cytoscape. It is a hand-rolled SVG `<g>` viewport with a manual
pan/zoom transform.

The main canvas component is `GraphCanvas` at `app.js:537-694`. Edges are drawn
inside the transformed `<g>` at `app.js:660-667`:

```
${ownershipEdges.map((e, i) => html`
  <path key=${`o-${i}`} class=${clsx("edge", isTraced(e) && "traced", isDimmed(e) && "dimmed")}
        d=${e.d}/>
`)}
${dependencyEdges.map((e, i) => html`
  <path key=${`d-${i}`} class=${clsx("edge dependency", isTraced(e) && "traced", isDimmed(e) && "dimmed")}
        d=${e.d}/>
`)}
```

Both kinds use the same path geometry function `ownershipPath(from, to)` at
`app.js:292-299`, which produces a cubic Bezier. The dependency variant just
gets a CSS dasharray.

### Current label rendering behaviour

**Edges render no label, ever.** Concrete evidence:

1. The edge JSX above is a single `<path>` element. No sibling `<text>`, no
   `<title>`, no `<textPath>`, no foreignObject. The map projects every
   `GraphEdgeResponse` to `{ ...e, from, to, d: ownershipPath(...) }` at
   `app.js:567-572` and `app.js:580-586`. The original `description` field
   that the wire format carries (see next section) is preserved on the object
   via the spread but is never read in the render branch.

2. `grep -n "description" app.js` returns **only** node-description usages
   (`node.description` at lines 780, 851, 1057, 1058): every hit is inside a
   node-detail or inspector path, never inside the edge-render branch.

3. CSS has no `.edge-label`, `.edge text`, no rule that hides-or-shows label
   text. The full edge styling block at `style.css:734-761` is opacity, stroke,
   stroke-width, stroke-dasharray. There is no display-toggling rule, no
   `:hover` rule, no `.selected` rule on edges.

4. The hover-trace behaviour at `app.js:636-643` does change edge appearance
   (`traced` adds 0.5px stroke and switches to `--prov-1` colour, `dimmed`
   drops opacity to 0.2), but this is a stroke-only treatment. No text gets
   inserted.

5. Selection-driven highlighting on edges does **not** exist at all. Trace the
   wiring: `App` at `app.js:1267` keeps `selectionId` state; `GraphCanvas` at
   `app.js:1437-1445` is passed `selection={ id: selectionId }` and
   `edgeTrace=${hoveredId}`: note the divergence. The canvas's
   `isTraced`/`isDimmed` predicates at `app.js:636-643` consult `edgeTrace`,
   which is sourced exclusively from `hoveredId` (line 1444). Selection state
   never flows into edge styling, only into the inspector panel and the node
   `selected` prop.

The data field that *would* drive a label, if a render call existed, is
`edge.description` (a string field on `GraphEdgeResponse`).

### Spec requirement

From `openspec/specs/graph-explorer/spec.md:55-59`:

> #### Scenario: Dependency edge labels
>
> - **GIVEN** a dependency edge with a label (e.g., "reads user records")
> - **WHEN** the user selects either the source or target node
> - **THEN** the edge highlights and its label becomes visible

Spec semantics:

- "selects": selection-driven, not hover-driven.
- "either the source or target node": the trigger is selection of a node
  incident to the edge.
- "highlights": already satisfied for hover; **not** wired to selection.
- "its label becomes visible": labels must render (under that condition).

The wire shape is provisioned: `GraphEdgeResponse.description` exists on every
edge per `src/map/query.rs:55-64` and is serialised verbatim by the
`/api/graph` endpoint at `src/ui/api.rs:31-45` as
`{from, to, kind, description}`. So the data path is already in place.

### Gap analysis

**Absent**, on two dimensions:

1. **Rendering**: zero edge-label rendering exists. Edges are pure `<path>`
   strokes.
2. **Trigger wiring**: even highlight (which would meet the first half of the
   THEN clause) is wired to hover, not selection. Spec says "selects". Hover-
   only highlight already misses the spec literally.

The `description` data **is** plumbed end-to-end through the wire format and
arrives intact in the `dependencyEdges` map step, so the implementation gap is
purely client-side: text rendering plus a selection→edgeTrace bridge.

### Adversarial pressure-test results

I considered each false-positive route a "no-op" verdict could hide behind:

- **Conditional render gated on a flag/toggle?** No. Searched
  `grep -i "showLabel|showLabels|labels|EDGE_LABELS|toggle|featureFlag"`: no
  hits relevant to edges. Edge labels are not behind a flag; they simply do
  not exist.
- **CSS-only tooltip on hover (e.g., `title` attribute)?** No. There is no
  `title=` attribute on the edge `<path>` elements, no `<title>` child, no
  `data-tooltip`. CSS contains no `.edge[title]:hover::after { content: }`
  pattern.
- **Separate code path for edges-from-selected vs all-edges?** No. There is
  exactly one render of ownership edges at `app.js:660-663` and exactly one
  render of dependency edges at `app.js:664-667`. Selection state is not
  threaded into either map step. The only edge-mutating predicates,
  `isTraced` and `isDimmed`, both consult `edgeTrace` (sourced from
  `hoveredId`).
- **Recent commit changed behaviour?** No. `app.js` has only four commits
  total (the canvas code post-dates the v2 rebuild on 2026-04-21, commit
  `403582e webui: rebuild graph explorer as v2 app`). The current state in HEAD
  is the canonical one. There is no half-merged feature.
- **Vendor library with edge-label support?** No. The `vendor/` dir holds
  Preact + htm only. No graph library is loaded that could render labels
  implicitly.
- **Demo fixture lying about the wire format?** Watch out for this one. The
  static file `src/ui_assets/api/graph` (lines shown earlier) uses a different
  edge shape: `{from, to, kind: "contract"}` with no `description`. That
  fixture is dead/legacy: the live endpoint at `src/ui/api.rs:31-45` writes
  `description` unconditionally. Bundle C work should not be misled into
  thinking the wire format lacks the field.
- **The hover->selection assertion: could selection actually map to
  `edgeTrace`?** No. Line 1444 reads `edgeTrace=${hoveredId}` literally. There
  is no compositing of selection and hover into a unified trace ID.

All adversarial routes ruled out. The verdict holds.

## Decision

- **Verdict**: full build (within the local definition of "full", small but
  not no-op, not a one-line defect fix).
- **Reasoning**: spec requires (a) selection-triggered highlight and (b)
  label rendering on incident-to-selection edges. (a) requires re-wiring
  `edgeTrace` to compose selection plus hover (or duplicating the predicate
  set for selection). (b) requires inserting an SVG `<text>` element per
  rendered edge, computing a midpoint along the cubic Bezier, applying
  conditional visibility (only when traced), and adding CSS for the new
  element (font, fill, paint-order/stroke for legibility against background,
  opacity transition matching the existing `var(--dur-quick)` pattern).
  Nothing about this is exotic, but it is genuinely missing, it is **not** a
  confirm-and-ship verification.
- **Confidence**: high. Direct file:line evidence of zero `<text>` in the edge
  branch, zero `description` reads in the canvas, hover-only `edgeTrace`
  source, and a clean four-commit history with no in-flight work to muddy the
  picture.
- **LOC estimate for C5.b under this verdict**: **40-70 LOC**, distributed
  roughly as:
  - `app.js`: a path-midpoint helper (4-6 LOC), an `<text>` element per edge
    map step (2 elements x ~6-8 LOC each = 12-16 LOC), and the
    selection→edgeTrace bridge or an `isIncident(selection)` predicate (8-12
    LOC). Subtotal ~25-35 LOC of JS.
  - `style.css`: an `.edge-label` rule, `.edge-label.visible`, `.edge.traced
    + .edge-label`, font + paint-order + opacity transition. Subtotal ~10-20
    LOC of CSS.
  - Optional but spec-aligned: a unit/integration sanity test that the edge
    description survives the wire round-trip (the data-path is already
    covered in Rust; a `cargo test` exists; an additional UI snapshot is
    probably not warranted at this phase). Subtotal 0-15 LOC if added.

This puts C5.b at the upper-end of the cross-check's 30-50 LOC envelope, or
slightly above. If a lighter "highlight-on-selection only, no text element"
half-shipping is acceptable to the spec author, it drops to ~12-18 LOC. But
the spec literally says "label becomes visible," so the text element must
ship; partial implementation would leave the scenario uncovered.

## What this changes for the integrated plan

Bundle C's LOC estimate should be set at the **upper bound of the original
range** (closer to 700 than 400) because C5.b is a definite small-build, not a
no-op. Sequencing-wise, C5.b does not gate anything else in Bundle C: the
`description` field is already on the wire, so no upstream Rust change is
required, and the bridge from selection to highlight is local to
`GraphCanvas`. Ship C5.b as a discrete commit inside Bundle C with the
"selection-driven highlight + text-render on incident edges" scope explicit in
the task description, citing `spec.md:55-59` as the gating contract.
