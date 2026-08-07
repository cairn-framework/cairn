---
node: cairn.kernel.artefacts
status: open
created: 2026-08-04
related: [todo.driver-in-repo, dec.orchestration-placement]
---

# Orchestrator skills layering: reference with mechanical provisioning

Filed by the orchestration grill (`studio/orchestration-grill-brief.md`,
Q6, 2026-08-04), under the brief's ratification proviso. The maintainer
ruled the layering in session; `dec.orchestrator-skills-layering`
(accepted 2026-08-06, binding) owns the ruling itself.

The signature gate is met, so tasks 2 and 3 are startable. Task 2 lands
with the workflow schema in `todo.driver-in-repo`, which is where the
capability descriptor is declared, so that unit paces this one.

One gap the ruling leaves to implementation, recorded here rather than
read back into the decision: clause 3 does not say what a driver does
with a harness it has no capability descriptor for. Rendering the
referenced skills inline is the safe default, because it degrades to
delivery every harness can accept and never silently drops a skill a unit
was promised. Task 2 settles it against the schema; if it needs ruling
rather than deciding, it arrives as a refining decision, never an in-place
amendment.

## The ruling

Owned by `dec.orchestrator-skills-layering`; this todo does not restate
it, in any form; read the decision. Tasks 2 and 3 below implement the
ruling, which was signed on 2026-08-06.

## Task

1. Done: `dec.orchestrator-skills-layering` was authored and enqueued in
   session (2026-08-04) and signed on 2026-08-06.
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
