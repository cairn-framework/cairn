# Refined adoption analysis: Batch D (lifecycle / workflow)

## Scope

Candidates C1, C4, C11, C12. Framework: 4-dimension (problem/solution clarity, layer classification, partial-adoption decomposition, refined verdict + open questions) plus a fifth dimension unique to this batch, cflx-lifecycle integration. C4's salvage analysis is critical: the matrix sharpened to REJECT precisely because the visual *is* the framing, but discrete sub-components (GAP tile, re-center, prerequisite/enables widget, lane mechanics) deserve honest pressure-testing as separable artefacts. C11 entered the matrix as flipped-to-ADOPT-modified; this pass refines the implementation scope. C1 and C12 entered as DEFER; this pass refines the preconditions and (for C12) extracts the Quick Export half from Professional Export with sharper boundaries.

The cflx lifecycle anchor for every analysis below is `propose → apply → accept → archive`, with the pre-commit hook (`cargo fmt --check`) and verification battery (`cargo build` zero warnings, `cargo clippy --all-targets --all-features -D warnings`, `cargo fmt --check`, `cargo test`, `cargo test --locked`, `cflx.py validate <phase> --strict`) sitting on top. Phase artefacts are `proposal.md`, `design.md`, `tasks.md`, and `specs/`. The two-chain topology (provenance: source → research → decision; authority: decision → blueprint → contract → code) and the rejection of flat-six framing (CLAUDE.md, spec v0.5/v0.7) are load-bearing constraints.

---

## C1: Multi-round interview as genesis

### Section 1: Problem and solution clarity

**The cairn-side problem.** Phase genesis is the boundary cairn currently under-invests in. Today a phase begins with a human-authored `proposal.md` written into `openspec/changes/<phase>/`. The rationale, alternatives considered, and discarded directions are flattened into prose by the time the phase opens; the elicitation context that produced the proposal is not preserved as queryable data. cflx then runs `apply` against an already-collapsed proposal. The provenance chain (source → research → decision) has no native genesis hook, research artefacts are typically authored after the fact, not produced as a side effect of the proposal-shaping conversation. The `cflx-proposal` skill (listed under available skills) does interactive proposal authoring but its transcripts are not preserved as `research` artefacts.

**getcairn.dev's mechanism.** Three rounds of structured clarifying questions, each producing a confidence pill (78%, 82%) keyed to a working brief plus an architecture-signals panel. The full Q&A transcript is preserved as `Project Genesis - Preserved as provenance` and rendered alongside build progress for permanent reference. The interview is a soft gate: confidence pill is a signal, not a hard threshold (or at minimum the surface does not communicate hardness; the candidates file logs hard-threshold variant as out-of-scope).

**Solution → problem fit.** Strong on the provenance side (interview transcript becomes a navigable research artefact lineage that *cairn's two-chain topology specifically privileges*), weak on the workflow side (chat UX is not cflx's idiom; cflx is CLI-and-codex-driven). The fit is asymmetric: the *output shape* (preserved provenance) is exactly what cairn wants; the *input shape* (multi-round chat) is exactly what cairn deliberately doesn't impose.

**Ambiguities/missing data.** (a) Whether Phase 9 brownfield design will surface a concrete elicitation requirement; (b) whether the `cflx-proposal` skill's existing interactive flow can be promoted to first-class transcript-as-research without a new command; (c) whether confidence pills could be honest in a cairn context where the calibration data does not exist.

### Section 2: Layer classification

- **Architectural ~30%**: transcript-as-research-artefact, the typed link from a `research` artefact to the resulting `decision` (this is the load-bearing change to the kernel).
- **Process ~50%**: multi-round prompt orchestration, integration with `cflx propose` (or a new `cflx interview`), cflx-proposal skill upgrade. This is workflow plumbing.
- **UI/UX ~20%**: confidence-pill UI, transcript-rendering surface, architecture-signals panel.

### Section 3: Partial adoption breakdown

