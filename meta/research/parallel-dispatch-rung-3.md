---
id: res.parallel-dispatch-rung-3
nodes: [cairn.root]
date: 2026-08-07
sources: [src.orchestration-plan-dispatch-mock]
---

# Rung 3 design: the coordination substrate

Design document owed by `todo.parallel-dispatch-granularity`. It designs three
things and nothing else: plan identity, the ruling fact store, and write-set
derivation with its disjointness test. It contains no implementation.

Written against the accepted mockup evidence
(`studio/mocks/orchestration-plan-dispatch.html`,
`orchestration-mixed-repository.html`, `orchestration-return-orient.html`) and
the repository at `2dcf9af`. Every repository claim below was verified against
the tree; the two load-bearing ones are recorded with their commands in the
appendix.

The rulings this design needs a maintainer signature on are collected in
`dec.rung-three-coordination-substrate`. This document is the evidence; that
record is the authority.

## Rung vocabulary, verbatim

Reproduced from `todo.parallel-dispatch-granularity` so no consumer mistakes
rung 2 for rung 3:

1. **Order**: typed `blocked_by` edges (`dec.todo-relationship-model`) give
   topological waves. Owned by `todo.todo-relationship-schema-implementation`.
2. **Advisory overlap**: the one-hop conflicts query
   (`todo.node-overlap-conflicts-query`), committed state only. This is a
   warning precursor, explicitly NOT merge-safety: it cannot see shared files,
   registries, blueprint edits, generated assets, or unpushed claims.
3. **Merge-safety**: a write-set/lease model plus a shared multi-ref derived
   index (`res.overharness-design-threads` thread c; the B-queue md5 ledger is
   the acknowledged single-writer prototype). Canonicity never moves: the index
   is derived and disposable.

This document designs rung 3.

## Lineage: what was deleted, and why nothing here extends it

`dec.change-format-only` made two distinct removals, and rung 3 must name both
rather than appear to revive either.

The generic `StateBackend` persistence abstraction (with `StateRecord`,
`FilesystemStateBackend`, `BeadsStateBackend`, and `storage_backend()`) was
deleted as production-dead. Separately, the live create, claim, and sequence
workflow methods on its beads backend (`create_change_epic`,
`create_task_beads`, `list_child_tasks`, `claim_change`, and `src/state/beads.rs`
itself) were deleted because claiming and sequencing are workflow, which the
core does not do.

The consequence for this design is exact: **no atomic claim path exists to
extend**, so the store below is greenfield rather than a restoration. What it
restores is nothing. The ledger stores and serves driver-recorded facts. Cairn
still runs no claiming, sequencing, or workflow logic: it appends a fact when a
sanctioned verb is invoked, and it serves raw facts when a query is invoked.
`src/state/` stays what `dec.task-tracking-authority` made it, a read-only
adapter over `.beads/issues.jsonl` with no mutating surface, and is not touched.

## Part 1: plan identity

### The argument

```
cairn ruling run plan-9f4c1d0b7a3e5628
```

A `plan-` prefix and the first sixteen hexadecimal characters of a SHA-256 over
a canonical preimage. Sixteen hex characters are copy-pasteable from the
console, greppable in the history channel, and speakable in halves. Nothing is
recorded when the preview is rendered, so the identity costs no write.

### The canonical preimage

Line-oriented, LF-terminated text rather than JSON, because a decline is
explained by diffing two preimages and the console must print the changed line
as a sentence.

```
cairn-plan-v1
rule=wf.default:3
unit=todo.driver-in-repo@9f3c1a7b4d02
ws=docs/registries
ws=src/driver
unit=todo.lease-read-surface@4d02af19c3be
ws=src/query_api
```

Each `unit=` line carries the todo id and, after `@`, the first twelve hex of
SHA-256 over the todo artefact's bytes at composition. This is the panel's
correctness graft: without it, a unit's task definition can be rewritten on
main while the unit set and write-sets stay identical, and the driver would
dispatch work the maintainer never read. With it, any change to what was
actually consented to changes the digest and declines. The separator is safe
because todo stems are kebab-case ids that cannot contain `@`.

Normalisation rules, all load-bearing:

- Units sorted ascending by todo id; `ws=` prefixes sorted ascending within a
  unit.
- Paths carry **no trailing slash**. This is not cosmetic. The existing
  component-boundary check at `src/reconcile/generic.rs:410` reads
  `file.as_bytes().get(path.len()) == Some(&b'/')`, so a stored prefix of
  `docs/registries/` makes the check inspect the first byte of the filename and
  fail. Prefixes are stored bare.
- Leading `./` stripped, backslashes normalised to forward slashes.
- Exactly one trailing LF, no other trailing whitespace. One field per line and
  no escaping anywhere: todo ids and repository paths cannot contain a line
  feed, so the line-oriented form is already delimiter-unambiguous across
  implementations.

