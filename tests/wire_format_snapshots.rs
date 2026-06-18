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

/// (`snapshot_name`, `api_path`)
const ENDPOINTS: &[(&str, &str)] = &[
    ("api_meta", "/api/meta"),
    ("api_status", "/api/status"),
    ("api_graph", "/api/graph"),
    ("api_lint", "/api/lint"),
    ("api_blueprint", "/api/blueprint"),
    ("api_node_app_api", "/api/node/app.api"),
    ("api_node_app_api_contract", "/api/node/app.api/contract"),
    ("api_node_app_api_decisions", "/api/node/app.api/decisions"),
    ("api_node_app_api_todos", "/api/node/app.api/todos"),
    ("api_node_app_api_research", "/api/node/app.api/research"),
    ("api_node_app_api_sources", "/api/node/app.api/sources"),
    ("api_node_app_api_rationale", "/api/node/app.api/rationale"),
    ("api_depends_app_api", "/api/depends/app.api"),
    ("api_dependents_app_api", "/api/dependents/app.api"),
];

#[test]
fn wire_format_snapshots() -> Result<(), Box<dyn std::error::Error>> {
    for (snapshot_name, api_path) in ENDPOINTS {
        let root = temp_root(snapshot_name)?;
        write_project(&root)?;
        let server = ui::start_background(UiOptions {
            port: 0,
            no_open: true,
            blueprint_path: root.join("cairn.blueprint"),
        })?;
        let mut value = get_json(server.address(), api_path)?;
        server.stop();

        // Normalise the unstable file-system path in the blueprint response.
        if *snapshot_name == "api_blueprint" {
            value["path"] = json!("<blueprint>");
        }

        assert_json_snapshot!(*snapshot_name, value);
    }
    Ok(())
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
    fs::write(
        root.join("src/api/lib.rs"),
        "pub fn serve() {}\n#[cfg(test)]\nmod tests {}\n",
    )?;
    fs::write(
        root.join("src/core/lib.rs"),
        "pub fn core() {}\n#[cfg(test)]\nmod tests {}\n",
    )?;
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
