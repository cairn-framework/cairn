//! Remediation and next-action query renderers.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use crate::query_api;

pub(crate) fn render_remediate(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
) -> String {
    let changes_dir = root.join(&parsed.changes_dir);
    let remediate = query_api::remediate_json(root, &changes_dir, scan_result);
    if parsed.json {
        format!("{remediate}\n")
    } else {
        format_remediate_human(&remediate)
    }
}

fn format_remediate_human(remediate: &serde_json::Value) -> String {
    let empty: Vec<serde_json::Value> = Vec::new();
    let actions = remediate
        .get("actions")
        .and_then(serde_json::Value::as_array)
        .unwrap_or(&empty);
    if actions.is_empty() {
        return "No actions required.\n".to_owned();
    }
    let mut lines = Vec::new();
    lines.push(format!("Actions ({}):", actions.len()));
    for action in actions {
        let priority = action
            .get("priority")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(99);
        let name = action
            .get("action")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("unknown");
        let command = action
            .get("command")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");
        let description = action
            .get("description")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");
        let nodes = action
            .get("nodes")
            .and_then(serde_json::Value::as_array)
            .map(|arr: &Vec<serde_json::Value>| {
                arr.iter()
                    .filter_map(serde_json::Value::as_str)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        lines.push(format!("  [{priority}] {name}"));
        if !description.is_empty() {
            lines.push(format!("      {description}"));
        }
        if !command.is_empty() {
            lines.push(format!("      run: {command}"));
        }
        if !nodes.is_empty() {
            lines.push(format!("      nodes: {}", nodes.join(", ")));
        }
    }
    lines.join("\n") + "\n"
}

pub(crate) fn render_next(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
) -> String {
    let changes_dir = root.join(&parsed.changes_dir);
    let health = query_api::health_json(root, &changes_dir, scan_result);
    let clean = health
        .get("clean")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    if parsed.json {
        if clean {
            let items = crate::state::backlog::read(root);
            let ready = crate::state::backlog::ready(&items);
            return ready.first().map_or_else(
                || "{\"next\":null,\"clean\":true,\"ready\":0}\n".to_owned(),
                |top| {
                    format!(
                        "{{\"next\":{{\"bead\":\"{}\",\"title\":\"{}\",\"priority\":{},\"source\":\"beads-backlog\"}},\"clean\":true,\"ready\":{}}}\n",
                        esc(&top.id),
                        esc(&top.title),
                        top.priority,
                        ready.len()
                    )
                },
            );
        }
        let remediate = query_api::remediate_json(root, &changes_dir, scan_result);
        let empty: Vec<serde_json::Value> = Vec::new();
        let actions = remediate
            .get("actions")
            .and_then(serde_json::Value::as_array)
            .unwrap_or(&empty);
        let first = actions.first().unwrap_or(&serde_json::Value::Null);
        return format!("{{\"next\":{first},\"clean\":false}}\n");
    }
    if clean {
        let items = crate::state::backlog::read(root);
        let ready = crate::state::backlog::ready(&items);
        return ready.first().map_or_else(
            || "Next: nothing to do. Project is clean.\n".to_owned(),
            |top| {
                let mut out = vec![
                    format!("Next: {} [P{}] {}", top.id, top.priority, top.title),
                    format!("  source: beads backlog ({} ready)", ready.len()),
                    format!("  run: bd show {}", top.id),
                ];
                if let Some(node) = top.linked_node() {
                    out.push(format!("  node: {node}"));
                }
                out.join("\n") + "\n"
            },
        );
    }
    let remediate = query_api::remediate_json(root, &changes_dir, scan_result);
    let empty: Vec<serde_json::Value> = Vec::new();
    let actions = remediate
        .get("actions")
        .and_then(serde_json::Value::as_array)
        .unwrap_or(&empty);
    actions.first().map_or_else(
        || "Next: nothing to do.\n".to_owned(),
        |first| {
            let name = first
                .get("action")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown");
            let command = first
                .get("command")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("");
            let description = first
                .get("description")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("");
            let nodes = first
                .get("nodes")
                .and_then(serde_json::Value::as_array)
                .map(|arr: &Vec<serde_json::Value>| {
                    arr.iter()
                        .filter_map(serde_json::Value::as_str)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let mut lines = Vec::new();
            lines.push(format!("Next action: {name}"));
            if !description.is_empty() {
                lines.push(format!("  {description}"));
            }
            if !command.is_empty() {
                lines.push(format!("  run: {command}"));
            }
            if !nodes.is_empty() {
                lines.push(format!("  nodes: {}", nodes.join(", ")));
            }
            lines.join("\n") + "\n"
        },
    )
}

/// Extracts a one-line summary from a decision body (first markdown heading or
/// first non-empty line), trimmed to a readable length.
fn decision_summary(body: &str) -> String {
    let line = body
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .unwrap_or("");
    let cleaned = line.trim_start_matches('#').trim();
    if cleaned.chars().count() > 100 {
        let truncated: String = cleaned.chars().take(97).collect();
        format!("{truncated}...")
    } else {
        cleaned.to_owned()
    }
}

/// Resolves the bead a brief targets: an explicit id argument, else the top
/// ready item from the backlog.
fn resolve_brief_bead(
    parsed: &ParsedArgs,
    items: &[crate::state::backlog::BacklogItem],
) -> Option<crate::state::backlog::BacklogItem> {
    match parsed.command_args.get(1) {
        Some(id) => items.iter().find(|item| &item.id == id).cloned(),
        None => crate::state::backlog::ready(items)
            .first()
            .map(|item| (*item).clone()),
    }
}

/// Renders `cairn brief [<id>]`: the next (or named) ready unit fused with its
/// binding decisions, contract, acceptance criteria, and the gates that judge
/// it, so a fresh agent can pick up work safely from one command.
pub(crate) fn render_brief(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
) -> String {
    let items = crate::state::backlog::read(root);
    let Some(bead) = resolve_brief_bead(parsed, &items) else {
        let message = match parsed.command_args.get(1) {
            Some(id) => crate::cli::copy::lookup("brief.not-found").replace("{id}", id),
            None if items.is_empty() => crate::cli::copy::lookup("brief.empty").to_owned(),
            None => crate::cli::copy::lookup("brief.none-ready").to_owned(),
        };
        if parsed.json {
            return format!("{{\"brief\":null,\"message\":\"{}\"}}\n", esc(&message));
        }
        return format!("{message}\n");
    };

    let ready_now = crate::state::backlog::ready(&items)
        .iter()
        .any(|item| item.id == bead.id);
    let node = bead
        .linked_node()
        .and_then(|id| scan_result.graph.resolve(id).ok());
    let decisions: Vec<&Decision> = node.map_or_else(Vec::new, |node| {
        scan_result
            .artefacts
            .decisions
            .iter()
            .filter(|decision| {
                decision.status == DecisionStatus::Accepted && decision.nodes.contains(&node.id)
            })
            .collect()
    });
    let contract = node.and_then(|node| {
        node.contracts
            .iter()
            .find_map(|path| scan_result.contracts.contracts.get(path))
            .filter(|contract| contract.node == node.id)
            .map(|contract| contract.body.trim().to_owned())
    });
    let gates = crate::cli::copy::lookup("brief.gates");
    let staleness = crate::cli::copy::lookup("brief.staleness-note");

    let data = BriefData {
        bead: &bead,
        ready_now,
        node,
        decisions: &decisions,
        contract: contract.as_deref(),
        gates,
        staleness,
    };
    if parsed.json {
        format_brief_json(&data)
    } else {
        format_brief_human(&data)
    }
}

/// Assembled inputs for rendering a brief in either format.
struct BriefData<'a> {
    bead: &'a crate::state::backlog::BacklogItem,
    ready_now: bool,
    node: Option<&'a crate::map::NodeRecord>,
    decisions: &'a [&'a Decision],
    contract: Option<&'a str>,
    gates: &'a str,
    staleness: &'a str,
}

/// Renders a [`BriefData`] as the machine-readable `{"brief": {...}}` payload,
/// carrying the gates and staleness note so a JSON consumer sees the same
/// constraints as the human surface.
fn format_brief_json(data: &BriefData) -> String {
    let decisions_json = serde_json::Value::Array(
        data.decisions
            .iter()
            .map(|decision| {
                serde_json::json!({
                    "id": decision.id,
                    "path": decision.path,
                    "summary": decision_summary(&decision.body),
                })
            })
            .collect(),
    );
    let payload = serde_json::json!({
        "brief": {
            "bead": data.bead.id,
            "title": data.bead.title,
            "priority": data.bead.priority,
            "ready": data.ready_now,
            "node": data.node.map(|node| node.id.clone()),
            "task": data.bead.description,
            "decisions": decisions_json,
            "contract": data.contract,
            "gates": data.gates,
            "staleness": data.staleness,
        }
    });
    format!("{payload}\n")
}

/// Renders a [`BriefData`] as the human-readable brief: header, readiness
/// warning, linked node, task body, binding decisions, contract, and gates.
fn format_brief_human(data: &BriefData) -> String {
    let bead = data.bead;
    let mut out = vec![
        format!("Brief: {} [P{}] {}", bead.id, bead.priority, bead.title),
        format!("  run: bd show {}", bead.id),
    ];
    if !data.ready_now {
        out.push(format!("  {}", crate::cli::copy::lookup("brief.not-ready")));
    }
    match data.node {
        Some(node) => out.push(format!("  node: {}", node.id)),
        None => out.push(format!(
            "  node: (unlinked) {}",
            crate::cli::copy::lookup("brief.unlinked-hint").replace("{id}", &bead.id)
        )),
    }
    out.push(String::new());
    out.push("Task:".to_owned());
    out.push(bead.description.trim().to_owned());
    out.push(String::new());
    out.push("Binding decisions (work within these or write a superseding one):".to_owned());
    if data.decisions.is_empty() {
        out.push("  none linked".to_owned());
    } else {
        for decision in data.decisions {
            out.push(format!("- {} [{}]", decision.id, decision.path));
            let summary = decision_summary(&decision.body);
            if !summary.is_empty() {
                out.push(format!("    {summary}"));
            }
        }
    }
    out.push(String::new());
    out.push("Contract:".to_owned());
    out.push(match data.contract {
        Some(body) if !body.is_empty() => body.to_owned(),
        _ => "  none".to_owned(),
    });
    out.push(String::new());
    out.push(data.gates.to_owned());
    out.push(String::new());
    out.push(data.staleness.to_owned());
    out.join("\n") + "\n"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_remediate_human_empty_actions() {
        let json = serde_json::json!({"actions": []});
        assert_eq!(format_remediate_human(&json), "No actions required.\n");
    }

    #[test]
    fn format_remediate_human_lists_actions() {
        let json = serde_json::json!({
            "actions": [{
                "priority": 1,
                "action": "fix-lint",
                "description": "Run cargo fmt",
                "command": "cargo fmt",
                "nodes": ["app"]
            }]
        });
        let rendered = format_remediate_human(&json);
        assert!(rendered.contains("Actions (1):"));
        assert!(rendered.contains("[1] fix-lint"));
        assert!(rendered.contains("Run cargo fmt"));
        assert!(rendered.contains("run: cargo fmt"));
        assert!(rendered.contains("nodes: app"));
    }

    #[test]
    fn format_remediate_human_omits_optional_fields() {
        let json = serde_json::json!({
            "actions": [{"action": "noop"}]
        });
        let rendered = format_remediate_human(&json);
        assert!(rendered.contains("[99] noop"));
        assert!(!rendered.contains("run:"));
        assert!(!rendered.contains("nodes:"));
    }

    use crate::{
        artefacts::contract::{Contract, ContractSet},
        artefacts::registry::{Decision, DecisionStatus},
        map::{Graph, NodeRecord, NodeState},
        scanner::{ScanResult, state::TargetHashes},
    };
    use std::collections::BTreeMap;

    fn tmpdir(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("cairn-brief-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn write_export(dir: &Path, lines: &[&str]) {
        let beads = dir.join(".beads");
        std::fs::create_dir_all(&beads).unwrap();
        std::fs::write(beads.join("issues.jsonl"), lines.join("\n")).unwrap();
    }

    fn brief_parsed(args: &[&str], json: bool) -> ParsedArgs {
        ParsedArgs {
            json,
            strict: false,
            file: std::path::PathBuf::from("cairn.blueprint"),
            changes_dir: std::path::PathBuf::from("meta/changes"),
            command: "brief".to_owned(),
            command_args: args.iter().map(|arg| (*arg).to_owned()).collect(),
        }
    }

    fn node_record(id: &str, contracts: Vec<String>) -> NodeRecord {
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
            contracts,
            state: NodeState::Synced,
            files: Vec::new(),
            span: crate::blueprint::Span::point("test", 1, 1),
        }
    }

    fn scan_with(
        nodes: Vec<NodeRecord>,
        decisions: Vec<Decision>,
        contracts: ContractSet,
    ) -> ScanResult {
        let mut node_map = BTreeMap::new();
        for node in nodes {
            node_map.insert(node.id.clone(), node);
        }
        ScanResult {
            graph: Graph {
                nodes: node_map,
                names: BTreeMap::new(),
                outbound: BTreeMap::new(),
                inbound: BTreeMap::new(),
                findings: Vec::new(),
            },
            artefacts: crate::artefacts::registry::ArtefactSet {
                decisions,
                ..Default::default()
            },
            contracts,
            interface_hash: String::new(),
            target_reports: Vec::new(),
            target_hashes: TargetHashes::default(),
            blueprint_snapshot: crate::scanner::state::BlueprintSnapshot::default(),
        }
    }

    fn decision(id: &str, nodes: &[&str], body: &str) -> Decision {
        decision_with_status(id, nodes, body, DecisionStatus::Accepted)
    }

    fn decision_with_status(
        id: &str,
        nodes: &[&str],
        body: &str,
        status: DecisionStatus,
    ) -> Decision {
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

    #[test]
    fn test_decision_summary_strips_heading_marker() {
        assert_eq!(decision_summary("# Title here\nbody"), "Title here");
        assert_eq!(decision_summary("\n\n  plain line\nmore"), "plain line");
        assert_eq!(decision_summary(""), "");
    }

    #[test]
    fn test_decision_summary_truncates_long_line() {
        let long = "x".repeat(200);
        let summary = decision_summary(&long);
        assert!(summary.ends_with("..."));
        assert_eq!(summary.chars().count(), 100);
    }

    #[test]
    fn test_brief_named_bead_includes_gates() {
        let dir = tmpdir("named");
        write_export(
            &dir,
            &[
                r#"{"id":"cairn-a","title":"Alpha","status":"open","priority":1}"#,
                r#"{"id":"cairn-b","title":"Beta","status":"open","priority":2}"#,
            ],
        );
        let scan = scan_with(Vec::new(), Vec::new(), ContractSet::default());
        let out = render_brief(&brief_parsed(&["brief", "cairn-b"], false), &dir, &scan);
        assert!(out.contains("Brief: cairn-b [P2] Beta"));
        assert!(out.contains("Gates that will judge"));
        assert!(out.contains("cairn hook all"));
        assert!(out.contains("bd ready` is authoritative"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_brief_no_arg_picks_top_ready() {
        let dir = tmpdir("topready");
        write_export(
            &dir,
            &[
                r#"{"id":"cairn-low","title":"Low","status":"open","priority":3}"#,
                r#"{"id":"cairn-hi","title":"High","status":"open","priority":1}"#,
                r#"{"id":"cairn-done","title":"Done","status":"closed","priority":0}"#,
            ],
        );
        let scan = scan_with(Vec::new(), Vec::new(), ContractSet::default());
        let out = render_brief(&brief_parsed(&["brief"], false), &dir, &scan);
        assert!(out.contains("Brief: cairn-hi [P1] High"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_brief_surfaces_linked_decision_and_contract() {
        let dir = tmpdir("linked");
        write_export(
            &dir,
            &[
                r#"{"id":"cairn-x","title":"X","status":"open","priority":1,"labels":["cairn-node:cairn.kernel.cli"]}"#,
            ],
        );
        let node = node_record("cairn.kernel.cli", vec!["meta/contracts/cli.md".to_owned()]);
        let mut contracts = ContractSet::default();
        contracts.contracts.insert(
            "meta/contracts/cli.md".to_owned(),
            Contract {
                path: "meta/contracts/cli.md".to_owned(),
                declared_by: "cairn.kernel.cli".to_owned(),
                node: "cairn.kernel.cli".to_owned(),
                body: "Public interface: parse_args".to_owned(),
            },
        );
        let decisions = vec![decision(
            "dec.kernel-tooling",
            &["cairn.kernel.cli"],
            "# CLI is the agent surface",
        )];
        let scan = scan_with(vec![node], decisions, contracts);
        let out = render_brief(&brief_parsed(&["brief", "cairn-x"], false), &dir, &scan);
        assert!(out.contains("node: cairn.kernel.cli"));
        assert!(out.contains("dec.kernel-tooling"));
        assert!(out.contains("CLI is the agent surface"));
        assert!(out.contains("Public interface: parse_args"));
        assert!(!out.contains("none linked"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_brief_excludes_non_accepted_decision() {
        let dir = tmpdir("nonaccepted");
        write_export(
            &dir,
            &[
                r#"{"id":"cairn-p","title":"P","status":"open","priority":1,"labels":["cairn-node:cairn.kernel.cli"]}"#,
            ],
        );
        let node = node_record("cairn.kernel.cli", Vec::new());
        let decisions = vec![decision_with_status(
            "dec.proposed-only",
            &["cairn.kernel.cli"],
            "# Not yet binding",
            DecisionStatus::Proposed,
        )];
        let scan = scan_with(vec![node], decisions, ContractSet::default());
        let out = render_brief(&brief_parsed(&["brief", "cairn-p"], false), &dir, &scan);
        assert!(out.contains("node: cairn.kernel.cli"));
        assert!(!out.contains("dec.proposed-only"));
        assert!(out.contains("none linked"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_brief_unlinked_bead_shows_hint() {
        let dir = tmpdir("unlinked");
        write_export(
            &dir,
            &[r#"{"id":"cairn-u","title":"U","status":"open","priority":1}"#],
        );
        let scan = scan_with(Vec::new(), Vec::new(), ContractSet::default());
        let out = render_brief(&brief_parsed(&["brief", "cairn-u"], false), &dir, &scan);
        assert!(out.contains("(unlinked)"));
        assert!(out.contains("cairn-node:<node>"));
        assert!(out.contains("none linked"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_brief_missing_id_reports_not_found() {
        let dir = tmpdir("missing");
        write_export(&dir, &[r#"{"id":"cairn-a","title":"A","status":"open"}"#]);
        let scan = scan_with(Vec::new(), Vec::new(), ContractSet::default());
        let out = render_brief(&brief_parsed(&["brief", "nope"], false), &dir, &scan);
        assert!(out.contains("No bead matches `nope`"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_brief_json_includes_gates_and_staleness() {
        let dir = tmpdir("json");
        write_export(
            &dir,
            &[r#"{"id":"cairn-j","title":"J","status":"open","priority":1}"#],
        );
        let scan = scan_with(Vec::new(), Vec::new(), ContractSet::default());
        let out = render_brief(&brief_parsed(&["brief", "cairn-j"], true), &dir, &scan);
        let value: serde_json::Value = serde_json::from_str(&out).unwrap();
        let brief = &value["brief"];
        assert_eq!(brief["bead"], "cairn-j");
        assert_eq!(brief["ready"], true);
        assert!(brief["gates"].as_str().unwrap().contains("cairn hook all"));
        assert!(brief["staleness"].as_str().unwrap().contains("bd ready"));
        assert!(brief["node"].is_null());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_brief_named_closed_bead_flags_not_ready() {
        let dir = tmpdir("closed");
        write_export(
            &dir,
            &[r#"{"id":"cairn-c","title":"C","status":"closed","priority":1}"#],
        );
        let scan = scan_with(Vec::new(), Vec::new(), ContractSet::default());
        let out = render_brief(&brief_parsed(&["brief", "cairn-c"], false), &dir, &scan);
        assert!(out.contains("Brief: cairn-c"));
        assert!(out.contains("not in the ready set"));
        let json = render_brief(&brief_parsed(&["brief", "cairn-c"], true), &dir, &scan);
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["brief"]["ready"], false);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_brief_named_blocked_bead_flags_not_ready() {
        let dir = tmpdir("blocked");
        write_export(
            &dir,
            &[
                r#"{"id":"cairn-blocker","title":"Blocker","status":"open","priority":1}"#,
                r#"{"id":"cairn-gated","title":"Gated","status":"open","priority":2,"dependencies":[{"depends_on_id":"cairn-blocker","type":"blocks"}]}"#,
            ],
        );
        let scan = scan_with(Vec::new(), Vec::new(), ContractSet::default());
        let out = render_brief(&brief_parsed(&["brief", "cairn-gated"], false), &dir, &scan);
        assert!(out.contains("not in the ready set"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_brief_no_arg_empty_backlog_says_clean() {
        let dir = tmpdir("emptybacklog");
        write_export(&dir, &[]);
        let scan = scan_with(Vec::new(), Vec::new(), ContractSet::default());
        let out = render_brief(&brief_parsed(&["brief"], false), &dir, &scan);
        assert!(out.contains("backlog is empty"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_brief_no_arg_all_blocked_says_none_ready() {
        let dir = tmpdir("allblocked");
        write_export(
            &dir,
            &[
                r#"{"id":"cairn-blocker","title":"B","status":"open","priority":1,"dependencies":[{"depends_on_id":"cairn-other","type":"blocks"}]}"#,
                r#"{"id":"cairn-other","title":"O","status":"open","priority":2,"dependencies":[{"depends_on_id":"cairn-blocker","type":"blocks"}]}"#,
            ],
        );
        let scan = scan_with(Vec::new(), Vec::new(), ContractSet::default());
        let out = render_brief(&brief_parsed(&["brief"], false), &dir, &scan);
        assert!(out.contains("No ready work"));
        assert!(!out.contains("backlog is empty"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