Two things are deliberately absent from the hashed preimage.

`query_api::SCHEMA_VERSION` is absent: a wire bump has nothing to do with
whether a wave is safe, and including it would kill outstanding consent for an
unrelated reason.

**The base commit is absent.** This is the correction that makes the rest of
Part 1 coherent. An earlier draft put `base=<40-hex>` inside the hashed
preimage while simultaneously claiming the plan is not pinned to the commit.
Those cannot both hold: hashing the base commit pins the digest to it by
construction, so a recomputed wave with an identical unit set and identical
write-sets at a later commit yields a different digest, and every plan dies on
the next commit anyway. The base commit is recorded on the ruling fact envelope
as provenance, and it is never compared.

The `cairn-plan-v1` magic line versions the preimage format itself.

### Who computes it, and when

Nobody records a plan. The console renders the preview by calling the same wave
composer the driver calls, and prints the digest it computed. `cairn ruling run`
records exactly one fact, whose `payload.target` is the digest string.

This is why the content-addressed identity beats a recorded plan artefact. A
recorded preview would be a second write the console performs on its own
initiative, at preview time rather than at consent time, and
`dec.webui-write-authority` clause 2 rules that a ruling is a recorded fact,
never an action. Recording nothing at preview keeps the console's entire write
surface to the ratified `ruling` verbs.

**The composer is one function with two callers.** Recompute-equality is only
trustworthy because the console preview and the driver's re-read run identical
code. That is a design obligation, not an implementation detail, and it is
stated so the console and the driver cannot drift into two compositions. The
console is an in-binary HTTP server and the driver is a separate layer above the
core, so both consume the composer through the same passive query surface
(`cairn wave`, Part 2) rather than by linking two copies.

### The predicate: does the plan still hold

```
holds(plan_digest, at) :=
    let wave = compose_wave(HEAD, at)
    in digest(preimage(wave)) == plan_digest
```

A plan holds when recomposing the wave at the driver's current HEAD yields the
same composer rule, the same unit set, and the same derived write-sets. It is
stale when the composition changed, not when the repository moved.

`dec.webui-write-authority` clause 4 lists three staleness conditions:
readiness moved, write-sets no longer disjoint, the commit advanced. The first
two are subsumed exactly by recompute-equality: if readiness moved the unit set
changes, and if write-sets stopped being disjoint the composer excludes a unit
and the set changes. The third, read literally, means any unrelated commit (a
typo fix in a README) invalidates every outstanding run ruling. That clause
carries its own recorded revisit trigger for precisely this failure, and the
trigger would fire on the first day of use.

**Refining ruling 1** substitutes recompute-equality for base-commit equality.

### Consent is bounded, by the reader

A run ruling has no expiry of its own, because the core evaluates nothing. The
driver applies a consent TTL from its own policy, derived by the reader from
`recorded_at` plus an explicit observation time. Default twenty-four hours.
Past it the driver declines with `consent-expired` rather than dispatching a
wave the maintainer approved yesterday.

### The decline path

```
kind: outcome.run_declined
payload:
  target: plan-9f4c1d0b7a3e5628
  reason: <closed enum>
  observed_at: <RFC3339, supplied by the driver>
  head: <40 hex>
  causes:
    - unit: todo.lease-read-surface
      predicate: write_sets_overlap
      blocking_fact_id: 4d02af19c3be   # or null when the cause is derived
      detail: "docs/registries is claimed by todo.driver-in-repo"
  preimage_diff: <unified diff, spilled to a sidecar file above 4 KiB>
  dispatched: []
```

`causes` is the console's material and is structured per unit, because a raw
preimage diff tells a maintainer that a line changed without telling them which
unit lost and why. The diff is retained as supplementary evidence.

`dispatched: []` is normative, not descriptive: **dispatch is all-or-nothing per
wave**. A subset wave is a wave the maintainer never previewed.

The closed reason enum, each with a registered error code:
`readiness-moved`, `write-sets-overlap`, `unit-set-moved`, `parked`,
`lease-held`, `ruleset-changed`, `consent-expired`, `already-consumed`,
`superseded-by-concurrent-ruling`.

### Replay, collision, and a driver already mid-wave

Consent is single-use. On dispatch the driver records `outcome.run_consumed`
carrying the digest. A run ruling whose digest already has a `run_consumed` fact
declines with `already-consumed`, which also closes the case where repository
state cycles back to a previously consented composition.

Two rulings on the same digest, or two rulings whose recomputed waves share a
unit: deterministic reader-side tie-break on the pair
`(recorded_at, fact_id)`. This is a total order over the **fully listed** fact
set, not a cursor, so it is unaffected by the ordering hazard Part 2 records and
both readers reach the same answer without a sequence allocator. The loser
declines with `superseded-by-concurrent-ruling`. This is the case where the
maintainer consents from the console and from a terminal within the same
minute, and both consumers must render the same outcome.