| Sub-component | Status | Reasoning |
|---|---|---|
| Transcript-as-research-artefact | RESEARCH (precondition for adopt) | The kernel-shaped sub-piece: capture interview transcripts as first-class `research` artefacts with edges to the resulting `decision`. Earns its keep regardless of UX shape. |
| Interview-runner CLI mode (`cflx interview`) | MILESTONE-GATED on Phase 9 | If brownfield ingestion needs interactive elicitation, build it then. Otherwise the existing `cflx-proposal` skill covers the developer-CLI use case. |
| Multi-round prompt orchestration | MILESTONE-GATED on Phase 9 | Same gate. The orchestration logic lives inside the interview runner. |
| Confidence-pill UI | REJECT | Calibration is unsolved. The candidates file already lists hard confidence gate as out-of-scope; the soft variant decorates without informing. Building it imports a number cairn cannot defend. |
| Architecture-signals panel | LATER (Phase 9+) | Useful only after a brownfield-elicitation surface exists; same gate as the runner. |
| Integration with `cflx-proposal` skill | NOW (small) | Promote the existing skill's transcript output to a written `research` artefact in `openspec/changes/<phase>/research/genesis.md`. This is the cheapest, most-load-bearing bit of the candidate. |

### Section 4: Lifecycle/workflow integration

- **Stage touched**: `propose` (entirely). Does not touch `apply`, `accept`, or `archive`.
- **New commands**: `cflx interview` (gated on Phase 9 adoption). No modification to existing commands required if transcript writing is bolted onto `cflx-proposal`.
- **Phase artifacts**: introduces a new convention: `openspec/changes/<phase>/research/genesis.md` (or similar). This is additive; does not modify `proposal.md`, `design.md`, `tasks.md`, or `specs/`.
- **Pre-commit/verification battery**: no interaction. Genesis is pre-`apply`; nothing in the battery runs.
- **Existing primitive duplication**: `cflx-proposal` skill already does interactive proposal drafting. The duplication risk is high: a separate `cflx interview` command that does mostly the same conversation would invert the cost. Recommendation: extend `cflx-proposal` to write its transcript out before introducing a parallel command.

### Section 5: Refined verdict

**DEFER. Preconditions sharpened.** The matrix said "DEFER, gate on Phase 9 brownfield." This pass refines: the *transcript-as-research-artefact* sub-component should ship sooner than Phase 9 as an extension to `cflx-proposal`'s output, write a `research/genesis.md` artefact when the skill produces a proposal. That much earns its keep regardless of the larger interview decision and is the load-bearing piece for the two-chain provenance story. The *interview-runner CLI mode* and *multi-round orchestration* stay gated on Phase 9 elicitation needs. The *confidence pill* flips from DEFER to REJECT: it imports a calibration problem cairn cannot solve, and the candidates file already noted the hard variant is out-of-scope.

Sub-component flips from matrix:
- Confidence pill: DEFER → REJECT.
- Transcript-as-research-artefact: DEFER → RESEARCH/NOW (as `cflx-proposal` extension).

### Section 6: Open research questions

1. Does extending `cflx-proposal` to write a `research/genesis.md` artefact require any kernel change, or can it land as a skill-level convention?
2. If Phase 9 brownfield needs elicitation, can the runner be a generalisation of the `cflx-proposal` skill rather than a separate command?
3. What is the schema for a "genesis" research artefact, full transcript, summary plus turns, or just the architecture-signals output?

---

## C4: Causality pyramid (and its salvageable sub-components)

### Section 1: Problem and solution clarity

**The cairn-side problem.** cairn has rich authority-chain data (decision → blueprint → contract → code) but no learnable navigation surface for "what depends on what" or "what is this node a prerequisite for." The graph explorer renders nodes and edges but not ordering. A non-specialist staring at the graph cannot easily answer "what comes before this" or "what does this enable."

**getcairn.dev's mechanism.** A five-tier pyramid (System / Domain Technologies / Parts and Materials / Instruments and Connections / Knowledge Foundation) rendered as horizontal lanes. Inline GAP tiles surface unresolved layers. The pyramid re-centers on any node. Per-node "Causal Position" widgets render "Prerequisite for X" or "Enabled by Y, System capstone."

**Solution → problem fit.** The framing as a whole is wrong for cairn (matrix is correct: visual *is* the framing, and a five-tier pyramid is the exact shape v0.5 explicitly rejected). But the candidate is a bundle of distinct sub-mechanisms; the bundle decision is REJECT, the per-piece decisions are not all REJECT.

