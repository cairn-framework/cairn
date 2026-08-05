---
node: cairn.root
status: open
created: 2026-08-04
related: [todo.driver-in-repo, dec.orchestration-placement]
---

# Orchestrator skills layering: reference with mechanical provisioning

Filed by the orchestration grill (`studio/orchestration-grill-brief.md`,
Q6, 2026-08-04), under the brief's ratification proviso. The maintainer
ruled the layering; this unit records it as a proposed decision and
plans the migrations it implies.

## The ruling

- **Reference is authoritative.** Skills live in the pack; workflow
  artefacts name them in their context slot and never carry procedure
  inline. Nothing is authored twice.
- **Provisioning is operative.** At dispatch, the driver curates the
  session's skill surface to exactly the named set, so a unit sees only
  the skills its class needs (no UI skills on purely backend work).
- **Delivery is capability-aware.** A harness that mounts skills
  natively gets them mounted; a harness that cannot gets the referenced
  skills rendered inline into the dispatch briefing. That is derived
  subsumption from the single source at dispatch time, never a second
  authored copy. The driver consults a per-harness capability
  descriptor, declared with the workflow's route or in a harness
  registry.
- **Full subsumption stays open.** If driver experience shows workflows
  should own more procedure, that arrives as a refining decision and
  stays single-source.
- **The pack teaches the contract.** Sanctioned verbs, the three
  terminal tokens, and the no-orchestration rule stay pack content for
  every agent; routing tables do not.
- **Driver absorption is planned, not accidental.** As the driver
  absorbs supervision halves of the loop skills (recovery preflight
  classification, landing supervision), those skills shrink to their
  in-session halves through their owning todos.

## Task

1. Done in session (2026-08-04): `dec.orchestrator-skills-layering` is
   authored and enqueued in `cairn pending`; the signature itself stays
   the maintainer's.
2. Define the capability descriptor shape with `todo.driver-in-repo`
   task 2 (workflow artefacts) so route declarations can carry it.
3. Plan the skill-shrink migrations for the loop skills the driver
   absorbs, one owning todo per skill, filed when the driver lands.

## Acceptance

- A proposed decision in `cairn pending` carrying this ruling with the
  grill provenance.
- The workflow schema names skills by id and cairn validates the
  references at scan time.
- No skill content is duplicated into any workflow artefact.
