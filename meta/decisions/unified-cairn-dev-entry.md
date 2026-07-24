---
id: dec.unified-cairn-dev-entry
nodes:
  - cairn.kernel.cli
status: accepted
date: 2026-07-23
informed_by:
  - res.harness-engineering
  - res.agent-experiment-linklint
  - src.harness-engineering
refines:
  - dec.loop-command-harness-model
related:
  - dec.no-orchestrator
  - dec.loop-resolves-knowable-gaps
  - dec.agent-pack-packaging
  - dec.adopt-cairn-dev-loop
---

# Unify the one-iteration Cairn development procedure under explicit `cairn-dev` loop mode

## Status

Owner accepted this decision on 2026-07-23. Acceptance sanctions the later
scoped `todo.agent-guidance-router-playbooks` migration, but does not itself
move loop authority: standalone `/cairn-loop` remains the sole normative
authority until that cutover lands. The relation remains
`refines: [dec.loop-command-harness-model]`, not `supersedes`.

## Context

`dec.loop-command-harness-model` ratified the harness-loop shape for
`/cairn-loop`: one unit per fresh session, fail-closed recovery, isolation,
single-commit landing, fail-closed node resolution, terminal tokens, and sole
normative procedure authority in the command plus the skills it loads.
`dec.no-orchestrator` keeps Layer 3 orchestration (controllers, sessions,
formulas, runtime providers, repetition, concurrency) outside Cairn.
`dec.loop-resolves-knowable-gaps` requires investigation and a decision-ready
package for knowable gaps without self-ratification.
`dec.adopt-cairn-dev-loop` remains the accepted ancestry for the development
loop.

The agent-guidance programme needs one logical public entry, `cairn-dev`, with
an explicit loop mode so interactive routing and autonomous loop procedure stop
competing as public guidance surfaces. The fresh three-arm baseline in
`res.agent-experiment-linklint` (`Three-arm navigation baseline (2026-07-23)`)
shows reliable retrieval and Cairn invocation in its secondary engaged cohort,
but found no meaningful quality gain and no support for changing public queries
or runtime authority. The pinned Harness Engineering research
(`res.harness-engineering`) corroborates thin routing, repository-owned
contracts, claim-boundary proof, and source-manifest integrity without
granting implementation or runtime authority.

This unit therefore proposes a refining decision that changes only the
conditional future canonical location of the one-iteration semantic procedure.
It does not implement the migration, does not open a second authority, and does
not grant Cairn any scheduler, controller, session, or parallel-dispatch power.

## Decision

### 1. Acceptance only sanctions later migration

Owner acceptance of this decision sanctions a later implementation unit
(`todo.agent-guidance-router-playbooks` or its successor) to migrate the
canonical repository location of the one-iteration semantic procedure from
standalone `/cairn-loop` into the unified `cairn-dev` entry's explicit loop
mode plus exactly the required procedure closure that mode loads.

Acceptance does not, by itself:

- move loop authority;
- install loop mode;
- demote standalone `/cairn-loop`;
- open a second normative authority;
- require or perform `supersedes` against `dec.loop-command-harness-model`;
- mark `dec.loop-command-harness-model` as `status: superseded`.

Standalone `/cairn-loop` remains the sole normative repository-owned procedure
for one Cairn development iteration until that later migration unit lands the
cutover. This decision continues to `refines: [dec.loop-command-harness-model]`.


### 2. Eight-clause preservation

All eight ratified clauses of `dec.loop-command-harness-model` are preserved.
They are restated here in the vocabulary of logical entry / explicit mode /
canonical asset / adapter transport. The restatement does not weaken any
clause. The only conditional future change is the canonical location named by
clause 8 after the later migration lands:

1. **Harness-owned iteration.** Explicit loop mode performs exactly one unit
   in one fresh session, lands it, emits its terminal outcome, and ends. It
   never selects a second unit; only the invoking user or external harness may
   repeat it.
