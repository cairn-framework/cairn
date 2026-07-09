//! Node-level query renderers.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::format::{lines, node_arg, render_node, string_array_json};
use super::super::*;
use super::{scan_error_count, scan_error_warning};
use crate::query_api::{neighbourhood_ids, research_for_nodes};

pub(crate) fn render_get(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    node_arg(&parsed.command_args).and_then(|node| {
        match query::get(&scan_result.graph, node) {
            Ok(response) => Ok(render_node(&response.node, parsed.json)),
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
        let include_orphans = parsed
            .command_args
            .iter()
            .any(|arg| arg == "--include-orphans");
        let response =
            query::neighbourhood_with_options(&scan_result.graph, node, include_orphans)?;
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
                lines(&decisions.iter().map(super::super::format::decision_line).collect::<Vec<_>>()),
                lines(&todos.iter().map(super::super::format::todo_line).collect::<Vec<_>>()),
                lines(&research.iter().map(super::super::format::research_line).collect::<Vec<_>>()),
                lines(&reviews.iter().map(super::super::format::review_line).collect::<Vec<_>>())
            )
        })
    })
}

pub(crate) fn render_files(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    node_arg(&parsed.command_args).and_then(|node| {
        let node_record = scan_result.graph.resolve(node)?;
        let target_reports_for_node: Vec<_> = scan_result
            .target_reports
            .iter()
            .filter(|r| r.target_id.node_id == node_record.id)
            .collect();
        let has_multi_target = target_reports_for_node.len() > 1;
        if parsed.json {
            let targets_json = if target_reports_for_node.is_empty() {
                "[]".to_string()
            } else {
                let items: Vec<String> = target_reports_for_node
                    .iter()
                    .map(|r| {
                        let hash_field = if let Some(hash) = &r.hash {
                            format!(",\"hash\":\"{}\"", esc(hash))
                        } else {
                            String::new()
                        };
                        format!(
                            "{{\"path\":\"{}\",\"language\":\"{}\",\"reconciler_id\":\"{}\",\"files\":{}{}}}",
                            esc(&r.target_id.path.to_string_lossy()),
                            r.language.as_str(),
                            r.reconciler_id.0,
                            string_array_json(&r.claimed_files),
                            hash_field
                        )
                    })
                    .collect();
                format!("[{}]", items.join(","))
            };
            if has_multi_target {
                Ok(format!(
                    "{{\"node\":\"{}\",\"targets\":{}}}\n",
                    esc(&node_record.id),
                    targets_json
                ))
            } else {
                Ok(format!(
                    "{{\"node\":\"{}\",\"files\":{},\"targets\":{}}}\n",
                    esc(&node_record.id),
                    string_array_json(&node_record.files),
                    targets_json
                ))
            }
        } else {
            let mut output = format!("Files for {}:\n", node_record.id);
            if has_multi_target {
                for r in &target_reports_for_node {
                    use std::fmt::Write;
                    writeln!(
                        output,
                        "  {} ({}): {}",
                        r.target_id.path.display(),
                        r.language.as_str(),
                        r.claimed_files.join(", ")
                    ).unwrap();
                    writeln!(output, "    reconciler: {}", r.reconciler_id.0).unwrap();
                    if let Some(hash) = &r.hash {
                        writeln!(output, "    hash: {hash}").unwrap();
                    }
                }
            } else if let Some(r) = target_reports_for_node.first() {
                use std::fmt::Write;
                writeln!(
                    output,
                    "  {}: {}",
                    r.target_id.path.display(),
                    r.claimed_files.join(", ")
                ).unwrap();
                writeln!(output, "  language: {}", r.language.as_str()).unwrap();
                writeln!(output, "  reconciler: {}", r.reconciler_id.0).unwrap();
                if let Some(hash) = &r.hash {
                    writeln!(output, "  hash: {hash}").unwrap();
                }
            } else {
                use std::fmt::Write;
                writeln!(output, "  {}", lines(&node_record.files)).unwrap();
            }
            output.push('\n');
            Ok(output)
        }
    })
}

