//! Tests for query API request handling and envelope formatting.

use super::*;

#[test]
fn test_registry_lists_known_tools() {
    let tools = registry();
    assert!(!tools.is_empty());
    assert!(tools.iter().any(|t| t.cli_name == "get"));
    assert!(tools.iter().any(|t| t.cli_name == "scan"));
}

#[test]
fn test_visible_tools_filter_mutating_entries() {
    let visible = visible_tools(false);
    assert!(visible.iter().all(|t| t.safety == SafetyClass::ReadOnly));
}

#[test]
fn test_envelope_json_wraps_response() {
    let response = QueryResponse {
        project_context: "ctx".to_owned(),
        rules: BTreeMap::new(),
        data: json!({"key": "value"}),
        findings: Vec::new(),
    };
    let envelope = envelope_json(&response);
    assert_eq!(
        envelope.get("project_context").unwrap().as_str().unwrap(),
        "ctx"
    );
    assert!(envelope.get("data").is_some());
}

#[test]
fn test_error_json_includes_optional_fields() {
    let error = QueryError {
        code: "TEST".to_owned(),
        message: "msg".to_owned(),
        source_span: Some("span".to_owned()),
        remediation: Some("fix".to_owned()),
    };
    let json = error_json(&error);
    assert_eq!(json.get("code").unwrap().as_str().unwrap(), "TEST");
    assert_eq!(json.get("source_span").unwrap().as_str().unwrap(), "span");
    assert_eq!(json.get("remediation").unwrap().as_str().unwrap(), "fix");
}

#[test]
fn test_execute_rejects_unknown_or_invalid_requests() {
    let request = QueryRequest {
        tool: "nonexistent".to_owned(),
        ..QueryRequest::default()
    };
    let result = execute(
        Path::new("."),
        Path::new("cairn.blueprint"),
        Path::new("meta/changes"),
        &request,
    );
    assert!(result.is_err());
}

#[test]
fn test_execute_returns_node_json_for_valid_request() {
    let tmp = std::env::temp_dir().join(format!("cairn-test-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&tmp);
    let _ = std::fs::write(
        tmp.join("cairn.blueprint"),
        "System Test \"T\" id \"t\" {\n}\n",
    );
    let request = QueryRequest {
        tool: "status".to_owned(),
        ..QueryRequest::default()
    };
    let result = execute(
        &tmp,
        &tmp.join("cairn.blueprint"),
        &tmp.join("meta/changes"),
        &request,
    );
    assert!(
        result.is_ok(),
        "execute must succeed for valid request: {result:?}"
    );
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn test_registry_includes_watch_tool() {
    let tools = registry();
    let watch = tools.iter().find(|t| t.cli_name == "watch");
    assert!(watch.is_some(), "registry must include watch tool");
    let watch = watch.unwrap();
    assert_eq!(watch.mcp_name, "cairn_watch");
    assert_eq!(watch.safety, SafetyClass::ReadOnly);
}

#[test]
fn test_execute_watch_returns_finding_added_events() {
    let tmp = std::env::temp_dir().join(format!("cairn-watch-test-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&tmp);
    // Blueprint referencing a missing contract produces a finding.
    let _ = std::fs::write(
        tmp.join("cairn.blueprint"),
        "System Test \"T\" id \"t\" {\n    Module M \"M\" id \"t.m\" {\n        contract \"meta/contracts/m.md\"\n    }\n}\n",
    );
    let request = QueryRequest {
        tool: "watch".to_owned(),
        ..QueryRequest::default()
    };
    let result = execute(
        &tmp,
        &tmp.join("cairn.blueprint"),
        &tmp.join("meta/changes"),
        &request,
    );
    assert!(result.is_ok(), "watch execute must succeed: {result:?}");
    let response = result.unwrap();
    let events = response
        .data
        .get("events")
        .expect("response must have events array");
    assert!(events.is_array(), "events must be an array");
    let arr = events.as_array().unwrap();
    assert!(
        !arr.is_empty(),
        "watch should emit at least one finding_added event"
    );
    for ev in arr {
        assert_eq!(ev.get("event").unwrap().as_str(), Some("finding_added"));
        assert!(ev.get("timestamp").is_some());
        assert!(ev.get("finding").is_some());
    }
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn test_registry_includes_health_tool() {
    let tools = registry();
    let health = tools.iter().find(|t| t.cli_name == "health");
    assert!(health.is_some(), "registry must include health tool");
    let health = health.unwrap();
    assert_eq!(health.mcp_name, "cairn_health");
    assert_eq!(health.safety, SafetyClass::ReadOnly);
}

#[test]
fn test_execute_health_returns_structured_response() {
    let tmp = std::env::temp_dir().join(format!("cairn-health-test-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&tmp);
    let _ = std::fs::write(
        tmp.join("cairn.blueprint"),
        "System Test \"T\" id \"t\" {\n}\n",
    );
    let request = QueryRequest {
        tool: "health".to_owned(),
        ..QueryRequest::default()
    };
    let result = execute(
        &tmp,
        &tmp.join("cairn.blueprint"),
        &tmp.join("meta/changes"),
        &request,
    );
    assert!(result.is_ok(), "health execute must succeed: {result:?}");
    let response = result.unwrap();
    assert!(response.data.get("clean").is_some());
    assert!(response.data.get("summary").is_some());
    assert!(response.data.get("lint").is_some());
    assert!(response.data.get("hooks").is_some());
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn test_registry_includes_remediate_tool() {
    let tools = registry();
    let remediate = tools.iter().find(|t| t.cli_name == "remediate");
    assert!(remediate.is_some(), "registry must include remediate tool");
    let remediate = remediate.unwrap();
    assert_eq!(remediate.mcp_name, "cairn_remediate");
    assert_eq!(remediate.safety, SafetyClass::ReadOnly);
}

#[test]
fn test_execute_remediate_returns_action_plan() {
    let tmp = std::env::temp_dir().join(format!("cairn-remediate-test-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&tmp);
    let _ = std::fs::write(
        tmp.join("cairn.blueprint"),
        "System Test \"T\" id \"t\" {\n}\n",
    );
    let request = QueryRequest {
        tool: "remediate".to_owned(),
        ..QueryRequest::default()
    };
    let result = execute(
        &tmp,
        &tmp.join("cairn.blueprint"),
        &tmp.join("meta/changes"),
        &request,
    );
    assert!(result.is_ok(), "remediate execute must succeed: {result:?}");
    let response = result.unwrap();
    assert!(response.data.get("actions").is_some());
    assert!(response.data.get("total_actions").is_some());
    let _ = std::fs::remove_dir_all(&tmp);
}

// ── requires_valid_map (serialise path) ───────────────────────────────────

#[test]
fn test_requires_valid_map_neighbourhood_missing_from_mcp_path() {
    // serialise::requires_valid_map is the gate used by the MCP/query_api
    // path.  It was missing "neighbourhood" even after the CLI path was
    // fixed — the two parallel copies diverged.
    assert!(
        requires_valid_map("neighbourhood"),
        "neighbourhood must require a valid map on the MCP path too"
    );
}
