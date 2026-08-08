---
node: cairn.kernel.query
status: open
created: 2026-07-11
---

# Query: node-overlap view for concurrent work

This todo was re-scoped on 2026-08-09 against the shipped rung-three
coordination substrate. It remains an advisory view that answers "who else has
in-flight work intersecting node X". It is not merge safety, dispatch
authorization, or a replacement for the driver's write-set and lease policy.

## Scope

Given a node ID `X` and an explicit observation instant
`--at <RFC3339>`, compute the one-hop set with
`src/query_api/serialise.rs::neighbourhood_ids`: `X` plus the nodes on its
inbound and outbound edges. Return every matching unit and change with its
stable ID, anchored node or referenced node, evidence layer, and reason for
the match.

### Committed todo/change baseline (working-tree read)

- Read the current working-tree view of the `meta/todos/todo.<slug>.md`
  artefacts through `src/scanner/mod.rs::load_project`, already present in
  `scan_result.artefacts.todos`. Include `open` and `in_progress` todos whose
  `node` is in the one-hop set. Exclude `done` and `blocked` todos from the
  in-flight list, while retaining their IDs only when needed to explain a
  relationship.
- Discover active change directories through
  `src/changes/mod.rs::discover` using the resolved `--changes-dir` (default
  `meta/changes`). These loaders read the working tree, not git HEAD. Use
  `operations_for_nodes` to include deltas and artefact references that mention
  a node in the one-hop set. This is the committed todo/change baseline as
  loaded from the working tree, not a git HEAD snapshot; it may include
  uncommitted edits and may lag another worktree.
- The `open`/`in_progress` filter is new for the coordination section only.
  Keep the existing `cairn neighbourhood --include-todos` listing unchanged,
  including its current status coverage.

### Live coordination

- The shipped lease read verb is `cairn lease list`, routed by
  `src/cli/commands/coord.rs`. It is the raw lease and driver-singleton
  source; `cairn coord verify|compact` remains store administration. Its
  `--json`, `--at <RFC3339>`, and `--since <filename>` forms are already
  wired by `coordination_query` and contracted by `todo.coord-read-surface`.
  The current help-spec/validator mismatch is a grammar defect to repair, not
  a new substrate flag: expose these shipped forms in help, copy usage, and
  validation before relying on them from this query.
- Read the family-local store through
  `src/coord/read.rs::read_facts`. Lease `payload.unit_id` is already the full
  todo stem, such as `todo.driver-in-repo`; join it exactly to the stem
  returned by `compose_wave::todo_stem` from `todo.<slug>.md`. Do not prepend
  another `todo.`. Use the shared reader predicates `lease_chain_head`,
  `held`, `stale`, and `no_lease` at the supplied observation instant. A
  lease is held on a dispatch unit, never directly on a node. Preserve the
  lease fact ID, holder, expiry, and residue in the result. Do not infer
  `active`, `expired`, or `stale` from the raw lease wire, and do not classify
  from a paginated `--since` subset.
- Obtain one facts snapshot and use it for both the lease join and wave
  composition. `src/query_api/wave/compose.rs::compose_wave` currently calls
  `read_facts` internally, so the future unit must thread the single snapshot
  through a facts-taking composer variant or equivalent. Two independent reads
  must not disagree when a grant lands between them.
- Include the matching `units` and `held` rows from the shared composer behind
  the current `cairn wave` command. `src/cli/render/project.rs::render_wave`
  already passes `--at <RFC3339>` into `wave_json` and `compose_wave`, and the
  command already has a JSON rendering path. If `FLAGS_HELP_ONLY` and
  `validate_command_flags` block that shipped observation form, repair the
  help spec, copy usage, and validation rather than redesigning the substrate.
  Preserve the plan digest, unit IDs, write-set prefixes, and held reasons
  (`lease-held`, `write-sets-overlap`, `runs-alone`, and `parked`) with any
  blocking fact ID. A wave preview is live coordination context, not proof
  that a unit has started; only a lease chain classified as held at `--at`
  may be rendered as a current claim. `cairn ruling run <plan-digest>`
  records consent and is not itself an in-flight claim.
- Require `--at <RFC3339>` for this view's coordination rows. Missing `--at`
  is a usage or query error, never an implicit "now". The reader predicates
  compare single-format UTC strings lexically, so reject or normalize non-UTC
  and fractional inputs before calling them. Only valid envelopes produce
  per-lease rows: `held`, `stale`, `no_lease`, or `release-head`. A malformed
  lease fact, or a malformed or missing `expires_at`, is the store-level
  failed-read and fail-closed outcome defined by
  `todo.coord-fact-store-hardening`, never a no-lease or stale row.
- An uninitialised store produces an explicit empty coordination row with
  `store_state` and a reason, not silent admission of every unit. A malformed
  or partially unreadable store fails closed.

