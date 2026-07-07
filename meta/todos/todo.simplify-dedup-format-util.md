---
node: cairn.kernel.cli
status: open
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
