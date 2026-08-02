// cairn:allow-large-module reason: ui module hosts integration tests for the embedded web server.
//! Embedded HTTP server and browser UI for graph exploration.

use std::{
    error::Error,
    fmt, fs,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    path::{Path, PathBuf},
    process::Command,
    sync::{
        Arc, LazyLock,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

use crate::scanner;

mod server;
mod wire;

use server::{Server, open_browser};

const INDEX_HTML: &str = include_str!("../ui_assets/index.html");
const APP_JS: &str = include_str!("../ui_assets/app.js");
const UTILS_JS: &str = include_str!("../ui_assets/utils.js");
const SEARCH_JS: &str = include_str!("../ui_assets/search.js");
const CANVAS_NAV_JS: &str = include_str!("../ui_assets/canvas-nav.js");
const APP_DATA_JS: &str = include_str!("../ui_assets/app-data.js");
const GRAPH_LAYOUT_JS: &str = include_str!("../ui_assets/graph-layout.js");
const STATUS_BEZEL_JS: &str = include_str!("../ui_assets/status-bezel.js");
const QUERY_RAIL_JS: &str = include_str!("../ui_assets/query-rail.js");
const GRAPH_WORKSPACE_JS: &str = include_str!("../ui_assets/graph-workspace.js");
const EVIDENCE_RAIL_JS: &str = include_str!("../ui_assets/evidence-rail.js");
const NODE_MODULE_JS: &str = include_str!("../ui_assets/node-module.js");
const CHANNEL_BAR_JS: &str = include_str!("../ui_assets/channel-bar.js");
/// Canonical design-system tokens; single source of truth.
const DESIGN_TOKENS_CSS: &str = include_str!("../../docs/design-system/tokens.css");
/// Canonical design-system component primitives.
const DESIGN_COMPONENTS_CSS: &str = include_str!("../../docs/design-system/components.css");
/// Graph-explorer-specific layout and overrides.
const UI_STYLE_CSS: &str = include_str!("../ui_assets/style.css");

/// Vendored Preact runtime (UMD).
const VENDOR_PREACT_JS: &str = include_str!("../ui_assets/vendor/preact.min.js");
/// Vendored Preact hooks (UMD, depends on Preact global).
const VENDOR_PREACT_HOOKS_JS: &str = include_str!("../ui_assets/vendor/preact-hooks.min.js");
/// Vendored htm tagged-template helper (UMD).
const VENDOR_HTM_JS: &str = include_str!("../ui_assets/vendor/htm.min.js");

/// Concatenated stylesheet served as `/assets/style.css`: tokens, then canonical
/// components, then the graph-explorer-specific layer. Consumers read tokens
/// via `var(--...)` so the three layers compose in definition order.
static STYLE_CSS: LazyLock<String> = LazyLock::new(|| {
    let mut combined = String::with_capacity(
        DESIGN_TOKENS_CSS.len() + DESIGN_COMPONENTS_CSS.len() + UI_STYLE_CSS.len() + 128,
    );
    combined.push_str("/* Cairn Graph Explorer stylesheet.\n");
    combined.push_str(
        "   Concatenated: design-system tokens, design-system components, ui overrides.\n",
    );
    combined
        .push_str("   Single source of truth for tokens is docs/design-system/tokens.css. */\n");
    combined.push_str(DESIGN_TOKENS_CSS);
    combined.push_str("\n/* ---- design-system components ---- */\n");
    combined.push_str(DESIGN_COMPONENTS_CSS);
    combined.push_str("\n/* ---- graph-explorer overrides ---- */\n");
    combined.push_str(UI_STYLE_CSS);
    combined
});

/// Copy data from design-system/copy.toml, served as JSON for the webui.
static COPY_JSON: LazyLock<String> = LazyLock::new(|| {
    let table: toml::Table = include_str!("../../docs/design-system/copy.toml")
        .parse()
        .expect("copy.toml must be valid TOML");
    serde_json::to_string(&table).expect("TOML table must serialise to JSON")
});

const SCHEMA_VERSION: u32 = 9;

/// Runtime options for the graph explorer server.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiOptions {
    /// Requested local port. Port `0` asks the OS for an available port.
    pub port: u16,
    /// Whether browser opening is disabled.
    pub no_open: bool,
    /// Cairn blueprint path.
    pub blueprint_path: PathBuf,
}

impl Default for UiOptions {
    fn default() -> Self {
        Self {
            port: 3000,
            no_open: false,
            blueprint_path: PathBuf::from("cairn.blueprint"),
        }
    }
}

impl UiOptions {
    /// Parses `cairn ui` command arguments.
    ///
    /// # Errors
    ///
    /// Returns a human-readable message when an option is malformed.
    pub fn from_args(args: &[String]) -> Result<Self, String> {
        let mut options = Self::default();
        let mut iter = args.iter().skip(1);
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--port" => {
                    let Some(value) = iter.next() else {
                        return Err("--port requires a value".to_owned());
                    };
                    options.port = value
                        .parse::<u16>()
                        .map_err(|_| format!("invalid port `{value}`"))?;
                }
                "--no-open" => options.no_open = true,
                value => return Err(format!("unknown ui option `{value}`")),
            }
        }
        Ok(options)
    }
}

