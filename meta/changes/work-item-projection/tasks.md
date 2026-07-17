# Tasks: work-item-projection

- [x] Add `WorkItem`/`WorkItemSource` + `work_item_from_finding_action` in `src/query_api/handlers/work_item.rs`
- [x] Move `decision_summary` to `next_selection.rs`; add `work_item_for_selection`; re-export from `cli::render::remediate`
- [x] Extract `remediate_actions` from `remediate_json`; `remediate_json` projects to `WorkItem`
- [x] Wire `status_json.next_recommended` through `work_item_for_selection`
- [x] Wire `cairn next --json` (`render_next_todo`/`render_next_bead`/`render_next_action`) to emit `WorkItem`
- [x] Fix `cairn status --json` (`render_status`) to use `select_next`/`work_item_for_selection`; keep human output unchanged
- [x] Bump `query_api::SCHEMA_VERSION` 2 -> 3 and `ui::SCHEMA_VERSION` 2 -> 3 (+ literal test assertions)
- [x] Add `schemars`/`jsonschema` to `Cargo.toml`; derive `JsonSchema` on `MapSnapshot`/`SnapshotNode`/`SnapshotEdge`/`SymbolRecord`/`SymbolKind`/`Finding`/`FindingSeverity`/`WorkItem`/`WorkItemSource`
- [x] Generate and commit `schemas/map.schema.json`, `schemas/finding.schema.json`, `schemas/work-item.schema.json`
- [x] Add `tests/schema_validation.rs` validating dogfood map, a Finding sample, and status WorkItem output
- [x] Add registry `response_schema` coverage test + `UNSCHEMAD_ALLOWLIST` in `src/query_api/registry.rs`
- [x] Update `docs/integration-contract.md` to point at `schemas/`
- [x] Update `query_api/tests.rs`, `cli/render/project/tests.rs`, `cli/render/remediate.rs` tests for the new shapes
- [x] Regenerate `tests/snapshots/wire_format_snapshots__*.snap`; review diff is version+shape only where expected
- [x] Run `scripts/pre-archive-rust-gates.sh` and `cairn scan --strict`; fix findings
- [x] Capture acceptance transcripts (`status --json`, `next --json`, `remediate --json`)
- [x] Flip `todo.next-recommended-unification` and `todo.wire-format-schemas` status with dated Resolutions
- [ ] Parent archives completed change
