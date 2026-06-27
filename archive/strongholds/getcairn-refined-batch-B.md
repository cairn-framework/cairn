# Refined adoption analysis: Batch B (rejections)

## Scope

Candidates C9, C10, C14. Framework: 4-dimension analysis with emphasis on identity-clarifying lessons and salvageable sub-components. The matrix already affirmed all three rejections at high confidence. This pass extracts what cairn *becomes* by virtue of these rejections, and probes for sub-component salvage so a clean reject does not throw away portable adjacent ideas.

A note on method: rejections are the most expensive candidates to analyse honestly because the easy move is to nod at the matrix and move on. The harder move (and the only useful one) is to steelman the case-for one more time per candidate, then articulate the positive principle the rejection commits cairn to. That is what is below.

---

## C9: Reject flat 2-node schema

### 1. Problem-solution clarity

**What problem the choice solves for getcairn.dev.** Their domain (hardware MBSE) has a fixed shape: a root system, four to seven subsystems, key interfaces, requirements, verifications. Two surface node types (`system`, `subsystem`) plus a free-form `properties` bag plus foreign-key fields scattered across collections is exactly enough structure for that fixed shape. It minimises serialisation surface, simplifies rendering (one record renderer covers everything), and lowers the learning curve for a non-specialist who just wants to draw a system diagram. For their domain, the flatness is right-sized.

**What adopting it would break for cairn.** It would dissolve the *typed artefact taxonomy*, which is the kernel primitive that makes the authority chain mechanically enforceable. Specifically: `contract` is the only place an `interface hash` can live, because the hash is computed from the contract's declared interface surface and reconciled against the code reconciler's fingerprint of reality. A flat schema with one record shape has no place to put an interface hash that is distinct from arbitrary `properties`. Their `interfaceHash` (per the ontology comparison) is a pipe-joined ID list, not a content checksum. That is not an interface hash in cairn's sense; it is an edge identifier. Adopting flatness would force cairn into the same shape, which means surrendering drift detection, the gating story, and ultimately the whole authority chain.

**Honest sliver check.** Is there *any* part of cairn where flat-record uniformity might fit? The closest is the *rendering* layer of the webui inspector: every artefact card today has a different shape (decision shows status/revisited, contract shows interface section, research shows synthesis), and a uniform card chrome with type-specific content slots could read more cleanly than the current bespoke renders. That is a UI affordance, not a schema concession. The schema-level rejection holds; a uniform-chrome rendering pattern is a separate question handled under salvage below.

### 2. Layer classification

**Layers the choice would touch if adopted.** Architectural (kernel), process (everything authoring-shaped), UI/UX (inspector, blueprint editor, change diff display). It would touch the parser, the reconciler interface, the artefact registry, the change-delta semantics, the drift detector, the pre-commit hook, and the entire spec section 3 framing.

**Layers the rejection itself clarifies.** Architectural identity. The rejection says: cairn's kernel is *typed-by-design*, and the types are not decorative. Each direct artefact type (`contract`, `decision`, `todo`, `research`, `review`, `source`) carries different *obligations* in the two-chain topology: contracts produce interface hashes, decisions hinge between provenance and authority, sources only flow inward, etc. The rejection sharpens the architectural layer specifically, and it has soft consequences for the process and UI layers (they can use uniform vocabulary without dissolving the underlying schema).

### 3. Salvageable sub-components

**(a) Uniform record chrome in the UI inspector.** Their entire product reads with one card aesthetic for any node, swapping content blocks based on type. cairn's inspector currently has bespoke rendering per artefact type, which means the inspector's visual rhythm is inconsistent. A uniform-chrome pattern (consistent header, consistent footer, type-specific middle slots) would be a UI win that imports zero schema flatness. Verdict: **ADOPT** as a webui inspector convention. Tracked as a downstream UI candidate, not a kernel concern.

