# Console orchestration brief: journeys and vocabulary

Round 1 output of `todo.console-orchestration-ux-design` tasks 2 and 4,
authored 2026-08-05 and aligned with the maintainer in session the same
day (recorded answers: ghost vocabulary, quiet synced, quadrant legend,
four driver lamps, six-type queue). A disagreement
amends this file in session or routes through the owning artefact per
the ratification proviso in `studio/orchestration-grill-brief.md`.
This brief
extends `studio/ux-brief.md` (the accepted read-only instrument design) and
does not replace it: every region, component, and interaction there survives
unless a line here names it. The state grammar this brief renders is
`studio/orchestration-state-matrix.md`. No console code changes in this
unit; implementation is owned by `todo.console-signed-widening`.

## Job and audience

Operate mode. One person: the solo maintainer of a repository AI agents are
building, returning at intervals rather than watching live (maintainer
answer, 2026-08-05, amendable through this unit under the proviso). Design
load: about ten units in flight. The eventual shape includes work the
maintainer never started (an external event, a qualifying agent, an
autonomous claim inside a signed boundary), so the console must explain
runs whose origin is not the maintainer.

**Session success: the queue is drained and nothing surprised me.** Both
halves bind. Every entry that waited on the maintainer got resolved, and no
fact turned up later that the console had not surfaced. Completeness and
trust, not triage speed.

## Direction

Calibrated Instrument, unchanged (`dec.webui-write-authority` clause 5; settled
ground). Three moves define the orchestration console against the shipped
read-only instrument:

1. **Invert the signal budget.** Default state is neutral chassis; teal
   marks only the place a move is needed. Today 23 of 24 fixture nodes glow
   mint; after this design, a fully synced idle repository renders calm and
   nearly colourless.
2. **Give each kind of truth its own carrier.** Node chassis for intent and
   fact, bands and marked readouts for projections, run cards and lamps for
   execution state (`dec.orchestration-placement` clause 3; the matrix).
   Nothing about execution ever paints a node.
3. **Add the three orchestration surfaces, all read-or-record, none
   act:** a runs rail (peer of the map, sized for ten units), a typed
   waiting-on-you queue (the only interrupt surface, Q5), and the
   consequence sentence (what signing a ruling moves, stated in words).

Non-goals, standing for this design: no lease acquisition, no execution,
no supervision, no free-form command surface, and no act on the driver
from any console affordance. Amended on acceptance of
`dec.webui-write-authority` (2026-08-06), through this brief's own
ratification proviso: the earlier form read "no dispatch button", "no
retry trigger", and "no dispatch from any console affordance", which
clauses 3 and 4 settle. The act and record distinction is what moved, not
the boundary. The console records rulings through sanctioned verbs (retry,
release, park, unpark, budget, and run), and the driver alone acts on
them, re-reading state first and declining a plan that no longer holds.
The console shows, and it records.

## Surface inventory

Named here so journeys can reference them; layout geometry stays with
`studio/ux-brief.md` regions.

- **Bezel annunciator row** (extends `StatusBezel`): reconciliation state,
  findings count, and a driver lamp with exactly four states (NO DRIVER;
  DRIVER IDLE with last-poll time; DRIVER CRASHED with recorded exit
  status and time; DRIVER UNRESPONSIVE with last heartbeat and a stated
  missing termination record. ACTIVE is implied by a non-empty runs rail,
  not a fifth lamp).
- **Map** (existing `GraphCanvas`): nodes carry intent and fact states;
  strata bands group by dependency tier (`/api/roadmap`, frontier `tier`).
- **Runs rail** (new, peer of the evidence rail): one card per unit in
  flight; lamp, unit id (mono), elapsed or as-of readout, lease tag.
- **Waiting-on-you queue** (new): the single typed interrupt surface. Six
  entry types (Q5): signature, retry approval, quarantine release, feedback
  triage, budget approval, park and unpark.
- **Evidence rail** (existing): gains the consequence sentence in decision
  context.
- **History channel** (extends `ChannelBar`): receipts, outcomes, and
  superseded records; where local-tier decisions land with receipts.

## Journeys

Each step names what the maintainer sees and the query that answers it.
Sources marked **proposed** do not exist; they name the owning artefact and
render honestly as absent until built. The spine (J1 to J5) is the
maintainer's own stated path.

