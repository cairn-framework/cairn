//! Tests for the MCP stdio server.

use super::*;

// ── handle_line ──────────────────────────────────────────────────────────

#[test]
fn test_handle_line_invalid_json_returns_parse_error() {
    let config = ServerConfig::default();
    let response = handle_line("not json", &config);
    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["error"]["code"], -32700);
    assert_eq!(response["id"], Value::Null);
    assert_eq!(response["error"]["data"]["code"], "CAIRN_MCP_PARSE_ERROR");
}

#[test]
fn test_handle_line_empty_string_returns_parse_error() {
    let config = ServerConfig::default();
    let response = handle_line("", &config);
    assert_eq!(response["error"]["code"], -32700);
}

#[test]
fn test_handle_line_initialize_echoes_id_and_returns_capabilities() {
    let config = ServerConfig::default();
    let response = handle_line(
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#,
        &config,
    );
    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 1);
    assert_eq!(response["result"]["protocolVersion"], "2024-11-05");
    assert!(response["result"]["capabilities"]["tools"].is_object());
    assert_eq!(response["result"]["serverInfo"]["name"], "cairn-mcp");
}

#[test]
fn test_handle_line_tools_list_returns_tools_array() {
    let config = ServerConfig::default();
    let response = handle_line(r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#, &config);
    assert_eq!(response["id"], 2);
    assert!(response["result"]["tools"].is_array());
    assert!(!response["result"]["tools"].as_array().unwrap().is_empty());
}

#[test]
fn test_handle_line_unknown_method_returns_method_not_found() {
    let config = ServerConfig::default();
    let response = handle_line(
        r#"{"jsonrpc":"2.0","id":3,"method":"unknown/method"}"#,
        &config,
    );
    assert_eq!(response["id"], 3);
    assert_eq!(response["error"]["code"], -32601);
    assert_eq!(
        response["error"]["data"]["code"],
        "CAIRN_MCP_METHOD_NOT_FOUND"
    );
}

#[test]
fn test_handle_line_tools_call_missing_name_returns_invalid_params() {
    let config = ServerConfig::default();
    // params present but no "name" field
    let response = handle_line(
        r#"{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"arguments":{}}}"#,
        &config,
    );
    assert_eq!(response["id"], 4);
    assert_eq!(response["error"]["code"], -32602);
    assert_eq!(response["error"]["data"]["code"], "CAIRN_MCP_MISSING_TOOL");
}

#[test]
fn test_handle_line_id_null_when_absent_from_request() {
    let config = ServerConfig::default();
    let response = handle_line(
        r#"{"jsonrpc":"2.0","method":"initialize","params":{}}"#,
        &config,
    );
    assert_eq!(response["id"], Value::Null);
}

// ── config_from_args ────────────────────────────────────────────────────

#[test]
fn test_config_from_args_empty_returns_default() {
    let config = config_from_args(&[]).unwrap();
    assert_eq!(config, ServerConfig::default());
}

#[test]
fn test_config_from_args_root_sets_root_path() {
    let config = config_from_args(&["--root".to_owned(), "/tmp/proj".to_owned()]).unwrap();
    assert_eq!(config.root, PathBuf::from("/tmp/proj"));
}

#[test]
fn test_config_from_args_root_missing_value_returns_err() {
    let result = config_from_args(&["--root".to_owned()]);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("--root"));
}

#[test]
fn test_config_from_args_file_sets_blueprint_path() {
    let config = config_from_args(&["--file".to_owned(), "cairn.blueprint".to_owned()]).unwrap();
    assert_eq!(config.blueprint_path, PathBuf::from("cairn.blueprint"));
}

#[test]
fn test_config_from_args_changes_dir_sets_path() {
    let config =
        config_from_args(&["--changes-dir".to_owned(), "meta/changes".to_owned()]).unwrap();
    assert_eq!(config.changes_dir, PathBuf::from("meta/changes"));
}

#[test]
fn test_config_from_args_allow_mutating_tools_flag() {
    let config = config_from_args(&["--allow-mutating-tools".to_owned()]).unwrap();
    assert!(config.allow_mutating_tools);
}

#[test]
fn test_config_from_args_unknown_option_returns_err() {
    let result = config_from_args(&["--unknown".to_owned()]);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("unknown option"));
}

#[test]
fn test_config_from_args_help_returns_err_with_usage() {
    let err = config_from_args(&["--help".to_owned()]).unwrap_err();
    assert!(err.contains("usage:"));
}

// ── request_from_arguments ────────────────────────────────────────────────

#[test]
fn test_request_from_arguments_node_field_used_directly() {
    let args = json!({ "node": "app.api" });
    let req = request_from_arguments("get", &args, false);
    assert_eq!(req.node.as_deref(), Some("app.api"));
}

#[test]
fn test_request_from_arguments_id_field_is_alias_for_node() {
    // Older clients sent "id" instead of "node". Must still work.
    let args = json!({ "id": "app.api" });
    let req = request_from_arguments("get", &args, false);
    assert_eq!(req.node.as_deref(), Some("app.api"));
}

#[test]
fn test_request_from_arguments_node_wins_over_id_alias() {
    let args = json!({ "node": "app.api", "id": "app.db" });
    let req = request_from_arguments("get", &args, false);
    assert_eq!(req.node.as_deref(), Some("app.api"));
}

#[test]
fn test_request_from_arguments_status_alias_kind() {
    let args = json!({ "kind": "accepted" });
    let req = request_from_arguments("decisions", &args, false);
    assert_eq!(req.status.as_deref(), Some("accepted"));
}

#[test]
fn test_request_from_arguments_mutating_requires_allow_flag() {
    let args = json!({ "mutating": true });
    // Without the server-level allow flag, mutating stays false.
    let req = request_from_arguments("some-tool", &args, false);
    assert!(!req.mutating);
    // With the server-level allow flag AND the per-call flag, mutating is true.
    let req = request_from_arguments("some-tool", &args, true);
    assert!(req.mutating);
}

// ── input_schema ────────────────────────────────────────────────────────

/// `HookRequest` uses request.status (mapped from "kind") to select the
/// hook class. The schema must document "kind" so MCP clients can discover it.
#[test]
fn test_input_schema_hook_request_documents_kind_field() {
    let schema = input_schema("HookRequest");
    assert!(
        schema["properties"]["kind"].is_object(),
        "HookRequest schema must include 'kind' property; got: {schema}"
    );
}

/// `WatchRequest` is a no-argument schema — properties MUST be a present
/// (possibly empty) object, not absent. All registered schemas must resolve
/// to a non-null object from `input_schema` (the `_` wildcard returning `{}`
/// is correct only for zero-argument tools).
#[test]
fn test_input_schema_returns_object_for_every_registered_schema() {
    use crate::query_api;
    for tool in query_api::registry() {
        let schema = input_schema(tool.request_schema);
        assert!(
            schema["properties"].is_object(),
            "input_schema({:?}) must return an object with 'properties'; got: {schema}",
            tool.request_schema
        );
    }
}

// ── resolve_path ────────────────────────────────────────────────────────

#[test]
fn test_resolve_path_relative_is_joined_with_root() {
    let root = Path::new("/project");
    let result = resolve_path(root, Path::new("cairn.blueprint"));
    assert_eq!(result, PathBuf::from("/project/cairn.blueprint"));
}

#[test]
fn test_resolve_path_absolute_is_returned_unchanged() {
    let root = Path::new("/project");
    let result = resolve_path(root, Path::new("/absolute/blueprint"));
    assert_eq!(result, PathBuf::from("/absolute/blueprint"));
}
