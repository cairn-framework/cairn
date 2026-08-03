//! Phase 2.5 graph explorer integration tests.

use std::{
    fmt::Write as FmtWrite,
    fs,
    io::{Read, Write as IoWrite},
    net::TcpStream,
    path::{Path, PathBuf},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use cairn::ui::{self, UiOptions};

#[test]
fn test_ui_port_zero_starts_and_serves_graph_api() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("ui-port-zero")?;
    write_project(&root)?;
    let server = ui::start_background(UiOptions {
        port: 0,
        no_open: true,
        blueprint_path: root.join("cairn.blueprint"),
    })?;

    let graph = get(server.address(), "/api/graph")?;
    let meta = get(server.address(), "/api/meta")?;
    let node = get(server.address(), "/api/node/app.api")?;

    server.stop();

    assert!(graph.contains("\"nodes\""));
    assert!(graph.contains("\"edges\""));
    assert!(graph.contains("\"kind\":\"Ownership\""));
    assert!(graph.contains("\"kind\":\"Dependency\""));
    assert!(graph.contains("\"kind\":\"Module\""));
    assert!(graph.contains("\"id\":\"app.api.lib\""));
    assert!(meta.contains("\"schema_version\":10"));
    assert!(meta.contains("\"name\":\"ui\""));
    let meta_json: serde_json::Value = serde_json::from_str(&meta)?;
    assert!(meta_json["last_reconciled"].is_u64());
    assert!(node.contains("\"id\":\"app.api\""));
    // Canonical query_api wire: owns_files/span present, Debug-case enums.
    assert!(node.contains("\"owns_files\""));
    assert!(node.contains("\"span\""));
    assert!(node.contains("\"kind\":\"Container\""));
    Ok(())
}
#[test]
fn test_ui_status_and_lint_endpoints_return_two_hundred() -> Result<(), Box<dyn std::error::Error>>
{
    let root = temp_root("ui-status-lint")?;
    write_project(&root)?;
    let server = ui::start_background(UiOptions {
        port: 0,
        no_open: true,
        blueprint_path: root.join("cairn.blueprint"),
    })?;
    let status = get(server.address(), "/api/status")?;
    let lint = get(server.address(), "/api/lint")?;
    server.stop();
    assert!(status.contains("\"open_todos\""));
    assert!(lint.contains("\"findings\""));
    Ok(())
}

#[test]
fn test_ui_frontier_endpoint_returns_ready_shape() -> Result<(), Box<dyn std::error::Error>> {
    // A dedicated acyclic fixture: a ghost module whose only dependency is
    // synced is buildable now, so the happy frontier wire shape carries it.
    let root = temp_root("ui-frontier")?;
    fs::create_dir_all(root.join("src/core"))?;
    fs::write(root.join("src/core/lib.rs"), "pub fn core() {}\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        "System App \"d\" id \"app\" @product {\n    Module Core \"c\" id \"app.core\" @backend {\n        path \"./src/core\"\n    }\n    Module Api \"a\" id \"app.api\" @backend {\n        path \"./src/api\"\n    }\n}\napp.api -> app.core \"calls\"\n",
    )?;
    let server = ui::start_background(UiOptions {
        port: 0,
        no_open: true,
        blueprint_path: root.join("cairn.blueprint"),
    })?;
    let frontier = get(server.address(), "/api/frontier")?;
    server.stop();
    assert!(frontier.contains("\"schema_version\":10"), "{frontier}");
    assert!(frontier.contains("\"ready\""), "{frontier}");
    assert!(frontier.contains("\"blocked\""), "{frontier}");
    assert!(
        frontier.contains("\"node\":\"app.api\""),
        "the ghost module with a synced dependency is buildable now: {frontier}"
    );
    Ok(())
}

#[test]
fn test_ui_pending_endpoint_serves_proposed_decisions() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("ui-pending")?;
    write_project(&root)?;
    fs::write(
        root.join("meta/decisions/proposed.md"),
        "---\nid: dec.proposed\nnodes: [app.api]\nstatus: proposed\ndate: 2026-04-02\n---\n# Proposed Decision\n",
    )?;
    let server = ui::start_background(UiOptions {
        port: 0,
        no_open: true,
        blueprint_path: root.join("cairn.blueprint"),
    })?;
    let pending = get(server.address(), "/api/pending")?;
    server.stop();
    // The queue lists the proposed decision and not the accepted fixture one,
    // with the same payload shape the CLI prints.
    assert!(pending.contains("\"pending\""), "{pending}");
    assert!(pending.contains("\"id\":\"dec.proposed\""), "{pending}");
    assert!(pending.contains("\"age_days\""), "{pending}");
    assert!(
        pending.contains("\"ratification\":\"binding\""),
        "{pending}"
    );
    assert!(!pending.contains("dec.api"), "{pending}");
    Ok(())
}

