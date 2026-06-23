//! LSP server loop for Cairn diagnostics.

use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use camino::Utf8PathBuf;
use crossbeam_channel::Sender;
use lsp_server::{Connection, Message, Request, Response};
use lsp_types::{
    InitializeParams, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, Uri,
    WorkspaceFoldersServerCapabilities,
};
use serde_json::Value;

use crate::error::CairnError;

use super::{
    LspOpts,
    diagnostics::{MIN_INTERVAL_SECS, start_watch_thread},
};

/// Runs the Cairn LSP diagnostics server over the supplied connection.
///
/// # Errors
///
pub fn run(connection: &Connection, opts: &LspOpts) -> Result<(), CairnError> {
    let capabilities =
        serde_json::to_value(server_capabilities()).map_err(|error| CairnError::Lsp {
            message: format!("serialize capabilities: {error}"),
        })?;
    let init_params = connection
        .initialize(capabilities)
        .map_err(|error| CairnError::Lsp {
            message: error.to_string(),
        })?;
    let params: InitializeParams =
        serde_json::from_value(init_params).map_err(|error| CairnError::Lsp {
            message: format!("parse init params: {error}"),
        })?;

    let root = resolve_root(&params, opts)?;
    let interval = Duration::from_secs(opts.interval_secs.max(MIN_INTERVAL_SECS));
    let stop = Arc::new(AtomicBool::new(false));

    start_watch_thread(connection.sender.clone(), root, interval, Arc::clone(&stop));

    let mut state = State::Running;
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => handle_request(&connection.sender, &req, &mut state),
            Message::Notification(not) => {
                if not.method == "exit" {
                    break;
                }
            }
            Message::Response(_) => {}
        }
    }

    stop.store(true, Ordering::SeqCst);
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    Running,
    ShuttingDown,
}

fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        workspace: Some(lsp_types::WorkspaceServerCapabilities {
            workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                supported: Some(true),
                change_notifications: Some(lsp_types::OneOf::Left(true)),
            }),
            file_operations: None,
        }),
        ..ServerCapabilities::default()
    }
}

fn resolve_root(params: &InitializeParams, opts: &LspOpts) -> Result<Utf8PathBuf, CairnError> {
    if let Some(root) = &opts.root {
        return Ok(root.clone());
    }
    if let Some(folder) = params
        .workspace_folders
        .as_ref()
        .and_then(|folders| folders.first())
    {
        return uri_to_path(&folder.uri);
    }
    // Reason: root_uri is deprecated by LSP 3.17 but still sent by older
    // clients; workspace_folders is checked first.
    #[allow(deprecated)]
    if let Some(root_uri) = &params.root_uri {
        return uri_to_path(root_uri);
    }
    let cwd = std::env::current_dir().map_err(|error| CairnError::Lsp {
        message: format!("no workspace folder and cannot get cwd: {error}"),
    })?;
    Utf8PathBuf::try_from(cwd).map_err(|error| CairnError::Lsp {
        message: format!("cwd is not utf-8: {error:?}"),
    })
}

fn uri_to_path(uri: &Uri) -> Result<Utf8PathBuf, CairnError> {
    let path = uri
        .path()
        .as_estr()
        .decode()
        .into_string()
        .map_err(|error| CairnError::Lsp {
            message: format!("invalid uri path: {error}"),
        })?;
    // Reason: Windows file URIs look like `file:///C:/foo`; fluent_uri
    // returns `/C:/foo`, so strip the leading slash when a drive letter
    // follows. This is a best-effort heuristic for Unix-like hosts.
    let mut path = path.into_owned();
    if is_windows_drive_path(&path) {
        path.remove(0);
    }
    Ok(Utf8PathBuf::from(path))
}

fn is_windows_drive_path(path: &str) -> bool {
    let bytes = path.as_bytes();
    bytes.len() >= 3 && bytes[0] == b'/' && bytes[1].is_ascii_alphabetic() && bytes[2] == b':'
}

