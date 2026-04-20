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

const INDEX_HTML: &str = include_str!("ui_assets/index.html");
const APP_JS: &str = include_str!("ui_assets/app.js");

/// Canonical design-system tokens; single source of truth.
const DESIGN_TOKENS_CSS: &str = include_str!("../docs/design-system/tokens.css");
/// Canonical design-system component primitives.
const DESIGN_COMPONENTS_CSS: &str = include_str!("../docs/design-system/components.css");
/// Graph-explorer-specific layout and overrides.
const UI_STYLE_CSS: &str = include_str!("ui_assets/style.css");

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

struct Server {
    options: UiOptions,
    root: PathBuf,
    listener: TcpListener,
    address: SocketAddr,
}

impl Server {
    fn bind(options: UiOptions) -> Result<Self, UiError> {
        let root = options
            .blueprint_path
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();
        let address = SocketAddr::from(([127, 0, 0, 1], options.port));
        let listener = TcpListener::bind(address).map_err(|source| UiError::Bind {
            port: options.port,
            source,
        })?;
        listener.set_nonblocking(true)?;
        let address = listener.local_addr()?;
        Ok(Self {
            options,
            root,
            listener,
            address,
        })
    }

    fn url(&self) -> String {
        format!("http://{}", self.address)
    }

    fn serve(self, stop: &Arc<AtomicBool>) -> Result<(), UiError> {
        while !stop.load(Ordering::SeqCst) {
            match self.listener.accept() {
                Ok((mut stream, _peer)) => {
                    stream.set_nonblocking(false)?;
                    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
                    if let Err(error) = self.handle_stream(&mut stream)
                        && !matches!(
                            error,
                            UiError::Io(ref source)
                                if matches!(
                                    source.kind(),
                                    std::io::ErrorKind::ConnectionReset
                                        | std::io::ErrorKind::BrokenPipe
                                )
                        )
                    {
                        return Err(error);
                    }
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(25));
                }
                Err(error) => return Err(UiError::Io(error)),
            }
        }
        Ok(())
    }

    fn handle_stream(&self, stream: &mut TcpStream) -> Result<(), UiError> {
        let request = read_http_request(stream)?;
        if request.is_empty() {
            return Ok(());
        }
        let request = String::from_utf8_lossy(&request);
        let Some(path) = request_path(request.as_ref()) else {
            write_response(stream, 400, "text/plain; charset=utf-8", "bad request")?;
            return Ok(());
        };
        let response = self.route(path);
        write_response(
            stream,
            response.status,
            response.content_type,
            &response.body,
        )?;
        Ok(())
    }

    fn route(&self, path: &str) -> Response {
        match path {
            "/" | "/index.html" => html(INDEX_HTML),
            "/assets/style.css" => asset("text/css; charset=utf-8", STYLE_CSS.as_str()),
            "/assets/app.js" => asset("application/javascript; charset=utf-8", APP_JS),
            _ if path.starts_with("/api/") => self.api(path),
            _ => text(404, "not found"),
        }
    }

    fn api(&self, path: &str) -> Response {
        let project = match self.load_project() {
            Ok(project) => project,
            Err(error) => return json(500, &finding_json(&project_finding(error.to_string()))),
        };
        let graph = &project.graph;
        if path == "/api/meta" {
            return json(200, &meta_json());
        }
        if path == "/api/status" {
            return json(200, &status_json(&project));
        }
        if path == "/api/graph" {
            return json(200, &graph_json(&query::graph(graph)));
        }
        if path == "/api/lint" {
            return json(200, &lint_json(graph));
        }
        if let Some(node) = path.strip_prefix("/api/node/") {
            return self.node_api(&project, node);
        }
        if let Some(node) = path.strip_prefix("/api/dependents/") {
            return dependency_json(graph, node, false);
        }
        if let Some(node) = path.strip_prefix("/api/depends/") {
            return dependency_json(graph, node, true);
        }
        text(404, "not found")
    }

    fn node_api(&self, project: &scanner::ScanResult, path: &str) -> Response {
        let (node, suffix) = path.split_once('/').unwrap_or((path, ""));
        let node = percent_decode(node);
        match suffix {
            "" => query::get(&project.graph, &node).map_or_else(
                |finding| json(404, &finding_json(&finding)),
                |response| json(200, &node_json(&response.node)),
            ),
            "contract" => json(200, &contract_response_json(project, &node)),
            "decisions" => json(200, &artefact_response_json(&self.root, "decisions", &node)),
            "todos" => json(200, &artefact_response_json(&self.root, "todos", &node)),
            "research" => json(200, &artefact_response_json(&self.root, "research", &node)),
            "sources" => json(200, &artefact_response_json(&self.root, "sources", &node)),
            "rationale" => json(200, &rationale_json(&self.root, &node)),
            _ => text(404, "not found"),
        }
    }

    fn load_project(&self) -> Result<scanner::ScanResult, UiError> {
        scanner::load_project(&self.root, &self.options.blueprint_path).map_err(UiError::Project)
    }
}

