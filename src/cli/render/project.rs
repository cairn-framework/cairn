//! Project-wide query renderers (context, status, dependencies).
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::format::{lines, node_arg, string_array_json, todos_json};
use super::super::*;
use super::{scan_error_count, scan_info_count, scan_warning_count};

// NOTE: render_context has no Config access, so it cannot show project_context
// (the context_json endpoint includes it). The backlog summary is text-only too.
pub(crate) fn render_context(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
) -> String {
    use std::fmt::Write as _;

    let system = scan_result
        .graph
        .nodes
        .values()
        .find(|n| n.kind == crate::blueprint::ast::NodeKind::System);
    let system_name = system.map_or("unknown", |n| n.name.as_str());
    let system_desc = system.map_or("", |n| n.description.as_str());

    let edge_count: usize = scan_result.graph.outbound.values().map(Vec::len).sum();
    let errors = scan_error_count(scan_result);
    let warnings = scan_warning_count(scan_result);
    let infos = scan_info_count(scan_result);

    let mut out = format!(
        "{} ({} nodes, {} edges)\n{}\n\nFindings: {} errors, {} warnings, {} info\n\nStructure:\n",
        system_name,
        scan_result.graph.nodes.len(),
        edge_count,
        system_desc,
        errors,
        warnings,
        infos,
    );

    let prefix = system.map(|s| format!("{}.", s.id)).unwrap_or_default();
    let opts = super::context_view::ContextOpts::parse(&parsed.command_args);
    let structure = if opts.mermaid {
        super::context_view::render_mermaid(&scan_result.graph, &opts, &prefix)
    } else {
        super::context_view::render_structure(&scan_result.graph, &opts, &prefix)
    };
    out.push_str(&structure);

    let ac = &scan_result.artefacts;
    write!(
        out,
        "\nArtefacts: {} contracts, {} decisions, {} todos, {} research, {} reviews, {} sources\n",
        ac.contracts.contracts.len(),
        ac.decisions.len(),
        ac.todos.len(),
        ac.research.len(),
        ac.reviews.len(),
        ac.sources.len(),
    )
    .unwrap();

    let backlog = crate::state::backlog::read(root);
    let ready = crate::state::backlog::ready(&backlog);
    let _ = write!(out, "\nBacklog: {} ready\n", ready.len());
    for item in ready.iter().take(5) {
        let _ = writeln!(out, "  {} [P{}] {}", item.id, item.priority, item.title);
    }

    out
}

/// Renders the beads (issues) linked to a node via their `cairn-node:<id>`
/// label, the CLI counterpart of the webui inspector's beads panel.
pub(crate) fn render_backlog(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    use std::fmt::Write as _;
    node_arg(&parsed.command_args).and_then(|node| {
        let node = scan_result.graph.resolve(node)?;
        let items = crate::state::backlog::read(root);
        let beads = crate::state::backlog::for_node(&items, &node.id);
        Ok(if parsed.json {
            let arr = beads
                .iter()
                .map(|b| b.to_json().to_string())
                .collect::<Vec<_>>()
                .join(",");
            format!("{{\"node\":\"{}\",\"beads\":[{arr}]}}\n", esc(&node.id))
        } else if beads.is_empty() {
            format!(
                "{}\n",
                crate::copy::lookup("empty-states.node-no-beads.body")
            )
        } else {
            let mut out = format!("Beads for {}:\n", node.id);
            for b in &beads {
                let _ = writeln!(
                    out,
                    "  {} [P{}] [{}] {}",
                    b.id, b.priority, b.status, b.title
                );
            }
            out
        })
    })
}

