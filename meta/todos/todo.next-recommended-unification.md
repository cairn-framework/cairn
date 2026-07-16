---
node: cairn.kernel.query
status: in_progress
created: 2026-07-16
---

# Next Recommended Unification

`cairn status` disagrees with `cairn next` about what to do next.
`status_json` computes next_recommended from the beads backlog only
(`src/query_api/handlers/project.rs:31`), ignoring findings and native
todos, while `cairn next` implements the correct priority order
(`src/cli/render/remediate.rs:100`: findings first via top remediation
action, then top open native todo, then top ready bead, per
dec.native-todos-first). An agent asking "what is the state" gets a
different answer than one asking "what should I do next". Distinct from
the fixed `todo.status-active-changes-bug`, which covered active_changes.

Fix in two steps:
1. Make status_json's next_recommended delegate to the same selection
   logic `cairn next` uses (extract the selection into a shared helper;
   render layers stay separate).
2. Consider a shared work-item projection (source, title, node, command,
   rank) across remediation actions, native todos, and beads in
   status/next/remediate --json, so agents see one queue vocabulary.
   Findings stay ephemeral and todos stay durable; only the presentation
   unifies. Materializing findings as durable todo artefacts was
   assessed and rejected (desync risk; findings are the error signal in
   the controller framing).

Motivation: `res.a2ui-analysis`, follow-on findings section (findings as
tasks). Step 1 is a small correctness fix, no proposal needed. Step 2
touches the external JSON contract; propose before implementing, and
coordinate with `todo.wire-format-schemas` so the projected shape ships
with a schema.

## Status

2026-07-16: Step 1 is complete. Shared next-selection logic now ranks the top remediation action first, then the earliest open native todo, then the top ready bead. The `status_json` handler and `cairn next` renderer both delegate to this helper, with a regression test proving the status response and helper agree when a native todo competes with a ready bead. The status wire snapshot was deliberately updated because the existing fixture exposes a remediation action instead of `null`.

Step 2 remains pending a change proposal. It would introduce a shared work-item projection across status, next, and remediate JSON and is explicitly out of scope for this unit.

Review follow-up (2026-07-16): the `next_recommended` field now admits remediation-action and native-todo shapes where schema v1 only admitted the ready-bead/null contract, so per `dec.query-json-schema-version` `query_api::SCHEMA_VERSION` is bumped 1 -> 2. Verified `src/ui/server.rs` `spine_data` strips the query stamp and re-stamps `/api/status` with the independent `ui::SCHEMA_VERSION` constant, and that endpoint serves this same `status_json` payload, so per `dec.webui-json-schema-version` `ui::SCHEMA_VERSION` is bumped 1 -> 2 as well. Every `wire_format_snapshots__*.snap` fixture and the two literal `"schema_version":1` test assertions in `src/ui/mod.rs` were regenerated/updated to `2` accordingly; the snapshot diff is version-only for every endpoint except `api_status`, whose `next_recommended` content diff was already reviewed above.
