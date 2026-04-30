# getcairn.dev to cairn: Adoption Matrix

## Debate methodology

Each of the 14 candidates from `getcairn-learnings-candidates.md` was pressure-tested against the canonical spec (`docs/spec.md` sections 1-6), CLAUDE.md's load-bearing constraints (em-dash ban, terminology preservation, two-chain framing, voice and audience), `openspec/conventions.md` (error codes, state versioning, module size), and the domain-expandability stronghold. The debate prioritises whether each candidate is *load-bearing* in cairn's enforcement story, whether it *collides* with kernel invariants, and whether the scout's initial leaning survives adversarial scrutiny. Confidence is calibrated low on candidates whose pivotal question depends on artifacts not in this repo (e.g. unbuilt UI surfaces, unspecified Phase 10 reconciler interfaces).

## Verdict summary table

| ID | Candidate (short) | Initial leaning | Verdict | Confidence | Flipped? |
|---|---|---|---|---|---|
| C1 | Multi-round interview as proposal genesis | needs-debate | DEFER | medium | no |
| C2 | Three-axis fidelity radar plus prose-nudge | adopt | DEFER | medium | yes |
| C3 | Quality Check panel with severity buckets | adopt | ADOPT | high | no |
| C4 | Causality pyramid as navigation lens | needs-debate | REJECT | high | yes (sharpened) |
| C5 | AI-derived narrative analysis (mainstay) | needs-debate (lean L) | DEFER | medium | no |
| C6 | Pipeline trace with stages, models, timing | adopt | ADOPT | medium | no |
| C7 | Per-field AI provenance tagging | needs-debate | DEFER | medium | no |
| C8 | Suggest Trace Links as queued AI affordance | adopt (queued) | ADOPT | high | no |
| C9 | Reject two-node-type flat schema | reject | REJECT | high | no |
| C10 | Reject six-type aerospace requirement enum | reject | REJECT | high | no |
| C11 | Verification status lifecycle (5-state) | needs-debate | ADOPT (modified) | medium | yes |
| C12 | Two-tier export model | needs-debate | DEFER | medium | no |
| C13 | Empty-state CTAs naming next action | adopt | ADOPT | high | no |
| C14 | Reject AI-only domain flexibility | reject | REJECT | high | no |

## Per-candidate debate

### C1: Multi-round confidence-bounded interview as proposal genesis

**Case-for**: Genesis is the boundary cairn under-invests in. A `cflx interview` mode that produces a `research` artefact tied to the resulting `decision` would fit the provenance chain natively. The two-chain topology is uniquely positioned to do interview-as-research better than getcairn.dev does it (their record is a single QA snapshot; ours would be a navigable chain into source artefacts). Phase 9 brownfield ingestion needs an elicitation surface anyway, so this earns its keep there even if developer-CLI users skip it.

**Case-against**: cflx today is deliberately CLI-and-codex-driven. Forcing a multi-round chat surface in front of `cflx apply` inverts the cost: the value is precisely that there is no chat-app to learn. Worse, the "78 percent confidence pill" is exactly the hard-confidence-gate the candidates file lists as out-of-scope. Building an interview with no gate is just a transcript log; building one with a gate imports a calibration problem we cannot solve. The current human-authored `proposal.md` plus existing `cflx-proposal` skill already capture rationale; adoption risks duplicating that.

**Pivotal question**: Does Phase 9 brownfield ingestion need an interactive elicitation step that produces structured `research` artefacts, or can it ingest existing artefact corpora deterministically?

**Verdict**: DEFER. Precondition: Phase 9 design must surface a concrete brownfield elicitation requirement that the existing `cflx-proposal` skill cannot serve. If Phase 9's design rejects interactive elicitation, this candidate becomes REJECT.

**Confidence**: medium. The pivotal question cannot be settled until Phase 9 design exists.

**Flipped from scout?**: no. Scout said needs-debate; debate concluded defer-with-precondition, which is the structured form of needs-debate.

### C2: Three-axis fidelity radar with prose-nudge banners

