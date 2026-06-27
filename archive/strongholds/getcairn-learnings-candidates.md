# getcairn.dev to cairn: Candidate Adoptions

## Context

cairn (this repo) is a connective-tissue framework with a two-chain architecture (Source to Research to Decision provenance, Decision to Blueprint to Contract to Code authority) that gates commits when code drifts from declarations. getcairn.dev is a different product, an AI-assisted MBSE workspace for hardware systems, that happens to share the name. Both make a "model is the artefact, not the conversation" pitch but diverge sharply at the load-bearing point: theirs anchors at human-readable documentation, ours at content-addressable interface hashes against built code. The name collision is not relevant for adoption decisions but should be acknowledged in any external comparison writing.

## Candidates

### C1: Multi-round confidence-bounded interview as proposal genesis

- **What getcairn.dev does**: Pre-build, three rounds of clarifying questions (78 percent then 82 percent confidence pills) refine a working brief and architecture-signals panel before any structural generation runs. The full Q-and-A transcript is preserved as `Project Genesis - Preserved as provenance` and shown alongside the build progress.
- **Why it might apply to cairn**: We currently start a phase from a human-authored `proposal.md`. There is no AI-driven elicitation step in front of it, and no durable record of the prompts that shaped the proposal. Their interview pattern fits cleanly as an optional `cflx interview` mode whose output is a `research` artefact tied to the resulting `decision`.
- **Strongest case for adopting**: Genesis is exactly the boundary we under-invest in. Phases today drop the agent into "implement the proposal" with the rationale already collapsed into prose. An interview mode would let us capture the full elicitation as queryable provenance, which is something our two-chain topology is uniquely positioned to do well (theirs is a single QA snapshot; ours would be a navigable chain).
- **Strongest case against**: Adds a UX surface (multi-round chat) we currently do not own and do not need for the developer-CLI workflow. cflx's value is precisely that it does not require a chat UI. Forcing one in front of `apply` could invert the cost.
- **Initial leaning**: needs-debate
- **Evidence pointers**: `docs/research/getcairn-dev/screenshots/01-round3-shape-the-starting-model.png`, `02-round3-comms-safe-state-question.png`, `03-ready-to-build-project-genesis.png`; `docs/research/getcairn-dev/08-borrow-list.md` (H entry); `docs/research/getcairn-dev/working-notes.md` entries 01-03.

### C2: Three-axis fidelity radar with prose-nudge banners

- **What getcairn.dev does**: Every node gets a Completeness lens that scores it on three axes (Entities, Processes, Relationships). The radar is paired with a yellow inline banner that translates the numeric finding into a plain-English diagnostic ("the model knows what it is, but not what it does") plus a one-click `Fix with AI` action.
- **Why it might apply to cairn**: Two patterns combined. The multidimensional scoring (refusing a single completeness number) maps cleanly onto our blueprint primitives plus contracts plus state-of-evidence. The prose-nudge surface is a thin templated layer over reconciler findings but the UX is genuinely strong.
- **Strongest case for adopting**: We already track ghost/synced/orphaned plus verified/external/unverified plus drift. We have the underlying signals; we lack the rollup surface that makes a non-specialist see "this node is structurally complete but evidentially thin." This is high-leverage UI for low backend cost.
- **Strongest case against**: Picking three axes that are stable across our broader domain scope (code plus eventual non-code) is non-trivial. Their three axes are MBSE-shaped (Entities/Processes/Relationships). Translating to our two-chain world means inventing axis names, which is design work that could go wrong.
- **Initial leaning**: adopt
- **Evidence pointers**: `docs/research/getcairn-dev/screenshots/13-completeness-three-axis-radar.png`, `14-completeness-with-fix-with-ai-banner.png`, `15-completeness-pgd-with-side-panel.png`; `docs/research/getcairn-dev/08-borrow-list.md` (H entry); wiki article `[[Triaxial Fidelity Radar]]`.

### C3: Quality Check panel with severity buckets and inline remediation

