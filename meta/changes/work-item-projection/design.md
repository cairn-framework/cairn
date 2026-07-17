# Design: work-item-projection

## Approach

### The `WorkItem` projection

A new type in `src/query_api/handlers/work_item.rs`:

```rust
pub(crate) enum WorkItemSource { Finding, Todo, Bead }  // serde rename_all "lowercase"

pub(crate) struct WorkItem {
    pub source: WorkItemSource,
    pub title: String,
    pub node: Option<String>,
    pub command: Option<String>,
    pub rank: u32,
}
```

Both derive `Serialize` and `schemars::JsonSchema` so `work-item.schema.json`
is generated from the real type, not hand-written.

`rank` is a single ordering field across all three sources instead of
per-source semantics (a remediation action's `priority`, a bead's
`priority`, nothing for a todo): findings keep their existing 1-4
remediation-action priority scale (lower = more urgent, matches
`remediate_json`'s existing `priority` field, sorted ascending); an open
native todo always ranks `100`; a ready bead always ranks `200`. This
directly encodes `dec.native-todos-first`'s ordering (finding action >
native todo > bead) in the wire contract's own `rank` field, not just in
selection-time behaviour a client can't see. Only the single selected item
is ever exposed for the todo/bead tiers (`cairn next`/`status` show one
"next" item; nothing today exposes a ranked list of open todos or ready
beads through this projection), so the coarse tier constants are
sufficient; if that changes later the todo/bead tiers can be subdivided
without breaking the finding tier's meaning.

`command`/`node` are `Option<String>`: `None` (not e.g. `""`) when a
remediation action carries no `command` (the synthetic `"none"` action,
excluded — see below) or no affected node.

A single conversion point per source:

- **Finding** (`work_item_from_finding_action` in `work_item.rs`, takes
  the existing ad-hoc remediation-action `Value`): `rank` = the action's
  `priority`; `title` = the action's `description` (already the
  human-sentence field; the short `action` slug like `fix_blueprint` is
  kept internal); `node` = the action's `nodes` array's first entry (the
  unified shape is deliberately single-node — see Node fan-out below);
  `command` = the action's `command`, or `None` if empty.

  Returns `Option<WorkItem>`, not `WorkItem`: `remediate_json` always
  injects a synthetic `{"action":"none", "priority":0, ...}` placeholder
  when its `actions` vec would otherwise be empty (existing behaviour,
  unchanged, still needed so `format_remediate_human` keeps its exact
  current text). That placeholder is not a finding and must not surface
  as `source: "finding"` in any WorkItem-shaped output — `source` is a
  closed three-way enum describing real provenance, and exposing a
  bookkeeping placeholder as one would be a false statement on the wire,
  including in `status`/`next --json` when clean-detection and the
  placeholder-selection edge case coincide (a dirty project where none of
  `remediate_json`'s specific finding-code branches matched, so the
  synthetic placeholder gets selected as `NextSelection::Dirty`'s
  candidate). `work_item_from_finding_action` returns `None` for it,
  which becomes wire `null`/an omitted list entry rather than a fabricated
  finding.

- **Todo** (`CleanItem::NativeTodo`, built directly in `next_selection.rs`):
  `rank` = 100; `title` = `decision_summary(&todo.body)` (moved here, see
  below); `node` = `Some(todo.node.clone())`; `command` =
  `Some(format!("cairn todos {}", todo.node))`, reusing the exact command
  string `render_next_todo`'s human branch already prints, so the
  suggested command doesn't drift between human and JSON.

- **Bead** (`CleanItem::Bead`, built directly in `next_selection.rs`):
  `rank` = 200; `title` = `bead.title.clone()`; `node` =
  `bead.linked_node().map(ToOwned::to_owned)`; `command` =
  `Some(format!("bd show {}", bead.id))`, again reusing
  `render_next_bead`'s existing command string.

`next_selection.rs` gets one new function, `work_item_for_selection(&NextSelection) -> Option<WorkItem>`,
the single place that turns a resolved selection into the wire shape.
`CleanItem::None` and `NextSelection::Dirty(None)` both map to `None`
(wire `null`), as today.

### `decision_summary` moves

`decision_summary` (one-line title extraction with truncation, previously
`pub(super) fn` in `src/cli/render/remediate.rs`) moves to
`src/query_api/handlers/next_selection.rs` as `pub(crate)`, because
`work_item_for_selection` needs it and `query_api` cannot depend on `cli`.
`cli::render::remediate` keeps every existing call site working unqualified
by re-exporting it (`pub(super) use crate::query_api::decision_summary;`)
instead of redefining it, so `project.rs`'s `super::remediate::decision_summary`
reference is untouched. The function body and its two unit tests move
verbatim; no behaviour change.

### Node fan-out is intentionally lossy

