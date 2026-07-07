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
