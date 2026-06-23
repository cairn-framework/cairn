//! LSP diagnostic publishing for Cairn findings.

use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use camino::{Utf8Path, Utf8PathBuf};
use crossbeam_channel::Sender;
use lsp_server::Message;
use lsp_types::{
    Diagnostic, DiagnosticSeverity, NumberOrString, Position, PublishDiagnosticsParams, Range, Uri,
};

use crate::error::CairnError;
use crate::map::graph::{Finding, FindingSeverity};

/// Background scan interval floor in seconds.
pub const MIN_INTERVAL_SECS: u64 = 1;

/// Publishes Cairn findings as LSP diagnostics.
pub struct DiagnosticPublisher {
    sender: Sender<Message>,
    previous_uris: BTreeSet<String>,
}

impl DiagnosticPublisher {
    /// Creates a new publisher bound to the LSP message sender.
    #[must_use]
    pub fn new(sender: Sender<Message>) -> Self {
        Self {
            sender,
            previous_uris: BTreeSet::new(),
        }
    }

    /// Scans the project and publishes diagnostic deltas.
    ///
    /// Returns `true` if the LSP client is still connected.
    ///
    /// # Errors
    ///
    /// Returns an error if the project scan fails.
    pub fn scan_and_publish(
        &mut self,
        root: &Utf8Path,
        blueprint: &Utf8Path,
    ) -> Result<bool, CairnError> {
        let findings = project_findings(root, blueprint)?;
        let by_uri = findings_by_uri(&findings, root);
        let current_uris: BTreeSet<String> = by_uri.keys().cloned().collect();

        let mut connected = true;
        for uri in self.previous_uris.difference(&current_uris) {
            if !self.publish(uri, Vec::new()) {
                connected = false;
            }
        }
        for (uri, diagnostics) in by_uri {
            if !self.publish(&uri, diagnostics) {
                connected = false;
            }
        }

        self.previous_uris = current_uris;
        Ok(connected)
    }

    /// Sends a `textDocument/publishDiagnostics` notification.
    ///
    /// Returns `true` if the send succeeded (client still connected).
    fn publish(&self, uri: &str, diagnostics: Vec<Diagnostic>) -> bool {
        let Ok(uri_value) = Uri::from_str(uri) else {
            return true;
        };
        let params = PublishDiagnosticsParams::new(uri_value, diagnostics, None);
        let notification = lsp_server::Notification::new(
            "textDocument/publishDiagnostics".to_owned(),
            serde_json::to_value(params).unwrap_or(serde_json::Value::Null),
        );
        self.sender
            .send(Message::Notification(notification))
            .is_ok()
    }
}

/// Runs the diagnostic watch loop on a background thread.
pub fn start_watch_thread(
    sender: Sender<Message>,
    root: Utf8PathBuf,
    interval: Duration,
    stop: Arc<AtomicBool>,
) {
    std::thread::spawn(move || {
        let blueprint = root.join("cairn.blueprint");
        let mut publisher = DiagnosticPublisher::new(sender);
        while !stop.load(Ordering::SeqCst) {
            match publisher.scan_and_publish(&root, &blueprint) {
                Ok(continue_running) => {
                    if !continue_running {
                        break;
                    }
                }
                Err(error) => {
                    eprintln!("cairn-lsp: scan error: {error}");
                }
            }
            std::thread::sleep(interval);
        }
    });
}

/// Scans the project and returns its findings.
fn project_findings(root: &Utf8Path, blueprint: &Utf8Path) -> Result<Vec<Finding>, CairnError> {
    let result = crate::scanner::scan(root.as_std_path(), blueprint.as_std_path())
        .map_err(|error| CairnError::ScannerLoad { detail: error })?;
    Ok(result.graph.findings)
}

/// Groups findings by their file URI string.
fn findings_by_uri(findings: &[Finding], root: &Utf8Path) -> BTreeMap<String, Vec<Diagnostic>> {
    let mut map: BTreeMap<String, Vec<Diagnostic>> = BTreeMap::new();
    for finding in findings {
        let Some(uri) = finding_uri(finding, root) else {
            continue;
        };
        map.entry(uri)
            .or_default()
            .push(finding_to_diagnostic(finding));
    }
    map
}