2. **Router-skeleton shape.** The authoritative loop-mode router asset itself
   retains observation, state classification, the preflight verdict table,
   typed exit-token routing, and fail-closed backstops inline. Only procedures
   such as recovery, landing, scope, implement, and test may move to required
   private assets, each with declared typed exits. Any required-asset failure
   halts rather than permits improvisation.
3. **Fail-closed preflight.** Before selection, observe read-only and apply
   the first matching verdict; classify surviving branches by PR state rather
   than ancestry; finish interrupted work as this iteration; and quarantine
   unlandable work by preserving it with a commit and push, filing a blocked
   recover-todo, and parking the worktree clean. Unclassifiable state is
   `LOOP HALTED` with a report and no writes. Never stash, clean, reset, or
   delete unmerged work.
4. **Isolation.** Every adapter-native loop invocation uses the persistent
   dedicated loop worktree and `loop/*` namespace, and treats everything
   outside that namespace as owned by other sessions. Harness-neutral packaging
   must not weaken this into a harness-specific convention.
5. **Landing atomicity.** One unit, one branch, one PR, one squash commit on
   main, with explicit-path staging and no blanket staging. Relocation through
   a resolver or adapter must not split or aggregate the unit.
6. **Fail-closed node resolution.** A new-work mission resolves only from an
   exact node id or a file path owned by exactly one node. Ambiguity produces
   candidate suggestions and exits. Suffix aliases, semantic inference, and
   filesystem search may help a user form a corrected mission in interactive
   discovery, but must never become accepted resolution forms in loop mode.
7. **Mechanical terminal contract.** Every loop session ends with exactly one
   of `ITERATION COMPLETE`, `LOOP EXHAUSTED`, or `LOOP HALTED` as the final
   line. Resolvers, generated wrappers, and harness adapters must pass the
   token through verbatim and append nothing afterward.
8. **Sole normative authority (location only).** Today, under accepted
   `dec.loop-command-harness-model`, standalone `/cairn-loop` plus the skills
   it loads remains the sole normative repository-owned authority for the
   semantic procedure within one harness-invoked iteration. After the later
   router-playbooks migration lands, the canonical harness-neutral `cairn-dev`
   loop-mode router asset plus exactly its required procedure closure becomes
   that sole normative authority, and standalone `/cairn-loop` becomes
   adapter-native transport only. In either location, the procedure is not a
   runtime orchestrator. The default interactive `cairn-dev` router, adapter
   rows, generated copies, manifests, campaign snapshots, and workflow docs
   are routing, transport, generated, integrity, or descriptive surfaces,
   never competing normative sources.

### 3. Explicit activation

Default interactive `cairn-dev` is a compact router and index. Loop mode
activates only through explicit user or harness selection for each invocation.
Ordinary `cairn-dev` routing must never infer, load, or auto-enter loop mode
from broad skill matching or ordinary development intent. The interactive
router may display an adapter-native invocation, but must not invoke it.

### 4. Selection versus scheduling

Cairn may compute eligibility, dependency order, default priority, and select
exactly one unit inside an already-started invocation. Those are semantic
graph and workflow decisions.

The external harness decides whether and when to invoke, which runtime and
workspace execute the unit, whether another invocation follows, and whether
independent eligible units or review lenses run in parallel. Dispatching a
unit to a worker, claiming a workspace, choosing cadence, and dispatching
multiple units are harness operations.

### 5. Negative authority (runtime exclusion)

`cairn-dev` loop mode is not an orchestrator. It MUST NOT:

- start, repeat, retry, resume, supervise, cancel, or parallelize sessions or
  units;
- own a worker pool, queue, cadence, or concurrency limit;
- select a runtime or provider;
- manage parallel workspaces;
- introduce a scheduler, controller, or loop primitive in the Cairn binary or
  query API.

