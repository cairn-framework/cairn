# Design: Phase 7.5b Cleansing Splits

## References

- `openspec/conventions.md`: 500-line ceiling and allow-list mechanism.
- `openspec/changes/archive/phase-7.5a-test-fortification/design.md`: regression wall established here.
- `scripts/check-file-sizes.sh`: gate script; reads first non-blank line for allow-list.

## Split Principle

A split is valid when: (a) the moved symbols form a cohesive responsibility cluster, (b) the `mod.rs` re-exports everything moved so call sites outside the module need zero `use` path changes, and (c) the inline `#[cfg(test)] mod tests` block for the moved symbols migrates into the same submodule file.

No split introduces a new public module path at the crate boundary. All submodules are `pub(crate)` or private; `mod.rs` holds the public re-export surface.

---

## Module 1: `src/changes.rs` -> `src/changes/`

**Current line count:** 1444

**Seams identified:**

| Lines (approx.) | Cluster | Proposed submodule |
|---|---|---|
| 1-115 | Public types: `Change`, `BlueprintDelta`, `Rename`, `EdgeRename`, `ArtefactOperation`, `ChangeOperation`, `ArchiveReport`, `Snapshot` | `src/changes/types.rs` |
| 127-484 | Public functions: `discover`, `load_change`, `parse_blueprint_delta`, `validate_change`, `archive`, `create_rename_change`, `operation_summary`, `operations_for_nodes`, `active_changes_lines` | `src/changes/mod.rs` (kept here; these are the public entry points) |
| 486-642 | Delta parsing internals: `delta_sections`, `parse_node_section`, `parse_edge_section`, `parse_id_lines`, `parse_rename_lines`, `parse_edge_renames`, `uncomment_lines`, `clean_list_line`, `clean_scalar`, `flatten_nodes` | `src/changes/delta.rs` |
| 644-733 | Artefact operation parsing: `parse_artefact_operations`, `collect_artefact_operations`, `parse_operation` | `src/changes/artefact_ops.rs` |
| 735-870 | Validation: `validate_edges`, `graph_edge_exists`, `mark_node_touch`, `validate_artefacts`, `validate_artefact_refs` | `src/changes/validate.rs` |
| 854-1287 | Archive application: `mutation_paths`, `snapshot_paths`, `restore_snapshots`, `apply_archive`, `apply_blueprint_delta`, `rename_node_id`, `remove_node`, `replace_node`, `same_edge`, `serialize_ast`, `serialize_node`, `node_kind_name`, `serialize_field_values`, `replace_exact_id`, `apply_artefact_operations`, `write_artefact_target`, `strip_change_frontmatter`, `archive_path`, `today_utc`, `append_archive_log`, `atomic_write`, `atomic_write_bytes` | `src/changes/apply.rs` |
| 1179-1288 | Rename support: `copy_referencing_artefacts`, `copy_referencing_artefacts_from`, `frontmatter_references`, `update_frontmatter_reference`, `insert_operation`, `artefact_content_refs`, `read_to_string`, `proposal_title` | `src/changes/rename.rs` |

**Test migration:** The `#[cfg(test)] mod tests` block (lines 1289-1443) exercises `parse_blueprint_delta`, `apply_blueprint_delta`, and `validate_change`. These tests move to `src/changes/mod.rs` since they cross submodule boundaries (delta + apply + validate). The test block stays in `mod.rs` with `use super::delta::*; use super::apply::*;` added so the existing test code runs unmodified.

**Expected submodule sizes after split:**
- `mod.rs`: ~200 lines (public fns + tests)
- `types.rs`: ~115 lines
- `delta.rs`: ~160 lines
- `artefact_ops.rs`: ~90 lines
- `validate.rs`: ~140 lines
- `apply.rs`: ~370 lines
- `rename.rs`: ~115 lines

All submodules will be under 500 lines. The allow-list comment is removed.

---

## Module 2: `src/cli/mod.rs` -> `src/cli/`

**Current line count:** 1471

Note: this file is already `src/cli/mod.rs`, so it becomes the slimmed-down `mod.rs` and gains sibling files in the existing `src/cli/` directory.

**Seams identified:**