A driver already dispatching a prior wave queues an arriving run ruling in its
own policy layer. That is driver scheduling, which is where scheduling belongs;
the core neither queues nor orders anything.

## Part 2: the ruling fact store

### Host

```
<git-common-dir>/cairn/coord/
```

resolved by `git rev-parse --git-common-dir`. It must be a **dedicated helper,
not the existing `git_path` at `src/cli/commands/hook.rs:181`**. Verified in a
secondary worktree:

```
$ cd /Users/george/repos/cairn-vibe-edit
$ git rev-parse --git-path cairn/coord
/Users/george/repos/cairn/.git/worktrees/cairn-vibe-edit/cairn/coord
$ git rev-parse --git-common-dir
/Users/george/repos/cairn/.git
```

`--git-path` resolves an unrecognised path against the per-worktree gitdir, so
reusing it would give every worktree its own private store and silently defeat
the one seam rung 3 exists to provide. `src/hooks/architecture.rs:172` already
hardcodes `root/.git/HEAD` and is already wrong in a secondary worktree for the
same reason; the helper fixes both callers.

### Why here, against the alternatives by name

`meta/` is out. `dec.artefact-layout-authority` reserves it, flat, for
scanner-loaded graph artefacts, and it is branch-local, so worktree B cannot see
worktree A's grant.

`.cairn/` is out. Gitignored is correct for this data, but `.cairn/` is
per-worktree, which fails the required seam outright.

`.beads/` is out. `dec.task-tracking-authority` makes it a subordinate read-only
view that never mints anything.

A dedicated git ref is the most correct concurrency story available and the most
new machinery by a wide margin. It needs `hash-object -w`, `mktree`,
`commit-tree`, `update-ref --stdin`, and `cat-file --batch`: five write-side git
plumbing protocols in a codebase whose entire git surface today is four
read-only subprocesses and no library. It also costs `ls` and `cat`, which is
the maintainer's only recourse when the store is wedged.

An orphan branch checked out as a shared worktree inside the common dir gets
cross-clone sync for free, and stakes the store's existence on `git worktree
prune` never running.

### Format: one file per fact, no log, no lock

```
<git-common-dir>/cairn/coord/
  format                                                   # "1\n"
  facts/20260807T034512Z-ruling.run-9f3c1a7b4d02.json
  facts/20260807T034513Z-lease.grant-4d02af19c3be.json
  leases/<unit-id>/epoch-000003.json                       # exclusion tokens
  singleton/epoch-000007.json                              # exclusion token
  cache/parsed.json                                        # derived, disposable
  archive/2026-06/...
