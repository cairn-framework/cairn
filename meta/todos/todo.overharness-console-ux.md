---
node: cairn.ui
status: blocked
created: 2026-07-31
blocked_by: [todo.roadmap-derived-view, todo.pending-queue-briefing]
related: [res.inversion-convergence-minutes, res.overharness-design-threads]
---

# Over-harness console: the human steering surface

`res.inversion-convergence-minutes` row R5. The inversion programme
(graph is the programme; drivers and harnesses are stateless executors)
needs the human-facing half: a webui surface where the maintainer sees
what the repository truth selects, in one place: the signature queue
(`cairn pending`), buildable structure (`cairn frontier`), and the work
DAG (the roadmap projection). This is the surface the first external
user test said was missing (src.reddit-gregerw-first-user-test: "how to
make humans aware of these and help steer the agents").

## Task

1. Author the programme decision this unit needs before implementation.
   It must explicitly separate three ownerships rather than blur them:
   policy and control (cairn: selection truth, dependencies, readiness,
   leases, dispatch policy), dispatch (the driver: when work starts,
   retries, supervision), and execution (the harness: how work runs).
   If cairn owns the first set it is the declarative control plane by
   construction; the decision says so plainly and declares `refines:`
   (or, at the maintainer's signing call, `supersedes:`) against the
   current authorities `dec.product-perimeter` and
   `dec.north-star-continuous-loop` (`dec.no-orchestrator` is their
   ancestry). No containment by rewording. Unratified candidate framing
   from the slate's post-ratification intake
   (res.inversion-convergence-minutes): "build/delivery/runtime FACTS
   sit inside cairn's investigation boundary; scheduling, execution,
   supervision stay outside its actuation boundary". The decision
   author weighs it; the maintainer's signature makes any wording
   binding. Binding tier; enqueue via `cairn pending`.
2. Compose, do not re-own: `todo.pending-queue-briefing` owns the
   pending queue's presentation (chat-first briefing vocabulary), and
   `todo.roadmap-derived-view` owns the backlog projection. This todo
   owns only the one-screen composition and integration per the design
   system: pending, frontier tiers, and the roadmap DAG in one
   workspace, one selection vocabulary with the CLI (WorkItem).
3. Interim constraint, not a settled boundary: until the Task 1 decision
   is signed, the console stays read-only (renders what the driver would
   read, dispatches nothing), matching the current
   `meta/changes/driver-v2-selection` scope. Final dispatch ownership is
   exactly what that decision assigns; if it rules the control plane may
   dispatch, this todo's scope widens under that authority rather than
   the old boundary silently winning. The same conditionality covers
   upstream read-only clauses (for example the pending channel's
   read-only rule per dec.user-surfaces): presentation ownership stays
   with the upstream todos, write and dispatch behaviour follows the
   signed decision.
4. Intake lane: maintainer thoughts land as feedback or draft artefacts
   for triage (the `cairn feedback` / `.cairn/feedback.md` seam and
   draft decisions in the pending queue), so steering the graph never
   requires interrupting a running session. The console surfaces the
   intake queue beside pending; triage promotes entries through the
   sanctioned write verbs.

## Acceptance

- The programme decision is signed and its ownership split is quotable.
- The console renders pending, frontier, and roadmap from live query
  data on this repository; visual harness gates pass; no token
  hardcoding.
- A maintainer can answer "what is waiting on me, what is buildable,
  what runs next and why" from one screen, each answer traceable to the
  CLI command that produces it.
