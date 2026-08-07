---
id: dec.rung-three-coordination-substrate
nodes:
  - cairn.root
  - cairn.kernel.query
  - cairn.ui
status: accepted
receipts:
  - rev.rung-three-coordination-substrate-correctness
  - rev.rung-three-coordination-substrate-alternatives
  - rev.rung-three-coordination-substrate-reversibility
ratification: binding
date: 2026-08-07
informed_by:
  - res.parallel-dispatch-rung-3
  - res.inversion-convergence-minutes
  - res.overharness-design-threads
refines: [dec.webui-write-authority, dec.orchestration-placement, dec.control-plane-programme]
related:
  - dec.change-format-only
  - dec.artefact-layout-authority
  - dec.task-tracking-authority
  - dec.todo-relationship-model
  - dec.reviewer-panel-ratification
affects:
  - meta/research/parallel-dispatch-rung-3.md
  - studio/mocks/orchestration-plan-dispatch.html
revisit_triggers:
  - "a driver is run against a second independent clone of this repository, which is the case clause 2 declares out of scope and which Q9 deferred"
  - "the first twenty recorded wave exclusions carry merge evidence, which is the window that sets the promotion threshold clause 3 deliberately leaves unset"
  - "the coordination store is placed on a network filesystem, where the O_EXCL exclusion clause 2 relies on is unreliable"
  - "a unit legitimately spans two unrelated nodes, which the scalar Todo.node cannot express and which would make derived write-sets systematically under-approximate"
  - "the derived-fact slate is ratified with an envelope shape that disagrees with clause 4's required evidence_class"
  - "containment closure is measured to exclude units that could safely have run together, which would question clause 3's reading of node-closure"
---

# Rung three: the coordination substrate

Accepted 2026-08-07 on panel receipts under `dec.reviewer-panel-ratification`
(see the panel record below); authored the same day. `dec.webui-write-authority` sanctioned six verbs and fixed
their contract while deliberately leaving their format to rung 3. None of the
six is implementable until this record is signed: `cairn ruling` is not a
command, the store the verbs append to does not exist, and `cairn ruling run`
has described plan content with no encoding for its argument.

The design is `res.parallel-dispatch-rung-3`. This record carries only the
rulings that need a signature because they refine an accepted decision or fix a
boundary the design cannot fix on its own authority. Everything else in that
document is design detail owned by the implementation todos and needs no
signature.

## Context

Two accepted records meet here. `dec.orchestration-placement` clauses 1 and 3
fix that the core stores and serves facts, evaluates nothing, and starts
nothing. `dec.webui-write-authority` clause 2 fixes that a ruling is an
append-only fact carrying ruling type, target, acting maintainer, `recorded_at`,
commit at recording, and payload, stored "in the shared coordination store that
rung 3 designs", and states plainly that the clause "fixes the contract, not the
format".

The lineage matters and is named rather than implied. `dec.change-format-only`
deleted the generic `StateBackend` persistence abstraction as production-dead,
and separately deleted the live create, claim, and sequence workflow methods on
its beads backend because claiming and sequencing are workflow. **No atomic
claim path exists to extend.** The store this record sanctions is greenfield: it
appends a fact when a sanctioned verb is invoked and serves raw facts when a
query is invoked. Cairn still runs no claiming, sequencing, or workflow logic.

## Decision

### 1. Plan identity is recompute-equality, not commit-pinning

`cairn ruling run` takes `plan-<16 hex>`, the truncated SHA-256 of a canonical
line-oriented preimage over the composer rule, the sorted unit set with each
unit's todo-artefact content hash (`unit=<id>@<sha256-12>`), and their sorted
derived write-set prefixes. The content hash is the panel's correctness graft:
without it a unit's task definition can be rewritten on main while the unit set
and write-sets stay identical, and the driver would dispatch work the
maintainer never read. The base commit is recorded on the ruling fact as
provenance and is **excluded from the hashed preimage**, because hashing it
would pin the digest to the commit by construction and defeat the rest of this
clause. The preimage is one field per line with no escaping; ids and paths
cannot contain a line feed, so the encoding is deterministic across
implementations.