**Ambiguities/missing data.** Whether the GAP tile pattern is structurally tied to lane mechanics (suggesting it imports the framing) or is genuinely portable (a marker on any view).

### Section 2: Layer classification

- **Architectural ~5%**: nothing. The pyramid is a derived view; no kernel data needs to change.
- **Process ~5%**: nothing meaningful. The pyramid does not enter cflx workflow.
- **UI/UX ~90%**: this is almost entirely a visual-lens candidate. That's exactly why the framing risk dominates.

### Section 3: Partial adoption breakdown (the salvage pass)

| Sub-component | Status | Reasoning |
|---|---|---|
| Pyramid layout (five-tier lanes) | REJECT | The framing problem. CLAUDE.md and spec v0.5 explicit prohibition. No flip. |
| "Knowledge Foundation" tier as a concept | REJECT | This is the *most* pyramid-shaped sub-piece, it presupposes a foundation tier with everything else "above." Imports the linear-pipeline framing. The provenance chain is not a foundation under the authority chain; they meet at the hinge (decision). |
| Lane visualization | REJECT | Lanes are the visual encoding of the tier hierarchy. Without tiers there are no lanes; the lanes are the framing's visual signature. |
| Re-center on any node | ADOPT (NOW-ish) | Genuinely separable. The cairn graph explorer would benefit from a "make this node the centroid" affordance regardless of whether the surrounding view is a pyramid, an H-shape, or an unconstrained graph. This is a viewport primitive, not a framing primitive. |
| Per-node "Prerequisite for X / Enables Y" widget | ADOPT (NOW-ish) | This is the most surprising salvage. It does NOT require a pyramid; it requires the authority-chain edges cairn already computes. A node-detail panel showing "Prerequisite for: [list]" and "Enables: [list]" reads from cairn's edges directly. It teaches "what depends on what" without teaching tier ordering. |
| GAP tile pattern (inline marker for unresolved layers) | RESEARCH | Salvageable in principle: a "GAP" or "missing" marker showing where authority-chain links terminate without a fulfilling artefact (e.g., a contract referenced by a decision but not present, or a blueprint primitive without a backing source). But cairn already has `ghost`/`orphaned` reconciliation states which serve a similar function. The research question: is GAP a richer marker than `ghost`/`orphaned`, or is it the same thing wearing different vocabulary? Default to LATER and re-evaluate during the C2 fidelity-rollup design. |
| "Re-frame on hinge" view | RESEARCH (cairn-native generalisation) | Not a getcairn.dev feature directly, but the obvious cairn analogue of "re-center the pyramid" is "re-frame the H around any hinge node." This is a cairn-native navigation primitive worth scoping; flag as research, not adopt-now. |

### Section 4: Lifecycle/workflow integration

- **Stage touched**: none of `propose`/`apply`/`accept`/`archive`. C4 is entirely a webui/visualisation concern.
- **New commands**: none. The salvaged sub-components live in the webui at `src/ui_assets/`.
- **Phase artifacts**: none affected.
- **Pre-commit/verification battery**: no interaction.
- **Existing primitive duplication**: GAP tile potentially duplicates `ghost`/`orphaned` reconciliation states, this is the strongest reason to keep GAP as RESEARCH rather than adopt-now.

### Section 5: Refined verdict

**REJECT (bundle), with two adopt sub-components.** Matrix was right that the pyramid framing is decisive. Salvage list:

- "Prerequisite for / Enables" per-node widget → **ADOPT** (flips from bundled REJECT).
- "Re-center on any node" graph-explorer affordance → **ADOPT** (flips from bundled REJECT).
- GAP tile pattern → **RESEARCH** (don't ship until C2's fidelity-rollup design clarifies whether it duplicates `ghost`/`orphaned`).
- "Knowledge Foundation" tier concept → confirmed REJECT (the most framing-laden sub-piece).
- Lane visualization → confirmed REJECT.
- Pyramid layout → confirmed REJECT.

The salvage is meaningful: the per-node prerequisite/enables widget is real UX value at near-zero kernel cost, and "re-center on any node" is a generic graph-explorer primitive.

