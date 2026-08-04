---
id: dec.orchestration-placement
nodes:
  - cairn.root
  - cairn.reconcile
  - cairn.sse
  - cairn.ui
status: proposed
ratification: binding
date: 2026-08-03
informed_by:
  - res.inversion-convergence-minutes
  - res.overharness-design-threads
  - res.decision-accumulation-cairn-root
  - res.messaging-workshop
  - res.cairn-identity
  - res.gas-city-cairn-integration
  - res.herdr-plugin-feasibility
  - res.cairn-domain-expandability
refines: [dec.no-orchestrator]
related:
  - dec.product-perimeter
  - dec.control-plane-programme
  - dec.north-star-continuous-loop
  - dec.webui-design-authority
  - dec.decision-ratification-tiers
  - dec.task-tracking-authority
  - dec.artefact-layout-authority
  - dec.spec-authority-retirement
  - dec.marketing-visual-world
  - dec.close-blueprint-drift
revisit_triggers:
  - "the driver needs an authoritative fact, derived view, or explicit mutation that passive query and sanctioned-command contracts cannot expose (revisit those contracts without giving the core transition or actuation authority)"
  - "required workflow policy cannot be represented as inert cairn artefacts evaluated entirely in the driver layer (revisit the artefact schema or the driver, never by granting the core observation, transition, scheduling, or side-effect authority)"
  - "a dependency appears from the core into the driver layer, which would make the core an orchestrator rather than a repository that hosts one"
  - "a third-party case study or testimonial corpus materialises and changes the proof strategy"
  - "the brownfield first-run gap closes through a one-step first map and permits stronger time-to-value claims"
  - "a competitor occupies the free bidirectional-reconciliation slot"
  - "a non-code reconciler is proposed for the distribution and extension band"
  - "a future decision revisits the public map framing itself"
---

# Orchestration placement: the core stays passive, the driver moves in beside it

## Decision

This record is about where orchestration logic sits, not whether the
repository contains it.

### 1. The core stays a passive substrate

The core answers explicit queries and applies explicit sanctioned
mutations. It never interprets a state change or a harness outcome as a
trigger, and it never repeats, schedules, dispatches, retries, or
supervises. Readiness and selectability are deterministic, side-effect
free projections of declared inputs and recorded facts; they are not
execution state and they start nothing.

### 2. The driver owns the loop, and it lives here

The driver is a distinct layer above the passive core, hosted in this
repository, with its own node or nodes in the blueprint and no
dependency from the core into the driver layer. It owns the outer loop:
it observes authoritative cairn state, receives execution outcomes from
harnesses, detects the transitions it cares about, applies declarative
routing policy, and alone triggers the next orchestration action.

This is the answer to the question the maintainer put on 2026-08-03. An
agent working inside a harness (OMP, Claude Code, or any successor) does
not orchestrate. It changes repository and graph state only through
sanctioned verbs and returns its outcome. The next orchestration action
happens because the driver observed that recorded change or that
returned outcome, not because the agent, the harness, or the core
started anything. A harness executes one assigned action and returns its
outcome; it never selects or dispatches its successor.

### 3. What each value is, and who may act on it

- Authored declarations are intent. Reconciled observations and
  explicitly recorded results are graph facts. Both are cairn truth.
- Readiness, selectability, and the frontier are projections. They rank
  and filter; they do not run.
- A harness outcome is a message returned to the driver. It becomes a
  graph fact only when an authorised caller records it through a
  sanctioned verb.
- Assignment, live lease state, and active, waiting, or blocked
  execution state belong to the driver. They are not cairn truth and the
  core does not infer them.
- Polling and subscription belong to the driver. A cairn change
  notification, where one exists, is an invalidation hint only: never
  canonical state, never a transition, never a dispatch command. The
  driver re-reads the authoritative query before acting. The existing
  `cairn watch` wire stays finding-change only unless a separately
  specified and versioned contract widens it.
- Declarative workflow artefacts are inert policy. They name graph or
  readiness predicates, outcome classes, and permitted next actions. The
  driver evaluates them. Cairn parses, validates, stores, and exposes
  them, and consumes none of them.

### 4. The webui fronts the driver and dispatches nothing

The console is a view and ruling surface, not an orchestrator. It shows
the source for, and visibly separates, current and actual graph facts,
declared and aspirational intent, derived readiness, and driver or
harness execution state including active, waiting, and blocked. It may
record maintainer rulings only through the verbs
`dec.control-plane-programme` clause 3 sanctions. It never assigns,
acquires or renews a lease, dispatches, retries, supervises, or
executes. Any orchestration action that follows a ruling happens later,
when the driver observes the recorded change and applies policy.

Orchestration-facing console implementation is gated, by this decision,
on `todo.console-orchestration-ux-design`: user journeys and evaluated
mockups that prove the state separation above, and a decision that
resolves `dec.webui-design-authority` clause 4's read-only rule for this
console, whose own revisit trigger has fired. This signature does not
authorise that implementation.

