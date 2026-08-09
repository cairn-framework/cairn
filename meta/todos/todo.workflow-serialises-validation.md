---
node: cairn.kernel.artefacts
status: blocked
created: 2026-08-07
related: [dec.rung-three-coordination-substrate]
blocked_by: [todo.driver-in-repo-workflow-artefacts]
---

# Workflow serialises validation

Carved out of `todo.hotspot-node-ownership` task 2 on 2026-08-07: no workflow
artefact type exists yet, so the `serialises:` list has nowhere to live until
`todo.driver-in-repo-workflow-artefacts` defines it under the driver's task 2.

## Task

Add `serialises:` to the workflow artefact as a list of path prefixes, not
node ids, per `dec.rung-three-coordination-substrate` clause 3 and
`res.parallel-dispatch-rung-3` Part 3, "The hotspot problem". Cairn validates
that each prefix exists and evaluates nothing further, per
`dec.orchestration-placement` clause 3.

## Acceptance

- A test asserts an unknown `serialises:` prefix is a validation finding and
  that a valid one is stored and never evaluated by the core.
- No per-unit authoring is introduced anywhere.

2026-08-07 audit (todo.roadmap-assumption-audit): status set blocked this session:
the workflow artefact dependency was not yet defined, and lint recorded the
unresolved dependency while the parent driver unit remained blocked.