/// Graph explorer server error.
#[derive(Debug)]
pub enum UiError {
    /// Port binding failed.
    Bind {
        /// Requested port.
        port: u16,
        /// Source I/O error.
        source: std::io::Error,
    },
    /// I/O failed while serving a request.
    Io(std::io::Error),
    /// Project loading failed.
    Project(String),
    /// Ctrl+C handler installation failed.
    ShutdownHandler(String),
}

impl fmt::Display for UiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bind { port, source } => {
                write!(formatter, "port conflict on {port}: {source}")
            }
            Self::Io(error) => write!(formatter, "{error}"),
            Self::Project(error) | Self::ShutdownHandler(error) => {
                write!(formatter, "{error}")
            }
        }
    }
}

impl Error for UiError {}

impl From<std::io::Error> for UiError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

/// A running server used by tests and embedders.
pub struct ServerHandle {
    address: SocketAddr,
    stop: Arc<AtomicBool>,
    thread: Option<thread::JoinHandle<()>>,
}

impl ServerHandle {
    /// Returns the local URL for this server.
    #[must_use]
    pub fn url(&self) -> String {
        format!("http://{}", self.address)
    }

    /// Returns the bound socket address.
    #[must_use]
    pub const fn address(&self) -> SocketAddr {
        self.address
    }

    /// Stops the server and waits for the serving thread.
    pub fn stop(mut self) {
        self.stop.store(true, Ordering::SeqCst);
        let _ignored = TcpStream::connect(self.address);
        if let Some(thread) = self.thread.take() {
            let _ignored = thread.join();
        }
    }
}

/// Starts the graph explorer in the current thread until Ctrl+C.
///
/// # Errors
///
/// Returns an error when binding, project loading, or request serving fails.
pub fn serve_current_thread(options: UiOptions) -> Result<String, UiError> {
    let stop = Arc::new(AtomicBool::new(false));
    crate::signal::install_sigint_handler(Arc::clone(&stop)).map_err(UiError::ShutdownHandler)?;

    let server = Server::bind(options)?;
    let url = server.url();
    println!("Graph explorer running at {url}");
    println!("Press Ctrl+C to stop.");
    if !server.options.no_open {
        open_browser(&url);
    }
    server.serve(&stop)?;
    Ok(format!("Graph explorer stopped: {url}"))
}

/// Starts the graph explorer on a background thread.
///
/// # Errors
///
/// Returns an error when binding or project loading fails.
pub fn start_background(options: UiOptions) -> Result<ServerHandle, UiError> {
    let server = Server::bind(options)?;
    let address = server.address;
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);
    let thread = thread::spawn(move || {
        let _ignored = server.serve(&thread_stop);
    });
    Ok(ServerHandle {
        address,
        stop,
        thread: Some(thread),
    })
}