**Case-for**: cairn already tracks the underlying signals (`ghost`/`synced`/`orphaned`, `verified`/`external`/`unverified`, `drift`). The rollup surface is missing. A multidimensional score that refuses a single completeness number is genuinely high-leverage UI for low backend cost: it makes a non-specialist see "this node is structurally complete but evidentially thin" without flattening any taxonomy. The prose-nudge banner ("the model knows what it is, but not what it does") is a templating layer over reconciler findings, mechanically straightforward.

**Case-against**: The three axes are MBSE-shaped (Entities, Processes, Relationships). Translating to cairn's two-chain world means inventing axes, and inventing axes is exactly the kind of design work that quietly re-introduces a flat framing through naming. cairn's natural rollup is two-chain (provenance strength on the left, authority strength on the right), already present in the chain-balance widget per the domain-expandability doc. A three-axis radar that competes with the two-chain widget would either contradict it (two displays of "fidelity" disagreeing) or duplicate it. Worst case it implies a third axis that does not exist in cairn's topology.

**Pivotal question**: Can the rollup surface be expressed as the existing two-chain balance widget (left/right strength bars) extended with severity gradients, rather than as a three-axis radar that imports MBSE shape?

**Verdict**: DEFER. Precondition: a concrete UI design that uses the two-chain balance widget as the primitive (not a three-axis radar). The "prose-nudge banner translating numeric finding into plain English" portion of the candidate is unambiguously good and could be adopted independently. The radar shape is the part that risks misframing.

**Confidence**: medium. Pivotal question is a UI design call, not yet made.

**Flipped from scout?**: yes. Scout said adopt. The debate elevated the framing risk: the radar imports a three-axis taxonomy that does not exist in cairn's two-chain topology. Adopt the prose-nudge layer; defer the radar shape pending a UI design that is faithful to two chains.

### C3: Quality Check panel with severity buckets and inline remediation

**Case-for**: Mechanical fit. cairn's reconciler already produces structured findings (interface contradictions, rationale tensions, structural errors). A `cflx check` command with `--json` plus a webui Quality panel rolls these up without inventing new semantics. Severity buckets map cleanly: structural errors and interface contradictions are blocking (errors), rationale tensions are advisory (warnings), informational findings (orphaned files, missing artefact pointers) are info. The `Fix` button maps onto existing decision/contract authoring affordances and the existing `cflx apply` flow.

**Case-against**: Risk of duplicating cflx's existing verification battery output in a less canonical form. The Quality panel must not become a parallel truth source diverging from `cflx accept`. There is also a subtle naming risk: "Quality" is not a cairn term and could imply a fitness assessment that the framework deliberately does not make.

**Pivotal question**: Can the panel be wired so its findings are literally the same data structure produced by `cflx accept` (single source), or does it require a parallel inspection pipeline?

**Verdict**: ADOPT. Rationale: the underlying data exists, the rollup surface is missing, the case-against is a design discipline question (single source, not a parallel pipeline) that good engineering resolves. Name it `cflx check` (or surface as the existing reconciler findings panel) rather than "Quality Check" to avoid importing a fitness vocabulary that cairn does not own.

**Confidence**: high.

**Flipped from scout?**: no.

### C4: Causality pyramid as a navigation lens

**Case-for**: cairn has a richer two-chain topology but no equivalent visual scaffold for "what depends on what." The pyramid as a derived reading lens (not as kernel data) gives a learnable navigation surface for the authority chain without changing semantics. A non-specialist sees ordering at a glance.

**Case-against**: This is the candidate the user explicitly flagged for special rigour. CLAUDE.md says "Do NOT propose flattening the taxonomy" and "Describing the architecture as a flat six-layer stack" is on the explicit avoid list. Spec section 3 v0.5 *explicitly rejected* the flat hierarchy framing because it conflated evidence and norms. Their pyramid collapses both into one linear pipeline. The argument that adoption is "visual lens only, not kernel data" sounds reassuring but does not survive scrutiny: a five-tier pyramid as the dominant navigation surface *is* the public face of the framing. New readers will reason from the picture, not from spec section 3. Even if the underlying data remains two-chain, the pyramid teaches the wrong mental model. The risk of the visual back-dooring the misframing is not theoretical; it is the most predictable consequence of adopting a linear-pyramid lens onto a two-chain topology.

