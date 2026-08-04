---
node: cairn.root
status: blocked
created: 2026-08-03
related: [dec.orchestration-placement, dec.control-plane-programme, todo.console-orchestration-ux-design, todo.console-signed-widening, todo.parallel-dispatch-granularity, todo.review-gate-machine-check]
---

# Driver In Repo

`dec.orchestration-placement` (proposed, binding) would let the driver
live in this repository as a distinct layer beside the passive core,
fronted by the webui. This unit builds that layer once the record is
signed.

Blocked on the maintainer signing `dec.orchestration-placement`; that
gate is external, so no `blocked_by:` entry is declarable (ruling 4 of
`dec.todo-relationship-model`). The signature is not a formality here: it
supersedes `dec.product-perimeter`, whose clause 4 keeps the driver
outside the repository until then.

The boundary is clauses 1 to 3 of that decision and it is not negotiable
here: the core answers queries and applies sanctioned mutations and
starts nothing. The driver observes, decides, and dispatches. A harness
executes one assigned action and returns its outcome.

## Task

1. Blueprint node or nodes for the driver layer with no core-to-driver
   dependency; the driver consumes the query wire and the sanctioned
   verbs.
2. Declarative workflow artefacts the driver reads: a task routed to a
   harness with context guidance, and on-outcome routing to another
   harness or destination. These are inert policy under clause 3. Cairn
   parses, validates, stores, and exposes them and evaluates none of
   them.
3. The reaction loop, stated explicitly rather than implied. The harness
   returns a structured outcome keyed by unit and commit; the driver
   verifies that refreshed `main` carries the landed work and reconciles
   it; the driver records lease and outcome state through a declared
   driver-owned surface or a sanctioned verb; the driver re-queries the
   canonical selector; the driver dispatches the next unit. Nothing in
   that sequence is a core behaviour.
4. The selector wire the loop needs. `cairn next` today exposes no
   stable unit id and no reproducible selection evidence, groups findings
   into remediation actions, and orders todos by creation date and path
   rather than the loop's own precedence. Establish what the driver needs
   (commit and schema version, unit id, node, selection ground,
   reproducible evidence) and land it as a passive query, or file the
   exact missing field against the owning node. This is the prerequisite
   for proving that a dispatched mission equals what a loop session's
   Orient step would select at the same commit.
5. Steering surface: `todo.console-signed-widening` wires the console to
   this driver under `dec.control-plane-programme`'s ownership split. It
   is blocked on `todo.console-orchestration-ux-design` and does not gate
   this unit's non-UI work.

## Acceptance

- The placement decision's layer rules are quotable against the shipped
  structure: the driver has its own node or nodes, the core has no
  dependency into it, and the substrate gained no orchestration
  behaviour.
- A dry run prints the mission it would dispatch, with unit id, selection
  ground, and evidence lines, and that mission equals what a loop
  session's Orient step selects at the same commit.
- The outcome-to-next-dispatch sequence in task 3 is exercised end to
  end, including a harness outcome that must not advance the loop.
- Every fail-closed condition the external v1 driver enforced still stops
  this one: a dirty park, HEAD off `origin/main`, a surviving loop branch
  or open loop PR, a nonzero session exit, a final line that is not
  exactly a terminal token, `LOOP EXHAUSTED` without the unit's todo
  `done` on main, and a completion where the todo went `blocked`.

## Lease shape

Do not freeze the lease and claim schema before
`todo.parallel-dispatch-granularity` rules on what a claim is held on,
its identity, expiry, and renewal, and what execution granularity means
when one unit touches several nodes. That unit owns the ruling;
`todo.console-orchestration-ux-design` contributes the mockup evidence
it is made against. `lease` currently appears in no Rust source, so the
shape is genuinely open.

## Relationship to the driver v2 change

`meta/changes/driver-v2-selection` remains as authored: it audits the
read surface and hardens the external v1 supervisor. Its read-surface
audit is the direct input to task 4 here. This unit is the in-repo
successor that change deliberately did not become.
