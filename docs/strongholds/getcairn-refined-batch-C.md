# Refined adoption analysis: Batch C (provenance / trace family)

## Scope

Candidates C5, C6, C7, C8. Framework: 4-dimension + bundling/sequencing analysis. These four interact heavily and the bundling output is the headline of this batch. The matrix already produced first-pass verdicts (C5 DEFER, C6 ADOPT as per-change sidecar, C7 DEFER as bottom-block, C8 ADOPT as change-shaped queue). This pass goes deeper on sub-component decomposition, asks how shared infrastructure should be sequenced, and tests whether the four belong in one phase, two phases, or four.

The cluster's underlying question: how does cairn want to talk about AI-derived content, given that cairn's enforcement value comes from deterministic reconciliation? The four candidates are four lenses on the same question. They should not be designed in isolation.

---

## C5: Mainstay narrative (AI-derived narrative analysis)

### 1. Problem and solution clarity

**Cairn-side problem.** cairn's two-chain framing is precise but invisible at the surface. A reader landing on `map.md` (or a future webui graph view) sees node IDs, edges, and contracts. There is no sentence-level summary of what the system *does*. CLAUDE.md broadens audience to "people building with AI tools, including non-devs"; non-devs need a sentence to enter the graph. Stakeholder-comms surface is presently empty.

A subtler second problem: cairn's marketing (`docs/landing/`) currently has no auto-generated demo of "here is what your system looks like through cairn." A mainstay sentence or systemigram render would be a natural shape for a landing-page demo carousel without re-implementing a whole UI.

**getcairn.dev's mechanism.** A `_narrativeAnalysis` layer attached to the system root: `mainstaySentence` (English), `cards[]` (prose blocks), `connections[]` with `verbPhrase` per edge, `islands[]` for disconnected subgraphs, `layout` and `mainstayPath` for rendering. Output is a Boardman-style systemigram with verb-labeled curved arrows. The whole thing is computed at AI run time; the input is the typed graph plus the requirements/decompositions.

**Solution-to-problem fit.** Partial. The mainstay sentence is the most portable component and maps cleanly onto cairn's authority chain (decision → blueprint → contract → code is *literally* a sentence about flow, just stated structurally). The systemigram visual is more questionable: cairn already has a planned graph explorer (Phase 2.5, archived), and a second render shape risks competing with the existing surface rather than supplementing it. Verb-labeled edges are a small upgrade to existing edge labels (cairn already supports edge labels like `"Reads user records"`). Islands surfacing is genuinely new and useful: cairn does not currently have a query that says "which nodes are not reachable from the system root?", and the orphan/disconnected detection is independently valuable.

**Ambiguities.** The original candidate does not separate "mainstay sentence" (text) from "systemigram" (visual) cleanly. They share input data but ship as different deliverables. Also unclear whether the mainstay regenerates on every scan (cost) or on demand. And how the output gets stamped as AI-derived (which is what makes C7 a hard precondition).

### 2. Layer classification

The candidate is a four-layer composite:

| Sub-part | Layer | % weight |
|---|---|---|
| Mainstay-sentence generation | Architectural (new map output) + Process (when it regenerates) | 35% |
| Systemigram visual render | UI-UX | 25% |
| Verb-labelled edges in render | UI-UX (light) | 10% |
| Islands surfacing (disconnected subgraphs) | Architectural (new query) | 20% |
| Layout / mainstayPath helpers | UI-UX | 10% |

The architectural pieces (sentence-as-output, islands-as-query) are the load-bearing parts. The UI-UX pieces are valuable but downstream and replaceable. The honest split is roughly **55% architectural, 45% UI-UX**.

### 3. Partial adoption breakdown