### 5. Surviving perimeter obligations

Clause 2 requires retiring `dec.product-perimeter` clause 4, so this
record supersedes that decision on acceptance. Everything it ruled
outside clause 4 survives here unchanged:

1. **Public identity: a falsifiable map, authored by the agent.** The
   coding agent is the primary author of blueprint, contracts, and
   decisions; the developer converses; cairn reconciles both directions
   and gates drift. Hand authoring stays supported and is never the
   pitch. The distinction is falsifiability. Public copy uses named
   guarantees (Clean Exit, Nothing-Leaves-Your-Machine) kept factually
   complete against what `cairn init` creates, with no scarcity or
   urgency devices and never leading with "free". Copy matches binary
   behaviour exactly: an overclaim is softened and filed as a todo, and
   a stronger claim ships only after the capability. Every public
   surface frames the agent as author; asking the developer to maintain
   artefacts by hand is a regression. `res.messaging-workshop` stays the
   living document for copy rounds.
2. **Internal identity: a reconciliation controller.** Publicly a map,
   internally a reconciliation controller. Prose about the mechanism
   stays consistent with setpoint, sensor, error signal, boundary, and
   actuator, and invents no competing metaphor.
   `dec.marketing-visual-world` still owns `docs/index.html`.
3. **Domain boundary: an agnostic kernel, pluggable reconcilers.** The
   blueprint grammar and the two-chain model stay domain-agnostic. The
   code reconciler is the only domain-specific component and the one v1
   reference implementation. `cairn.reconcile` stays a pluggable
   `Reconciler` trait producing a content-addressable fingerprint and
   claimed sub-elements. A new domain arrives by registering a
   deterministic reconciler, leaving kernel, grammar, and artefact model
   untouched; non-code reconcilers stay deferred to the distribution and
   extension band and non-code domains stay analysis-only until one
   lands.
4. **Retired scope stays retired.** The `adapters/` reference-adapter
   directory, the Gas City formula scheduler, and the governance pack
   are not revived. Artefact content stays file-backed and the only
   other work-item surface is the read-only beads view under
   `dec.task-tracking-authority`. The lapsed `ArtefactStore` sketch is
   not revived. The SSE cut stands: `src/sse.rs`, its `pub mod sse`
   export, `meta/contracts/sse.md`, and the `cairn.sse` Module node stay
   deleted, this decision names `cairn.sse` in its own `nodes:` so an
   accepted record still covers the CH001 module-remove gate, the semver
   obligation stays discharged at `315f2cf`, and the webui eval keeps
   `cairn.sse` as a commented fixture-only id in `harness/eval.mjs`
   until that scenario is rebuilt. Subscribing to an event stream is
   still not core behaviour; clause 3 above governs any notification
   wire.
5. **External consumers: read-only, and visibly so.** Any consumer
   outside this repository reads only public read-only surfaces. The six
   Herdr dashboard rules bind that consumer specifically and stand
   unchanged: a checked-in `scripts/` script on the python3 standard
   library calling only `cairn lint --json`, `cairn status --json`, and
   `cairn watch`; no blueprint node, Rust code, or release artefact; no
   caching, with collection time and a monotonic counter stamped on
   every rerun; deltas and timers deciding when to render, never what;
   `CAIRN_DASH_OVERLAY` data rendered only under
   `ORCHESTRATOR CLAIMS (overlay, unverified)`; and shape changes to
   those three surfaces breaking that dashboard but not cairn.

The in-repo driver is not an external consumer under clause 5 and is not
bound by its read-only rule. It is a first-party layer with its own
nodes, and its write path is the sanctioned verbs, never direct graph
mutation.

## Why this supersedes rather than refines

`dec.no-orchestrator` ruled that the core is a graph other orchestrators
traverse and that moving the driver in-repo requires a new binding
decision; `dec.product-perimeter` superseded it and carries that rule
forward as the live authority. This record refines that superseded
lineage for the exact clause it revisits, and supersedes the live
authority.