**Pivotal question**: Is there a navigation lens that captures "what depends on what" while preserving the two-chain split (e.g., a horizontal H-shape where provenance flows left-to-right into the hinge and authority flows hinge-to-right, with depth-ordering inside each chain)?

**Verdict**: REJECT. Rationale: the pyramid is load-bearing for getcairn.dev's framing precisely because their domain *is* a linear pipeline. cairn deliberately is not. Importing the visual costs the two-chain framing for a navigation gain that an H-shape or balance widget extension can provide without that cost. If a navigation lens is wanted, design one native to the two-chain topology rather than borrowing one whose shape contradicts ours.

**Confidence**: high.

**Flipped from scout?**: yes (sharpened to reject). Scout said needs-debate; the debate found that the case-against is decisive on first-principles reading of CLAUDE.md and spec v0.5's explicit framing rejection. The "visual lens only" caveat does not survive: the visual is the framing.

### C5: AI-derived narrative analysis (mainstay sentence plus systemigram)

**Case-for**: Stakeholder communication. cairn's two-chain framing is precise but invisible at the UI surface. A generated mainstay sentence over the authority chain lets a reader summarise what the system *does* in one paragraph. CLAUDE.md's voice direction explicitly broadens cairn's audience to "people building with AI tools, including non-devs"; non-devs reading a graph need a sentence-level summary to engage. The systemigram (verb-labeled curved arrows) is also valuable for marketing/landing surfaces where the audience is not in the CLI yet.

**Case-against**: Squarely in "narrate the graph" territory that does not gate any commit. cflx already passes/fails on real evidence; narrative is decoration on a system whose value comes from gating, not summarisation. Easy to over-invest in. Worse, an AI-generated sentence sitting prominently in the UI risks being read as authoritative ("the system is X"), when in fact it is an LLM gloss with no provenance link. If users start citing the mainstay sentence as truth, cairn has invented the very loose-context drift problem it exists to prevent. Per-field AI provenance (C7) would be required to stamp the sentence honestly, raising costs.

**Pivotal question**: Is non-dev stakeholder communication a near-term cairn goal (Phase 8-10), or a Phase-12+ marketing concern?

**Verdict**: DEFER. Precondition: paired with C7 (AI-provenance tagging) so generated narrative is stamped as AI-derived and traceable. Without that pairing, the narrative becomes a parallel un-attributed truth source. With it, it is a useful stakeholder lens. Also defer until cairn has a marketing or non-dev surface that needs it; today's CLI users do not.

**Confidence**: medium. Stakeholder-communication value is real but the timing question depends on roadmap, which the debate cannot settle.

**Flipped from scout?**: no. Scout leaned L (low/reject); debate concluded defer-with-pairing-precondition. The CLAUDE.md voice broadening reads as a soft case-for that survives but does not push past the gating-vs-decoration concern alone.

### C6: Pipeline trace with named stages, models, and per-stage timing

**Case-for**: Trust through transparency. Stakeholders reviewing an archived phase currently take `cflx accept`'s pass/fail at face value. Per-stage model name, token cost, and latency lets them audit *how* a phase was generated, not just whether it passed. cflx already records per-phase telemetry; promoting a structured per-stage record (`pipelineTrace`) into the artefact metadata makes AI provenance native rather than out-of-band. Aligns with the existing `Pre-Phase Tests` discipline (conventions.md section 5): test-first phases plus per-stage trace gives a near-complete audit record.

**Case-against**: Storage and schema bloat for a question most users do not ask. A new schema field on every artefact (or a sidecar JSON) requires versioning per conventions.md section 3. Token cost data has retention and privacy implications (especially if cairn is run against private codebases on shared CI). Risk of leaking model identifiers that change as Anthropic updates models, creating noisy diffs.

**Pivotal question**: Is the `pipelineTrace` per-artefact metadata, per-change metadata, or out-of-band telemetry that the archive references but does not version?

**Verdict**: ADOPT. Rationale: cflx already collects this; surfacing it as a structured per-change archive sidecar (not per-artefact-frontmatter) avoids the schema-version-bump-per-artefact-type cost while delivering the auditability win. The "per-change archive sidecar" shape side-steps the case-against without losing the case-for. Place under `openspec/changes/archive/<phase>/pipeline-trace.json` with a state-versioning header per conventions section 3.