A remediation action can name several affected nodes (`nodes: [...]`,
e.g. `add_decision` with multiple `decision_nodes`). The unified shape
exposes one `node`. This is an explicit, documented trade-off: the
projection is a *presentation* layer over already-ephemeral finding data
(per `todo.next-recommended-unification`, "only the presentation
unifies"), not a new authoritative multi-node data model; a client that
needs every affected node still has `cairn lint --json` /
`cairn health --json`, which are unchanged.

### `remediate_json` vs. `remediate_actions`: keeping human output unchanged

Today `remediate_json` builds the full `Vec<Value>` of rich
priority/action/command/description/nodes objects (including the `"none"`
placeholder) and wraps it as `{"actions": [...], "total_actions": N}` in
one function. `format_remediate_human` (CLI human renderer) reads that
same rich shape's fields directly.

If `remediate_json` started returning `WorkItem`-shaped actions, human
rendering would silently lose the `priority`/`action`/`description`/
`nodes` fields it depends on. So the existing detection-and-action-building
logic is extracted verbatim into a new `pub(crate) fn remediate_actions(root, changes_dir, scan_result) -> Vec<Value>`
(same behaviour, same `"none"` placeholder-when-empty, just returning the
vec instead of the wrapped envelope). `remediate_json` becomes a thin
wrapper: `remediate_actions(...)` filter-mapped through
`work_item_from_finding_action`, wrapped as `{"actions": [WorkItem...], "total_actions": N}`.
`render_remediate` (CLI) now calls `remediate_actions` directly and wraps
it itself for the human branch (`format_remediate_human` unchanged, byte
for byte, same tests), and calls `remediate_json` only for the `--json`
branch. `select_next`'s dirty-candidate lookup switches from parsing
`remediate_json(...)["actions"]` back out of JSON to calling
`remediate_actions(...)` directly (same data, no round-trip).

Net effect: when a project is clean, `remediate --json` now reports
`{"actions": [], "total_actions": 0}` (previously `{"actions": [{"action":"none",...}], "total_actions": 1}`)
per the point above; the human renderer's existing (previously
unreachable in practice) `actions.is_empty()` branch was already tested
against `"No actions required.\n"` — it does not fire here for the human
path because the human path still runs against the rich (placeholder
present) actions vec, unchanged. Only the wire JSON changes.

### `cairn next --json`

`render_next_todo`/`render_next_bead`/`render_next_action` each keep their
existing signature and human branch untouched; only their `if json`
branch changes, from hand-formatted per-source strings to
`serde_json::to_string(&work_item)` (or `null` when the projection
returns `None`, only reachable from `render_next_action` when the
selected action was the synthetic placeholder). The enclosing envelope
(`{"next": ..., "clean": ..., "ready": ...}` for the clean cases,
`{"next": ..., "clean": false}` for the dirty case — no `ready` field,
matching current behaviour) is unchanged; only the `next` value's shape
changes.

### `status_json.next_recommended`

`src/query_api/handlers/project.rs` replaces its inline
`match select_next(...) { ... }` with `work_item_for_selection(&select_next(...))`,
serialised to `Value` (`Value::Null` when `None`). `CleanItem`/`NextSelection`
imports in `project.rs` are dropped (no longer matched there directly).

### `cairn status --json` (CLI, `render_status`)

Investigation found `render_status` (`src/cli/render/project.rs`) never
adopted `todo.next-recommended-unification` step 1: it has its own
`next_recommended` lookup (`open_native_todos(...).first()` /
`state::backlog::ready(...).first()`) that never calls `select_next` and
so never surfaces a dirty-project remediation action, and its `--json`
output is a bare string, not an object. Per the constraint that human
rendering stays unchanged, only the JSON branch is touched: it now calls
`select_next` + `work_item_for_selection` independently (a second,
JSON-only call site; the existing human/`--brief` `next_recommended:
Option<String>` computation is untouched, same code, same tests). This
makes `cairn status --json`'s `next_recommended` an object matching
`status` (MCP/webui)'s shape, closing the gap the acceptance transcript
exercises. The human/brief text-only gap (never showing a dirty-project
action) is now a *known, documented* pre-existing limitation of
`render_status`'s human path, not touched by this change.

`select_next` performs real I/O against `root` (via `hooks::run` ->
`architecture_findings_from_project(root)`, reads `root/cairn.blueprint`
if present). The existing test
`render_status_json_next_recommended_null_when_clean` already isolates
itself in a fresh temp directory instead of the real repo checkout for
exactly this reason; `render_status_json_includes_next_recommended_for_native_todo`
previously used `Path::new(".")` because its old implementation never
touched `root` for I/O. It now must isolate itself the same way, or it
would depend on the real repo's transient dirty/clean state at test time.

### Schemas

`schemars` (`derive` feature only) becomes a normal dependency, not a
dev-dependency: `#[derive(JsonSchema)]` is applied directly to production
structs (`MapSnapshot`, `Finding`, `WorkItem`, ...) that live in `src/`
and compile unconditionally, so the crate and its derive macro must be
available in ordinary (non-test) builds. `jsonschema` (the validator)
stays dev-dependency only, per `todo.wire-format-schemas`: nothing at
runtime validates against these schemas, only tests do.

