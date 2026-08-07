---
id: dec.webui-write-authority
nodes:
  - cairn.ui
  - cairn.root
status: accepted
ratification: binding
date: 2026-08-06
informed_by:
  - res.inversion-convergence-minutes
refines: [dec.control-plane-programme, dec.orchestration-placement]
supersedes:
  - dec.webui-design-authority
related:
  - dec.cli-agent-workflow-consolidation
revisit_triggers:
  - "the maintainer sanctions both a Node/Playwright toolchain and a vision provider, which reverses the vision-loop foreclosure and requires superseding this decision"
  - "the maintainer sanctions a paid or network vision provider on its own, which could replace the manual inspect_image review step"
  - "a design-quality proxy is shown to reward a change a visual review judges worse, requiring the proxy or its saturation to be retuned"
  - "the adopted state-vocabulary motifs fail the design-quality scorer or the visual harness"
  - "the UX-first result fails on real repos with graphs much larger than the dogfood 25 nodes"
  - "design-studio tooling becomes unavailable or its output quality is insufficient to compare tracks"
  - "a maintainer moment appears that cannot be expressed as a typed recorded ruling and would need a live console-to-driver channel"
  - "prototype feedback shows stale-plan declines making run rulings routinely fail to dispatch, which questions the commit-pinned plan identity"
---

# Webui write authority: rulings are recorded facts, and run is one of them

Accepted 2026-08-06. All four ratification actions landed in that commit:
the supersession marked, the brief's non-goal line and J6 table amended,
`cairn todo new` confirmed into the clause 1 union, and every live pointer
moved. Clause 6 and the ratification section below are written from before
the signature and describe the state they were authored in, not a live
boundary.

## Context

`dec.webui-design-authority` clause 4 rules that the webui stays read-only,
and its own revisit trigger names the condition that retires that rule: the
two-way exploration is taken up, or the webui's scope otherwise grows beyond
read-only exploration. That trigger has fired twice over.
`dec.control-plane-programme` clause 3 granted the control-plane console a
narrow sanctioned write path (2026-08-03), and `dec.orchestration-placement`
clause 4 made the console the driver's steering surface and gated
orchestration console implementation on, among other things, "a decision
that resolves `dec.webui-design-authority` clause 4's read-only rule for
this console". This is that decision.

Two inputs fix its scope. First, the J6 write-path table in
`studio/orchestration-console-brief.md`: of the six typed waiting-on-you
queue entries, four (retry approval, quarantine release, park and unpark,
budget approval) have no write path today and render display-only. The Q5
grill ruling (2026-08-04, recorded in
`todo.console-orchestration-ux-design`) already settled their shape:
recorded facts the driver obeys, whose sanctioned verbs "must be specified
by the write-authority decision". Second, the brief's 2026-08-06 amendment:
the guided creation journey ends in one run action, which collides with the
round-1 non-goal (no dispatch from any console affordance). The recorded
candidate reconciliation, run is a recorded ruling the driver obeys, is this
decision's to accept or reject.

The method follows the precedent in `dec.control-plane-programme` lines 41
to 46: supersede the prior record whole, carry every surviving obligation
forward in the successor, and never edit the accepted record in place.

## Decision

### 1. The read-only rule is retired, narrowly

On acceptance this record supersedes `dec.webui-design-authority`. What
changes is one sentence of its clause 4: the webui is no longer read-only
without exception. What replaces it is a closed union of two sanctioned
sets, and the console may write through nothing outside it. First, the
grant `dec.control-plane-programme` clause 3 already made, which this
record refines rather than supersedes and therefore leaves in force:
`cairn todo set` and `cairn feedback` today, plus the paired `cairn todo
link` and `cairn todo unlink` relationship verbs when
`todo.todo-relationship-schema-implementation` lands, with
`dec.cli-agent-workflow-consolidation` and `dec.todo-relationship-model`
keeping their live obligations over them. Second, `cairn todo new`, which
ratification confirms into the union as the feedback-promotion verb
(clause 3). Third, the `ruling` family this record adds in clauses 3 and 4
below. No free-form command surface
exists anywhere in the console. The general graph explorer and the MCP
surface keep their read-only obligations and receive no part of either
set (`dec.control-plane-programme` clauses 3 and 6, unchanged).