**Confidence**: medium. Confident in the principle; less confident in the exact location until a design discussion makes the sidecar-vs-frontmatter call.

**Flipped from scout?**: no.

### C7: Per-field AI provenance tagging

**Case-for**: Sharper trust calibration. Today's binary "this artefact came from a human or an agent" loses information when only some fields were touched by the agent. Reviewers see exactly which lines need eyes. Pairs with C5 to stamp AI-narrative honestly. Aligns with cairn's existing reconciliation-state semantics: per-field provenance is to artefacts what `synced`/`ghost`/`orphaned` is to nodes.

**Case-against**: Substantial per-artefact metadata burden. Every artefact type's parser/writer changes. Pre-commit hooks treating field-tag changes as content changes will produce churn; inverse, hooks ignoring them lose the very signal the tag exists for. The frontmatter schema for each artefact type would need to grow a `field_provenance` map, adding a state-version bump per type (conventions section 3). Most painfully: human-edits-after-AI-suggested mean the tag must be downgraded on edit, which requires diff-aware tooling cairn does not have.

**Pivotal question**: Can per-field provenance be expressed as line-range markers in a single `provenance` block at the artefact bottom (small, version-bumped once, no per-field schema change), rather than as inline tokens that touch every field?

**Verdict**: DEFER. Precondition: a design proposal that uses a single bottom-of-artefact `provenance` block with line ranges rather than per-field inline tokens. Inline-token form is too costly. Bottom-block form is feasible but requires a paired phase to design. Also recommend deferring until C5 or C8 makes per-field provenance load-bearing rather than nice-to-have.

**Confidence**: medium.

**Flipped from scout?**: no.

### C8: Suggest Trace Links as a separate AI-assisted affordance

**Case-for**: Brownfield ingestion (Phase 9) is exactly where this earns its keep: scan the existing artefact corpus, propose edges (contract-to-research, decision-to-source), queue for human approval. Pairs with the rename-propagation capability already in the spec. The "queued, never auto-applied" framing protects the kernel's edge-integrity invariant. cairn's two-chain topology is precisely the kind of graph where suggested cross-cutting edges are most valuable (because manual authoring of the provenance chain is the most tedious part).

**Case-against**: Edge integrity is the very enforcement primitive cairn sells. AI-suggested edges that get accepted into the authority chain expose cairn to silent corruption of its core promise. The Phase 9 brownfield gate is exactly where un-reviewed-AI edges are most likely to slip through ("we'll review them later, just import the corpus first"). A queued affordance is only as safe as its review discipline.

**Pivotal question**: Can the queue's review boundary be made non-bypassable, e.g., suggested edges live in a separate `meta/changes/<change>/suggested-edges.md` that *cannot* be merged by `cflx accept` until a human has marked each one approved or rejected?

**Verdict**: ADOPT. Rationale: scout's framing ("queued proposals, never auto-applied") is correct, and the kernel already has the right primitive (changes are isolated until merged). Build it as a change-shaped artefact: suggested edges land in a change directory with delta semantics, and `cflx accept` refuses to merge a change containing un-triaged suggested edges. This makes the review boundary non-bypassable by construction.

**Confidence**: high.

**Flipped from scout?**: no.

### C9: Reject getcairn.dev's two-node-type flat schema

**Case-for-adoption** (steelmanning the harder side): Lower learning curve. New users do not need to learn the contract-vs-decision-vs-research distinction. The user noted their software-domain PRD experiment "worked"; perhaps the two-chain taxonomy is over-engineered.

**Case-against-adoption** (the rejection): Their `interfaceHash` is a pipe-joined ID list, not a content checksum, so they have no drift detection. Their decisions are implicit in `history[].pipelineTrace` rather than first-class queryable artefacts. Their verification is "declarative and human-asserted" rather than operational. This is the choice that costs them the entire authority chain. CLAUDE.md explicitly forbids flattening the taxonomy: "Everything else in v0.6 is kept deliberately. Do NOT propose flattening the taxonomy; it encodes distinctions the framework depends on." Their "it worked for software" is sample-size-one and the captured frame was Round 1 of 3, not the produced model.

**Pivotal question**: Is there any version of cairn that retains its enforcement value while collapsing the artefact taxonomy?

