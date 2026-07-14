//! LSP diagnostic publishing for Cairn findings.

use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
    sync::{Arc, atomic::AtomicBool},
    time::Duration,
};

use camino::{Utf8Path, Utf8PathBuf};
use crossbeam_channel::Sender;
use lsp_server::Message;
use lsp_types::{
    Diagnostic, DiagnosticSeverity, NumberOrString, Position, PublishDiagnosticsParams, Range, Uri,
};

use crate::map::graph::{Finding, FindingSeverity};
use crate::watch::{WatchEvent, WatchOpts};

/// Background scan interval floor in seconds.
pub const MIN_INTERVAL_SECS: u64 = 1;

/// Publishes Cairn findings as LSP diagnostics.
pub struct DiagnosticPublisher {
    sender: Sender<Message>,
    root: Utf8PathBuf,
    /// Current finding set, keyed by file URI, so `WatchEvent`s can be applied
    /// incrementally and only changed URIs republished.
    state: BTreeMap<String, Vec<Finding>>,
}

impl DiagnosticPublisher {
    /// Creates a new publisher bound to the LSP message sender.
    #[must_use]
    pub fn new(sender: Sender<Message>, root: Utf8PathBuf) -> Self {
        Self {
            sender,
            root,
            state: BTreeMap::new(),
        }
    }

    /// Applies a batch of watch events and publishes diagnostics for every
    /// affected URI. Returns an error (which stops the watch loop) when the
    /// LSP client has disconnected.
    ///
    /// # Errors
    ///
    /// Returns an error string when publishing to a disconnected client fails.
    pub fn apply_events(&mut self, events: &[WatchEvent]) -> Result<(), String> {
        let mut affected: BTreeSet<String> = BTreeSet::new();
        for event in events {
            let (finding, added) = match event {
                WatchEvent::FindingAdded { finding, .. } => (finding, true),
                WatchEvent::FindingResolved { finding, .. } => (finding, false),
            };
            let Some(uri) = finding_uri(finding, &self.root) else {
                continue;
            };
            if added {
                self.state
                    .entry(uri.clone())
                    .or_default()
                    .push(finding.clone());
            } else {
                let key = finding_key(finding);
                if let Some(entry) = self.state.get_mut(&uri) {
                    entry.retain(|f| finding_key(f) != key);
                }
            }
            affected.insert(uri);
        }

        let mut connected = true;
        for uri in affected {
            let diagnostics: Vec<Diagnostic> = self
                .state
                .get(&uri)
                .map(|findings| findings.iter().map(finding_to_diagnostic).collect())
                .unwrap_or_default();
            if !self.publish(&uri, diagnostics) {
                connected = false;
            }
            if let Some(findings) = self.state.get(&uri)
                && findings.is_empty()
            {
                self.state.remove(&uri);
            }
        }

        if connected {
            Ok(())
        } else {
            Err("cairn-lsp: client disconnected".to_owned())
        }
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

/// Runs the diagnostic watch loop on a background thread, sourcing findings
/// from the shared `lint` query operation rather than scanning directly.
pub fn start_watch_thread(
    sender: Sender<Message>,
    root: Utf8PathBuf,
    interval: Duration,
    stop: Arc<AtomicBool>,
) {
    std::thread::spawn(move || {
        let blueprint = root.join("cairn.blueprint");
        let mut publisher = DiagnosticPublisher::new(sender, root.clone());
        let opts = WatchOpts {
            interval_secs: interval.as_secs().max(1),
            once: false,
        };
        let scan = {
            let root = root.clone();
            let blueprint = blueprint.clone();
            move || {
                crate::query_api::lint_findings(root.as_std_path(), blueprint.as_std_path())
                    .map_err(|error| error.message)
            }
        };
        let on_diff = |events: &[WatchEvent]| publisher.apply_events(events);
        let _ = crate::watch::run_watch_loop(&opts, &stop, scan, on_diff);
    });
}

/// Groups findings by their file URI string.
#[cfg(test)]
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

/// Stable key for a finding, matching the diff algorithm in `crate::watch`.
fn finding_key(finding: &Finding) -> (String, Option<String>, Option<String>, Option<String>) {
    (
        finding.code.clone(),
        finding.node.clone(),
        finding.target.clone(),
        finding.path.clone(),
    )
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
            deferred_by: None,
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
                deferred_by: None,
            },
            Finding {
                code: "B".to_owned(),
                severity: FindingSeverity::Warning,
                message: "msg b".to_owned(),
                node: None,
                target: None,
                path: Some("src/a.rs".to_owned()),
                deferred_by: None,
            },
            Finding {
                code: "C".to_owned(),
                severity: FindingSeverity::Info,
                message: "msg c".to_owned(),
                node: None,
                target: None,
                path: Some("src/b.rs".to_owned()),
                deferred_by: None,
            },
        ];
        let by_uri = findings_by_uri(&findings, Utf8Path::new("/project"));
        assert_eq!(by_uri.len(), 2);
        let a_uri = path_to_uri(Utf8Path::new("/project/src/a.rs")).unwrap();
        assert_eq!(by_uri.get(&a_uri).unwrap().len(), 2);
    }
}
