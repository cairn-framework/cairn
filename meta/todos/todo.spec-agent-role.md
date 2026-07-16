---
node: cairn.root
status: open
created: 2026-07-16
---

# Spec Agent Role

How docs/spec.md serves agents, from the deferral language audit in
`res.a2ui-analysis` (follow-on findings section), refined by owner
discussion (2026-07-16) into a target information architecture. Framing:
a large, partly irrelevant, partly contradictory document in agent
context degrades performance; the fix is composition, so no routine
agent path loads the spec at all.

## Target composition: four layers by load moment

1. Injected core (always-on): the AGENTS.md block plus agent guide that
   `cairn init` emits. What cairn is, first commands (context, next),
   the gates, and skill routing. Routes only; never workflow detail.
   Already exists (79 lines, verified clean of deferral language).
2. Skills (loaded per task): ALL guidance. Test: prose that says what to
   DO is a skill; prose that says what IS belongs in the graph. Spec
   passages that read as guidance (for example the change-flow walkthrough
   ending "the human runs cairn archive", spec:612-613) are category
   errors: the cairn-archive skill is the canonical home and the two can
   contradict.
3. The graph (queried, never bulk-loaded): all normative and
   forward-looking facts. Planned structure = ghost nodes (healthy state,
   no finding owed, `frontier` computes buildable-now vs blocked); plans
   = todos and changes (blueprint.delta ADDED nodes are the phase
   mechanism); open questions = `cairn gap` decisions (linted by
   CAIRN_GAP_UNRESOLVED); rules = spec-rules and error-codes registries
   (CK004 enforces rule-to-emitter pairing); per-node design = contracts;
   rationale = decisions. The spec maturity ladder maps to graph states:
   Declared = ghost plus todo/change, Designed = pending registry row or
   contract on a ghost, Implemented = synced plus enforcing finding code.
4. The essay (humans; agents only when the graph lacks an answer): what
   remains of docs/spec.md. Two-chain model, controller framing,
   positioning, changelog. Zero MUSTs, zero workflow, zero plans.

Invariant that removes the contradictions: if a routine agent task
requires loading spec.md, that is a composition bug (missing skill
reference, contract, or registry row), not a reason to keep the
read-first rule.

## Write-side rule (the ratchet, answers "does the spec ever retire")

Without a write-side rule the migration leaks: the spec gained content
through v0.8 and would keep gaining it. Adopt the convention (record in
docs/conventions.md) that content is added at the layer that consumes
it: new rules to the spec-rules registry, new planned structure to the
blueprint as ghost nodes, new plans to todos/changes, new questions to
gap decisions, new subsystem design to contracts, new rationale to
decisions. The spec accepts only narrative and clarity edits. Its
authority retires; the file remains as the essay and collapses section
by section to pointers as content migrates. Not proposed: freezing spec
version snapshots or splitting the file (refuted in res.a2ui-analysis).

## Steps

1. Actuator wording pass (no proposal needed). Later spec sections
   shorten section 2's correct framing ("a human or an agent, never
   cairn", spec:49) to "the human" for agent-capable or deterministic
   actions: spec:612-613, spec:777 (contradicting spec:771 four lines
   above), spec:63, spec:252, spec:817-824. Align with "the operator,
   human or agent", keeping "human" where genuinely meant (review
   subtype, the two-human example at spec:234, cairn ui as a browser
   surface). Also reword remediate.rs:224 "must be fixed manually" to
   "by editing the blueprint; no command repairs parse errors".
2. Rerouting (small, touches CLAUDE.md and emitted guide; maintainer
   ratification since it changes the documented read-first rule):
   replace "Read this first for any architecture question" with
   graph-first routing (context, get, rationale, bundle), spec as
   fallback narrative.
3. Write-side convention entry in docs/conventions.md, plus guidance in
   the cairn-dev skill.
4. Section-by-section migration as a ratchet, not a rewrite: each time a
   spec section's content lands in its graph primitive, collapse the
   section to a pointer. Candidate order: section 16 open questions to
   gap decisions; section 17 Declared list reconciled with
   docs/registries/declared-items.md and ghost nodes; section 14 future
   phases to changes/todos (past phases are already archive material);
   guidance passages folded into the owning skills. Grammar and artefact
   schema sections need a canonical-home decision (contracts on
   cairn.kernel.blueprint / cairn.kernel.artefacts with docs as rendered
   guides); that step needs a change proposal.
