//! Artefact query renderers (todos, decisions, research, sources, rationale).
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::format::{
    decision_line, decisions_json, flag_value, lines, node_arg, parse_decision_status_filter,
    parse_todo_status_filter, research_for_nodes, research_json, research_line, source_line,
    sources_for_nodes, sources_json, todo_line, todos_json,
};
use super::super::*;

pub(crate) fn render_todos(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    let status = flag_value(&parsed.command_args, "--status").and_then(parse_todo_status_filter);
    node_arg(&parsed.command_args).and_then(|node| {
        let node = scan_result.graph.resolve(node)?;
        let todos = scan_result
            .artefacts
            .todos
            .iter()
            .filter(|todo| {
                todo.node == node.id && status.is_none_or(|filter| todo.status == filter)
            })
            .cloned()
            .collect::<Vec<_>>();
        Ok(if parsed.json {
            format!(
                "{{\"node\":\"{}\",\"todos\":{}}}\n",
                esc(&node.id),
                todos_json(&todos)
            )
        } else {
            format!(
                "Todos for {}:\n{}\n",
                node.id,
                lines(&todos.iter().map(todo_line).collect::<Vec<_>>())
            )
        })
    })
}

pub(crate) fn render_decisions(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    let status =
        flag_value(&parsed.command_args, "--status").and_then(parse_decision_status_filter);
    if let Some(query) = flag_value(&parsed.command_args, "--grep") {
        return Ok(render_decisions_grep(parsed, scan_result, query));
    }
    node_arg(&parsed.command_args).and_then(|node| {
        let node = scan_result.graph.resolve(node)?;
        let decisions = scan_result
            .artefacts
            .decisions
            .iter()
            .filter(|decision| {
                decision.nodes.contains(&node.id)
                    && status.is_none_or(|filter| decision.status == filter)
            })
            .cloned()
            .collect::<Vec<_>>();
        Ok(if parsed.json {
            format!(
                "{{\"node\":\"{}\",\"decisions\":{}}}\n",
                esc(&node.id),
                decisions_json(&decisions)
            )
        } else {
            format!(
                "Decisions for {}:\n{}\n",
                node.id,
                lines(&decisions.iter().map(decision_line).collect::<Vec<_>>())
            )
        })
    })
}

fn render_decisions_grep(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
    query: &str,
) -> String {
    let status =
        flag_value(&parsed.command_args, "--status").and_then(parse_decision_status_filter);
    let needle = query.to_lowercase();
    let matches = scan_result
        .artefacts
        .decisions
        .iter()
        .filter(|decision| {
            status.is_none_or(|filter| decision.status == filter)
                && (decision.id.to_lowercase().contains(&needle)
                    || decision.body.to_lowercase().contains(&needle)
                    || decision
                        .nodes
                        .iter()
                        .any(|node| node.to_lowercase().contains(&needle)))
        })
        .cloned()
        .collect::<Vec<_>>();
    if parsed.json {
        format!(
            "{{\"query\":\"{}\",\"decisions\":{}}}\n",
            esc(query),
            decisions_json(&matches)
        )
    } else {
        format!(
            "Decisions matching \"{}\":\n{}\n",
            query,
            lines(&matches.iter().map(decision_line).collect::<Vec<_>>())
        )
    }
}

pub(crate) fn render_research(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    node_arg(&parsed.command_args).and_then(|node| {
        let node = scan_result.graph.resolve(node)?;
        let research = research_for_nodes(scan_result, &BTreeSet::from([node.id.clone()]));
        Ok(if parsed.json {
            format!(
                "{{\"node\":\"{}\",\"research\":{}}}\n",
                esc(&node.id),
                research_json(&research)
            )
        } else {
            format!(
                "Research for {}:\n{}\n",
                node.id,
                lines(&research.iter().map(research_line).collect::<Vec<_>>())
            )
        })
    })
}

pub(crate) fn render_sources(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    node_arg(&parsed.command_args).and_then(|node| {
        let node = scan_result.graph.resolve(node)?;
        let sources = sources_for_nodes(scan_result, &BTreeSet::from([node.id.clone()]));
        Ok(if parsed.json {
            format!(
                "{{\"node\":\"{}\",\"sources\":{}}}\n",
                esc(&node.id),
                sources_json(&sources)
            )
        } else {
            format!(
                "Sources for {}:\n{}\n",
                node.id,
                lines(&sources.iter().map(source_line).collect::<Vec<_>>())
            )
        })
    })
}