**(b) `properties` bag for type-specific extension fields.** Their flat record uses a free-form `properties` bag that absorbs domain variance. cairn could plausibly add a small `extension` block on each artefact frontmatter for project-specific metadata that is not load-bearing (e.g., a custom `priority` field on a decision). This is genuinely tempting. Verdict: **REJECT**. Reason: project config already supports `artefact_types` overrides per the spec (§6 project config block, surfaced in the domain-expandability stronghold). A `properties` bag would compete with that mechanism and create two ways to extend artefact schemas, which is exactly the kind of duplication cairn should not introduce. The existing `artefact_types` config is the right extension point.

**(c) One-renderer-fits-all for graph nodes.** Their graph view renders all nodes with the same SVG primitive (size differs, content varies). cairn already does something similar for blueprint primitives (System/Container/Module/Actor share a card shape). No change needed; cairn already converged on this where it makes sense. Verdict: **already done**, no salvage required.

### 4. Identity-clarifying lesson

Because cairn rejects the flat 2-node schema, cairn is committed to the principle that *typed artefacts encode obligations, not just labels*: each direct artefact type (`contract`, `decision`, `todo`, `research`, `review`, `source`) participates differently in the two-chain topology, and the kernel's enforcement value is a function of that differentiation. A schema flat enough to make all artefacts interchangeable is a schema that makes the authority chain unenforceable.

Quotable form for spec.md or CLAUDE.md: *"Artefact types are obligation-bearing, not decorative. Each direct type's place in the provenance or authority chain determines what the kernel can enforce about it. Flattening the taxonomy collapses the obligations into labels, and labels are not enforceable."*

### 5. Refined verdict

REJECT, confirmed at high confidence. One sub-component flips: uniform-chrome rendering in the webui inspector is a UI portability win that survives the schema rejection. Catalogued separately as a downstream UI candidate.

### 6. Open research questions

- Would a future "lightweight artefact" type (e.g., a bare note attached to a node, no obligation chain) ever benefit from a flatter shape? Probably yes, and that already exists in the form of the `todo` type, which is the closest cairn has to a flat record. The question is whether *more* of those exist in the latent design space; if yes, they should be added as named direct types (not as a generic flat shape).
- If cairn ever adds a non-code reconciler (Phase 10), do any new artefact types emerge for that domain (e.g., `role-description`, `process-step`)? Almost certainly. The question becomes whether they enter as new direct types or as project-config schema overrides on existing types. The domain-expandability stronghold leans toward the latter.

---

## C10: Reject 6-type aerospace requirement enum

### 1. Problem-solution clarity

**What problem the choice solves for getcairn.dev.** Aerospace MBSE is a domain where requirements are formally categorised, and a closed enum (`functional`, `performance`, `interface`, `safety`, `environmental`, `constraint`) maps to industry standards (ECSS, MIL-STD, ARP). For their users, the enum is a recognition primitive: a safety engineer scanning requirements can filter to the safety bucket without reading prose. The closed shape is also a good authoring scaffold: when a user starts a new requirement, the type-picker is the first decision they make, and that decision constrains the rest of the form.

**What adopting it would break for cairn.** The closed enum is the wrong taxonomy for cairn's broader scope. cairn's contracts include interface contracts in code, declared obligations in non-code domains, and (per the domain-expandability stronghold) eventual org/process/research artefacts in Phase 10. Mapping a Phase 10 org chart's role description onto "environmental constraint" or "performance" is meaningless. The enum is not just *too small* for cairn; it is *aerospace-shaped*, and aerospace is one domain among many cairn aims to support. Adopting it would force every non-aerospace project to bend authoring guidance to a vocabulary that does not describe their domain, which directly contradicts CLAUDE.md's voice and audience direction ("people building with AI tools, including non-devs").

**Honest sliver check.** Is there any part of cairn where a closed enum on contract type might fit? The closest sliver is *within the code reconciler's scope*. Code-domain contracts probably do partition cleanly into a small set of shapes (interface contract, invariant contract, dependency contract, test contract). If a closed enum were ever introduced, it would belong as a code-reconciler-specific tag taxonomy, not as a kernel type. And even there, the existing open `@`-tag system (per the matrix's pivotal-question resolution) covers the same ground without the closed-enum cost. Verdict: no salvageable sliver at the kernel layer.