### J1. Return and orient

1. Open the console. Bezel: reconciliation state and findings count
   (`GET /api/status`, `GET /api/lint`); driver lamp (**proposed** driver
   contract v1, `todo.driver-in-repo`; absent contract renders NO DRIVER).
2. "What happened while I was away": history channel renders
   `recent_log_entries` (`GET /api/status`) and recorded outcomes
   (**proposed** cairn outcome-facts read surface; facts driver-written
   through a sanctioned verb). Known gap, not papered over: no since-cursor
   query exists; round 2 renders the panel from what `recent_log_entries`
   carries, and if that proves too thin the gap routes to the driver unit
   as a query need, never invented here.
3. "What waits on me": queue badge with typed counts (`GET /api/pending`
   for signatures; other types **proposed**, see J6).
4. "What is in flight": runs rail (**proposed** driver contract; leases
   from the **proposed** cairn lease-facts read surface,
   `todo.parallel-dispatch-granularity`).

### J2. Open a ruling that waits on you

1. Queue shows signature entries oldest-first with age
   (`GET /api/pending`: `id`, `age_days`, `nodes`, `ruling_summary`).
2. Open one: rubric, evidence links, `changed_since_review`, and
   `ruling_prompt` render in the evidence rail (same payload).
3. The entry is display-only routing to the maintainer's own signing flow
   (Q5): the console shows `reopen_command` and the ruling prompt verbatim
   (mono), and offers no accept button.

### J3. Follow it to the map

1. The ruling names its nodes (`pending[].nodes`); selecting one focuses
   the map (`GET /api/graph`), state per the matrix.
2. The evidence rail lineage plate shows the chain: evidence in, decision
   hinge, authority out (`GET /api/node/<id>/decisions`, `/research`,
   `/sources`; `cairn decisions <node>`).
3. Neighbourhood reads answer blast radius (`GET /api/depends/<id>`,
   `GET /api/dependents/<id>`).

### J4. See what signing it moves

1. The consequence sentence renders in the decision context, composed
   entirely from existing queries: the decision's gated nodes
   (`pending[].nodes`) intersected with frontier blocking chains
   (`GET /api/frontier` `blocked[].blocking`) and tier positions
   (`GET /api/roadmap`).
2. Stated as a sentence, not a badge: "Signing dec.example unblocks 3
   units in tier 2 (a, b, c) and moves nothing else." A composition is a
   projection, so it carries the derivation mark and its observation time.
3. On the map, the affected units' band positions are indicated while the
   sentence is in view; chassis states do not change (signing has not
   happened).

### J5. Understand what the driver will do next, and why

1. The `next` slot shows selection truth with attribution: today
   `next_recommended` (`GET /api/status`, from the committed export),
   and an empty `next_recommended` renders as "no recommendation right
   now", never as an invented id. The selector wire with stable unit id
   and reproducible evidence is a named seam (Q8, `todo.driver-in-repo`
   task 4); when it lands, the slot shows its evidence instead.
2. Policy context: workflow artefacts are inert typed policy the driver
   alone evaluates (**proposed** schema, `todo.driver-in-repo` task 2); the
   console renders the matched rule read-only.
3. With no driver attached, this journey stays honest: "No driver
   attached. cairn status recommends `<id>`" (or, when
   `next_recommended` is empty, "cairn status recommends nothing right
   now"). "A driver would evaluate workflow policy against the
   frontier." Nothing simulates a dispatch.

### J6. Drain the queue

Each of the six typed entries names its write path (Q5 ruling, recorded in
the design todo):

| Entry type | Write path today | Write path when built |
|---|---|---|
| signature | display-only routing to the maintainer's signing flow | unchanged |
| feedback triage | `cairn feedback` intake exists; promotion via `cairn todo new` | unchanged, and now sanctioned explicitly: `cairn todo new` was confirmed into the console's write set on acceptance (`dec.webui-write-authority` clauses 1 and 3) |
| retry approval | none: renders display-only | `cairn ruling retry <todo-id>`, a recorded fact the driver obeys (`dec.webui-write-authority` clause 3), implemented by `todo.console-signed-widening` |
| quarantine release | none: display-only; never auto-retried | `cairn ruling release <todo-id>` |
| park and unpark | none: display-only | `cairn ruling park` and `cairn ruling unpark <todo-id>` |
| budget approval | none: display-only (Q9 budget-exhausted moment) | `cairn ruling budget <todo-id or wave-id> --cap <value>` |
| run | none: every specimen labels it not wired | `cairn ruling run <plan>`, consent to one composed plan the driver re-reads before acting (clause 4) |

