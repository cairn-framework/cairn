//! Parked-finding classification from todo `defers:` references.

use std::path::Path;

use super::ArtefactSet;
use crate::artefacts::registry::TodoStatus;
use crate::map::Graph;
use crate::map::graph::{Finding, FindingSeverity};

/// Classifies parked findings and validates every todo `defers:` reference
/// (`todo.lint-selection-folding` item 1a).
///
/// A reference is a finding code plus the path or node it was raised
/// against. While its todo is `blocked`, a matching reference parks each
/// matching Info finding by setting `parked_by` to the todo id: a
/// report-level classification that renderers annotate and the query wire
/// publishes, never a suppression. Two misuses are findings of their own,
/// whatever the todo's status, so the links cannot rot silently: a
/// reference matching no emitted finding raises
/// `CAIRN_TODO_DEFERS_UNMATCHED`, and a reference whose match is an Error
/// or Warning raises `CAIRN_TODO_DEFERS_BLOCKING` and parks nothing, so
/// nothing can park a blocking finding by accident or on purpose.
///
/// Artefact loaders record root-joined paths, while `defers:` references are
/// authored root-relative, so both sides are reduced to root-relative form
/// before comparison.
///
/// Must run after every finding-emitting check so it sees the complete set.
pub(crate) fn check_todo_defers(graph: &mut Graph, artefacts: &ArtefactSet, root: &Path) {
    let relative = |path: &str| -> String {
        // Component-wise: a sibling of the root (`<root>-other/...`) is left
        // untouched rather than mangled by a string prefix strip.
        let candidate = Path::new(path);
        let stripped = candidate.strip_prefix(root).unwrap_or(candidate);
        let text = stripped.to_string_lossy();
        text.strip_prefix("./").unwrap_or(&text).to_owned()
    };
    let mut raised = Vec::new();
    for todo in &artefacts.todos {
        if todo.defers.is_empty() {
            continue;
        }
        let todo_id = Path::new(&todo.path)
            .file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or(&todo.path)
            .to_owned();
        for reference in &todo.defers {
            let location = relative(&reference.location);
            // Phase one: collect matches without touching anything, so a
            // reference that hits any Error or Warning parks nothing at all,
            // not even an Info sibling at the same code and location.
            let mut matched_indices = Vec::new();
            let mut blocking = None;
            for (index, finding) in graph.findings.iter().enumerate() {
                if finding.code != reference.code {
                    continue;
                }
                let hits_path = finding
                    .path
                    .as_deref()
                    .is_some_and(|path| relative(path) == location);
                let hits_node = finding.node.as_deref() == Some(location.as_str());
                if !(hits_path || hits_node) {
                    continue;
                }
                matched_indices.push(index);
                if finding.severity != FindingSeverity::Info && blocking.is_none() {
                    blocking = Some(finding.severity);
                }
            }
            let matched = !matched_indices.is_empty();
            // Phase two: park only when every match is Info.
            if matched && blocking.is_none() && todo.status == TodoStatus::Blocked {
                for index in matched_indices {
                    let finding = &mut graph.findings[index];
                    // A decision-deferred finding stays under the deferral
                    // regime (`dec.loop-selection-deferred-findings`); parking
                    // never re-classifies it, but the reference still counts
                    // as matched so it is not reported stale.
                    if finding.deferred_by.is_none() && finding.parked_by.is_none() {
                        finding.parked_by = Some(todo_id.clone());
                    }
                }
            }
            let (code, extra) = if !matched {
                ("CAIRN_TODO_DEFERS_UNMATCHED", None)
            } else if let Some(severity) = blocking {
                ("CAIRN_TODO_DEFERS_BLOCKING", Some(severity))
            } else {
                continue;
            };
            let mut message = crate::copy::lookup(&format!("findings.codes.{code}.body"))
                .replace("{todo}", &todo_id)
                .replace("{code}", &reference.code)
                .replace("{location}", &reference.location);
            if let Some(severity) = extra {
                message = message.replace("{severity}", severity.name());
            }
            raised.push(Finding {
                code: code.to_owned(),
                severity: FindingSeverity::Warning,
                message,
                node: None,
                target: Some(format!("{} {}", reference.code, reference.location)),
                path: Some(todo.path.clone()),
                deferred_by: None,
                parked_by: None,
            });
        }
    }
    graph.findings.extend(raised);
}