```

One file per fact means disjoint paths, which means concurrent writers never
contend, which means no `flock`, no new dependency in a crate with
`clippy::cargo` at deny, and no `PIPE_BUF` interleaving hazard of the kind the
existing `cairn feedback` append at `src/cli/commands/feedback.rs:51` accepts.
Writes reuse `persist::atomic_write` (temp file in the target parent, then
rename) exactly as every other cairn state file does.

The filename sorts lexicographically by recording time and names its kind, so a
maintainer runs `ls` and reads the store. `--since <filename>` is a display and
pagination convenience only.

**It is never a correctness boundary, and no reader may fold incrementally above
a high-water mark.** Filenames are second-precision, so within one second the
order is decided by `<kind>-<fact_id>`, which has nothing to do with creation
order. `persist::atomic_write` stamps the name, writes a temp file, then
renames, so a writer can choose a name and land it after a reader has already
marked a higher-sorting name from the same second. One machine, one second, no
clock skew: a console writes `...034512Z-ruling.run-9f3c....json` and a reader
marks there; the driver's `...034512Z-lease.grant-4d02....json` renames in
microseconds later, sorts below the mark, and is never folded. The dropped fact
is a lease grant, so the driver's ready-set filter sees an unleased unit and
dispatches over a held claim, which is the single failure rung 3 exists to
prevent. Failing closed on a truncated read does not help: a reader cannot fail
closed on a fact it never learned existed.

### Cross-worktree visibility

The common dir is shared by every worktree of a checkout family, whatever branch
each one sits on. A lease granted in worktree A is readable in worktree B on its
next read, with no commit, no merge, and no branch. This is the seam
`todo.parallel-dispatch-granularity` requires, and it is the reason the store is
not a tracked file.

### Within-family atomicity

Two records need mutual exclusion: the driver singleton grant, and a lease grant
on a unit. Both use the same primitive, and it is not a plain O_EXCL on a
timestamped name, which would provide no exclusion at all (a fresh timestamp
never collides) nor on a fixed name, which would block every future acquisition
once written into an append-only store.

**Epoch succession.** To acquire, read the highest existing `epoch-NNNNNN.json`
under the target directory, then `OpenOptions::new().create_new(true)` on
`epoch-<N+1>.json`. Exactly one writer wins the create; a loser sees
`AlreadyExists`, re-reads, and either accepts the winner or retries against the
new maximum. Tokens are never deleted, so the append-only discipline and the
audit trail both hold, and release is a `driver.singleton.release` or
`lease.release` fact naming the epoch rather than an unlink.

The driver singleton is the primary guarantee, and it already serialises lease
grants because only one driver grants leases within a family. Using the same
primitive on unit leases costs nothing and keeps the single-driver assumption
from being load-bearing.

**Takeover of a dead holder's token.** A crash before release leaves the
highest epoch file in place forever, and that is by design: the token is the
exclusion primitive, not the liveness record. Who currently holds is read from
the fact log, and staleness is reader-derived (`expires_at <= at`). A successor
acquires by creating `epoch-<N+1>` after deriving, from the facts and an
explicit observation time, that the incumbent chain is stale or released. The
core enforces none of that ordering, because checking liveness at append time
would be the core evaluating expiry; a misbehaving driver could seize early,
which is the authorised-caller trust gap Q9 already deferred with its own
revisit trigger, and `cairn coord verify` makes a premature seizure visible
after the fact (two overlapping live chains).

The honest caveat, stated rather than buried: O_EXCL is unreliable on network
filesystems, and this design assumes a local checkout family.

### Cross-clone

Declared divergence. Q9 says that across independent clones "repo-synced facts
cannot mutually exclude" and promises detection after sync. That paragraph
presumes the store is repo-synced. This store is family-local and untracked, so
there is no sync and therefore nothing to detect.

**Refining ruling 2**: cross-clone coordination is out of scope for format 1,
and the singleton grant is scoped to a checkout family, which is exactly the
case Q9 ratifies as atomic and enforced. Two clones are two coordination
domains. This is more honest than specifying an export and import verb that
nobody will remember to run, and strictly better than promising a detection with
no mechanism behind it. The revisit trigger is first observed multi-clone driver
use, and Q9's own deferral of true cross-device exclusion carries forward
unchanged.

**Nothing is foreclosed.** Facts are self-contained JSON with a `format` field,
and a reader that meets an unknown format fails closed, following the
`read_versioned_json` discipline in `src/persist.rs`. If a synced store is ever
sanctioned, migration is a replay: fold the append-only facts into the new
store in filename order. The family-local choice defers cross-clone
coordination; it does not spend it.

### The record schema

One envelope, four fact families.

```json
{ "format": 1,
  "fact_id": "<12 hex of sha256 over the canonical body>",
  "kind": "ruling.run",
  "recorded_at": "2026-08-07T03:45:12Z",
  "recorded_by": { "kind": "maintainer|driver|console", "id": "<string>" },
  "commit": "<40 hex at recording>",
  "evidence_class": "deterministic|attested|observed",
  "supersedes": "<fact_id or null>",
  "payload": { }
}
```

Ruling payloads carry the clause 2 contract field for field: the ruling type is
`kind`, the target is `payload.target` (a todo id, or the plan digest for run),
the acting maintainer is `recorded_by`, `recorded_at` and `commit` are envelope
fields, and the type's payload is `payload`. `ruling.park` carries no status:
it is a deferral fact the ready-set projection subtracts, and the todo keeps
whatever status it had.

Lease payloads:

```json
{ "unit_id": "todo.driver-in-repo",
  "holder": { "harness_kind": "omp", "session": "<id>" },
  "commit_at_grant": "<40 hex>",
  "granted_at": "<RFC3339>",
  "expires_at": "<RFC3339>",
  "epoch": 3,
  "residue": { "branch": "loop/driver-in-repo", "worktree": "../cairn-wt/driver", "pr": null } }