Tier split: binding-tier items interrupt by queueing; local-tier decisions
land agent-side and appear as receipts in history, never as interrupts. A
ruling is a recorded fact, so it survives driver restarts and shows in
history: nothing here is a console action on the driver.

### J7. Reconstruct work you never started

The unattended ambition (maintainer, 2026-08-05): an external event (a PR,
a qualified bug report) enters, an agent qualifies it against a signed
boundary, and inside that boundary the unit is claimed, worked, and merged
autonomously. The console explains it after the fact, entirely from
recorded truth:

1. Intake receipt in history: the event and its qualification evidence
   (**proposed**: intake is a driver and workflow concern; until it exists
   the feedback lane, `cairn feedback`, is the only intake).
2. Claim: lease fact with holder and grant time (**proposed** cairn
   lease-facts read surface).
3. Work: run card while live (**proposed** driver contract); outcome fact
   with terminal token and commit when done (**proposed** cairn
   outcome-facts read surface).
4. Landing: the outcome's commit link; the map shows the unit's chassis
   state change on the next reconcile (`GET /api/graph`, existing).
5. Every step that crossed the boundary shows its receipt; the one that
   would have crossed a binding line shows the queue entry it would have
   raised instead.

This journey is the "nothing surprised me" test: if any step cannot name
its recorded source, the design has failed it.

## Vocabulary ruling (task 4)

`planned`, `ghost`, `declared`, and `buildable` are four words for one
state today. Audit of where each lives:

- `ghost`: the wire and enum word (`NodeState::Ghost`,
  `src/map/graph.rs:14-23`, serialised verbatim); spec-protected taxonomy
  (AGENTS.md); the CLI's word (`cairn frontier` reports ghost nodes).
- `planned`: the shipped UI label for ghost (`docs/design-system/copy.toml`
  `[webui.states]`), normalised back to ghost in `src/ui_assets/utils.js:23-27`.
  Collides with an unrelated Rust gate state `Planned`
  (`src/verification.rs:4-16`, verification lifecycle), which the console
  will eventually also render.
- `declared`: the name of the intent **family axis** in
  `dec.orchestration-placement` clause 3; using it as one state's label
  poisons the axis it belongs to.
- `buildable`: a projection, not a state: frontier `ready[]` membership,
  time-varying, computed at query time.

**Ruling: the word is `ghost`, everywhere.** The console renders the state
word `ghost` (mono, lowercase, the wire word) with the legend gloss
"Declared, not yet observed". The `Planned` UI label is retired: one
concept, one name, and a maintainer reading `cairn frontier` in a terminal
and the console side by side must never translate. `declared` is reserved
for the family axis label. `buildable` never names a state: the frontier
band is titled Ready, with the one-time gloss "buildable now", and blocked
units read "blocked by N".

Rationale against the voice bar (`docs/agent/voice.md`): accuracy is the
floor and load-bearing taxonomy is never flattened; with the gloss adjacent,
a non-developer reads `ghost` correctly on first contact, and the euphemism
tax of `Planned` (two vocabularies, one collision) fails the "nothing
surprised me" success test. Scope: this ruling is a design output; the
rename itself is `todo.console-signed-widening` task 3
(`copy.toml` label, legend copy); `todo.console-state-legibility`
deliberately does not carry it.

## States and ranges

- Load: ten units in flight (cards stay readable; no virtualisation
  needed); 24-node fixture as map baseline; queue depth zero to a dozen.
- Empty states are first-class: driver absent, driver idle, empty queue
  ("Nothing waits on you"), empty history, zero findings. Each states the
  fact and the next read-only action (`studio/ux-brief.md` convention).
- Degraded states: driver crashed (recorded termination fact: exit status
  and time); driver unresponsive (stale heartbeat, no termination record,
  derived and labelled as such, never hardened into a crash verdict); stale
  lease (unclassified, with residue); findings error present (the
  `findings_error` blast radius, Q8, may disable unrelated read queries
  today: the console states which queries are unavailable rather than
  rendering blanks).