pub(crate) fn render_status(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
    root: &Path,
) -> String {
    let open = scan_result
        .artefacts
        .todos
        .iter()
        .filter(|todo| todo.status == TodoStatus::Open || todo.status == TodoStatus::InProgress)
        .cloned()
        .collect::<Vec<_>>();
    let log_entries = fs::read_to_string(root.join(".cairn/log.md"))
        .map(|content| {
            content
                .lines()
                .rev()
                .take(5)
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let backlog = crate::state::backlog::read(root);
    let ready = crate::state::backlog::ready(&backlog);
    let next_recommended = ready.first();
    if parsed.json {
        format!(
            "{{\"active_changes\":[],\"open_todos\":{},\"recent_log_entries\":{}}}\n",
            todos_json(&open),
            string_array_json(&log_entries)
        )
    } else {
        format!(
            "Status:\nActive changes:\nNone\nOpen todos:\n{}\nRecent log entries:\n{}\nNext recommended:\n{}\n",
            lines(
                &open
                    .iter()
                    .map(super::super::format::todo_line)
                    .collect::<Vec<_>>()
            ),
            lines(&log_entries),
            next_recommended.map_or_else(
                || "None".to_owned(),
                |top| format!("{} [P{}] {}", top.id, top.priority, top.title)
            )
        )
    }
}

pub(crate) fn render_dependencies(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    let transitive = parsed.command_args.iter().any(|arg| arg == "--transitive");
    node_arg(&parsed.command_args).and_then(|node| {
        let response = if parsed.command == "depends" {
            query::depends(&scan_result.graph, node, transitive)
        } else {
            query::dependents(&scan_result.graph, node, transitive)
        }?;
        Ok(if parsed.json {
            format!(
                "{{\"node\":\"{}\",\"nodes\":{}}}\n",
                esc(&response.node),
                string_array_json(&response.nodes)
            )
        } else {
            format!("{}:\n{}\n", response.node, lines(&response.nodes))
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        artefacts::registry::{Todo, TodoStatus},
        blueprint::{NodeKind, Span},
        map::{Graph, NodeRecord, NodeState},
        scanner::{ScanResult, state::TargetHashes},
    };
    use std::collections::BTreeMap;

    fn node_record(id: &str) -> NodeRecord {
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
            symbols: Vec::new(),
            contracts: Vec::new(),
            state: NodeState::Synced,
            files: Vec::new(),
            span: Span::point("test", 1, 1),
        }
    }

    fn system(id: &str, name: &str, desc: &str) -> NodeRecord {
        NodeRecord {
            kind: NodeKind::System,
            id: id.to_owned(),
            name: name.to_owned(),
            description: desc.to_owned(),
            tags: Vec::new(),
            parent: None,
            children: Vec::new(),
            paths: Vec::new(),
            owns_files: false,
            symbols: Vec::new(),
            contracts: Vec::new(),
            state: NodeState::Synced,
            files: Vec::new(),
            span: Span::point("test", 1, 1),
        }
    }

    fn scan_with_nodes(nodes: Vec<NodeRecord>) -> ScanResult {
        let graph_nodes: BTreeMap<String, NodeRecord> =
            nodes.into_iter().map(|n| (n.id.clone(), n)).collect();
        ScanResult {
            graph: Graph {
                nodes: graph_nodes,
                names: BTreeMap::new(),
                outbound: BTreeMap::new(),
                inbound: BTreeMap::new(),
                findings: Vec::new(),
            },
            artefacts: crate::artefacts::registry::ArtefactSet::default(),
            contracts: crate::artefacts::contract::ContractSet::default(),
            interface_hash: String::new(),
            target_reports: Vec::new(),
            target_hashes: TargetHashes::default(),
            blueprint_snapshot: crate::scanner::state::BlueprintSnapshot::default(),
        }
    }

    fn scan_with_todos(todos: Vec<Todo>) -> ScanResult {
        let mut nodes = BTreeMap::new();
        nodes.insert("app".to_owned(), node_record("app"));
        ScanResult {
            graph: Graph {
                nodes,
                names: BTreeMap::new(),
                outbound: BTreeMap::new(),
                inbound: BTreeMap::new(),
                findings: Vec::new(),
            },
            artefacts: crate::artefacts::registry::ArtefactSet {
                todos,
                ..Default::default()
            },
            contracts: crate::artefacts::contract::ContractSet::default(),
            interface_hash: String::new(),
            target_reports: Vec::new(),
            target_hashes: TargetHashes::default(),
            blueprint_snapshot: crate::scanner::state::BlueprintSnapshot::default(),
        }
    }

    fn parsed(json: bool) -> ParsedArgs {
        ParsedArgs {
            json,
            strict: false,
            file: std::path::PathBuf::from("cairn.blueprint"),
            changes_dir: std::path::PathBuf::from("meta/changes"),
            command: "status".to_owned(),
            command_args: vec!["status".to_owned()],
        }
    }

    fn todo(status: TodoStatus) -> Todo {
        Todo {
            path: "./todo.md".to_owned(),
            node: "app".to_owned(),
            status,
            created: "2026-01-01".to_owned(),
            satisfies: None,
            body: String::new(),
        }
    }

    #[test]
    fn render_status_human_lists_open_and_in_progress_todos() {
        let scan = scan_with_todos(vec![todo(TodoStatus::Open), todo(TodoStatus::Done)]);
        let rendered = render_status(&parsed(false), &scan, std::path::Path::new("."));
        assert!(rendered.contains("Status:"));
        assert!(rendered.contains("[open]"));
        assert!(!rendered.contains("[done]"));
    }

    #[test]
    fn render_status_human_empty_todos_renders_none() {
        let scan = scan_with_todos(Vec::new());
        let rendered = render_status(&parsed(false), &scan, std::path::Path::new("."));
        assert!(rendered.contains("Open todos:"));
        assert!(!rendered.contains("[open]"));
    }

    #[test]
    fn render_status_json_includes_open_todos() {
        let scan = scan_with_todos(vec![todo(TodoStatus::InProgress)]);
        let rendered = render_status(&parsed(true), &scan, std::path::Path::new("."));
        assert!(rendered.contains("\"open_todos\""));
        assert!(rendered.contains("\"active_changes\""));
        assert!(rendered.contains("\"recent_log_entries\""));
    }

    #[test]
    fn render_status_json_omits_done_todos() {
        let scan = scan_with_todos(vec![todo(TodoStatus::Done)]);
        let rendered = render_status(&parsed(true), &scan, std::path::Path::new("."));
        assert!(!rendered.contains("\"node\":\"app\""));
        assert!(!rendered.contains("in-progress"));
        assert!(!rendered.contains("\"status\":\"open\""));
    }

    #[test]
    fn render_context_shows_system_name_and_counts() {
        let scan = scan_with_nodes(vec![
            system("sys", "MySystem", "A test system"),
            node_record("app"),
        ]);
        let rendered = render_context(&parsed(false), std::path::Path::new("/nonexistent"), &scan);
        assert!(rendered.contains("MySystem (2 nodes, 0 edges)"));
        assert!(rendered.contains("A test system"));
        assert!(rendered.contains("Findings: 0 errors, 0 warnings, 0 info"));
        assert!(rendered.contains(
            "Artefacts: 0 contracts, 0 decisions, 0 todos, 0 research, 0 reviews, 0 sources"
        ));
    }

    #[test]
    fn render_context_node_line_omits_path_and_synced_state() {
        let mut app = node_record("app");
        app.paths = vec!["./src".to_owned()];
        let scan = scan_with_nodes(vec![system("sys", "Sys", ""), app]);
        let rendered = render_context(&parsed(false), std::path::Path::new("/nonexistent"), &scan);
        assert!(
            rendered.contains("Structure:\n  app\n"),
            "node line: {rendered}"
        );
        assert!(
            !rendered.contains("./src"),
            "path must be dropped: {rendered}"
        );
    }

    #[test]
    fn render_context_lists_labeled_dependencies() {
        let mut scan = scan_with_nodes(vec![
            system("sys", "Sys", ""),
            node_record("app.a"),
            node_record("app.b"),
        ]);
        scan.graph.outbound.insert(
            "app.a".to_owned(),
            vec![crate::map::graph::EdgeRef {
                from: "app.a".to_owned(),
                to: "app.b".to_owned(),
                description: "calls".to_owned(),
            }],
        );
        let rendered = render_context(&parsed(false), std::path::Path::new("/nonexistent"), &scan);
        assert!(
            rendered.contains("  app.a\n    -> app.b  # calls"),
            "missing labeled edge under source: {rendered}"
        );
    }

    #[test]
    fn render_context_defaults_when_no_system() {
        let scan = scan_with_nodes(vec![node_record("app")]);
        let rendered = render_context(&parsed(false), std::path::Path::new("/nonexistent"), &scan);
        assert!(rendered.contains("unknown (1 nodes, 0 edges)"));
    }

    #[test]
    fn render_context_includes_backlog_section() {
        let dir = std::env::temp_dir().join(format!("cairn-ctx-backlog-{}", std::process::id()));
        let beads = dir.join(".beads");
        std::fs::create_dir_all(&beads).unwrap();
        std::fs::write(
            beads.join("issues.jsonl"),
            r#"{"id":"cairn-aaa","title":"Do thing","status":"open","priority":2,"issue_type":"task"}"#,
        )
        .unwrap();
        let scan = scan_with_nodes(vec![node_record("app")]);
        let rendered = render_context(&parsed(false), &dir, &scan);
        assert!(rendered.contains("Backlog: 1 ready"));
        assert!(rendered.contains("cairn-aaa [P2] Do thing"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn backlog_args(node: &str, json: bool) -> ParsedArgs {
        let mut p = parsed(json);
        p.command = "backlog".to_owned();
        p.command_args = vec!["backlog".to_owned(), node.to_owned()];
        p
    }

    fn with_beads(tag: &str, lines: &str) -> std::path::PathBuf {
        let dir =
            std::env::temp_dir().join(format!("cairn-backlog-render-{tag}-{}", std::process::id()));
        let beads = dir.join(".beads");
        std::fs::create_dir_all(&beads).unwrap();
        std::fs::write(beads.join("issues.jsonl"), lines).unwrap();
        dir
    }

    #[test]
    fn render_backlog_lists_node_linked_beads() {
        let dir = with_beads(
            "human",
            r#"{"id":"cairn-z","title":"Wire it","status":"open","priority":1,"issue_type":"task","labels":["cairn-node:app"]}"#,
        );
        let scan = scan_with_nodes(vec![node_record("app")]);
        let rendered = render_backlog(&backlog_args("app", false), &dir, &scan).unwrap();
        assert!(rendered.contains("Beads for app:"), "{rendered}");
        assert!(
            rendered.contains("cairn-z [P1] [open] Wire it"),
            "{rendered}"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn render_backlog_json_emits_beads_array() {
        let dir = with_beads(
            "json",
            r#"{"id":"cairn-z","title":"Wire it","status":"open","priority":1,"issue_type":"task","labels":["cairn-node:app"]}"#,
        );
        let scan = scan_with_nodes(vec![node_record("app")]);
        let rendered = render_backlog(&backlog_args("app", true), &dir, &scan).unwrap();
        let value: serde_json::Value = serde_json::from_str(&rendered).expect("valid JSON");
        assert_eq!(value["node"], "app");
        let beads = value["beads"].as_array().expect("beads array");
        assert_eq!(beads.len(), 1);
        assert_eq!(beads[0]["id"], "cairn-z");
        assert_eq!(beads[0]["title"], "Wire it");
        assert_eq!(beads[0]["status"], "open");
        assert_eq!(beads[0]["priority"], 1);
        assert_eq!(beads[0]["issue_type"], "task");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn render_backlog_empty_node_uses_copy() {
        let dir = with_beads(
            "empty",
            r#"{"id":"cairn-other","title":"X","status":"open","priority":2,"issue_type":"task","labels":["cairn-node:other"]}"#,
        );
        let scan = scan_with_nodes(vec![node_record("app")]);
        let rendered = render_backlog(&backlog_args("app", false), &dir, &scan).unwrap();
        let expected = crate::copy::lookup("empty-states.node-no-beads.body");
        assert!(rendered.contains(expected), "{rendered}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn render_backlog_unknown_node_errs() {
        let dir = with_beads("unknown", "");
        let scan = scan_with_nodes(vec![node_record("app")]);
        assert!(render_backlog(&backlog_args("missing", false), &dir, &scan).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
