---
node: cairn.root
status: open
created: 2026-07-31
related: [res.inversion-convergence-minutes, todo.node-overlap-conflicts-query, todo.console-orchestration-ux-design]
---

# Parallel dispatch granularity: name the three rungs, design the third

`res.inversion-convergence-minutes` row R2. For concurrent units to land
as mergeable PRs, the dispatcher needs computable disjointness, and the
repository currently offers none of it in typed form.

## The three rungs (design constraint, not implementation order)

1. **Order**: typed `blocked_by` edges (`dec.todo-relationship-model`)
   give topological waves. Owned by
   `todo.todo-relationship-schema-implementation`.
2. **Advisory overlap**: the one-hop conflicts query
   (`todo.node-overlap-conflicts-query`), committed state only. This is
   a warning precursor, explicitly NOT merge-safety: it cannot see
   shared files, registries, blueprint edits, generated assets, or
   unpushed claims.
3. **Merge-safety**: a write-set/lease model plus a shared multi-ref
   derived index (`res.overharness-design-threads` thread c; the B-queue
   md5 ledger is the acknowledged single-writer prototype). Canonicity
   never moves: the index is derived and disposable.

## Task

Research and design rung 3 under the driver-v2 umbrella: how a unit
declares or derives its write-set, how leases are granted and observed
across worktrees, and how the serialisation hotspots get explicit
ownership (docs/registries/, cairn.blueprint,
docs/design-system/copy.toml, wire snapshots: the files every unit
touches). Ratified slate constraint the eventual decision must carry
(res.inversion-convergence-minutes fork note): start with derived
node-closure over committed state (zero new authoring burden), promote
to declared write-sets only on measured false-overlap evidence.
Unratified candidate from the slate's post-ratification intake:
every derived fact carries source, extractor plus version, observed_at,
freshness, and completeness; deterministic, attested, and observed are
distinct evidence classes and never blur. Output is a design plus an
enqueued decision, not code.

Added 2026-08-03, from the console's needs: rung 3's lease model must
also answer what the steering surface has to render. What is a claim held
on, a node or a work item, when one unit touches several nodes? What is a
claim's identity, its expiry, and its renewal? What does a stale claim
look like to a reader, and how does it differ from no claim at all? The
word `lease` appears in no Rust source today, so this is genuinely open.
`todo.console-orchestration-ux-design` contributes mockup evidence for
these questions and consumes the ruling; it does not author a competing
one.

## Grill rulings (2026-08-04, maintainer in session)

The orchestration grill (`studio/orchestration-grill-brief.md`) put Q1
and Q2 to the maintainer. Both answers are provisional grill direction
for rung 3's design document, under the brief's ratification proviso;
the pre-existing slate constraint inside Q1 keeps its own ratified
authority, and the document itself still gets authored here.

- **Q1, dispatch unit: the todo (work item).** Confirms the ratified
  slate constraint unchanged: write-set derived as node-closure over
  committed state, promoted to declared write-sets only on measured
  false-overlap evidence. Dispatch is set-valued, not serial: the
  driver takes the ready set (rung 1 waves), filters it to
  pairwise-disjoint write-sets (rung 3), and dispatches one wave of
  units to parallel worktrees. Units whose closures overlap queue
  behind the lease, and the serialisation hotspots named in the task
  keep explicit ownership.
  Units are the only dispatchable identity for the driver: every
  dispatched unit has a todo id for its lease and Q3's terminal
  verification. How a finding-first state (a selectable finding
  precedes any todo in today's loop precedence) becomes dispatchable
  is explicitly open, owned by the selector-wire work
  (`todo.driver-in-repo` task 4) together with rung 3's design
  document, under four recorded constraints. First, the shipped
  `todo.next-recommended-unification` resolution keeps findings
  ephemeral and rejected durable materialisation for
  desynchronisation risk, so any todo-creating transition must
  supersede it explicitly and name its deduplication owner. Second,
  the todo-parked fold skips Info findings only, and `defers:`
  references to Error or Warning findings are invalid, so parking
  alone cannot make every severity non-selectable. Third, the
  ready-set query stays a side-effect-free projection, so any
  transition is a sanctioned mutation by the acting party (driver or
  human), never the query. Fourth, the wave's first member must equal
  a manual Orient selection at the same commit, so both selectors
  must consume the same unit. Until that design lands, the manual
  loop's finding-first precedence stands unchanged and the driver
  dispatches todo-sourced units only.
- **Q2, lease shape: lease facts are cairn truth.** Held on the
  dispatch unit, never the node. This is the reading
  `dec.control-plane-programme` clause 1 already signed: cairn owns
  leases as declarative truth, declarative lease policy, and lease
  facts; the driver owns acquisition, renewal, and active state as
  actions. A lease fact carries unit id, holder (harness kind plus
  session), commit at grant, granted_at, and expires_at. Renewal is a
  driver-performed fact update to expires_at (rare, session-scale TTL
  set by driver policy, never a heartbeat stream). The core stores and
  serves the raw facts, evaluates no expiry, and starts nothing on any
  lease transition: staleness is derived by the reader, driver or
  console, from those facts and an explicit observation time (unit
  in_progress with an expired lease). Stale is first-class and distinct
  from no lease: it carries who held it, when it expired, and the
  recoverable residue (surviving branch, worktree, open PR) that
  recovery policy acts on.

Required core seams this ruling implies (feeds the grill's Q8 and
`todo.driver-in-repo` task 4): sanctioned lease verbs (grant, renew,
release) written only by the driver, a lease-facts read surface the
console and driver re-read, and one shared coordination store visible
across parallel worktrees.

The store is greenfield, and the lineage has two distinct removals
(`dec.change-format-only`): the generic `StateBackend` persistence
abstraction was deleted as production-dead, and the live
create/claim/sequence workflow methods on its beads backend were
deleted because claiming and sequencing are workflow. Either way no
atomic claim path exists to extend. Rung 3's decision must name that
lineage explicitly: the ledger stores and serves driver-recorded facts,
and cairn still runs no claiming, sequencing, or workflow logic.

## Acceptance

- A design document (research artefact) covering derivation, grant,
  observation, and hotspot ownership, with the derived-first ruling and
  its promotion trigger stated.
- The rung vocabulary above appears verbatim, so no consumer mistakes
  rung 2 for rung 3.
- Follow-up implementation todos filed with `blocked_by` edges.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves extendable. It designs safer fan-out as the driver grows.