- **What getcairn.dev does**: Dedicated Quality Check surface aggregates findings across the whole model into typed severity buckets (errors, warnings, info) with a single headline count, scope toggles (Entire Model vs This Node), category filters (`COMPLETENESS`, `TRACEABILITY`), per-row `Fix` actions, and a `Re-run` button with last-run timestamp.
- **Why it might apply to cairn**: Our reconciler already produces structured findings. We lack the rolled-up surface. A `cflx check` command with `--json` plus a webui Quality panel would close the gap directly.
- **Strongest case for adopting**: Mechanical fit. The severity bucket pattern is a portable spec-linter UI shape that any system with a finding pipeline benefits from. The `Fix` button maps naturally onto our existing decision/contract authoring affordances.
- **Strongest case against**: Risk of duplicating cflx's existing verification battery output in a less canonical form. We need to be careful that the Quality panel does not become a parallel truth source diverging from cflx accept.
- **Initial leaning**: adopt
- **Evidence pointers**: `docs/research/getcairn-dev/screenshots/61-quality-check-panel.png` (referenced in working-notes); `docs/research/getcairn-dev/08-borrow-list.md` (H entry); wiki article `[[Completeness Scoring UI]]`.

### C4: Causality pyramid as a navigation lens

- **What getcairn.dev does**: Renders a five-tier pyramid (System, Domain Technologies, Parts and Materials, Instruments and Connections, Knowledge Foundation) as horizontal lanes. Each tier surfaces unresolved layers as inline `GAP` tiles, and the pyramid can be re-centered on any node. Per-node `Causal Position` widgets frame each node as "Prerequisite for X" or "System capstone, Enabled by Y".
- **Why it might apply to cairn**: We have a richer two-chain topology but no equivalent visual scaffold. A pyramid view as a derived reading lens (not as kernel data) would give a learnable navigation surface for our authority chain without changing semantics.
- **Strongest case for adopting**: It is genuinely useful UX. A non-specialist sees "what depends on what" without having to read the full graph. Reuses authority-chain edges we already compute.
- **Strongest case against**: Their pyramid collapses evidence and norms into a single linear pipeline; that is exactly the framing v0.5 explicitly rejected for cairn ("Describing CAIRN as 'six layers' flattens the topology"). Adopting the visual could re-introduce the misframing through the back door even if the underlying data remains two-chain. Significant risk of confusing first-time readers about cairn's actual topology.
- **Initial leaning**: needs-debate
- **Evidence pointers**: `docs/research/getcairn-dev/screenshots/30-causality-pyramid-system.png`, `31-causality-lanes-rov.png`, `36-subsystem-causal-position.png`, `37-capstone-causal-position.png`; `docs/research/getcairn-dev/08-borrow-list.md` (H entry); `docs/research/getcairn-dev/07-ontology-comparison.md` section 8; `CLAUDE.md` v0.5 framing rejection.

### C5: AI-derived narrative analysis (mainstay sentence plus systemigram)

- **What getcairn.dev does**: An AI-generated `_narrativeAnalysis` layer attached to the system root: `mainstaySentence` (English summary of the canonical causal chain), `cards[]` (prose narrative blocks), `connections[]` with `verbPhrase` per edge ("energises and commands"), `islands[]` for subgraphs disconnected from the mainstay, plus `layout` and `mainstayPath` for rendering. Renders as a Boardman-style systemigram diagram with verb-labeled curved arrows.
- **Why it might apply to cairn**: Distinctive to them; we have no analogue. The pattern is portable. The shape of the input (interface graph plus typed edges) is compatible with our blueprint plus contracts. The shape of the output (mainstay sentence plus verb-labeled connections) is generic.
- **Strongest case for adopting**: Stakeholder communication. cairn's two-chain framing is precise but invisible at the UI surface; a generated mainstay sentence over our authority chain would let a reader summarise what the system *does* in one paragraph without abandoning kernel semantics.
- **Strongest case against**: Squarely in "narrate the graph" territory that does not gate any commit. Adoption is additive expressivity, not load-bearing. cflx already passes/fails on real evidence; the narrative is decoration on a system whose value comes from gating, not summarisation. Easy to over-invest in.
- **Initial leaning**: needs-debate (lean L per the prior pass)
- **Evidence pointers**: `docs/research/getcairn-dev/screenshots/54-narrative-pre-generation.png`, `56-systemigram-usv-rov-narrative.png`; `docs/research/getcairn-dev/07-ontology-comparison.md` section 7; `docs/research/getcairn-dev/08-borrow-list.md` (L entry); wiki article `[[Mainstay Primitive]]`, `[[Systemigram Primitive]]`.