| Sub-component | Verdict | Reason |
|---|---|---|
| Islands surfacing as new query (`cairn islands` or `cairn neighbourhood --include-orphans`) | NOW | Pure graph-traversal capability, no AI dependency, no provenance question. Independently useful and feeds C8 (suggested edges to reconnect islands). |
| Verb-labelled edges in render | NOW (already supported in grammar; just ensure the UI displays them) | Edges already carry a description string per spec section 7. The UI just needs to render it. |
| Mainstay-sentence generation | RESEARCH | The shape of the sentence (template? LLM-summarised? structurally derived from `cairn order` output?) is undetermined. Needs a research artefact before scoping. |
| Systemigram visual render | LATER (post-Phase 2.5 graph explorer maturity) | The TUI graph explorer must exist and be loved before adding a second render. |
| Layout / mainstayPath helpers | REJECT (as kernel data) | These are render-time concerns. Per spec section 5 ("rendering is a distribution concern"), they belong in a downstream tool that consumes cairn's JSON. |

**Bundleable starter pack:** islands + verb-edge display. Both are "we already have the data; surface it." Ship as a small phase or fold into Phase 2.5 follow-up.

**Dependency on C7:** The mainstay-sentence sub-component is un-shippable without C7 (or some lighter form of AI-vs-authored stamping). An AI-generated sentence sitting prominently in the UI without a provenance stamp would create the very loose-context drift cairn exists to prevent. C7's bottom-block provenance form is the minimum needed to ship mainstay honestly.

### 5. Refined verdict

**Confirmed: DEFER for the AI-narrative core.** Two refinements to the matrix:

1. *Split the candidate.* The matrix treats C5 as one thing; it is four. Adopt islands and verb-edge display NOW as Phase 2.5 follow-ups; defer mainstay-sentence to RESEARCH; defer systemigram to LATER; reject layout helpers from kernel.
2. *Pairing precondition narrows.* Matrix said "pair with C7." The narrower truth: only the mainstay-sentence sub-component depends on C7. Islands and verb-edge display do not.

### 6. Open research questions

- Is the mainstay sentence templated from structural facts (cheap, deterministic) or LLM-summarised (expensive, probabilistic)? The structural form is much closer to cairn's stance.
- Where does it live: a new `narrative.md` artefact, a section in `map.md`, or out-of-band?
- Does it regenerate on every scan or on demand only?
- Is the marketing/landing surface (`docs/landing/`) the actual driver, or is the TUI? If marketing, scope shrinks to "a single sentence per repo, regenerated when the blueprint changes."

---

## C6: Pipeline trace (named stages, models, per-stage timing)

### 1. Problem and solution clarity

**Cairn-side problem.** cflx already produces telemetry per phase (build/clippy/fmt/test pass/fail, validate strict). It is *not* surfaced in archive output as a structured per-stage record. A reviewer reading `openspec/changes/archive/<phase>/` sees the proposal, design, tasks, and specs, but cannot answer "which AI model wrote which artefact, when, with what input?" cflx accept's pass/fail is taken on faith. This is a transparency gap that grows as cairn's audience widens to non-devs (CLAUDE.md voice direction).

A second cairn-side problem the matrix did not name: the cflx run logs already exist transiently but are not promoted to durable artefacts. Each cflx run produces a log; that log is currently a working file, not part of the archived change. Promoting it is mostly a packaging exercise, not a new capability.

**getcairn.dev's mechanism.** Every AI specialist run exposes a four-stage pipeline (`ROUTE → CONTEXT → GENERATE → VALIDATE`). The post-run review modal shows per-stage model name, wall-clock seconds, and token cost. Long-running operations show live elapsed counter with "typically 30 to 60s" expectation copy. Stored as `pipelineTrace` data tied to the operation that produced the artefact.

**Solution-to-problem fit.** Strong fit on principle, mediocre fit on shape. The principle (AI provenance native to the artefact, not out-of-band) is exactly what cairn needs. The shape (per-artefact-frontmatter `pipelineTrace`) is wrong for cairn: every artefact-type schema would version-bump per conventions section 3, and most artefacts would carry duplicate model/timing data because they came from the same phase run.

The matrix's "per-change archive sidecar" reframing is correct and survives further pressure-testing. A single `pipeline-trace.json` per change captures the same information once, indexed by artefact path, without forcing every artefact's schema to grow a metadata field.

