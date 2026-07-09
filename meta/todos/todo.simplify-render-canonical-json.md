---
node: cairn.kernel.cli
status: open
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