fn read_http_request(stream: &mut TcpStream) -> Result<Vec<u8>, std::io::Error> {
    let mut request = Vec::new();
    let mut buffer = [0_u8; 1024];
    loop {
        let read = stream.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        request.extend_from_slice(&buffer[..read]);
        if request.windows(4).any(|window| window == b"\r\n\r\n") || request.len() >= 8192 {
            break;
        }
    }
    Ok(request)
}

struct Response {
    status: u16,
    content_type: &'static str,
    body: String,
}

fn html(body: &str) -> Response {
    asset("text/html; charset=utf-8", body)
}

fn asset(content_type: &'static str, body: &str) -> Response {
    Response {
        status: 200,
        content_type,
        body: body.to_owned(),
    }
}

fn json(status: u16, body: &str) -> Response {
    Response {
        status,
        content_type: "application/json; charset=utf-8",
        body: body.to_owned(),
    }
}

fn text(status: u16, body: &str) -> Response {
    Response {
        status,
        content_type: "text/plain; charset=utf-8",
        body: body.to_owned(),
    }
}

fn write_response(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &str,
) -> Result<(), std::io::Error> {
    let status_text = match status {
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "OK",
    };
    write!(
        stream,
        "HTTP/1.1 {status} {status_text}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
}

fn request_path(request: &str) -> Option<&str> {
    let mut parts = request.lines().next()?.split_whitespace();
    let method = parts.next()?;
    let path = parts.next()?;
    if method == "GET" {
        Some(path.split('?').next().unwrap_or(path))
    } else {
        None
    }
}

fn open_browser(url: &str) {
    let result = if cfg!(target_os = "macos") {
        Command::new("open").arg(url).status()
    } else if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "start", url]).status()
    } else {
        Command::new("xdg-open").arg(url).status()
    };
    drop(result);
}

