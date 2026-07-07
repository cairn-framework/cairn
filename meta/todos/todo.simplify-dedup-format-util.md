---
node: cairn.kernel.cli
status: done
created: 2026-07-06
---

# Dedup format/util Helpers Against query_api/serialise

Part of todo.simplify-architecture (wave 1). Depends on: nothing.
Follow the shared rules in todo.simplify-architecture.

`src/cli/format/util.rs:17-60` duplicates helpers that already exist in
`src/query_api/serialise.rs`: `neighbourhood_ids`, `research_for_nodes`,
`sources_for_nodes`, and any siblings found during the change. Two copies
means a query fix in one silently misses the other.

- Make the `query_api::serialise` versions the single implementation
  (move them to a shared location if visibility requires it; do not leave
  a re-export shim).
- Point all `src/cli/format/` and `src/cli/render/` callers at the single
  implementation and delete the copies.
- While in there, inventory any further verbatim shape-building
  duplication between `src/cli/format/json.rs` and
  `src/query_api/serialise.rs`; dedup what is mechanical, list what is
  not in this todo for todo.simplify-render-canonical-json.

Acceptance: no function body duplicated between `src/cli/format/` and
`src/query_api/serialise.rs`; CLI snapshot tests and `cargo test` green;
`--json` output byte-identical (it already flows through query_api).

Resolution (2026-07-07): eight verbatim copies removed, not three. The
`query_api::serialise` versions are the single implementation, raised to
`pub(crate)` and re-exported as `crate::query_api::<name>` beside the
existing `requires_valid_map` re-export: `neighbourhood_ids`,
`research_for_nodes`, `sources_for_nodes`, `parse_todo_status_filter`,
`parse_decision_status_filter`, `todo_status`, `decision_status`,
`source_verification`. Copies deleted from `src/cli/format/util.rs` and
`src/cli/format/render.rs`; callers (`cli/render/artefacts.rs`,
`cli/render/node.rs`, `cli/format/json.rs`, `cli/format/render.rs`) now
import from `crate::query_api`; the status roundtrip, unknown-filter, and
source_verification display tests moved to `serialise.rs` with the
implementation, and the two all-variants parse tests were dropped as
subsumed by the roundtrips. `review_type` stays in render.rs
(no serialise counterpart).

Inventory for todo.simplify-render-canonical-json (not mechanical, not
deduped here): `cli/format/json.rs` hand-builds escaped JSON strings for
the same shapes `serialise.rs` builds as `serde_json::Value` (`node_json`,
`todos_json`/`todo_json`, `decisions_json`/`decision_json`,
`research_json`, `reviews_json`/`review_json`, `sources_json`/
`source_json`, `finding_json`/`findings_json`). Same keys, different
mechanism; collapsing them means routing the human-path renderers through
Value building, which is that todo's measured, per-command scope.