**Verdict**: REJECT (i.e., affirm the rejection). Rationale: cairn's enforcement value *is* the taxonomy distinction. There is no version that keeps the value while flattening the schema. The user instruction in CLAUDE.md is explicit. The rejection survives full pressure-testing.

**Confidence**: high.

**Flipped from scout?**: no.

### C10: Reject six-requirement-type aerospace taxonomy as our contract type

**Case-for-adoption** (steelmanning): Authoring guidance. A new user staring at a blank contract knows nothing; six concrete shapes give them somewhere to start. Even cairn's own contract template (the v1 markdown sections "Purpose", "Public interface", "Invariants", "Dependencies", "Tests") admits that authoring guidance matters. A typed enum is a stricter form of the same idea.

**Case-against-adoption** (the rejection): Their taxonomy is right for them (aerospace MBSE) and wrong for cairn. Mapping cairn contracts (interface contracts in code, declared obligations in non-code domains, eventual org/process artefacts in Phase 10) onto six aerospace-shaped types loses the broader scope. Per the domain-expandability stronghold, "environmental constraint" is meaningless for an org chart. cairn's audience is broadening to non-devs; importing a closed-enum aerospace taxonomy would force every non-code project to bend to it.

**Pivotal question**: Is there a tag-based (open) authoring-guidance affordance that gives the case-for benefit without the case-against cost?

**Verdict**: REJECT (i.e., affirm the rejection). Rationale: the closed enum is the load-bearing wrong choice. An open tag system (which cairn already has via `@security`, `@auth`, etc.) is the same authoring-guidance benefit without the closed-enum cost. The rejection is correct.

**Confidence**: high.

**Flipped from scout?**: no.

### C11: Verification status lifecycle (Passed / Planned / Draft / Failed / Blocked)

**Case-for**: cairn currently treats verification as binary at commit time (`cflx accept` passes or fails). Roadmap-shaped phases routinely declare verifications that cannot run until later phases ship the surface they verify. The pre-phase test pattern (conventions.md section 5) is exactly this: tests are written `#[ignore = "awaits phase-<N>"]` *because* there is no first-class way to mark them planned. A five-state lifecycle would let cairn model planned-but-not-yet-run verifications as first-class, replacing the `#[ignore]` workaround with a structured state.

**Case-against**: Could collide with cflx's gate-pass-or-fail invariant. If a phase ships with `Planned` verifications, what does `cflx accept` return?

**Resolving the pivotal collision**: The collision is *imagined*, not real. cflx has two distinct gates: the per-phase verification battery (which produces Passed/Failed) and the overall acceptance decision. `Planned` does not collide with the verification battery because a `Planned` verification is *out-of-scope for this phase's gate by definition*. It collides only if treated as in-scope; treated as scoped-to-future-phase, it is exactly the `#[ignore = "awaits phase-N"]` pattern the codebase already uses. The five states map cleanly: Draft = under construction (cflx ignores), Planned = scoped to future phase (cflx ignores, surfaces as expected-future-work), Passed = current phase's battery pass, Failed = current phase's battery fail (gate blocks), Blocked = upstream dependency missing (gate blocks with a different error class than Failed).

**Pivotal question**: Resolved above.

**Verdict**: ADOPT (modified). Rationale: the collision is illusory once states are scoped (current-phase versus future-phase). Adopt the lifecycle, but rename to fit cairn vocabulary: Draft, Planned (with explicit phase-target field), Passed, Failed, Blocked. Replace the existing `#[ignore = "awaits phase-N"]` pattern with `Planned(phase = N)` as a first-class state. This earns its keep by hardening the pre-phase test discipline.

**Confidence**: medium. Confident the collision is imagined; less confident on the exact rename to cairn vocabulary without a design phase.

**Flipped from scout?**: yes. Scout said needs-debate; debate resolved the collision by scoping states to current-vs-future-phase. The pre-phase test discipline already does this informally; promoting it to first-class is net positive.

### C12: Two-tier export model (raw graph plus narrative artefacts)