```

`residue` is seeded at grant with the branch and worktree the driver is about to
create, because those are the two facts it knows then, and Q2 requires a stale
claim to carry its recoverable residue. `pr` is null at grant and arrives on a
later `lease.renew` or on the unit's outcome fact; the reader joins the
supersedes chain to render the current residue. Without this the stale
projection has nothing to show, which is the state
`studio/mocks/orchestration-return-orient.html` renders for `r-041`.

Renewal is a `lease.renew` fact with `supersedes` naming the prior lease fact
and a new `expires_at`. Nothing is ever edited. The reader folds the chain.

Append-only is enforced structurally (files are content-named and written once)
and checked by `cairn coord verify`, which asserts the fact set is a superset of
every prior observation and that no `supersedes` chain has a missing antecedent.

### The console barrier

The appender refuses `lease.*` and `driver.singleton.*` when
`recorded_by.kind == "console"`. `dec.orchestration-placement` clause 4 says the
console never acquires or renews a lease; this makes that a checked invariant on
the one shared write path rather than a convention nobody enforces.

### The read surface

```
cairn ruling list [--since <cursor>] [--at <RFC3339>] [--json]
cairn ruling show <fact-id> [--json]
cairn lease  list [--at <RFC3339>] [--json]
cairn wave   [--at <RFC3339>] [--json]          # the dispatch preview and its plan digest
cairn wave   stats [--since <date>] [--json]    # the promotion ratio, Part 3
cairn coord  verify | compact --before <date>
```

The response stamps `schema_version` and returns **raw** facts. It contains no
`active`, no `expired`, no `stale`, and no `status`. It echoes `observed_at`
back exactly as the caller supplied it, and when the caller supplies none it
echoes `null` and consults no clock. The core evaluates no expiry, and that is
the whole of `dec.orchestration-placement` clause 3.

For the same reason there is no `--unresolved` flag on the wire. Deciding
whether a `ruling.run` is unresolved means joining it against
`outcome.run_consumed` and `outcome.run_declined`, and whether a `ruling.park`
is live means joining it against `ruling.unpark`. Clause 1 would permit those as
side-effect-free projections, but keeping the wire raw means the console and the
driver share one set of reader predicates instead of trusting a server-side
one, so the filter lives in the renderer.

The response also carries `store_state: "uninitialised" | "ready"` (a read never
creates the store), `cursor`, `truncated`, and a first-class `conflicts: []`. A
read that cannot fully resolve the store **fails closed** rather than returning a
short list: the driver must never dispatch against a partially read store, and
the console must never render a wave computed from one.

The handler lands in `src/query_api/handlers/coordination.rs`, registered in
`TOOL_REGISTRY` and dispatched through `execute_data_with_scan` like every other
read.

### The reader predicates, named once

Both consumers implement these three and no others:

- `held(unit, at)`: a `lease.grant` or `lease.renew` folded chain with no
  `lease.release`, and `expires_at > at`.
- `stale(unit, at)`: the same chain with `expires_at <= at`. Renders holder,
  `expires_at`, and residue. This is a first-class state.
- `no_lease(unit)`: no chain at all. This is a different state and a different
  sentence.

### Driver presence, which the mocks require

The console must distinguish no driver attached, attached but not yet read,
attached and acting, and crashed, and it must do so from raw facts.

The singleton grant is the attachment signal, and the driver renews it at
session scale exactly as it renews a lease. Each `driver.singleton.renew`
carries `read_at`, the observation time of the driver's last complete fold, and
`folded_count`. That is one fact per session-scale renewal, not a heartbeat
stream, which Q2 forbids. `read_at` is a timestamp rather than a filename
cursor, for the ordering reason above.

| Reader-derived state | Predicate |
|---|---|
| no driver attached | no singleton chain, or the latest has a release |
| attached, has not read this ruling | grant live at `at`, and the ruling's `recorded_at` >= `read_at` |
| attached and acting | grant live at `at`, and the ruling's `recorded_at` < `read_at` |
| crashed or gone | grant chain live, `expires_at <= at`, residue rendered |

The console's plain-register line follows directly: "Recorded at 14:02. The
driver last read at 13:58." No countdown, no simulation. Equal timestamps render
the conservative side, "has not read this yet".

### Retention

`cairn coord compact --before <date>` is a maintainer verb, never automatic. It
moves facts into `archive/<yyyy-mm>/` and deletes nothing, so the history
channel `dec.webui-write-authority` clause 2 requires can always be rendered by
reading archives.

Two facts are never compactable, and `verify` checks both: a `ruling.park` with
no matching `unpark`, because the ready-set projection honours it indefinitely
and compacting it would silently change readiness; and any fact that is the
antecedent of a live `supersedes` chain.

### Read cost, and the derived cache

One file per fact means a full fold is O(N) directory entries and JSON parses.
At swarm scale that is a real cost on every driver poll and every console
hydration.

The saving is safe only because facts are immutable and one per file, which
makes filename an exact content key. **Every read lists `facts/` in full**, then
parses only the names it has not already parsed. `cache/parsed.json` holds
parsed bodies keyed by filename. The parse cost is O(new facts), the same saving
a tail fold would have given, and no ordering assumption appears anywhere. The
cache is rebuilt from the facts at any time and may be deleted at any time
without loss, because a full listing is always the ground truth.

This is precisely what rung 3's own definition calls for: a shared derived index
where **canonicity never moves, and the index is derived and disposable**. It is
the successor to `res.overharness-design-threads` thread c, and the facts
directory is the canonical content the thread says stays git-adjacent while the
derived store widens without gaining authority.

#### Implementation reconciliation (2026-08-09)

The S8 hardening round removed the parsed-envelope cache rather than carrying
this optimization into the shipped reader. Full reads still list `facts/` in
full and now parse every immutable fact directly; `cache/observed.json` remains
only the append-only verification snapshot. The canonical fact bytes remain the
sole read authority.

### The derived-fact metadata slate: adopt, not defer

`evidence_class` is a **required** envelope field from format 1. The slate this
pre-adopts is named so the eventual ruling sees the dependency: the unratified
candidate recorded in `res.inversion-convergence-minutes`' post-ratification
intake and carried in `todo.parallel-dispatch-granularity` ("every derived fact
carries source, extractor plus version, observed_at, freshness, and
completeness; deterministic, attested, and observed are distinct evidence
classes and never blur"). Retrofitting a required field into an append-only
store is structurally hard on purpose, and the console needs it to caveat an
observed harness outcome differently from an attested maintainer ruling in the
same history channel. Requiring now is also the cheap-to-reverse direction:
relaxing a required field later is trivial, while requiring a missing one later
strands every existing fact.

The class is fixed per fact kind, so writers cannot choose inconsistently:

| Fact kind | evidence_class | Why |
|---|---|---|
| `ruling.*` | attested | a maintainer states their own ruling |
| `lease.*`, `driver.singleton.*` | attested | the driver states its own action |
| `outcome.run_declined`, `outcome.run_consumed`, `outcome.unit` | attested | the driver states its own classification |
| `outcome.touched_files` | observed | derived from a merge diff, external evidence |
| `write_set_derivation` payload blocks | deterministic | a pure function of committed state |

A reader that meets an unknown class fails closed, the same rule as an unknown
`format`. `source`, `extractor` with version, `observed_at`, `freshness`, and
`completeness` are carried inside derived payloads only, not on every fact.
Part 3 makes `completeness` load-bearing rather than decorative.

**Refining ruling 4** adopts this ahead of the slate's own ruling.

## Part 3: write-set derivation and the disjointness test

### Derivation

From `Todo.node` (a scalar, 1:1 today) take the **containment closure**: the
node plus its descendants via `NodeRecord.children`.

Dependency edges are deliberately excluded. They are rung 1 Order, and folding
them into the write-set collapses every wave to a single unit on a graph as
densely connected as the dogfood blueprint. This is an interpretive reading of
the ratified phrase "node-closure over committed state" and is flagged as such
rather than presented as settled; the maintainer may read it the other way, and
the cost of the other reading is stated here so the choice is informed.

Map the closed node set to a file set through `Node.paths`, then subtract paths
owned more specifically by a node outside the closure.

### What derived-first actually buys, measured on this corpus

The promotion trigger below is written as though the evidence is pending. Part
of it is not: the anchor distribution is measurable today, and the derived-first
ruling should be read with the number rather than discovering it after twenty
exclusions.

Measured 2026-08-07 over `meta/todos/` (191 todos, 39 of them `open`):

| Anchor | Open units | Derived write-set (own paths) |
|---|---|---|
| `cairn.root` | 16 | seven files: `src/main.rs`, `lib.rs`, `error.rs`, `verification.rs`, `signal.rs`, `copy.rs`, `report.rs` |
| `cairn.brownfield` | 5 | `src/brownfield` |
| `cairn.kernel.scanner` | 4 | `src/scanner` |
| `cairn.ui` | 3 | `src/ui`, `src/ui_assets` |
| `cairn.kernel.hooks` | 3 | `src/hooks` |
| `cairn.kernel.artefacts` | 2 | `src/artefacts` |
| `cairn.kernel.cli` | 2 | `src/cli`, `tools/agent-pack` |
| `cairn.kernel.map` | 2 | `src/map` |
| `cairn.kernel.query` | 1 | `src/query_api` |
| `cairn.reconcile` | 1 | `src/reconcile` |

Two facts follow, and they point in opposite directions.

The good one: all ten open anchors have **pairwise-disjoint** paths, so the
ceiling on wave width from today's open backlog is **ten**, not one. Derived
node closure over a flat module graph is a usable partition, and the worry that
derived-first degenerates to serial execution is wrong for containment closure.
It is true only for the dependency-edge reading, which is why that reading is
excluded above.

The bad one: **16 of 39 open units, 41 percent, share `cairn.root`**, so at most
one of them enters any wave and the other fifteen queue. `cairn.kernel.cli`
carries 40 units across all statuses on the same one-anchor-one-write-set
footing. The ceiling of ten is a ceiling; the realistic first wave is nearer five
or six once rung 1 order and hotspot serialisation apply.

The `cairn.root` pile is also the sharpest promotion signal available, and it is
a different defect from the one the ratio measures. `cairn.root` is a Module
owning seven specific files, but it is used as the catch-all anchor for
repository-wide and process work. `todo.parallel-dispatch-granularity` itself is
anchored there and touches none of those seven files. So for a large minority of
units the derived write-set is simultaneously **too coarse** (they block each
other over files none of them will edit) and **too narrow** (it misses what they
actually touch). Both errors are invisible to the disjointness predicate.

Two consequences. First, the promotion trigger watches `cairn.root`-anchored
units specifically rather than only the global ratio: that is where declared
write-sets pay off first, and where the derived phase is least honest. Second, a
cheaper intervention may beat promotion entirely and costs no schema change:
re-anchoring catch-all `cairn.root` todos onto the nodes they really touch is
ordinary graph hygiene, and it raises wave width without adding a per-unit
authoring contract. Filed as `todo.root-anchor-hygiene`; not a prerequisite for
anything here.

### Disjointness

Component-boundary prefix overlap. Two write-sets are disjoint when no prefix in
one equals, or is a component-boundary prefix of, any prefix in the other. The
worked counterexample belongs in the implementation's test: `src/ui` must not
match `src/ui_assets`.

The existing check at `src/reconcile/generic.rs:410` already has the right
shape, and `trim_dot` at `:419` already normalises the leading `./`. Both are
private `fn`, and the check requires the prefix to carry no trailing slash. The
implementation extracts one `pub(crate)` overlap helper used by
`most_specific_owner` and by the disjointness test, so there is a single
implementation of path containment in the codebase rather than two that drift.

### Fail closed, and visibly

An unresolvable anchor, an `owns_files: true` node with no declared paths, or a
graph snapshot whose commit differs from HEAD yields the universal prefix `.`
with `resolution: "unresolved"` and an `unresolved_reason`. The unit dispatches
alone rather than vanishing from the preview, and the console has a sentence:
"todo.X runs alone: its write-set could not be derived (reason), so cairn treats
it as touching every file."

### The hotspot problem, resolved honestly

`docs/registries/`, `cairn.blueprint`, and `docs/design-system/copy.toml` are
declared by no node path in `cairn.blueprint` today. Only the wire snapshots are
covered, through `cairn.kernel.query` owning `./src/query_api`. There are two
separate moves here and conflating them is how earlier drafts went wrong.

**First**, declare blueprint nodes that own those paths. This uses the existing
`path` and `owns_files` keywords, needs no grammar change, is a one-off
repository act rather than a per-unit burden, and repairs a real gap: those
files are invisible to ownership entirely, which is a latent defect independent
of dispatch.

**Second, and this is the part no candidate design got right at first:
declaring the nodes does not attribute the hotspot to a unit.** A unit anchored
at `cairn.kernel.scanner` that appends an error code to
`docs/registries/error-codes.md` has no registry node in its containment
closure, and no amount of graph work will put one there, because the fact "this
unit will touch that file" does not exist in committed state at composition
time. Node closure is provably blind here, and the design says so rather than
asserting a derivation it cannot perform.

The resolution is three-phase, which is the phasing Q1 already ratified.

**Phase 0, deterministic and blind. This is now.** Hotspot paths are in no
unit's derived write-set. Every derived write-set is stamped
`completeness: "partial"` with a `completeness_reason` naming the uncovered
hotspot prefixes, so a recorded plan always carries the honest limit of its own
evidence.

Contention is resolved by policy rather than by the graph. The inert workflow
artefact carries a `serialises:` list of **path prefixes** (not node ids: node
ids cannot name a path no node owns, which is the whole problem). Cairn
validates that each prefix exists and evaluates nothing further. The driver
grants the hotspot permission to exactly one unit per wave in deterministic
order (topological rank, then todo id ascending); every other unit in the wave
dispatches with a stated constraint that it must not edit those prefixes.

Stated plainly, because a reviewer will otherwise assume it: in phase 0 the
driver **cannot predict which units will touch a hotspot**, and it does not
try. That prediction is exactly the per-unit fact committed state lacks, which
is why the permission is exclusive rather than targeted: one unit per wave may
touch the hotspot prefixes, every other unit is constrained not to, and a
constrained unit that turns out to need a hotspot stops and records a blocked
outcome rather than editing it. Those blocked outcomes are themselves promotion
evidence.

This reuses ratified machinery. Q4 already made workflows inert typed artefacts
with closed vocabularies that cairn parses, validates, stores, and never
evaluates, and the driver alone evaluates them. It adds no blueprint grammar and
no per-unit authoring, so Q1's zero-new-authoring-burden constraint holds.

**Phase 1, observed.** At merge the driver records `outcome.touched_files` with
`evidence_class: "observed"`, from `git diff --name-only <commit_at_grant>..<merge>`.
That is a genuinely new driver-side git read which does not exist today, and it
is named as a cost rather than assumed. These facts answer, per anchor node, how
often a unit really touched a hotspot and how often a unit was excluded from a
wave for an overlap its own merge diff proves never happened.

**Phase 2, declared.** On a maintainer refining decision, todos gain a declared
`writes:` field. Only then can the preview name specific hotspot files per unit.

### The promotion trigger, measured rather than asserted

A reader-side projection over append-only facts, with no mutable counter
anywhere:

```
false_overlap_rate := |{ excluded units whose outcome.touched_files proves no real overlap }|
                    / |{ excluded units with a recorded outcome.touched_files }|
