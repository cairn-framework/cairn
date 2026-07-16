// cairn:allow-large-module reason: single-purpose render surface for remediation/next-action queries; a mechanical split would fragment one cohesive algorithm across files
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
    match query_api::select_next(root, &changes_dir, scan_result) {
        query_api::NextSelection::Dirty(action) => render_next_action(action, parsed.json),
        query_api::NextSelection::Clean(query_api::CleanItem::NativeTodo(todo)) => {
            render_next_todo(todo, scan_result, parsed.json)
        }
        query_api::NextSelection::Clean(query_api::CleanItem::Bead(bead)) => {
            render_next_bead(root, &bead, parsed.json)
        }
        query_api::NextSelection::Clean(query_api::CleanItem::None) => {
            if parsed.json {
                "{\"next\":null,\"clean\":true,\"ready\":0}\n".to_owned()
            } else {
                "Next: nothing to do. Project is clean.\n".to_owned()
            }
        }
    }
}

fn render_next_todo(todo: &Todo, scan_result: &scanner::ScanResult, json: bool) -> String {
    let open_count = query_api::open_native_todos(scan_result).len();
    if json {
        format!(
            "{{\"next\":{{\"todo\":\"{}\",\"node\":\"{}\",\"title\":\"{}\",\"source\":\"native-todos\"}},\"clean\":true,\"ready\":{open_count}}}\n",
            esc(&todo.path),
            esc(&todo.node),
            esc(&decision_summary(&todo.body)),
        )
    } else {
        format!(
            "Next: {}\n  source: native todos ({} open)\n  run: cairn todos {}\n",
            decision_summary(&todo.body),
            open_count,
            todo.node
        )
    }
}

fn render_next_bead(root: &Path, bead: &crate::state::backlog::BacklogItem, json: bool) -> String {
    let backlog = crate::state::backlog::read(root);
    let ready_count = crate::state::backlog::ready(&backlog).len();
    if json {
        format!(
            "{{\"next\":{{\"bead\":\"{}\",\"title\":\"{}\",\"priority\":{},\"source\":\"beads-backlog\"}},\"clean\":true,\"ready\":{ready_count}}}\n",
            esc(&bead.id),
            esc(&bead.title),
            bead.priority,
        )
    } else {
        let mut out = vec![
            format!("Next: {} [P{}] {}", bead.id, bead.priority, bead.title),
            format!("  source: beads backlog ({} ready)", ready_count),
            format!("  run: bd show {}", bead.id),
        ];
        if let Some(node) = bead.linked_node() {
            out.push(format!("  node: {node}"));
        }
        out.join("\n") + "\n"
    }
}

fn render_next_action(action: Option<serde_json::Value>, json: bool) -> String {
    let first = action.unwrap_or(serde_json::Value::Null);
    if json {
        return format!("{{\"next\":{first},\"clean\":false}}\n");
    }
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
}

/// Extracts a one-line summary from a decision body (first markdown heading or
/// first non-empty line), trimmed to a readable length.
pub(super) fn decision_summary(body: &str) -> String {
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

/// Resolves a `todo.<slug>` brief target against the loaded native todos by
/// file name: the target names `todo.<slug>.md` regardless of which declared
/// todos directory holds it. Any status matches; readiness is reported
/// separately so a done or blocked todo still briefs with a warning.
fn resolve_brief_todo<'a>(scan_result: &'a scanner::ScanResult, target: &str) -> Option<&'a Todo> {
    let file_name = format!("{target}.md");
    scan_result
        .artefacts
        .todos
        .iter()
        .find(|todo| Path::new(&todo.path).file_name() == Some(std::ffi::OsStr::new(&file_name)))
}

/// The task-tracking source a brief resolves from. Native Todo artefacts take
/// priority over the beads backlog (`dec.native-todos-first`). An explicit
/// argument starting with `todo.` targets a native todo by slug; any other
/// argument resolves against the backlog.
enum BriefSource<'a> {
    /// A native `meta/todos/` artefact.
    Todo(&'a Todo),
    /// A beads-backlog item.
    Bead(&'a crate::state::backlog::BacklogItem),
}

impl BriefSource<'_> {
    fn node_id(&self) -> Option<&str> {
        match self {
            Self::Todo(todo) => Some(todo.node.as_str()),
            Self::Bead(bead) => bead.linked_node(),
        }
    }

    fn task_body(&self) -> String {
        match self {
            Self::Todo(todo) => todo.body.trim().to_owned(),
            Self::Bead(bead) => bead.description.trim().to_owned(),
        }
    }

    /// Identifier shown in the "(unlinked)" hint when no node resolves.
    fn hint_id(&self) -> &str {
        match self {
            Self::Todo(todo) => todo.path.as_str(),
            Self::Bead(bead) => bead.id.as_str(),
        }
    }
}

