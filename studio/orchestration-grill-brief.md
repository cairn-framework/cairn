# Orchestration vision: grilling seed brief

Seed for a `grilling` / `grill-me` session (one question at a time,
dependency order, a recommended answer per question, explore the repo
before asking the human). Written 2026-08-03. The interviewer plays
product manager; the maintainer is the customer. Output feeds
`todo.console-orchestration-ux-design` task 2 (journeys); the decisions
it seeds stay with their owners: the webui write-authority resolution
(that unit's task 6), the lease ruling
(`todo.parallel-dispatch-granularity`), and, if Q6 resolves, a
skills-layering ruling whose owning todo is filed against `cairn.root`
as the first step of recording that answer.

The question tree below is a starting hypothesis, not a script. Grilling
branches: an early answer can reshape, reorder, or delete later
questions, and how the product decomposes is itself under test. Only the
"settled" section is fixed ground.

## Settled: the grill must not relitigate these

- `dec.control-plane-programme` (signed): cairn owns policy and
  selection truth; the driver owns assignment, leases, dispatch,
  retries, supervision; the harness owns execution; the console shows
  and records only.
- An agent inside a harness never orchestrates. It writes through
  sanctioned verbs and returns an outcome. The next action happens
  because the driver observed the recorded change or outcome.
- Aesthetic: Calibrated Instrument (`dec.webui-design-authority`); the
  read-only legibility fixes (`todo.console-state-legibility`) proceed
  regardless of any answer below.
- Scope: single repository now; multi-repo aggregation stays parked
  (`dec.workspace-aggregation`).

## Ratification proviso (recorded mid-grill, 2026-08-04)

Every grill ruling from this session is direction with a named
falsifier, at the maintainer's request: the mockup rounds and the first
driver experience test all of it. When a rendered screen or early
orchestrator UX contradicts a ruling, the amendment routes through the
owning artefact (a todo amendment while the ruling is pre-decision, a
refining decision once one is accepted), never as an untracked tweak or
a silent relitigation. The settled section above stays fixed ground;
this proviso covers the question-tree answers only.

## Waiting on one signature

`dec.orchestration-placement` (proposed, binding, in `cairn pending`):
hosts the driver in this repository as a layer beside the passive core,
superseding `dec.product-perimeter` on acceptance with all surviving
obligations carried forward. Recommendation: sign it at the END of the
grill, not before. Design work proceeds now regardless;
`todo.driver-in-repo` and the orchestration-facing console
implementation inside `todo.console-signed-widening` both wait on
acceptance, and signing after the vision session costs nothing.

## The question tree, dependency order

Each question names its downstream dependents, the artefact that owns
the answer, and the current recommended answer.

**Q1. What is the unit of dispatch?** Node, todo, bead, or run, when one
unit touches several nodes. Depends on nothing; almost everything below
depends on it. Owner: `todo.parallel-dispatch-granularity`.
Recommended: the todo (work item) is the dispatch unit; its write-set is
derived node-closure over committed state, promoted to declared
write-sets only on measured false-overlap evidence (already the ratified
constraint in that todo).

**Q2. What is a lease?** Held on what (follows Q1), identity, expiry,
renewal, and what a stale claim looks like against no claim. Owner:
`todo.parallel-dispatch-granularity`, rendered as screens by the design
unit. Recommended: lease on the dispatch unit, not the node; expiry
visible in the console; stale is a first-class rendered state.

**Q3. What does a harness return?** The outcome vocabulary: classes,
keyed how, verified how. Owner: `todo.driver-in-repo` task 3.
Recommended: structured outcome keyed by unit id and commit, using the
ratified terminal tokens (`ITERATION COMPLETE`, `LOOP EXHAUSTED`,
`LOOP HALTED`) as the base vocabulary; any finer split of `LOOP HALTED`
is a proposed mapping the grill must decide, not an assumption. The
driver verifies landed state on `main` before advancing.

**Q4. What is a workflow definition?** The artefact shape for "route
this class of work to that harness with this context" and "on outcome X
go to Y". Depends on Q1 and Q3. Owner: `todo.driver-in-repo` task 2.
Recommended: inert artefact naming predicates, outcome classes, and
permitted next actions; evaluated only by the driver
(`dec.orchestration-placement` clause 3).

**Q5. What are the human moments?** Which rulings reach the console,
and when the loop interrupts a person: signature queue, feedback triage,
park/unpark, retry approval, anything else. Depends on Q3. Owner:
design unit journeys. Recommended: start from the existing spine
(orient, open a ruling, see what signing moves, see what runs next and
why) and add triage; no free-form command surface.

**Q6. Where do skills live once an orchestrator exists?** In-session
skills stay in the pack; do orchestrator-level workflows subsume any of
them, and does the pack teach the driver's workflows to harness agents?
Depends on Q4. Owner: none yet. If resolved, recording the answer starts
by filing the owning todo against `cairn.root`, then the proposed
decision from it. Recommended: workflows reference skills, never
duplicate them; the pack stays the in-session delivery vehicle.

**Q7. Does the visual grammar hold?** Declared intent, observed
actuality, and execution state, each on at least two non-colour
channels. Not an interview question: answered in the mockup rounds by
accepting or rejecting rendered screens. Listed so the interviewer does
not ask it.

**Q8. Which core seams get refactored?** Known candidates, confirm and
order: a selector wire with stable unit id and reproducible evidence
(`cairn next` lacks both); widening the `cairn watch` contract behind a
version; a lease-facts read surface; `findings_error` blast radius (one
Error finding currently disables unrelated read queries). Owner:
`todo.driver-in-repo` task 4 and `todo.convergence-receipt-hash-drift`.
Recommended: selector wire first; everything else on demand.

**Q9. Anything the settled scope missed?** The single-repository
guardrail is settled above; this slot exists for whatever the answers
upstream surfaced that no question anticipated.

## Protocol for the session

1. Load this brief, `cairn pending dec.orchestration-placement`, and the
   three console todos. Explore the repo before asking anything it can
   answer.
2. Ask the remaining interviewable questions one at a time, in the
   current dependency order, each with its recommended answer stated.
   Reshape, reorder, or drop questions as upstream answers collapse
   branches; Q7 is settled by screens, never by interview.
3. Record answers as amendments to the owning artefacts named above,
   through sanctioned verbs and proposed decisions, never as loose
   notes.
4. End state: the placement signature given or refused, the design unit
   unblocked with a ratified vision, and mockup rounds scheduled where
   the interviewer renders screens and the maintainer accepts or
   rejects them.
