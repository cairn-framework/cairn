---
id: genesis-phase-9-brownfield
nodes: [phase-9-brownfield]
date: 2026-05-01
sources: []
informed_by:
  - docs/strongholds/getcairn-cross-check-integrated.md
  - docs/strongholds/oq4-phase9-rescope-timing.md
  - openspec/changes/phase-7.6-ai-provenance-foundation/design.md
type: genesis
---

## Summary

- Wave 4 of the getcairn.dev adoption campaign rescopes `phase-9-brownfield` to absorb four AI-assisted authoring sub-components that earlier bundles deferred to "the brownfield phase": C8.c suggest engine, C1.b interview runner, C15 templated authoring, and the conditional C4.b decision-attached obligations follow-on.
- Trigger event: phase-7.6-ai-provenance-foundation (Bundle A) proposal-draft merged on PR #18; its design.md ratifies the identifiers this rescope cites (`suggested-edges.json`, error code `CC002`, capability area `provenance-foundation`, sidecar location `<archive-root>/<phase>/.cflx-trace.json`). Per the Option B refined verdict in `oq4-phase9-rescope-timing.md`, the rescope unlocks at design level once those identifiers are fixed; phase-7.6's apply registers `CC002` and ships the queue file class before this phase's apply.
- The rescope is docs-only: `proposal.md`, `tasks.md`, `design.md`, and `specs/brownfield/spec.md`. It does not move Phase 9's implementation start; it only makes the plan honest before code lands.
- Phase dependency added: `phase-7.6-ai-provenance-foundation` is now a required predecessor alongside the existing `phase-8-summariser` dependency.
- Path drift fixed in `tasks.md` 10.6: validate invocation path migrated from `.agents/skills/...` to `.claude/skills/...` to match the live tree.
- Spec deltas added under `specs/brownfield/spec.md`: three new ADDED Requirements (suggest engine, interview runner, templated authoring) plus one conditional ADDED Requirement for the obligations follow-on whose scenarios stay dormant until the decision schema gains the field.

## Transcript

Decision record synthesised from committed strongholds; no live elicitation Q/A occurred. This rescope was authored autonomously by a Nazgul force against the committed strongholds `docs/strongholds/getcairn-cross-check-integrated.md` (Pattern 3 absorption rationale and Integrator decision 5 path policy) and `docs/strongholds/oq4-phase9-rescope-timing.md` (Option B refined, design.md-ratification trigger). Those strongholds supplied the absorbed sub-component list (C8.c, C1.b, C15, C4.b follow-on) and the timing rationale. The transcript below records the key decisions made during authoring, in lieu of a turn-by-turn user dialogue.

**Q: Should the rescope wait for phase-7.6 to fully apply, or is design.md ratification sufficient?**

A: Per `oq4-phase9-rescope-timing.md` Option B refined, design.md ratification is sufficient. Phase-7.6 has design.md ratification (PR #18) so this question is moot for design-level integration; the design.md identifiers are stable. Phase-7.6's apply remains a downstream prerequisite that runs before this phase's apply.

**Q: Should the suggest engine be allowed to auto-accept high-confidence entries?**

A: No. Cross-check A's load-bearing argument and the phase-7.6 design both reject auto-accept on confidence; the gate's value is its non-bypassability. The rescope mirrors that policy: every emitted entry is `pending`, every entry routes through human triage, no exceptions.

**Q: Should the interview runner be a brownfield-only feature or a general `cflx-proposal` extension?**

A: Brownfield-onboarding-specific in this phase. Generalisation to other proposal types is left to a future phase. Scope discipline keeps the rescope tight.

**Q: Should templated authoring extend to non-contract artefact types (decisions, todos, research, reviews)?**

A: No. Templates apply to contract drafting only in this phase. Other artefact-type templates are deferred. This matches cross-check C and the integrated plan's "tag-extensible, never closed-enum" principle without over-promising surface area.

**Q: How does the C4.b obligations follow-on stay non-blocking when the decision schema may or may not grow the field?**

A: The follow-on is conditional. If the field exists, the brownfield generator populates and surfaces it for triage. If the field does not exist, section 8 of `tasks.md` records the no-op and section 8 tests stay `#[ignore]`. The spec scenarios for obligations carry "WHEN obligations field is present" guards so dormant scenarios do not regress validate-strict.

**Q: Does the rescope add a new capability area or stay inside `specs/brownfield/`?**

A: Stays inside `specs/brownfield/`. The absorbed sub-components are extensions of the brownfield workflow, not new capability surfaces. The phase-7.6 capability area `provenance-foundation` is referenced from this design.md but not duplicated.

**Q: Does the rescope require a `## MODIFIED Requirements` section, or are pure ADDED requirements sufficient?**

A: Pure ADDED is sufficient. The existing "Keep human review authoritative" requirement covers the suggest engine's gate semantics at the abstract level; the new "Suggest cross-cutting edges through the phase 7.6 queue" requirement adds the concrete queue-and-gate scenarios without overriding the existing requirement.