#[cfg(test)]
mod tests {
    use super::server::request_path;
    use super::*;
    use serde_json::Value;
    use std::{
        fs,
        io::Read,
        net::TcpStream,
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn test_ui_project_load_failure_returns_diagnostic() -> Result<(), Box<dyn Error>> {
        let root = temp_root("project-load-failure")?;
        write_project(&root)?;
        fs::write(
            root.join("cairn.blueprint"),
            "System App \"desc\" id \"app\" {\n",
        )?;
        let server = start_background(UiOptions {
            port: 0,
            no_open: true,
            blueprint_path: root.join("cairn.blueprint"),
        })?;

        let response = request(server.address(), "GET", "/api/graph")?;

        server.stop();

        assert!(response.head.contains("500 Internal Server Error"));
        assert!(
            response
                .body
                .contains("\"code\":\"CAIRN_UI_PROJECT_LOAD_FAILED\"")
        );
        assert!(response.body.contains("cairn.blueprint"));
        assert!(response.body.contains("expected"));

        Ok(())
    }

    /// Scenario: the boot banner surfaces the server's structured error.
    /// The frontend runs only in a browser, so this locks the source-level
    /// contract the same way the phase 7.7 explorer tests do: fetchJson
    /// must extract `code: message` from a JSON error body and keep the
    /// generic message for non-JSON bodies.
    #[test]
    fn test_ui_fetch_json_surfaces_structured_error_body() {
        let js = UTILS_JS;
        assert!(
            js.contains("`${body.code}: ${body.message}`"),
            "fetchJson must surface the structured code and message from an error body"
        );
        assert!(
            js.contains("request failed: ${url} (${response.status})"),
            "fetchJson must keep the generic message as the non-JSON fallback"
        );
        assert!(
            js.contains("typeof body.code === \"string\""),
            "fetchJson must only trust a well-formed structured error body"
        );
    }

    #[test]
    fn test_ui_project_load_failure_serves_cached_scan() -> Result<(), Box<dyn Error>> {
        let root = temp_root("project-load-cache")?;
        write_project(&root)?;
        let blueprint_path = root.join("cairn.blueprint");
        let server = start_background(UiOptions {
            port: 0,
            no_open: true,
            blueprint_path: blueprint_path.clone(),
        })?;

        let first = request(server.address(), "GET", "/api/graph")?;
        assert!(first.head.contains("200 OK"));
        assert!(first.body.contains("\"nodes\""));

        // The reload decision compares file mtimes; rewrite until the
        // timestamp provably differs so the failing-reload path is
        // exercised even on coarse-resolution filesystems.
        let original_mtime = fs::metadata(&blueprint_path)?.modified()?;
        loop {
            fs::write(&blueprint_path, "System App \"desc\" id \"app\" {\n")?;
            if fs::metadata(&blueprint_path)?.modified()? != original_mtime {
                break;
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        let second = request(server.address(), "GET", "/api/graph")?;

        server.stop();

        assert!(second.head.contains("200 OK"));
        assert!(second.body.contains("\"nodes\""));

        Ok(())
    }

    #[test]
    fn test_ui_route_dispatch_and_content_types() -> Result<(), Box<dyn Error>> {
        let root = temp_root("route-dispatch")?;
        write_project(&root)?;
        let server = start_background(UiOptions {
            port: 0,
            no_open: true,
            blueprint_path: root.join("cairn.blueprint"),
        })?;

        let graph = request(server.address(), "GET", "/api/graph")?;
        let asset = request(server.address(), "GET", "/assets/style.css")?;
        let meta = request(server.address(), "GET", "/api/meta")?;

        server.stop();

        assert!(graph.head.contains("200 OK"));
        assert!(graph.head.contains("application/json"));
        assert!(graph.body.contains("\"nodes\""));
        assert!(asset.head.contains("text/css"));
        assert!(
            meta.body
                .contains(&format!("\"schema_version\":{SCHEMA_VERSION}"))
        );
        assert!(
            meta.body
                .contains(&format!("\"version\":\"{}\"", env!("CARGO_PKG_VERSION")))
        );
        let meta_json: serde_json::Value = serde_json::from_str(&meta.body)?;
        assert!(meta_json["last_reconciled"].is_u64());

        Ok(())
    }

    #[test]
    fn test_ui_returns_not_found_for_unknown_routes() -> Result<(), Box<dyn Error>> {
        let root = temp_root("not-found")?;
        write_project(&root)?;
        let server = start_background(UiOptions {
            port: 0,
            no_open: true,
            blueprint_path: root.join("cairn.blueprint"),
        })?;

        let missing = request(server.address(), "GET", "/missing")?;
        let unknown_api = request(server.address(), "GET", "/api/node/app.api/unknown")?;

        server.stop();

        assert!(missing.head.contains("404 Not Found"));
        assert_eq!(missing.body, "not found");
        assert!(unknown_api.head.contains("404 Not Found"));

        Ok(())
    }

    #[test]
    fn test_ui_symbols_endpoint_returns_extracted_symbols() -> Result<(), Box<dyn Error>> {
        let root = temp_root("symbols-endpoint")?;
        write_project(&root)?;
        let server = start_background(UiOptions {
            port: 0,
            no_open: true,
            blueprint_path: root.join("cairn.blueprint"),
        })?;

        let response = request(server.address(), "GET", "/api/node/app.api/symbols")?;

        server.stop();

        assert!(response.head.contains("200 OK"));
        assert!(
            response
                .body
                .contains(&format!("\"schema_version\":{SCHEMA_VERSION}"))
        );
        assert!(response.body.contains("\"id\":\"app.api\""));
        assert!(response.body.contains("\"name\":\"serve\""));
        assert!(response.body.contains("\"kind\":\"function\""));

        Ok(())
    }

    #[test]
    fn test_ui_rejects_unsupported_methods() -> Result<(), Box<dyn Error>> {
        let root = temp_root("unsupported-method")?;
        write_project(&root)?;
        let server = start_background(UiOptions {
            port: 0,
            no_open: true,
            blueprint_path: root.join("cairn.blueprint"),
        })?;

        let response = request(server.address(), "POST", "/api/meta")?;

        server.stop();

        assert!(response.head.contains("400 Bad Request"));
        assert!(response.head.contains("text/plain"));
        assert_eq!(response.body, "bad request");

        Ok(())
    }

    #[test]
    fn test_request_path_supports_get_only() {
        assert_eq!(
            request_path("GET /api/meta HTTP/1.1\r\nHost: test\r\n\r\n"),
            Some("/api/meta")
        );
        assert_eq!(
            request_path("POST /api/meta HTTP/1.1\r\nHost: test\r\n\r\n"),
            None
        );
    }

    struct HttpResponse {
        head: String,
        body: String,
    }

    fn request(
        address: SocketAddr,
        method: &str,
        path: &str,
    ) -> Result<HttpResponse, Box<dyn Error>> {
        let mut stream = TcpStream::connect(address)?;
        write!(
            stream,
            "{method} {path} HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\n\r\n"
        )?;
        let mut response = String::new();
        stream.read_to_string(&mut response)?;
        let Some((head, body)) = response.split_once("\r\n\r\n") else {
            return Err("missing http response body".into());
        };
        Ok(HttpResponse {
            head: head.to_owned(),
            body: body.to_owned(),
        })
    }

    #[test]
    fn test_ui_todos_endpoint_returns_enriched_canonical_shape() -> Result<(), Box<dyn Error>> {
        let root = temp_root("todos-endpoint")?;
        write_artefact_project(&root)?;
        let server = start_background(UiOptions {
            port: 0,
            no_open: true,
            blueprint_path: root.join("cairn.blueprint"),
        })?;

        let response = request(server.address(), "GET", "/api/node/app.api/todos")?;
        server.stop();

        assert!(response.head.contains("200 OK"));
        let body: Value = serde_json::from_str(&response.body)?;
        assert_eq!(body["node"], "app.api");
        let todos = body["todos"].as_array().expect("todos array");
        assert_eq!(todos.len(), 1);
        let todo = &todos[0];
        assert_eq!(todo["path"], "meta/todos/todo.api.md");
        assert_eq!(todo["node"], "app.api");
        assert_eq!(todo["status"], "open");
        assert_eq!(todo["created"], "2026-04-01");
        assert_eq!(todo["satisfies"], "status.contract");
        assert_eq!(todo["title"], "API Todo");
        assert_eq!(todo["body"], "# API Todo\nShip the endpoint.");
        assert!(body["schema_version"].is_u64());

        Ok(())
    }

    #[test]
    fn test_ui_reloads_when_an_artefact_changes() -> Result<(), Box<dyn Error>> {
        let root = temp_root("artefact-reload")?;
        write_artefact_project(&root)?;
        let todos_dir = root.join("meta/todos");
        let todo_path = todos_dir.join("todo.api.md");
        let server = start_background(UiOptions {
            port: 0,
            no_open: true,
            blueprint_path: root.join("cairn.blueprint"),
        })?;

        let first = request(server.address(), "GET", "/api/node/app.api/todos")?;
        let first: Value = serde_json::from_str(&first.body)?;
        assert_eq!(first["todos"].as_array().expect("todos array").len(), 1);

        let original_mtime = fs::metadata(&todo_path)?.modified()?;
        let mut file_changed = false;
        for _ in 0..100 {
            fs::write(
                &todo_path,
                "---\nnode: app.api\nstatus: open\ncreated: 2026-04-01\n---\n# Edited API Todo\nReload the API.\n",
            )?;
            if fs::metadata(&todo_path)?.modified()? != original_mtime {
                file_changed = true;
                break;
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        assert!(file_changed, "todo mtime did not advance");

        let second = request(server.address(), "GET", "/api/node/app.api/todos")?;
        let second: Value = serde_json::from_str(&second.body)?;
        let todos = second["todos"].as_array().expect("todos array");
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0]["title"], "Edited API Todo");

        let original_dir_mtime = fs::metadata(&todos_dir)?.modified()?;
        let added_path = todos_dir.join("todo.added.md");
        let mut directory_changed = false;
        for _ in 0..100 {
            fs::write(
                &added_path,
                "---\nnode: app.api\nstatus: open\ncreated: 2026-04-01\n---\n# Added API Todo\nReload the API.\n",
            )?;
            if fs::metadata(&todos_dir)?.modified()? != original_dir_mtime {
                directory_changed = true;
                break;
            }
            fs::remove_file(&added_path)?;
            std::thread::sleep(Duration::from_millis(5));
        }
        assert!(directory_changed, "todo directory mtime did not advance");

        let third = request(server.address(), "GET", "/api/node/app.api/todos")?;
        server.stop();

        let third: Value = serde_json::from_str(&third.body)?;
        let todos = third["todos"].as_array().expect("todos array");
        assert_eq!(todos.len(), 2);
        assert!(todos.iter().any(|todo| todo["title"] == "Added API Todo"));

        Ok(())
    }
    #[test]
    fn test_ui_spine_endpoint_returns_404_for_unknown_node() -> Result<(), Box<dyn Error>> {
        let root = temp_root("todos-endpoint-unknown-node")?;
        write_artefact_project(&root)?;
        let server = start_background(UiOptions {
            port: 0,
            no_open: true,
            blueprint_path: root.join("cairn.blueprint"),
        })?;

        let response = request(server.address(), "GET", "/api/node/no.such.node/todos")?;
        server.stop();

        assert!(response.head.contains("404"));
        let body: Value = serde_json::from_str(&response.body)?;
        assert_eq!(body["code"], "CAIRN_QUERY_NODE_NOT_FOUND");
        Ok(())
    }

    #[test]
    fn test_ui_decisions_endpoint_returns_enriched_canonical_shape() -> Result<(), Box<dyn Error>> {
        let root = temp_root("decisions-endpoint")?;
        write_artefact_project(&root)?;
        let server = start_background(UiOptions {
            port: 0,
            no_open: true,
            blueprint_path: root.join("cairn.blueprint"),
        })?;

        let response = request(server.address(), "GET", "/api/node/app.api/decisions")?;
        server.stop();

        assert!(response.head.contains("200 OK"));
        let body: Value = serde_json::from_str(&response.body)?;
        assert_eq!(body["node"], "app.api");
        let decisions = body["decisions"].as_array().expect("decisions array");
        assert_eq!(decisions.len(), 1);
        let decision = &decisions[0];
        assert_eq!(decision["path"], "meta/decisions/api.md");
        assert_eq!(decision["id"], "dec.api");
        assert_eq!(decision["status"], "accepted");
        assert_eq!(decision["nodes"], serde_json::json!(["app.api"]));
        assert_eq!(decision["informed_by"], serde_json::json!(["res.api"]));
        assert_eq!(decision["title"], "API Decision");
        assert_eq!(
            decision["body"],
            "# API Decision\nUse stable JSON payloads."
        );
        assert!(body["schema_version"].is_u64());

        Ok(())
    }

    #[test]
    fn test_ui_research_endpoint_returns_enriched_canonical_shape() -> Result<(), Box<dyn Error>> {
        let root = temp_root("research-endpoint")?;
        write_artefact_project(&root)?;
        let server = start_background(UiOptions {
            port: 0,
            no_open: true,
            blueprint_path: root.join("cairn.blueprint"),
        })?;

        let response = request(server.address(), "GET", "/api/node/app.api/research")?;
        server.stop();

        assert!(response.head.contains("200 OK"));
        let body: Value = serde_json::from_str(&response.body)?;
        assert_eq!(body["node"], "app.api");
        let research_items = body["research"].as_array().expect("research array");
        assert_eq!(research_items.len(), 1);
        let research = &research_items[0];
        assert_eq!(research["path"], "meta/research/api.md");
        assert_eq!(research["id"], "res.api");
        assert_eq!(research["nodes"], serde_json::json!(["app.api"]));
        assert_eq!(research["sources"], serde_json::json!(["src.api"]));
        assert_eq!(research["date"], "2026-03-20");
        assert_eq!(research["title"], "API Research");
        assert_eq!(
            research["body"],
            "# API Research\nStudied payload evolution."
        );
        assert!(body["schema_version"].is_u64());

        Ok(())
    }

    #[test]
    fn test_ui_sources_endpoint_returns_enriched_canonical_shape() -> Result<(), Box<dyn Error>> {
        let root = temp_root("sources-endpoint")?;
        write_artefact_project(&root)?;
        let server = start_background(UiOptions {
            port: 0,
            no_open: true,
            blueprint_path: root.join("cairn.blueprint"),
        })?;

        let response = request(server.address(), "GET", "/api/node/app.api/sources")?;
        server.stop();

        assert!(response.head.contains("200 OK"));
        let body: Value = serde_json::from_str(&response.body)?;
        assert_eq!(body["node"], "app.api");
        let sources = body["sources"].as_array().expect("sources array");
        assert_eq!(sources.len(), 1);
        let source = &sources[0];
        assert_eq!(source["path"], "meta/sources/api.md");
        assert_eq!(source["id"], "src.api");
        assert_eq!(source["file"], "docs-source.txt");
        assert_eq!(source["verification"], "verified");
        assert_eq!(source["type"], "note");
        assert_eq!(source["date"], "2026-03-19");
        assert_eq!(source["title"], "API Source");
        assert_eq!(source["body"], "# API Source\nBootstrap evidence.");
        assert!(body["schema_version"].is_u64());

        Ok(())
    }

    #[test]
    fn test_ui_contract_endpoint_returns_enriched_canonical_shape() -> Result<(), Box<dyn Error>> {
        let root = temp_root("contract-endpoint")?;
        write_artefact_project(&root)?;
        let server = start_background(UiOptions {
            port: 0,
            no_open: true,
            blueprint_path: root.join("cairn.blueprint"),
        })?;

        let response = request(server.address(), "GET", "/api/node/app.api/contract")?;
        server.stop();

        assert!(response.head.contains("200 OK"));
        let body: Value = serde_json::from_str(&response.body)?;
        assert_eq!(body["node"], "app.api");
        assert_eq!(
            body["contract"],
            "# API Contract\nGET /api/status returns health details."
        );
        let contracts = body["contracts"].as_array().expect("contracts array");
        assert_eq!(contracts.len(), 1);
        let contract = &contracts[0];
        assert_eq!(contract["path"], "./meta/contracts/api.md");
        assert_eq!(contract["node"], "app.api");
        assert_eq!(contract["declared_by"], "app.api");
        assert_eq!(contract["title"], "API Contract");
        assert_eq!(
            contract["body"],
            "# API Contract\nGET /api/status returns health details."
        );
        assert!(body["schema_version"].is_u64());

        Ok(())
    }

    #[test]
    fn test_ui_rationale_endpoint_returns_enriched_canonical_shape() -> Result<(), Box<dyn Error>> {
        let root = temp_root("rationale-endpoint")?;
        write_artefact_project(&root)?;
        let server = start_background(UiOptions {
            port: 0,
            no_open: true,
            blueprint_path: root.join("cairn.blueprint"),
        })?;

        let response = request(server.address(), "GET", "/api/node/app.api/rationale")?;
        server.stop();

        assert!(response.head.contains("200 OK"));
        let body: Value = serde_json::from_str(&response.body)?;
        assert_eq!(body["node"], "app.api");

        let decisions = body["decisions"].as_array().expect("decisions array");
        assert_eq!(decisions.len(), 1);
        let decision = &decisions[0];
        assert_eq!(decision["path"], "meta/decisions/api.md");
        assert_eq!(decision["id"], "dec.api");
        assert_eq!(decision["title"], "API Decision");
        assert_eq!(
            decision["body"],
            "# API Decision\nUse stable JSON payloads."
        );

        let research_items = body["research"].as_array().expect("research array");
        assert_eq!(research_items.len(), 1);
        let research = &research_items[0];
        assert_eq!(research["path"], "meta/research/api.md");
        assert_eq!(research["id"], "res.api");
        assert_eq!(research["title"], "API Research");
        assert_eq!(
            research["body"],
            "# API Research\nStudied payload evolution."
        );

        let sources = body["sources"].as_array().expect("sources array");
        assert_eq!(sources.len(), 1);
        let source = &sources[0];
        assert_eq!(source["path"], "meta/sources/api.md");
        assert_eq!(source["id"], "src.api");
        assert_eq!(source["title"], "API Source");
        assert_eq!(source["body"], "# API Source\nBootstrap evidence.");

        assert!(body["schema_version"].is_u64());

        Ok(())
    }

    fn write_artefact_project(root: &Path) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(root.join("src/api"))?;
        fs::create_dir_all(root.join("meta/contracts"))?;
        fs::create_dir_all(root.join("meta/todos"))?;
        fs::create_dir_all(root.join("meta/decisions"))?;
        fs::create_dir_all(root.join("meta/research"))?;
        fs::create_dir_all(root.join("meta/sources"))?;
        fs::write(
            root.join("src/api/lib.rs"),
            "pub fn serve() {}\n#[cfg(test)]\nmod tests {}\n",
        )?;
        fs::write(
            root.join("cairn.blueprint"),
            r#"System App "desc" id "app" {
    Container Api "api" id "app.api" {
        path "./src/api"
        contract "./meta/contracts/api.md"
        todos "./meta/todos"
        decisions "./meta/decisions"
        research "./meta/research"
        sources "./meta/sources"
    }
}
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
            root.join("meta/decisions/api.md"),
            "---\nid: dec.api\nnodes: [app.api]\nstatus: accepted\ndate: 2026-04-01\ninformed_by: [res.api]\n---\n# API Decision\nUse stable JSON payloads.\n",
        )?;
        fs::write(
            root.join("meta/research/api.md"),
            "---\nid: res.api\nnodes: [app.api]\ndate: 2026-03-20\nsources: [src.api]\ntags: [wire]\n---\n# API Research\nStudied payload evolution.\n",
        )?;
        fs::write(root.join("docs-source.txt"), "wire format source\n")?;
        fs::write(
            root.join("meta/sources/api.md"),
            "---\nid: src.api\nfile: docs-source.txt\nsha256: ecf5dae7a91b73f6faec1d386583345afe598f4b8af0d647f28f0b0f46f7c633\nverification: verified\ntype: note\ndate: 2026-03-19\ntags: [wire]\ndescription: bootstrap source\n---\n# API Source\nBootstrap evidence.\n",
        )?;
        Ok(())
    }

    fn write_project(root: &Path) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(root.join("src/api"))?;
        fs::create_dir_all(root.join("meta/contracts"))?;
        fs::write(root.join("src/api/lib.rs"), "pub fn serve() {}\n")?;
        fs::write(
            root.join("cairn.blueprint"),
            r#"System App "desc" id "app" {
    Container Api "api" id "app.api" {
        path "./src/api"
        contract "./meta/contracts/api.md"
    }
}
"#,
        )?;
        fs::write(
            root.join("meta/contracts/api.md"),
            "---\nnode: app.api\n---\n# API Contract\n",
        )?;
        Ok(())
    }

    fn temp_root(name: &str) -> Result<PathBuf, Box<dyn Error>> {
        let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let root = std::env::temp_dir().join(format!("cairn-ui-tests-{name}-{suffix}"));
        fs::create_dir_all(&root)?;
        Ok(root)
    }
}