/// Renders `cairn brief [<target>]`: the next (or named) ready unit fused
/// with its binding decisions, contract, acceptance criteria, and the gates
/// that judge it, so a fresh agent can pick up work safely from one command.
/// With no argument an open native todo takes priority over the beads
/// backlog. A target starting with `todo.` resolves a native todo by slug;
/// any other target resolves as a bead id.
pub(crate) fn render_brief(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
) -> String {
    if parsed.command_args.get(1).is_none() {
        let todos = query_api::open_native_todos(scan_result);
        if let Some(todo) = todos.first().copied() {
            return render_brief_data(parsed, scan_result, &BriefSource::Todo(todo), true);
        }
    }
    if let Some(target) = parsed
        .command_args
        .get(1)
        .filter(|target| target.starts_with("todo."))
    {
        let Some(todo) = resolve_brief_todo(scan_result, target) else {
            let message = crate::copy::lookup("brief.todo-not-found").replace("{id}", target);
            if parsed.json {
                return format!("{{\"brief\":null,\"message\":\"{}\"}}\n", esc(&message));
            }
            return format!("{message}\n");
        };
        let ready_now = todo.status == TodoStatus::Open;
        return render_brief_data(parsed, scan_result, &BriefSource::Todo(todo), ready_now);
    }

    let items = crate::state::backlog::read(root);
    let Some(bead) = resolve_brief_bead(parsed, &items) else {
        let message = match parsed.command_args.get(1) {
            Some(id) => crate::copy::lookup("brief.not-found").replace("{id}", id),
            None if items.is_empty() => crate::copy::lookup("brief.empty").to_owned(),
            None => crate::copy::lookup("brief.none-ready").to_owned(),
        };
        if parsed.json {
            return format!("{{\"brief\":null,\"message\":\"{}\"}}\n", esc(&message));
        }
        return format!("{message}\n");
    };
    let ready_now = crate::state::backlog::ready(&items)
        .iter()
        .any(|item| item.id == bead.id);
    render_brief_data(parsed, scan_result, &BriefSource::Bead(&bead), ready_now)
}