`#[derive(JsonSchema)]` added alongside the existing `Serialize`/
`Deserialize` derives on: `MapSnapshot`, `SnapshotNode`, `SnapshotEdge`
(`src/scanner/snapshot.rs`), `SymbolRecord`, `SymbolKind`
(`src/reconcile/symbol.rs`, nested in `SnapshotNode.symbols`), `Finding`,
`FindingSeverity` (`src/map/graph.rs`, nested in `MapSnapshot.findings`),
`WorkItem`, `WorkItemSource` (new). No field/shape changes to any of
these; `JsonSchema` is purely additive.

`schemas/map.schema.json`, `schemas/finding.schema.json`,
`schemas/work-item.schema.json` are generated once via
`schemars::schema_for!` and committed. A drift test regenerates each
schema in memory and asserts (`assert_eq!`) it is byte-identical to the
committed file's parsed JSON. On mismatch it fails with a message naming
the file to regenerate, exactly like `map.json`/wire-snapshot drift gates.
It never writes to the source tree during an ordinary `cargo test` run.

Tests (in `tests/schema_validation.rs`, new): build the scanner's own
dogfood snapshot in-process and validate it against `map.schema.json`;
validate a constructed `Finding` sample against `finding.schema.json`;
validate the `WorkItem`s inside a `status --json` run (dirty and clean
fixtures) against `work-item.schema.json`. All via the `jsonschema` crate.

### Registry coverage test

`src/query_api/registry.rs` gains a test asserting every distinct
`response_schema` label across `TOOL_REGISTRY`'s 43 entries (42 unique;
`deps`'s in/out variants share `DependencyResponse`) either has a file at
`schemas/<label>.schema.json` (none currently do — none of the 42
registered tool response labels is literally named `Map`, `Finding`, or
`WorkItem`; `WorkItem` backs `StatusResponse`'s and `RemediateResponse`'s
*payload sub-shape*, not their full response envelope, so this change does
not yet claim full-envelope schema coverage for either) or is listed in an
explicit `UNSCHEMAD_ALLOWLIST` with a burn-down comment, mirroring
`tests/finding_code_coverage.rs`'s `UNCOVERED_ALLOWLIST` pattern (dual
assertion: every label is covered-or-allowlisted, and every allowlist
entry is still genuinely unschema'd, so a stale entry left behind after a
future schema lands also fails). All 42 current labels seed the allowlist.

### `docs/integration-contract.md`

Adds a short section naming `schemas/` as the authority for `map.json`,
`Finding`, and the work-item projection shapes, with the registry test as
the enforcement mechanism, and notes most `response_schema` labels remain
allowlisted (not yet backed by a file) pending future increments.

## Changes

ADDED:
- `src/query_api/handlers/work_item.rs`: `WorkItem`, `WorkItemSource`,
  `work_item_from_finding_action`.
- `src/query_api/handlers/next_selection.rs`: `decision_summary` (moved),
  `work_item_for_selection`.
- `src/query_api/handlers/remediate.rs`: `remediate_actions`.
- `schemas/map.schema.json`, `schemas/finding.schema.json`,
  `schemas/work-item.schema.json`.
- `tests/schema_validation.rs`.
- `src/query_api/registry.rs`: `UNSCHEMAD_ALLOWLIST` + coverage test.
- `docs/integration-contract.md`: schemas section.

MODIFIED:
- `src/query_api/mod.rs`: `SCHEMA_VERSION` 2 -> 3; re-exports for the new
  functions/types.
- `src/ui/mod.rs`: `SCHEMA_VERSION` 2 -> 3; its two literal
  `"schema_version":2` test assertions -> 3.
- `src/query_api/handlers/project.rs`: `status_json`'s `next_recommended`
  construction.
- `src/query_api/handlers/remediate.rs`: `remediate_json` becomes a thin
  `WorkItem`-projecting wrapper over the extracted `remediate_actions`.
- `src/cli/render/remediate.rs`: `render_remediate` sources human/JSON
  output from different query_api entry points; `render_next_todo`/
  `render_next_bead`/`render_next_action`'s `json` branches emit
  `WorkItem`; local `decision_summary` definition replaced by a
  re-export.
- `src/cli/render/project.rs`: `render_status`'s `--json` branch computes
  `next_recommended` via `select_next`/`work_item_for_selection`.
- `src/scanner/snapshot.rs`, `src/reconcile/symbol.rs`, `src/map/graph.rs`:
  add `JsonSchema` derives (additive only).
- `Cargo.toml`: `schemars` (normal dependency, `derive` feature),
  `jsonschema` (dev-dependency).
- `tests/wire_format_snapshots.rs`: `schema_version` assertion 2 -> 3;
  snapshot fixtures regenerated (`api_status` payload shape change +
  version bump; every other endpoint version-bump only).
- `src/query_api/tests.rs`: `test_execute_status_includes_next_recommended`
  updated to the new `WorkItem` shape.
- `src/cli/render/project/tests.rs`:
  `render_status_json_includes_next_recommended_for_native_todo` updated
  to the new object shape and isolated temp-dir root (was `"."`, now
  unsafe given `select_next`'s real root I/O).

REMOVED:
- The synthetic `"none"` remediation-action placeholder's exposure as a
  `source: "finding"` WorkItem (it still exists internally for human
  rendering; it never reaches wire JSON as a fabricated finding).

RENAMED:
- None.
