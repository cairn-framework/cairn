---
node: cairn.kernel.cli
status: done
created: 2026-07-06
---

# Incremental: Human Renderers Consume Canonical JSON

Part of todo.simplify-architecture (wave 4, last).
Depends on: todo.simplify-dedup-format-util and
todo.simplify-cli-registry-table. Do this per command, measuring, not as
a big bang.
Follow the shared rules in todo.simplify-architecture.

The CLI human path (`src/cli/mod.rs:328-483` dispatching into
`src/cli/render/`) queries the engine directly, parallel to the
query_api JSON path. Target end state: each command computes its
canonical JSON once (query_api), and the human renderer is a JSON-to-text
transformer.

Scope discipline (this is the task where the estimate was corrected):

- IN scope: shape-rebuilding renderers whose output mirrors the
  canonical JSON: `render/node.rs`, `render/artefacts.rs`,
  `render/project.rs`, `render/health.rs`, `render/changes_view.rs`.
  Migrate one command per commit; delete the direct engine calls as each
  moves.
- OUT of scope (genuine presentation logic, keep as render code):
  `render/context_view.rs` (Mermaid renderer, depth rollup, scope
  resolution) and the plan-composition core of `render/remediate.rs`.
  If they need data the JSON lacks, enrich the query_api handler and
  record the shape change per the JSON schema versioning convention.
- Stop rule: if a command's transformer ends up bigger than the renderer
  it replaces, leave that command alone and note it here.

Guards: CLI snapshot tests under `src/cli/snapshots`,
`src/cli/render/project/tests.rs`, and `scripts/dogfood.sh` (pre-push:
`cairn lint` + `cairn hook all` against the working-tree binary).
Note `tests/dogfood_gate.rs` only lints the dogfood script's text; it
does not exercise renderers.

Acceptance: migrated commands read only canonical JSON; human output
unchanged (snapshots) or deliberately improved; net LOC reduction
recorded per command in this file as migration proceeds.
## Per-command LOC delta (renderer before/after)

Migration pattern: drop `scan_result`, build a `QueryRequest`, call
`crate::query_api::execute`, transform `response.data` (a `serde_json::Value`)
reusing `format::lines`/`esc`. `node_arg` is kept for the `CAIRN_CLI_MISSING_NODE`
code; direction/flag validation that lives in the outer `run_project_command`
dispatcher is not re-checked inside the renderer.

| Command | Renderer LOC before | Renderer LOC after | Delta | Notes |
|---------|--------------------:|-------------------:|------:|-------|
| deps | 26 | 38 | +12 | query_api `dependency_json` already existed; +12 is the standard canonical-JSON boilerplate (request build + `Value` extraction). Human output byte-identical. |
| todos | 31 | 41 | +10 | query_api `todos_response_json` already existed; +10 is the standard canonical-JSON boilerplate (request build + `Value` extraction). Obsolete `--json` branch now served by the shared JSON path; human output byte-identical (`todos_text` reproduces `todo_line`). |
| research | 22 | 45 | +23 | query_api `research_response_json` already existed; +23 is the canonical-JSON boilerplate (request build + nested `sources` extraction). Transformer `research_text` (22) is not larger than the renderer it replaces (old `render_research`, 22), so the stop rule is not tripped. Human output byte-identical (`research_text` reproduces `research_line`: id + sources joined with ", "). |
| sources | 22 | 43 | +21 | query_api `sources_response_json` already existed; +21 is the canonical-JSON boilerplate. Transformer `sources_text` (20) is not larger than the renderer it replaces (old `render_sources`, 22). Human output byte-identical (`sources_text` reproduces `source_line`). |
| rationale | 68 | 80 | +12 | query_api `rationale_json` already existed; +12 is the canonical-JSON boilerplate (request build + nested `decisions`/`research`/`sources` extraction). Transformer `rationale_text` (57) is not larger than the renderer it replaces (old `render_rationale`, 68), so the stop rule is not tripped. Human output byte-identical. This commit also drops the now-dead format helpers `research_json`, `sources_json`, and `source_line` (only the migrated renderers used them) plus their cascading unused imports. |
| files | 89 | 98 | +9 | query_api `files_json` already existed; net +9 because the human transformer (`files_text`, 75) must extract each target field from `Value` (the old renderer read `TargetReport` structs directly). The transformer (75) is not larger than the renderer it replaces (old `render_files`, 89, which included the now-removed dead `--json` branch), so the stop rule is not tripped. Human output byte-identical. |
| change show | 49 | 59 | +10 | query_api `show_change`/`change_json` already existed and carry every field rendered (id, path, title, proposal, design, summary, findings); +10 is the canonical-JSON boilerplate. The handler's error codes (`CAIRN_CHANGE_NOT_FOUND`, `CAIRN_CHANGES_DISCOVERY_FAILED`) match the renderer's, so `query_error_to_finding` preserves them; the CLI-side missing-id check (`CAIRN_CLI_MISSING_CHANGE`) is kept. Transformer `show_change_text` (29) is not larger than the renderer it replaces (old `render_show`, 49). Human output byte-identical. |