### C6: Pipeline trace with named stages, models, and per-stage timing

- **What getcairn.dev does**: Every AI specialist run exposes a four-stage pipeline (`ROUTE -> CONTEXT -> GENERATE -> VALIDATE`) with the stage 3 label specialising to operation kind (`REQUIREMENTS`, `DECOMPOSE`, etc). The post-run review modal shows per-stage model name (Router Haiku, Requirements Sonnet) plus wall-clock seconds plus token cost (`~13.2k tokens / ~$0.0790`). Long-running operations show a live elapsed counter with honest "typically 30 to 60s" expectation copy.
- **Why it might apply to cairn**: cflx already records per-phase telemetry. We do not surface it in cflx archive output as a structured per-stage record. Adopting `pipelineTrace`-shaped per-artefact metadata (stage name, model, input/output tokens, latency, success flag) would harden the authority chain by making AI provenance native rather than out-of-band.
- **Strongest case for adopting**: Trust through transparency. Stakeholders reviewing an archived phase currently must take cflx accept's pass/fail at face value. Per-stage model and cost data lets them audit not just whether it passed but how it was generated.
- **Strongest case against**: Storage and schema bloat for a question most users do not ask. cflx telemetry already exists; promoting it to kernel data is a real spec change for marginal day-to-day value.
- **Initial leaning**: adopt
- **Evidence pointers**: `docs/research/getcairn-dev/screenshots/18-generation-pipeline-route-context-generate-validate.png`, `26-pipeline-trace-zoom-named-models.png`, `27-generation-pipeline-requirements-stage-specialized.png`, `22-ai-reasoning-panel-token-cost.png`; `docs/research/getcairn-dev/08-borrow-list.md` (M entry "AI Reasoning panel"); wiki article `[[Pipeline Cost Transparency]]`, `[[Generation Pipeline (REQ)]]`.

### C7: Per-field AI provenance tagging

- **What getcairn.dev does**: Every property value carries an inline `. ai .` token in its category line. Authored values lack the tag. The user can see at a glance which numbers are commitments and which are AI proposals.
- **Why it might apply to cairn**: We track authored-vs-generated at the artefact level (declared blueprint vs scanned map) but not at the property-or-field level inside an artefact. Per-field tagging would let the user see "the title was authored, but the rationale was AI-suggested" inside a single decision artefact.
- **Strongest case for adopting**: Sharper trust calibration. Today's binary "this artefact came from a human or an agent" loses information when only some fields were touched by the agent. Reviewers can act faster when they see exactly which lines need eyes.
- **Strongest case against**: Substantial per-artefact metadata burden plus parser/writer changes across every artefact type. Could also leak into pre-commit churn if hooks treat field-tag changes as content changes.
- **Initial leaning**: needs-debate
- **Evidence pointers**: `docs/research/getcairn-dev/screenshots/09-node-detail-properties-budgets.png`, `10-node-detail-decompose-generate.png`, `35-property-suggestion-chips.png`; `docs/research/getcairn-dev/08-borrow-list.md` (M entry "AI badge on every generated value").

### C8: Suggest Trace Links as a separate AI-assisted affordance