### 2. A ruling is a recorded fact, never an action

Every ruling is an append-only graph fact carrying: ruling type, target
(todo id, or plan identity for run), the acting maintainer, recorded_at,
the commit at recording, and the type's payload. Ruling facts are cairn
truth: the core stores and serves them, evaluates nothing, and starts
nothing on any ruling transition (`dec.orchestration-placement` clauses 1
and 3, unchanged). The driver observes recorded rulings on its next read
and alone acts on them. A ruling therefore survives driver restarts and
console sessions, and renders in the history channel. Storage lives in the
shared coordination store that rung 3 designs
(`todo.parallel-dispatch-granularity` owns that design); this clause fixes
the contract, not the format.

### 3. The queue verbs

One new CLI noun, `ruling`, in the noun-then-subcommand grammar, added to
the clause 1 union rather than replacing any part of it. The ruling
family is exactly the verbs below, invoked on one write path shared with
the maintainer's own terminal, never a parallel console-only channel.

- `cairn ruling retry <todo-id>`: approves one further attempt on a failed
  unit; payload names the attempt number approved. Absent an approval the
  driver never retries past its policy cap.
- `cairn ruling release <todo-id>`: releases a quarantined unit (a claim
  that failed verification) back to the ready set. Quarantine is never
  auto-released.
- `cairn ruling park <todo-id>` and `cairn ruling unpark <todo-id>`:
  excludes a unit from driver selection, or returns it. Parking is a
  deferral fact, not a status edit: the todo keeps its status, and the
  ready-set projection honours the fact.
- `cairn ruling budget <todo-id|wave-id> --cap <value>`: records a new
  spend cap for the named unit or wave (the Q9 budget-exhausted moment).

Signature entries stay display-only routing to the maintainer's own
signing flow. Feedback triage stays on the `cairn feedback` intake, and
acceptance settles its promotion step by confirming `cairn todo new` into
the clause 1 union. The other reading the question offered, routing
promotion through an already-granted verb, is not available: `cairn todo
set` changes an existing todo's status and creates nothing, so a triage
entry routed through it would be decoration. Creating a todo fits the
shape this record is built on anyway. It is a recorded fact, it starts
nothing, it acts on no driver, and feeding native todos is what the
feedback lane exists for (`dec.native-todos-first`). The signature is the
authority for that one widening, which is exactly why the question was
left to ratification instead of taken on this record's own. That completes the J6 table:
six typed entries, six named write paths, none of them a console action on
the driver.

### 4. Run is ruled in, as a recorded ruling the driver obeys

`cairn ruling run <plan>` records the maintainer's consent to one composed
dispatch plan: the wave the preview rendered, identified by its unit set,
their derived write-sets, and the commit the composition was derived at.
On observing a run ruling the driver re-reads authoritative state before
acting (`dec.orchestration-placement` clause 3). If the plan no longer
holds (readiness moved, write-sets no longer disjoint, the commit
advanced), the driver dispatches nothing from the stale plan, records the
decline as an outcome fact, and the console composes a fresh preview.
Consent never becomes execution of a plan the maintainer did not see. With
no driver attached, the ruling sits recorded and visible, and the console
says so rather than simulating a dispatch.

The creation journey's run affordance is enabled only once its grill is
drained; that gating is design, owned by the brief and the prototype. What
this clause does not change: the console still never assigns, acquires or
renews a lease, dispatches, retries, supervises, or executes. Run records
consent; the driver alone starts work.

### 5. Surviving obligations, carried forward whole

The superseded record's live obligations continue in force here,
compressed but complete:

- **Aesthetic direction.** Calibrated Instrument, not a geological
  metaphor rebuild. The geological vocabulary survives only as a
  state-clarity motif: Strata Survey's node-state treatments (ghost and
  orphaned styling, selection emphasis) are adopted into the design
  system. The full metaphor stays declined on evidence, not taste (zone
  rubric 124/160 against 126/160), with its best contribution adopted
  anyway. "Refine, do not redesign" stays narrowed to the product's
  identity: name, voice, and the instrument framing.
