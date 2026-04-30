# Cross-check: Bundle F (identity additions)

## Scope

Bundle F is a docs-only commit that promotes Batch B's three rejection-derived positive principles (C9, C10, C14) into spec.md as a new §3.5 and into CLAUDE.md as a new "What cairn is, positively" section, plus the cluster-observation principle "deterministic-typed at the bottom, configurable-templated in the middle, AI-assisted at the top, and the layer ordering is non-negotiable." Total proposed surface: ~350 words across two files. This cross-check tests whether the proposed scope collides with current spec.md, CLAUDE.md, or the active openspec content; whether the cluster-observation principle is genuinely new; and whether Batch B's quotable wording is publication-ready.

## Inputs read

- `/Users/george/repos/cairn/docs/spec.md` (sections 3.1–3.4 in full; section header inventory through §9; specifically confirmed §3.4 closes at line 92 and §4 begins at line 94, so §3.5 insertion has zero structural conflict).
- `/Users/george/repos/cairn/CLAUDE.md` (full file; one cross-reference to spec found, at line 9: `The framework's role (spec §3.4): "a fence around the authority chain and a navigator for the provenance chain."`, the sentence quoted is the closing line of §3.4 today, the §3.4 title is "Current-state authority").
- `/Users/george/repos/cairn/docs/strongholds/getcairn-refined-batch-B.md` (full; C9/C10/C14 quotable forms in §4 of each candidate plus cluster observation).
- `/Users/george/repos/cairn/docs/strongholds/getcairn-roadmap-debate.md` (Bundle F section §5 with proposed wording, plus §6 ordering placing Bundle F as the first commit out of the analysis).
- `/Users/george/repos/cairn/openspec/specs/foundation/spec.md` (validate-pass status: assumed passing, content scoped to Cargo skeleton + quality gates + fixtures; carries no identity statements that collide with §3.5).
- `/Users/george/repos/cairn/openspec/specs/terminology-rename/spec.md` (likely failing validate per the user's note about 11 failing items; content scoped to file-extension and snapshot-name renames + preservation of v0.6 taxonomy; one Requirement on "Preserved technical taxonomy" at line 52 is adjacent to identity but enumerates terms, not principles).
- `/Users/george/repos/cairn/openspec/specs/testing-baseline/spec.md` (likely failing validate; content scoped to snapshot-test discipline, file-size ceilings, test-first pre-phase convention; no identity statements).
- `/Users/george/repos/cairn/openspec/changes/` listing: active proposals are `phase-0-foundation`, `phase-8-summariser`, `phase-8.0-tests`, `phase-9-brownfield`, `phase-9.0-tests`, `phase-10-distribution`, `phase-10.0-tests`. None are about terminology, identity, or spec §3 prose.
- `/Users/george/repos/cairn/openspec/changes/archive/` listing: phase-2.6-terminology-rename is archived (per CLAUDE.md merge `3f15946`), so identity/terminology work is in maintenance mode, not active.

Validate-pass status note: spec.md and CLAUDE.md are not openspec-validated artefacts; they are repo-root prose. Bundle F therefore does not depend on the 11 failing validate items being green to land. The failing items are scoped under `openspec/specs/<area>/spec.md` and gate openspec validation, not git commits. Confirmed: Bundle F is structurally independent of the failing-validate fix backlog.

## Findings

### F1: §3.5 insertion has zero structural collision with current spec.md

§3 currently has four subsections: 3.1 (provenance chain), 3.2 (authority chain), 3.3 (the hinge), 3.4 (current-state authority). §3.4 ends at line 92 with the famous "a fence around the authority chain and a navigator for the provenance chain" sentence. §4 ("Related work") begins at line 94. A new §3.5 inserted between line 92 and line 94 takes pure additive space; nothing renumbers, no existing subsection moves, no cross-reference invalidates. The only cross-reference to a §3.x identifier in the entire repo is `CLAUDE.md` line 9 (`spec §3.4`), and that points at §3.4 by number, which is unchanged. The roadmap debate's secondary suggestion of inserting as `§3.4.1` to avoid numbering churn is now unnecessary. Adopt §3.5 as the placement.

### F2: Batch B's three quotable forms are publication-ready, with one wording tweak

Verbatim from Batch B §4 of each rejection:

- C9: *"Artefact types are obligation-bearing, not decorative. Each direct type's place in the provenance or authority chain determines what the kernel can enforce about it. Flattening the taxonomy collapses the obligations into labels, and labels are not enforceable."*
- C10: *"Cairn's authoring guidance is template-driven and tag-extensible, not enum-bound. The kernel ships a generic contract type; projects compose domain vocabulary on top via templates and tags. A closed enum would constrain cairn's domain scope at the kernel layer; templates and tags do not."*
- C14: *"Cairn extends to new domains by adding deterministic reconcilers, not by leaning on AI to normalise reality. The reality layer must produce a content-addressable fingerprint; without it, drift detection is impossible and the authority chain collapses to documentation. AI assistance lives at the authoring layer, never at the reality layer."*

All three are em-dash-free (the `, ` in C9's "labels, and labels are not enforceable" is a comma, not an em-dash; verified by inspection). All three use plain English at the level CLAUDE.md's voice section calls for. C10's "ships a generic contract type" is the only weak phrase: cairn ships contract, decision, todo, research, review, and source as direct types, not just contract. Tweak to "ships generic types" to match the actual taxonomy. Otherwise the three are quotable verbatim into spec.md and CLAUDE.md.

### F3: The cluster-observation principle is partly new, partly already implied

The Batch B cluster observation is *"cairn is deterministic-typed at the bottom, configurable-templated in the middle, and AI-assisted at the top, and the layer ordering is non-negotiable."* Cross-checking spec.md:

- "Deterministic-typed at the bottom" is implied by §3.2 (authority chain mechanical checkability) and §6 (kernel) but never stated as a layer-ordering principle. The "deterministic" word is used five times in §3.2 and §10; the "typed" word appears in `typed neighbourhood queries` in §5 goals. No single sentence joins them.
- "Configurable-templated in the middle" is novel as a phrase. §6 references `artefact_types` project-config overrides, and §8 carries per-type schemas, but there is no current statement that authoring guidance ships via templates rather than closed enums. This is a genuine addition.
- "AI-assisted at the top" is genuinely novel. spec.md as it stands is silent on AI's role. The "authoring vs reality" boundary that C14's principle establishes is not in the current spec at all.
- "The layer ordering is non-negotiable" is the operational claim. The spec implies ordering through the chains' direction (provenance flows in, authority flows out) but never names a flexibility ceiling.

Verdict: the cluster observation is a real addition, not a restatement. It belongs in spec.md §3.5 as the framing sentence, with C9/C10/C14 as the three layer expansions beneath it.

### F4: CLAUDE.md positive-form section nests cleanly above "What to avoid"

CLAUDE.md currently has these sections in order: What CAIRN is (line 5), Where things live (11), Architecture: two chains (28), Terminology state (38), Voice and audience (62), UI and visual work (68), Workflow: cflx (72), What to avoid (78), Further reading (87), graphify (94). The `What to avoid` section at line 78 is bulleted and negative-form. A new `What cairn is, positively` section sits naturally either immediately above it (preferred: positive precedes negative) or immediately below the `Architecture: two chains` section as an extension of the architecture framing. The roadmap debate's suggestion of "above or alongside What to avoid" is correct.

Recommendation: place it between `Workflow: cflx` (line 72–76) and `What to avoid` (line 78). This positions it as architectural identity sitting just before the operational don't-do list, which mirrors §3.5 sitting just before §4 in spec.md. The two surfaces become structurally parallel: positive principles, then exclusions.

### F5: No active openspec change overlaps Bundle F's content

Active proposals: phase-0-foundation, phase-8-summariser, phase-8.0-tests, phase-9-brownfield, phase-9.0-tests, phase-10-distribution, phase-10.0-tests. None touch spec.md §3 prose or CLAUDE.md identity sections. The terminology-rename consolidated spec at `openspec/specs/terminology-rename/spec.md` carries one Requirement on "Preserved technical taxonomy" (line 52) that enumerates which v0.6 terms are kept, but it does not state positive identity principles. Bundle F adds *principles* alongside that *enumeration* without overlap.

The closest adjacent content is CLAUDE.md's existing paragraph block at lines 50–58 ("Everything else in v0.6 is kept deliberately. Do NOT propose flattening the taxonomy..."), which is the negative-form of C9's principle ("don't flatten") expressed as a CLAUDE.md don't-do directive. Bundle F's C9 positive principle ("typed artefacts encode obligations, not labels") is the same principle stated affirmatively. The two are complementary, not duplicative; CLAUDE.md says "don't flatten the taxonomy" in operational voice, the new section will say "types encode obligations" in identity voice.

### F6: Bundle F is structurally independent of the 11 failing validate items

The failing-validate items live in `openspec/specs/<area>/spec.md` files governed by `cflx.py validate`. Spec.md (the canonical spec at `docs/spec.md`) and CLAUDE.md are not openspec-validated; they are repo-root prose with no `cflx.py validate` gate. Bundle F's commit will pass the standard pre-commit hook (`cargo fmt --check`) trivially because it touches no Rust. It will not interact with the validate gate at all. Confirmed: Bundle F neither blocks nor is blocked by the failing validate backlog.

### F7: The C10 principle has a wording precision problem worth fixing

C10's quotable form says cairn "ships a generic contract type" but cairn actually ships six direct artefact types (contract, decision, todo, research, review, source). The C10 rejection is specifically about the contract type's authoring vocabulary not being a closed enum, but the principle generalises to all artefact types: none of them are closed-enum-shaped. Tighten the wording to "the kernel ships generic types; projects compose domain vocabulary on top via templates and tags." This is a one-word edit (drop "a" and "contract") and it makes the principle correctly scoped.

### F8: The "AI-at-authoring vs AI-at-reality" boundary deserves explicit naming

Batch B's open research question Q43 (in roadmap-debate §8) flags: *"Where exactly is the boundary between AI assists authoring and AI substitutes for reconciler? Worth making explicit in spec.md as a positive-form design constraint."* The C14 principle as quoted answers this implicitly ("AI assistance lives at the authoring layer, never at the reality layer") but the boundary mechanic (AI may *propose*, humans *approve into the deterministic record*) is the load-bearing detail. Bundle F should add one operational sentence that names the propose-then-approve mechanic, not just the layer assignment. This addresses Q43 within Bundle F rather than deferring it to a separate session.

## Recommendations

### R1: Adopt §3.5 placement (not §3.4.1) and append the cluster-observation framing sentence

Insert §3.5 between current line 92 and line 94 of spec.md. Use the cluster-observation as the section's framing sentence, and present C9/C10/C14 as three bulleted layer expansions beneath it. This produces a four-sentence section that reads as a single coherent thought rather than three loosely-joined principles. Concretely: ~210 words, one new H3 header, three bulleted sub-principles, no churn to existing numbering.

### R2: Tighten C10's wording before promoting it

Change "ships a generic contract type" to "ships generic types" so the principle correctly scopes across all six direct artefact types, not just contract. This is the only wording change recommended; C9 and C14 ship verbatim.

### R3: Add one sentence to C14's principle naming the propose-vs-approve mechanic

Append: *"AI may propose edges, draft contracts, and suggest narrative summaries, all reviewable through the change-isolation primitive. AI may not produce the deterministic record itself."* This addresses Q43 within Bundle F. Roughly 30 added words; it makes C14 answer "where is the boundary" not just "what is the layer."

### R4: Place CLAUDE.md positive-form section between Workflow:cflx and What to avoid

Specifically between current line 76 (end of `Workflow: cflx (Conflux)` section) and line 78 (start of `What to avoid`). This makes the positive section the immediate sibling of the negative section, and it positions both above `Further reading` so a new contributor reads identity then exclusions before navigation links. The CLAUDE.md cross-reference at line 9 stays untouched.

### R5: Cross-link the two surfaces explicitly

The CLAUDE.md positive-form section's last line should read: *"These three principles are stated in spec.md §3.5 with rationale."* The spec.md §3.5 needs no reciprocal link (specs typically don't reference CLAUDE.md). One-direction link only.

### R6: Ship Bundle F as a single commit with both file edits, before any other work from this debate

Roadmap debate §6 already places Bundle F as commit #1 of the entire adoption plan. Confirm: ship Bundle F before phase-7.5c, before Bundle A scaffold, before Bundle B scaffold. Reason from finding F4 of the debate: every subsequent decision benefits from the principles being explicit. Single commit, two files, ~400 words after R3's expansion. No tests, no code, no validate interaction.

## Decisions made (with reasoning)

### D1: Bundle F survives with expanded scope

- **Decision**: Bundle F ships substantively as proposed. Scope expands from ~350 to ~400 words to accommodate R3 (the propose-vs-approve sentence on C14). Still one commit, still two files. The cluster-observation framing sentence is included as proposed.
- **Reasoning**: Findings F1, F4, F5, F6 confirm zero collisions and zero blockers. F3 confirms the cluster-observation is genuinely new content, not a restatement. F8 surfaces a real gap (Q43) that Bundle F is the natural place to close. Splitting Q43 into a separate later commit costs another commit for one sentence and risks the propose-vs-approve mechanic being relitigated against the layer-assignment claim.
- **Confidence**: high
- **What would flip this**: discovering an active openspec change that already adds identity prose to spec.md §3 (none found); or discovering CLAUDE.md has a hidden style guide forbidding positive-form sections (none found, the file already mixes positive-form `What CAIRN is` and negative-form `What to avoid`).

### D2: §3.5 placement, not §3.4.1

- **Decision**: Insert as §3.5 (new top-level subsection of §3), between current §3.4 and §4. Do not insert as §3.4.1.
- **Reasoning**: F1 confirms zero downstream cross-reference breakage. §3.4.1 sub-numbering would suggest the new content is a refinement of "current-state authority" specifically, but the layer-ordering principle is parallel to all four existing subsections, not nested under one of them. §3.5 expresses the parallelism correctly.
- **Confidence**: high
- **What would flip this**: discovering an external citation (paper, blog post, internal doc) referencing §3.5 already in some other capacity. None found in the repo.

### D3: Cluster-observation principle is included as the §3.5 framing sentence, not as a fourth bullet

- **Decision**: Use *"Cairn is deterministic-typed at the bottom, configurable-templated in the middle, AI-assisted at the top, and the layer ordering is non-negotiable"* as the section's lead-in sentence, with C9/C10/C14 as three bulleted layer expansions.
- **Reasoning**: F3 establishes that the cluster observation is the most general statement; C9/C10/C14 are its layer-specific manifestations. Listing the cluster observation as a fourth peer bullet flattens the hierarchy. Listing it as the framing sentence preserves the relationship.
- **Confidence**: high
- **What would flip this**: editorial judgement that bulleted lists read better than framing-sentence-plus-bullets in this specific spec context. The current spec uses framing-sentence-plus-bullets in §5 (Goals/Non-goals) and §10 (Findings), so the precedent supports the framed form.

### D4: Tighten C10 wording; ship C9 and C14 verbatim

- **Decision**: Change "ships a generic contract type" to "ships generic types" in C10. Leave C9 and C14 unchanged from Batch B's quotable forms (with R3's appended sentence on C14).
- **Reasoning**: F7 identifies the C10 wording as factually narrow; the fix is one word and lossless. C9 and C14 are factually correct and stylistically clean as written.
- **Confidence**: high
- **What would flip this**: a stricter editorial pass that found prose issues in C9 or C14. None found in F2.