- **What getcairn.dev does**: The traceability matrix view distinguishes manual `+ Add Link` from AI-driven `+ Suggest Links`. The suggest action runs through the same four-stage pipeline as other AI specialist runs. The split is honest about cost and probability.
- **Why it might apply to cairn**: We have provenance and authority chain edges but no `cflx suggest-links` affordance. Adding one would let an AI specialist propose cross-cutting edges (contract to research, decision to source) for human review, separately from deterministic edges declared in the blueprint.
- **Strongest case for adopting**: Brownfield ingestion is on our roadmap (Phase 9). A trace-link suggester is exactly the kind of tool brownfield work needs: scan the existing artefact corpus, propose edges, queue for human approval. Pairs directly with that phase.
- **Strongest case against**: AI-suggested edges that get accepted into the authority chain expose us to silent corruption of the very enforcement primitive we sell. Needs careful gating: suggested edges should be queued, not committed, until reviewed. Higher risk than other AI assistance because the kernel relies on edge integrity.
- **Initial leaning**: adopt (but only as queued proposals, never auto-applied)
- **Evidence pointers**: `docs/research/getcairn-dev/screenshots/29-trace-link-agent-route-stage.png`, `49-traceability-matrix-three-axis.png`; `docs/research/getcairn-dev/08-borrow-list.md` (M entry); wiki article `[[Traceability & Cascade Impact]]`.

### C9: Reject getcairn.dev's two-node-type flat schema

- **What getcairn.dev does**: Their entire data model uses two surface node types (`system`, `subsystem`) with one uniform record shape across all types. Differences across types live in a free-form `properties` bag, not in the schema. Cross-cutting structure is reified as foreign-key fields scattered across collections (no top-level `edges[]`).
- **Why it might apply to cairn**: A simpler schema is easier to serialise, render, and ingest. The user already noted their software-domain PRD experiment worked. Tempting to ask whether our two-family taxonomy (blueprint primitives plus six artefact direct types) is over-engineered.
- **Strongest case for adopting**: Lower learning curve. New users would not need to learn the contract-vs-decision-vs-research distinction.
- **Strongest case against**: This is the choice that costs them the entire authority chain. Their `interfaceHash` is a pipe-joined ID list, not a content checksum, so they have no drift detection. Their decisions are implicit in `history[].pipelineTrace` rather than first-class queryable artefacts. Their verification is "declarative and human-asserted" rather than operational. Adopting their flatness would surrender exactly the structural advantages our spec calls out (`07-ontology-comparison.md` "Where ours has structural advantages"). The taxonomy encodes load-bearing distinctions; CLAUDE.md explicitly forbids flattening it.
- **Initial leaning**: reject
- **Evidence pointers**: `docs/research/getcairn-dev/07-ontology-comparison.md` sections 1, 2, 6, 9; `CLAUDE.md` "Everything else in v0.6 is kept deliberately"; wiki article `[[Edge Model Comparison]]`, `[[Schema Comparison]]`.

### C10: Reject six-requirement-type aerospace taxonomy as our contract type

- **What getcairn.dev does**: Closed `requirement.type` enum: `functional`, `performance`, `interface`, `safety`, `environmental`, `constraint`. Every requirement must pick one.
- **Why it might apply to cairn**: We have `contract` as a broader umbrella. A typed taxonomy could give clearer authoring guidance.
- **Strongest case for adopting**: Authoring guidance. A new user staring at a blank contract knows nothing; the taxonomy gives them six concrete shapes to start from.
- **Strongest case against**: Their taxonomy is right for them (aerospace MBSE) and wrong for us. Mapping our contracts (interface contracts in code, declared obligations in non-code domains, eventual org/process artefacts in Phase 10) onto six aerospace-shaped types would lose the broader scope. Our framework explicitly aims at "people building with AI tools, including non-devs"; "environmental constraint" is meaningless for an org chart.
- **Initial leaning**: reject
- **Evidence pointers**: `docs/research/getcairn-dev/07-ontology-comparison.md` section 3; `docs/research/getcairn-dev/08-borrow-list.md` (Skip entry); `docs/strongholds/cairn-domain-expandability.md` recommendations on domain neutrality.