**Case-for**: cflx archive collapses raw-graph and composed-deliverable into one operation. Splitting into Quick (no API key, JSON/Markdown/CSV, downloaded) and Professional (API key, PPTX/DOCX, kept inside the provenance chain as Assets) pulls reviewers, executives, and non-dev collaborators into the workflow without forcing them to read raw spec deltas. The "Assets stays inside the provenance chain" pattern is the harder-to-copy and more-valuable piece: generated documents do not silently escape to a folder somewhere.

**Case-against**: AI-rendered DOCX/PPTX is a new asset surface cairn does not currently own. Adds an entire subsystem (asset library, export queue, render templates) for what is fundamentally a presentation concern. cairn's spec section 5 explicitly calls "a visual dashboard" a non-goal that belongs in downstream tools. DOCX/PPTX export is the same shape: rendering for stakeholder consumption is a downstream concern. Worse, "kept inside the provenance chain" sounds like a benefit but is a whole new artefact-storage path that has to be designed not to collide with the existing change/archive directory layout.

**Pivotal question**: Is the "Assets stays inside the provenance chain" claim genuinely separable from "build a render subsystem," or does adoption require the whole subsystem?

**Verdict**: DEFER. Precondition: extract the "Quick Export" half (JSON, Markdown, CSV) as a near-term `cflx export` command; defer "Professional Export" (PPTX, DOCX) to a downstream tool per spec section 5's "rendering is a distribution concern." Quick Export is unambiguously good and small. Professional Export is a render subsystem that does not belong in the kernel.

**Confidence**: medium. Confident on the split; less confident on whether anyone is asking for Quick Export today.

**Flipped from scout?**: no.

### C13: Empty-state CTAs that name the next concrete action

**Case-for**: Aligns directly with CLAUDE.md voice direction: "would a non-dev feel nervous typing this command or reading this doc?" Empty-state CTAs lower the nervousness floor by always pointing at the next move. Pure UX investment with low backend cost. The webui empty states are largely undeveloped; first-time users staring at fresh `cairn.config.yaml` plus empty blueprint see nothing telling them their next move.

**Case-against** (the user explicitly asked me to find one): Three real cases-against, none decisive but worth surfacing.
1. **Suggested-action drift.** "Run `cflx interview` to start" CTAs hardcoded into webui empty states will silently lie when cflx commands are renamed (cf. the Phase 2.6 terminology rename: `dsl` to `blueprint` shifted command surface). Empty-state CTAs become a maintenance surface that must track CLI changes.
2. **Naming nervousness inversion.** Per CLAUDE.md voice section: a non-dev who cannot tell what `cflx accept` does will not be reassured by an empty-state CTA that says "Run cflx accept." The CTA only helps if the named command is itself self-explanatory; otherwise it papers over a deeper docs problem.
3. **Implies a webui workflow that does not yet exist.** getcairn.dev's CTAs work because their webui is the primary surface. cairn's webui is currently a read-mostly graph explorer. Adding "Generate 3D" style CTAs implies write affordances cairn has not yet built. CTAs in a read-mostly UI become broken promises ("Click here to start", but where is "here"?).

**Pivotal question**: Is the cairn webui evolving into a write surface (where CTAs lead to actions completed in-UI), or staying read-mostly (where CTAs are pointers back to CLI commands)?

**Verdict**: ADOPT. Rationale: the cases-against are real but they shape *how* CTAs are written, not whether to have them. Specifically: write CTAs that point to CLI commands the user can copy-paste, not webui actions that imply non-existent affordances. Use cairn vocabulary (`blueprint`, `map`, `cflx`) so CTAs reinforce the voice and audience direction. Tie CTAs to the design-system tokens so the visual treatment is consistent. The case-against case-1 (drift) is mitigated by storing CTA strings in a single configurable location, not scattered.

**Confidence**: high.

**Flipped from scout?**: no. Scout said no substantive case against; debate found three real ones, but they shape implementation rather than flip the verdict.

### C14: Reject AI-only flexibility as our domain-expandability strategy

**Case-for-adoption** (steelmanning): Lower implementation cost for non-code domains. A `cflx interview` plus an AI normaliser could plausibly absorb org charts, research programmes, and product BOMs into a uniform structure without writing a non-code reconciler. Phase 10 is several phases away; if AI normalisation is good enough, why bother building a typed reconciler at all? The scout's "sample size of one and mid-Round-1" critique might understate how flexible getcairn.dev's AI pipeline actually is.