### D5: CLAUDE.md positive-form section sits between Workflow:cflx and What to avoid

- **Decision**: Insert at line 77-ish (between `Workflow: cflx (Conflux)` end and `What to avoid` start). Three numbered principles. Closes with a cross-reference to spec.md §3.5.
- **Reasoning**: F4 establishes structural parallelism with spec.md (positive section immediately precedes negative section in both surfaces). Above `What to avoid` and below `Workflow: cflx` is the only placement that achieves this without disrupting CLAUDE.md's existing flow.
- **Confidence**: high
- **What would flip this**: a request to keep CLAUDE.md tightly operational and push positive-form principles into a separate file (e.g., `docs/identity.md`). The roadmap debate did not propose this and CLAUDE.md already carries identity content (`What CAIRN is`, `Architecture: two chains`).

### D6: Ship Bundle F as commit #1 of the adoption plan

- **Decision**: Bundle F lands first, before phase-7.5c, Bundle A, Bundle B. Single commit. Two file edits.
- **Reasoning**: Roadmap debate §6 already prescribes this ordering. Bundle F has zero technical dependencies. Its principles inform every subsequent phase's scoping decisions. Cost of delay (subsequent phases scoped without explicit identity reference) exceeds cost of shipping early.
- **Confidence**: high
- **What would flip this**: discovering that phase-7.5c verification-states work needs identity wording that Bundle F doesn't yet articulate. Reviewed; phase-7.5c is purely workflow/state-enum work and doesn't depend on Bundle F.