### Section 6: Open research questions

1. Does the per-node "Prerequisite for / Enables" widget read from blueprint edges only, or also from decision-attached obligations (which would couple it to the authority chain more tightly than the simple edge set)?
2. Is "GAP" semantically the same as `ghost` (a referenced node that doesn't exist) or `orphaned` (an existing node with no incoming references), or is it a third category cairn currently lacks?
3. Should a cairn-native navigation lens be an H-shape (provenance left, authority right, hinge in the middle) or remain unconstrained-graph with re-centering? This is a cairn-native UI question, not a getcairn.dev borrowing.

---

## C11: Verification status lifecycle

### Section 1: Problem and solution clarity

**The cairn-side problem.** cairn currently treats verification as binary at commit time: `cflx accept` either passes the verification battery or fails. But roadmap-shaped phases routinely declare verifications that cannot run until later phases ship the surface they verify. The codebase already encodes this informally via `#[ignore = "awaits phase-N"]` markers in tests, paired with the test-first pre-phase convention (conventions.md section 5; AGENTS.md instruction to remove `#[ignore]` markers as the feature lands). The `phase-{8.0,9.0,10.0}-tests` directories visible in `openspec/changes/` (recent commit `c98d506`) are exactly this pattern: tests committed in advance of the feature phase.

**getcairn.dev's mechanism.** Five-state enum (Draft, Planned, Passed, Failed, Blocked) on verification records. Captured as kernel data, not as a comment in test files.

**Solution → problem fit.** Strong. The matrix already established that the imagined collision with `cflx accept` is illusory, `Planned` verifications are out-of-scope-for-this-phase by definition. The existing `#[ignore]` workaround is a hand-rolled, comment-string-typed version of exactly this lifecycle.

**Ambiguities/missing data.** Whether the lifecycle is a property of *individual tests* (Rust test attributes), *verification artefacts* (cairn artefact-type level, like contract or decision), *phase-level verification declarations* (in `tasks.md` or `specs/`), or all three. Plausibly all three; the design needs to land it.

### Section 2: Layer classification

- **Architectural ~50%**: state enum, per-state semantics, integration with the verification battery, `Planned(phase = N)` field schema.
- **Process ~40%**: integration with `cflx accept`'s gate logic, replacement of `#[ignore = "awaits phase-N"]`, conventions.md section 5 update, AGENTS.md update for `#[ignore]` removal flow.
- **UI/UX ~10%**: query/dashboard surface (e.g., `cflx verifications --status planned --phase 9`).

### Section 3: Partial adoption breakdown

| Sub-component | Status | Reasoning |
|---|---|---|
| State enum (Draft/Planned/Passed/Failed/Blocked) | NOW | Matrix verdict; refined here as a Rust enum on verification records plus the test-attribute mapping. |
| `Planned(phase = N)` with explicit phase target | NOW | Replaces stringly-typed `#[ignore = "awaits phase-N"]`. The phase target is structurally required so cflx can enforce that planned verifications don't accumulate undeleted past their target phase. |
| Per-state semantics in `cflx accept` | NOW | Draft + Planned are out-of-scope for the current phase's gate; Passed satisfies; Failed blocks; Blocked blocks with a different error class than Failed. The error code distinction matters for surfacing root cause. |
| Replacement of `#[ignore = "awaits phase-N"]` | NOW (paired) | The replacement is the load-bearing change. Could be a custom test attribute (e.g., `#[cflx_planned(phase = 9)]`) that expands to `#[ignore]` plus registry write so cflx knows the planned set without parsing comment strings. |
| Query/dashboard surface | LATER | A `cflx verifications --status planned` command is a natural follow-on but lower priority than the state enum itself. |
| `Blocked` state semantics | RESEARCH | Distinct from `Failed` how exactly? Matrix said "upstream dependency missing"; the design needs to specify what dependencies (missing artefact, missing phase, missing infrastructure) trigger Blocked vs Failed. This is the slipperiest state in the enum. |
| Integration with conventions.md section 5 | NOW (paired) | The pre-phase test convention currently mandates `#[ignore = "awaits phase-N"]`; that text needs updating once the structured replacement lands. AGENTS.md instruction (line 25) needs the same update. |

### Section 4: Lifecycle/workflow integration

- **Stages touched**: `apply` (verification artefacts authored or updated), `accept` (gate logic consults state enum), `archive` (the question of what happens to `Planned` verifications scoped to a phase that just merged, they should transition to in-scope for the target phase, but only after a re-validation pass).
- **New commands**: `cflx verifications` (later); custom test attribute `#[cflx_planned]` (now, paired with conventions update).
- **Phase artifacts affected**: `specs/` may grow a `verifications:` section listing planned verifications with phase targets; `tasks.md` references to "remove `#[ignore]`" become "transition to Passed via `cflx_planned` decoration removal."
- **Pre-commit/verification battery interaction**: this is the load-bearing integration. The battery (`cargo test`) currently runs all non-ignored tests. The structured replacement must either (a) keep `#[ignore]` semantics so existing infrastructure works and merely *register* the planned set in a sidecar, or (b) replace `#[ignore]` with a custom skip mechanism. (a) is lower-risk; (b) is cleaner. Recommend (a) in the first iteration.
- **Existing primitive duplication**: the `phase-N.0-tests` pattern in `openspec/changes/` is itself a structural form of "Planned." The state enum should NOT duplicate the phase-tests directory convention; it should *complement* it. Phase-tests directories declare future tests; the state enum on individual verifications tracks per-test status. Both can coexist.

### Section 5: Refined verdict

**ADOPT (modified). Implementation scope refined.** Matrix already flipped to ADOPT-modified. This pass refines the implementation:

1. The state enum lives on individual verification artefacts and on test attributes, not at phase-level (phase-level is already covered by `phase-N.0-tests` directory convention).
2. The custom attribute `#[cflx_planned(phase = 9)]` is the load-bearing ergonomic. It expands to `#[ignore]` plus registry registration in the first iteration; a future phase can swap `#[ignore]` for a native skip mechanism if churn warrants.
3. `Blocked` semantics need explicit specification (missing artefact reference vs. missing infrastructure vs. missing upstream phase). RESEARCH-tagged.
4. Conventions.md section 5 and AGENTS.md line 25 need paired updates in the same phase.
5. Phase scope: small dedicated phase, not folded into Phase 8/9/10. Likely Phase 7.5c-shaped (small cleanup phase between feature phases).

Sub-component flips from matrix: none net (matrix was already ADOPT-modified); refinements are scope-level.

### Section 6: Open research questions

1. What exactly distinguishes `Blocked` from `Failed`? Proposed: Blocked = upstream dependency missing or environment-unavailable (test cannot run); Failed = test ran and assertion failed. The error-code distinction in `openspec/registries/error-codes.md` will need a new code.
2. Does `cflx accept` for a phase that *targets* phase-N transition all `Planned(phase=N)` verifications to in-scope, or are they only re-evaluated on the next `cflx apply` after merge? (Recommendation: latter, to keep accept idempotent.)
3. Can the `#[cflx_planned]` attribute be implemented as a proc-macro in the cairn workspace, or does it require a separate crate?

---

## C12: Two-tier export

### Section 1: Problem and solution clarity

**The cairn-side problem.** cflx today couples raw-graph data with composed deliverables, the `archive` operation merges deltas and updates `openspec/specs/`, but there is no `cflx export` command that produces a portable artefact for review-by-non-developer or round-trip-through-other-tools. Reviewers, executives, and non-dev collaborators need a way to consume cairn output without reading raw spec deltas or running the CLI.

**getcairn.dev's mechanism.** Settings export tab splits Quick (no API key, instant: JSON, Markdown, Requirements CSV, downloaded to user filesystem) and Professional (API key, ~30s: PPTX, DOCX, saved into project Assets, kept inside the provenance chain rather than escaping).

**Solution → problem fit.** Quick Export half is a strong fit: cairn has structured data (graph, declarations, contracts, decisions) that a `cflx export --format json|md|csv` command can render mechanically. Professional Export half is a poor fit: AI-rendered DOCX/PPTX is a render subsystem cairn does not own and shouldn't grow (spec section 5: rendering is a downstream concern).

**Ambiguities/missing data.** Whether anyone is asking for Quick Export today (matrix flagged this honestly). The "Assets stays in the provenance chain" pattern is the most cairn-resonant idea in the candidate but is also the most ambiguous: cairn has changes/archive directories, not Assets; mapping the pattern requires a design call.

### Section 2: Layer classification

- **Architectural ~30%**: graph→JSON serialiser, "exports stay inside the provenance chain" pattern (if adopted), Assets-equivalent storage location.
- **Process ~50%**: new `cflx export` command, format-selection plumbing, output destination policy.
- **UI/UX ~20%**: settings-pane surface (Phase webui-direction-dependent), CLI flag ergonomics.

### Section 3: Partial adoption breakdown

| Sub-component | Status | Reasoning |
|---|---|---|
| Quick Export: JSON | NOW | The graph and artefact corpus are already structured; serialising to JSON is mechanical. Highest value: round-trip use, MCP tooling, external consumers. |
| Quick Export: Markdown | NOW | Spec sections, archived-change summaries, and decision rationales serialise cleanly to a single Markdown bundle. Useful for paste-into-issue, GH discussion contexts. |
| Quick Export: CSV (requirements/contracts) | LATER | Useful but lower priority than JSON/MD. CSV implies a flattened tabular projection; cairn contracts have nested structure that loses fidelity. Consider only if a concrete reviewer asks. |
| Professional Export: PPTX | DEFER (probably permanent) | Render subsystem cost. Spec section 5 explicit non-goal. Better served by downstream tooling that consumes the JSON Quick Export. |
| Professional Export: DOCX | DEFER (probably permanent) | Same. |
| "Assets stays inside the provenance chain" pattern | RESEARCH | The valuable idea inside the candidate. cairn's analogue would be: exports written to `openspec/changes/<phase>/exports/` (during phase) or `openspec/specs/<area>/exports/` (post-archive), tracked in git, rather than escaping to `~/Downloads/`. Requires a design call on storage location and lifecycle (do exports stay forever, get GC'd, or tracked-but-gitignored?). Worth one design pass. |
| Settings-pane UI surface | LATER | Webui-direction-dependent (open question 1 from matrix). Today cairn webui is read-mostly; adding an export button makes sense once the webui has more write affordances. CLI-first is the right starting point. |
| `cflx export` CLI command | NOW | The core command. `cflx export --format json --scope phase|spec|all --output <path>`. |