### C11: Verification status lifecycle (Passed / Planned / Draft / Failed / Blocked)

- **What getcairn.dev does**: Verification records carry a five-state status enum: Draft (under construction), Planned (committed but not run), Passed, Failed, Blocked. Captured in screenshot 28 and the docs.
- **Why it might apply to cairn**: We currently treat verification as binary at commit time (cflx accept either passes or fails). Adopting a five-state lifecycle for verification artefacts would let us model planned-but-not-yet-run verifications as first-class. Useful for phase planning where a verification is committed in a phase but executed later.
- **Strongest case for adopting**: Roadmap-shaped phases routinely declare verifications that cannot run until later phases ship the surface they verify. We currently have no clean way to declare "this verification exists, will run in phase N+2." A status lifecycle handles it.
- **Strongest case against**: Could collide with cflx's gate-pass-or-fail invariant. If a phase ships with `Planned` verifications, what does cflx accept return? Adding new states to a binary system risks breaking the gating story unless states are clearly out-of-band.
- **Initial leaning**: needs-debate
- **Evidence pointers**: `docs/research/getcairn-dev/screenshots/28-verification-table-draft.png`; `docs/research/getcairn-dev/08-borrow-list.md` (M entry); `docs/research/getcairn-dev/07-ontology-comparison.md` section 4; wiki article `[[Verification Methods]]`, `[[Verification Panel]]`.

### C12: Two-tier export model (raw graph plus narrative artefacts)

- **What getcairn.dev does**: Settings export splits by where the work happens and where it lands. Quick Export (no API key, instant): JSON, Markdown, Requirements CSV, downloaded. Professional Export (API key, ~30s): PPTX presentation, DOCX report, saved into project Assets (kept inside the provenance chain rather than escaping to the filesystem).
- **Why it might apply to cairn**: cflx archive collapses raw-graph and composed-deliverable into one operation. The split has UX merit: a reviewer can grab JSON for round-trip use or DOCX for a stakeholder meeting from the same surface.
- **Strongest case for adopting**: Pulls reviewers, executives, and non-dev collaborators into the cairn workflow without forcing them to read raw spec deltas. The "Assets stays inside the provenance chain" pattern is the harder part to copy and the more valuable: generated documents do not silently escape to a folder somewhere.
- **Strongest case against**: AI-rendered DOCX/PPTX is a new asset surface we do not currently own. Adds an entire subsystem (asset library, export queue, render templates) for what is fundamentally a presentation concern.
- **Initial leaning**: needs-debate
- **Evidence pointers**: `docs/research/getcairn-dev/screenshots/64-settings-export-tiers.png`; `docs/research/getcairn-dev/07-ontology-comparison.md` section 10; `docs/research/getcairn-dev/08-borrow-list.md` (M entry); `docs/research/getcairn-dev/_export-analysis.md`.

### C13: Empty-state CTAs that name the next concrete action

- **What getcairn.dev does**: Empty subsystem detail pages show a diamond icon, prompt text "Start decomposing Launch and Recovery System / Break this node into subsystems, add requirements, or define interfaces", and an orange `Open Command Palette` CTA. 3D Viewer empty state shows a placeholder card, a heading "No 3D model generated", body "Create an interactive 3D mesh from your system description", and a primary `Generate 3D` button. Every empty state names a concrete next action.
- **Why it might apply to cairn**: Our webui empty states are largely undeveloped. A first-time user staring at a fresh `cairn.config.yaml` plus an empty blueprint sees nothing telling them "your next move is to declare a System node and set its path." This is pure UX investment with low backend cost.
- **Strongest case for adopting**: Aligns directly with CLAUDE.md's stated voice direction: "would a non-dev feel nervous typing this command or reading this doc?" Empty-state CTAs lower the nervousness floor by always pointing at the next move.
- **Strongest case against**: None substantive; this is plain good UX. The only cost is the design work to figure out which empty states exist and what each should suggest.
- **Initial leaning**: adopt
- **Evidence pointers**: `docs/research/getcairn-dev/screenshots/08-subsystem-empty-state-decompose-prompt.png`, `42-3d-viewer-empty-state.png`; `docs/research/getcairn-dev/working-notes.md` entries 08, 42; wiki article `[[3D Viewer Empty State]]`.

