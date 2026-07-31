---
id: dec.todo-relationship-model
nodes:
  - cairn.kernel.artefacts
  - cairn.kernel.scanner
status: accepted
ratification: binding
date: 2026-07-31
informed_by: [res.inversion-convergence-minutes]
related: [dec.native-todos-first, dec.north-star-continuous-loop]
revisit_triggers:
  - "topological tiers plus parent grouping plus WorkItem rank prove insufficient to order the roadmap view or the pending-queue unblock sort (reopens the deferred priority field)"
  - "projector or driver work surfaces a relationship kind the trio cannot express (for example discovered-from), rather than stretching related: to carry it"
  - "a terminal todo status beyond done is introduced (redefines unresolved for the contradiction advisory and the roadmap projection)"
---

# Todo relationship model: three typed edges, roadmap as a derived view

Accepted 2026-07-31 by maintainer ratification in session (convergence
slate `res.inversion-convergence-minutes`, row R1 agreed individually,
then all rows agreed). Anticipated by name in accepted
`dec.north-star-continuous-loop` goal 5, which orders the signature queue
"by what each item unblocks once dec.todo-relationship-model gives todos
typed edges". This record is the schema ruling; implementation stays in
the todos it unblocks.

## Context

Todo frontmatter today parses node, status, created, satisfies, and (since
the parked-deferral work) defers. Free-text `related:` appears on todos in
the wild but is parsed only for decisions and silently ignored for todos.
`todo.todo-relationship-model-and-issue-links` is blocked on exactly this
ruling, and the forward programme is invisible to the graph: dependency
order, hierarchy, and parallel-dispatch safety all live in session memory
or external queue files rather than in typed artefact state.

## Ruling

1. **Vocabulary.** A todo may declare exactly three relationship fields:
   `blocked_by:` (list of todo references; a directed dependency, authored
   on the downstream todo), `parent:` (a single todo reference;
   containment, the milestone/epic edge), and `related:` (list of
   references from the resolvable set ruling 2 defines; weak,
   non-directional). No other
   relationship key is recognised.
2. **Reference syntax and identity.** Todos carry no `id:` frontmatter;
   a todo's canonical reference is its filename stem, `todo.<slug>`,
   unique by construction in the flat `meta/todos/` directory. This
   matches the shipped stem conventions: decision `receipts` resolve
   against review file stems (`validate_receipt_links`,
   `CAIRN_DECISION_RECEIPT_UNKNOWN`) and filename validation already
   recognises todos by the `todo.<slug>` stem shape. `blocked_by:` and
   `parent:` entries must be todo stems resolving against loaded todos.
   `related:` entries must be a `dec.`, `res.`, or `src.` id, or a todo
   stem: the identities the registry actually resolves. Contracts,
   reviews, and changes are not referenceable targets (contracts carry
   no id, reviews resolve only as receipt stems on decisions, changes
   are directories); widening the target set is a schema amendment, not
   a stretch of this clause. Renames cascade nothing: a
   rename is an edit whose author updates inbound references, and the
   dangling-reference Warning below is the designed net, the same
   behaviour decision references have today.
3. **`related:` becomes parsed on todos.** Existing in-the-wild `related:`
   entries become typed on parse under the resolution rules of ruling 2.
   `related:` is authored on either side and read symmetrically: one
   entry suffices, reciprocity is never required, and no finding prompts
   for it (the same one-sided convention decision `related:` uses today).
4. **Scanner findings.** Dangling relationship reference: Warning. A
   cycle through `blocked_by` (a work-order deadlock) or through
   `parent` (a containment loop): Error, detected per graph, never
   across their union. `parent` carries grouping semantics only and no
   order, so a mixed chain (a child blocking the epic that contains it)
   is legal; only `blocked_by` orders work. If `parent` ever gains
   completion semantics, union detection is a schema amendment. A
   blocker is unresolved while its status is anything but `done`.
   Status contradiction: Advisory (Info), in two forms: a todo whose
   `blocked_by:` is nonempty and entirely resolved (every entry
   `done`) while its status stays `blocked`, and an `open` todo whose
   declared blocker is unresolved (should the author have marked it
   blocked). A blocked todo declaring no `blocked_by:` is out of this
   finding's scope: blockers outside the todo graph (a maintainer
   sanction, an external capability) are legal and undeclarable under
   ruling 2. Only `open` is flagged on the downstream side: dispatch
   selection considers only open todos, and an `in_progress` todo may
   legitimately carry an unresolved dependency mid-flight. Advisory
   severity preserves agent judgment; the finding is a prompt, not a
   gate.