### 2. Layer classification

**Layers the choice would touch if adopted.** Architectural (kernel artefact schema for `contract`), process (every contract authoring flow gets a type-picker), UI/UX (inspector filters, change diff display, badge rendering).

**Layers the rejection itself clarifies.** Architectural identity *and* domain-scope identity. The rejection says: cairn's kernel is *domain-plural by construction*, and domain-specific authoring vocabulary belongs in *project config* (per spec §6 `artefact_types` overrides) or *open tags*, never in a closed enum baked into the kernel. The rejection has process-layer consequences too: it commits cairn to authoring-guidance via templates and tags, not via mandatory taxonomies.

### 3. Salvageable sub-components

**(a) Structured authoring guidance: non-aerospace shape.** This is the high-leverage salvage. The case-for in the matrix is real: a new user staring at a blank contract knows nothing, and "six concrete shapes" is genuine onboarding value. The salvageable form is *contract templates* (multiple, configurable, project-selectable) rather than a *contract type enum* (one closed, kernel-baked). cairn already has a v1 template (Purpose / Public interface / Invariants / Dependencies / Tests) per the domain-expandability stronghold. Verdict: **ADOPT** the principle of "templated authoring scaffolds for each declared contract type." Specifically:
  - Ship *multiple* default templates for the code domain (e.g., `interface-contract.tmpl`, `invariant-contract.tmpl`, `data-contract.tmpl`).
  - Allow project config to register additional templates for project-specific contract shapes.
  - Surface template choice as the first authoring step, mimicking the type-picker UX without importing a closed enum.

**(b) Filter-by-type UI affordance.** Their UI lets users filter the requirement list to one type at a time. cairn could expose the same affordance over open tags (filter to `@security`-tagged contracts, etc.). This is just the existing query system surfaced differently. Verdict: **ADOPT** as a webui filter widget over the existing tag system. Already implementable with current primitives.

**(c) Per-type form-validation rules.** Their typed enum drives form validation (a `safety` requirement must declare a hazard reference). cairn does not need this at the kernel layer; project config can declare per-template required sections. Verdict: **RESEARCH**: depends on whether project-config-driven template validation is wanted as a near-term feature. Low priority.

### 4. Identity-clarifying lesson

Because cairn rejects the closed 6-type aerospace enum, cairn is committed to the principle that *authoring guidance is delivered via configurable templates and open tags, never via closed kernel-baked enums*. Domain-specific vocabulary lives in project config or in tag conventions, both of which are extensible. The kernel speaks taxonomy; the project speaks domain.

Quotable form: *"cairn's authoring guidance is template-driven and tag-extensible, not enum-bound. The kernel ships a generic contract type; projects compose domain vocabulary on top via templates and tags. A closed enum would constrain cairn's domain scope at the kernel layer; templates and tags do not."*

### 5. Refined verdict

REJECT the closed enum, confirmed at high confidence. One sub-component flips strongly: *templated authoring scaffolds* are a real adoption with a clean implementation path (project config already supports `artefact_types` overrides per spec §6). The "six concrete shapes" intuition becomes "six (or more, or fewer, or project-specified) templates."

### 6. Open research questions

- Are there any contract sub-shapes that show up frequently enough across cairn's current users (mostly the bootstrap fixture and the codebase itself) to justify shipping more than one default template? The honest answer is probably: not yet, but Phase 9 brownfield will create demand.
- Should template selection be reflected in artefact frontmatter as an informational field (`template: interface-contract`) so the change diff and inspector can render template-specific affordances? Probably yes, but it is a separate design call that should not block the rejection.

---

## C14: Reject AI-only flexibility as our domain-expandability strategy

### 1. Problem-solution clarity