#[test]
fn test_ui_serves_static_assets_with_detail_behaviour() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("ui-static")?;
    write_project(&root)?;
    let server = ui::start_background(UiOptions {
        port: 0,
        no_open: true,
        blueprint_path: root.join("cairn.blueprint"),
    })?;

    let html = get(server.address(), "/")?;
    let js = get(server.address(), "/assets/app.js")?;
    let module_assets = [
        "utils.js",
        "search.js",
        "app-data.js",
        "canvas-nav.js",
        "graph-layout.js",
        "status-bezel.js",
        "query-rail.js",
        "graph-workspace.js",
        "evidence-rail.js",
        "node-module.js",
        "channel-bar.js",
    ]
    .iter()
    .map(|name| get(server.address(), &format!("/assets/{name}")))
    .collect::<Result<Vec<_>, _>>()?;
    let copy_json = get(server.address(), "/assets/copy.json")?;
    let preact = get(server.address(), "/vendor/preact.min.js")?;
    let htm = get(server.address(), "/vendor/htm.min.js")?;

    server.stop();

    // Shell markers from the instrument-workspace app frame.
    assert!(html.contains("id=\"root\""));
    assert!(html.contains("/vendor/preact.min.js"));
    assert!(html.contains("/assets/app.js"));

    // App entry points: composition root wires live-data fetch helpers and
    // renders the instrument shell; the evidence rail is served as its own
    // feature module.
    assert!(
        js.contains("fetchGraph")
            || module_assets
                .iter()
                .any(|asset| asset.contains("fetchGraph"))
    );
    assert!(js.contains("instrument-shell"));
    assert!(
        module_assets
            .iter()
            .any(|asset| asset.contains("evidence-rail") || asset.contains("EvidenceRail"))
    );

    // Copy data served as valid JSON with expected structure.
    let parsed: serde_json::Value =
        serde_json::from_str(&copy_json).expect("/assets/copy.json must return valid JSON");

    assert!(
        parsed["empty-states"]["no-findings"]["body"].is_string(),
        "copy.json must contain empty-states.no-findings.body"
    );
    // Vendored runtime delivered alongside static assets.
    assert!(preact.contains("self.preact"));
    assert!(htm.contains("self.htm"));

    Ok(())
}

#[test]
fn test_ui_query_bridge_serves_artefact_layers() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("ui-artefacts")?;
    write_project(&root)?;
    let server = ui::start_background(UiOptions {
        port: 0,
        no_open: true,
        blueprint_path: root.join("cairn.blueprint"),
    })?;

    let contract = get(server.address(), "/api/node/app.api/contract")?;
    let decisions = get(server.address(), "/api/node/app.api/decisions")?;
    let rationale = get(server.address(), "/api/node/app.api/rationale")?;

    server.stop();

    assert!(contract.contains("API Contract"));
    assert!(decisions.contains("API Decision"));
    assert!(rationale.contains("API Decision"));

    Ok(())
}

#[test]
fn test_ui_lint_endpoint_reports_structural_badge_data() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("ui-lint")?;
    fs::write(
        root.join("cairn.blueprint"),
        "System App \"desc\" id \"app\" {\n    Module A \"a\" id \"app.a\" {}\n    Module B \"b\" id \"app.a\" {}\n}\n",
    )?;
    let server = ui::start_background(UiOptions {
        port: 0,
        no_open: true,
        blueprint_path: root.join("cairn.blueprint"),
    })?;

    let lint = get(server.address(), "/api/lint")?;

    server.stop();

    assert!(lint.contains("CAIRN_INTEGRITY_DUPLICATE_ID"));
    assert!(lint.contains("\"node\":\"app.a\""));

    Ok(())
}

#[test]
fn test_ui_large_graph_api_serves_two_hundred_nodes() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("ui-large")?;
    let mut blueprint = String::from("System App \"desc\" id \"app\" {\n");
    for container in 0..10 {
        writeln!(
            blueprint,
            "    Container C{container} \"container\" id \"app.c{container}\" {{"
        )?;
        for module in 0..20 {
            writeln!(
                blueprint,
                "        Module M{container}_{module} \"module\" id \"app.c{container}.m{module}\" {{}}"
            )?;
        }
        blueprint.push_str("    }\n");
    }
    blueprint.push_str("}\n");
    fs::write(root.join("cairn.blueprint"), blueprint)?;
    let server = ui::start_background(UiOptions {
        port: 0,
        no_open: true,
        blueprint_path: root.join("cairn.blueprint"),
    })?;

    let started = std::time::Instant::now();
    let graph = get(server.address(), "/api/graph")?;
    let elapsed = started.elapsed();

    server.stop();

    assert!(graph.matches("\"kind\":\"Module\"").count() >= 200);
    assert!(elapsed.as_secs_f32() < 2.0);

    Ok(())
}

fn write_project(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(root.join("src/api"))?;
    fs::create_dir_all(root.join("meta/contracts"))?;
    fs::create_dir_all(root.join("meta/decisions"))?;
    fs::write(root.join("src/api/lib.rs"), "pub fn serve() {}\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" @product {
    Container Api "api" id "app.api" @backend {
        Module Lib "lib" id "app.api.lib" {
            path "./src/api/lib.rs"
        }
        contract "./meta/contracts/api.md"
        decisions "./meta/decisions"
    }
}
app.api -> app "reports"
"#,
    )?;
    fs::write(
        root.join("meta/contracts/api.md"),
        "---\nnode: app.api\n---\n# API Contract\n",
    )?;
    fs::write(
        root.join("meta/decisions/api.md"),
        "---\nid: dec.api\nnodes: [app.api]\nstatus: accepted\ndate: 2026-04-01\n---\n# API Decision\n",
    )?;
    Ok(())
}

fn get(address: std::net::SocketAddr, path: &str) -> Result<String, Box<dyn std::error::Error>> {
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

fn get_once(
    address: std::net::SocketAddr,
    path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
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
    let root = std::env::temp_dir().join(format!("cairn-{name}-{suffix}"));
    fs::create_dir_all(&root)?;
    Ok(root)
}