```

over a rolling window of the last twenty exclusions that have merge evidence.
`cairn wave stats --since <date>` renders it, and a maintainer can watch it
approach a threshold rather than being told after the fact.

The threshold is deliberately **not** invented here. Every candidate design
produced one (0.5, 60 percent, three in thirty days) and none had an evidence
base for it; the first window of real data sets it, in the refining decision
that promotes. The trip is advisory: it recommends a refining decision and
promotes nothing, because promotion changes an authoring contract and only the
maintainer signs that.

One named bias, stated so the number is read correctly: an abandoned branch
never merges, so it never records `touched_files`, which shrinks the denominator
and biases the measurement against promotion.

### What rung 3 closes, and what stays open

Rung 2, Advisory overlap, cannot see shared files, registries, blueprint edits,
generated assets, or unpushed claims.

Rung 3, Merge-safety, closes three of those: **shared files**, through derived
file-level write-sets rather than node adjacency; **registries and blueprint
edits**, through phase 0 hotspot serialisation by policy; and **unpushed
claims**, through lease facts in a store visible across the checkout family
before anything is pushed.

It leaves open: **generated assets**, and **unpushed claims across independent
clones** (refining ruling 2).

Rung 1, Order, is unchanged and remains the source of the wave.

### What rung 3 explicitly does not promise

Disjoint write-sets do not guarantee a clean merge. `Cargo.lock`, `map.json`,
formatter output, and wire snapshots are regenerated by tooling and can collide
between two units whose file sets never touched. Rung 3 promises that two units
in a wave do not edit the same source paths. It promises nothing about generated
artefacts, and it offers repair rather than prevention.

The repair path, stated as a human procedure because a maintainer will need it:
the driver records `outcome.unit` with a `merge_conflict` class and records
`lease.release`, and the unit enters **quarantine**. It does not re-enter the
ready set on its own. `dec.webui-write-authority` clause 3 already rules that
quarantine is never auto-released and gives the maintainer the verb:
`cairn ruling release <todo-id>` returns it to the ready set once the conflict is
resolved. The console renders the quarantined unit with its branch, its
worktree, and the conflicting paths from the outcome fact.

### What the preview renders, derived against recorded

| Rendered line | Class |
|---|---|
| `Next wave · 2 units` | derived |
| `write-sets disjoint · parallel worktrees` | derived |
| `wf.default 3: ready · contract present · write-sets disjoint` | derived, quoting the inert workflow |
| `worktree ../cairn-wt/driver · lease on grant` | derived; "on grant" is a promise, not yet a fact |
| the hotspot queueing sentence | derived from the workflow `serialises:` prefixes and the permission grant, never from node closure |
| "it queues behind that claim" | RECORDED only when a lease fact exists; otherwise "queues behind that unit" |

Two corrections to the accepted mock copy fall out, and both are honest
downgrades rather than losses.

The mock renders "todo.lease-read-surface waits for this wave: it would change
`docs/registries/declared-items.md`, and so would `todo.driver-in-repo`. Same
files, one at a time." In phase 0 the first clause asserts per-unit knowledge
the derivation does not have. The renderable phase-0 sentence keeps the plain
register and the load-bearing half: "todo.lease-read-surface waits for this
wave: only one unit at a time may change `docs/registries/`, and
todo.driver-in-repo holds that permission. Same files, one at a time." The
mock's original sentence becomes true in phase 2, when declared write-sets land.

The word "claim" may only be rendered when a lease fact with a holder and an
expiry exists. Otherwise the console is asserting a claim nobody granted.

**Refining ruling 5** covers both corrections.

## Appendix: verified repository claims

| Claim | Verification |
|---|---|
| `.cairn/` is entirely gitignored and nothing under it is tracked | `.gitignore:43`; `git ls-files .cairn` is empty |
| `--git-path` resolves per-worktree, `--git-common-dir` does not | run in `/Users/george/repos/cairn-vibe-edit`, output quoted above |
| `most_specific_owner` and `trim_dot` are private and reject trailing-slash prefixes | `src/reconcile/generic.rs:405-421` |
| Cairn has no git library and writes no refs | `Cargo.toml`; all git use is `std::process::Command` in `hook.rs`, `hooks/architecture.rs`, `hooks/ratification/git.rs` |
| `Todo.node` is a scalar `String` | `src/artefacts/registry/types.rs:76-101` |
| The three hotspot paths are owned by no node | `cairn.blueprint:13-139` |
| `persist::atomic_write` is temp-file plus rename | `src/persist.rs:38-58` |
| `cairn feedback` appends unlocked | `src/cli/commands/feedback.rs:51-56` |
