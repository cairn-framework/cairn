//! Node-level query renderers.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::format::{
    decision_index, decision_line_with_index, decisions_json, lines, node_arg, render_node,
};
use super::super::*;
use super::{scan_error_count, scan_error_warning};
use crate::query_api::{QueryRequest, accepted_decisions, neighbourhood_ids, research_for_nodes};
use serde_json::Value;
use std::collections::BTreeSet;

pub(crate) fn render_get(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    node_arg(&parsed.command_args).and_then(|node| {
        match query::get(&scan_result.graph, node) {
            Ok(response) => {
                let mut output = render_node(&response.node, parsed.json);
                let decisions = accepted_decisions(scan_result, &response.node.id);
                let records = decisions
                    .iter()
                    .map(|decision| (*decision).clone())
                    .collect::<Vec<_>>();
                if parsed.json {
                    let decision_json = decisions_json(&records);
                    if let Some(prefix) = output.strip_suffix("}\n") {
                        output = format!("{prefix},\"decisions\":{decision_json}}}\n");
                    }
                } else {
                    use std::fmt::Write;
                    let index = decision_index(&scan_result.artefacts.decisions);
                    let _ = write!(
                        output,
                        "Accepted decisions:\n{}\n",
                        lines(
                            &decisions
                                .iter()
                                .map(|decision| decision_line_with_index(decision, &index))
                                .collect::<Vec<_>>(),
                        )
                    );
                }
                if parsed.command_args.iter().any(|arg| arg == "--symbols") {
                    output.push_str(&symbols_block(scan_result, node));
                }
                Ok(output)
            }
            // A graph node wins; otherwise resolve a beads task id so the loop
            // can `cairn get <bead>` and see the task plus the node it touches.
            // JSON/MCP `get` stays strictly node-typed.
            Err(finding) => crate::state::backlog::find(root, node)
                .filter(|_| !parsed.json)
                .map(|item| render_backlog_item(&item))
                .ok_or(finding),
        }
    })
}

fn render_backlog_item(item: &crate::state::backlog::BacklogItem) -> String {
    let linked = item.linked_node().unwrap_or("(unlinked)");
    format!(
        "Task {} [{}, P{}] {}\n  {}\n  linked node: {}\n  Description: {}\n",
        item.id, item.status, item.priority, item.issue_type, item.title, linked, item.description
    )
}

/// Node-scoped active-change operation lines for a neighbourhood.
fn change_lines_for(
    root: &Path,
    changes_dir: &Path,
    node_ids: &std::collections::BTreeSet<String>,
) -> Vec<String> {
    let changes = crate::changes::discover(root, changes_dir).unwrap_or_default();
    crate::changes::operations_for_nodes(&changes, node_ids)
}

pub(crate) fn render_neighbourhood(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    node_arg(&parsed.command_args).and_then(|node| {
        // Same query the JSON handler uses (query_api::handlers::graph), so
        // human and JSON output always show the same inbound/outbound edges.
        let response = query::neighbourhood(&scan_result.graph, node)?;
        Ok({
            let include_todos = parsed.command_args.iter().any(|arg| arg == "--include-todos");
            let include_research = parsed
                .command_args
                .iter()
                .any(|arg| arg == "--include-research");
            let include_reviews = parsed
                .command_args
                .iter()
                .any(|arg| arg == "--include-reviews");
            let include_deprecated = parsed
                .command_args
                .iter()
                .any(|arg| arg == "--include-deprecated-decisions");
            let include_changes = parsed
                .command_args
                .iter()
                .any(|arg| arg == "--include-changes");
            let node_ids = neighbourhood_ids(&scan_result.graph, &response.node.id);
            let decisions = scan_result
                .artefacts
                .decisions
                .iter()
                .filter(|decision| {
                    decision.nodes.iter().any(|node| node_ids.contains(node))
                        && (decision.status == DecisionStatus::Accepted || include_deprecated)
                })
                .cloned()
                .collect::<Vec<_>>();
            let index = decision_index(&scan_result.artefacts.decisions);
            let todos = if include_todos {
                scan_result
                    .artefacts
                    .todos
                    .iter()
                    .filter(|todo| node_ids.contains(&todo.node))
                    .cloned()
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            };
            let research = if include_research {
                research_for_nodes(scan_result, &node_ids)
            } else {
                Vec::new()
            };
            let reviews = if include_reviews {
                scan_result
                    .artefacts
                    .reviews
                    .iter()
                    .filter(|review| node_ids.contains(&review.node))
                    .cloned()
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            };
            let error_count = scan_error_count(scan_result);
            // --json neighbourhood routes to query_api via uses_shared_json
            // (src/cli/mod.rs), so only the human branch is reachable here.
            let warnings = scan_error_warning(error_count, false);
            let active_changes = if include_changes {
                format!(
                    "\nActive changes:\n{}",
                    lines(&change_lines_for(
                        root,
                        &root.join(&parsed.changes_dir),
                        &node_ids
                    ))
                )
            } else {
                String::new()
            };
            format!(
                "Node: {}\nInbound:\n{}\nOutbound:\n{}\nContracts:\n{}\nAccepted decisions:\n{}\nTodos:\n{}\nResearch:\n{}\nReviews:\n{}{active_changes}{warnings}\n",
                response.node.id,
                lines(&response.inbound),
                lines(&response.outbound),
                lines(&response.node.contracts),
                lines(
                    &decisions
                        .iter()
                        .map(|decision| decision_line_with_index(decision, &index))
                        .collect::<Vec<_>>(),
                ),
                lines(&todos.iter().map(super::super::format::todo_line).collect::<Vec<_>>()),
                lines(&research.iter().map(super::super::format::research_line).collect::<Vec<_>>()),
                lines(&reviews.iter().map(super::super::format::review_line).collect::<Vec<_>>())
            )
        })
    })
}