Those Layer 3 responsibilities remain with the invoking user or external
harness under `dec.no-orchestrator`. Adapters (OMP, Claude, Ralph, Gas City,
cron, or future integrations) are transport and runtime bindings only. Each
resolves the same pinned semantic procedure and required asset closure, but
owns its own invocation, session, repetition, retry, runtime, and
parallel-scheduling policy. No adapter may fork the semantic procedure into a
competing authority.


### 6. Parallel review obligations

Requirements for independent discovery or adversarial review are semantic
obligations. Cairn may require two independent lenses and define how their
findings are adjudicated. The adapter or harness decides whether they run
concurrently, serially, as subagents, or through humans. One primary trajectory
remains the accountable integrator of that evidence.

### 7. Relationship to `dec.loop-resolves-knowable-gaps`

The investigate, frame, and recommend obligation stands:

1. Classify a decision-blocked candidate as a knowable gap or a true external
   blocker.
2. For a knowable gap, the gap is the unit: investigate code, artefacts, prior
   decisions, and sources; adversarially stress-test two to four options;
   persist trade-offs and a justified recommendation as a `meta/` artefact;
   create the blocked or deferred tracker item; land the package; and end with
   `ITERATION COMPLETE` without waiting or self-ratifying.
3. For a true external blocker, surface exactly what is needed and why, then
   end the iteration without inventing authority.

Persistence plus `ITERATION COMPLETE` ends this unit. Only the outer harness
continues. The older wording that an unattended command itself continues to the
next unit is refined by the one-unit harness model and is not restored here.

### 8. Relationship to `dec.no-orchestrator` and packaging

This decision is constrained by and does not reopen or supersede
`dec.no-orchestrator`. It grants no new scheduler, controller, or runtime
authority to Cairn core. It is also compatible with
`dec.agent-pack-packaging`: after the later migration lands, the canonical
harness-neutral loop-mode source is normative; rendered and adapter-native
copies remain generated or transport surfaces, not hand-edited second
authorities.

### 9. Campaign and adapter integrity

An immutable campaign snapshot, when required by a loop invocation, is part of
the required procedure closure; an incomplete snapshot halts before work.
Adapter conformance must exercise clauses 3 to 7, not merely resolve to shared
assets. Shared asset availability proves content, not execution semantics.

### 10. Evidence and proof limits

The anthology supports thin routing, repository-owned contracts, retrieval, and
claim-matched proof. The 2026-07-23 baseline supports guidance retrieval and
invocation only. Neither source supports:

- a measured quality improvement from the unified entry;
- router necessity as a causal claim;
- public query redesign;
- runtime-authority change.

Those limits remain in force for any later implementation or evaluation unit.


## Rationale

One logical entry removes public competing guidance while preserving the
accepted one-unit harness contract. Formal refinement at proposal time keeps
scan clean and avoids premature supersession. Making acceptance a sanction for
later migration, rather than an atomic authority move, prevents deadlock with
`todo.agent-guidance-router-playbooks`, which depends on acceptance first and
then implements the cutover. Explicit negative authority prevents the phrase
"canonical loop authority" from being read as Layer 3 orchestration. The
baseline and anthology are used for what they actually
establish: retrieval and mechanism corroboration, not quality or runtime
claims.

## Consequences

- Owner acceptance has cleared this decision prerequisite for
  `todo.agent-guidance-router-playbooks`; that unit may implement the sanctioned
  migration once its other prerequisites are met, and must not author a second
  competing authority.
- After acceptance and before that later cutover lands, standalone
  `/cairn-loop` remains the sole normative loop authority.
- Interactive node-not-found ladders that use suffix aliases or filesystem
  search remain interactive discovery aids only; loop-mode resolution stays
  fail-closed under clause 6.
- External scheduling, repetition, runtime, retry, and parallelism remain
  owned by the invoking user or external harness.
- Acceptance sanctions only the scoped router-playbooks migration. It does not
  itself move loop authority or authorize unrelated code, pack, adapter, or
  query changes.