### Section 4: Lifecycle/workflow integration

- **Stages touched**: orthogonal to lifecycle (export reads current state). No `propose`/`apply`/`accept`/`archive` interaction in the basic case. The "Assets stays inside the provenance chain" variant *would* touch `apply` (export written into change directory) and `archive` (export carried into specs).
- **New commands**: `cflx export`. Single new top-level command with format/scope flags.
- **Phase artifacts affected**: only if the "exports-in-change-directory" pattern lands. Default starting form (export to user-specified path) does not touch phase artifacts.
- **Pre-commit/verification battery interaction**: none in the simple form. If exports land in change directories, they need to be excluded from `cargo fmt --check` (already are; .md/.json) and considered by `cflx.py validate` for size/freshness gates.
- **Existing primitive duplication**: cflx archive already produces a snapshot of the merged state in `openspec/changes/archive/<phase>/`. The risk: a JSON export of the spec at archive time might duplicate what archive already does in markdown. Recommendation: `cflx export` reads from the same data the archive operation reads, but produces a different format (JSON for machine consumption); both are derived views.

### Section 5: Refined verdict

**DEFER PPTX/DOCX, ADOPT Quick Export half. Preconditions sharpened.** The matrix established the split. This pass refines:

1. JSON export ships first, alone, as `cflx export --format json`. This is the highest-leverage single piece: it unlocks downstream rendering by external tools (which is exactly what spec section 5 says rendering should be).
2. Markdown export ships second, in the same phase or the next.
3. CSV is LATER, contingent on a concrete reviewer request.
4. "Assets stays inside the provenance chain" pattern goes to RESEARCH; design pass to determine whether exports are tracked artefacts or transient outputs.
5. Webui settings-pane UI surface is LATER, pending webui direction (matrix open question 1).