pub(crate) fn render_files(parsed: &ParsedArgs, root: &Path) -> Result<String, Finding> {
    let node = node_arg(&parsed.command_args)?;
    let request = QueryRequest {
        tool: "files".to_owned(),
        node: Some(node.to_owned()),
        symbol: None,
        change: None,
        old_id: None,
        new_id: None,
        status: None,
        language: None,
        flags: BTreeSet::new(),
        mutating: false,
    };
    let data = crate::query_api::execute(
        root,
        &parsed.file,
        &root.join(&parsed.changes_dir),
        &request,
    )
    .map_err(super::query_error_to_finding)?
    .data;
    Ok(files_text(&data))
}

/// Renders the canonical `files_json` data as human text.
fn files_text(data: &Value) -> String {
    use std::fmt::Write;
    let node_id = data["node"].as_str().unwrap_or_default();
    let targets = data["targets"]
        .as_array()
        .map_or(&[][..], std::ops::Deref::deref);
    let has_multi_target = targets.len() > 1;
    let mut output = format!("Files for {node_id}:\n");
    if has_multi_target {
        for target in targets {
            let files: Vec<String> = target["files"]
                .as_array()
                .map_or(&[][..], std::ops::Deref::deref)
                .iter()
                .map(|value| value.as_str().unwrap_or_default().to_owned())
                .collect();
            writeln!(
                output,
                "  {} ({}): {}",
                target["path"].as_str().unwrap_or_default(),
                target["language"].as_str().unwrap_or_default(),
                files.join(", ")
            )
            .unwrap();
            writeln!(
                output,
                "    reconciler: {}",
                target["reconciler_id"].as_str().unwrap_or_default()
            )
            .unwrap();
            if let Some(hash) = target["hash"].as_str() {
                writeln!(output, "    hash: {hash}").unwrap();
            }
        }
    } else if let Some(target) = targets.first() {
        let files: Vec<String> = target["files"]
            .as_array()
            .map_or(&[][..], std::ops::Deref::deref)
            .iter()
            .map(|value| value.as_str().unwrap_or_default().to_owned())
            .collect();
        writeln!(
            output,
            "  {}: {}",
            target["path"].as_str().unwrap_or_default(),
            files.join(", ")
        )
        .unwrap();
        writeln!(
            output,
            "  language: {}",
            target["language"].as_str().unwrap_or_default()
        )
        .unwrap();
        writeln!(
            output,
            "  reconciler: {}",
            target["reconciler_id"].as_str().unwrap_or_default()
        )
        .unwrap();
        if let Some(hash) = target["hash"].as_str() {
            writeln!(output, "  hash: {hash}").unwrap();
        }
    } else {
        let files: Vec<String> = data["files"]
            .as_array()
            .map_or(&[][..], std::ops::Deref::deref)
            .iter()
            .map(|value| value.as_str().unwrap_or_default().to_owned())
            .collect();
        writeln!(output, "  {}", lines(&files)).unwrap();
    }
    output.push('\n');
    output
}

