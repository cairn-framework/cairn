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
    let _ = writeln!(out, "## Orphaned");
    let mut orphaned_paths: Vec<&str> = graph
        .findings
        .iter()
        .filter(|f| f.code == "CAIRN_RECONCILE_ORPHANED_FILE")
        .filter_map(|f| f.path.as_deref())
        .collect();
    orphaned_paths.sort_unstable();
    orphaned_paths.dedup();
    if orphaned_paths.is_empty() {
        let _ = writeln!(out, "None");
    } else {
        for p in &orphaned_paths {
            let _ = writeln!(out, "- {p}");
        }
    }
    let _ = writeln!(out);
    let _ = writeln!(out, "## Active changes");
    let _ = writeln!(out);
    let _ = writeln!(out, "None in Phase 1.");
    let _ = writeln!(out);
    let _ = writeln!(out, "## Findings");
    let mut sorted: Vec<&_> = graph.findings.iter().collect();
    sorted.sort_by(|a, b| {
        fn rank(s: FindingSeverity) -> u8 {
            match s {
                FindingSeverity::Error => 0,
                FindingSeverity::Warning => 1,
                FindingSeverity::Info => 2,
            }
        }
        rank(a.severity)
            .cmp(&rank(b.severity))
            .then_with(|| a.code.cmp(&b.code))
            .then_with(|| a.message.cmp(&b.message))
    });
    if sorted.is_empty() {
        let _ = writeln!(out, "None");
    } else {
        for finding in sorted {
            let _ = writeln!(
                out,
                "- {}: {} {}",
                finding.severity.name(),
                finding.code,
                finding.message
            );
        }
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
            deferred_by: None,
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
        assert!(content.contains("## Orphaned"));
        assert!(content.contains("## Findings"));
        assert!(
            content.contains("error: F1 oops"),
            "findings must use canonical lowercase severity via .name()"
        );
    }

    #[test]
    fn write_map_renders_orphaned_section() {
        let tmp = tempdir().unwrap();
        let mut graph = empty_graph();
        graph.nodes.insert("sync-a".to_owned(), bare_node("sync-a"));
        // Production orphaned files come from findings, not NodeState.
        graph.findings.push(Finding {
            code: "CAIRN_RECONCILE_ORPHANED_FILE".to_owned(),
            severity: FindingSeverity::Info,
            message: "Rust file `src/stray.rs` is not owned by any eligible node".to_owned(),
            node: None,
            target: None,
            path: Some("src/stray.rs".to_owned()),
            deferred_by: None,
        });
        graph.findings.push(Finding {
            code: "CAIRN_RECONCILE_ORPHANED_FILE".to_owned(),
            severity: FindingSeverity::Info,
            message: "Rust file `src/another.rs` is not owned by any eligible node".to_owned(),
            node: None,
            target: None,
            path: Some("src/another.rs".to_owned()),
            deferred_by: None,
        });

        write_map(tmp.path(), &graph).unwrap();

        let content = std::fs::read_to_string(tmp.path().join("map.md")).unwrap();
        assert!(
            content.contains("- src/another.rs"),
            "orphaned file paths must appear in the map"
        );
        assert!(
            content.contains("- src/stray.rs"),
            "orphaned file paths must appear in the map"
        );
        // Verify deterministic sort order (another.rs < stray.rs).
        let another_pos = content.find("- src/another.rs").unwrap();
        let stray_pos = content.find("- src/stray.rs").unwrap();
        assert!(another_pos < stray_pos, "orphaned paths must be sorted");
        // Verify section order: Ghost before Orphaned before Findings.
        let ghost_pos = content.find("## Ghost").unwrap();
        let orphaned_pos = content.find("## Orphaned").unwrap();
        let findings_pos = content.find("## Findings").unwrap();
        assert!(
            ghost_pos < orphaned_pos && orphaned_pos < findings_pos,
            "section order must be Ghost < Orphaned < Findings"
        );
    }

    #[test]
    fn write_map_empty_orphaned_section_says_none() {
        let tmp = tempdir().unwrap();
        let graph = empty_graph();

        write_map(tmp.path(), &graph).unwrap();

        let content = std::fs::read_to_string(tmp.path().join("map.md")).unwrap();
        // Orphaned section exists and says None when empty.
        let orphaned_pos = content.find("## Orphaned").unwrap();
        let after_orphaned = &content[orphaned_pos..];
        let next_section = after_orphaned.find("## Active").unwrap();
        let orphaned_body = &after_orphaned[..next_section];
        assert!(
            orphaned_body.contains("None"),
            "empty orphaned section must say None"
        );
    }

    #[test]
    fn write_map_sorts_findings_by_severity() {
        let tmp = tempdir().unwrap();
        let mut graph = empty_graph();
        // Insert in reverse severity order: Info, Warning, Error.
        graph
            .findings
            .push(sample_finding("I1", FindingSeverity::Info, "info msg"));
        graph
            .findings
            .push(sample_finding("W1", FindingSeverity::Warning, "warn msg"));
        graph
            .findings
            .push(sample_finding("E1", FindingSeverity::Error, "err msg"));

        write_map(tmp.path(), &graph).unwrap();

        let content = std::fs::read_to_string(tmp.path().join("map.md")).unwrap();
        let error_pos = content.find("error: E1").unwrap();
        let warning_pos = content.find("warning: W1").unwrap();
        let info_pos = content.find("info: I1").unwrap();
        assert!(
            error_pos < warning_pos && warning_pos < info_pos,
            "findings must be sorted Error < Warning < Info by position: E@{error_pos} W@{warning_pos} I@{info_pos}"
        );
    }

    #[test]
    fn write_map_sorts_findings_deterministic_tie_breaker() {
        let tmp = tempdir().unwrap();
        let mut graph = empty_graph();
        // Two warnings: sort by code, then message.
        graph
            .findings
            .push(sample_finding("W2", FindingSeverity::Warning, "beta"));
        graph
            .findings
            .push(sample_finding("W1", FindingSeverity::Warning, "alpha"));

        write_map(tmp.path(), &graph).unwrap();

        let content = std::fs::read_to_string(tmp.path().join("map.md")).unwrap();
        let w1_pos = content.find("warning: W1").unwrap();
        let w2_pos = content.find("warning: W2").unwrap();
        assert!(
            w1_pos < w2_pos,
            "same-severity findings must be sorted by code: W1@{w1_pos} W2@{w2_pos}"
        );
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