5. **The roadmap is a derived projection, never an authored artefact.** An
   authored roadmap artefact type is declined: it would be a second writer
   for ordering and status that todos own, with staleness as its steady
   state. The roadmap view is computed over todos whose status is not
   `done`: topological tiers from `blocked_by`, grouped by `parent`,
   ordered within a tier by the
   existing WorkItem rank; surfaced through the CLI and the webui backlog
   channel by the implementation todos.
6. **Priority stays deferred.** No priority field ships with this ruling;
   the first revisit trigger names the reopening condition.
7. **Implementation is not this decision.** Schema and scanner land under
   `todo.todo-relationship-schema-implementation`; the derived view under
   `todo.roadmap-derived-view`; deterministic GitHub links remain
   `todo.todo-relationship-model-and-issue-links`, whose prerequisite (a)
   this ruling half-satisfies (the decision half; the surfaces half stays
   with the implementation todo).

## The rubric

- **Tier**: `binding`. Mechanical facts: both `nodes:` sit inside the
  single container `cairn.kernel` and it supersedes nothing, but it amends
  the Todo artefact schema every adopting repository inherits (spec
  section 8.2 surface) and its affected paths include
  `src/artefacts/registry/`, inside the binding-surface allowlist. The
  maintainer signed in session, 2026-07-31.
- **Unblocks**: `todo.todo-relationship-schema-decision` (done by this
  record); `todo.todo-relationship-model-and-issue-links` (decision half
  of its prerequisite); `dec.north-star-continuous-loop` goal 5's
  unblock-sort for `cairn pending` (named there verbatim);
  `todo.todo-relationship-schema-implementation`,
  `todo.roadmap-derived-view`, and `todo.overharness-console-ux` (filed
  alongside this record); the order rung of parallel dispatch under the
  driver-v2 programme (`res.inversion-convergence-minutes` row R2).
- **Alignment**: against `dec.cairn-mission` first: typed work edges move
  the forward programme from session memory into the graph, which is the
  investigable and maintainable half of "graph, decisions, and gates"
  applied to the work queue itself. Goal 1: dependency edges expose which
  units are genuinely ready, so agents keep working on parallel-safe work
  instead of serialising behind guesses. Goal 2: when a plan dies, the
  blocked chain and parent grouping show exactly what dies with it, so
  re-aiming is graph surgery rather than archaeology. Goal 3: the schema
  amendment itself is binding and carries the maintainer's signature;
  authoring edges thereafter is self-serve. Goal 4: this ruling was
  enqueued and ratified through a convergence slate rather than
  surfacing mid-implementation. Goal 5: it supplies the exact typed data
  goal 5 names for sorting the signature queue by what each item
  unblocks.
- **Options**: (a) an authored roadmap artefact type: declined, second
  writer and staleness by design, and the repository's own history
  (stale queue files, stale next-steps notes) is the measured failure
  mode. (b) Keep free-text `related:` only: declined, unparsed text
  cannot project links, order a queue, or gate contradictions, and it
  produced the measured confusion this session. (c) Ship the full
  priority-and-milestone schema now: declined, authoring burden ahead of
  evidence; deferred behind a revisit trigger. (d) Three typed edges
  with a derived projection (this ruling): the smallest schema that
  makes order, hierarchy, and weak linkage machine-readable.

## Consequences

- The Todo struct gains `blocked_by`, `parent`, and typed `related`;
  parse, validation, and findings land under
  `todo.todo-relationship-schema-implementation` with finding copy in
  `docs/design-system/copy.toml` per convention.
- `cairn pending` gains its goal-5 unblock sort once edges exist.
- The webui backlog channel graduates from a flat list to the derived
  roadmap view under `todo.roadmap-derived-view`.
- `todo.todo-relationship-model-and-issue-links` stays blocked until the
  implementation todo lands its surfaces, then proceeds under its own
  two-phase design.