A refinement is not enough, and an earlier draft of this record that
tried one was wrong. Read exactly,
`dec.north-star-continuous-loop` places the consumer layer's
"assignments, leases, and scheduling outside the repository" (line 71),
then rules: "Building that layer beside cairn supersedes nothing. Moving
it inside cairn would require superseding `dec.product-perimeter`"
(line 78). Beside means outside this repository, so hosting the driver
here is the inside case. Perimeter clause 4.1 ("cairn does not own an
orchestrator"), 4.3 ("cairn maintains no workflow runner"), and 4.6
("workflow lives in external skills and packs") each speak at project
level rather than core level, and a first-party driver with its own
blueprint nodes contradicts all three. Arguing that a distinct layer
with no inbound core dependency is beside rather than inside does not
survive line 71.

That perimeter revisit trigger (a scheduling primitive inexpressible
outside cairn, or community demand for a zero-dependency orchestrator)
did not fire on its own terms. This supersession is maintainer
direction, recorded as such: the driver should share this repository's
gates, and the console should front it. Naming that honestly is better
than reading the trigger loosely.

What does not change: the core keeps every behavioural obligation clause
4 imposed on it. It owns no scheduler, evaluates no workflow, and starts
nothing. Clauses 1 and 3 above state that more precisely than clause 4
did. What changes is that the repository may host the layer that does
those things.

## What ratification must do

Accepting this is not a status flip alone. It narrows an accepted
authority, so ratification must resolve that contradiction in the graph,
following the precedent `dec.control-plane-programme` set:

- Mark `dec.product-perimeter` `status: superseded` and add
  `supersedes: [dec.product-perimeter]` here. Clause 5 carries every
  surviving obligation forward. The two edits land together in the
  acceptance commit, because `supersedes` only validates once the target
  is already marked.
- Until that commit, `dec.product-perimeter` stands and the driver stays
  outside the repository. `todo.driver-in-repo` stays blocked on this
  signature, and `meta/changes/driver-v2-selection` remains the live
  external-driver plan.
- Set `todo.driver-in-repo` to `open` on acceptance.

## The rubric

- **Tier**: `binding`. It supersedes an accepted perimeter authority,
  retires one of its clauses, and moves a repository boundary. Only the
  maintainer can sign it.
- **Unblocks**: `todo.driver-in-repo`. The journeys-and-mockups phase of
  the console scope (`todo.console-orchestration-ux-design`) and the
  read-only legibility fixes (`todo.console-state-legibility`) are
  deliberately outside this signature and can proceed without it.
  Orchestration-facing console implementation stays blocked on that
  design phase and on the webui read-only resolution named in clause 4.
- **Alignment**: against `dec.cairn-mission` first, it keeps the
  properties the mission guards by drawing the orchestration boundary
  explicitly in the graph instead of leaving it to session memory.
  Goal 1: agents keep working because the substrate they read stays
  passive and stable while the layer above it evolves. Goal 2:
  guardrails hold because the layer split, the value taxonomy, and the
  dispatch owner are recorded and queryable. Goal 3: the maintainer
  signs one placement boundary rather than re-arbitrating every driver
  change. Goal 4: the intent is recorded before any code exists.
  Goal 5: the signature queue carries this record with its briefing.
- **Options considered**: (a) keep the driver outside the repository,
  which splits one product across two repos and starves the driver of
  the graph's gates; (b) build orchestration into the core, which turns
  the substrate into a loop engine and breaks every consumer that wants
  a passive graph; (c) host the driver in-repo as a distinct layer above
  the passive core, fronted by the webui, with workflows as declarative
  artefacts. (c) is the recommendation. The cost of rejecting it is a
  permanently external driver that cannot share the repository's gates,
  or a core that stops being a substrate.

## Rationale

The maintainer ratified the inversion programme framing on 2026-07-31
(`res.inversion-convergence-minutes`, row R5: cairn steers rather than
passively maps), and the 2026-08-02 campaign shipped the read-only
console under `dec.control-plane-programme`'s three-owner split.
`res.overharness-design-threads` thread d parked the driver-in-monorepo
question for a future decision; this is that decision.

The ambiguity was never whether orchestration exists. It was which layer
owns it, which repository hosts that layer, and what an agent inside a
harness is allowed to be. Clause 2 answers all three, and clause 3 is
the part that makes the answer safe: the loop is driven by the driver
re-reading authoritative truth, so no notification, projection, or
harness message ever becomes an instruction on its own.

Clause 4 is deliberately a gate rather than a grant. The maintainer's
condition on this signature was that orchestration UX be designed
through journeys and mockups before it is built, because the current
console cannot distinguish what is actual from what is aspirational.
Recording that as a blocking prerequisite is what keeps the signature
from authorising a surface nobody has designed yet.

## Consequences

- On acceptance, `dec.product-perimeter` is `superseded`, keeping its
  provenance and its coverage of the nodes it names. Clauses 1, 2, 3,
  and 5 live on in clause 5 above; clause 4 is replaced by clauses 1 to
  3 above.
- `dec.control-plane-programme` would then refine a superseded decision,
  the same lineage shape this record has with `dec.no-orchestrator`. Its
  three-owner split is untouched and stays the live ownership authority.
- `dec.webui-design-authority` clause 4's read-only rule is not
  overridden here. Its revisit trigger has fired and its resolution is a
  prerequisite of console implementation, recorded in
  `todo.console-orchestration-ux-design`.
- Assets citing `dec.no-orchestrator` or `dec.product-perimeter` by name
  continue to resolve and are not rewritten. The core-passivity rule
  moves to clauses 1 to 3 above.
