//! Generated scanner outputs.

use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
};

use crate::{
    changes,
    map::graph::{FindingSeverity, Graph, NodeState},
};

/// Writes generated `map.md`.
///
/// # Errors
///
/// Returns an I/O error when the file cannot be written.
pub fn write_map(root: &Path, graph: &Graph) -> io::Result<()> {
    let mut synced = Vec::new();
    let mut ghost = Vec::new();
    for node in graph.nodes.values() {
        match node.state {
            NodeState::Synced => synced.push(node.id.clone()),
            NodeState::Ghost => ghost.push(node.id.clone()),
            NodeState::Orphaned => {}
        }
    }
    let findings = graph
        .findings
        .iter()
        .map(|finding| {
            format!(
                "- {:?}: {} {}",
                finding.severity, finding.code, finding.message
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let active_changes = changes::discover(root)
        .map(|items| changes::active_changes_lines(&items))
        .unwrap_or_default();
    fs::write(
        root.join("map.md"),
        format!(
            "---\ngenerated: true\n---\n\n# Cairn Map\n\n## Synced\n{}\n\n## Ghost\n{}\n\n## Active changes\n\n{}\n\n## Findings\n{}\n",
            bullet_list(&synced),
            bullet_list(&ghost),
            bullet_list(&active_changes),
            if findings.is_empty() {
                "None".to_owned()
            } else {
                findings
            },
        ),
    )
}

/// Appends `.cairn/log.md` scan event.
///
/// # Errors
///
/// Returns an I/O error when the log directory or file cannot be written.
pub fn append_log(root: &Path, graph: &Graph) -> io::Result<()> {
    fs::create_dir_all(root.join(".cairn"))?;
    let error_count = graph
        .findings
        .iter()
        .filter(|finding| finding.severity == FindingSeverity::Error)
        .count();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(root.join(".cairn/log.md"))?;
    writeln!(
        file,
        "- scan: nodes={}, findings={}, errors={}",
        graph.nodes.len(),
        graph.findings.len(),
        error_count
    )
}

fn bullet_list(values: &[String]) -> String {
    if values.is_empty() {
        "None".to_owned()
    } else {
        values
            .iter()
            .map(|value| format!("- {value}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