The scope precondition the matrix flagged ("only if there is demand"): the JSON export earns its keep on internal grounds alone (MCP integration, debug tooling, future cflx tooling) regardless of external demand. Ship it. Markdown waits for one concrete reviewer ask.

Sub-component flips from matrix:
- JSON export: DEFER → NOW.
- Markdown export: DEFER → NOW (paired or close).
- CSV: DEFER → LATER (downgraded but not flipped).
- "Assets in provenance chain" pattern: DEFER → RESEARCH (separated from the export tier decision; deserves its own design).

### Section 6: Open research questions

1. Where does an exported JSON live by default? User-specified path (most flexible), `target/cairn-export/` (analogous to `target/` build artefacts, gitignored), or `openspec/exports/` (tracked, but creates churn)?
2. What is the JSON schema? A direct serialisation of the in-memory graph, a bespoke export schema, or the same JSON-shape the MCP tooling already produces?
3. Does `cflx export` run before or after the verification battery? (Recommendation: orthogonal: export reads current state regardless of gate status.)
4. Is the "exports-in-provenance-chain" pattern worth a phase, or does it stay a design note until a concrete consumer asks for it?

---

## Section 4: cflx lifecycle integration across all four

This batch's distinguishing analysis: where each candidate hits the cflx lifecycle.