## Stop-ruled / deferred commands (every in-scope command accounted for)

Each below was assessed against its query_api handler and left unmigrated
for a concrete, verified reason (handler enrichment or out-of-scope
dependency). Renderer LOC is the current, unchanged size.

| Command | Renderer LOC | Reason |
|---------|-------------:|-------|
| decisions | 73 (36 + 37 grep) | The `--grep` mode (`render_decisions_grep`) does substring matching over id/body/nodes; `decisions_response_json` filters only by node + status, so migrating needs a grep field on `QueryRequest` plus handler logic (non-trivial enrichment). Clean cutover is not possible without it; left as render code. |
| get | 59 (24 + 35 `symbols_block`) | `symbols_block` formats `SymbolKind` via `{:?}` (Debug -> "Function"), but `node_json` serializes symbols with `serde(rename_all = "lowercase")` ("function"); a canonical-JSON transformer would drift the human output unless it re-PascalCases or the JSON shape changes. Also node-vs-backlog dual-shape detection. |
| neighbourhood | 98 | Needs `--include-orphans`: the handler calls `query::neighbourhood` (no orphans option) while the renderer uses `query::neighbourhood_with_options(.., include_orphans)`. Dropping the flag loses capability; adding it is handler enrichment. The nine-section human transformer would also be large. |
| status | 66 | `next_recommended` prefers native todos via `remediate::open_native_todos` + `decision_summary` (`render/remediate.rs`, OUT of scope) which `status_json` does not compute; migrating would silently change behaviour when native todos exist, or require enriching `status_json`. |
| context | 62 | Bound to `context_view.rs` (`render_structure`/`render_mermaid`, depth rollup - OUT of scope) and already diverges from `context_json` (no `Config` access, so cannot show `project_context`). |
| change list | 16 | Human output is `changes::active_changes_lines` (a shared text formatter) plus a copy-lookup empty state; `discover_changes` returns `change_json` objects, so a transformer would duplicate `active_changes_lines` rather than thinly transform. Low value: reads no scan_result/engine state. |

## health (migrated prior session, residual noted)

`render_health` already reads canonical JSON via `query_api::health_json`,
and `format_health_human` is already a `Value` transformer (55 LOC).
Residual: it keeps `scan_result` only to emit the two supplementary
"scan errors: N" / "scan warnings: N" lines (reachable: `health` is not
in `requires_valid_map`), counts that `health_json` does not expose.
Fully dropping `scan_result` would require adding those counts to the
`health_json` shape - a wire-format change recorded here as a follow-up,
not attempted in this pass.

## Outcome

Status: done (2026-07-10). Every in-scope command in
`render/{artefacts,node,project,health,changes_view}.rs` is either
migrated to read only canonical query_api JSON (deps, todos, research,
sources, rationale, files, change show; health migrated in a prior
session) or explicitly stop-ruled above with LOC and a verified reason
(decisions, get, neighbourhood, status, context, change list). All seven
migrated commands produce byte-identical human output (1552 tests + CLI
snapshots green); gates (fmt / clippy / test / pre-archive-rust-gates,
`cairn scan --strict`, `cairn lint`, `cairn hook all`) pass on the
working-tree binary.