fn handle_request(sender: &Sender<Message>, req: &Request, state: &mut State) {
    match state {
        State::ShuttingDown => respond_server_shutdown(sender, req.id.clone()),
        State::Running => {
            if req.method == "shutdown" {
                *state = State::ShuttingDown;
                respond_ok(sender, req.id.clone(), Value::Null);
            } else {
                respond_method_not_found(sender, req.id.clone(), &req.method);
            }
        }
    }
}

fn respond_ok(sender: &Sender<Message>, id: lsp_server::RequestId, result: Value) {
    let resp = Response::new_ok(id, result);
    sender.send(Message::Response(resp)).ok();
}

fn respond_server_shutdown(sender: &Sender<Message>, id: lsp_server::RequestId) {
    let resp = Response::new_err(
        id,
        lsp_server::ErrorCode::InvalidRequest as i32,
        "server is shutting down".to_owned(),
    );
    sender.send(Message::Response(resp)).ok();
}

fn respond_method_not_found(sender: &Sender<Message>, id: lsp_server::RequestId, method: &str) {
    let resp = Response::new_err(
        id,
        lsp_server::ErrorCode::MethodNotFound as i32,
        format!("method not found: {method}"),
    );
    sender.send(Message::Response(resp)).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uri_to_path_decodes_percent_encoding() {
        let uri = "file:///tmp/my%20file.rs".parse::<Uri>().unwrap();
        assert_eq!(
            uri_to_path(&uri).unwrap(),
            Utf8PathBuf::from("/tmp/my file.rs")
        );
    }

    #[test]
    fn test_uri_to_path_strips_windows_drive_letter_slash() {
        let uri = "file:///C:/project".parse::<Uri>().unwrap();
        assert_eq!(uri_to_path(&uri).unwrap(), Utf8PathBuf::from("C:/project"));
    }

    #[test]
    fn test_server_capabilities_declares_full_sync() {
        let caps = server_capabilities();
        assert_eq!(
            caps.text_document_sync,
            Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL))
        );
        assert!(caps.workspace.is_some());
    }

    #[test]
    fn test_resolve_root_prefers_opts_override() {
        let opts = LspOpts {
            root: Some(Utf8PathBuf::from("/override")),
            interval_secs: 1,
        };
        let params = InitializeParams::default();
        assert_eq!(
            resolve_root(&params, &opts).unwrap(),
            Utf8PathBuf::from("/override")
        );
    }

    #[test]
    fn test_run_performs_initialize_handshake() {
        let (server_conn, client_conn) = Connection::memory();
        let opts = LspOpts {
            root: Some(Utf8PathBuf::from(".")),
            interval_secs: 60,
        };

        let server = std::thread::spawn(move || run(&server_conn, &opts));

        let init_id = lsp_server::RequestId::from(1);
        let init_req = lsp_server::Request::new(
            init_id.clone(),
            "initialize".to_owned(),
            serde_json::to_value(InitializeParams::default()).unwrap(),
        );
        client_conn.sender.send(Message::Request(init_req)).unwrap();

        let resp = recv_response(&client_conn.receiver);
        assert!(resp.result.is_some(), "initialize failed: {resp:?}");

        client_conn
            .sender
            .send(
                lsp_server::Notification::new(
                    "initialized".to_owned(),
                    Value::Object(serde_json::Map::new()),
                )
                .into(),
            )
            .unwrap();

        let shutdown_id = lsp_server::RequestId::from(2);
        client_conn
            .sender
            .send(Message::Request(lsp_server::Request::new(
                shutdown_id,
                "shutdown".to_owned(),
                Value::Null,
            )))
            .unwrap();
        let resp = recv_response(&client_conn.receiver);
        assert!(resp.error.is_none(), "shutdown failed: {resp:?}");

        client_conn
            .sender
            .send(lsp_server::Notification::new("exit".to_owned(), Value::Null).into())
            .unwrap();

        server
            .join()
            .expect("server panicked")
            .expect("server error");
    }

    fn recv_response(receiver: &crossbeam_channel::Receiver<Message>) -> Response {
        loop {
            match receiver.recv().unwrap() {
                Message::Response(resp) => return resp,
                Message::Notification(_) => {}
                Message::Request(req) => panic!("unexpected request: {req:?}"),
            }
        }
    }
}
