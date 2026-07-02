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
    let mut ghost_ids = Vec::new();
    for node in graph.nodes.values() {
        if node.state == NodeState::Synced {
            let _ = writeln!(out, "- {}", node.id);
            has_synced = true;
        } else if node.state == NodeState::Ghost {
            ghost_ids.push(node.id.as_str());
        }
    }
    if !has_synced {
        let _ = writeln!(out, "None");
    }
    let _ = writeln!(out);
    let _ = writeln!(out, "## Ghost");
    if ghost_ids.is_empty() {
        let _ = writeln!(out, "None");
    } else {
        for id in ghost_ids {
            let _ = writeln!(out, "- {id}");
        }
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
    if let Ok(existing) = fs::read_to_string(&path)
        && existing == out
    {
        return Ok(());
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use tempfile::tempdir;

    use crate::blueprint::{NodeKind, ast::Span};
    use crate::map::graph::{Finding, FindingSeverity, Graph, NodeRecord, NodeState};

    use super::*;

    fn bare_node(id: &str) -> NodeRecord {
        NodeRecord {
            kind: NodeKind::Module,
            id: id.to_owned(),
            name: id.to_owned(),
            description: String::new(),
            tags: Vec::new(),
            parent: None,
            children: Vec::new(),
            paths: Vec::new(),
            owns_files: false,
            contracts: Vec::new(),
            state: NodeState::Synced,
            files: Vec::new(),
            symbols: Vec::new(),
            span: Span::point("test", 1, 1),
        }
    }

    fn bare_node_ghost(id: &str) -> NodeRecord {
        let mut node = bare_node(id);
        node.state = NodeState::Ghost;
        node
    }

    fn empty_graph() -> Graph {
        Graph {
            nodes: BTreeMap::new(),
            names: BTreeMap::new(),
            outbound: BTreeMap::new(),
            inbound: BTreeMap::new(),
            findings: Vec::new(),
        }
    }

    fn sample_finding(code: &str, severity: FindingSeverity, message: &str) -> Finding {
        Finding {
            code: code.to_owned(),
            severity,
            message: message.to_owned(),
            node: None,
            target: None,
            path: None,
        }
    }

    #[test]
    fn write_map_creates_expected_sections() {
        let tmp = tempdir().unwrap();
        let mut graph = empty_graph();
        graph.nodes.insert("sync-a".to_owned(), bare_node("sync-a"));
        graph
            .nodes
            .insert("ghost-b".to_owned(), bare_node_ghost("ghost-b"));
        graph
            .findings
            .push(sample_finding("F1", FindingSeverity::Error, "oops"));

        write_map(tmp.path(), &graph).unwrap();

        let content = std::fs::read_to_string(tmp.path().join("map.md")).unwrap();
        assert!(content.contains("generated: true"));
        assert!(content.contains("## Synced"));
        assert!(content.contains("- sync-a"));
        assert!(content.contains("## Ghost"));
        assert!(content.contains("- ghost-b"));
        assert!(content.contains("## Findings"));
        assert!(content.contains("Error: F1 oops"));
    }

    #[test]
    fn write_map_is_idempotent_when_content_unchanged() {
        let tmp = tempdir().unwrap();
        let graph = empty_graph();
        write_map(tmp.path(), &graph).unwrap();
        let first_modified = std::fs::metadata(tmp.path().join("map.md"))
            .unwrap()
            .modified()
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        write_map(tmp.path(), &graph).unwrap();
        let second_modified = std::fs::metadata(tmp.path().join("map.md"))
            .unwrap()
            .modified()
            .unwrap();
        assert_eq!(first_modified, second_modified);
    }

    #[test]
    fn append_log_creates_cairn_dir_and_appends_entry() {
        let tmp = tempdir().unwrap();
        let graph = empty_graph();
        append_log(tmp.path(), &graph).unwrap();
        let first = std::fs::read_to_string(tmp.path().join(".cairn/log.md")).unwrap();
        assert!(first.contains("scan: nodes=0, findings=0, errors=0"));
        append_log(tmp.path(), &graph).unwrap();
        let second = std::fs::read_to_string(tmp.path().join(".cairn/log.md")).unwrap();
        assert_eq!(second.lines().count(), 2);
    }

    #[test]
    fn append_log_counts_errors_only() {
        let tmp = tempdir().unwrap();
        let mut graph = empty_graph();
        graph
            .findings
            .push(sample_finding("W1", FindingSeverity::Warning, "warn"));
        graph
            .findings
            .push(sample_finding("E1", FindingSeverity::Error, "err"));
        append_log(tmp.path(), &graph).unwrap();
        let content = std::fs::read_to_string(tmp.path().join(".cairn/log.md")).unwrap();
        assert!(content.contains("scan: nodes=0, findings=2, errors=1"));
    }
}
