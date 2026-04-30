# Roadmap-level synthesis: getcairn.dev adoption plan

## Methodology

Four parallel deep-debate agents (Batches A through D) refined per-candidate analyses for 14 candidates from getcairn.dev plus one salvage-derived candidate (C15, surfaced from Batch B's C10 analysis). This synthesis integrates across the four, building a sequencing dependency graph, surfacing bundling opportunities, and mapping each ADOPT/DEFER/RESEARCH item onto cairn's actual roadmap (active phases visible in `openspec/changes/`: `phase-8-summariser`, `phase-9-brownfield`, `phase-10-distribution`, plus their `.0-tests` pre-phases). The goal is a planning artefact for the next session, not implementation depth.

The roadmap shape that emerges is unexpectedly clean: a small foundation phase (graph capability + sidecar + queueing safety), three sub-components that ride along inside or alongside Phase 9 brownfield, a small dedicated workflow phase for verification states, and a concentrated cluster of UX/copy work that can ship as a pre-phase mini-effort independent of any roadmap blocker. The deferrals are honest: AI narrative, per-field provenance, and full export-into-provenance-chain wait on either Phase 9 outcomes or design calls that require artifacts not yet built.

## Final unified verdict table

| ID | Candidate (incl. sub-components) | Verdict | Phase / Slot | Bundle | Confidence |
|---|---|---|---|---|---|
| C1.a | `cflx-proposal` skill writes transcript as `research/genesis.md` | ADOPT-NOW | Pre-phase mini-effort (skill convention) | n/a | high |
| C1.b | `cflx interview` CLI + multi-round orchestration | DEFER | Inside Phase 9 (if elicitation needed) | Phase 9 | medium |
| C1.c | Confidence pill UI | REJECT | n/a | n/a | high |
| C1.d | Architecture-signals panel | DEFER | Phase 9+ (post-runner) | Phase 9 | medium |
| C2.a | Three-axis radar widget shape | REJECT | n/a | n/a | high |
| C2.b | Multi-dimensional rollup native to two-chain | RESEARCH | Pre-current design | n/a | medium |
| C2.c | Prose-nudge banner: templated copy + severity rendering + placement | ADOPT-NOW | UX foundation mini-phase | Bundle B (copy/findings UX) | high |
| C2.d | `Fix with AI` button | DEFER | Gated on webui write-surface direction | n/a | medium |
| C2.e | Per-node single completeness number | REJECT | n/a | n/a | high |
| C3.a | Whole-model Findings rollup panel + severity buckets | ADOPT-NOW | UX foundation mini-phase | Bundle B (copy/findings UX) | high |
| C3.b | `cflx check` CLI + shared-data-source contract | ADOPT-NOW | UX foundation mini-phase | Bundle B (copy/findings UX) | high |
| C3.c | Scope toggle (Entire Model vs This Node) | ADOPT-NOW | UX foundation mini-phase | Bundle B (copy/findings UX) | high |
| C3.d | Category filters (cairn-vocabulary categories) | RESEARCH | Pre-current (taxonomy decision) | n/a | medium |
| C3.e | Per-row Fix button | DEFER | Gated on webui write-surface | n/a | medium |
| C3.f | Re-run button + timestamp | DEFER | Gated on user demand | n/a | low |
| C3.g | Naming "Quality Check" / `COMPLETENESS` / `TRACEABILITY` | REJECT | n/a | n/a | high |
| C4.a | Pyramid layout / lanes / Knowledge Foundation tier | REJECT | n/a | n/a | high |
| C4.b | Per-node "Prerequisite for / Enables" widget | ADOPT-NOW | Phase 2.5 follow-up / UX foundation | Bundle C (graph-explorer follow-ups) | high |
| C4.c | Re-center on any node | ADOPT-NOW | Phase 2.5 follow-up / UX foundation | Bundle C (graph-explorer follow-ups) | high |
| C4.d | GAP tile pattern | RESEARCH | Pending C2.b rollup design | n/a | medium |
| C4.e | Re-frame on hinge view (cairn-native) | RESEARCH | Pre-current navigation design | n/a | low |
| C5.a | Islands query (`cairn neighbourhood --include-orphans` or similar) | ADOPT-NOW | AI-provenance foundation | Bundle A (AI-provenance foundation) | high |
| C5.b | Verb-labelled edges in graph render | ADOPT-NOW | Phase 2.5 follow-up | Bundle C (graph-explorer follow-ups) | high |
| C5.c | Mainstay-sentence generation | DEFER | Joint with C7 in post-Phase-9 phase | Bundle D (AI-narrative + stamping) | medium |
| C5.d | Systemigram visual render | DEFER | Post Phase 2.5 graph explorer maturity | n/a | low |
| C5.e | Layout / mainstayPath helpers as kernel data | REJECT | n/a | n/a | high |
| C6 | Per-change archive sidecar (`.cflx-trace.json`) + `cflx trace` | ADOPT-NOW | AI-provenance foundation | Bundle A (AI-provenance foundation) | high |
| C7.a | Bottom-of-artefact provenance block (schema) | DEFER | Joint with C5.c in post-Phase-9 phase | Bundle D (AI-narrative + stamping) | medium |
| C7.b | Inline `. ai .` token form | REJECT | n/a | n/a | high |
| C7.c | Line-range vs content-anchor tagging | RESEARCH | Long-pole question for Bundle D | n/a | medium |
| C7.d | Multi-state field flag (`ai_drafted` etc.) | RESEARCH | Pending real reviewer workflow data | n/a | low |
| C8.a | Suggested-edges format (change-shaped queue) | ADOPT-NOW | AI-provenance foundation | Bundle A (AI-provenance foundation) | high |
| C8.b | `cflx accept` untriaged-block safety check | ADOPT-NOW | AI-provenance foundation | Bundle A (AI-provenance foundation) | high |
| C8.c | Suggest engine itself | DEFER (milestone-gated) | Inside Phase 9 brownfield | Phase 9 | high |
| C8.d | Manual `cflx suggest-edges <node>` | DEFER | Post-Phase-9 if demand | n/a | low |
| C9 | Flat 2-node schema | REJECT | n/a | n/a | high |
| C9-salvage | Uniform inspector card chrome | ADOPT (later, low-priority) | Phase 2.5 follow-up | Bundle C (graph-explorer follow-ups) | medium |
| C10 | Closed 6-type aerospace requirement enum | REJECT | n/a | n/a | high |
| C11.a | 5-state enum (Draft/Planned/Passed/Failed/Blocked) | ADOPT-NOW | Small dedicated phase (e.g. `phase-7.5c-verification-states`) | Phase 7.5c (standalone) | high |
| C11.b | `#[cflx_planned(phase = N)]` proc-macro | ADOPT-NOW | Same phase | Phase 7.5c | high |
| C11.c | `cflx accept` gate logic per state | ADOPT-NOW | Same phase | Phase 7.5c | high |
| C11.d | Conventions.md §5 + AGENTS.md paired updates | ADOPT-NOW | Same phase | Phase 7.5c | high |
| C11.e | `Blocked` semantics + new error code | RESEARCH | Inside same phase, design pass first | Phase 7.5c | medium |
| C11.f | `cflx verifications --status` query surface | DEFER (LATER) | Follow-on phase | n/a | medium |
| C12.a | `cflx export --format json` | ADOPT-NOW | Pre-phase mini-effort or Phase 7.x | Bundle E (export starter) | high |
| C12.b | `cflx export --format md` (Markdown) | ADOPT-NOW (paired or close) | Same | Bundle E (export starter) | medium |
| C12.c | CSV export | DEFER (LATER) | Demand-gated | n/a | low |
| C12.d | PPTX/DOCX (Professional Export) | REJECT (probably permanent) | n/a | n/a | high |
| C12.e | "Assets stays in provenance chain" pattern | RESEARCH | Pre-current design pass | n/a | medium |
| C12.f | Webui settings-pane export UI | DEFER | Gated on webui write-surface | n/a | medium |
| C13.a | Empty-state component + sweep + cairn-voice copy | ADOPT-NOW | UX foundation mini-phase | Bundle B (copy/findings UX) | high |
| C13.b | CLI parallel empty-state copy | ADOPT-NOW | UX foundation mini-phase | Bundle B (copy/findings UX) | high |
| C13.c | CLI-handoff CTAs (copy-pasteable command strings) | ADOPT-NOW | UX foundation mini-phase | Bundle B (copy/findings UX) | high |
| C13.d | Centralised copy strings (single configurable location) | ADOPT-NOW | UX foundation mini-phase | Bundle B (copy/findings UX) | high |
| C13.e | Voice review checklist | ADOPT-NOW | UX foundation mini-phase | Bundle B (copy/findings UX) | high |
| C13.f | In-webui CTA actions | DEFER | Gated on webui write-surface | n/a | medium |
| C13.g | Custom illustrations beyond icons | DEFER (LATER) | Marketing/onboarding push gated | n/a | low |
| C14 | AI-only flexibility as domain-expandability strategy | REJECT | n/a | n/a | high |
| C14-spec | Quotable principle into spec.md / CLAUDE.md | ADOPT-NOW | Pre-phase mini-effort (docs) | Bundle F (identity confirmations) | high |
| C15 | Templated authoring scaffolds via spec §6 `artefact_types` | ADOPT (later) | Phase 9-adjacent (brownfield surfaces demand) | n/a | medium |

## Section 1: Dependency graph

The dependency graph below tracks only ADOPT and DEFER items (REJECT items have no successors). Arrows read "must precede" or "enables."

```
                              [Webui write-surface direction]
                              (open question 1, gates 4 sub-components)
                                        |
              +-------------------------+---------------------+
              v                         v                     v
        C2.d Fix btn             C3.e Fix-row btn         C13.f in-UI CTA
        (DEFER)                  (DEFER)                  (DEFER)
                                                          C12.f settings-pane
                                                          (DEFER)

  [Bundle A: AI-provenance foundation]
  C6 sidecar  --->  enables run-identity references in all later AI work
  C8.a queue  --->  enables C8.c engine (Phase 9)
  C8.b safety --->  must precede ANY AI-stamping mechanism
  C5.a islands -->  optional input to C8.c engine (suggest reconnect-edges)

  [Phase 9 brownfield] (existing in openspec/changes/phase-9-brownfield)
        |
        +-- consumes C8.a + C8.b (Bundle A wave 1)
        +-- consumes C6 (run identity in suggestions)
        +-- gates C1.b interview runner (if elicitation needed)
        +-- gates C8.c suggest engine
        +-- creates demand for C15 templated scaffolds

  [Bundle D: AI-narrative + stamping] (post-Phase-9)
  C7.c line-range/anchor research -- precedes -->  C7.a provenance block schema
  C6 sidecar -- precedes -->  C7.a (block references runs)
  C7.a -- precedes -->  C5.c mainstay sentence
  C5.c -- needs --> C7.a stamping (so AI sentence is honest)

  [Bundle B: UX foundation mini-phase]
  C13.d centralised copy strings (foundation) -- precedes/co-located -->
        C2.c prose-nudge templated copy
  C13.a empty-state component -- co-located --> C13.b/c/e voice work
  C3.b cflx check + shared-data-source contract -- precedes -->
        C3.a/c rollup panel & scope toggle (panel reads from cflx check)
  C3.d category taxonomy decision -- gates --> filter UI sub-component

  [Bundle C: graph-explorer follow-ups]
  Phase 2.5 graph explorer (already archived) -- enables -->
        C4.b prerequisite/enables widget
        C4.c re-center
        C5.b verb-labelled edges
        C9-salvage uniform card chrome
        C5.d systemigram (only when explorer matures further)

  [Phase 7.5c: verification states] (standalone)
  C11.a state enum + C11.b proc-macro + C11.c gate logic +
  C11.d paired conventions/AGENTS updates -- ship together -->
        replaces #[ignore = "awaits phase-N"] pattern
  C11.e Blocked-vs-Failed error-code research -- inside same phase

  [Bundle E: export starter]
  C12.a JSON export (no upstream deps) -- precedes/co-locates -->
        C12.b Markdown export
  C12.e Assets-in-provenance-chain pattern -- separate research --
        eventually feeds export-destination policy

  [Bundle F: identity confirmations to docs]
  C9 / C10 / C14 quotable principles -- write straight into
        spec.md §3 sub-section + CLAUDE.md "What to avoid" / new
        positive-form section
```

The single highest-leverage edge: **C8.b safety check must precede any AI-stamping mechanism**. If an AI-suggested edge or AI-drafted artefact lands without `cflx accept` having a structurally non-bypassable triage gate, cairn ships its own corruption surface. Batch C named this as the highest-risk pitfall across the AI family; Bundle A discharges it.

The second-highest-leverage edge: **C6 sidecar before any per-run reference**. Every later candidate that wants to point at "the run that produced this" needs a stable run identity. Shipping C6 first lets C7, C8 wave 2, C5 mainstay all reference the same trace records without inventing parallel identifiers.

## Section 2: Bundling recommendations

Six natural bundles emerge. Two of them (Bundle A, Bundle B) are the headline near-term work. Bundle D is the post-Phase-9 joint. Bundles C, E, F are smaller riders.

### Bundle A: AI-provenance foundation

**Members:** C6 (full sidecar + `cflx trace`), C8.a (suggested-edges format), C8.b (cflx accept untriaged-block check), C5.a (islands query).

**Rationale:** All four ship without AI dependencies and without artefact-schema bumps. They prepare the substrate (run identity, change-shaped queueing, traversal completeness, safety gate) without committing cairn to any AI-stamping decisions. Critical: this bundle's safety primitives must land *before* Phase 9's suggest engine, otherwise the engine ships into an unsafe accept gate.

**Suggested phase ID:** `phase-7.6-ai-provenance-foundation` (slotting between current 7.5b archive and the in-flight 8.0/9.0/10.0 test pre-phases). Could also be split into two phases if conventions.md §3 module-size pressure warrants.

**Out of scope:** any AI-output stamping mechanism (lives in Bundle D), the suggest engine itself (Phase 9).

### Bundle B: UX foundation mini-phase (copy + findings + empty states)

**Members:** C13.a/b/c/d/e (empty-state component, sweep, CLI parallel, CLI-handoff CTAs, centralised copy strings, voice review checklist), C2.c (prose-nudge banner: templated copy + severity rendering + placement), C3.a/b/c (rollup panel, `cflx check` CLI, shared-data-source contract, scope toggle).

**Rationale:** All three Batch A candidates share a single architectural dependency surfaced explicitly in the cluster-level observation: a centralised copy + structured finding feed. Implementing in the order C13 → C3 → C2, each later candidate inherits the copy-string-location discipline established by C13 and the finding-feed discipline established by C3, and C2 ships almost for free. Splitting them across phases would either duplicate the copy infrastructure or build the structured feed twice.

**Suggested phase ID:** `phase-7.7-ux-foundation` or a pre-current-phase mini-effort. Could ship in parallel with Bundle A since the only overlap is "both produce an artefact to render," which is a coordinator concern, not a code conflict.

**Internal sequencing inside the bundle:**
1. C13.d centralised copy strings (the foundation; one config file, design-system-adjacent).
2. C13.a/b/c empty-state component, sweep, CLI parallel.
3. C13.e voice review checklist (process layer; lives in `docs/design-system/README.md` voice section).
4. C3.b `cflx check` CLI + shared-data-source contract (forces single-source discipline before any rollup UI is built).
5. C3.a/c rollup panel + scope toggle (renders from cflx check output).
6. C2.c prose-nudge banner (reuses C13's copy strings and C3's finding feed).

**Out of scope:** webui write-surface-dependent items (C2.d, C3.e, C13.f). Those wait.

### Bundle C: Graph-explorer follow-ups

**Members:** C4.b (prerequisite/enables widget), C4.c (re-center on any node), C5.b (verb-labelled edges in render), C9-salvage (uniform card chrome).

**Rationale:** All four are graph-explorer enrichments that read existing data. Phase 2.5 (graph explorer) is already archived; these are post-archive enrichments that don't need a new phase scaffold. They could fold into a single `phase-7.x-graph-explorer-followups` mini-phase, or ride along with Bundle B as UX work.

**Suggested phase ID:** Could slot anywhere; lowest urgency of the bundles. Recommend deferring to whenever there's a UI-investment window.

### Bundle D: AI-narrative + stamping (post-Phase-9 joint)

**Members:** C7.a (bottom-of-artefact provenance block schema), C5.c (mainstay-sentence generation), with C7.c (line-range vs content-anchor research) as the long-pole prerequisite.

**Rationale:** Batch C's headline finding: C5 mainstay and C7 stamping unblock each other and have no purpose without each other. C5 needs C7 so its AI-generated sentence is honestly stamped; C7 needs C5 (or C8 wave 2) as its first non-toy consumer. Designing them in one phase avoids two artefact-schema iterations.

**Suggested phase ID:** `phase-(post-9)-ai-narrative-and-stamping` (intentionally vague timing; depends on Phase 9 outcomes).

**Pre-work:** C7.c line-range vs content-anchor decision must be settled before tasks can be sized.

### Bundle E: Export starter

**Members:** C12.a (`cflx export --format json`), C12.b (`cflx export --format md`).

**Rationale:** Small, mechanical, lifecycle-orthogonal. JSON export earns its keep on internal grounds alone (MCP integration, debug tooling). Markdown follows naturally. Together they're maybe a small phase or even a pre-phase mini-effort. Could ride along with Bundle A or B if scoped tightly.

**Suggested phase ID:** No dedicated phase needed; could be a small commit on its own merit, possibly attached to the next infrastructure-shaped phase.

### Bundle F: Identity confirmations to spec/CLAUDE.md

**Members:** Three quotable principles from Batch B's C9/C10/C14 analyses, plus a positive-form constraint about AI-at-authoring-vs-AI-at-reality boundary.

**Rationale:** Pure docs work; no code. Strengthens spec.md and CLAUDE.md against future architectural drift by making the rejections' positive-form principles portable.

**Suggested slot:** Pre-current mini-effort. One commit, two file edits.

### Bundles NOT recommended

- **Joining C11 with anything.** C11 is a small dedicated workflow phase (`phase-7.5c-verification-states`-shaped). It has no shared infrastructure with the other candidates and bundling it would dilute its tight scope.
- **Joining C12 with C8.a.** Both produce derived views, but they read from different data and ship for different audiences. Bundling would force a fake architectural connection.
- **Joining Bundle A with Bundle B.** Tempting because both could be "near-term mini-phases," but they share zero code and target different surfaces (CLI/safety vs UI/copy). Keep them separate phases that can ship in parallel.

## Section 3: Roadmap fit

Mapping each ADOPT/DEFER/RESEARCH item onto cairn's actual roadmap. Active phases per `openspec/changes/` glob: `phase-8-summariser`, `phase-9-brownfield`, `phase-10-distribution`, plus `phase-{8.0,9.0,10.0}-tests` pre-phases. Recently archived: through `phase-7.5b-cleansing-splits` (commit `66dcf16`).

| Candidate / sub-component | Slot | Why |
|---|---|---|
| C1.a (transcript-as-research) | Pre-phase mini-effort (skill convention) | No code change; extends `cflx-proposal` skill output. Lands as soon as the convention is decided. |
| C1.b (interview runner CLI) | Phase 9 brownfield | Phase 9 is the natural forcing function for elicitation; if not needed there, reject. |
| C2.b (multi-dim rollup native to two-chain) | Pre-current research | Needs to investigate whether a chain-balance widget exists in `src/ui_assets/app.js` today. |
| C2.c (prose-nudge banner) | Bundle B (`phase-7.7-ux-foundation`) | Shares centralised-copy infrastructure with C13. |
| C3.a/b/c (Findings rollup + cflx check) | Bundle B (`phase-7.7-ux-foundation`) | Shares structured-finding-feed infrastructure with C2.c. |
| C3.d (category filters) | Pre-current research → then Bundle B if data-ready | Depends on whether reconciler findings carry stable category field today. |
| C4.b/c (per-node widget + re-center) | Bundle C (`phase-7.x-graph-explorer-followups`) | Reads existing edges; rides existing graph explorer. |
| C5.a (islands query) | Bundle A (`phase-7.6-ai-provenance-foundation`) | Pure graph capability, no AI dependency, feeds C8 engine downstream. |
| C5.b (verb-labelled edges) | Bundle C (`phase-7.x-graph-explorer-followups`) | Edge data already exists per spec §7; UI just needs to render. |
| C5.c (mainstay sentence) | Bundle D (`phase-(post-9)-ai-narrative-and-stamping`) | Needs C7.a stamping; depends on C7.c research. |
| C6 (pipeline trace sidecar) | Bundle A (`phase-7.6-ai-provenance-foundation`) | Foundation for all later run-identity references. |
| C7.a (provenance block schema) | Bundle D (`phase-(post-9)-ai-narrative-and-stamping`) | First non-toy consumer is C5.c. |
| C7.c (line-range research) | Pre-Bundle-D research | Long-pole; must be settled before Bundle D tasks can be sized. |
| C8.a (suggested-edges format) | Bundle A (`phase-7.6-ai-provenance-foundation`) | Pure schema work, no AI dependency. |
| C8.b (cflx accept untriaged-block) | Bundle A (`phase-7.6-ai-provenance-foundation`) | Safety primitive that gates Phase 9 engine. |
| C8.c (suggest engine) | Phase 9 brownfield | Brownfield is the engine's natural use case. |
| C9-salvage (uniform card chrome) | Bundle C (`phase-7.x-graph-explorer-followups`) | UI portability win; lives in webui inspector. |
| C11.a-e (verification states) | New dedicated phase: `phase-7.5c-verification-states` | Standalone; replaces `#[ignore = "awaits phase-N"]`. Pairs with conventions/AGENTS updates. |
| C12.a (JSON export) | Bundle E (small phase or rider) | Lifecycle-orthogonal; small. |
| C12.b (Markdown export) | Bundle E | Pairs with JSON. |
| C12.e (Assets-in-provenance-chain) | Pre-Bundle-E research | Needs design pass on storage location and lifecycle. |
| C13 (full UX cluster) | Bundle B (`phase-7.7-ux-foundation`) | Foundation for centralised copy and voice discipline. |
| C14-spec (quotable principles) | Bundle F (pre-phase docs commit) | Pure docs; one commit. |
| C15 (templated authoring scaffolds) | Phase 9-adjacent | Brownfield will create demand for multiple contract templates. |

The phase mapping deliberately avoids inventing many new phase numbers. The four genuinely new slots are:

1. `phase-7.5c-verification-states` (standalone, small).
2. `phase-7.6-ai-provenance-foundation` (Bundle A).
3. `phase-7.7-ux-foundation` (Bundle B).
4. `phase-(post-9)-ai-narrative-and-stamping` (Bundle D, vague timing).

Bundles C, E, F either ride along with one of the above or land as small commits. Phases 8/9/10 absorb their natural sub-components (C1.b, C8.c, C15) without needing scope expansion.

## Section 4: Conflicts and contradictions

I tested each cross-batch pairing for direction conflicts. Three live tensions surfaced; none are decision-blocking, but each deserves an explicit call.

### Tension 1: C2.c prose-nudge copy vs C13.d centralised copy strings

**The pull:** Batch A C2 wants templated copy keyed by reconciler finding class (e.g., `findings.toml`). Batch A C13 wants centralised empty-state copy keyed by surface state (e.g., `copy.toml`).

**Resolution:** Same infrastructure with two top-level keyspaces. One file, two sections: `[empty-states]` (keyed by surface state) and `[findings]` (keyed by finding class). The Batch A cluster observation already calls this out: "if the implementation order is C13 → C3 → C2, each later candidate inherits the copy-string-location discipline." Confirming: **no real conflict**, just a sequencing requirement that Bundle B already encodes.

**Action:** Bundle B's internal sequencing (C13.d first, then C2.c last) handles this. The decision needed is the *file location* (`docs/design-system/copy.toml` vs `src/ui_assets/copy.json` vs new `docs/design-system/voice.md` per Batch A C13 Q2). Resolve before Bundle B starts.

### Tension 2: C5.c mainstay sentence richness vs C7.a minimal stamping

**The pull:** C5.c wants per-edge `verbPhrase`, `cards[]` of prose, and potentially per-edge metadata in the rendered narrative. C7.a wants a single bottom-of-artefact provenance block with line ranges (or content anchors), kept minimal so markdown tooling stays clean and interface-hash semantics aren't disturbed.

**Resolution:** These are at different levels of the data model. C5.c renders narrative from graph edges + node attributes; C7.a tags artefact field provenance. They can coexist because:

- C5.c's `verbPhrase` lives on **edges** (already supported in spec §7 edge labels).
- C7.a's stamping lives on **artefact fields** (bottom block per artefact).
- Neither is per-edge metadata in the contested sense.

Confirming: **no real conflict**, but Bundle D should design them together precisely so the load-bearing primitives (run-identity reference, per-field tag, line-range or anchor) don't get re-litigated between C5 and C7.

**Action:** When Bundle D scopes, write a joint design.md that names which primitive each candidate consumes. This avoids two schema iterations.

### Tension 3: C8.a queueing format vs C7.a stamping schema

**The pull:** Batch C cluster observation explicitly cautioned: "the *queue* (C8 wave 1 format) and the *stamping* (C7 bottom-block) are different infrastructures, not one. Build C8 wave 1 in Joint 1; build C7 stamping in Joint 2. Do not conflate them." But the matrix's cross-cutting finding 1 originally said "build the queue once and reuse" for C5/C7/C8 collectively.

**Resolution:** Batch C's refinement is correct and supersedes the matrix's early framing. The queue (delta-shaped suggested-edges file inside a change directory) and the stamping (bottom-block schema on artefacts) are different artefacts at different layers of the kernel:

- Queue lives in `openspec/changes/<change>/suggested-edges.<ext>` and consumes existing change-isolation primitive.
- Stamping lives in artefact frontmatter + bottom block; bumps each artefact-type schema once.

Confirming: **no real conflict at the implementation level**, but a real conflict at the matrix-level framing that Batch C correctly resolves. Bundle A builds the queue; Bundle D builds the stamping. Do not let "build the queue once" be misread as "build the stamping in the same phase as the queue."

**Action:** Document this distinction in the Bundle A and Bundle D proposal.md headers when those phases are drafted, so the next-session author doesn't relitigate.

### Non-conflicts (worth noting briefly)

- **C4.d GAP tile vs C2.b multi-dim rollup:** Both are RESEARCH on related territory (visual surfacing of unresolved/weak structure). They could share a design pass. Not a conflict, just a candidate for combined research.
- **C11 verification states vs phase-N.0-tests directories:** Batch D explicitly calls out non-duplication: "the state enum should NOT duplicate the phase-tests directory convention; it should *complement* it. Phase-tests directories declare future tests; the state enum on individual verifications tracks per-test status." Confirmed not a conflict; both coexist at different granularities.
- **C12 export vs cflx archive:** Batch D calls out: "`cflx export` reads from the same data the archive operation reads, but produces a different format." Single source of truth maintained. Not a conflict.

Net: there are no roadmap-level conflicts that block decision. The three tensions are sequencing/communication artifacts that the bundle structure already discharges, provided proposal.md headers in Bundle A and Bundle D preserve the distinctions.

## Section 5: Identity-confirming additions to spec/CLAUDE.md

Batch B extracted three positive-form principles from the C9/C10/C14 rejections. All three are quotable and short. Recommend adding them as a new spec.md sub-section or as a positive-form complement to CLAUDE.md's "What to avoid" list. Plus C15 as a new candidate emerging from Batch B's salvage analysis.

### Proposed spec.md addition: new sub-section §3.5 "Layer ordering of enforcement, configuration, and AI"

**Suggested placement:** End of section 3 (the two-chain section), before §3.4's current "Current-state authority" discussion (which becomes §3.6 by re-numbering, or insert as §3.4.1 if numbering churn is a concern).

**Proposed prose (no em-dashes, cairn voice):**

> ### 3.5 Layer ordering of enforcement, configuration, and AI
>
> Cairn's enforcement value lives in the deterministic, typed, two-chain primitives at the kernel layer. Flexibility is delivered above the kernel via templates, tags, project config, and queued AI assistance. The layer ordering is non-negotiable:
>
> - **Bottom: deterministic-typed.** Artefact types are obligation-bearing, not decorative. Each direct type's place in the provenance or authority chain determines what the kernel can enforce about it. Flattening the taxonomy collapses the obligations into labels, and labels are not enforceable.
> - **Middle: configurable-templated.** Cairn's authoring guidance is template-driven and tag-extensible, never enum-bound. The kernel ships generic types; projects compose domain vocabulary on top via templates (per `artefact_types` in §6) and tags. A closed enum would constrain cairn's domain scope at the kernel layer; templates and tags do not.
> - **Top: AI-assisted at authoring only.** Cairn extends to new domains by adding deterministic reconcilers, not by leaning on AI to normalise reality. The reality layer must produce a content-addressable fingerprint; without it, drift detection is impossible and the authority chain collapses to documentation. AI assistance lives at the authoring layer (drafting, suggesting, narrating). It does not substitute for the deterministic record at the reality layer.

This adds roughly 200 words to spec.md. It complements §3.1 (provenance), §3.2 (authority), §3.3 (hinge), §3.4 (current-state authority) by naming the *flexibility ceiling* the chains operate within.

### Proposed CLAUDE.md addition: new "What cairn is, positively" section

**Suggested placement:** Above or alongside "What to avoid." The "What to avoid" section is negative-space; this is positive-space and should sit in proximity.

**Proposed prose:**

> ## What cairn is, positively
>
> Three principles, complementary to the negative-space "What to avoid" list below:
>
> 1. **Typed artefacts encode obligations, not labels.** Each direct type (`contract`, `decision`, `todo`, `research`, `review`, `source`) has a different role in the two-chain topology. The kernel's enforcement value comes from those role differences. Treating types as decorative labels (or proposing a flat schema) is the same mistake as flattening the two chains into a six-layer stack.
> 2. **Authoring guidance is template-driven and tag-extensible, never closed-enum.** Domain-specific vocabulary belongs in project config (`artefact_types`) or in tag conventions, both of which are extensible. The kernel speaks taxonomy; the project speaks domain.
> 3. **AI assists authoring; AI does not substitute for the reconciler.** AI may propose edges, draft contracts, suggest narrative summaries, all reviewable through the change-isolation primitive. AI may not produce the deterministic reality fingerprint that drift detection compares against. The enforcement layer stays mechanically checkable.
>
> These three are the positive form of the rejections in "What to avoid." They are quoted in spec.md §3.5 with rationale.

This adds roughly 150 words. Together with the spec.md insert, the identity-confirming addition is one commit, two files, ~350 words.

### C15: Templated authoring scaffolds via spec §6 `artefact_types`

This emerged from Batch B's C10-salvage analysis but isn't in the original 14 candidates. Adding here as **C15**:

**Verdict:** ADOPT, milestone-gated on Phase 9 brownfield demand.

**Mechanism:** Ship multiple default templates for the code domain (`interface-contract.tmpl`, `invariant-contract.tmpl`, `data-contract.tmpl`, `dependency-contract.tmpl`, etc.), each registered via `artefact_types` config. Allow project config to register additional templates. Surface template choice as the first authoring step in `cflx propose` and equivalent CLI surfaces, mimicking the type-picker UX without importing a closed enum.

**Why milestone-gated:** Batch B Q1 honestly noted that current cairn users (bootstrap fixture + the codebase itself) don't show enough contract-shape variance to justify shipping more than one default template today. Phase 9 brownfield will bring real contract diversity; that's the natural forcing function.

**Slot:** Phase 9-adjacent or inside Phase 9 brownfield. Could be its own small follow-on phase if Phase 9 surfaces concrete template demand without time to ship in-phase.

**Confidence:** medium. The principle is high-confidence (templates not enums); the timing depends on Phase 9 outcomes.

## Section 6: When (ordered phase sequence)

Recommended ordering, with rationale per slot:

### Pre-current (no roadmap blocker, ship NOW)

1. **Bundle F: identity confirmations** (one commit; spec.md §3.5 + CLAUDE.md positive-form section). No code, no roadmap impact, sharpens decision-making for every later phase. Ship first, single commit.
2. **C1.a: extend `cflx-proposal` skill to write `research/genesis.md`** transcripts. Skill-level convention update; no kernel change. Ship as a small commit; confirms the provenance-chain genesis discipline before Phase 9 design begins.
3. **C2.b research:** quick sweep of `src/ui_assets/app.js` to determine whether a chain-balance widget exists today. One investigation, one note in Bundle B's design.md (or Bundle B precondition doc).
4. **C3.d research:** check whether reconciler findings carry a stable `category` field today. One investigation; output feeds Bundle B scope.
5. **C7.c research:** line-range vs content-anchor decision for Bundle D. Long-pole; start the design pass even if Bundle D itself is far out.

### Pre-current mini-phases (parallel-shippable)

6. **`phase-7.5c-verification-states`** (C11 cluster). Standalone, small, replaces `#[ignore = "awaits phase-N"]`. No dependencies on anything else in this analysis. Could ship in parallel with Bundle A or Bundle B; pick whichever has free orchestrator capacity.

### Active near-future (Bundle A and Bundle B, parallel-shippable)

7. **`phase-7.6-ai-provenance-foundation`** (Bundle A: C6 + C8.a + C8.b + C5.a). Highest-leverage near-term phase. Lays the safety primitives that Phase 9's suggest engine will land into. Must precede Phase 9 implementation, period.
8. **`phase-7.7-ux-foundation`** (Bundle B: C13 cluster + C2.c + C3.a-c). Parallel-shippable with Bundle A; no code overlap. Internal sequencing matters (C13.d first, C2.c last). High user-facing leverage; widens the audience.

### Roadmap-aligned (inside existing active phases)

9. **`phase-9-brownfield`** absorbs:
   - C1.b interview runner (if elicitation is needed; otherwise reject).
   - C8.c suggest engine (consumes Bundle A's queue + safety).
   - C15 templated authoring scaffolds (or as immediate follow-on).
10. **`phase-10-distribution`** is unaffected by this analysis (the distribution scope per the proposal.md is LSP/plugin/extension; no candidates land here).

### Far-future (post-Phase-9 or roadmap-shaped)

11. **`phase-(post-9)-ai-narrative-and-stamping`** (Bundle D: C7.a + C5.c). Consumes Bundle A's run-identity foundation and Phase 9's suggest engine maturity. Vague timing; depends on Phase 9 outcomes and on C7.c research being settled.
12. **`phase-7.x-graph-explorer-followups`** (Bundle C: C4.b + C4.c + C5.b + C9-salvage). Lowest urgency; ships when there's a UI-investment window. Could ride along with Bundle B if scoped tightly enough.
13. **Bundle E** (`cflx export --format json|md`). Could ship anywhere; small. Suggest attaching to whichever near-term infrastructure phase has space, or as a standalone small commit when momentum is right.

### Deferred until precondition unlocks

- **C2.d/C3.e/C13.f/C12.f**: gated on **webui write-surface direction** decision. This is the single most consequential open question for shipping the deferred items. Recommend a dedicated design pass to settle it: could be a research artefact written into `docs/strongholds/` rather than a phase.
- **C3.f Re-run button**: gated on user demand for manual re-run. File-watch refresh is the better default; revisit if a concrete user surface need emerges.
- **C13.g custom illustrations**: gated on a marketing/onboarding push surfacing concrete need.
- **C12.c CSV export**: gated on a concrete reviewer request.
- **C5.d systemigram visual**: gated on Phase 2.5 graph explorer maturity (already archived but maturity is multi-phase).
- **C12.e Assets-in-provenance-chain**: gated on a design pass.

## Section 7: How (smallest first commits per ADOPT-NOW)

For each ADOPT-NOW item, the smallest first commit and expected scope (lines/files/spec changes). Light touch only; implementation depth is next session's job.

### Bundle F (identity confirmations, ship first)

**Smallest first commit:** Edit `docs/spec.md` to add §3.5 (~200 words new prose). Edit `CLAUDE.md` to add "What cairn is, positively" section (~150 words). Cross-reference each from the other.

**Scope:** ~350 words across 2 files. No code. No tests. Review pass for cairn voice (em-dash check, plain-English check). One commit.

### C1.a (cflx-proposal skill writes genesis.md)

**Smallest first commit:** Update the `cflx-proposal` skill (location depends on skill registry) to emit transcripts to `openspec/changes/<phase>/research/genesis.md` when a proposal is drafted. Add convention note to AGENTS.md and possibly conventions.md.

**Scope:** Skill update + ~50 words of convention text in two docs. No kernel change. One commit.

### `phase-7.5c-verification-states`

**Smallest first commit:** New phase scaffold: `openspec/changes/phase-7.5c-verification-states/{proposal.md, design.md, tasks.md, specs/}`. Initial proposal.md drafts the 5-state enum, the `#[cflx_planned(phase = N)]` proc-macro shape, and the conventions.md/AGENTS.md update plan.

**Scope:** Phase scaffold (~500 words across 4 files), then implementation: a proc-macro crate or workspace member, `cflx accept` gate logic update (~100 lines Rust), state enum on verification artefacts (schema bump per conventions §3 state-versioning), conventions.md §5 + AGENTS.md line 25 paired updates, new error code in `openspec/registries/error-codes.md` for `Blocked` if the research lands. Total: maybe 500-800 LOC + ~1000 words of spec/docs. Mid-sized phase.

### `phase-7.6-ai-provenance-foundation` (Bundle A)

**Smallest first commit:** Phase scaffold + initial proposal.md committing to the four-piece scope. Then internal ordering:

1. **C6 sidecar (~200-400 LOC):** First implementation commit. cflx workflow integration to write `openspec/changes/archive/<phase>/.cflx-trace.json` per phase run. Schema with state-versioning header. `cflx trace <phase>` CLI surface (~100 LOC).
2. **C8.a suggested-edges format (~100-200 LOC):** Schema for `openspec/changes/<change>/suggested-edges.<ext>` (likely JSON or TOML). Per-edge fields: source, target, relation, confidence (optional), provenance reference (cflx run ID from C6). No engine yet.
3. **C8.b cflx accept untriaged-block (~50-100 LOC):** Gate check that fails accept if change directory contains untriaged suggested-edges. New error code in `error-codes.md`. Pre-phase tests should pin behaviour.
4. **C5.a islands query (~100-150 LOC):** New CLI subcommand or `--include-orphans` flag on existing `cairn neighbourhood`. Pure graph traversal. No AI dep.

**Scope:** ~500-850 LOC + spec/registries/conventions updates. Bundles cleanly into one phase.

### `phase-7.7-ux-foundation` (Bundle B)

**Smallest first commit:** Phase scaffold + initial proposal.md committing to the three-piece scope. Then internal ordering:

1. **C13.d centralised copy strings (~50-100 lines config + design-system doc update):** Pick file location (recommend `docs/design-system/copy.toml` or new `docs/design-system/voice.md`). Document conventions for keying.
2. **C13.a/b/c empty-state component (~200-400 lines CSS + HTML/JS in `docs/design-system/components.css` and `src/ui_assets/`):** Component variant + sweep of `src/ui_assets/app.js` empty states + CLI parallel copy.
3. **C13.e voice review checklist (~100 words doc):** Lives in `docs/design-system/README.md` voice section.
4. **C3.b cflx check + shared-data-source contract (~150-300 LOC):** New CLI subcommand. JSON output mode optional (`--json`). No gate semantics. Inspection only.
5. **C3.a/c rollup panel + scope toggle (~300-500 lines JS/CSS in `src/ui_assets/`):** Panel that consumes `cflx check` output (or its in-memory equivalent). Severity bucket UI keyed off existing tokens.
6. **C2.c prose-nudge banner (~150-300 lines JS/CSS):** Reuses C13.d copy file (new section `[findings]`) + C3 finding stream. Banner placement at top of node-detail panel.

**Scope:** ~1000-1700 LOC + ~500 words of design-system/spec docs. Larger phase than Bundle A but parallel-shippable.

### Bundle E (export starter, opportunistic)

**Smallest first commit:** New `cflx export --format json` command (~100-200 LOC). Reads same in-memory data the archive operation reads. Outputs to stdout or `--output <path>`.

**Followup commit:** `cflx export --format md` (~100-150 LOC). Reuses the JSON serialisation as input, renders Markdown.

**Scope:** ~250-400 LOC total. No phase scaffold needed; ships as small commits in any near-term phase that has space.

### Bundle C riders (opportunistic, post-Phase-2.5-maturity)

**C4.b/c smallest commits:** Per-node "Prerequisite for / Enables" widget reads from existing edges (~100-200 lines JS/CSS in `src/ui_assets/app.js`); re-center on any node is a viewport primitive (~50-100 lines). Both are pure UI work.

**C5.b verb-labelled edges:** UI-only render update (~30-50 lines) since edge labels already exist in spec §7.

**C9-salvage uniform card chrome:** Inspector refactor (~200-400 lines) for consistent header/footer, type-specific middle slots.

**Scope:** ~400-750 LOC total Bundle C. Could ship as one combined post-Phase-2.5 polish phase.

## Section 8: Open research questions consolidated

Aggregated and deduplicated from all four batches. Each question is tagged with batch origin and the candidate(s) it gates.

### Webui direction (gates 4 deferred sub-components)

1. **Webui write-surface direction**: read-mostly graph explorer, or evolves into write surface? *Batches A+D, gates C2.d, C3.e, C13.f, C12.f*. **Recommended slot:** dedicated stronghold/research artefact, not a phase. Highest-leverage open question.

### Reconciler-finding shape (gates Bundle B scope)

2. **Do reconciler findings carry a structured `severity` field today, or is severity inferred at render time from finding class?** *Batch A C2 Q2, gates C2.c severity rendering implementation*.
3. **Do reconciler findings carry a stable `category` field today, or only `class`?** *Batch A C3 Q1, gates C3.d category-filter sub-component*.
4. **Does `cflx accept` emit findings in a form `cflx check` can reuse verbatim, or does inspection-mode need a separate code path?** *Batch A C3 Q2, gates C3.b shared-data-source contract*.
5. **Is there a CI consumer that wants `cflx check --json`, or is the JSON form speculative?** *Batch A C3 Q3, affects scope of `cflx check` CLI*.

### Copy/voice infrastructure (gates Bundle B foundation)

6. **Where do copy strings live canonically?** Candidates: `docs/design-system/copy.toml`, `src/ui_assets/copy.json`, new `docs/design-system/voice.md`. *Batch A C2 Q3 + C13 Q2, gates Bundle B sequencing*.
7. **Does the CLI surface deserve parallel empty-state treatment?** *Batch A C13 Q4. Recommended answer: yes per voice direction; confirm scope*.
8. **Does `src/ui_assets/app.js` already have a chain-balance widget, or is that future work?** *Batch A C2 Q1, gates C2.b multi-dim rollup research*.

### Graph explorer (gates Bundle C scope)

9. **Does the per-node "Prerequisite for / Enables" widget read from blueprint edges only, or also from decision-attached obligations?** *Batch D C4 Q1, gates C4.b implementation depth*.
10. **Is "GAP" semantically the same as `ghost` or `orphaned`, or a third category cairn currently lacks?** *Batch D C4 Q2, gates C4.d GAP-tile RESEARCH outcome*.
11. **Should a cairn-native navigation lens be H-shape (provenance/authority around hinge) or unconstrained graph with re-centering?** *Batch D C4 Q3, gates C4.e re-frame-on-hinge research*.

### AI-provenance foundation (gates Bundle A and Bundle D)

12. **Should the cflx-trace sidecar include the prompts themselves (full reconstruction) or only metadata (cheaper)?** *Batch C C6 Q1, gates Bundle A scope*.
13. **Are there fields that should never be persisted (API keys, source-file content snippets)?** *Batch C C6 Q2, gates Bundle A privacy/retention design*.
14. **Does cflx telemetry today already capture per-task data, or only per-phase?** *Batch C C6 Q3, affects Bundle A engineering scope*.
15. **Do we want a `--no-trace` flag for sensitive runs, or always-on?** *Batch C C6 Q4, gates Bundle A CLI surface*.
16. **Where does the suggested-edges file live in the change directory?** Sibling to `blueprint.delta`? Separate file? Embedded in `proposal.md`? *Batch C C8 Q1, gates Bundle A C8.a schema*.
17. **Does `cflx accept` print the untriaged count, or block silently?** *Batch C C8 Q2, gates Bundle A C8.b UX*.
18. **What confidence/score does the engine attach to each suggestion? Single threshold, or human triages all?** *Batch C C8 Q3, gates Phase 9 C8.c engine design*.
19. **How does this interact with existing `cairn rename` propagation (§9.6)?** *Batch C C8 Q4, possible Phase 9 first use case*.

### AI-narrative + stamping (gates Bundle D)

20. **Line ranges or content-addressed anchors for provenance block?** *Batch C C7 Q1, **long-pole question for Bundle D***.
21. **Binary tag or multi-state for provenance block field flag?** *Batch C C7 Q2, gates C7.d sub-component*.
22. **How does the provenance block participate in the artefact's interface hash? (Probably excluded.)** *Batch C C7 Q3, gates Bundle D schema*.
23. **How does pre-commit hook treat block edits (ignore, diff-ignore, distinct check)?** *Batch C C7 Q4, gates Bundle D hook integration*.
24. **Is the mainstay sentence templated from structural facts (cheap, deterministic) or LLM-summarised (expensive, probabilistic)?** *Batch C C5 Q1, gates Bundle D C5.c implementation*.
25. **Where does the mainstay sentence live: new `narrative.md` artefact, section in `map.md`, or out-of-band?** *Batch C C5 Q2, gates Bundle D C5.c artefact location*.
26. **Does mainstay regenerate on every scan or on demand only?** *Batch C C5 Q3, gates Bundle D C5.c trigger semantics*.
27. **Is the marketing/landing surface (`docs/landing/`) the actual driver, or is the TUI?** *Batch C C5 Q4, gates Bundle D C5.c scope*.

### Verification states (Phase 7.5c)

28. **What exactly distinguishes `Blocked` from `Failed`?** *Batch D C11 Q1, **research scoped inside Phase 7.5c***. Recommendation: Blocked = upstream dep missing or environment unavailable; Failed = test ran and assertion failed. New error code needed.
29. **Does `cflx accept` for a phase that targets phase-N transition all `Planned(phase=N)` verifications to in-scope, or only on next `cflx apply` after merge?** *Batch D C11 Q2. Recommendation: latter, to keep accept idempotent*.
30. **Can `#[cflx_planned]` be implemented as a proc-macro in the cairn workspace, or does it require a separate crate?** *Batch D C11 Q3, gates Phase 7.5c implementation approach*.

### Genesis / interview (gates Phase 9 C1.b)

31. **Does extending `cflx-proposal` to write `research/genesis.md` require any kernel change, or can it land as a skill-level convention?** *Batch D C1 Q1, gates C1.a NOW commit*.
32. **If Phase 9 brownfield needs elicitation, can the runner be a generalisation of `cflx-proposal` rather than a separate command?** *Batch D C1 Q2, gates C1.b inside Phase 9*.
33. **What is the schema for a "genesis" research artefact: full transcript, summary plus turns, or just the architecture-signals output?** *Batch D C1 Q3, gates C1.a artefact shape*.

### Export (gates Bundle E + C12.e)

34. **Where does an exported JSON live by default? User-specified path, `target/cairn-export/` (gitignored), or `openspec/exports/` (tracked)?** *Batch D C12 Q1, gates Bundle E destination policy*.
35. **What is the JSON schema? Direct serialisation of in-memory graph, bespoke export schema, or same JSON-shape MCP tooling already produces?** *Batch D C12 Q2, gates Bundle E schema*.
36. **Does `cflx export` run before or after the verification battery?** *Batch D C12 Q3. Recommendation: orthogonal: export reads current state regardless of gate status*.
37. **Is the "exports-in-provenance-chain" pattern worth a phase, or stays a design note until concrete consumer asks?** *Batch D C12 Q4, gates C12.e research*.

### Identity / domain expandability (open after rejections)

38. **Would a future "lightweight artefact" type (e.g., bare note attached to a node, no obligation chain) ever benefit from a flatter shape? Probably yes; `todo` is the closest existing one.** *Batch B C9 Q1, gates future schema decisions if more flat-shape types emerge*.
39. **If cairn ever adds a non-code reconciler (Phase 10), do new artefact types emerge for that domain (e.g., `role-description`, `process-step`)?** *Batch B C9 Q2, gates Phase 10 design*.
40. **Are there contract sub-shapes that show up frequently enough across cairn's current users to justify shipping more than one default template?** *Batch B C10 Q1, gates C15 timing*.
41. **Should template selection be reflected in artefact frontmatter as an informational field (`template: interface-contract`)?** *Batch B C10 Q2, gates C15 schema*.
42. **What is the deterministic fingerprinting strategy for non-code reality layers?** *Batch B C14 Q1, **central design question for Phase 10***.
43. **Where exactly is the boundary between "AI assists authoring" and "AI substitutes for reconciler"? Worth making explicit in spec.md as a positive-form design constraint.** *Batch B C14 Q2, partially answered by Bundle F's spec.md addition; refine*.
44. **Is there a confidence-signal pattern that would be honest at the authoring layer without becoming a hard gate?** *Batch B C14 Q3, gates C1.c-rejection-revisitation if calibration improves*.

**Total: 44 open research questions.** Of these:

- **5 are pre-current research** (Q1, Q2, Q3, Q6, Q20) gating Bundle B and Bundle D scoping.
- **2 are central spec/design questions** (Q1 webui direction, Q42 fingerprint strategy) deserving dedicated stronghold-level investigation.
- **The remaining 37** can be deferred to per-bundle design.md authoring without blocking near-term decisions.

## Section 9: Roadmap-level non-decisions

Items where deciding now would be premature. The honest answer is "we don't have enough information to decide yet" and stating the information we'd need.

### N1: Final destination of the webui (read-mostly vs write-surface)

**Why we can't decide:** Four sub-components (C2.d, C3.e, C13.f, C12.f) all defer on this. The decision shapes UI work for the next 12-18 months. Deciding without evidence would either over-build (write affordances that nobody asks for) or under-build (CTAs that all hand off to CLI when in fact users want in-browser actions).

**What we'd need to learn:**
- Are real users (current bootstrap-fixture users + early adopters) opening the webui to *do work* or to *understand the graph*?
- If write surfaces were available, what would the highest-leverage first one be (edit a contract? trigger `cflx propose`? approve a suggested edge?)?
- Does the cflx CLI idiom plus the webui graph explorer cover the workflows users actually run?

**Recommended next step:** A short stronghold-level investigation that surveys current webui usage (probably qualitative since the user base is small) and reports back. Could be a 2-3 day effort. Output: a positioning artefact (`docs/strongholds/webui-direction.md`) that the four deferred sub-components consume.

### N2: Whether mainstay sentence (C5.c) is templated/structural or LLM-summarised

**Why we can't decide:** The cost/value tradeoff differs by an order of magnitude. Templated form is cheap and deterministic; LLM form is expensive and probabilistic. Both are plausible. The right call depends on whether the consuming surface is the marketing landing page (where one-shot generation per blueprint change is fine) or the TUI graph explorer (where regeneration cost matters).

**What we'd need to learn:**
- Does anyone want the mainstay sentence today, or is it a "build it and they will come" candidate?
- If yes, what's the actual surface: `docs/landing/`, the TUI, an export, a `narrative.md` artefact?
- Does Bundle A's run-identity infrastructure plus Bundle D's stamping schema cover the "honesty" requirement enough that an LLM summary is acceptable?

**Recommended next step:** Defer Bundle D scoping until Phase 9 ships. Phase 9 brownfield will surface real corpora that test whether structural sentences read meaningfully. If they do, the cheap form wins; if they don't, the LLM form earns its cost.

### N3: Whether `phase-7.5c` (verification states) ships before or after Bundle A

**Why we can't decide:** Both phases are small and standalone. Both are non-blocking on the others. The ordering depends on orchestrator capacity, not on technical sequencing. Deciding ahead of execution time is premature.

**What we'd need to learn:** Whether the phase-8.0/9.0/10.0 pre-phase tests already in `openspec/changes/` (per the recent commit `c98d506`) are blocking on the structured `#[cflx_planned]` replacement, or if their `#[ignore = "awaits phase-N"]` form is fine for now.

**Recommended next step:** Treat both phases as ship-ready and let cflx scheduling decide the order. Do not pre-bind.

### N4: Whether C15 templated authoring scaffolds ships inside Phase 9 or as immediate follow-on

**Why we can't decide:** Phase 9 brownfield is the natural forcing function but the proposal.md scope (`cairn init --from-code`, `cairn refine`, structural candidate extraction) doesn't currently include multi-template authoring. Adding C15 in-phase risks bloating Phase 9; deferring risks shipping brownfield without the contract diversity it surfaces.

**What we'd need to learn:** Whether Phase 9's structural candidate extraction (per the existing proposal.md) outputs contracts in shapes that need template diversity, or if a single template covers the brownfield case.

**Recommended next step:** Defer to Phase 9's design.md authoring. The author of that design will see the contract shapes and decide. C15 is a follow-on candidate noted in this synthesis; it doesn't need a binding decision today.

### N5: Whether Bundle C (graph-explorer follow-ups) ships before or after Bundle B

**Why we can't decide:** Both touch the webui at `src/ui_assets/app.js`. They could ship together (combined polish phase) or separately. The combined approach has lower context-switching cost; the separate approach has tighter scope per phase.

**What we'd need to learn:** Whether the orchestrator wants concentrated UI investment or distributed UI work across multiple phases.

**Recommended next step:** Default to separate phases (Bundle B first as the higher-leverage cluster, Bundle C as a later polish pass) but be willing to combine if scoping pressure makes a joint phase cleaner.

## Cross-cutting findings

Five observations that emerged at the roadmap level but weren't visible at per-candidate level:

### F1: The webui write-surface question is the single most consequential open decision

Across the 14 candidates, *four sub-components* defer on the same question (C2.d, C3.e, C13.f, C12.f). No other open question gates this many sub-components. Resolving the webui direction before any of the deferred sub-components are designed in detail would prevent the work-twice scenario Batch A explicitly flagged. Recommend treating this as a stronghold-level investigation and prioritising it ahead of Bundle B, even though Bundle B can ship without the answer (because its CLI-handoff fallbacks are honest).

### F2: Bundle A is the highest-leverage near-term phase, full stop

Three reasons: (a) it ships safety primitives that Phase 9 brownfield needs to land safely; (b) it establishes run-identity infrastructure that Bundle D, C5.c mainstay, and C7.a stamping all later consume; (c) it has zero AI dependencies, so it ships into existing kernel without speculation. Of all candidates and bundles in this synthesis, Bundle A is the one whose absence creates the most downstream risk. Phase 9's suggest engine should not land before Bundle A's untriaged-block check, period.

### F3: The "peel-off-the-sub-piece-from-the-framing" pattern recurs across batches

Batch D explicitly named it for C4 (peel salvage sub-components from pyramid framing) and C11 (peel state enum from chat UX). Batch B implicitly named it for C10 (peel templates from closed enum). Batch C named it for C5 (peel islands + verb-edges from systemigram). The pattern is: when a getcairn.dev candidate bundles a structural sub-piece with a framing-shaped wrapper, evaluate the sub-piece on its own merits independent of the wrapper. The verdict cluster: structural sub-pieces that preserve cairn's two-chain semantics often adopt; framing wrappers that import linear-pipeline shape always reject. Worth elevating to a methodological principle for any future external-pattern adoption analysis.

### F4: The rejections form a near-complete identity-portrait that should be in spec.md

Batch B's cluster observation is the strongest single observation in the four batches: *"the three rejections each clarify a different architectural identity claim, and together they form a near-complete negative-space portrait of cairn's identity."* The Bundle F spec.md addition (§3.5) captures this. Recommend prioritising Bundle F as the very first commit out of this analysis, because every subsequent decision benefits from those principles being explicit. Pre-current, single commit, ~350 words.

### F5: The eight `phase-N.0-tests` directories are themselves a structural form of "Planned"

Visible in the openspec/changes/ glob: `phase-8.0-tests`, `phase-9.0-tests`, `phase-10.0-tests`. These directories declare future-phase tests with `#[ignore = "awaits phase-N"]` markers, which is exactly the pattern C11 promotes to first-class. Phase 7.5c's design must explicitly address how the per-test state enum coexists with the per-phase pre-phase-tests directory convention, since both are expressing "Planned" at different granularities. Batch D names this; the roadmap-level observation is that this co-existence is one of the few places where two existing cairn primitives both encode the same concept and the C11 work has to clarify (not unify) them.

---

**End of synthesis.** Total: ~6700 words.
