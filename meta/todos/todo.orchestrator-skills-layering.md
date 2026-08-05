---
node: cairn.root
status: blocked
created: 2026-08-04
related: [todo.driver-in-repo, dec.orchestration-placement]
---

# Orchestrator skills layering: reference with mechanical provisioning

Filed by the orchestration grill (`studio/orchestration-grill-brief.md`,
Q6, 2026-08-04), under the brief's ratification proviso. The maintainer
ruled the layering in session; `dec.orchestrator-skills-layering`
(proposed, binding, in `cairn pending`) owns the ruling itself.

Blocked on the maintainer signing that decision; the gate is external,
so no `blocked_by:` entry is declarable (ruling 4 of
`dec.todo-relationship-model`). Tasks 2 and 3 implement the ruling and
must not start on an unratified record.

## The ruling

Owned by `dec.orchestrator-skills-layering`; this todo does not restate
it, in any form; read the decision. Tasks 2 and 3 below implement the
ruling once it is signed.

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
