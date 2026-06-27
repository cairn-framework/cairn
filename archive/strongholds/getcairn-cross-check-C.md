# Cross-check: Bundle C (graph-explorer salvage)

## Scope

Bundle C is the graph-explorer salvage cluster from the roadmap stronghold. Inputs from the refined batches name four sub-components attached to the bundle:

- **C4.b**: per-node "Prerequisite for / Enables" widget (salvaged from rejected causality pyramid; flagged as Batch D's highest-leverage flip).
- **C4.c**: re-center on any node affordance (salvaged from same rejected pyramid).
- **C5.b**: verb-labelled edges in the graph render (salvaged from Batch C's deferred mainstay/systemigram cluster; this is the verb-labels piece, not the systemigram visual).
- **C9-salvage**: uniform inspector card chrome (salvaged from Batch B's rejected flat 2-node schema).

Note one terminology drift in the dispatch brief: it framed "C5.b" as the *systemigram visual*. The roadmap-debate table is explicit that **C5.b is verb-labelled edges (ADOPT-NOW)**; the systemigram visual is **C5.d (DEFER)**. Bundle C as listed in the roadmap-debate table only contains C5.b. C5.d does not enter Bundle C; it remains deferred behind Phase 2.5 maturity. This cross-check treats Bundle C as **C4.b + C4.c + C5.b + C9-salvage**, which is what the unified verdict table actually says, and discusses C5.d only to confirm it stays out.

This cross-check tests Bundle C against the consolidated graph-explorer and query specs, looks for collisions with active phases, and answers two mandatory questions: the cflx-validate failure on `spec/graph-explorer`, and bundle coherence.

## Inputs read

- `/Users/george/repos/cairn/docs/strongholds/getcairn-refined-batch-D.md` (C4 salvage section).
- `/Users/george/repos/cairn/docs/strongholds/getcairn-refined-batch-C.md` (C5 verb-labels and systemigram).
- `/Users/george/repos/cairn/docs/strongholds/getcairn-refined-batch-B.md` (C9 uniform-chrome salvage).
- `/Users/george/repos/cairn/docs/strongholds/getcairn-roadmap-debate.md` (Bundle C scope).
- `/Users/george/repos/cairn/openspec/specs/graph-explorer/spec.md` (primary area, validate-failing).
- `/Users/george/repos/cairn/openspec/specs/query/spec.md` (neighbourhood/edge queries).
- Headers of `/Users/george/repos/cairn/openspec/changes/phase-{8,8.0,9,9.0,10,10.0}-*/proposal.md` (collision check).
- `/Users/george/repos/cairn/openspec/changes/archive/` (presence check; phase-2.5 archived).
- `/Users/george/repos/cairn/src/ui_assets/` glob and `app.js` keyword sweep for re-center / inspector / prerequisite-style affordances.
- `/Users/george/repos/cairn/openspec/conventions.md` section structure (to interpret what `cflx validate --strict` likely enforces on a consolidated spec).

## Findings

### F1: Graph-explorer spec is missing `## Purpose`; query spec has one

The two consolidated specs sit in the same `openspec/specs/<area>/spec.md` slot. Section structure compared:

- `openspec/specs/query/spec.md` opens with `# Query capability spec`, then `## Purpose` (3 paragraphs), then `## Requirements`. Validates.
- `openspec/specs/graph-explorer/spec.md` opens with `# Graph Explorer Capability Spec`, then jumps directly to `## Requirements` with **no `## Purpose` section**. This is the most likely cause of the image #65 validate failure. Every other consolidated spec in `openspec/specs/` that I'm aware of carries a `## Purpose`; cflx validate strict appears to require it.

This is a single-section omission, not a structural/terminology drift. The spec text is otherwise current (uses `map` not `ontology`, `blueprint` not `dsl`, query-API-versus-data-path framing matches the post-2.5/2.6 state of the world). The fix is approximately 80 to 120 words of new prose summarising the area, sitting between the H1 and `## Requirements`.

The "UI Maintenance Contract" requirement at the end of the file references "Phase 2.5", "Phase 3 temporal addendum", and "Phase 7 transport addendum" by name. Phase 2.5, 3, and 7 are all archived. Those scenarios are historical addenda that were *correct at the time of the consolidating archive*, but reading them now feels like a snapshot. They are not strictly broken (the scenarios still describe behaviour the UI should preserve) but the spec would read better with the addendum scenarios re-phrased as live invariants rather than future-tense addenda. This is a soft observation, not a validate-blocker.

### F2: Per-node detail surface already exists; widget is a section, not a new surface

The graph-explorer spec already defines a node detail panel (Requirement: "Node detail panel with artefact drill-down"). It opens on click, lists artefacts as expandable sections, supports next-layer navigation, and closes cleanly. C4.b's "Prerequisite for / Enables" widget is therefore **a new section inside an existing panel**, not a new surface. Implementation-wise this is a sub-100-line JS/CSS change in `src/ui_assets/app.js` (the inspector renders at app.js:847, decision-detail variant at 987, empty state at 1051; the panel render-loop is around line 1398). No spec scaffold for "node detail panel" needs to be invented; one new requirement scenario under the existing requirement covers it.

### F3: Query spec supports neighbourhood and direction-based queries; not edge-type-filtered explicitly

The query spec (`openspec/specs/query/spec.md`) declares:
- `get` (by ID or name).
- `neighbourhood` (returns inbound + outbound edge entries with connected node metadata).
- `dependents` and `depends` (direction-based: in-edges vs out-edges).
- `order` (dependency tiers).

For C4.b's "Prerequisite for" / "Enables" lists, the operation needed is **direction-based traversal over the authority chain** (decision → blueprint → contract → code) plus possibly ownership edges. The existing `dependents` / `depends` pair is direction-based and structurally close, but neither query is explicitly edge-type-filtered. Two implementation paths:

1. The widget reads `neighbourhood` output and filters client-side by edge type. Cheapest; no spec change to query.
2. The query spec grows a `--filter <edge_type>` or scope arg. Requires a new requirement scenario.

Path 1 is the right starting form (graph sizes today are small; client-side filtering is fine) and matches the spec's "query layer is data path, UI does presentation" stance. Path 2 becomes worth doing only if the data set grows enough that filtering on the wire matters, which is post-Phase-9 territory.

### F4: No precedent for derived per-node widgets, but the architecture invites them

The graph-explorer spec's `Forward-compatible artefact rendering` scenario commits to a generic artefact template (title + frontmatter + body) for unknown artefact types. That is the only mention of derivation; otherwise the panel renders declared artefacts. The "Prerequisite for / Enables" widget is the **first derived widget** (it computes from edges, not from a declared artefact) the spec would acquire. This is not a problem; it is a small extension of the existing surface, but the spec language should be precise that the widget is derived rather than artefact-shaped, so it doesn't get mistaken for a new artefact type.

### F5: No re-center affordance exists in current webui

`src/ui_assets/app.js` keyword sweep for `centerNode`, `center_on`, `recenter`, `re-center` returns zero hits. The inspector exists (lines 847, 987, 1051, 1398, 1446-1447) but the viewport is not re-centerable on click. C4.c is therefore a **genuinely new interaction**, not a no-op. Implementation is small (50 to 100 lines: a click handler that updates the layout's center coordinate or the SVG viewBox transform; no state needs to leave the page). It does not require URL-parameter persistence in v1: re-centering is an in-session viewport operation. URL persistence is a follow-on if anyone asks.

### F6: Edge labels are already in the data; only the render is missing for C5.b

Per Batch C analysis section 3, "Edges already carry a description string per spec section 7. The UI just needs to render it." Confirmed against the graph-explorer spec: the `Dependency edge labels` scenario already specifies that on selection of either source or target node, the edge highlights and **its label becomes visible**. So C5.b is even smaller than the per-node widgets, it is a **render-already-specified, implementation-may-or-may-not-fully-honour-it** sub-component. The cross-check should verify whether the current `app.js` already shows edge labels on selection or only highlights the edge. If labels already render, C5.b collapses to nothing or to a polish tweak. If labels don't render, C5.b is implementing an existing scenario, which is a defect-fix, not a new feature.

### F7: Inspector has bespoke per-type rendering today; uniform-chrome refactor is real work but small

The keyword sweep showed `inspector` appears in three named variants: generic (847), `decision-detail` (987), and `empty-inspector` (1051). That confirms Batch B's claim that inspector rendering is bespoke per artefact type today. Uniform-chrome (consistent header/footer with type-specific middle slots) is a 200-to-400-line refactor. The cflx-side risk is *visual regression* on existing screens, not behavioural change.

### F8: No active phase touches graph-explorer

Active phase proposals:
- `phase-8-summariser`: drafts contracts/docstrings; no UI.
- `phase-8.0-tests`: pre-phase test wall; no UI.
- `phase-9-brownfield`: `cairn init --from-code`, `cairn refine`; CLI-shaped, no UI.
- `phase-9.0-tests`: pre-phase test wall; no UI.
- `phase-10-distribution`: LSP, plugin packaging, reconciler extension points; LSP touches editors, not the graph explorer webui.
- `phase-10.0-tests`: pre-phase test wall; no UI.

None mention `cairn ui`, `src/ui_assets`, the graph explorer, the inspector, or webui rendering. **Bundle C is collision-free with active phases.** It can land independently.

### F9: `phase-2.5-graph-explorer` is archived; spec consolidation produced the current `openspec/specs/graph-explorer/spec.md`

`openspec/changes/archive/phase-2.5-graph-explorer/` exists with `proposal.md`, `design.md`, `tasks.md`, `specs/`. The consolidated spec is the post-archive output. Per CLAUDE.md, archived phases are historical record and must not be rewritten. Bundle C therefore writes against the consolidated spec, never against the archived phase content.

### F10: C5.d (systemigram) is not in Bundle C

Re-confirming the dispatch-brief drift: the roadmap-debate's unified verdict table puts C5.d (systemigram visual render) in the **DEFER** column with no bundle assignment, gated on "Post Phase 2.5 graph explorer maturity". Bundle C contains C5.b (verb-labelled edges, ADOPT-NOW) and not C5.d. The dispatch brief's "C5.b: Systemigram visual" naming is incorrect against the source.

For completeness on the dispatch brief's actual systemigram questions: a systemigram would be a new render mode (today the spec implies a single force-directed-or-similar layout; no `--render-mode` flag exists). Curved labelled edges would be new edge-rendering capability. Batch C's "LATER" reasoning is not blocked-on-something-specific; it's blocked on **Phase 2.5 maturity** plus the framing risk that adding a second render shape competes with rather than supplements the existing surface. Treat as non-Bundle-C and move on.

### F11: UI Maintenance Contract requirement constrains what Bundle C must do, not what it must avoid

The graph-explorer spec's "UI Maintenance Contract" requirement says: when a phase modifies a `CairnResponse` shape, the phase must include a UI compatibility note. Bundle C does *not* modify `CairnResponse` (it adds derived UI widgets that read existing fields), but per the contract, Bundle C's acceptance criteria should explicitly state that no `CairnResponse` shape changes occur and existing rendering is preserved.

## Recommendations

1. Treat Bundle C as a single phase, scope **C4.b + C4.c + C5.b + C9-salvage**. Drop the "C5.b systemigram" framing from the dispatch brief; that was a label drift.

2. Fold the validate-failure fix into Bundle C's first commit. The fix is a `## Purpose` section in `openspec/specs/graph-explorer/spec.md`. ~80-120 words. Bundle C is the single phase most likely to touch this spec for legitimate reasons (it adds 1-3 new requirement scenarios for the salvaged sub-components anyway), so read-and-edit-once is genuinely cheaper than a separate maintenance phase. See D1 for full reasoning.

3. Implement C4.b as a derived widget inside the existing node-detail panel, reading `neighbourhood` output and filtering client-side by edge type. Add one new requirement scenario under the existing "Node detail panel with artefact drill-down" requirement: "Prerequisite for / Enables widget renders authority-chain edges as inbound/outbound lists." Do not invent a new artefact type.

4. Implement C4.c as a viewport primitive: click-to-recenter with no URL persistence in v1. Add one new requirement scenario under "Render the structural graph": "Re-center on click changes layout center to the clicked node." URL persistence is a deferred follow-on.

5. Implement C5.b as a polish/defect on the existing `Dependency edge labels` scenario. Verify the label-on-selection behaviour against current `app.js`; if it already works, C5.b is a no-op confirmation; if not, it's a 30-50-line render fix to honour the already-specified scenario.

6. Implement C9-salvage as an inspector chrome refactor. Add one new requirement scenario: "Inspector renders all artefact types with consistent header/footer and type-specific middle slots." This is the largest sub-component (200-400 lines).

7. Confirm C5.d (systemigram) stays deferred. Note in Bundle C's `out-of-scope` that the systemigram visual is explicitly not part of the bundle.

8. For C4.b open question 1 from Batch D ("does the widget read from blueprint edges only, or also from decision-attached obligations?"), default to **blueprint edges only** in v1 because Phase 2.5's edge model is what the consolidated spec already privileges. Decision-attached obligations are a Bundle D concern.

9. For C4.b spec-level precision: the spec should call the widget a *derived* surface, not an *artefact*, so future authors don't mistake it for a kernel-level addition. One sentence in the requirement scenario suffices.

10. Bundle C's acceptance criteria should explicitly include a "no `CairnResponse` shape changes; existing rendering preserved" line per the UI Maintenance Contract requirement.

## Decisions made (with reasoning)

### D1: Validate-failure resolution: fix-with-bundle

**Decision:** Bundle C fixes the missing `## Purpose` section in `openspec/specs/graph-explorer/spec.md` as part of its scope, in the same phase that adds the four salvage sub-components.

**Reasoning:** Three reasons converge.

First, the failure is small. A consolidated spec needs `## Purpose` between H1 and `## Requirements`, and graph-explorer is missing it. ~100 words of prose. The opportunity cost of *not* fixing it now is one extra phase scaffold for a single-paragraph edit, which is wasteful.

Second, Bundle C is the only phase in the visible roadmap that touches `openspec/specs/graph-explorer/spec.md` for legitimate scope reasons, it adds three or four requirement scenarios anyway. Read-and-edit-once on the spec file is strictly cheaper than two passes (one to fix the validate failure, one to add the salvage scenarios), since both passes go through the same review and the same cflx-validate gate.

Third, the validate failure currently *blocks* every other phase that wants to touch graph-explorer. Even Bundle D's eventual systemigram work would have to fix `## Purpose` before it could land. Discharging the fix in Bundle C means future phases find the spec already validate-clean. That is correct hygiene.

Risk of fixing within Bundle C: minimal. The `## Purpose` text is a summary of what the area does; that text doesn't need to coordinate with the salvage scenarios. The two changes are independent within the same file.

### D2: Bundle coherence: keep as a single bundle

**Decision:** Keep Bundle C as a single phase containing all four sub-components. Do not split.

**Reasoning:** Tested against the four bundle-coherence questions:

1. *Do they share infrastructure?* Yes. All four touch `src/ui_assets/app.js` (the inspector and graph render). All four read existing data without modifying `CairnResponse` shapes. All four can use the same UI Maintenance Contract acceptance line. C4.b and C4.c share the inspector-and-viewport call paths. C9-salvage is the deepest of the four (chrome refactor) but it's *the same file* that C4.b modifies for the widget addition. Splitting forces two passes through the same code paths.

2. *Are they testable as a unit?* Yes. The pre-phase-tests pattern (per phase-N.0-tests directories) would hold one pre-phase test file with a few `#[ignore = "awaits Bundle C"]` tests covering: Prerequisite/Enables widget renders, re-center moves viewport, edge label visible on selection, inspector chrome consistent across artefact types. The Maintenance Contract preservation check is a single browser-snapshot comparison. Single pre-phase, single phase, single archive.

3. *Could one slip without delaying the others?* The largest single sub-component (C9-salvage chrome refactor, 200-400 lines) is the most likely to slip on visual-regression issues. C4.b and C4.c are sub-100-line pure-additive features. C5.b is a 30-50-line confirmation-or-fix. If C9-salvage has trouble, the right move is to *defer C9-salvage out of Bundle C and ship the other three*, not to split the bundle into four phases up front. The asymmetry in size and risk argues for a single bundle with C9-salvage as the optional in-bundle rider.

4. *Is the bundle name "graph-explorer salvage" honest?* Mostly. C4.b/c/5.b are graph-explorer enhancements; C9-salvage is an inspector enhancement (the inspector lives inside the graph explorer surface). All four touch the same file. The name reads honestly. The only mild reframe worth making is to say *graph-explorer enrichments* rather than *salvage*, three of the four came from rejected larger candidates, and "salvage" carries a slightly defensive tone. The roadmap-debate uses the term anyway, so leaving it is fine.

Net: keep as one bundle. **The asymmetric C9-salvage size argues for declaring it the optional rider** so Bundle C ships even if chrome refactor slips, but that's an internal sequencing call, not a split.

### D3: Drop the "C5.b systemigram" framing from Bundle C

**Decision:** Do not include the systemigram visual (C5.d) in Bundle C. Bundle C's C5.b is verb-labelled edges, not the systemigram render mode.

**Reasoning:** The dispatch brief conflated C5.b and C5.d. The roadmap-debate unified verdict table is unambiguous: C5.b is verb-labelled edges (ADOPT-NOW, Bundle C), C5.d is systemigram visual (DEFER, no bundle, gated on Phase 2.5 maturity). Including a deferred sub-component in an ADOPT-NOW bundle would either inflate scope unnecessarily or commit the bundle to a render-mode decision that Batch C deliberately deferred. Keep them separate.

### D4: Default for C4.b's edge source: blueprint edges only in v1

**Decision:** The v1 widget reads from blueprint edges only. Decision-attached obligations are out of scope for Bundle C.

**Reasoning:** Batch D's open question 1 named the choice between blueprint-only and "blueprint plus decision-attached obligations." The spec privileges blueprint edges in the existing data path (`Graph endpoint uses explorer graph response` scenario). Decision-attached obligations are a richer authority-chain feature that intersects with Bundle D's stamping work and with possible kernel changes. Shipping the widget against blueprint edges first, with explicit "decision obligations not yet included" prose, gets the high-leverage UX win without coupling Bundle C to Bundle D's schedule. If decision obligations turn out to add real value, that's a follow-on extension, not a scope addition for Bundle C.

### D5: C9-salvage stays in Bundle C with declared optional-rider status

**Decision:** Include C9-salvage in Bundle C, but mark it explicitly as the optional-in-bundle rider that may be deferred if the chrome refactor's visual-regression risk surfaces during apply.

**Reasoning:** It belongs with the other three because it touches the same file (`app.js`) and shares the inspector code path. Its size (200-400 lines) and visual-regression risk profile (largest of the four) make it the natural candidate to drop if the bundle's apply phase pressures scope. Pre-emptively declaring it the rider keeps the bundle's intent honest and gives the apply agent permission to ship without it if needed, rather than silently cutting it or blocking the bundle.

The alternative: defer C9-salvage to its own future phase, is also reasonable but creates a third pass through inspector code (one for C4.b widget, one for chrome refactor, one for any post-Phase-9 inspector work). The single-bundle path with explicit rider status keeps the inspector pass count to one.

### D6: Re-center is local UI state, not URL-persisted in v1

**Decision:** C4.c re-centers the viewport in-session only. No URL parameter, no view-state persistence.

**Reasoning:** Batch D's analysis flagged this as an implementation choice. URL persistence is a real feature only when sharing-by-URL or back-button-re-state matters. Today the graph explorer is a single-user, in-session tool started via `cairn ui`. Adding URL persistence to v1 would be over-engineering. If a real share-link use case emerges, it's an additive follow-on (the URL parameter reads the centered node ID; the page restores center state on load).

## Open questions for next session

1. **Does `app.js` already render edge labels on selection?** F6 named this as the C5.b verification step. Confirms whether C5.b is a no-op confirmation (edges already render labels) or a 30-50-line defect-fix (the spec says they should but they don't). One quick browser test or one keyword sweep against the SVG render code resolves it.

2. **What's the cleanest spec language for "derived widget"?** The graph-explorer spec has no current vocabulary distinguishing derived UI surfaces from artefact-rendering surfaces. Bundle C introduces the first derived widget (Prerequisite/Enables). Worth a one-sentence definition near the top of the spec, but the exact wording should be drafted with the Bundle C apply agent rather than pre-emptively chosen here.

3. **Should the inspector chrome refactor (C9-salvage) split header/footer/middle into separate templates, or remain one consolidated template?** This is an implementation-detail question that affects the size of the refactor. The Batch B analysis described "consistent header, consistent footer, type-specific middle slots" but didn't commit to a code-level structure. Bundle C's design.md is the right place to land it.

4. **Does decision-attached obligations belong in a Bundle C v2, or in Bundle D?** D4 deferred the question to follow-on work. The honest answer depends on whether Bundle D's stamping schema produces an obligations field on decisions; if it does, the widget extension follows naturally. If not, a v2 of Bundle C carries it. Worth a marker in the Bundle D scoping for "potential consumer of Bundle C v2."

5. **Should the addendum scenarios in the UI Maintenance Contract requirement (Phase 2.5/3/7 named) be re-phrased as live invariants?** F1 named this as a soft observation. Not validate-blocking, but the spec would read cleaner. Not Bundle C's job; could be a separate one-line edit in a docs-only commit, or quietly absorbed into Bundle C if the apply agent has a few minutes.

## Recommended Bundle C final scope

| Sub-component | Status in Bundle C | Notes |
|---|---|---|
| **C4.b: Prerequisite for / Enables widget** | **Confirmed** | Derived widget inside existing node-detail panel. Reads `neighbourhood` output, client-side filters by edge type. Blueprint edges only in v1; decision-attached obligations deferred. ~100 LOC. One new requirement scenario in spec. |
| **C4.c: Re-center on any node** | **Confirmed** | Viewport primitive. In-session only; no URL persistence in v1. ~50-100 LOC. One new requirement scenario in spec. |
| **C5.b: Verb-labelled edges in render** | **Scope-clarified** | The roadmap-debate label, not the systemigram visual. Likely a no-op confirmation or sub-50-LOC defect-fix; verify whether labels already render on selection. No new spec scenario needed (existing `Dependency edge labels` scenario covers behaviour). |
| **C9-salvage: Uniform inspector card chrome** | **Confirmed as optional in-bundle rider** | 200-400 LOC chrome refactor. Largest visual-regression risk. Declared rider so Bundle C can ship without it if pressured. One new requirement scenario in spec. |
| **C5.d: Systemigram visual** | **Out of scope** | Stays deferred per roadmap-debate; gated on Phase 2.5 maturity. Dispatch brief's framing was a label drift. |

**Plus:** Bundle C's first commit fixes the `## Purpose` validate failure on `openspec/specs/graph-explorer/spec.md` (~100 words new prose). This is in-scope by D1.

**Plus (acceptance criteria addition):** Bundle C's accept gate explicitly declares no `CairnResponse` shape changes per the UI Maintenance Contract requirement.

**Phase-ID suggestion:** `phase-7.x-graph-explorer-followups` per the roadmap-debate, or whichever number is free in the orchestrator's slotting. Bundle C is collision-free with all active phases (8/8.0/9/9.0/10/10.0) per F8, so it can ship in parallel with anything currently in flight.

**Total scope estimate:** ~400 to 700 LOC across `src/ui_assets/app.js` and `style.css`, plus ~150 to 250 words of new spec prose (one Purpose section + 3 to 4 new requirement scenarios + acceptance criteria additions). Mid-small phase. One pre-phase tests directory (`phase-N.0-tests`-shaped) holds the failing assertions for the four sub-components.

**The honest one-line characterisation:** Bundle C is a single inspector-and-viewport pass that ships three small high-leverage UX additions, one chrome refactor, and a spec hygiene fix, against a graph-explorer surface that no active phase touches.