/// Builds a `file://` URI string for a finding's path, resolved against the project root.
fn finding_uri(finding: &Finding, root: &Utf8Path) -> Option<String> {
    let path = finding.path.as_ref()?;
    let abs = if Utf8Path::new(path).is_absolute() {
        Utf8PathBuf::from(path)
    } else {
        root.join(path)
    };
    path_to_uri(&abs)
}

/// Converts an absolute filesystem path to a `file://` URI string.
fn path_to_uri(path: &Utf8Path) -> Option<String> {
    let abs = path
        .canonicalize_utf8()
        .unwrap_or_else(|_| path.to_path_buf());
    Some(format!("file://{abs}")).filter(|uri| Uri::from_str(uri).is_ok())
}

/// Converts a Cairn finding into an LSP diagnostic.
fn finding_to_diagnostic(finding: &Finding) -> Diagnostic {
    Diagnostic {
        range: file_start_range(),
        severity: Some(severity_to_lsp(finding.severity)),
        code: Some(NumberOrString::String(finding.code.clone())),
        code_description: None,
        source: Some("cairn".to_owned()),
        message: finding.message.clone(),
        related_information: None,
        tags: None,
        data: None,
    }
}

/// Returns a zero-length range at the start of a file.
const fn file_start_range() -> Range {
    Range {
        start: Position {
            line: 0,
            character: 0,
        },
        end: Position {
            line: 0,
            character: 0,
        },
    }
}

/// Maps a Cairn finding severity to an LSP diagnostic severity.
const fn severity_to_lsp(severity: FindingSeverity) -> DiagnosticSeverity {
    match severity {
        FindingSeverity::Error => DiagnosticSeverity::ERROR,
        FindingSeverity::Warning => DiagnosticSeverity::WARNING,
        FindingSeverity::Info => DiagnosticSeverity::INFORMATION,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finding_to_diagnostic_maps_severity_and_code() {
        let finding = Finding {
            code: "CAIRN_TEST".to_owned(),
            severity: FindingSeverity::Warning,
            message: "test message".to_owned(),
            node: None,
            target: None,
            path: None,
        };
        let diagnostic = finding_to_diagnostic(&finding);
        assert_eq!(diagnostic.severity, Some(DiagnosticSeverity::WARNING));
        assert_eq!(
            diagnostic.code,
            Some(NumberOrString::String("CAIRN_TEST".to_owned()))
        );
        assert_eq!(diagnostic.source, Some("cairn".to_owned()));
        assert_eq!(diagnostic.message, "test message");
    }

    #[test]
    fn test_severity_error_maps_to_error() {
        assert_eq!(
            severity_to_lsp(FindingSeverity::Error),
            DiagnosticSeverity::ERROR
        );
    }

    #[test]
    fn test_severity_info_maps_to_information() {
        assert_eq!(
            severity_to_lsp(FindingSeverity::Info),
            DiagnosticSeverity::INFORMATION
        );
    }

    #[test]
    fn test_path_to_uri_uses_file_scheme() {
        let uri = path_to_uri(Utf8Path::new("/tmp/example.rs")).expect("valid uri");
        assert!(uri.starts_with("file:///tmp/example.rs"));
    }

    #[test]
    fn test_findings_by_uri_groups_by_path() {
        let findings = vec![
            Finding {
                code: "A".to_owned(),
                severity: FindingSeverity::Error,
                message: "msg a".to_owned(),
                node: None,
                target: None,
                path: Some("src/a.rs".to_owned()),
            },
            Finding {
                code: "B".to_owned(),
                severity: FindingSeverity::Warning,
                message: "msg b".to_owned(),
                node: None,
                target: None,
                path: Some("src/a.rs".to_owned()),
            },
            Finding {
                code: "C".to_owned(),
                severity: FindingSeverity::Info,
                message: "msg c".to_owned(),
                node: None,
                target: None,
                path: Some("src/b.rs".to_owned()),
            },
        ];
        let by_uri = findings_by_uri(&findings, Utf8Path::new("/project"));
        assert_eq!(by_uri.len(), 2);
        let a_uri = path_to_uri(Utf8Path::new("/project/src/a.rs")).unwrap();
        assert_eq!(by_uri.get(&a_uri).unwrap().len(), 2);
    }
}