**What problem the choice solves for getcairn.dev.** They have one sample (the OpenSpine software-domain PRD experiment, captured at Round 1 of 3). Their AI pipeline accepted the input without complaint and ran software-flavored decomposition. Whether downstream specialists carry the software domain through into the produced model or normalise toward subsystem-with-mass-and-power-budget shapes is unknown. What is known: their *strategy* is to absorb domain variance via AI normalisation rather than via a richer schema or a pluggable reconciler. For an MBSE platform that wants to demo "look, it works on software too," AI-only flexibility is plausibly the cheapest path.

**What adopting it would break for cairn.** It would make drift detection impossible. cairn's primary enforcement value is the deterministic interface hash: declared contract surface vs reconciler-computed reality fingerprint, mechanically compared, drift detected, commit gated. AI summarisation is by definition non-deterministic; an AI-summarised "reality layer" cannot be deterministically fingerprinted, which means drift cannot be mechanically detected, which means the authority chain's mechanical-checkability claim disappears. Per the domain-expandability stronghold §"Recommendations": *"AI-only normalisation makes drift detection impossible: if the 'reality layer' for an org is an AI-summarised org chart, cairn cannot mechanically detect when reality diverges from the contract because there is no deterministic fingerprint to compare against. cairn's primary enforcement value disappears."* The rejection survives the steelmanned case-for at full force.

**Honest sliver check.** Is there *any* place where AI normalisation might earn a role in cairn? Yes, but a constrained one: at the *authoring layer*, not the *reality layer*. AI assistance in drafting a contract, suggesting trace links (C8), or producing a research synthesis (within a queued, human-reviewed workflow) is genuine value. AI normalisation as a substitute for the deterministic reconciler interface is the part that fails. The sliver is real but small, and it is already covered under C8 (queued AI suggestions) and C5/C7 (narrative with provenance tagging). Nothing in C14 is salvageable that is not already covered by other candidates.

### 2. Layer classification

**Layers the choice would touch if adopted.** Architectural (kernel reconciler interface), process (Phase 10 design), UI/UX (interview surfaces). Most consequentially: it would touch the *enforcement layer* by removing the deterministic-fingerprint requirement, which is the layer cairn is named for (a cairn is a marker pile; the marker is the fingerprint).

**Layers the rejection itself clarifies.** Architectural identity, *most strongly*. The rejection says: cairn's domain expandability is *deterministic-reconciler-driven*, not *AI-normalisation-driven*. Per the domain-expandability stronghold, the kernel is already domain-neutral; the work to do for non-code domains is build a deterministic non-code reconciler interface (with structured-data reality layer and a fingerprinting strategy that survives diff), not lean on AI summarisation. The rejection commits cairn to a specific Phase 10 strategy.

### 3. Salvageable sub-components

**(a) Interview-driven onboarding (constrained form).** Their multi-round interview is genuinely useful for *eliciting authoring input*, even if it is not useful for normalising reality. cairn could plausibly adopt an interview surface that produces a `research` artefact (and feeds a `decision`) without any kernel concession. This is precisely candidate C1 in the matrix, deferred pending Phase 9 design. Verdict for the C14 sliver: **ADOPT** in spirit, but note that C1 is the actual carrier of this sub-component. C14's rejection does not block C1's eventual adoption; it constrains its shape (interview produces deterministic artefacts that flow into the provenance chain, not AI-normalised "reality" that substitutes for a reconciler).

**(b) AI assistance at the authoring layer (not the reality layer).** AI specialists drafting contracts, suggesting trace links, producing narrative summaries. All covered by C5, C7, C8. Verdict: **ADOPT via the other candidates**, not via C14. C14's role is to clarify that AI-at-authoring is welcome but AI-as-reconciler is not.

**(c) Confidence pills as a soft authoring signal.** Their Round-1 confidence pill (72 percent for OpenSpine) signals to the user "here is how confident the AI is." cairn could plausibly surface AI specialist confidence on suggested edges or drafted contracts. Verdict: **RESEARCH**: confidence calibration is a known unsolved problem in LLM tooling, and a hard-confidence-gate is on the candidates' explicit out-of-scope list. A *soft* indicator might be useful in inspector chrome, but it depends on whether anyone has solved the calibration problem.

