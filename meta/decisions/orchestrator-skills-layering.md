---
id: dec.orchestrator-skills-layering
nodes:
  - cairn.root
status: proposed
ratification: binding
date: 2026-08-04
related:
  - dec.orchestration-placement
  - dec.control-plane-programme
revisit_triggers:
  - "the mockup rounds or the first driver experience contradict a clause (the grill brief's ratification proviso; amend through a refining decision, never in place)"
  - "evidence that workflows should own more procedure than a reference (full subsumption arrives as a refining decision and stays single-source)"
  - "a harness appears whose delivery capabilities no descriptor row can express"
---

# Orchestrator skills layering: reference with mechanical provisioning

## Context

The 2026-08-04 orchestration grill (`studio/orchestration-grill-brief.md`,
Q6) asked where skills live once the driver exists. Q4 of the same
session ruled that workflow definitions are inert typed cairn artefacts
with a context slot naming the skills a dispatched session receives. The
maintainer answered Q6 in session; the ruling is recorded in
`todo.orchestrator-skills-layering` and this record enqueues it for
signature.

## Decision

1. **Reference is authoritative.** Skills live in the pack; workflow
   artefacts name them in their context slot and never carry procedure
   inline. Nothing is authored twice.
2. **Provisioning is operative.** At dispatch, the driver curates the
   session's skill surface to exactly the named set, so a unit sees only
   the skills its class needs.
3. **Delivery is capability-aware.** A harness that mounts skills
   natively gets them mounted; a harness that cannot gets the referenced
   skills rendered inline into the dispatch briefing. That is derived
   subsumption from the single source at dispatch time, never a second
   authored copy. The driver consults a per-harness capability
   descriptor declared with the workflow's route or in a harness
   registry.
4. **The pack teaches the contract.** Sanctioned verbs, the three
   terminal tokens, and the no-orchestration rule stay pack content for
   every agent; routing tables do not.
5. **Driver absorption is planned, not accidental.** As the driver
   absorbs supervision halves of the loop skills, those skills shrink to
   their in-session halves through their owning todos.

## The rubric

- **Tier**: binding. It rules the shipped pack surface
  (`tools/agent-pack/content/`) and the workflow schema's context slot;
  only the maintainer can sign it.
- **Unblocks**: the context and capability-descriptor fields of the
  workflow schema (`todo.driver-in-repo` task 2) and the skill-shrink
  migration plan (`todo.orchestrator-skills-layering` task 3).
- **Alignment**: against `dec.cairn-mission` first, this keeps procedure
  maintainable, investigable, and extendable by giving routing,
  procedure, and delivery one owner each. Goal 1: agents keep working
  because every session, manual or dispatched, loads the same
  single-source procedure. Goal 2: guardrails hold because per-unit
  context is declared in scan-validated artefacts rather than
  per-harness config. Goal 3: the maintainer signs one layering rule
  instead of arbitrating every skill-copy question. Goal 4: the ruling
  is recorded before any driver code exists. Goal 5: this record sits in
  the signature queue with its grill provenance.
- **Options considered**: (a) workflows subsume skills, which duplicates
  procedure into a second home and strands manual runs when no driver
  exists; (b) full separation where each harness picks its own skills,
  which moves per-unit-class context out of the graph and blinds the
  console; (c) reference with mechanical, capability-aware provisioning.
  (c) is the maintainer's answer. Rejecting it costs either drift
  between copies or routing that cannot say what it will load.

## Rationale

The maintainer's own framing in session: workflows are delegation
templates. The workflow mentions skills by name, and the dispatch
programmatically makes exactly those skills available to the session,
depending on the capabilities of the harness being delegated to. This
resolves the reference-against-subsume tension without picking a side:
the artefact never duplicates, only the delivery adapts. Procedural
parity is the load-bearing property: a manual session and a dispatched
session receive the same referenced skill content and the same
contract, which keeps the orchestrator optional and cairn without a
driver a complete product.

## Consequences

- The workflow schema carries skill references validated at scan time
  and a per-harness capability descriptor.
- Inline rendering into a briefing is derived at dispatch time from the
  pack source; a second authored copy of any skill is a defect.
- The loop skills the driver absorbs shrink to their in-session halves
  through owning todos filed when the driver lands.
- Manual and dispatched sessions receive the same referenced skill
  content and contract for the same unit. Delivery mechanics may differ
  by harness capability; behavioural identity is not claimed.
