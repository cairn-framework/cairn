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

### Committed baseline

- Read the checked-in `meta/todos/todo.<slug>.md` artefacts already loaded in
  `scan_result.artefacts.todos`. Include `open` and `in_progress` todos whose
  `node` is in the one-hop set. Exclude `done` and `blocked` todos from the
  in-flight list, while retaining their IDs only when needed to explain a
  relationship.
- Discover active change directories through
  `src/changes/mod.rs::discover` using the resolved `--changes-dir` (default
  `meta/changes`). Use `operations_for_nodes` to include deltas and artefact
  references that mention a node in the one-hop set. Mark this layer as the
  repository baseline: it can lag work that has not been pushed.

### Live coordination

- The lease read verb is `cairn lease list`, routed by
  `src/cli/commands/coord.rs`. Its `--json`, `--at <RFC3339>`, and
  `--since <filename>` forms expose raw lease and driver-singleton facts.
  The separate `cairn coord verify|compact` verbs are store administration,
  not the source of in-flight matches.
- Read the family-local store through
  `src/coord/read.rs::read_facts`, then join `payload.unit_id` to the
  `todo.<unit_id>.md` stem. Use the shared reader predicates
  `lease_chain_head`, `held`, `stale`, and `no_lease` at the supplied
  observation instant. A lease is held on a dispatch unit, never directly on
  a node. Preserve the lease fact ID, holder, expiry, and residue in the
  result. Do not infer `active`, `expired`, or `stale` from the raw
  `cairn lease list` wire, and do not classify from a paginated `--since`
  subset.
- Include the matching `units` and `held` rows from
  `src/query_api/wave/compose.rs::compose_wave`, the shared composer used by
  `cairn wave --json --at <RFC3339>`. Preserve the plan digest, unit IDs,
  write-set prefixes, and held reasons (`lease-held`, `write-sets-overlap`,
  `runs-alone`, and `parked`) with any blocking fact ID. A wave preview is
  live coordination context, not proof that a unit has started; only a
  lease chain classified as held at `--at` may be rendered as a current
  claim. `cairn ruling run <plan-digest>` records consent and is not itself
  an in-flight claim.

The old `StateBackend` seam is dropped. `dec.change-format-only` deleted the
generic persistence abstraction and the beads claim and sequence workflow, so
this read-only view must not revive either. It composes the scanner's
committed artefacts with `read_facts` and `compose_wave`; it writes no facts
and performs no claiming, sequencing, or dispatch. The core and appender do
not evaluate expiry; only the reader predicates derive `held`, `stale`, and
`no_lease` from an explicit `--at`. Coordination reads fail closed when the
family store cannot be fully resolved, and no clock is consulted when the
caller does not supply `--at`.

## CLI surface

Fold this view into the existing node-scoped
`cairn neighbourhood <node>` query, rather than adding a `cairn conflicts`
top-level noun or extending project-wide `cairn status`. `neighbourhood`
already owns the one-hop graph scope and the opt-in
`--include-todos`/`--include-changes` artefact sections. Add an opt-in
`--include-coordination` section and pass `--at <RFC3339>` through the shared
request path; require `--at` when resolving live lease state. Keep
`--json` as the machine-readable form and make the human sections mirror the
same baseline and live rows.

The implementation surfaces are the existing `neighbourhood` registry and
dispatch in `src/query_api/registry.rs` and `src/query_api/mod.rs`,
`src/query_api/handlers/graph.rs::neighbourhood_json`,
`src/cli/commands/mod.rs::shared_flags` and `shared_request`,
`src/cli/render/node.rs::render_neighbourhood`, and the request flag/schema
mapping in `src/mcp/mod.rs` and `src/cli/help/mod.rs`. Update
`docs/commands.md` and `docs/integration-contract.md` for the new opt-in
section. No new command row is expected, but
`tests/command_reference_consistency.rs` and the existing query wire
contract tests must remain green.

## Acceptance

- A fixture graph with target `X`, two open todos on adjacent nodes, and an
  active change referencing one adjacent node returns exactly the matching
  baseline todos and change operations, while excluding unrelated nodes.
- The same fixture has lease facts for one todo and a wave containing both
  admitted and held units. With `--at <RFC3339>`, the response joins the
  lease holder, expiry, residue, fact ID, wave plan digest, unit write-sets,
  and held reasons to the matching node IDs. It distinguishes held, stale,
  and no-lease states; an unleased wave preview is never called a claim.
- `cairn neighbourhood X --include-todos --include-changes
  --include-coordination --at <RFC3339> --json` emits a stable response with
  the node scope, baseline and live sections, observation instant, store
  state, and plan digest. Human output contains the same rows and reasons.
- A missing or partially unreadable family store fails closed, and omitting
  `--at` does not consult a clock or invent a live lease verdict.
- Command-reference consistency, query wire/schema snapshots, and the
  repository's `--json` contract tests pass. The todo frontmatter remains
  `status: open` and `node: cairn.kernel.query`.

## Re-scope record

2026-08-09: re-scope completed against
`dec.rung-three-coordination-substrate`, `cairn lease list`, and the shipped
wave composer. This resolves the 2026-08-07 audit; the former pre-substrate
StateBackend and committed-only-as-live-signal framing are no longer
applicable. Committed todos and changes remain the baseline, augmented by live
lease and wave evidence.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves maintainable. It keeps
query results honest when graph nodes overlap.