## Round 2 handoff

Required static scenarios (todo task 3), each rendering rows from the
matrix: mixed repository carrying every grammar row at once;
decision-to-consequence; driver connected with runs, leases, and a dispatch
preview (the preview is a readable declaration of what the driver would do,
never a trigger); driver absent against idle against crashed (recorded
exit) against unresponsive (derived, stated missing record); narrow
layout. Lease and granularity screens are the evidence handoff to
`todo.parallel-dispatch-granularity` (task 5): they must render claim
identity, holder, expiry, renewal, stale-against-no-claim, and write-set
overlap so the rung 3 ruling is made against pixels. The webui
write-authority decision (task 6) is authored in round 3, following the
`dec.control-plane-programme` supersession precedent; the sanctioned-verb
gaps J6 names are its input.

## Amendment: the guided journey and the layered graph (2026-08-06, in session)

Recorded from the maintainer's vision statements during round 2 and aligned
in session the same day; provisional under the ratification proviso in
`studio/orchestration-grill-brief.md`. Where this section touches an
accepted decision or a round-1 non-goal, it changes nothing by itself: the
owning artefact rules, and the collisions are routed below.

### The primary register is plain language

The console's end user may not be technical; they must be able to trust
that developing with cairn yields quality software without reading the
machine layer. Outcomes and guided next steps are primary; the technical
working (ids, rules, leases, contract paths) is progressively disclosed,
never the front door, and even the disclosure layer prefers intuitive
visual representations over text. The aligned reference rendering is
`studio/mocks/orchestration-guided-journey.html` (aligned 2026-08-06):
conversation rail, grill questions in plain words, one run action, waves
as sentences beside a ghost mini-map, a glyph-strip working drawer.

### The talking rail is the spine

The journey is conversational: the maintainer describes UX and outcomes,
and that decodes into the blueprint. Doubts drain through a grill
workflow surfaced as plain questions with selectable answers: cairn
agentic workflows baked into the UI layer, extrapolating from
requirements and user stories and orchestrating stress-testing until the
map of what is to be done exists visually. Then, and only then, run.

### Run, and its collision

The vision names a run moment: after every question is answered, one
action starts the orchestrated waves, and layers unlock as dependencies
land. This collides with this brief's non-goal line (no dispatch from any
console affordance) and with `dec.orchestration-placement` clause 4. The
candidate reconciliation, pending ruling: run is a recorded ruling the
driver obeys, the same verb shape as retry and park. The write-authority
decision (todo task 6, round 3) owns this verb; until it rules, every
specimen labels run as not wired.

### The layered graph

The workflow and execution layers are a separate graph from the
architecture graph, and cairn's UI connects the layers rather than
forcing one graph to be everything: architecture visualised from
blueprint to actual, execution visualised as real progress (ghost
structures visibly filling in, elegant readouts of how many processes and
agents run), and the temporal axis alongside (the maintainer's phrase: a
4D graph). Workflow tuning surfaces (strong defaults, visually tunable)
are explicitly deferred. AutoGPT's execution graphs are noted as
reference material for the execution layer, to be reviewed when that
layer is designed.

### Prototype first, no deep hypotheticals

Designing for universal harness compatibility as we go was adding
complexity and stops. The target deployment is the maintainer's own:
self-hosted in Docker on a cloud server (later possibly several), long
running. The path: settle this first phase (the creation journey), build
a prototype the maintainer can run against a demo project (for instance a
calculator app), then design against their real user feedback. Remaining
round-2 scenario mocks beyond the aligned screens are deferred in favour
of that loop; the reconciliation of task 3's required-scenario list
happens with the round-3 close-out.

### Screen ledger as of 2026-08-06

- `orchestration-guided-journey.html`: aligned; the creation-phase
  primary view and the register reference.
- `orchestration-return-orient.html`: called good as the reactive
  catch-up layer, with the density critique standing; retrofit pending
  under the plain-primary register.
- `orchestration-plan-dispatch.html`: called right direction, not
  aligned; its jargon moves behind disclosure in the prototype.
- `orchestration-mixed-repository.html`: grammar and lease evidence
  appendix; not a composition reference.