A plan holds when recomposing the wave at the driver's current HEAD yields the
same digest. `dec.webui-write-authority` clause 4 lists "the commit advanced"
as a staleness condition; read literally, any unrelated commit invalidates every
outstanding run ruling, and that clause's own recorded revisit trigger fires
immediately. This clause substitutes recompute-equality for base-commit
equality. Readiness moving and write-sets ceasing to be disjoint are subsumed
exactly, because either changes the recomputed digest.

Nothing is recorded when a preview is rendered, so the console gains no action.
Dispatch is all-or-nothing per wave: a subset wave is a wave the maintainer
never previewed. Consent is single-use and bounded by a driver-policy TTL
derived by the reader from `recorded_at` and an explicit observation time.

### 2. The store is family-local, and cross-clone coordination is out of scope

Ruling, lease, outcome, and singleton facts live under
`<git-common-dir>/cairn/coord/`, one atomically written file per fact, resolved
through a dedicated `--git-common-dir` helper rather than the existing
`git_path`, which resolves per-worktree and would give each worktree a private
store. Mutual exclusion for the driver singleton and for unit lease grants uses
epoch succession with an exclusive create; tokens are never deleted and release
is a fact.
A dead holder's token is never deleted: a successor acquires the next epoch
after deriving, from the facts and an explicit observation time, that the
incumbent chain is stale or released. The core enforces none of that ordering,
since checking liveness at append time would be the core evaluating expiry; a
premature seizure is visible to `cairn coord verify` as two overlapping live
chains, and the authorised-caller trust gap stays deferred exactly where Q9
left it, with its revisit trigger.


Q9 says that across independent clones "repo-synced facts cannot mutually
exclude" and promises detection after sync. That presumes a repo-synced store.
This one is untracked and family-local, so there is no sync and nothing to
detect. **Cross-clone coordination is out of scope for format 1 and the
singleton grant is scoped to a checkout family**, which is exactly the case Q9
ratifies as atomic and enforced. Two clones are two coordination domains. This
is chosen over specifying an export and import verb nobody will run, and over
promising a detection with no mechanism.
Nothing is foreclosed: facts carry a `format` field, unknown formats fail
closed per the existing `read_versioned_json` discipline, and migration to any
future synced store is a fold of the append-only facts in filename order.


The read surface returns raw facts, stamps `schema_version`, echoes the caller's
observation time and consults no clock when none is given, and fails closed
rather than returning a partially resolved store. It carries no `active`, no
`expired`, no `stale`, and no `status`. The appender refuses `lease.*` and
`driver.singleton.*` from a console actor, making
`dec.orchestration-placement` clause 4 a checked invariant.

### 3. Hotspot serialisation is workflow policy, and derived write-sets say so

A unit's write-set is the containment closure of its anchor node (the node plus
its descendants, dependency edges excluded because they are rung 1 Order) mapped
to file prefixes. Declaring blueprint nodes over `docs/registries/`,
`cairn.blueprint`, and `docs/design-system/copy.toml` repairs a real ownership
gap, and **it does not attribute those files to a unit**: a unit anchored
elsewhere that appends an error code has no registry node in its closure, and
that fact does not exist in committed state at composition time.

Therefore, in the derived-first phase, hotspot paths are in no unit's derived
write-set, every derived write-set is stamped `completeness: "partial"` naming
the uncovered prefixes, and contention is resolved by policy: the inert workflow
artefact carries a `serialises:` list of **path prefixes**, cairn validates them
and evaluates nothing, and the driver grants the hotspot permission to one unit
per wave in deterministic order. This reuses Q4's ratified inert-workflow
machinery, adds no blueprint grammar, and adds no per-unit authoring, so Q1's
zero-new-authoring-burden constraint holds.

Promotion to declared write-sets stays gated on measured false-overlap evidence,
computed as a reader-side projection over `outcome.touched_files` facts with no
mutable counter. **The threshold is deliberately left unset**: every candidate
produced one and none had an evidence base, so the first window of real data
sets it in the decision that promotes. The trip is advisory and promotes
nothing.

### 4. `evidence_class` is required from format 1