- **How aesthetic calls are made.** Two parallel design-studio tracks
  are compared on a shared zone rubric. Track A is greenfield-simulated:
  a worktree with `src/ui_assets` and `docs/design-system` removed, a
  project brief, frozen real `map.json` and API fixtures, and design
  agents running the full create loop under context denial so existing
  assets cannot anchor the output. Track B is iterate-current: a
  review-lane audit of the live webui proposing a polish direction. The
  exploration runs on a throwaway fork and never touches main; its output
  enters the repo only through the existing token and component gates.
  If the tracks converge the direction ships with evidence; if they
  diverge the maintainer picks between two rendered artefacts rather than
  abstractions. Cross-track numbers stay directional, never cardinal.
- **Design quality as a measurable axis.** Pursued as four bets in the
  order D, B, C, A: D, a deterministic design-quality scorer in the
  autoresearch benchmark, built on measurable proxies (severity is
  colour-encoded, the graph uses two or more layout dimensions, dead-zone
  ratio, brand-tone lexicon in copy, motion-safe affordance density),
  each proxy saturated so it cannot be gamed by stuffing, paired with a
  mandatory visual-verification step; B, severity and drift encoded
  visually through colour-coded finding cards, an error-versus-warning
  escalation, and drift felt on the node it points to; C, the two-chain
  hinge as a "trace the truth" surface, rendering missing proof as a
  visible gap rather than the quietest pixel; A, the map placed on the
  declared PROVENANCE / HINGE / AUTHORITY axis with size, weight, and
  visible edges. Bets B and C stay bounded, token-based, reversible; bet
  D stays unbuilt, gated on the visual harness, tracked by
  `todo.webui-design-quality`; committing the scorer as a new benchmark
  baseline goes through a segment bump
  (`init_experiment new_segment: true`), never an ordinary keep.
- **UX-first redesign.** The webui is designed from what cairn is
  supposed to do (orient in a codebase, inspect a node, trace provenance
  into a decision hinge and authority out of it, review findings and
  drift, follow changes), not by iterating the current UI. The two
  greenfield mocks and the landed card-column canvas are scored evidence,
  not the spec; no layout structure is prescribed in advance, the UX
  phase chooses it. Two outcomes stand regardless of information
  architecture: a bounded desktop workspace (only designated panels
  scroll internally, no endless page scroll) and whole-graph legibility
  (the dogfood graph readable in one viewport without scrolling). The
  greenfield codified output (`design-dna.md`, `tokens.css`, the skill)
  stays seed corpus reconciled with `docs/design-system/` into one
  canonical system. Token conformance stays enforced by the existing
  token gate (`dec.webui-design-token-gate`,
  `scripts/check-design-tokens.sh`), extended to the rebuilt
  `docs/design-system/components.css` through that gate's existing
  parameterised-target mechanism, with no new mechanism; all consumers
  (webui, landing, live reference) follow the reconciled set. The
  implementation stays component-based and modular as an outcome, not a
  mapping rule: cohesive responsibilities, explicit data and event
  interfaces, independently testable rendering, built on reusable
  primitives, and never required to mirror the existing ES-module split.
  Public-asset staleness stays tracked in
  `todo.ui-asset-refresh`.
- **The two-way bullet resolves here.** The superseded record held two-way
  interaction as a future exploration whose take-up triggers revisit. This
  decision is that take-up: the clause 1 write set (the surviving
  `dec.control-plane-programme` grant plus the ruling family) is the
  sanctioned two-way surface. Anything beyond it (free-form notes for the
  harness) stays in the `cairn feedback` seam and gains no console
  surface from this record.