| Lines (approx.) | Cluster | Proposed submodule |
|---|---|---|
| 1-62 | Public types (`CliResult`), `registry()`, `run()`, `ParsedArgs`, `parse_args`, `run_project_command`, `render_loaded_project_command` | `src/cli/mod.rs` (kept; top-level dispatch) |
| 221-409 | Command handlers: `run_shared_json_command`, `shared_request`, `shared_flags`, `shared_exit_code`, `run_hook_command`, `run_archive_command`, `parse_hook_kind`, `legacy_blueprint_warning`, `run_ui_command`, `requires_valid_map`, `init_project` | `src/cli/commands.rs` |
| 411-828 | Per-command renderers: `render_get`, `render_neighbourhood`, `render_files`, `render_todos`, `render_decisions`, `render_research`, `render_sources`, `render_rationale`, `render_status`, `render_dependencies` | `src/cli/render.rs` |
| 879-1260 | JSON serialisation and output helpers: `node_json`, `finding_json`, `todos_json`, `decisions_json`, `research_json`, `reviews_json`, `sources_json`, `neighbourhood_ids`, `research_for_nodes`, `sources_for_nodes`, `flag_value`, `parse_todo_status_filter`, `parse_decision_status_filter`, `todo_line`, `decision_line`, `research_line`, `review_line`, `source_line`, `todo_status`, `decision_status`, `review_type`, `source_verification`, `findings_output`, `finding_output`, `error_output`, `ok`, `err`, `string_array_json`, `lines`, `esc`, `node_arg`, `render_node`, `render_findings` | `src/cli/format.rs` |

**Test migration:** The `#[cfg(test)] mod tests` block (lines 1261-1470) exercises `run()` end-to-end. It stays in `mod.rs` where `run` lives. The helper functions it calls (`run_in`, `run_in_str`, `write_project`, `write_change`, `temp_root`) stay in the test block in `mod.rs`. No test code moves.

**Expected submodule sizes after split:**
- `mod.rs`: ~280 lines (dispatch + tests)
- `commands.rs`: ~195 lines
- `render.rs`: ~420 lines
- `format.rs`: ~390 lines

All under 500 lines. The allow-list comment is removed from `mod.rs`.

---

## Module 3: `src/query_api.rs` -> `src/query_api/`

**Current line count:** 1323

**Seams identified:**

| Lines (approx.) | Cluster | Proposed submodule |
|---|---|---|
| 1-218 | Public types, `registry()`, `visible_tools()`, `execute()`, `envelope_json()`, `error_json()`, `TOOL_REGISTRY`, `tool()` const fn, `metadata_for_tool()` | `src/query_api/mod.rs` |
| 406-535 | Change-query functions: `execute_data` archive/rename/changes/show branches, `discover_changes`, `show_change`, `change_json` | `src/query_api/change_queries.rs` |
| 537-900 | Per-tool response builders: `neighbourhood_json`, `contract_json`, `single_contract_json`, `docstring_json`, `files_json`, `dependency_json`, `status_json`, `rationale_json`, `todos_response_json`, `decisions_response_json`, `research_response_json`, `sources_response_json`, `hook_json` | `src/query_api/handlers.rs` |
| 902-1172 | JSON serialisation: `node_json`, `todo_json`, `decision_json`, `research_json`, `review_json`, `source_json`, `findings_json`, `neighbourhood_ids`, `research_for_nodes`, `sources_for_nodes`, `relevant_rules`, `requires_valid_map`, `findings_error`, `finding_error`, `command_error`, `load_for`, `required`, `parse_todo_status_filter`, `parse_decision_status_filter`, `todo_status`, `decision_status`, `source_verification`, `hook_kind_name`, `hook_decision_name` | `src/query_api/serialise.rs` |

**Note on `execute_data`:** This function is the main dispatch arm inside `execute`. The archive/rename/changes/show branches are self-contained enough to extract into `change_queries.rs`. The `execute_data` function itself stays in `mod.rs` as a thin dispatcher calling into `handlers.rs` and `change_queries.rs`.

**Test migration:** The `#[cfg(test)] mod tests` block (lines 1173-1322) exercises `registry()`, `visible_tools()`, `execute()`, `envelope_json()`, `error_json()`. These all live in `mod.rs`. The test block stays in `mod.rs` unchanged.

**Expected submodule sizes after split:**
- `mod.rs`: ~280 lines (types + registry + execute + execute_data + tests)
- `change_queries.rs`: ~135 lines
- `handlers.rs`: ~370 lines
- `serialise.rs`: ~275 lines

All under 500 lines. The allow-list comment is removed.

---

## Module 4: `src/artefacts/registry.rs` -> `src/artefacts/registry/`

**Current line count:** 1239

Note: this file is already `src/artefacts/registry.rs`. It becomes `src/artefacts/registry/mod.rs` with siblings.

**Seams identified:**