fn symbols_block(scan_result: &scanner::ScanResult, node: &str) -> String {
    match scan_result.graph.resolve(node) {
        Ok(node_record) => {
            let mut output = format!("\nSymbols for {}:\n", node_record.id);
            if node_record.symbols.is_empty() {
                output.push_str("  (none)\n");
                return output;
            }
            let mut by_file: std::collections::BTreeMap<
                &str,
                Vec<&crate::reconcile::SymbolRecord>,
            > = std::collections::BTreeMap::new();
            for symbol in &node_record.symbols {
                by_file
                    .entry(symbol.file.as_str())
                    .or_default()
                    .push(symbol);
            }
            for (file, symbols) in by_file {
                use std::fmt::Write;
                writeln!(output, "  {file}:").unwrap();
                for symbol in symbols {
                    writeln!(
                        output,
                        "    {}  {:?}  {}",
                        symbol.name, symbol.kind, symbol.line
                    )
                    .unwrap();
                }
            }
            output
        }
        Err(_) => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        artefacts::registry::{Decision, DecisionStatus},
        map::{Graph, NodeRecord, NodeState},
        scanner::{ScanResult, state::TargetHashes},
    };
    use serde_json::json;
    use std::collections::BTreeMap;

    fn node_record(id: &str) -> NodeRecord {
        NodeRecord {
            kind: crate::blueprint::NodeKind::Module,
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
            span: crate::blueprint::Span::point("test", 1, 1),
        }
    }

    fn decision(id: &str, status: DecisionStatus) -> Decision {
        Decision {
            id: id.to_owned(),
            path: format!("meta/decisions/{id}.md"),
            nodes: vec!["app".to_owned()],
            status,
            date: "2026-07-16".to_owned(),
            revisited: None,
            revisit_triggers: Vec::new(),
            informed_by: Vec::new(),
            supersedes: Vec::new(),
            refines: Vec::new(),
            refined_by: Vec::new(),
            superseded_by: Vec::new(),
            related: Vec::new(),
            orphaned: false,
            orphan_reason: None,
            gap: false,
            claims: None,
            body: String::new(),
            ratification: crate::artefacts::registry::RatificationTier::Binding,
            affects: Vec::new(),
            ratified_by_machine: false,
            receipts: Vec::new(),
        }
    }

    #[test]
    fn render_get_human_lists_accepted_decisions() {
        let mut nodes = BTreeMap::new();
        nodes.insert("app".to_owned(), node_record("app"));
        let scan = ScanResult {
            graph: Graph {
                nodes,
                names: BTreeMap::new(),
                outbound: BTreeMap::new(),
                inbound: BTreeMap::new(),
                findings: Vec::new(),
            },
            artefacts: crate::artefacts::registry::ArtefactSet {
                decisions: vec![
                    decision("dec.kept", DecisionStatus::Accepted),
                    decision("dec.pending", DecisionStatus::Proposed),
                ],
                ..Default::default()
            },
            contracts: crate::artefacts::contract::ContractSet::default(),
            interface_hash: String::new(),
            target_reports: Vec::new(),
            target_hashes: TargetHashes::default(),
            blueprint_snapshot: crate::scanner::state::BlueprintSnapshot::default(),
        };
        let parsed = ParsedArgs {
            json: false,
            strict: false,
            file: std::path::PathBuf::from("cairn.blueprint"),
            changes_dir: std::path::PathBuf::from("meta/changes"),
            command: "get".to_owned(),
            command_args: vec!["get".to_owned(), "app".to_owned()],
            verbose: false,
            brief: false,
        };
        let rendered = render_get(&parsed, Path::new("."), &scan).expect("get must resolve");
        assert!(
            rendered.contains("Accepted decisions:\n- dec.kept"),
            "human get must list accepted decision pointers: {rendered}"
        );
        assert!(
            !rendered.contains("dec.pending"),
            "non-accepted decisions must not appear: {rendered}"
        );
    }

    #[test]
    fn files_text_lists_node_files_when_no_target_report() {
        let data = json!({
            "node": "app",
            "files": ["src/lib.rs"],
            "targets": [],
        });
        let rendered = files_text(&data);
        assert!(rendered.contains("Files for app:"));
        assert!(rendered.contains("src/lib.rs"));
    }

    #[test]
    fn files_text_includes_target_claimed_files() {
        let data = json!({
            "node": "app",
            "files": [],
            "targets": [{
                "path": "src",
                "language": "rust",
                "reconciler_id": "rust-code",
                "files": ["src/lib.rs"],
                "hash": "abcd1234",
            }],
        });
        let rendered = files_text(&data);
        assert!(rendered.contains("rust-code"));
        assert!(rendered.contains("src/lib.rs"));
    }

    #[test]
    fn files_text_multi_target_lists_each_target() {
        let data = json!({
            "node": "app",
            "files": [],
            "targets": [
                {"path": "src", "language": "rust", "reconciler_id": "rust-code", "files": ["src/lib.rs"]},
                {"path": "tests", "language": "rust", "reconciler_id": "rust-code", "files": ["tests/a.rs"]},
            ],
        });
        let rendered = files_text(&data);
        assert!(rendered.contains("src (rust): src/lib.rs"));
        assert!(rendered.contains("tests (rust): tests/a.rs"));
    }
}