| Candidate | propose | apply | accept | archive | Pre-commit/verify | New commands | Modified commands | Phase artifact impact |
|---|---|---|---|---|---|---|---|---|
| **C1** (interview) | Yes (entirely) | No | No | No | No | `cflx interview` (gated on Phase 9) | Extend `cflx-proposal` skill | New `research/genesis.md` artefact convention |
| **C4** (pyramid salvage) | No | No | No | No | No | None (UI-only sub-components) | None | None |
| **C11** (verification states) | No | Yes (verification authoring) | Yes (gate logic) | Yes (state transitions on merge) | Yes (battery consults states) | `cflx verifications` (LATER) | `cflx accept` (gate logic), AGENTS.md instruction | `specs/` may grow `verifications:` section; tasks.md `#[ignore]` instructions update |
| **C12** (export) | No (basic form) | Maybe (Assets variant) | No | Maybe (Assets variant) | No | `cflx export` | None | None (basic); change-directory storage (Assets variant) |

### Lifecycle-stage clustering

- **Propose-stage candidates (C1)**: pre-`apply` work; the only candidate that touches the proposal phase. Risk profile: chat-UX inversion of cflx's CLI idiom. Mitigation: extend existing `cflx-proposal` skill rather than build parallel command.
- **Apply/accept-stage candidates (C11)**: the only candidate that *modifies* the verification battery's gate logic. Risk profile: imagined-but-resolved collision with cflx accept's binary gate. Mitigation: state enum is scoped (current-phase vs future-phase), making the collision illusory.
- **Lifecycle-orthogonal candidates (C4 salvage, C12)**: read-only views and exports that don't enter the gate logic. Lower-risk surface; the only design discipline needed is avoiding parallel-truth-source drift (especially for C12, `cflx export` must read from the same data archive does, not a parallel inspection pipeline).

### Duplication-avoidance with existing cflx primitives

| Candidate | Existing primitive | Duplication risk | Mitigation |
|---|---|---|---|
| C1 | `cflx-proposal` skill | High: multi-round conversation already exists | Extend the skill to write transcript as `research/genesis.md`; do not build parallel `cflx interview` until Phase 9 forces it |
| C4 (salvaged pieces) | Graph explorer at `src/ui_assets/`; `ghost`/`orphaned` reconciliation states | Medium, GAP tile may duplicate `ghost`/`orphaned` | Hold GAP at RESEARCH; ship re-center and prerequisite/enables widget directly into existing graph explorer |
| C11 | `#[ignore = "awaits phase-N"]` pattern; `phase-N.0-tests` directories | High, both are hand-rolled forms of the lifecycle | Replace `#[ignore]` with structured attribute; let `phase-N.0-tests` directory and per-test state coexist (different granularities) |
| C12 | `cflx archive` | Medium: both produce derived views | `cflx export` reads same data, different format (JSON for machines vs MD for archive); both are derived views, single source of truth maintained |

### Pre-commit/verification battery interaction summary