### 4. Identity-clarifying lesson

Because cairn rejects AI-only flexibility as the domain-expandability strategy, cairn is committed to the principle that *domain extension is delivered via deterministic reconcilers, not via AI normalisation of reality*. The reality layer for any domain (code, org, process, research programme) must produce a deterministic fingerprint that drift detection can compare against. AI assistance is welcome at the authoring layer (drafting, suggesting, narrating) but the reality layer must remain mechanically checkable.

Quotable form: *"cairn extends to new domains by adding deterministic reconcilers, not by leaning on AI to normalise reality. The reality layer must produce a content-addressable fingerprint; without it, drift detection is impossible and the authority chain collapses to documentation. AI assistance lives at the authoring layer, never at the reality layer."*

### 5. Refined verdict

REJECT, confirmed at high confidence. The interview-driven-onboarding sub-component is salvageable but properly belongs to C1's domain (deferred pending Phase 9), not to C14's. The AI-at-authoring sub-components are salvageable but already covered by C5/C7/C8. C14's rejection holds without sub-component flips; its job is *negative space*, it tells cairn what *not* to do at the reality layer, which sharpens the Phase 10 design brief.

### 6. Open research questions

- What is the deterministic fingerprinting strategy for non-code reality layers? For code, it is extracted symbols + signatures hashed. For an org chart in YAML, is it sorted role IDs + accountability-section hashes? For a research programme, is it hash of citation list + question text? The domain-expandability stronghold flags this as Open Question 2; it remains the central design question for Phase 10.
- Where exactly is the boundary between "AI assists authoring" and "AI substitutes for reconciler"? The boundary is roughly: AI may *propose* edges/fields/text that humans approve into the deterministic record, but AI may not produce the deterministic record itself. Worth making explicit in spec.md as a positive-form design constraint.
- Is there a confidence-signal pattern that would be honest at the authoring layer without becoming a hard gate? Probably yes (e.g., "this draft was generated; review before approving") but it is more a UX honesty pattern than a calibration pattern.

---

## Cluster observation

The three rejections together teach a single positive principle: *cairn's enforcement value lives in the deterministic, typed, two-chain primitives at the kernel layer; flexibility is delivered above the kernel via templates, tags, project config, and queued AI assistance, never by dissolving the kernel's typed determinism*. C9 rejects schema flattening (which would remove typed determinism). C10 rejects closed-enum domain vocabulary (which would constrain typed determinism to one domain shape). C14 rejects AI-only reality (which would remove the deterministic reconcilable layer entirely). The three are the same principle at three layers: schema (C9), vocabulary (C10), reality (C14). The single positive form: *cairn is deterministic-typed at the bottom, configurable-templated in the middle, and AI-assisted at the top, and the layer ordering is non-negotiable*.

The highest-leverage salvage finding across the cluster is C10's *templated authoring scaffolds* sub-component. The candidate's case-for ("six concrete shapes give a new user somewhere to start") was real even though the closed enum was wrong; the salvage form (multiple default templates plus project-config registration) keeps the onboarding value while preserving cairn's domain plurality. The implementation path is short because spec §6 already supports `artefact_types` overrides; the missing piece is shipping more than one default template and surfacing template-pick as the first authoring step. Worth tracking as a candidate in its own right in the next batch.

A secondary observation: the three rejections each clarify a different architectural identity claim, and together they form a near-complete negative-space portrait of cairn's identity that complements the spec's positive-space description in §3 and CLAUDE.md's "What to avoid" list. Promoting the three quotable forms (one per rejection) into spec.md or CLAUDE.md as positive-form principles would strengthen both documents without adding much length. The rejections did the hard work of finding the load-bearing distinctions; surfacing them as principles makes the work portable to future contributors who never saw the getcairn.dev research.