**Ambiguities.** The candidate does not specify retention policy (do traces survive forever in `archive/`, or get GC'd after N phases?), privacy model (token-cost data may be sensitive on private codebases run via shared CI), or model-identifier stability (Anthropic model names change; how do diffs surface?).

### 2. Layer classification

| Sub-part | Layer | % weight |
|---|---|---|
| Stage names (route/context/generate/validate) | Architectural (vocabulary) | 10% |
| Model identity (which model ran which stage) | Architectural (provenance data) | 25% |
| Per-stage tokens | Architectural (telemetry) | 15% |
| Per-stage latency | Architectural (telemetry) | 10% |
| Success flag per stage | Architectural (telemetry) | 10% |
| Archive sidecar format (vs frontmatter) | Architectural (schema) | 20% |
| Telemetry collection point (cflx workflow integration) | Process | 10% |

Honest split: **80% architectural, 20% process.** Almost no UI-UX content; this is a kernel/spec change with an export shape.

The matrix's "sidecar not frontmatter" call is the right architectural decision and is **confirmed**. Refinement: the sidecar should be a JSON file with state-versioning header per conventions section 3, located at `openspec/changes/archive/<phase>/.cflx-trace.json` (or similar). Naming with `.cflx-` prefix signals it is a workflow-tool artefact, not a kernel artefact, which preserves the cairn/cflx separation called out in CLAUDE.md.

### 3. Partial adoption breakdown

| Sub-component | Verdict | Reason |
|---|---|---|
| Stage names (route/context/generate/validate) | LATER | These are getcairn.dev's pipeline shape. cairn's pipeline is different (apply/accept/archive). Borrowing stage names would import a foreign vocabulary. Use cairn's actual stages. |
| Model identity per stage | NOW | Already known at runtime (cflx invokes claude/codex with named models). Just persist. |
| Per-stage tokens | NOW | Same: already collected by the SDK. Just persist. |
| Per-stage latency | NOW | Same: trivial to log. |
| Success flag per stage | NOW | Same. |
| Archive sidecar format | NOW | The packaging decision the matrix already reached. |
| Telemetry collection point | NOW | The integration point in cflx workflow (apply/accept/archive). Required for any of the above to land. |

**Replacement for the four foreign stage names:** use cairn's actual lifecycle stages: `propose → apply → accept → archive`. Each phase run produces a sidecar with per-stage records keyed by these names, plus per-task records inside `apply` (since apply is the longest stage and has internal structure). This avoids importing aerospace/MBSE vocabulary and aligns with the existing cflx workflow.

### 5. Refined verdict

**Confirmed: ADOPT.** Refinements to the matrix:

1. *Stage names should be cairn-native (propose/apply/accept/archive), not getcairn.dev's (route/context/generate/validate).* Borrowing the names would be a small voice violation.
2. *Sidecar location is `openspec/changes/archive/<phase>/.cflx-trace.json`.* Prefix signals workflow-tool ownership, separating from kernel artefacts.
3. *State-versioning header per conventions section 3, starting at `version: 1`.* Mandatory.
4. *Add a CLI surface (`cflx trace <phase>`)* that pretty-prints the sidecar. Without a surface to read it, the data is invisible and the auditability claim is hollow.
5. *Privacy/retention deferred to design phase, not blocking.* Default: traces persist with the archive forever; opt-out via config for orgs that need to scrub.

The matrix's medium confidence becomes high once "stage names should be cairn-native" is settled.

### 6. Open research questions

- Should the sidecar include the prompts themselves (full reconstruction) or only metadata (cheaper)?
- Are there fields that should never be persisted (API keys leaked into prompts, source-file content snippets that are big)?
- Does cflx telemetry today already capture per-task data, or only per-phase? If only per-phase, this is a non-trivial cflx code change.
- Do we want a `--no-trace` flag for sensitive runs, or always-on?

---

## C7: Per-field AI provenance tagging

### 1. Problem and solution clarity

**Cairn-side problem.** cairn tracks authored-vs-generated at the *artefact* level (declared blueprint vs scanned map; `verified`/`external`/`unverified` for sources). It does not track this at the *field* level inside an artefact. A decision artefact whose Context section was AI-drafted but whose Decision section was human-written looks identical to a fully human-authored decision in current cairn. Reviewers cannot triage which lines deserve eyes.

The matrix correctly notes this is sharper than it sounds: cairn's *binary* "human or agent" tag at the artefact level *loses information* when most artefacts in real flows are mixed. A reviewer who treats "AI-generated artefact" as a flag-the-whole-thing signal will either over-review (slow) or under-review (risk). Per-field provenance gives a better triage primitive.

**getcairn.dev's mechanism.** Inline `. ai .` token in each property's category line. Authored values lack the tag. Visible at glance; AI-generated values stand out.

**Solution-to-problem fit.** Mechanism matches problem on intent. Mismatches on shape:

1. Their schema is uniform (same property bag for every node), so per-field tags are uniform. cairn's artefacts are typed and free-form; "every field" does not translate.
2. Their UI is the primary surface; cairn's primary surface is markdown files. Inline tokens in markdown are noisy and break standard tooling (Prettier, VS Code, GitHub diff renderer).

The matrix's reframing: *"bottom-of-artefact provenance block with line ranges"*, is the correct adaptation and survives pressure-testing. It version-bumps each artefact type's schema *once*, not per-field. Markdown remains clean. Diff hooks can ignore the block specifically rather than playing whack-a-mole with inline tokens.

**Ambiguities.** The original candidate is silent on provenance-block schema, line-range stability under edit (line numbers change when content is added above), and how human-edits-after-AI-suggestion downgrade the tag.

### 2. Layer classification

| Sub-part | Layer | % weight |
|---|---|---|
| Bottom-of-artefact provenance block (schema) | Architectural | 35% |
| Line-range tagging | Architectural | 25% |
| AI-vs-authored field flag (binary or multi-state) | Architectural (data model) | 15% |
| Diff/change-tracking integration (hooks ignore vs respect block) | Process | 15% |
| UI affordance (display the provenance to the reviewer) | UI-UX | 10% |

Honest split: **75% architectural, 15% process, 10% UI-UX.** The schema and integration work dominate; the UI is the smallest piece (and trivially handled by markdown rendering once the block exists).

### 3. Partial adoption breakdown

| Sub-component | Verdict | Reason |
|---|---|---|
| Bottom-of-artefact provenance block (schema) | LATER | Foundational, but no near-term consumer needs it. C5's mainstay sentence is the first plausible consumer; defer until that ships. |
| Line-range tagging | RESEARCH | Line numbers shift on edit. Either we accept stale ranges (cheap, lossy) or we adopt content-addressed anchors (expensive, robust). The choice is non-trivial. |
| AI-vs-authored binary flag | LATER | Cheap once the block exists, but valueless without the block. |
| Multi-state flag (`ai_drafted`, `ai_suggested`, `human_edited_after_ai`, `human_authored`) | RESEARCH | The right state set depends on real workflows. Premature now. |
| Hook integration (treat block changes as content changes? ignore?) | LATER | Depends on the schema decision. |
| UI affordance | LATER | Trivial once data exists. |

**No NOW component.** The matrix's DEFER verdict is correct: C7 has no near-term consumer that justifies the schema bump and tooling cost. Its case-for is *latent*: it becomes load-bearing only when paired with C5 (so AI narrative is stamped honestly) or C8 (so AI-suggested edges carry their own provenance). Without one of those consumers shipping, C7 is solving for a problem cairn does not yet have.

### 5. Refined verdict

**Confirmed: DEFER.** Refinements:

1. *Bottom-block form is correct; inline-token form is rejected outright.* The matrix already ranked this; this analysis confirms with no flip.
2. *C7 should ship at the same time as C5's mainstay-sentence sub-component, not before.* If C5 is deferred to RESEARCH (as recommended above), C7 is too. They unblock each other.
3. *Line-range tagging is the long-pole problem.* Whoever scopes the joint phase has to make the content-addressing-vs-line-number call before tasks can be sized.
4. *Reject "AI-vs-authored as inline token" permanently, not just defer.* The inline form is incompatible with markdown tooling and should not return as a future option.

### 6. Open research questions

- Line ranges or content-addressed anchors? (Long-pole question.)
- Binary tag or multi-state? (Depends on real reviewer workflow.)
- How does the provenance block participate in the artefact's interface hash? Probably excluded (otherwise edits to the block trigger contradiction churn) but needs explicit decision.
- How does pre-commit hook treat block edits? Ignore? Diff-ignore? Distinct check?

---

## C8: Suggest Trace Links (AI-suggested edges)

### 1. Problem and solution clarity

**Cairn-side problem.** Phase 9 (brownfield extraction) is on the roadmap. Brownfield ingestion needs exactly this affordance: walk the existing artefact corpus, propose cross-cutting edges (decision→source, contract→research) that humans should review. cairn currently has no such tool. Manual edge authoring is the most tedious part of building a provenance chain on a real codebase; un-tooled brownfield ingestion will fail because nobody hand-authors hundreds of edges.

A subtler problem: cairn's *enforcement story* depends on edge integrity. If AI-suggested edges silently land in the authority chain, cairn has shipped the very corruption it sells protection against. Any adoption must structurally prevent silent landing. The matrix correctly anchors on this: queued not auto-applied.

**getcairn.dev's mechanism.** The traceability matrix view distinguishes manual `+ Add Link` from AI-driven `+ Suggest Links`. Suggested links go through their four-stage pipeline. The split is honest (cost, probability) and surface-distinct.

**Solution-to-problem fit.** Strong fit on principle (AI assistance separated from deterministic authoring) and *very strong* fit on cairn's primitives. The matrix's reframing: *"build it as a change-shaped artefact"*, is excellent and survives pressure-testing: cairn already has change isolation, delta semantics, and `cflx accept` that refuses to merge invalid changes. Layering AI-suggested edges on top of changes makes the review boundary non-bypassable by construction. The kernel does the safety work.

**Ambiguities.** What is the format of a suggested-edges file? Does it live alongside other deltas in the change directory, or in a separate file the human triages first? How does triage record per-edge accept/reject? What does cflx accept output when the change has untriaged edges?

### 2. Layer classification

| Sub-part | Layer | % weight |
|---|---|---|
| Suggest engine (the AI specialist that produces candidate edges) | Architectural + Process | 30% |
| Change-shaped queueing (suggested-edges as part of a change directory) | Architectural | 25% |
| Review boundary (per-edge accept/reject) | Process + UI-UX | 20% |
| cflx accept refuses to merge untriaged | Architectural (kernel safety) | 15% |
| Phase 9 brownfield integration | Process | 10% |

Honest split: **65% architectural, 25% process, 10% UI-UX.** This is a kernel-aware feature with workflow integration, not a UI feature.

### 3. Partial adoption breakdown

| Sub-component | Verdict | Reason |
|---|---|---|
| Suggest engine | MILESTONE-GATED (Phase 9) | The engine is brownfield's natural use case. Shipping it earlier would mean shipping a tool with no users. |
| Change-shaped queueing format | NOW | Pure schema work; can be designed before Phase 9 ships. Could be a small pre-Phase-9 phase. |
| Review boundary mechanics (accept/reject per edge) | NOW | Same: schema/format work, no AI dependency. |
| cflx accept untriaged-block check | NOW | Simple safety check; should land before any suggestion engine ships, so the safety is in place when the engine arrives. |
| Phase 9 brownfield integration | MILESTONE-GATED (Phase 9) | By definition. |
| Manual `cairn suggest-edges <node>` (human invokes engine for one node) | LATER | Smaller use case than brownfield; defer. |

**Sequencing implication:** the queueing format and safety check (NOW) should land before the engine (MILESTONE-GATED Phase 9). This is a textbook pre-phase-tests pattern (conventions section 5): land the gate before the feature, so the feature lands into existing safety.

### 5. Refined verdict

**Confirmed: ADOPT.** Refinements:

1. *Split into two waves.* Wave 1 (NOW or near-term): queueing format + safety check + per-edge triage mechanics. Wave 2 (Phase 9): the actual suggest engine.
2. *Wave 1 is a small phase by itself.* It can ship as `phase-N-suggested-edges-format` ahead of Phase 9.
3. *cflx accept's untriaged-block check is the load-bearing safety.* If implemented wrong, cairn ships its own corruption surface. Pre-phase tests should pin the behaviour before any engine code lands.
4. *The engine's API surface should be `cflx suggest-edges <change>` (not `cairn`).* This is workflow tooling, not kernel; matches the cairn/cflx separation.

The matrix's high confidence is preserved.

### 6. Open research questions

- Where does the suggested-edges file live in the change directory? Sibling to `blueprint.delta`? Separate file? Embedded in `proposal.md`?
- Does `cflx accept` print the untriaged count, or block silently? (User experience question.)
- What confidence/score does the engine attach to each suggestion? Single threshold, or human triages all?
- How does this interact with the existing `cairn rename` propagation (section 9.6)? Should rename-detection be the first suggest-engine use case, since it is deterministic and high-confidence?

---

## Section 4: Bundling and sequencing across C5/C6/C7/C8

This is the headline of Batch C. The four candidates share infrastructure in non-obvious ways, and shipping them in the wrong order produces either over-built or under-safe systems.

### Shared infrastructure

| Shared concept | Used by | Cost if duplicated |
|---|---|---|
| AI-derived stamping ("this came from a model") | C5 (mainstay), C7 (per-field), C8 (suggested edges) | High. Three independent stamping schemes would create three diff surfaces, three review semantics, three failure modes. |
| Model + run identity | C5 (which model wrote the sentence), C6 (per-stage trace), C7 (which run produced the value), C8 (which run produced the suggestion) | Medium. Could converge on a single run ID surfaced everywhere. |
| Triage / review boundary | C7 (human-edited-after-AI promotion), C8 (suggested-edges accept/reject) | High. Two triage surfaces would confuse reviewers. |
| Telemetry collection point in cflx | C6 (the trace itself), C5/C7/C8 (which run produced what) | Medium. Single collection point; all four read it. |
| Change-shaped delta semantics | C8 (suggested edges as a delta), potentially C7 (provenance-block edits as deltas) | Low. cairn already has this. |

The big-three convergence: **AI-derived stamping, run identity, and triage boundary** are needed by three or four of the candidates each. If cairn ships C6 first (cleanly, because it is least entangled), then C8's wave 1 (queueing + safety), the remaining concepts can be designed against the foundation those two laid.

### Recommended sequencing

```
Wave 1 (near-term, low coupling):
  - C6 in full as per-change archive sidecar
    (`openspec/changes/archive/<phase>/.cflx-trace.json`)
  - C8 wave 1: suggested-edges format + safety check in cflx accept
  - C5 starter pack: islands query + verb-edge display
    (no AI; pure graph capability)

Wave 2 (Phase 9 - brownfield):
  - C8 wave 2: the actual suggest engine
    (consumes the wave-1 format)

Wave 3 (research-then-design phase, post-Phase 9):
  - C7: bottom-block provenance schema (line-range research)
  - C5: mainstay-sentence sub-component
    (consumes C7's stamping)
  - C5: systemigram visual
    (post Phase 2.5 graph explorer maturity)
```

### Why this order

1. **C6 unblocks the run-identity concept.** Once cflx writes a structured trace per phase, every later candidate can refer to a specific run and stage by ID. This is the cheap foundation; ship it first.
2. **C8 wave 1 is purely schema/safety.** No AI, no provenance question, no schema bump on artefacts. It can ride alongside C6 with no conflict.
3. **C5 starter pack (islands, verb-edges) is pure graph work.** No coupling with anything else. Folds into Phase 2.5 follow-up naturally.
4. **C8 wave 2 lands in Phase 9.** Brownfield is its real consumer; shipping earlier wastes effort.
5. **C7 and C5 mainstay belong in the same phase.** C5 mainstay is C7's first non-toy consumer; C7 is C5 mainstay's honesty stamp. Designing them together avoids two schema iterations.

### Joint-phase candidates

There are exactly two natural joint scopes:

**Joint 1: "AI provenance foundation"**
- C6 (full)
- C8 wave 1 (queueing + safety)
- C5 starter pack (islands + verb-edge)

These three all ship without AI dependencies and without artefact-schema bumps. Bundle them as one phase (call it `phase-N-ai-provenance-foundation` or similar). Together they prepare the substrate (run identity, change-shaped queueing, graph traversal completeness) without committing cairn to any AI-output-stamping decisions yet.

**Joint 2: "AI-derived narrative"**
- C5 mainstay-sentence sub-component
- C7 bottom-block provenance schema

These two unblock each other and have no purpose without each other. Ship as one later phase (post-Phase 9), with the line-range/anchor research resolved up-front.

C8 wave 2 stays as a Phase 9 task, not joined with the others.

### Dependencies: explicit graph

```
C6 (sidecar)
  └─ enables run-identity references in C5/C7/C8

C8 wave 1 (queueing + safety)
  └─ no upstream deps
  └─ enables C8 wave 2

C5 starter pack (islands + verb-edge)
  └─ no upstream deps
  └─ optional input to C8 wave 2 (suggested edges to reconnect islands)

C8 wave 2 (engine)
  ├─ depends on C8 wave 1
  ├─ depends on C6 (so suggestions carry run identity)
  └─ Phase 9 milestone-gated

C7 (provenance schema)
  ├─ depends on C6 (so blocks can reference runs)
  └─ enables C5 mainstay

C5 mainstay sentence
  ├─ depends on C7
  └─ depends on line-range/anchor research

C5 systemigram visual
  └─ depends on Phase 2.5 graph explorer being mature
```

### Naming the joint phases

If cairn adopts this analysis, the proposal IDs would naturally be:

- `phase-N-ai-provenance-foundation` (Joint 1)
- `phase-9-suggest-edges-engine` (folds into existing Phase 9)
- `phase-(post-9)-ai-narrative-and-stamping` (Joint 2)

The third phase ID is intentionally vague because timing depends on Phase 9 outcomes.

### What this answers from the cross-cutting findings (matrix)

The matrix's cross-cutting finding 1 said "C5, C7, C8 all hinge on queue-vs-auto-apply; build queue once and reuse." This analysis sharpens that: the *queue* (C8 wave 1 format) and the *stamping* (C7 bottom-block) are different infrastructures, not one. Build C8 wave 1 in Joint 1; build C7 stamping in Joint 2. Do not conflate them.

---

## Cluster observation

The whole family teaches a single principle: **cairn's stance on AI-derived content is "honest, queued, and structurally reviewable."** Not "auto-applied with a tag" (that would be C9's flatness mistake at a different layer), and not "rejected outright" (that would surrender the obvious productivity gains). Every adoption shape this analysis recommends: sidecar trace, change-shaped queueing, bottom-block stamping, untriaged-block safety, is a different expression of the same principle: AI output enters cairn the same way human output enters, through changes that the kernel inspects before merging. The provenance/trace family is *not* a feature set; it is the operationalisation of cairn's "fence around the authority chain" promise applied to AI as a special case of any author. The four candidates collectively answer "how does cairn talk about AI?" with "the same way it talks about anyone else, with extra metadata and explicit triage."

The single highest-risk pitfall across the family: shipping any AI-stamping mechanism (C5 mainstay, C7, C8 engine) *before* the kernel has the safety primitives (C6 trace, C8 wave 1 queueing + accept-block) that make the stamping non-bypassable. Get the order wrong and cairn becomes the corruption surface it sells protection against.