The old `StateBackend` seam is dropped. `dec.change-format-only` deleted the
generic persistence abstraction and the beads claim and sequence workflow, so
this read-only view must not revive either. It composes the scanner's
working-tree artefacts with one `read_facts` snapshot and `compose_wave`; it
writes no facts and performs no claiming, sequencing, or dispatch. The core
and appender do not evaluate expiry; only the reader predicates derive
`held`, `stale`, and `no_lease` from the required explicit `--at`.
Coordination reads fail closed when the family store cannot be fully resolved.

## CLI surface

Fold this view into the existing node-scoped
`cairn neighbourhood <node>` query, rather than adding a `cairn conflicts`
top-level noun or extending project-wide `cairn status`. `neighbourhood`
already owns the one-hop graph scope and the shipped opt-in
`--include-todos`/`--include-changes` artefact sections. Add
`--include-coordination` and `--at <RFC3339>` as new accepted flags for the
coordination section. Keep `--json` as the machine-readable form and make the
human sections mirror the same baseline and live rows.

Only `--include-coordination` and `--at <RFC3339>` on `neighbourhood` are
new flags for this unit. The shipped lease and wave observation forms already
exist in `coordination_query` and `render_wave`; the `FLAGS_HELP_ONLY` entries
and `validate_command_flags` mismatch is a help-grammar defect. Repair those
help specs, `docs/design-system/copy.toml` usage and argument copy, and
validation so the existing forms are discoverable and accepted. Add the new
neighbourhood flags through `src/cli/commands/mod.rs::shared_flags` and
`shared_request` and the relevant query plumbing. The new coordination
section must extend the existing `NeighbourhoodResponse` wire schema and its
snapshots without changing the existing neighbourhood todo listing.
No new command row is expected, but
`tests/command_reference_consistency.rs` and the query wire contract tests
must remain green.

The implementation surfaces are the existing `neighbourhood` registry and
dispatch in `src/query_api/registry.rs` and `src/query_api/mod.rs`,
`src/query_api/handlers/graph.rs::neighbourhood_json`,
`src/cli/render/node.rs::render_neighbourhood`, and the request flag/schema
mapping in `src/mcp/mod.rs`. Update `docs/commands.md` and
`docs/integration-contract.md` for the new opt-in section and wire fields.

## Acceptance

- A working-tree fixture graph with target `X`, two open todos on adjacent
  nodes, and an active change referencing one adjacent node returns exactly
  the matching baseline todos and change operations, while excluding
  unrelated nodes. It also proves that the new `open`/`in_progress` filter
  does not alter the existing `--include-todos` neighbourhood listing.
- The status matrix is explicit: `open` and `in_progress` are included in
  coordination rows, `done` and `blocked` are excluded, and legacy
  `cairn neighbourhood --include-todos` remains an all-status listing.
- The same fixture has lease facts for one todo and a wave containing both
  admitted and held units. With the future accepted `--at <RFC3339>` flag,
  the response joins the lease holder, expiry, residue, fact ID, wave plan
  digest, unit write-sets, and held reasons to the matching node IDs. It
  distinguishes valid-envelope `held`, `stale`, `no_lease`, and
  `release-head` classifications. A malformed fact or missing or malformed
  expiry produces the store-level failed-read and fail-closed outcome owned
  by `todo.coord-fact-store-hardening`; an unleased wave preview is never
  called a claim.
- The lease join and wave composition consume one `read_facts` snapshot. A
  regression fixture that changes the store between two potential reads must
  not produce disagreeing lease and wave rows.
- Once the new flags are wired and accepted,
  `cairn neighbourhood X --include-todos --include-changes
  --include-coordination --at <RFC3339> --json` emits a stable response with
  the node scope, baseline and live sections, observation instant, store
  state, explicit uninitialised-store reason when applicable, and plan
  digest. Human output contains the same rows and reasons.
- An uninitialised or missing store is an explicit empty-with-reason result,
  not silent admission. A malformed or partially unreadable family store
  fails closed. Missing `--at` is rejected for coordination rows, and
  non-UTC or fractional instants are rejected or normalized before lexical
  reader predicates run.
- The `NeighbourhoodResponse` schema extension has updated wire snapshots,
  while the existing neighbourhood todo fields remain behaviorally
  unchanged. Command-reference consistency, help-flag validation, and the
  repository's `--json` contract tests pass. The todo frontmatter remains
  `status: open` and `node: cairn.kernel.query`.

## Re-scope record

2026-08-09: re-scope completed against
`dec.rung-three-coordination-substrate`, `cairn lease list`, and the shipped
wave composer. This resolves the 2026-08-07 audit; the former pre-substrate
StateBackend and committed-only-as-live-signal framing are no longer
applicable. The committed todo/change baseline is loaded from the working tree
and remains the baseline, augmented by live lease and wave evidence.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves maintainable. It keeps
query results honest when graph nodes overlap.