**Case-against-adoption** (the rejection): Per the domain-expandability stronghold, cairn's *kernel is already domain-neutral* by construction; the missing piece is a non-code reconciler. AI-only normalisation surrenders the structural advantage cairn already has (the two-chain topology, the typed artefact system, the change-isolation primitive) in exchange for AI-pipeline flexibility we already get for free at the artefact-authoring layer. Worse, AI-only normalisation makes drift detection impossible: if the "reality layer" for an org is an AI-summarised org chart, cairn cannot mechanically detect when reality diverges from the contract because there is no deterministic fingerprint to compare against. cairn's primary enforcement value disappears. The scout's critique stands: sample size of one, captured frame mid-Round-1, downstream specialists likely normalise toward subsystem-with-mass-and-power-budget shapes regardless of input domain.

**Pivotal question**: Can drift detection survive AI-only normalisation, or does it require a deterministic reconciler?

**Verdict**: REJECT (i.e., affirm the rejection). Rationale: AI-only normalisation is a non-starter for cairn's enforcement story. Drift detection requires a deterministic fingerprint; AI summarisation is by definition non-deterministic. Adopting the AI-only strategy would surrender cairn's primary value claim. The rejection is the right call.

**Confidence**: high.

**Flipped from scout?**: no.

## Cross-cutting findings

1. **Three candidates hinge on whether AI-suggested artefacts auto-apply or stay queued (C5, C7, C8).** All three resolve the same way: queue, do not auto-apply. The change-isolation primitive (changes are isolated until merged) is the right pattern for AI suggestions. If cairn adopts any of them, build the queue infrastructure once and reuse.

2. **The voice-and-audience broadening (CLAUDE.md) supports adoption of UX candidates more than it supports adoption of taxonomy candidates.** C2, C5, C12, C13 all gain weight from "non-devs in the audience"; C9, C10, C14 do not (because cairn's broader scope is precisely what the rejected candidates would constrain). The pattern: borrow UX vocabulary, do not borrow schema vocabulary.

3. **The pyramid-rejection (C4) and the AI-only-rejection (C14) are the same pattern at different layers.** Both are getcairn.dev shapes that work for them because their domain is rigid; both fail for cairn because cairn's domain is by-design plural. When considering future getcairn.dev borrowings, ask first: "does this borrowing assume a fixed domain shape?" If yes, default to reject; if no, evaluate on merits.

4. **Phase 9 brownfield is a forcing function for several deferred candidates (C1, C8).** Several "needs-debate" candidates resolve faster once Phase 9 design exists. The next session on "when and how to adopt" should batch these by Phase 9 readiness.

5. **The pre-phase test discipline (conventions.md section 5) is a forcing function for C11.** The existing `#[ignore = "awaits phase-N"]` workaround is a hand-rolled version of the lifecycle C11 proposes. Promoting it to first-class is the cleanup the test discipline is asking for.

## Open questions for the next session

1. **What is the cairn webui's near-term direction (read-only graph explorer versus write surface)?** This gates C13 implementation shape. It also gates whether C3's `Fix` button has anywhere to land.

2. **Does Phase 9 brownfield design need an interactive elicitation step?** This gates C1. If yes, build `cflx interview` in Phase 9; if no, reject C1.

3. **Where does pipeline-trace metadata live (per-artefact frontmatter, per-change sidecar, or out-of-band telemetry)?** This gates C6 implementation. The recommendation is per-change sidecar, but the call belongs to a design phase.

4. **Is there a non-six-axis-radar two-chain rollup widget that captures the case-for of C2 without the framing risk?** Likely the existing chain-balance widget extended with severity gradients; needs a UI design.

5. **When does the marketing/landing surface need a mainstay sentence (C5)?** Roadmap question. If Phase 12+, defer; if sooner, design with C7 pairing.

6. **Should `Planned(phase = N)` (C11) replace the existing `#[ignore = "awaits phase-N"]` test pattern, or coexist with it?** Replace is cleaner; coexist is safer during transition. Belongs in a small dedicated phase, not folded into anything else.

7. **Is `cflx export --quick` (C12 first half) wanted by any current user?** Easy to ship, but only if there is demand. Without demand, it is solving for a non-problem.
