---
id: dec.beads-task-layer
nodes:
  - cairn.root
status: accepted
date: 2026-06-23
informed_by: [res.gas-city-cairn-integration]
related: [dec.no-orchestrator, dec.bead-github-sync]
revisit_triggers:
  - "A maintainer wants node-linked beads to become a first-class Todo artefact *source* (which would require ratifying a spec.md:11 / §8.2 amendment first, since shipped StateBackend keeps artefact content in files unconditionally)"
  - "bd drops the passive `.beads/issues.jsonl` export or the `cairn-node:<id>` label convention the view depends on"
  - "cairn adopts a non-beads task tracker, making the beads-derived view the wrong source"
---

# Beads as cairn's per-node task layer: a read-only derived view, not a Todo source

## Context

`cairn-2z9` asked how to close the gap between cairn's two disconnected task
worlds. Cairn HAS a first-class Todo artefact type (markdown under `meta/todos/`,
surfaced per node in the webui Todos panel), but this repo does not use it: task
tracking lives in beads (`bd`), and beads are not in the cairn graph, so the
per-node inspector and the webui Todos panel are empty. The proposal is to make
node-linked beads visible per node so a node's related tasks (with bd workflow
state) appear in the inspector.

Three accepted decisions, one shipped feature, and the shipped storage boundary
bound the question before any design work:

- **`dec.bead-github-sync` (accepted).** Rejects a second source of truth and the
  "two writers, one truth" divergence trap. Any task-layer design must keep a
  single canonical store.
- **The reader already exists.** `src/state/backlog.rs` (cairn.state, PR #140)
  parses the passive `.beads/issues.jsonl` export into `BacklogItem`, is strictly
  read-only ("Beads remains the single source of truth"), and already implements
  the node link: `BacklogItem::linked_node()` strips a `cairn-node:<id>` label and
  returns the bound node. Unlinked beads return `None`. PR #140 already surfaces
  the bead-to-node link via CLI (`cairn get`, `cairn next`); the missing piece is
  the inverse per-node grouping in the webui inspector.
- **The shipped storage boundary keeps content in files.** `src/state/mod.rs:3-5`:
  the `StateBackend` enum "abstracts artefact *state* storage (status, claim,
  ready-queries) from the filesystem default. Content (markdown bodies, blueprint
  text) stays as files unconditionally." This decision's own informing research
  records the refocus that produced that boundary: `#97` "refocused from
  `ArtefactStore` to `StateBackend` (state only; content stays as files)" and
  `#99` "refocused from Beads as content store to Beads as state backend"
  (`meta/research/gas-city-cairn-integration/README.md:61-62`). There is no
  shipped content-source pluggability, and `ArtefactStore` does not exist in the
  source tree.
- **`cairn-87n` (`CAIRN_TEST_COVERAGE_MISSING`, shipped; closed `cairn-a8z`).**
  Cairn's coverage gate keys off node *reconciliation* state (ghost/synced), never
  bead workflow state. Enforcement is reality-based and independent of any task
  surface.

The Todo consuming chain is markdown-bound at exactly one stage. `load_todos`
(`src/artefacts/registry/mod.rs:44`) reads markdown into `Vec<Todo>`; everything
downstream (query_api serialise -> artefacts handler -> UI server -> webui Todos
panel) consumes that vec.

## Decision

**Surface node-linked beads as a read-only, derived navigational view, not as
cairn Todo artefacts. Reject the export bridge.** Four rulings follow. This spike
rules the design only; no implementation lands here (per the bead's acceptance
criteria).

1. **Native read-only view over export bridge; build is a follow-up.** The
   sanctioned shape is a thin read-only projection over the existing
   `.beads/issues.jsonl` reader, filtered by the `cairn-node:<id>` label that
   `backlog.rs::linked_node()` already parses, rendered as a per-node view in the
   webui inspector. The export bridge (generating `meta/todos/<node>/*.md` from
   beads) is rejected: it creates a second on-disk representation, a sync step,
   and a staleness window, the exact divergence trap `dec.bead-github-sync`
   rejects. Building the view is a follow-up unit; this ruling fixes only its
   shape.

2. **Do not make beads a Todo artefact *source*; keep the view separate from the
   Todo type.** Sourcing a Todo's body, created date, or satisfies clause from a
   bead would make beads a content store, which contradicts the shipped boundary
   ("content stays as files unconditionally", `src/state/mod.rs:4-5`) and the
   `#97`/`#99` refocus away from "Beads as content store". So the beads view is
   **derived navigation, not a stored artefact**: it does not populate the `Todo`
   struct, does not write to `meta/todos/`, and does not redefine the Todo type.
   Crucially, **this keeps spec.md:11 ("typed markdown files attached to nodes")
   and §8.2 Todo intact**: no spec invariant is bent and no spec amendment is
   needed. Making node-linked beads a genuine Todo *source* would require a
   maintainer-ratified spec.md:11 / §8.2 amendment first (recorded as a revisit
   trigger); that path is explicitly out of scope here.

