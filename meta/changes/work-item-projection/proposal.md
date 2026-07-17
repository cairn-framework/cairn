# Proposal: work-item-projection

## Motivation

`todo.next-recommended-unification` step 1 (shipped v0.4.0) made
`status_json`'s `next_recommended` delegate to the same `select_next`
selection logic `cairn next` uses, so both surfaces agree on *which* item
comes next. They still disagree on *shape*: `status_json.next_recommended`
is a remediation-action object, a `todo_json` object, or a `BacklogItem`
JSON object depending on which source won; `cairn next --json`'s `next`
field is a third, independently hand-formatted object per source
(`{"todo":...,"node":...,"title":...,"source":"native-todos"}` vs
`{"bead":...,"title":...,"priority":...,"source":"beads-backlog"}` vs the
raw remediation-action object); `cairn remediate --json`'s `actions` array
is a list of ad-hoc remediation-action objects with no todo/bead
counterpart at all. A client has to special-case three shapes per source
per surface to answer "what's next" generically.

A parallel gap: `cairn status --json` (the CLI command, `render_status` in
`src/cli/render/project.rs`) never adopted step 1's fix. It reimplements
its own `next_recommended` lookup directly over `open_native_todos`/
`state::backlog::ready`, bypassing `select_next` entirely, so it never
surfaces a dirty-project remediation action and emits a bare *string*
(`"Wire the thing (native todo, node: app)"`) instead of an object. This
surfaced during investigation for this change and is fixed here (JSON path
only; see Non-goals).

Investigation also found the `query_api` wire surface has no schema
artifacts at all (`todo.wire-format-schemas`): `SCHEMA_VERSION` constants
exist but nothing declares the shapes they version, and the registry's
`response_schema` labels are bare strings with no backing file. Landing the
work-item projection is the natural first real schema (increment (c)); the
proposal folds in increments (a) and (b) (`map.schema.json`,
`finding.schema.json`) since they were already scoped as small,
independent, and this change is already touching the query-API wire
contract and its version.

## Scope

- A shared `WorkItem` projection (`source`, `title`, `node`, `command`,
  `rank`) in `src/query_api/handlers/work_item.rs`, derived from the
  existing `NextSelection`/`CleanItem` selection machinery.
- Wire `status_json.next_recommended`, `cairn next --json`'s `next` field,
  `cairn remediate --json`'s `actions` list, and `cairn status --json`'s
  `next_recommended` field (CLI path) to emit `WorkItem` values.
- `query_api::SCHEMA_VERSION` 2 -> 3 (breaking shape change to the
  `status`/`remediate` tool responses). `ui::SCHEMA_VERSION` 2 -> 3 to
  match, because `/api/status` re-serves `status_json`'s payload
  unchanged (`src/ui/server.rs` `spine_data` strips and the caller
  re-stamps with `ui::SCHEMA_VERSION`), so its contract changes too.
- `schemas/map.schema.json`, `schemas/finding.schema.json`,
  `schemas/work-item.schema.json` via `schemars`, plus tests validating
  real serialised output against each.
- A registry test requiring every `response_schema` label in
  `TOOL_REGISTRY` to resolve to a schema file or an explicit allowlist
  entry.
- `docs/integration-contract.md` updated to point at `schemas/` as the
  shape authority.

## Out of scope

- Materialising findings as durable todo artefacts (explicitly rejected
  by `todo.next-recommended-unification`: desync risk; findings are the
  controller's ephemeral error signal, todos are durable work).
- A parallel schema registry structure; the existing `TOOL_REGISTRY` stays
  the single source of truth for tool metadata, the new test only
  cross-checks it against `schemas/`.
- Schemas for internal-only shapes, or for every `response_schema` label
  (only the three shapes this change actually creates get files; the rest
  are allowlisted as pre-existing unschema'd surface, per
  `todo.wire-format-schemas`).
- Changing `cairn status`'s human-readable (non-`--json`) output. It keeps
  its pre-existing, independent next-recommended text derivation
  unchanged; only the JSON path is corrected to use the shared selection.
- Adding a `schema_version` stamp to `cairn status --json` beyond the
  version-number bump itself: `render_status` builds its JSON by hand
  outside the `query_api::execute` choke point (unlike `status`/`remediate`
  as MCP/webui tools) and already omits several fields those surfaces
  carry (e.g. real `active_changes`); bringing it fully onto the choke
  point is a larger, separate refactor not required by this change.