Every fact carries `evidence_class` as a required envelope field, pre-adopting
the unratified derived-fact slate recorded in
`res.inversion-convergence-minutes`' post-ratification intake and carried in
`todo.parallel-dispatch-granularity`; the pre-adoption is named so the slate's
eventual ruling sees the dependency and diverging from it becomes a refining
decision, never a silent conflict. `deterministic`, `attested`, and `observed`
stay three classes and never blur, the class is fixed per fact kind by the
design document's table so writers cannot choose inconsistently, and an
unknown class fails closed like an unknown format. Requiring now is the
cheap-to-reverse direction: relaxing later is trivial, requiring later strands
every existing fact. `source`, `extractor` with version, `observed_at`,
`freshness`, and `completeness` are carried inside derived payloads only,
where clause 3 makes `completeness` load-bearing.

### 5. The console may not say "claim" without a lease fact

Two corrections to accepted mock copy, both honest downgrades. The console
renders "queues behind that claim" only when a lease fact with a holder and an
expiry exists, and "queues behind that unit" otherwise. And the dispatch
preview's phase-0 sentence names the shared prefix and the permission holder
rather than asserting that a specific unit would change a specific file, which
is knowledge the derivation does not have until declared write-sets land:
"todo.lease-read-surface waits for this wave: only one unit at a time may change
`docs/registries/`, and todo.driver-in-repo holds that permission. Same files,
one at a time."

### 6. The CLI noun union widens by three nouns and two read verbs

`dec.webui-write-authority` clause 3 rules that the ruling family "is exactly
the verbs below". Implementing rung 3 needs more, so the widening is named here
rather than taken silently:

- `cairn ruling list` and `cairn ruling show`, read verbs on the ratified noun.
- `cairn lease grant|renew|release|list`, a new noun, written only by the driver.
  `dec.orchestration-placement` clause 3 already sanctions the concept ("written
  only by the driver through sanctioned verbs") without naming shapes; this
  names them.
- `cairn wave` and `cairn wave stats`, a new read-only noun rendering the
  dispatch preview, its plan digest, and the promotion ratio.
- `cairn coord verify|compact`, a new admin noun over the store.

Nothing here grants the console anything beyond the clause 1 union: `wave` and
the read verbs are passive queries, and `lease` is refused to a console actor by
clause 2's appender barrier. That barrier is a structural field check
(`recorded_by.kind`), deliberately not a liveness check, which the core cannot
perform without evaluating expiry; caller-identity trust stays the Q9 deferral.
`cairn coord compact` moves facts to archives and rewrites nothing, so every
ruling's observable content survives it and the history channel can always be
rendered from archives. Each new noun lands through the full consistency
surface `tests/command_reference_consistency.rs` enforces (docs/commands.md,
integration-contract.md, help module, copy.toml, registries). The noun names
`wave` and `coord` are implementer's taste, cheap to rename before
implementation; the verb SET is what this clause fixes.

## Panel record and acceptance

Accepted 2026-08-07 under `dec.reviewer-panel-ratification`, by a three-lens
adversarial panel (`docs/agent/lenses/contestedness-correctness.md`,
`contestedness-alternatives.md`, `contestedness-reversibility.md`) run
clause-by-clause. Receipts: `rev.rung-three-coordination-substrate-correctness`,
`rev.rung-three-coordination-substrate-alternatives`,
`rev.rung-three-coordination-substrate-reversibility`, each bound to this
record's subject manifest. The maintainer's veto stands open; a veto arrives as
a refining decision.

Clauses 3 and 5 were unanimously convergent. Clause 4 was convergent on the
reversibility lens's own test (requiring now is the cheap-to-reverse
direction), with the pre-adoption now named in the clause. Clause 6's one
alternative (fold `wave` into an existing noun) is naming taste, cheap to
reverse, convergent. Every concrete defect the panel raised was fixed in the
design document and the clauses above before acceptance. Clauses 1 and 2 were
genuinely contested and carry their debates:

**Clause 1 debate.** *For* (recompute-equality with content-hashed units): an
unrelated commit no longer kills consent; any change to what the maintainer
actually read (unit set, write-sets, task text, composer rule) still declines;
the accepted clause's own revisit trigger already forecast commit-pinning
failing, and all-or-nothing dispatch keeps consent meaningful. *Against* (keep
commit-pinning): "the commit advanced" is the ratified text as signed;
declines are safe, merely annoying; literal reading needs no refining ruling.
*Verdict*: recompute-equality, strengthened by the correctness lens's own
alternative (todo content hashes in the preimage, adopted into clause 1). A
repository that commits many times a day makes the literal reading decline
nearly every ruling, which is the exact failure the accepted trigger names,
and content hashing preserves every safety property commit-pinning bought.
Recompute-equality lands.