- **The AI-vision iteration loop stays foreclosed.** Exactly as ruled: a
  browser-to-vision-critique-to-patch-to-reload loop over
  `src/ui_assets/` is not built. It needs two maintainer-granted
  prerequisites: sanction to add a Node plus Playwright/puppeteer
  toolchain to this deliberately `package.json`-less single-binary Rust
  repo, and a vision provider with its config. A non-deterministic, paid,
  network-dependent model inside a gate is the opposite of how every
  other cairn gate works (`dec.toolchain-lint-strictness`, and the
  local-hooks-over-paid-CI posture). Reopening requires a decision
  superseding this one that records sanction for both prerequisites. A
  vision model stays a manual review aid, never an automated gate. A
  future deterministic visual-regression approach (for example, a pixel
  diff against a checked-in baseline rendered by an already-present tool)
  stays unprecluded as a new unit of work. This closure is why the vision
  question is not re-derived each session: it was raised, escalated, and
  answered.
- **The surface gates stay separate.** `dec.webui-design-token-gate`,
  `dec.webui-a11y-static-audit-gate`, `dec.landing-design-token-conformance`,
  and `dec.marketing-visual-world` remain accepted, distinct authorities.

### 6. Unsigned boundary

Until the maintainer signs this record and the supersession lands, the
`ruling` family and run do not exist: the console records no ruling,
wires no run, and every specimen labels run as not wired. This record
retires nothing on its own, so `dec.webui-design-authority` clause 4 is
not yet superseded. That clause already stands narrowed in practice by
`dec.control-plane-programme` clause 3's signed exception, which is the
tension this record exists to resolve rather than one it creates. The
fallback is therefore exactly today's status quo: the read-only rule
stays on the books unretired, and the console keeps that clause 3 grant
and gains nothing beyond it. A signature accepts or rejects this stated
boundary; silence never grants authority.

## What ratification must do

Accepting this is not a status flip alone. It retires the accepted
read-only ruling, so ratification must resolve that contradiction in the
graph, following the `dec.control-plane-programme` precedent:

- Mark `dec.webui-design-authority` `status: superseded` and add
  `supersedes: [dec.webui-design-authority]` here. Clause 5 carries every
  surviving obligation. The two edits land together in the acceptance
  commit, because `supersedes` only validates once the target is marked.
- Amend the round-1 non-goal line in
  `studio/orchestration-console-brief.md` through the brief's own
  ratification proviso to match clause 4 (run records consent, the driver
  dispatches), and point the J6 table's "when built" column at this
  record's verbs.
- Settle the `cairn todo new` question clause 3 names: either confirm it
  into the clause 1 union as the feedback-promotion verb, or route
  promotion through `cairn todo set` and correct the brief's J6 row. This
  record deliberately does not widen `dec.control-plane-programme` clause
  3's list on its own authority.
- Move every live pointer off the superseded record, correcting the ones
  acceptance falsifies rather than merely repointing them. The set is
  wider than the consequences note below first suggested, and was
  enumerated against the tree: `PRODUCT.md` states that the webui is
  declared read-only, which acceptance makes false; `DESIGN.md`'s
  aesthetic citation, the design-system README lane table and its
  showcase lineage, `dec.marketing-visual-world`'s related edge, the
  standing-trigger citation in `dec.revisit-trigger-correlator-deferred`,
  the aesthetic lines in `studio/orchestration-grill-brief.md` and
  `studio/orchestration-console-brief.md`, and the
  `related` edges and clause references in `todo.ui-asset-refresh`,
  `todo.webui-design-quality`, `todo.revisit-trigger-correlator`,
  `todo.console-signed-widening`, `todo.console-state-legibility`,
  `todo.console-orchestration-ux-design` (a `related` edge and the clause
  4 and clause 5 citations in its round records), and
  `todo.guided-console-prototype`, whose design-authority paragraph
  asserts that the superseded record is accepted and binding, all
  name this record instead. Accepted decisions and archived narrative keep
  naming the superseded id: they describe what was authoritative when they
  were written, which is history and not a stale pointer.

Until an acceptance commit carries all four, this proposal retires nothing
and grants nothing new: the console keeps `dec.control-plane-programme`
clause 3's existing grant with no ruling verbs, which is the fallback
clause 6 states.

## The rubric

- **Tier**: `binding`. It supersedes an accepted authority, grants write
  authority to a user surface, and legislates new sanctioned verbs; only
  the maintainer can sign it.
