//! Embedded HTTP server for the graph explorer UI.

// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::*;
use api::{dependency_json, finding_json, graph_json, node_json, project_finding, status_json};
use serialise::percent_decode;
use std::{cell::RefCell, time::SystemTime};

pub(super) struct Server {
    pub(super) options: UiOptions,
    root: PathBuf,
    listener: TcpListener,
    pub(super) address: SocketAddr,
    cached_scan: RefCell<Option<scanner::ScanResult>>,
    cached_mtime: RefCell<Option<SystemTime>>,
    cached_watched_mtime: RefCell<Option<SystemTime>>,
    changes_dir: PathBuf,
}

impl Server {
    pub(super) fn bind(options: UiOptions) -> Result<Self, UiError> {
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
            root: root.clone(),
            listener,
            address,
            cached_scan: RefCell::new(None),
            cached_mtime: RefCell::new(None),
            cached_watched_mtime: RefCell::new(None),
            changes_dir: root.join("meta/changes"),
        })
    }

    pub(super) fn url(&self) -> String {
        format!("http://{}", self.address)
    }

    pub(super) fn serve(self, stop: &Arc<AtomicBool>) -> Result<(), UiError> {
        while !stop.load(Ordering::SeqCst) {
            match self.listener.accept() {
                Ok((mut stream, _peer)) => {
                    stream.set_nonblocking(false)?;
                    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
                    stream.set_write_timeout(Some(Duration::from_secs(5)))?;
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
            "/assets/copy.json" => asset("application/json; charset=utf-8", COPY_JSON.as_str()),
            "/vendor/preact.min.js" => {
                asset("application/javascript; charset=utf-8", VENDOR_PREACT_JS)
            }
            "/vendor/preact-hooks.min.js" => asset(
                "application/javascript; charset=utf-8",
                VENDOR_PREACT_HOOKS_JS,
            ),
            "/vendor/htm.min.js" => asset("application/javascript; charset=utf-8", VENDOR_HTM_JS),
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
            return self.spine(&project, "ui_meta", None, std::collections::BTreeSet::new());
        }
        if path == "/api/status" {
            return json(200, &status_json(&project));
        }
        if path == "/api/graph" {
            return json(200, &graph_json(&query::graph(graph)));
        }
        if path == "/api/lint" {
            return self.spine(&project, "lint", None, std::collections::BTreeSet::new());
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
        if path == "/api/blueprint" {
            // Legacy behaviour: a blueprint read failure is a 404, not a 200
            // with an error field.
            return match self.spine_data(
                &project,
                "blueprint",
                None,
                std::collections::BTreeSet::new(),
            ) {
                Ok(data) => {
                    let status = if data["error"].is_null() { 200 } else { 404 };
                    json(status, &data.to_string())
                }
                Err(error) => error,
            };
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
            "contract" => self.spine(
                project,
                "contract",
                Some(node.clone()),
                std::collections::BTreeSet::new(),
            ),
            "symbols" => self.spine(
                project,
                "get",
                Some(node.clone()),
                std::collections::BTreeSet::from([crate::query_api::QueryFlag::Symbols]),
            ),
            "decisions" => self.spine(
                project,
                "decisions",
                Some(node.clone()),
                std::collections::BTreeSet::new(),
            ),
            "todos" => self.spine(
                project,
                "todos",
                Some(node.clone()),
                std::collections::BTreeSet::new(),
            ),
            "research" => self.spine(
                project,
                "research",
                Some(node.clone()),
                std::collections::BTreeSet::new(),
            ),
            "sources" => self.spine(
                project,
                "sources",
                Some(node.clone()),
                std::collections::BTreeSet::new(),
            ),
            "beads" => self.spine(
                project,
                "beads",
                Some(node.clone()),
                std::collections::BTreeSet::new(),
            ),
            "rationale" => self.spine(
                project,
                "rationale",
                Some(node.clone()),
                std::collections::BTreeSet::new(),
            ),
            _ => text(404, "not found"),
        }
    }

    /// Executes a `query_api` tool against the cached scan and returns its data
    /// payload. `query_api` stamps its own `schema_version`; the server's
    /// `json()` stamp is the single owner of the wire version key, so the
    /// inner stamp is stripped here.
    fn spine_data(
        &self,
        project: &scanner::ScanResult,
        tool: &str,
        node: Option<String>,
        flags: std::collections::BTreeSet<crate::query_api::QueryFlag>,
    ) -> Result<serde_json::Value, Response> {
        let request = crate::query_api::QueryRequest {
            tool: tool.to_owned(),
            node,
            flags,
            ..Default::default()
        };
        match crate::query_api::execute_with_scan(
            &self.root,
            &self.options.blueprint_path,
            &self.changes_dir,
            &request,
            project,
        ) {
            Ok(mut response) => {
                if let Some(map) = response.data.as_object_mut() {
                    map.remove("schema_version");
                }
                Ok(response.data)
            }
            Err(error) => {
                // Unknown node keeps the legacy 404 contract; every other
                // query failure is a server-side error.
                let status = if error.code == "CAIRN_QUERY_NODE_NOT_FOUND" {
                    404
                } else {
                    500
                };
                Err(json(status, &finding_json(&project_finding(error.message))))
            }
        }
    }

    /// `spine_data` served as a 200 response.
    fn spine(
        &self,
        project: &scanner::ScanResult,
        tool: &str,
        node: Option<String>,
        flags: std::collections::BTreeSet<crate::query_api::QueryFlag>,
    ) -> Response {
        self.spine_data(project, tool, node, flags)
            .map_or_else(|error| error, |data| json(200, &data.to_string()))
    }

    fn load_project(&self) -> Result<scanner::ScanResult, UiError> {
        let blueprint_path = &self.options.blueprint_path;
        let current_mtime = fs::metadata(blueprint_path).and_then(|m| m.modified()).ok();
        let current_watched_mtime = self.watched_files_mtime();
        let should_reload = {
            let cached_mtime = *self.cached_mtime.borrow();
            let cached_watched = *self.cached_watched_mtime.borrow();
            let blueprint_changed = match (cached_mtime, current_mtime) {
                (Some(c), Some(m)) => c != m,
                _ => true,
            };
            let watched_changed = match (cached_watched, current_watched_mtime) {
                (Some(c), Some(m)) => c != m,
                _ => true,
            };
            blueprint_changed || watched_changed
        };
        if !should_reload && let Some(scan) = self.cached_scan.borrow().as_ref() {
            return Ok(scan.clone());
        }
        let scan = scanner::load_project(&self.root, blueprint_path).map_err(UiError::Project)?;
        *self.cached_mtime.borrow_mut() = current_mtime;
        *self.cached_watched_mtime.borrow_mut() = current_watched_mtime;
        *self.cached_scan.borrow_mut() = Some(scan.clone());
        Ok(scan)
    }
    fn watched_files_mtime(&self) -> Option<SystemTime> {
        let scan = self.cached_scan.borrow();
        let scan = scan.as_ref()?;
        let mut max_mtime: Option<SystemTime> = None;
        for report in &scan.target_reports {
            let path = self.root.join(&report.target_id.path);
            if let Ok(meta) = fs::metadata(&path)
                && let Ok(mtime) = meta.modified()
            {
                max_mtime = Some(max_mtime.map_or(mtime, |m| m.max(mtime)));
            }
        }
        for contract in scan.contracts.contracts.values() {
            let path = self.root.join(&contract.path);
            if let Ok(meta) = fs::metadata(&path)
                && let Ok(mtime) = meta.modified()
            {
                max_mtime = Some(max_mtime.map_or(mtime, |m| m.max(mtime)));
            }
        }
        max_mtime
    }
}

pub(super) fn read_http_request(stream: &mut TcpStream) -> Result<Vec<u8>, std::io::Error> {
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

pub(super) struct Response {
    status: u16,
    content_type: &'static str,
    body: String,
}

pub(super) fn html(body: &str) -> Response {
    asset("text/html; charset=utf-8", body)
}

pub(super) fn asset(content_type: &'static str, body: &str) -> Response {
    Response {
        status: 200,
        content_type,
        body: body.to_owned(),
    }
}

/// Stamps the webui `schema_version` as the first key of a JSON object body so
/// every `/api/*` envelope shares one versioned contract from one constant.
/// `json` is the sole JSON `Response` constructor for the API surface, so the
/// stamp is universal and future endpoints are covered automatically.
fn versioned(body: &str) -> String {
    // `assert!` (not `debug_assert!`): this guards the sole API JSON constructor,
    // so the object precondition must hold in release too. It also makes the
    // `&body[1..]` slice below panic-safe (an empty or multi-byte-first body
    // fails here with a message instead of slicing on a non-char boundary).
    assert!(
        body.starts_with('{'),
        "json() body must be a JSON object so the schema_version stamp is valid"
    );
    if body == "{}" {
        return format!("{{\"schema_version\":{SCHEMA_VERSION}}}");
    }
    format!("{{\"schema_version\":{SCHEMA_VERSION},{}", &body[1..])
}

pub(super) fn json(status: u16, body: &str) -> Response {
    Response {
        status,
        content_type: "application/json; charset=utf-8",
        body: versioned(body),
    }
}

pub(super) fn text(status: u16, body: &str) -> Response {
    Response {
        status,
        content_type: "text/plain; charset=utf-8",
        body: body.to_owned(),
    }
}

pub(super) fn write_response(
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

pub(super) fn request_path(request: &str) -> Option<&str> {
    let mut parts = request.lines().next()?.split_whitespace();
    let method = parts.next()?;
    let path = parts.next()?;
    if method == "GET" {
        Some(path.split('?').next().unwrap_or(path))
    } else {
        None
    }
}

pub(super) fn open_browser(url: &str) {
    let result = if cfg!(target_os = "macos") {
        Command::new("open").arg(url).status()
    } else if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "start", url]).status()
    } else {
        Command::new("xdg-open").arg(url).status()
    };
    drop(result);
}