pub(crate) fn render_rationale(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    node_arg(&parsed.command_args).and_then(|node| {
        let node = scan_result.graph.resolve(node)?;
        let node_ids = super::super::format::neighbourhood_ids(&scan_result.graph, &node.id);
        let decisions = scan_result
            .artefacts
            .decisions
            .iter()
            .filter(|decision| {
                decision.status == DecisionStatus::Accepted
                    && decision.nodes.iter().any(|node| node_ids.contains(node))
            })
            .cloned()
            .collect::<Vec<_>>();
        let research_ids = decisions
            .iter()
            .flat_map(|decision| decision.informed_by.iter())
            .cloned()
            .collect::<BTreeSet<_>>();
        let source_ids = decisions
            .iter()
            .flat_map(|decision| decision.informed_by.iter())
            .cloned()
            .chain(
                scan_result
                    .artefacts
                    .research
                    .iter()
                    .filter(|research| research_ids.contains(&research.id))
                    .flat_map(|research| research.sources.iter().cloned()),
            )
            .collect::<BTreeSet<_>>();
        let research = scan_result
            .artefacts
            .research
            .iter()
            .filter(|research| research_ids.contains(&research.id))
            .cloned()
            .collect::<Vec<_>>();
        let sources = scan_result
            .artefacts
            .sources
            .iter()
            .filter(|source| source_ids.contains(&source.id))
            .cloned()
            .collect::<Vec<_>>();
        Ok(if parsed.json {
            format!(
                "{{\"node\":\"{}\",\"decisions\":{},\"research\":{},\"sources\":{}}}\n",
                esc(&node.id),
                decisions_json(&decisions),
                research_json(&research),
                sources_json(&sources)
            )
        } else {
            format!(
                "Rationale for {}:\nDecisions:\n{}\nResearch:\n{}\nSources:\n{}\n",
                node.id,
                lines(&decisions.iter().map(decision_line).collect::<Vec<_>>()),
                lines(&research.iter().map(research_line).collect::<Vec<_>>()),
                lines(&sources.iter().map(source_line).collect::<Vec<_>>())
            )
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        artefacts::registry::{Decision, DecisionStatus, Todo, TodoStatus},
        map::{Graph, NodeRecord, NodeState},
        scanner::{ScanResult, state::TargetHashes},
    };
    use std::collections::BTreeMap;

    fn parsed(node: &str, json: bool) -> ParsedArgs {
        ParsedArgs {
            json,
            strict: false,
            file: std::path::PathBuf::from("cairn.blueprint"),
            changes_dir: std::path::PathBuf::from("meta/changes"),
            command: "todos".to_owned(),
            command_args: vec!["todos".to_owned(), node.to_owned()],
        }
    }

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

    fn todo(node: &str, status: TodoStatus) -> Todo {
        Todo {
            path: "./todo.md".to_owned(),
            node: node.to_owned(),
            status,
            created: "2026-01-01".to_owned(),
            satisfies: None,
            body: String::new(),
        }
    }

    fn decision(id: &str, nodes: &[&str], body: &str, status: DecisionStatus) -> Decision {
        Decision {
            id: id.to_owned(),
            path: format!("meta/decisions/{id}.md"),
            nodes: nodes.iter().map(|node| (*node).to_owned()).collect(),
            status,
            date: "2026-01-01".to_owned(),
            revisited: None,
            revisit_triggers: Vec::new(),
            informed_by: Vec::new(),
            supersedes: Vec::new(),
            refines: Vec::new(),
            related: Vec::new(),
            orphaned: false,
            orphan_reason: None,
            claims: None,
            body: body.to_owned(),
        }
    }

    fn scan_with_decisions(decisions: Vec<Decision>) -> ScanResult {
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
                decisions,
                ..Default::default()
            },
            contracts: crate::artefacts::contract::ContractSet::default(),
            interface_hash: String::new(),
            target_reports: Vec::new(),
            target_hashes: TargetHashes::default(),
            blueprint_snapshot: crate::scanner::state::BlueprintSnapshot::default(),
        }
    }

    fn decisions_parsed(args: &[&str], json: bool) -> ParsedArgs {
        ParsedArgs {
            json,
            strict: false,
            file: std::path::PathBuf::from("cairn.blueprint"),
            changes_dir: std::path::PathBuf::from("meta/changes"),
            command: "decisions".to_owned(),
            command_args: args.iter().map(|arg| (*arg).to_owned()).collect(),
        }
    }

    #[test]
    fn render_decisions_grep_matches_body_keyword() {
        let scan = scan_with_decisions(vec![
            decision(
                "dec.beads-loader",
                &["app"],
                "Read beads jsonl export",
                DecisionStatus::Accepted,
            ),
            decision(
                "dec.unrelated",
                &["app"],
                "Something else entirely",
                DecisionStatus::Accepted,
            ),
        ]);
        let p = decisions_parsed(&["decisions", "--grep", "beads"], false);
        let rendered = render_decisions(&p, &scan).unwrap();
        assert!(rendered.contains("Decisions matching \"beads\":"));
        assert!(rendered.contains("dec.beads-loader"));
        assert!(!rendered.contains("dec.unrelated"));
    }

    #[test]
    fn render_decisions_grep_searches_without_node_arg() {
        let scan = scan_with_decisions(vec![decision(
            "dec.feedback",
            &["cairn.kernel.cli"],
            "feedback loop records friction",
            DecisionStatus::Accepted,
        )]);
        let p = decisions_parsed(&["decisions", "--grep", "friction"], false);
        let rendered = render_decisions(&p, &scan).unwrap();
        assert!(rendered.contains("dec.feedback"));
        assert!(!rendered.contains("Error"));
        assert!(!rendered.contains("NOT_FOUND"));
    }

    #[test]
    fn render_decisions_grep_json_mode() {
        let scan = scan_with_decisions(vec![decision(
            "dec.beads-loader",
            &["app"],
            "beads",
            DecisionStatus::Accepted,
        )]);
        let p = decisions_parsed(&["decisions", "--grep", "beads"], true);
        let rendered = render_decisions(&p, &scan).unwrap();
        assert!(rendered.contains("\"query\":\"beads\""));
        assert!(rendered.contains("\"decisions\""));
        assert!(rendered.contains("dec.beads-loader"));
    }

    #[test]
    fn render_decisions_grep_respects_status_filter() {
        let scan = scan_with_decisions(vec![
            decision("dec.live", &["app"], "beads", DecisionStatus::Accepted),
            decision("dec.old", &["app"], "beads", DecisionStatus::Superseded),
        ]);
        let p = decisions_parsed(
            &["decisions", "--grep", "beads", "--status", "accepted"],
            false,
        );
        let rendered = render_decisions(&p, &scan).unwrap();
        assert!(rendered.contains("dec.live"));
        assert!(!rendered.contains("dec.old"));
    }

    #[test]
    fn render_todos_human_lists_matching_todos() {
        let scan = scan_with_todos(vec![todo("app", TodoStatus::Open)]);
        let rendered = render_todos(&parsed("app", false), &scan).unwrap();
        assert!(rendered.contains("Todos for app:"));
        assert!(rendered.contains("[open]"));
    }

    #[test]
    fn render_todos_filters_by_status() {
        let scan = scan_with_todos(vec![
            todo("app", TodoStatus::Open),
            todo("app", TodoStatus::Done),
        ]);
        let mut p = parsed("app", false);
        p.command_args = vec![
            "todos".to_owned(),
            "app".to_owned(),
            "--status".to_owned(),
            "done".to_owned(),
        ];
        let rendered = render_todos(&p, &scan).unwrap();
        assert!(rendered.contains("[done]"));
        assert!(!rendered.contains("[open]"));
    }

    #[test]
    fn render_todos_json_mode() {
        let scan = scan_with_todos(vec![todo("app", TodoStatus::Open)]);
        let rendered = render_todos(&parsed("app", true), &scan).unwrap();
        assert!(rendered.contains("\"node\":\"app\""));
        assert!(rendered.contains("\"todos\""));
    }

    #[test]
    fn render_todos_unknown_node_returns_err() {
        let scan = scan_with_todos(Vec::new());
        let result = render_todos(&parsed("missing", false), &scan);
        assert!(result.is_err());
    }
}