- **Unblocks**: the four display-only J6 rows and the run moment:
  `todo.console-signed-widening` implements the verbs and queue actions,
  `todo.guided-console-prototype` gets an end-to-end creation journey, and
  `todo.driver-in-repo` gets the obedience contract its loop consumes.
- **Alignment**: against `dec.cairn-mission` first, this decision protects
  the investigable, maintainable, and fit-for-purpose properties by making
  every steering act a recorded, queryable fact on one write path instead
  of a session-bound console action.
  - Goal 1: agents keep working because a recorded ruling outlives the
    session that made it; the driver picks it up without the maintainer
    present.
  - Goal 2: guardrails hold because every console write is one of the
    named verbs recorded in the graph, and no free-form command surface
    exists.
  - Goal 3: the maintainer signs one verb family once; individual retry,
    park, budget, and run rulings are day-to-day facts, not signatures.
  - Goal 4: no surprise signatures: this record is enqueued before any
    verb is implemented, and run stays not wired until the signature.
  - Goal 5: the record sits in `cairn pending` with this rubric, and every
    future ruling renders in the history channel.
- **Options considered**: (a) keep the console at today's clause 3 grant
  and reject run: the creation journey dead-ends into a terminal, rulings
  keep living in chat transcripts, and the queue stays a list of things
  the console cannot do; (b) let the console act on the driver directly
  (a dispatch button, an RPC channel): actions die with the process,
  bypass history, and collapse `dec.orchestration-placement` clause 4;
  (c) rulings as append-only recorded facts the driver obeys, run
  included, the clause 3 grant preserved, general surfaces untouched. (c)
  is the recommendation. The cost of rejecting it is a frozen steering
  surface or an orchestrator built into the view layer.

## Rationale

Facts rather than commands is not a new idea here; it is the pattern the
accepted stack already runs on. `dec.orchestration-placement` clauses 1
and 3 define it (the core starts nothing; the driver re-reads truth before
acting), the Q2 grill ruling applied it to leases, and the Q5 ruling chose
it for the queue. This decision extends the same shape to the last
unsanctioned moments, and run inherits the strongest safety property for
free: a commit-pinned plan identity plus the driver's re-read means a run
ruling can only ever start the wave the maintainer looked at, or nothing.

Whole-record supersession rather than an in-place edit or a bare
refinement: accepted decisions are immutable history, and the narrow
change (one sentence of clause 4) cannot be expressed without restating
the record it lives in. The precedent is followed exactly, including the
acceptance-commit choreography.

Run is ruled in rather than out because the 2026-08-06 amendment names it
as the creation journey's single action. Without a sanctioned verb it
would be either a dead pixel or an unsanctioned side channel; as a
recorded ruling it is the same verb shape as retry and park, which is what
the brief's candidate reconciliation proposed.

## Consequences

- The decision was enqueued in `cairn pending` and accepted on
  2026-08-06; `todo.console-orchestration-ux-design` task 6 is discharged
  by the enqueue, not the signature.
- On signature, `dec.webui-design-authority` joins its own five superseded
  ancestors as historical detail, and this record becomes the binding
  webui design and write authority. Queries keep the lineage.
- Historical narrative in archived changes, research, done todos, and the
  changelog keeps naming the superseded ids. Those records describe what
  was authoritative when they were written and stay unchanged.
- `todo.console-signed-widening` implements the verbs and the queue
  actions; `todo.driver-in-repo` implements obedience (observe rulings,
  act, record outcomes); the rung 3 design under
  `todo.parallel-dispatch-granularity` hosts the fact store.
- `todo.guided-console-prototype` tests run end-to-end on the demo
  project once the signature and the driver seams exist; until then its
  run plate stays honestly not wired.
- Live pointers that name the superseded record move to this one in the
  acceptance commit. The inventory is the fourth ratification action
  above, kept in one place because two lists of it drift and the shorter
  one wins by being read first.
- Future changes to webui design or write authority amend or supersede
  this summary instead of adding another narrow accepted record beside
  it.