**Clause 2 debate.** *For* (family-local, untracked): zero git-write machinery
in a codebase with none; `ls`-and-`cat` legibility when wedged; O_EXCL
atomicity for the only records needing it; Q9's enforced case is exactly the
checkout family; append-only replay keeps a future synced store open. *Against*
(git-ref ledger): preserves Q9's cross-clone detection-after-sync; real
compare-and-swap; refs survive worktree pruning; it is the only candidate that
coordinates across clones at all. *Verdict*: family-local for format 1.
Cross-clone coordination has no consumer today (one maintainer, driver not yet
built), so paying five write-side git plumbing protocols now buys a guarantee
nobody exercises, while the replay path and the recorded revisit trigger
(first multi-clone driver use) keep the ref ledger available as the named
successor. The divergence from Q9's paragraph is declared here as a
refinement, not reinterpreted. Family-local ships.

## What acceptance executed

- This record set `status: accepted` with the receipts above; it refines
  rather than supersedes, so no target is marked and
  `dec.webui-write-authority` and `dec.orchestration-placement` stay in force,
  narrowed exactly by clauses 1, 2, 5, and 6.
- `studio/mocks/orchestration-plan-dispatch.html` line 917 was amended to
  clause 5's phase-0 sentence in the same change, through the brief's
  ratification proviso, so `todo.console-signed-widening` implements a
  sentence the store can actually back.
- `todo.parallel-dispatch-granularity` closed as done, and the follow-up todos
  whose only blocker it was (`todo.coord-common-dir-helper`,
  `todo.write-set-derivation`, `todo.hotspot-node-ownership`) went `open`; the
  chained ones stay blocked on their predecessors.

## The rubric

- **Tier**: `binding`. It refines two accepted authorities, narrows a clause of
  each, widens a ratified closed verb set, and fixes a storage boundary that
  every later consumer inherits. Accepted on panel receipts under
  `dec.reviewer-panel-ratification`; the maintainer's veto stands open.
- **Unblocks**: all six verbs of `dec.webui-write-authority` clause 3 and
  clause 4, and therefore `todo.console-signed-widening`'s write surface and
  `todo.guided-console-prototype`'s run plate. `todo.driver-in-repo` gets the
  lease surface and the ready-set write-set contract its task 4 needs.
  `todo.parallel-dispatch-granularity` closes on it.
- **Alignment**: against `dec.cairn-mission` first, this record protects the
  investigable and maintainable properties by making every coordination act a
  raw recorded fact a reader can audit, rather than state held in a running
  process.
  - Goal 1: agents keep working because a lease granted in one worktree is
    visible in every sibling worktree without a commit, so parallel units stop
    colliding silently.
  - Goal 2: guardrails hold because the core still evaluates nothing: expiry,
    resolution, and staleness are reader predicates over raw facts and an
    explicit observation time.
  - Goal 3: one substrate boundary is settled once, on recorded receipts,
    rather than arbitrating each verb's storage as it is built.
  - Goal 4: the intent is recorded before any of it is implemented, and the
    design states what it cannot do (per-unit hotspot attribution, cross-clone
    exclusion, generated-artefact conflicts) instead of overclaiming.
  - Goal 5: the queue carries only this record's outcome review; the panel
    record above and the design document carry the verified evidence.
- **Options considered**: (a) a dedicated git ref ledger, the most correct
  concurrency story and the most new machinery, requiring five write-side git
  plumbing protocols in a codebase whose entire git surface is four read-only
  subprocesses, at the cost of `ls` and `cat`; (b) tracked files on an orphan
  branch checked out as a shared worktree inside the common dir, which buys
  cross-clone sync and `git log` history and stakes the store's existence on
  `git worktree prune` never running; (c) an untracked one-file-per-fact store
  in the git common dir, which needs one new read-only git subprocess and reuses
  `persist::atomic_write`, and which pays for that by having no cross-clone
  story at all. (c) is the recommendation, with its cross-clone gap declared in
  clause 2 rather than papered over. The cost of rejecting it is either teaching
  cairn to write git objects before it has ever written one, or making the
  coordination store's survival depend on a git subcommand a maintainer runs for
  unrelated reasons.
