---
node: cairn.root
status: open
created: 2026-08-09
parent: todo.driver-in-repo
---

# Driver In Repo Blueprint Node

## Scope
Define and land the blueprint declaration for the in-repo driver layer. Keep
the driver beside the passive core, with no core-to-driver dependency. Declare
the driver node or nodes, their paths, and any ownership or contract pointers
needed for later implementation. The driver consumes the query wire and the
sanctioned verbs. This unit does not implement driver behavior, workflow
evaluation, the selector wire, leases, the reaction loop, or UI wiring.

## Parent constraints
The parent todo is `todo.driver-in-repo`, under `## Task`, item 1:

> Blueprint node or nodes for the driver layer with no core-to-driver
> dependency; the driver consumes the query wire and the sanctioned
> verbs.

The parent placement boundary is also binding:

> The boundary is clauses 1 to 3 of that decision and it is not negotiable
> here: the core answers queries and applies sanctioned mutations and
> starts nothing. The driver observes, decides, and dispatches. A harness
> executes one assigned action and returns its outcome.

The parent `## Acceptance` first bullet requires the placement decision's layer
rules to be quotable against the shipped structure, including that the core has
no dependency into the driver and the substrate gains no orchestration
behavior.

## Dependencies
This is the first seam. The workflow-artefacts, selector-wire, and
reaction-loop sub-todos depend on this declaration for their owning paths and
layer boundary. The selector wire is the prerequisite of the reaction loop.
All four sub-todos are a LATER batch and remain open; none is implemented in
this decomposition.

## Sizing
S. The later implementation should change the blueprint and at most one
contract or path declaration, with no Rust implementation and no cross-subsystem
behavior.

## Non-goals
Do not add a core-to-driver edge, move orchestration into the core, or settle
the selector, workflow, lease, or reaction schemas here.
