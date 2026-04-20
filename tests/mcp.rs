//! Integration tests for the Cairn MCP wrapper.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::{Value, json};

fn temp_root(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("cairn-mcp-{name}-{stamp}"));
    fs::create_dir_all(&root)?;
    Ok(root)
}

fn fixture(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let root = temp_root(name)?;
    fs::create_dir_all(root.join("src/auth"))?;
    fs::create_dir_all(root.join("src/store"))?;
    fs::create_dir_all(root.join("meta/contracts"))?;
    fs::create_dir_all(root.join("meta/todos"))?;
    fs::create_dir_all(root.join("meta/decisions"))?;
    fs::write(root.join("src/auth/lib.rs"), "pub fn authenticate() {}\n")?;
    fs::write(root.join("src/store/lib.rs"), "pub fn save() {}\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"
System App "Application" id "app" {
    Module Auth "Authentication" id "app.auth" @security {
        path "./src/auth"
        contract "./meta/contracts/auth.md"
        todos "./meta/todos"
        decisions "./meta/decisions"
    }
    Module Store "Storage" id "app.store" {
        path "./src/store"
    }
}
app.auth -> app.store "persists sessions"
"#,
    )?;
    fs::write(
        root.join("cairn.config.yaml"),
        r"context: |
  Agents should preserve the authority chain.
rules:
  decision: |
    Decisions touching security need review.
",
    )?;
    fs::write(
        root.join("meta/contracts/auth.md"),
        "---\nnode: app.auth\n---\n# Auth Contract\n",
    )?;
    fs::write(
        root.join("meta/todos/todo.md"),
        "---\nnode: app.auth\nstatus: open\ncreated: 2026-04-20\n---\n# Todo\n",
    )?;
    fs::write(
        root.join("meta/decisions/dec.auth.md"),
        "---\nid: dec.auth\nnodes: [app.auth]\nstatus: accepted\ndate: 2026-04-20\n---\n# Decision\n",
    )?;
    Ok(root)
}

#[test]
fn registry_lists_read_only_tools_without_mutations() {
    let tools = cairn::query_api::visible_tools(false);
    assert!(tools.iter().any(|tool| tool.mcp_name == "cairn_get"));
    assert!(
        tools
            .iter()
            .any(|tool| tool.mcp_name == "cairn_show_change")
    );
    assert!(!tools.iter().any(|tool| tool.mcp_name == "cairn_scan"));
    assert!(!tools.iter().any(|tool| tool.mcp_name == "cairn_archive"));
    assert!(!tools.iter().any(|tool| tool.mcp_name == "cairn_rename"));
}

#[test]
fn cli_json_and_library_query_share_get_data() -> Result<(), Box<dyn std::error::Error>> {
    let root = fixture("equivalence")?;
    let cli = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["--json", "get", "app.auth"])
        .output()?;
    assert!(cli.status.success());
    let mut cli_json: Value = serde_json::from_slice(&cli.stdout)?;
    let request = cairn::query_api::QueryRequest {
        tool: "get".to_owned(),
        node: Some("app.auth".to_owned()),
        ..cairn::query_api::QueryRequest::default()
    };
    let mut response = cairn::query_api::execute(
        &root,
        &root.join("cairn.blueprint"),
        &root.join("meta/changes"),
        &request,
    )?
    .data;
    cli_json["span"]["file"] = Value::Null;
    response["span"]["file"] = Value::Null;
    assert_eq!(cli_json, response);
    Ok(())
}

#[test]
fn mcp_response_composes_context_and_relevant_rules() -> Result<(), Box<dyn std::error::Error>> {
    let root = fixture("context")?;
    let config = mcp_config(&root);
    let line = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "cairn_decisions",
            "arguments": { "node": "app.auth" }
        }
    })
    .to_string();
    let response = cairn::mcp::handle_line(&line, &config);
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .expect("tool response text");
    let envelope: Value = serde_json::from_str(text)?;
    assert_eq!(
        envelope["project_context"],
        "Agents should preserve the authority chain."
    );
    assert_eq!(
        envelope["rules"]["decision"],
        "Decisions touching security need review."
    );
    assert_eq!(envelope["data"]["decisions"][0]["id"], "dec.auth");
    Ok(())
}

#[test]
fn mcp_response_uses_empty_context_and_rules_without_config()
-> Result<(), Box<dyn std::error::Error>> {
    let root = fixture("missing-config")?;
    fs::remove_file(root.join("cairn.config.yaml"))?;
    let config = mcp_config(&root);
    let line = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "cairn_get",
            "arguments": { "node": "app.auth" }
        }
    })
    .to_string();
    let response = cairn::mcp::handle_line(&line, &config);
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .expect("tool response text");
    let envelope: Value = serde_json::from_str(text)?;
    assert_eq!(envelope["project_context"], "");
    assert_eq!(envelope["rules"], json!({}));
    Ok(())
}

#[test]
fn mcp_tool_list_is_registry_backed_and_gates_mutations() -> Result<(), Box<dyn std::error::Error>>
{
    let root = fixture("list")?;
    let config = mcp_config(&root);
    let response =
        cairn::mcp::handle_line(r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#, &config);
    let tools = response["result"]["tools"].as_array().expect("tools array");
    assert!(tools.iter().any(|tool| tool["name"] == "cairn_get"));
    assert!(!tools.iter().any(|tool| tool["name"] == "cairn_archive"));

    let mut mutating = config.clone();
    mutating.allow_mutating_tools = true;
    let response = cairn::mcp::handle_line(
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#,
        &mutating,
    );
    let tools = response["result"]["tools"].as_array().expect("tools array");
    assert!(tools.iter().any(|tool| tool["name"] == "cairn_archive"));
    assert!(tools.iter().any(|tool| tool["name"] == "cairn_rename"));
    Ok(())
}

#[test]
fn mcp_missing_node_returns_stable_structured_error() -> Result<(), Box<dyn std::error::Error>> {
    let root = fixture("missing-node")?;
    let config = mcp_config(&root);
    let line = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "cairn_get",
            "arguments": { "node": "missing.node" }
        }
    })
    .to_string();
    let response = cairn::mcp::handle_line(&line, &config);
    assert_eq!(
        response["error"]["data"]["code"],
        "CAIRN_QUERY_NODE_NOT_FOUND"
    );
    assert!(
        response["error"]["data"]["message"]
            .as_str()
            .unwrap_or_default()
            .contains("missing.node")
    );
    Ok(())
}

fn mcp_config(root: &Path) -> cairn::mcp::ServerConfig {
    cairn::mcp::ServerConfig {
        root: root.to_path_buf(),
        blueprint_path: PathBuf::from("cairn.blueprint"),
        changes_dir: PathBuf::from("meta/changes"),
        allow_mutating_tools: false,
    }
}