pub(crate) fn render_symbols(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    node_arg(&parsed.command_args).and_then(|node| {
        let node_record = scan_result.graph.resolve(node)?;
        let mut output = format!("Symbols for {}:\n", node_record.id);
        if node_record.symbols.is_empty() {
            output.push_str("  (none)\n");
            return Ok(output);
        }
        let mut by_file: std::collections::BTreeMap<&str, Vec<&crate::reconcile::SymbolRecord>> =
            std::collections::BTreeMap::new();
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
        Ok(output)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        blueprint::{NodeKind, Span},
        map::{Graph, NodeRecord, NodeState},
        reconcile::{ReconcilerId, target::Language},
        scanner::{ScanResult, TargetReport, state::TargetHashes},
    };
    use std::{collections::BTreeMap, sync::Arc};

    fn node_record(id: &str, files: Vec<String>) -> NodeRecord {
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
            files,
            span: Span::point("test", 1, 1),
        }
    }

    fn parsed(files: &str, json: bool) -> ParsedArgs {
        ParsedArgs {
            json,
            strict: false,
            file: std::path::PathBuf::from("cairn.blueprint"),
            changes_dir: std::path::PathBuf::from("meta/changes"),
            command: "files".to_owned(),
            command_args: vec!["files".to_owned(), files.to_owned()],
        }
    }

    fn scan_with_files_and_reports(files: Vec<String>, reports: Vec<TargetReport>) -> ScanResult {
        let mut nodes = BTreeMap::new();
        nodes.insert("app".to_owned(), node_record("app", files));
        ScanResult {
            graph: Graph {
                nodes,
                names: BTreeMap::new(),
                outbound: BTreeMap::new(),
                inbound: BTreeMap::new(),
                findings: Vec::new(),
            },
            artefacts: crate::artefacts::registry::ArtefactSet::default(),
            contracts: crate::artefacts::contract::ContractSet::default(),
            interface_hash: String::new(),
            target_reports: reports,
            target_hashes: TargetHashes::default(),
            blueprint_snapshot: crate::scanner::state::BlueprintSnapshot::default(),
        }
    }

    fn report(path: &str, claimed_files: &[&str]) -> TargetReport {
        TargetReport {
            target_id: crate::reconcile::target::TargetId::new(
                "app".to_owned(),
                std::path::PathBuf::from(path),
            ),
            language: Language::Rust,
            reconciler_id: ReconcilerId("rust-code".to_owned()),
            claimed_files: claimed_files
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
            symbol_records: Arc::new(Vec::new()),
            symbols: Arc::new(Vec::new()),
            hash: Some("abcd1234".to_owned()),
        }
    }

    #[test]
    fn render_files_human_lists_node_files_when_no_target_report() {
        let scan = scan_with_files_and_reports(vec!["src/lib.rs".to_owned()], Vec::new());
        let rendered = render_files(&parsed("app", false), &scan).unwrap();
        assert!(rendered.contains("Files for app:"));
        assert!(rendered.contains("src/lib.rs"));
    }

    #[test]
    fn render_files_human_includes_target_claimed_files() {
        let scan = scan_with_files_and_reports(Vec::new(), vec![report("src", &["src/lib.rs"])]);
        let rendered = render_files(&parsed("app", false), &scan).unwrap();
        assert!(rendered.contains("rust-code"));
        assert!(rendered.contains("src/lib.rs"));
    }

    #[test]
    fn render_files_json_includes_files_and_targets() {
        let scan = scan_with_files_and_reports(
            vec!["src/lib.rs".to_owned()],
            vec![report("src", &["src/lib.rs"])],
        );
        let rendered = render_files(&parsed("app", true), &scan).unwrap();
        assert!(rendered.contains("\"node\":\"app\""));
        assert!(rendered.contains("\"files\""));
        assert!(rendered.contains("\"targets\""));
    }

    #[test]
    fn render_files_multi_target_uses_targets_wrapper() {
        let scan = scan_with_files_and_reports(
            Vec::new(),
            vec![
                report("src", &["src/lib.rs"]),
                report("tests", &["tests/a.rs"]),
            ],
        );
        let rendered = render_files(&parsed("app", true), &scan).unwrap();
        assert!(rendered.starts_with("{\"node\":\"app\",\"targets\":"));
    }
}
