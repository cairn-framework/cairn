//! Snapshot tests for the graph explorer JSON wire format.

use std::{
    fs,
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    path::{Path, PathBuf},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use cairn::ui::{self, UiOptions};
use insta::assert_json_snapshot;
use serde_json::{Value, json};

#[test]
fn test_api_meta_snapshot() -> Result<(), Box<dyn std::error::Error>> {
    let server = bootstrap_server("meta")?;
    let json = get_json(server.address(), "/api/meta")?;
    server.stop();
    assert_json_snapshot!("api_meta", json);
    Ok(())
}

#[test]
fn test_api_status_snapshot() -> Result<(), Box<dyn std::error::Error>> {
    let server = bootstrap_server("status")?;
    let json = get_json(server.address(), "/api/status")?;
    server.stop();
    assert_json_snapshot!("api_status", json);
    Ok(())
}

#[test]
fn test_api_graph_snapshot() -> Result<(), Box<dyn std::error::Error>> {
    let server = bootstrap_server("graph")?;
    let json = get_json(server.address(), "/api/graph")?;
    server.stop();
    assert_json_snapshot!("api_graph", json);
    Ok(())
}

#[test]
fn test_api_lint_snapshot() -> Result<(), Box<dyn std::error::Error>> {
    let server = bootstrap_server("lint")?;
    let json = get_json(server.address(), "/api/lint")?;
    server.stop();
    assert_json_snapshot!("api_lint", json);
    Ok(())
}

#[test]
fn test_api_blueprint_snapshot() -> Result<(), Box<dyn std::error::Error>> {
    let server = bootstrap_server("blueprint")?;
    let mut json = get_json(server.address(), "/api/blueprint")?;
    server.stop();
    json["path"] = json!("<blueprint>");
    assert_json_snapshot!("api_blueprint", json);
    Ok(())
}

#[test]
fn test_api_node_snapshot() -> Result<(), Box<dyn std::error::Error>> {
    let server = bootstrap_server("node")?;
    let json = get_json(server.address(), "/api/node/app.api")?;
    server.stop();
    assert_json_snapshot!("api_node_app_api", json);
    Ok(())
}

#[test]
fn test_api_contract_snapshot() -> Result<(), Box<dyn std::error::Error>> {
    let server = bootstrap_server("contract")?;
    let json = get_json(server.address(), "/api/node/app.api/contract")?;
    server.stop();
    assert_json_snapshot!("api_node_app_api_contract", json);
    Ok(())
}

#[test]
fn test_api_decisions_snapshot() -> Result<(), Box<dyn std::error::Error>> {
    let server = bootstrap_server("decisions")?;
    let json = get_json(server.address(), "/api/node/app.api/decisions")?;
    server.stop();
    assert_json_snapshot!("api_node_app_api_decisions", json);
    Ok(())
}

#[test]
fn test_api_todos_snapshot() -> Result<(), Box<dyn std::error::Error>> {
    let server = bootstrap_server("todos")?;
    let json = get_json(server.address(), "/api/node/app.api/todos")?;
    server.stop();
    assert_json_snapshot!("api_node_app_api_todos", json);
    Ok(())
}

#[test]
fn test_api_research_snapshot() -> Result<(), Box<dyn std::error::Error>> {
    let server = bootstrap_server("research")?;
    let json = get_json(server.address(), "/api/node/app.api/research")?;
    server.stop();
    assert_json_snapshot!("api_node_app_api_research", json);
    Ok(())
}

#[test]
fn test_api_sources_snapshot() -> Result<(), Box<dyn std::error::Error>> {
    let server = bootstrap_server("sources")?;
    let json = get_json(server.address(), "/api/node/app.api/sources")?;
    server.stop();
    assert_json_snapshot!("api_node_app_api_sources", json);
    Ok(())
}

#[test]
fn test_api_rationale_snapshot() -> Result<(), Box<dyn std::error::Error>> {
    let server = bootstrap_server("rationale")?;
    let json = get_json(server.address(), "/api/node/app.api/rationale")?;
    server.stop();
    assert_json_snapshot!("api_node_app_api_rationale", json);
    Ok(())
}

#[test]
fn test_api_depends_snapshot() -> Result<(), Box<dyn std::error::Error>> {
    let server = bootstrap_server("depends")?;
    let json = get_json(server.address(), "/api/depends/app.api")?;
    server.stop();
    assert_json_snapshot!("api_depends_app_api", json);
    Ok(())
}

#[test]
fn test_api_dependents_snapshot() -> Result<(), Box<dyn std::error::Error>> {
    let server = bootstrap_server("dependents")?;
    let json = get_json(server.address(), "/api/dependents/app.api")?;
    server.stop();
    assert_json_snapshot!("api_dependents_app_api", json);
    Ok(())
}

fn bootstrap_server(name: &str) -> Result<ui::ServerHandle, Box<dyn std::error::Error>> {
    let root = temp_root(name)?;
    write_project(&root)?;
    ui::start_background(UiOptions {
        port: 0,
        no_open: true,
        blueprint_path: root.join("cairn.blueprint"),
    })
    .map_err(Into::into)
}

fn write_project(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(root.join("src/api"))?;
    fs::create_dir_all(root.join("src/core"))?;
    fs::create_dir_all(root.join("meta/contracts"))?;
    fs::create_dir_all(root.join("meta/todos"))?;
    fs::create_dir_all(root.join("meta/decisions"))?;
    fs::create_dir_all(root.join("meta/research"))?;
    fs::create_dir_all(root.join("meta/sources"))?;
    fs::create_dir_all(root.join(".cairn"))?;
    fs::write(root.join("src/api/lib.rs"), "pub fn serve() {}\n")?;
    fs::write(root.join("src/core/lib.rs"), "pub fn core() {}\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" @product {
    Module Core "core" id "app.core" @backend {
        path "./src/core"
    }
    Container Api "api" id "app.api" @backend {
        path "./src/api"
        contract "./meta/contracts/api.md"
        todos "./meta/todos"
        decisions "./meta/decisions"
        research "./meta/research"
        sources "./meta/sources"
    }
}
app.api -> app.core "reports"
app.core -> app.api "serves"
"#,
    )?;
    fs::write(
        root.join("meta/contracts/api.md"),
        "---\nnode: app.api\n---\n# API Contract\nGET /api/status returns health details.\n",
    )?;
    fs::write(
        root.join("meta/todos/todo.api.md"),
        "---\nnode: app.api\nstatus: open\ncreated: 2026-04-01\nsatisfies: status.contract\n---\n# API Todo\nShip the endpoint.\n",
    )?;
    fs::write(
        root.join("meta/decisions/dec.api.md"),
        "---\nid: dec.api\nnodes: [app.api]\nstatus: accepted\ndate: 2026-04-01\ninformed_by: [res.api]\n---\n# API Decision\nUse stable JSON payloads.\n",
    )?;
    fs::write(
        root.join("meta/research/res.api.md"),
        "---\nid: res.api\nnodes: [app.api]\ndate: 2026-03-20\nsources: [src.api]\ntags: [wire]\n---\n# API Research\nStudied payload evolution.\n",
    )?;
    fs::write(root.join("docs-source.txt"), "wire format source\n")?;
    fs::write(
        root.join("meta/sources/src.api.md"),
        "---\nid: src.api\nfile: docs-source.txt\nsha256: ecf5dae7a91b73f6faec1d386583345afe598f4b8af0d647f28f0b0f46f7c633\nverification: verified\ntype: note\ndate: 2026-03-19\ntags: [wire]\ndescription: bootstrap source\n---\n# API Source\nBootstrap evidence.\n",
    )?;
    fs::write(root.join(".cairn/log.md"), "- note: bootstrap log entry\n")?;
    Ok(())
}

fn get_json(address: SocketAddr, path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let body = get(address, path)?;
    Ok(serde_json::from_str(&body)?)
}

fn get(address: SocketAddr, path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut last_error = None;
    for _attempt in 0..10 {
        match get_once(address, path) {
            Ok(body) => return Ok(body),
            Err(error)
                if error
                    .downcast_ref::<std::io::Error>()
                    .is_some_and(|error| error.kind() == std::io::ErrorKind::ConnectionReset) =>
            {
                last_error = Some(error);
                thread::sleep(Duration::from_millis(20));
            }
            Err(error) => return Err(error),
        }
    }
    Err(last_error.unwrap_or_else(|| "request failed".into()))
}

fn get_once(address: SocketAddr, path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(address)?;
    write!(
        stream,
        "GET {path} HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\n\r\n"
    )?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    let Some((head, body)) = response.split_once("\r\n\r\n") else {
        return Err("missing http response body".into());
    };
    assert!(head.contains("200 OK"), "{head}");
    Ok(body.to_owned())
}

fn temp_root(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("cairn-snapshots-{name}-{suffix}"));
    fs::create_dir_all(&root)?;
    Ok(root)
}