### C14: Reject AI-only flexibility as our domain-expandability strategy

- **What getcairn.dev does**: Sample size of one. A user fed a software-domain PRD (OpenSpine) into the platform and the UI accepted it without complaint, ran it through the same Round 1 of 3 flow with software-flavored decomposition questions ("intended deployment target and operator profile"), 72 percent confidence pill. Their domain flexibility appears to be supplied by AI specialists at inference rather than by a richer schema.
- **Why it might apply to cairn**: Tempting hypothesis: maybe our two-chain framework is over-engineered and we could ship faster by leaning harder on AI normalisation rather than typed artefacts.
- **Strongest case for adopting**: Lower implementation cost for non-code domains. A `cflx interview` plus AI normaliser could plausibly absorb org charts, research programs, and product BOMs into a uniform structure without us writing a non-code reconciler.
- **Strongest case against**: The sample size is one and the captured frame is mid-Round-1; we do not know whether downstream specialists carry the software domain through into the resulting model or normalise toward subsystem-with-mass-and-power-budget shapes. Worse, our domain-expandability stronghold (`docs/strongholds/cairn-domain-expandability.md`) shows that the *kernel* is already domain-neutral; the work to do is build a non-code reconciler interface, not reduce schema. Adopting their AI-only strategy would surrender the structural advantage we already have.
- **Initial leaning**: reject (treat as logged hypothesis only, per prior pass; do not let it influence Phase 10 design)
- **Evidence pointers**: `docs/research/getcairn-dev/screenshots/65-software-domain-prd-decomposition.png`; `docs/research/getcairn-dev/working-notes.md` "Observation: software-domain PRD ingestion (single sample)"; `docs/research/getcairn-dev/07-ontology-comparison.md` section 12; `docs/research/getcairn-dev/08-borrow-list.md` (L entry); `docs/strongholds/cairn-domain-expandability.md`.

## Out of scope

- **3D mesh generation pipeline**. Genuinely impressive (Claude Vision plus MeshBuilder DSL plus glTF export, 14,784-vertex tugboat in-app), but cairn is not in the geometry domain. Borrow does not fit our two-chain topology and would require a substantial scope expansion.
- **Per-property unit registry at the kernel** (`units[]`, `dataTypes[]`, `params[].unitId`). Useful for systems engineering but our scope is broader (code plus eventual non-code). SI factors do not belong in the kernel; they belong in domain extensions if anywhere.
- **Twelve-lens fixed taxonomy** (Overview, Brief, Visuals, Requirements, Architecture, Causality, Completeness, Narrative, Dendritic, Behavior, Verification, Operational). Closed and product-specific. Our neighbourhood and query system is programmatic and extensible; adopting a fixed lens set would constrain our query surface for no gain.
- **Hard confidence-score gate**. Their interview surfaces a confidence pill (78 percent then 82 percent); whether it is a soft indicator or a hard gate is unclear from the public surface. Adopting a hard variant would force the AI to commit to a number that is not meaningfully calibrated.
- **2D gallery render style picker** (Photorealistic / Blueprint / Concept Art / Clay / Isometric / Exploded). The category framing is theoretically borrowable for a "render the authority chain as Mermaid graph with style X" affordance, but we have no concrete user need for stylised graph rendering today. Revisit if a marketing surface ever needs it.
- **State machines and transitions** (`states[]`, `transitions[]` collections in their schema). Behavioural-modelling primitive we have no kernel-level analogue for; not on our roadmap and not load-bearing for code-or-org reconciliation.