| Lines (approx.) | Cluster | Proposed submodule |
|---|---|---|
| 1-265 | All public types and traits: `ArtefactType`, `ArtefactLoadRequest`, `ArtefactRecord`, `ArtefactError`, `ArtefactLoader`, `TodoStatus`, `Todo`, `DecisionStatus`, `Decision`, `ReviewType`, `Review`, `Research`, `SourceVerification`, `Source`, `ArtefactSet` | `src/artefacts/registry/types.rs` |
| 250-430 | Top-level loader and per-kind load functions: `load_artefacts`, `load_todos`, `load_decisions`, `load_reviews`, `load_research`, `load_sources` | `src/artefacts/registry/mod.rs` (public entry point stays here) |
| 432-715 | Integrity validation: `validate_integrity`, `validate_nodes`, `validate_node_list`, `validate_decision_refs`, `validate_provenance_refs`, `validate_sources`, `validate_verified_source` | `src/artefacts/registry/validate.rs` |
| 717-830 | File I/O helpers: `pointers`, `collect_pointers`, `collect_ids`, `collect_node_id`, `markdown_paths`, `read_dir_markdown`, `parse_file`, `required`, `optional`, `list`, `path_string`, `is_url`, `error`, `warning`, `error_finding` | `src/artefacts/registry/io.rs` |
| 836-916 | Status parsers: `parse_todo_status`, `parse_decision_status`, `parse_review_type`, `parse_source_verification` | `src/artefacts/registry/parse.rs` |
| 950-1102 | SHA-256 implementation: `SHA256_INITIAL_STATE`, `SHA256_ROUND_CONSTANTS`, `sha256_hex`, `compress_sha256_block`, `sha256_schedule` | `src/artefacts/registry/sha256.rs` |

**Test migration:** The `#[cfg(test)] mod tests` block (lines 1104-1238) exercises `load_artefacts` and status parsers. `load_artefacts` lives in `mod.rs`; the status parsers move to `parse.rs`. The test block moves to `mod.rs` with a `use super::parse::*;` import added. The existing test code is unchanged except for that one `use` line.

**Expected submodule sizes after split:**
- `mod.rs`: ~160 lines (load_artefacts + loaders + tests)
- `types.rs`: ~265 lines
- `validate.rs`: ~285 lines
- `io.rs`: ~115 lines
- `parse.rs`: ~80 lines
- `sha256.rs`: ~155 lines

All under 500 lines. The allow-list comment is removed.

---

## Module 5: `src/ui.rs` -> `src/ui/`

**Current line count:** 1009

**Seams identified:**

| Lines (approx.) | Cluster | Proposed submodule |
|---|---|---|
| 1-230 | Server infrastructure: static assets, `STYLE_CSS`, `UiOptions`, `UiError`, `ServerHandle`, `serve_current_thread`, `start_background`, `Server` struct + `bind`/`url`/`serve` | `src/ui/mod.rs` |
| 232-410 | Request handling and routing: `Server::handle_stream`, `Server::route`, `Server::api`, `Server::blueprint_json`, `Server::node_api`, `Server::load_project`, `read_http_request`, `Response`, `html`, `asset`, `json`, `text`, `write_response`, `request_path`, `open_browser` | `src/ui/server.rs` |
| 503-863 | API response builders: `meta_json`, `graph_json`, `node_json`, `dependency_json`, `lint_json`, `status_json`, `contract_response_json`, `contract_json`, `artefact_response_json`, `rationale_json`, `Artefact`, `collect_artefacts`, `collect_artefacts_from_dir`, `frontmatter_mentions_node`, `artefact_json`, `finding_json`, `project_finding`, `title_from_body` | `src/ui/api.rs` |
| 777-864 | Serialisation helpers: `kind_name`, `state_name`, `severity_name`, `graph_edge_kind_name`, `map_json`, `string_array_json`, `optional_json`, `percent_decode`, `esc` | `src/ui/serialise.rs` |

**Note on `Server` impl split:** `Server::handle_stream`, `Server::route`, `Server::api`, `Server::blueprint_json`, `Server::node_api`, `Server::load_project` are methods on `Server`. The `Server` struct is defined in `mod.rs` and the impl block for routing methods moves to `server.rs` via a separate `impl Server` block in that file. Rust allows split impl blocks across files in the same module.

**Test migration:** The `#[cfg(test)] mod tests` block (lines 865-1008) exercises server startup, route dispatch, 404, and method rejection. It calls `start_background` and `request_path` which live in `mod.rs`/`server.rs`. The test block stays in `mod.rs` and gains `use super::server::request_path;` for the `request_path` unit test. The network-level tests use `start_background` which remains in `mod.rs`.

**Expected submodule sizes after split:**
- `mod.rs`: ~280 lines (options + error + handle + serve + tests)
- `server.rs`: ~185 lines (routing methods + HTTP primitives)
- `api.rs`: ~365 lines (response builders + artefact collection)
- `serialise.rs`: ~95 lines (pure formatting helpers)

All under 500 lines. The allow-list comment is removed.

---

## No Borderline Modules

All five modules have clean cohesion seams with no encapsulation risk. None require a retained allow-list comment after this phase. The split preserves every public symbol at its existing import path via `pub use` in `mod.rs`.
