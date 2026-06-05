# Tasks: Phase 7.5b Cleansing Splits

Each task group is an atomic commit. Run `cargo fmt`, `cargo clippy --all-targets --all-features -D warnings`, `cargo test`, and `bash scripts/pre-archive-rust-gates.sh` before sealing each commit.

---

## 1. Split `src/changes.rs`

- [x] 1.1 Create `src/changes/` directory. Move `src/changes.rs` to `src/changes/mod.rs`.
- [x] 1.2 Create `src/changes/types.rs`. Move `Change`, `BlueprintDelta`, `Rename`, `EdgeRename`, `ArtefactOperation`, `ChangeOperation`, `ArchiveReport`, and the private `Snapshot` struct into it. Add `pub(crate) use types::*;` in `mod.rs` so all public types remain at `crate::changes::*`.
- [x] 1.3 Create `src/changes/delta.rs`. Move `delta_sections`, `parse_node_section`, `parse_edge_section`, `parse_id_lines`, `parse_rename_lines`, `parse_edge_renames`, `uncomment_lines`, `clean_list_line`, `clean_scalar`, `flatten_nodes`, `parse_blueprint_delta` into it. Expose `parse_blueprint_delta` as `pub` and re-export from `mod.rs`.
- [x] 1.4 Create `src/changes/artefact_ops.rs`. Move `parse_artefact_operations`, `collect_artefact_operations`, `parse_operation` into it.
- [x] 1.5 Create `src/changes/validate.rs`. Move `validate_change`, `validate_edges`, `graph_edge_exists`, `mark_node_touch`, `validate_artefacts`, `validate_artefact_refs` into it. Re-export `validate_change` from `mod.rs`.
- [x] 1.6 Create `src/changes/apply.rs`. Move `mutation_paths`, `snapshot_paths`, `restore_snapshots`, `apply_archive`, `apply_blueprint_delta`, `rename_node_id`, `remove_node`, `replace_node`, `same_edge`, `serialize_ast`, `serialize_node`, `node_kind_name`, `serialize_field_values`, `replace_exact_id`, `apply_artefact_operations`, `write_artefact_target`, `strip_change_frontmatter`, `archive_path`, `today_utc`, `append_archive_log`, `atomic_write`, `atomic_write_bytes` into it.
- [x] 1.7 Create `src/changes/rename.rs`. Move `copy_referencing_artefacts`, `copy_referencing_artefacts_from`, `frontmatter_references`, `update_frontmatter_reference`, `insert_operation`, `artefact_content_refs`, `read_to_string`, `proposal_title` into it.
- [x] 1.8 Ensure `mod.rs` `use` declarations pull from submodules. Add `mod delta; mod artefact_ops; mod validate; mod apply; mod rename; mod types;` with appropriate visibility.
- [x] 1.9 The existing `#[cfg(test)] mod tests` block in `mod.rs` references `parse_blueprint_delta`, `apply_blueprint_delta`, `validate_change`, and `scanner`. Add `use super::delta::parse_blueprint_delta; use super::apply::apply_blueprint_delta; use super::validate::validate_change;` inside the test block so all tests compile without assertion changes.
- [x] 1.10 Remove the `// cairn:allow-large-module reason: scheduled-for-phase-7.5b-split` comment from the first non-blank line of `src/changes/mod.rs`.
- [x] 1.11 Verify: `cargo fmt --check` passes. `cargo clippy --all-targets --all-features -D warnings` passes. `cargo test` passes. `bash scripts/pre-archive-rust-gates.sh` passes. All `src/changes/*.rs` files are under 500 lines.

---

## 2. Split `src/cli/mod.rs`

