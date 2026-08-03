---
node: cairn.root
status: blocked
created: 2026-08-03
related: [dec.orchestration-placement]
---

# Driver In Repo

Recorded under `dec.orchestration-placement` (proposed): the driver may
live in this repository as a distinct layer above the passive core,
fronted by the webui. Record only; nothing is built until the placement
decision is signed.

Blocked on the maintainer signing `dec.orchestration-placement`; that
gate is external, so no `blocked_by:` entry is declarable (ruling 4 of
`dec.todo-relationship-model`).

## Task

1. Blueprint node(s) for the driver layer with no core-to-driver
   dependency; the driver consumes the query wire and sanctioned verbs.
2. Declarative workflow artefacts the driver reads: a task routed to a
   harness with context guidance, and on-outcome routing to another
   harness or destination.
3. The console (`todo.console-signed-widening`) becomes the driver's
   steering surface per `dec.control-plane-programme`'s ownership split.

## Acceptance

- The placement decision's layer rules are quotable against the shipped
  structure; the substrate has no orchestration behaviour added.
