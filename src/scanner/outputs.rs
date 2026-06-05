//! Generated scanner outputs.
use crate::map::graph::{FindingSeverity, Graph, NodeState};
use std::{
    fmt::Write,
    fs::{self, OpenOptions},
    io::{self, Write as _},
    path::Path,
};

/// Writes generated `map.md`.
///
/// # Errors
///
/// Returns an I/O error when the file cannot be written.
pub fn write_map(root: &Path, graph: &Graph) -> io::Result<()> {
    let mut out = String::new();
    let _ = writeln!(out, "---");
    let _ = writeln!(out, "generated: true");
    let _ = writeln!(out, "---");
    let _ = writeln!(out);
    let _ = writeln!(out, "# Cairn Map");
    let _ = writeln!(out);
    let _ = writeln!(out, "## Synced");
    let mut has_synced = false;
    for node in graph.nodes.values() {
        if node.state == NodeState::Synced {
            let _ = writeln!(out, "- {}", node.id);
            has_synced = true;
        }
    }
    if !has_synced {
        let _ = writeln!(out, "None");
    }
    let _ = writeln!(out);
    let _ = writeln!(out, "## Ghost");
    let mut has_ghost = false;
    for node in graph.nodes.values() {
        if node.state == NodeState::Ghost {
            let _ = writeln!(out, "- {}", node.id);
            has_ghost = true;
        }
    }
    if !has_ghost {
        let _ = writeln!(out, "None");
    }
    let _ = writeln!(out);
    let _ = writeln!(out, "## Active changes");
    let _ = writeln!(out);
    let _ = writeln!(out, "None in Phase 1.");
    let _ = writeln!(out);
    let _ = writeln!(out, "## Findings");
    let mut has_findings = false;
    for finding in &graph.findings {
        let _ = writeln!(
            out,
            "- {:?}: {} {}",
            finding.severity, finding.code, finding.message
        );
        has_findings = true;
    }
    if !has_findings {
        let _ = writeln!(out, "None");
    }
    let path = root.join("map.md");
    if let Ok(existing) = fs::read_to_string(&path) {
        if existing == out {
            return Ok(());
        }
    }
    fs::write(path, out)
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