## Open questions for next session

- **OQ1**: Should the §3.5 section title be *"Layer ordering of enforcement, configuration, and AI"* (roadmap-debate's suggestion) or something tighter like *"Flexibility ceiling"* or *"Three layers of cairn's identity"*? Recommend the roadmap-debate title for now; revisit if the spec's overall §3 titles get a tightening pass.
- **OQ2**: Does the C10 principle's "tags" reference need a forward-reference to where tags are documented? spec.md mentions tags only briefly; the `@`-tag system is more thoroughly described in CLAUDE.md and in archived phase-2.5. If a reader hits the C10 principle and wonders "what's a tag?" the trail is thin. Recommend leaving as is for now and accepting the minor fragility; tags are mentioned across multiple sections and a future tags-consolidation phase would pick up the slack.
- **OQ3**: Should the cluster-observation principle be mirrored in `AGENTS.md`? AGENTS.md is the codex agent's reading; if codex is making architectural decisions during a phase apply, having the layer-ordering principle visible there might prevent regressions. Out of Bundle F's scope but worth surfacing as a follow-on micro-commit. Recommend deferring to a small AGENTS.md sync commit after Bundle F lands.
- **OQ4**: Is the propose-vs-approve sentence (R3) better as a §3.5 line or as a separate line in §6 ("The kernel") where the change-isolation primitive is described? Current recommendation: §3.5 because the propose-vs-approve mechanic is the *load-bearing detail of the AI-assisted layer*. If a future spec restructure wants to push operational mechanics down to §6, that's a separate move.

## Recommended Bundle F final scope

**Spec.md addition (new §3.5, ~210 words):**

Insert between current line 92 and line 94. Header: `### 3.5 Layer ordering of enforcement, configuration, and AI`. Body (no em-dashes, cairn voice):

> Cairn is deterministic-typed at the bottom, configurable-templated in the middle, AI-assisted at the top, and the layer ordering is non-negotiable. Enforcement value lives in the kernel's deterministic, typed, two-chain primitives. Flexibility is delivered above the kernel via templates, tags, project config, and queued AI assistance.
>
> - **Bottom: deterministic-typed.** Artefact types are obligation-bearing, not decorative. Each direct type's place in the provenance or authority chain determines what the kernel can enforce about it. Flattening the taxonomy collapses the obligations into labels, and labels are not enforceable.
> - **Middle: configurable-templated.** Cairn's authoring guidance is template-driven and tag-extensible, not enum-bound. The kernel ships generic types; projects compose domain vocabulary on top via templates (per `artefact_types` in §6) and tags. A closed enum would constrain cairn's domain scope at the kernel layer; templates and tags do not.
> - **Top: AI-assisted at authoring only.** Cairn extends to new domains by adding deterministic reconcilers, not by leaning on AI to normalise reality. The reality layer must produce a content-addressable fingerprint; without it, drift detection is impossible and the authority chain collapses to documentation. AI may propose edges, draft contracts, and suggest narrative summaries, all reviewable through the change-isolation primitive. AI may not produce the deterministic record itself.

**CLAUDE.md addition (new section, ~155 words):**

Insert between current line 76 (end of `Workflow: cflx (Conflux)`) and line 78 (start of `What to avoid`). Header: `## What cairn is, positively`. Body:

> Three principles, complementary to the negative-space "What to avoid" list below:
>
> 1. **Typed artefacts encode obligations, not labels.** Each direct type (`contract`, `decision`, `todo`, `research`, `review`, `source`) has a different role in the two-chain topology. The kernel's enforcement value comes from those role differences. Treating types as decorative labels (or proposing a flat schema) is the same mistake as flattening the two chains into a six-layer stack.
> 2. **Authoring guidance is template-driven and tag-extensible, never closed-enum.** Domain-specific vocabulary belongs in project config (`artefact_types`) or in tag conventions, both of which are extensible. The kernel speaks taxonomy; the project speaks domain.
> 3. **AI assists authoring; AI does not substitute for the reconciler.** AI may propose edges, draft contracts, suggest narrative summaries, all reviewable through the change-isolation primitive. AI may not produce the deterministic reality fingerprint that drift detection compares against. The enforcement layer stays mechanically checkable.
>
> These three are the positive form of the rejections in "What to avoid." Stated in spec.md §3.5 with rationale.

**Total scope:** ~365 words across two files. Single commit. Single-commit boundary preserved (the roadmap debate's "one commit, two file edits" framing holds; the +50 word expansion from R3 doesn't change the single-commit shape). No code, no tests, no schema bumps, no validate-gate interaction. Ships before any other work from this adoption plan.