/// Resolves node/decisions/contract for `source` and renders the brief in the
/// requested format. Shared by the native-todo and beads-backlog paths.
fn render_brief_data(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
    source: &BriefSource,
    ready_now: bool,
) -> String {
    let node = source
        .node_id()
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
    let gates = crate::copy::lookup("brief.gates");
    let staleness = match source {
        BriefSource::Todo(_) => crate::copy::lookup("brief.staleness-note-todos"),
        BriefSource::Bead(_) => crate::copy::lookup("brief.staleness-note-beads"),
    };

    let data = BriefData {
        source,
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
    source: &'a BriefSource<'a>,
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
    let payload = match data.source {
        BriefSource::Todo(todo) => serde_json::json!({
            "brief": {
                "todo": todo.path,
                "title": decision_summary(&todo.body),
                "source": "native-todos",
                "ready": data.ready_now,
                "node": data.node.map(|node| node.id.clone()),
                "task": data.source.task_body(),
                "decisions": decisions_json,
                "contract": data.contract,
                "gates": data.gates,
                "staleness": data.staleness,
            }
        }),
        BriefSource::Bead(bead) => serde_json::json!({
            "brief": {
                "bead": bead.id,
                "title": bead.title,
                "priority": bead.priority,
                "source": "beads-backlog",
                "ready": data.ready_now,
                "node": data.node.map(|node| node.id.clone()),
                "task": data.source.task_body(),
                "decisions": decisions_json,
                "contract": data.contract,
                "gates": data.gates,
                "staleness": data.staleness,
            }
        }),
    };
    format!("{payload}\n")
}

/// Renders a [`BriefData`] as the human-readable brief: header, readiness
/// warning, linked node, task body, binding decisions, contract, and gates.
fn format_brief_human(data: &BriefData) -> String {
    let mut out = match data.source {
        BriefSource::Todo(todo) => {
            let mut lines = vec![
                format!("Brief: {}", decision_summary(&todo.body)),
                "  source: native todos".to_owned(),
                format!("  run: cairn todos {}", todo.node),
            ];
            if !data.ready_now {
                lines.push(format!("  {}", crate::copy::lookup("brief.not-ready-todo")));
            }
            lines
        }
        BriefSource::Bead(bead) => {
            let mut lines = vec![
                format!("Brief: {} [P{}] {}", bead.id, bead.priority, bead.title),
                format!("  run: bd show {}", bead.id),
            ];
            if !data.ready_now {
                lines.push(format!("  {}", crate::copy::lookup("brief.not-ready")));
            }
            lines
        }
    };
    match data.node {
        Some(node) => out.push(format!("  node: {}", node.id)),
        None => out.push(format!(
            "  node: (unlinked) {}",
            crate::copy::lookup("brief.unlinked-hint").replace("{id}", data.source.hint_id())
        )),
    }
    out.push(String::new());
    out.push("Task:".to_owned());
    out.push(data.source.task_body());
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
            verbose: false,
            brief: false,
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
            symbols: Vec::new(),
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

    fn scan_with_todos(
        nodes: Vec<NodeRecord>,
        decisions: Vec<Decision>,
        contracts: ContractSet,
        todos: Vec<Todo>,
    ) -> ScanResult {
        let mut scan = scan_with(nodes, decisions, contracts);
        scan.artefacts.todos = todos;
        scan
    }

    fn todo_fixture(node: &str, created: &str, body: &str) -> Todo {
        Todo {
            path: format!("meta/todos/todo.{node}.md"),
            node: node.to_owned(),
            status: TodoStatus::Open,
            created: created.to_owned(),
            satisfies: None,
            body: body.to_owned(),
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
            gap: false,
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
                interface: Vec::new(),
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

    #[test]
    fn test_open_native_todos_filters_and_sorts() {
        let scan = scan_with_todos(
            Vec::new(),
            Vec::new(),
            ContractSet::default(),
            vec![
                Todo {
                    status: TodoStatus::Done,
                    ..todo_fixture("app.done", "2026-01-01", "# Done")
                },
                todo_fixture("app.later", "2026-02-01", "# Later todo"),
                todo_fixture("app.earlier", "2026-01-15", "# Earlier todo"),
            ],
        );
        let open = query_api::open_native_todos(&scan);
        assert_eq!(open.len(), 2, "done todo must be excluded");
        assert_eq!(open[0].node, "app.earlier", "earliest created sorts first");
        assert_eq!(open[1].node, "app.later");
    }

    #[test]
    fn test_render_next_clean_prefers_native_todo_over_beads() {
        let dir = tmpdir("next-native-todo");
        write_export(
            &dir,
            &[r#"{"id":"cairn-a","title":"Alpha","status":"open","priority":1}"#],
        );
        let scan = scan_with_todos(
            Vec::new(),
            Vec::new(),
            ContractSet::default(),
            vec![todo_fixture("app.core", "2026-01-01", "# Wire the thing")],
        );
        let human = render_next(&brief_parsed(&["next"], false), &dir, &scan);
        assert!(human.contains("Wire the thing"));
        assert!(human.contains("source: native todos (1 open)"));
        assert!(human.contains("run: cairn todos app.core"));
        assert!(!human.contains("Alpha"), "must not fall through to beads");

        let json = render_next(&brief_parsed(&["next"], true), &dir, &scan);
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["next"]["source"], "native-todos");
        assert_eq!(value["next"]["node"], "app.core");
        assert_eq!(value["clean"], true);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_render_next_clean_falls_back_to_beads_without_todos() {
        let dir = tmpdir("next-beads-fallback");
        write_export(
            &dir,
            &[r#"{"id":"cairn-a","title":"Alpha","status":"open","priority":1}"#],
        );
        let scan = scan_with(Vec::new(), Vec::new(), ContractSet::default());
        let human = render_next(&brief_parsed(&["next"], false), &dir, &scan);
        assert!(human.contains("Alpha"));
        assert!(human.contains("source: beads backlog"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_brief_prefers_open_native_todo_over_beads() {
        let dir = tmpdir("brief-native-todo");
        write_export(
            &dir,
            &[r#"{"id":"cairn-a","title":"Alpha","status":"open","priority":1}"#],
        );
        let node = node_record("app.core", vec!["meta/contracts/core.md".to_owned()]);
        let mut contracts = ContractSet::default();
        contracts.contracts.insert(
            "meta/contracts/core.md".to_owned(),
            Contract {
                path: "meta/contracts/core.md".to_owned(),
                declared_by: "app.core".to_owned(),
                node: "app.core".to_owned(),
                body: "Public interface: run".to_owned(),
                interface: Vec::new(),
            },
        );
        let decisions = vec![decision("dec.core-shape", &["app.core"], "# Core shape")];
        let scan = scan_with_todos(
            vec![node],
            decisions,
            contracts,
            vec![todo_fixture(
                "app.core",
                "2026-01-01",
                "# Wire the thing\n\nUse the shared client.",
            )],
        );
        let human = render_brief(&brief_parsed(&["brief"], false), &dir, &scan);
        assert!(human.contains("Brief: Wire the thing"));
        assert!(human.contains("source: native todos"));
        assert!(human.contains("run: cairn todos app.core"));
        assert!(human.contains("node: app.core"));
        assert!(human.contains("dec.core-shape"));
        assert!(human.contains("Public interface: run"));
        assert!(!human.contains("Alpha"), "must not fall through to beads");

        let json = render_brief(&brief_parsed(&["brief"], true), &dir, &scan);
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["brief"]["source"], "native-todos");
        assert_eq!(value["brief"]["node"], "app.core");
        assert_eq!(value["brief"]["ready"], true);
        assert_eq!(value["brief"]["todo"], "meta/todos/todo.app.core.md");
        assert_eq!(value["brief"]["title"], "Wire the thing");

        // An explicit argument that does not start with `todo.` still
        // targets the beads backlog.
        let named = render_brief(&brief_parsed(&["brief", "cairn-a"], false), &dir, &scan);
        assert!(named.contains("Brief: cairn-a [P1] Alpha"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_brief_native_todo_has_no_repo_specific_gates_or_staleness() {
        // A native-todo brief must not leak cairn-repo-specific gates (cargo)
        // or beads-backlog staleness guidance (`bd ready`) into a downstream
        // repository's surface. Gates stay generic; staleness uses the
        // todo-specific note instead of the beads one.
        let dir = tmpdir("brief-native-no-leak");
        write_export(&dir, &[]);
        let scan = scan_with_todos(
            Vec::new(),
            Vec::new(),
            ContractSet::default(),
            vec![todo_fixture("app.core", "2026-01-01", "# Wire the thing")],
        );
        let human = render_brief(&brief_parsed(&["brief"], false), &dir, &scan);
        assert!(
            !human.contains("cargo "),
            "native-todo brief must not carry cargo gates:\n{human}"
        );
        assert!(
            !human.contains("bd ready"),
            "native-todo brief must not carry beads staleness:\n{human}"
        );

        let json = render_brief(&brief_parsed(&["brief"], true), &dir, &scan);
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(
            !value["brief"]["gates"].as_str().unwrap().contains("cargo "),
            "gates must be generic, not repo-specific"
        );
        assert!(
            !value["brief"]["staleness"]
                .as_str()
                .unwrap()
                .contains("bd ready"),
            "staleness must be todo-specific, not beads guidance"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_brief_named_todo_slug_resolves_fused_context() {
        // `cairn brief todo.<slug>` must resolve the named native todo and
        // fuse the same node, decision, and contract context the bead path
        // gets, even when the beads backlog also has entries.
        let dir = tmpdir("brief-named-todo");
        write_export(
            &dir,
            &[r#"{"id":"cairn-a","title":"Alpha","status":"open","priority":1}"#],
        );
        let node = node_record("app.core", vec!["meta/contracts/core.md".to_owned()]);
        let mut contracts = ContractSet::default();
        contracts.contracts.insert(
            "meta/contracts/core.md".to_owned(),
            Contract {
                path: "meta/contracts/core.md".to_owned(),
                declared_by: "app.core".to_owned(),
                node: "app.core".to_owned(),
                body: "Public interface: run".to_owned(),
                interface: Vec::new(),
            },
        );
        let decisions = vec![decision("dec.core-shape", &["app.core"], "# Core shape")];
        let scan = scan_with_todos(
            vec![node],
            decisions,
            contracts,
            vec![
                todo_fixture("app.other", "2026-01-01", "# Earlier todo"),
                todo_fixture("app.core", "2026-02-01", "# Wire the thing"),
            ],
        );
        // Names the LATER todo, so the pick is targeting, not top-of-list.
        let human = render_brief(
            &brief_parsed(&["brief", "todo.app.core"], false),
            &dir,
            &scan,
        );
        assert!(human.contains("Brief: Wire the thing"));
        assert!(human.contains("source: native todos"));
        assert!(human.contains("node: app.core"));
        assert!(human.contains("dec.core-shape"));
        assert!(human.contains("Public interface: run"));
        assert!(!human.contains("Alpha"), "must not fall through to beads");
        assert!(!human.contains("Earlier todo"), "must pick the named todo");

        let json = render_brief(
            &brief_parsed(&["brief", "todo.app.core"], true),
            &dir,
            &scan,
        );
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["brief"]["source"], "native-todos");
        assert_eq!(value["brief"]["todo"], "meta/todos/todo.app.core.md");
        assert_eq!(value["brief"]["node"], "app.core");
        assert_eq!(value["brief"]["ready"], true);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_brief_named_todo_unknown_slug_errors() {
        // An unknown `todo.` target must fail with a clear message, never
        // fall through to bead resolution.
        let dir = tmpdir("brief-named-todo-missing");
        write_export(
            &dir,
            &[r#"{"id":"cairn-a","title":"Alpha","status":"open","priority":1}"#],
        );
        let scan = scan_with_todos(
            Vec::new(),
            Vec::new(),
            ContractSet::default(),
            vec![todo_fixture("app.core", "2026-01-01", "# Wire the thing")],
        );
        let human = render_brief(&brief_parsed(&["brief", "todo.nope"], false), &dir, &scan);
        assert!(human.contains("No todo matches `todo.nope`"), "{human}");
        assert!(!human.contains("Alpha"), "must not fall through to beads");

        let json = render_brief(&brief_parsed(&["brief", "todo.nope"], true), &dir, &scan);
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(value["brief"].is_null());
        assert!(
            value["message"]
                .as_str()
                .unwrap()
                .contains("No todo matches")
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_brief_named_todo_not_open_flags_not_ready() {
        // A named todo briefs in any status, with readiness reported so a
        // done or blocked unit is not silently presented as workable.
        let dir = tmpdir("brief-named-todo-done");
        write_export(&dir, &[]);
        let scan = scan_with_todos(
            Vec::new(),
            Vec::new(),
            ContractSet::default(),
            vec![Todo {
                status: TodoStatus::Done,
                ..todo_fixture("app.core", "2026-01-01", "# Wire the thing")
            }],
        );
        let human = render_brief(
            &brief_parsed(&["brief", "todo.app.core"], false),
            &dir,
            &scan,
        );
        assert!(human.contains("Brief: Wire the thing"));
        assert!(human.contains("this todo is not open"), "{human}");

        let json = render_brief(
            &brief_parsed(&["brief", "todo.app.core"], true),
            &dir,
            &scan,
        );
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["brief"]["ready"], false);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_brief_native_todo_unlinked_node_shows_hint() {
        // A todo whose `node:` does not resolve against the blueprint graph
        // (e.g. the node was renamed or removed after the todo was filed)
        // must still render, falling through to the "(unlinked)" hint the
        // beads path already exercises, rather than panicking or silently
        // dropping the task.
        let dir = tmpdir("brief-todo-unlinked");
        write_export(&dir, &[]);
        let scan = scan_with_todos(
            Vec::new(),
            Vec::new(),
            ContractSet::default(),
            vec![todo_fixture("app.gone", "2026-01-01", "# Orphaned task")],
        );
        let human = render_brief(&brief_parsed(&["brief"], false), &dir, &scan);
        assert!(human.contains("Brief: Orphaned task"));
        assert!(human.contains("source: native todos"));
        assert!(human.contains("(unlinked)"));
        assert!(human.contains("meta/todos/todo.app.gone.md"));
        assert!(human.contains("none linked"));

        let json = render_brief(&brief_parsed(&["brief"], true), &dir, &scan);
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["brief"]["source"], "native-todos");
        assert!(value["brief"]["node"].is_null());
        assert_eq!(value["brief"]["todo"], "meta/todos/todo.app.gone.md");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
