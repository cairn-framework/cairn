//! Embedded HTTP server and browser UI for graph exploration.

use std::{
    collections::BTreeMap,
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

use crate::{
    artefacts::{contract::Contract, frontmatter},
    blueprint::NodeKind,
    cli,
    map::{
        graph::{Finding, FindingSeverity, Graph, NodeRecord},
        query::{self, GraphEdgeKind, GraphResponse},
    },
    scanner,
};

mod api;
mod serialise;
mod server;

use server::{Server, open_browser};

const INDEX_HTML: &str = include_str!("../ui_assets/index.html");
const APP_JS: &str = include_str!("../ui_assets/app.js");

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

const SCHEMA_VERSION: u32 = 1;

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
    ShutdownHandler(ctrlc::Error),
}

impl fmt::Display for UiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bind { port, source } => {
                write!(formatter, "port conflict on {port}: {source}")
            }
            Self::Io(error) => write!(formatter, "{error}"),
            Self::Project(error) => write!(formatter, "{error}"),
            Self::ShutdownHandler(error) => write!(formatter, "{error}"),
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
    let shutdown = Arc::clone(&stop);
    ctrlc::set_handler(move || {
        shutdown.store(true, Ordering::SeqCst);
    })
    .map_err(UiError::ShutdownHandler)?;

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
    use std::{
        fs,
        io::Read,
        net::TcpStream,
        time::{SystemTime, UNIX_EPOCH},
    };

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
        assert!(meta.body.contains("\"schema_version\":1"));

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