Only **C11** touches the battery. C1, C4, and C12 are battery-orthogonal. The C11 integration is the load-bearing engineering: the structured replacement for `#[ignore]` must (a) preserve test-skip semantics so `cargo test` continues to pass, (b) make the planned set queryable without comment-string parsing, and (c) ensure that a `Planned` verification scoped to a future phase does not silently rot past its target phase (recommend a `cflx.py validate` check that flags overdue-Planned verifications).

### Phase-artifact impact summary

- **C1**: introduces `research/genesis.md` convention (additive, no existing artefact modified).
- **C4**: zero phase-artifact impact.
- **C11**: `specs/` grows a `verifications:` section (additive); `tasks.md` `#[ignore]` references migrate; `conventions.md` section 5 and `AGENTS.md` instruction need paired updates.
- **C12**: zero phase-artifact impact in the basic form; the "exports-in-provenance-chain" variant would touch change directories.

---

## Cluster observation

Across these four, cairn's lifecycle posture is consistent: **it wants more named-state surfaces (C11 verification states; C1 transcript-as-research) and more derived-view surfaces (C12 export; salvaged C4 sub-components like prerequisite/enables widgets), but it rejects framings that flatten the two-chain topology into a single linear pipeline (C4 pyramid bundle).** The pattern: cairn welcomes structure that makes existing kernel data more queryable, navigable, or auditable; it rejects structure that re-shapes the kernel topology into a stack. The C11 flip and the C4 salvage extraction both follow the same heuristic: peel off the structural sub-piece (state enum / per-node widget / re-center affordance) from the framing wrapper (chat UX / pyramid lanes / tier hierarchy). Where the structural sub-piece preserves two-chain semantics, adopt; where the framing wrapper imports linear-pipeline shape, reject.

Stated as one principle: cairn's lifecycle wants more named transitions and more derived views; it does not want a different topology. The four candidates in Batch D each test a sub-piece of that principle, and the verdicts cluster cleanly along it.

---

## Summary (7 lines)

1. **C1**: DEFER on the interview-runner CLI (gate Phase 9), but extend `cflx-proposal` *now* to write transcripts as `research/genesis.md`; flip confidence pill from DEFER to REJECT (calibration unsolved, candidates file already flagged hard variant out-of-scope).
2. **C4**: confirm bundle REJECT, salvage two sub-components: the per-node "Prerequisite for / Enables" widget and "re-center on any node" both flip to ADOPT-now (separable from pyramid framing); GAP tile to RESEARCH (pending C2 fidelity-rollup design).
3. **C11**: ADOPT-modified scope refined: ship as a small dedicated phase introducing a `#[cflx_planned(phase=N)]` proc-macro, a five-state enum on verification artefacts, and paired updates to `conventions.md` section 5 and `AGENTS.md`; `Blocked` semantics flagged RESEARCH (Blocked vs Failed needs explicit error-code distinction).
4. **C12**: ADOPT JSON export NOW (single highest-leverage piece), Markdown NOW-or-next, CSV LATER, PPTX/DOCX confirmed DEFER (probably permanent per spec section 5); the "Assets stays in provenance chain" pattern split off to RESEARCH for its own design pass.
5. **cflx-integration headline**: only C11 touches the verification battery's gate logic; C1 is propose-stage-only; C4 salvage and C12 are lifecycle-orthogonal, meaning three of four candidates can land without disturbing accept/archive semantics, which is the right risk profile for a workflow that already gates on real evidence.
6. **Cluster observation**: cairn's lifecycle welcomes named-state surfaces and derived views (peel-the-sub-piece-off-the-framing pattern works for C4 salvage and C11 alike) but rejects framings that re-shape the two-chain topology into a linear stack, cairn wants more transitions and views, not a different topology.
7. **Highest-leverage flip**: C4's per-node "Prerequisite for / Enables" widget, the matrix's REJECT was bundle-correct but missed that this widget needs no pyramid, no tiers, no lanes; it reads cairn's existing authority-chain edges and renders them in a node-detail panel. Pure UX win at near-zero kernel cost, and the most surprising salvage in the batch.

**Stronghold path**: `/Users/george/repos/cairn/docs/strongholds/getcairn-refined-batch-D.md`