- [x] 2.1 Create `src/cli/commands.rs`. Move `run_shared_json_command`, `shared_request`, `shared_flags`, `shared_exit_code`, `run_hook_command`, `run_archive_command`, `parse_hook_kind`, `legacy_blueprint_warning`, `run_ui_command`, `requires_valid_map`, `init_project` into it. These are all `fn` items not called from outside `cli`; keep them `pub(super)` or `pub(crate)` as needed.
- [x] 2.2 Create `src/cli/render.rs`. Move `render_get`, `render_neighbourhood`, `render_files`, `render_todos`, `render_decisions`, `render_research`, `render_sources`, `render_rationale`, `render_status`, `render_dependencies` into it.
- [x] 2.3 Create `src/cli/format.rs`. Move `node_arg`, `render_node`, `render_findings`, `node_json`, `finding_json`, `todos_json`, `decisions_json`, `research_json`, `reviews_json`, `sources_json`, `neighbourhood_ids`, `research_for_nodes`, `sources_for_nodes`, `flag_value`, `parse_todo_status_filter`, `parse_decision_status_filter`, `todo_line`, `decision_line`, `research_line`, `review_line`, `source_line`, `todo_status`, `decision_status`, `review_type`, `source_verification`, `findings_output`, `finding_output`, `error_output`, `ok`, `err`, `string_array_json`, `lines`, `esc` into it.
- [x] 2.4 In `mod.rs`, add `mod commands; mod render; mod format;` and add `use` pulls as needed so `render_loaded_project_command` and `run_project_command` can call into the submodules.
- [x] 2.5 The existing `#[cfg(test)] mod tests` block in `mod.rs` calls `run()` and helpers. All helpers (`run_in`, `write_project`, `write_change`, `temp_root`, `TEST_CWD_LOCK`) stay in the test block in `mod.rs`. Confirm no test imports are broken.
- [x] 2.6 Remove the `// cairn:allow-large-module reason: scheduled-for-phase-7.5b-split` comment from `mod.rs`.
- [x] 2.7 Verify: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`, `bash scripts/pre-archive-rust-gates.sh` pass. All `src/cli/*.rs` files under 500 lines.

---

## 3. Split `src/query_api.rs`

- [x] 3.1 Create `src/query_api/` directory. Move `src/query_api.rs` to `src/query_api/mod.rs`. Rust resolves `mod query_api` to either layout automatically; no `src/lib.rs` change needed.
- [x] 3.2 Create `src/query_api/change_queries.rs`. Move `discover_changes`, `show_change`, `change_json` into it. Extract the archive/rename/changes/show dispatch arms from `execute_data` into a `dispatch_change_tool` function in `change_queries.rs` called from `execute_data` in `mod.rs`.
- [x] 3.3 Create `src/query_api/handlers.rs`. Move `neighbourhood_json`, `contract_json`, `single_contract_json`, `docstring_json`, `files_json`, `dependency_json`, `status_json`, `rationale_json`, `todos_response_json`, `decisions_response_json`, `research_response_json`, `sources_response_json`, `hook_json` into it.
- [x] 3.4 Create `src/query_api/serialise.rs`. Move `node_json`, `todo_json`, `decision_json`, `research_json`, `review_json`, `source_json`, `findings_json`, `neighbourhood_ids`, `research_for_nodes`, `sources_for_nodes`, `relevant_rules`, `requires_valid_map`, `findings_error`, `finding_error`, `command_error`, `load_for`, `required`, `parse_todo_status_filter`, `parse_decision_status_filter`, `todo_status`, `decision_status`, `source_verification`, `hook_kind_name`, `hook_decision_name` into it.
- [x] 3.5 The existing `#[cfg(test)] mod tests` block tests `registry()`, `visible_tools()`, `execute()`, `envelope_json()`, `error_json()`. All these remain in `mod.rs`. Test block stays in `mod.rs` unchanged.
- [x] 3.6 Remove the `// cairn:allow-large-module reason: scheduled-for-phase-7.5b-split` comment from `mod.rs`.
- [x] 3.7 Verify: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`, `bash scripts/pre-archive-rust-gates.sh` pass. All `src/query_api/*.rs` files under 500 lines.

---

## 4. Split `src/artefacts/registry.rs`

- [x] 4.1 Create `src/artefacts/registry/` directory. Move `src/artefacts/registry.rs` to `src/artefacts/registry/mod.rs`. Rust resolves `pub mod registry` to either layout automatically; no `src/artefacts/mod.rs` change needed.
- [x] 4.2 Create `src/artefacts/registry/types.rs`. Move all type definitions: `ArtefactType`, `ArtefactLoadRequest`, `ArtefactRecord`, `ArtefactError`, `ArtefactLoader`, `TodoStatus`, `Todo`, `DecisionStatus`, `Decision`, `ReviewType`, `Review`, `Research`, `SourceVerification`, `Source`, `ArtefactSet` into it. Add `pub use types::*;` in `mod.rs`.
- [x] 4.3 Create `src/artefacts/registry/sha256.rs`. Move `SHA256_INITIAL_STATE`, `SHA256_ROUND_CONSTANTS`, `sha256_hex`, `compress_sha256_block`, `sha256_schedule` into it. Used only by `validate.rs`; make it `pub(super)`.
- [x] 4.4 Create `src/artefacts/registry/parse.rs`. Move `parse_todo_status`, `parse_decision_status`, `parse_review_type`, `parse_source_verification` into it. Make them `pub(super)`.
- [x] 4.5 Create `src/artefacts/registry/io.rs`. Move `pointers`, `collect_pointers`, `collect_ids`, `collect_node_id`, `markdown_paths`, `read_dir_markdown`, `parse_file`, `required`, `optional`, `list`, `path_string`, `is_url`, `error`, `warning`, `error_finding` into it. Make them `pub(super)`.
- [x] 4.6 Create `src/artefacts/registry/validate.rs`. Move `validate_integrity`, `validate_nodes`, `validate_node_list`, `validate_decision_refs`, `validate_provenance_refs`, `validate_sources`, `validate_verified_source` into it. Uses `sha256::sha256_hex`, `io::*`, and `parse::*`.
- [x] 4.7 Keep `load_artefacts`, `load_todos`, `load_decisions`, `load_reviews`, `load_research`, `load_sources` in `mod.rs`.
- [x] 4.8 The existing `#[cfg(test)] mod tests` block tests `load_artefacts` and the `parse_*` functions. Add `use super::parse::{parse_todo_status, parse_decision_status, parse_review_type, parse_source_verification};` inside the test block. All assertions stay unchanged.
- [x] 4.9 Remove the `// cairn:allow-large-module reason: scheduled-for-phase-7.5b-split` comment from `mod.rs`.
- [x] 4.10 Verify: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`, `bash scripts/pre-archive-rust-gates.sh` pass. All `src/artefacts/registry/*.rs` files under 500 lines.

---

## 5. Split `src/ui.rs`

- [x] 5.1 Create `src/ui/` directory. Move `src/ui.rs` to `src/ui/mod.rs`.
- [x] 5.2 Create `src/ui/server.rs`. Move the routing and HTTP primitive functions and the `Response` type into it: `Server::handle_stream`, `Server::route`, `Server::api`, `Server::blueprint_json`, `Server::node_api`, `Server::load_project`, `read_http_request`, `Response`, `html`, `asset`, `json`, `text`, `write_response`, `request_path`, `open_browser`. Add a second `impl Server` block in `server.rs`; the `Server` struct definition stays in `mod.rs`.
- [x] 5.3 Create `src/ui/serialise.rs`. Move `kind_name`, `state_name`, `severity_name`, `graph_edge_kind_name`, `map_json`, `string_array_json`, `optional_json`, `percent_decode`, `esc` into it. These are pure formatting helpers with no I/O dependency.
- [x] 5.4 Create `src/ui/api.rs`. Move `meta_json`, `graph_json`, `node_json`, `dependency_json`, `lint_json`, `status_json`, `contract_response_json`, `contract_json`, `artefact_response_json`, `rationale_json`, `Artefact`, `collect_artefacts`, `collect_artefacts_from_dir`, `frontmatter_mentions_node`, `artefact_json`, `finding_json`, `project_finding`, `title_from_body` into it. Uses `serialise::*` functions.
- [x] 5.5 The existing `#[cfg(test)] mod tests` block calls `start_background`, `request_path`, `write_project`, `temp_root`. `start_background` stays in `mod.rs`. `request_path` moves to `server.rs`; add `use super::server::request_path;` inside the test block for the `test_request_path_supports_get_only` test. All assertions unchanged.
- [x] 5.6 Remove the `// cairn:allow-large-module reason: scheduled-for-phase-7.5b-split` comment from `mod.rs`.
- [x] 5.7 Verify: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`, `bash scripts/pre-archive-rust-gates.sh` pass. All `src/ui/*.rs` files under 500 lines.

---

## 6. Final Verification

- [x] 6.1 Confirm no `// cairn:allow-large-module` comment remains anywhere under `src/`: `grep -r "cairn:allow-large-module" src/` returns empty.
- [x] 6.2 `cargo build` passes with zero warnings.
- [x] 6.3 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [x] 6.4 `cargo fmt --check` passes.
- [x] 6.5 `cargo test` passes. Zero snapshot diffs (no `cargo insta review` needed).
- [x] 6.6 `bash scripts/pre-archive-rust-gates.sh` passes end-to-end. The file-size check reports no violations.
- [x] 6.7 Confirm all public import paths are unchanged: the `cairn` binary, `cairn-mcp` binary, and integration tests under `tests/` compile without modification.