fn meta_json() -> String {
    let commands = cli::registry()
        .iter()
        .map(|command| {
            format!(
                "{{\"name\":\"{}\",\"request\":\"{}\",\"response\":\"{}\",\"safety\":\"{:?}\"}}",
                esc(command.cli_name),
                esc(command.request_schema),
                esc(command.response_schema),
                command.safety
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("{{\"schema_version\":{SCHEMA_VERSION},\"available_commands\":[{commands}]}}")
}

fn graph_json(graph: &GraphResponse) -> String {
    let nodes = graph
        .nodes
        .iter()
        .map(node_json)
        .collect::<Vec<_>>()
        .join(",");
    let edges = graph
        .edges
        .iter()
        .map(|edge| {
            format!(
                "{{\"from\":\"{}\",\"to\":\"{}\",\"kind\":\"{}\",\"description\":\"{}\"}}",
                esc(&edge.from),
                esc(&edge.to),
                graph_edge_kind_name(edge.kind),
                esc(&edge.description)
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("{{\"nodes\":[{nodes}],\"edges\":[{edges}]}}")
}

fn node_json(node: &NodeRecord) -> String {
    format!(
        "{{\"id\":\"{}\",\"kind\":\"{}\",\"name\":\"{}\",\"description\":\"{}\",\"tags\":{},\"parent\":{},\"children\":{},\"paths\":{},\"contracts\":{},\"state\":\"{:?}\",\"files\":{}}}",
        esc(&node.id),
        kind_name(node.kind),
        esc(&node.name),
        esc(&node.description),
        string_array_json(&node.tags),
        optional_json(node.parent.as_deref()),
        string_array_json(&node.children),
        string_array_json(&node.paths),
        string_array_json(&node.contracts),
        node.state,
        string_array_json(&node.files)
    )
}

fn dependency_json(graph: &Graph, node: &str, outbound: bool) -> Response {
    let decoded = percent_decode(node);
    let result = if outbound {
        query::depends(graph, &decoded, false)
    } else {
        query::dependents(graph, &decoded, false)
    };
    result.map_or_else(
        |finding| json(404, &finding_json(&finding)),
        |response| {
            json(
                200,
                &format!(
                    "{{\"node\":\"{}\",\"nodes\":{}}}",
                    esc(&response.node),
                    string_array_json(&response.nodes)
                ),
            )
        },
    )
}

fn lint_json(graph: &Graph) -> String {
    let findings = query::lint(graph)
        .findings
        .iter()
        .map(finding_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("{{\"findings\":[{findings}]}}")
}

fn status_json(project: &scanner::ScanResult) -> String {
    let findings = query::lint(&project.graph).findings;
    let errors = findings
        .iter()
        .filter(|finding| finding.severity == FindingSeverity::Error)
        .count();
    let warnings = findings.len().saturating_sub(errors);
    format!(
        "{{\"schema_version\":{SCHEMA_VERSION},\"nodes\":{},\"edges\":{},\"findings\":{},\"errors\":{errors},\"warnings\":{warnings},\"interface_hash\":\"{}\"}}",
        project.graph.nodes.len(),
        project.graph.outbound.values().map(Vec::len).sum::<usize>(),
        findings.len(),
        esc(&project.interface_hash)
    )
}

fn contract_response_json(project: &scanner::ScanResult, node: &str) -> String {
    let artefacts = project
        .contracts
        .contracts
        .values()
        .filter(|contract| contract.node == node || contract.declared_by == node)
        .map(contract_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("{{\"node\":\"{}\",\"artefacts\":[{artefacts}]}}", esc(node))
}

fn contract_json(contract: &Contract) -> String {
    format!(
        "{{\"type\":\"contract\",\"path\":\"{}\",\"title\":\"{}\",\"frontmatter\":{{\"node\":\"{}\"}},\"body\":\"{}\"}}",
        esc(&contract.path),
        esc(&title_from_body(&contract.body, "Contract")),
        esc(&contract.node),
        esc(&contract.body)
    )
}

fn artefact_response_json(root: &Path, kind: &str, node: &str) -> String {
    let artefacts = collect_artefacts(root, kind, node)
        .iter()
        .map(|artefact| artefact_json(kind, artefact))
        .collect::<Vec<_>>()
        .join(",");
    format!("{{\"node\":\"{}\",\"artefacts\":[{artefacts}]}}", esc(node))
}

fn rationale_json(root: &Path, node: &str) -> String {
    let artefacts = ["decisions", "research", "sources"]
        .iter()
        .flat_map(|kind| {
            collect_artefacts(root, kind, node)
                .into_iter()
                .map(|artefact| ((*kind).to_owned(), artefact))
        })
        .map(|(kind, artefact)| artefact_json(&kind, &artefact))
        .collect::<Vec<_>>()
        .join(",");
    format!("{{\"node\":\"{}\",\"artefacts\":[{artefacts}]}}", esc(node))
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Artefact {
    path: String,
    title: String,
    frontmatter: BTreeMap<String, String>,
    body: String,
}

fn collect_artefacts(root: &Path, kind: &str, node: &str) -> Vec<Artefact> {
    let mut artefacts = Vec::new();
    let directory = root.join("meta").join(kind);
    collect_artefacts_from_dir(root, &directory, node, &mut artefacts);
    artefacts.sort_by(|left, right| left.path.cmp(&right.path));
    artefacts
}

fn collect_artefacts_from_dir(
    root: &Path,
    directory: &Path,
    node: &str,
    artefacts: &mut Vec<Artefact>,
) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_artefacts_from_dir(root, &path, node, artefacts);
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }
        let Ok(source) = fs::read_to_string(&path) else {
            continue;
        };
        let parsed = frontmatter::parse(&source);
        if !frontmatter_mentions_node(&parsed.values, node) {
            continue;
        }
        let relative = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();
        artefacts.push(Artefact {
            path: relative,
            title: title_from_body(&parsed.body, "Artefact"),
            frontmatter: parsed.values,
            body: parsed.body,
        });
    }
}

fn frontmatter_mentions_node(values: &BTreeMap<String, String>, node: &str) -> bool {
    ["node", "nodes"]
        .iter()
        .filter_map(|key| values.get(*key))
        .any(|value| value.contains(node))
}

fn artefact_json(kind: &str, artefact: &Artefact) -> String {
    format!(
        "{{\"type\":\"{}\",\"path\":\"{}\",\"title\":\"{}\",\"frontmatter\":{},\"body\":\"{}\"}}",
        esc(kind),
        esc(&artefact.path),
        esc(&artefact.title),
        map_json(&artefact.frontmatter),
        esc(&artefact.body)
    )
}

fn finding_json(finding: &Finding) -> String {
    format!(
        "{{\"code\":\"{}\",\"severity\":\"{:?}\",\"message\":\"{}\",\"node\":{},\"path\":{}}}",
        esc(&finding.code),
        finding.severity,
        esc(&finding.message),
        optional_json(finding.node.as_deref()),
        optional_json(finding.path.as_deref())
    )
}

fn project_finding(message: String) -> Finding {
    Finding {
        code: "CAIRN_UI_PROJECT_LOAD_FAILED".to_owned(),
        severity: FindingSeverity::Error,
        message,
        node: None,
        path: None,
    }
}

fn title_from_body(body: &str, fallback: &str) -> String {
    body.lines()
        .find_map(|line| line.trim().strip_prefix("# "))
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .unwrap_or(fallback)
        .to_owned()
}

const fn kind_name(kind: NodeKind) -> &'static str {
    match kind {
        NodeKind::System => "system",
        NodeKind::Container => "container",
        NodeKind::Module => "module",
        NodeKind::Actor => "actor",
    }
}

const fn graph_edge_kind_name(kind: GraphEdgeKind) -> &'static str {
    match kind {
        GraphEdgeKind::Ownership => "ownership",
        GraphEdgeKind::Dependency => "dependency",
    }
}

fn map_json(values: &BTreeMap<String, String>) -> String {
    let fields = values
        .iter()
        .map(|(key, value)| format!("\"{}\":\"{}\"", esc(key), esc(value)))
        .collect::<Vec<_>>()
        .join(",");
    format!("{{{fields}}}")
}

fn string_array_json(values: &[String]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| format!("\"{}\"", esc(value)))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn optional_json(value: Option<&str>) -> String {
    value.map_or_else(|| "null".to_owned(), |text| format!("\"{}\"", esc(text)))
}

fn percent_decode(value: &str) -> String {
    let mut output = String::new();
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        if ch == '%' {
            let hi = chars.next();
            let lo = chars.next();
            if let (Some(hi), Some(lo)) = (hi, lo)
                && let (Some(hi), Some(lo)) = (hi.to_digit(16), lo.to_digit(16))
            {
                let Ok(byte) = u8::try_from(hi * 16 + lo) else {
                    output.push(ch);
                    continue;
                };
                output.push(char::from(byte));
                continue;
            }
        }
        output.push(ch);
    }
    output.replace('+', " ")
}

fn esc(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}