3. **Tasks remain navigation, never enforcement (declared-not-verified
   preserved).** A bead's status is a navigational claim cairn surfaces, not a
   reconciled fact. Cairn must never gate on bead status. Enforcement stays
   reality-based: `cairn-87n`'s coverage gate keys off reconciliation state, so
   surfacing bead status changes nothing cairn enforces. This supersedes the
   `cairn-2z9` framing that bead workflow-state would "ride" the TDD gate: the
   shipped `cairn-87n` design makes the gate independent of bead state.

4. **Single source of truth, no second projection.** Beads (local Dolt;
   `.beads/issues.jsonl` as the git-tracked reality layer) stays canonical. This
   ruling stays inside the `dec.bead-github-sync` boundary: no GitHub projection,
   no markdown mirror, no bidirectional sync. The view reads; it never writes.

## Answers to the spike questions

1. **Export bridge vs native loader, and why.** Native read-only view. The reader
   and the `cairn-node:<id>` link already exist in `backlog.rs`; the bridge adds a
   second representation and a sync/staleness surface that the single-source-of-
   truth invariant rules out.

2. **Is a non-markdown artefact source acceptable (spec.md:11)?** No, not as a
   Todo content source. spec.md:11 defines artefacts as "typed markdown files
   attached to nodes", and the shipped `StateBackend` keeps artefact content in
   files unconditionally (state only is pluggable). So beads are surfaced as a
   derived read-only view that does not redefine the Todo type and touches no
   invariant. spec.md:11 is unchanged. (A genuine beads-as-Todo-source would need
   a maintainer-ratified spec amendment first; out of scope.)

3. **Node-link convention, status display, field rendering.**
   - Link label: `cairn-node:<id>` (already implemented in
     `BacklogItem::linked_node`). A bead without the label is unlinked: not
     surfaced per node, not an error.
   - The view renders bd state directly (the bead's `status`, `priority`, `title`,
     `id`); it does not map into `TodoStatus`, because it is not a Todo. For
     reference, the bd-status to cairn-`TodoStatus`
     (`src/artefacts/registry/types.rs:62`) correspondence, were a maintainer to
     later promote beads to a Todo source, would be: `open`->`Open`,
     `in_progress`->`InProgress`, `closed`->`Done`, `blocked`->`Blocked`,
     `deferred`->`Blocked` (no `Deferred` variant). That mapping is recorded for
     the future path only; the read-only view itself displays bd state verbatim.

4. **Integrity rule for orphan task-beads.** Mirror spec.md:339: a
   `cairn-node:<id>` label that resolves to a deleted or unknown node is an orphan
   -> **warning** (informational, non-blocking), matching markdown-todo orphans.
   A bead with no `cairn-node:` label is simply unlinked (not an error), matching
   `backlog.rs` returning `None`.

5. **Tasks remain navigation/context, not enforcement.** Confirmed (ruling 3).
   Declared-not-verified is preserved; bead status is never reconciled fact.

6. **Relationship to `cairn-y1m` and `cairn-a8z`.**
   - `cairn-y1m` (beads<->GitHub label sync): CLOSED, resolved by
     `dec.bead-github-sync` (defer; no second source of truth). This ruling stays
     inside that boundary: jsonl-in-git remains the single source; no GitHub
     projection.
   - `cairn-a8z` (TDD/coverage gate): CLOSED, resolved by `cairn-87n`. Its gate
     keys off reconciliation state, not bead status, so the task layer does not
     carry or "ride" bead workflow-state for enforcement. The task layer is purely
     navigational.

## Implementation (follow-up bead, out of scope here)

A single small unit: add a read-only per-node beads view derived from
`backlog.rs` (group `BacklogItem`s by `linked_node()`, expose via `query_api`,
render in the webui per-node inspector), emit the orphan warning, and render bd
state verbatim. No change to `load_todos`, no `Todo` artefacts minted, no
`meta/todos/` files, and no spec.md:11 / §8.2 edit. If a maintainer later wants
beads promoted to a genuine Todo source, that is a separate change gated on a
ratified spec amendment.

## Risks

- **jsonl staleness vs canonical Dolt.** The git-tracked export can lag the local
  Dolt DB. Accepted: this is the same reality layer the dev loop already reads
  (PR #140), and the read-only view never writes a competing copy. Reconciliation
  is the existing `bd export -o` discipline, not a cairn concern.
- **View vs Todo confusion.** A per-node beads view sitting beside an (unused)
  Todo panel could read as two task surfaces. Mitigated by this repo declaring no
  `todos` pointers (the Todo panel is empty) and by the view being labelled as a
  beads view, not todos.

## Consequences

- `cairn-2z9` is satisfied and can be closed: the spike's deliverable (this
  ruling) exists. Implementation moves to a follow-up bead.
- The per-node inspector can gain a sanctioned, single-source, read-only beads
  view without a generator, a second source of truth, or any spec amendment.
- spec.md:11 and §8.2 are untouched; promoting beads to a genuine Todo source
  remains a maintainer-ratified, separate decision.
