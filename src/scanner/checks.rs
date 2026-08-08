//! Blueprint change, provenance, decision-accumulation, and gitignored-path checks emitted during scanning.

use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use super::{ArtefactSet, Graph, blueprint, config, state};
use crate::artefacts::registry::DecisionStatus;

pub(crate) fn check_blueprint_change_decisions(
    graph: &mut Graph,
    artefacts: &ArtefactSet,
    current: &state::BlueprintSnapshot,
    previous: &state::BlueprintSnapshot,
) {
    if previous.is_empty() {
        return;
    }
    if artefacts.decisions.is_empty() {
        return;
    }

    let covered: BTreeSet<&str> = artefacts
        .decisions
        .iter()
        .filter(|d| {
            matches!(
                d.status,
                DecisionStatus::Proposed | DecisionStatus::Accepted | DecisionStatus::Superseded
            )
        })
        .flat_map(|d| d.nodes.iter().map(String::as_str))
        .collect();

    let mut emitted: BTreeSet<String> = BTreeSet::new();
    let mut emit = |node_id: &str| {
        if !covered.contains(node_id) && emitted.insert(node_id.to_owned()) {
            graph.findings.push(crate::map::graph::Finding {
                code: "CAIRN_BLUEPRINT_CHANGE_NO_DECISION".to_owned(),
                severity: crate::map::graph::FindingSeverity::Error,
                message: format!(
                    "blueprint shape changed for node `{node_id}` but no decision artefact covers it"
                ),
                node: Some(node_id.to_owned()),
                target: None,
                path: None,
                        deferred_by: None,
                        parked_by: None,
});
        }
    };

    // Added nodes.
    for id in current.nodes.keys() {
        if !previous.nodes.contains_key(id) {
            emit(id);
        }
    }
    // Removed nodes.
    for id in previous.nodes.keys() {
        if !current.nodes.contains_key(id) {
            emit(id);
        }
    }
    // Structural changes: parent or kind changed. Path-only changes are not gated.
    for (id, cur_fp) in &current.nodes {
        if let Some(prev_fp) = previous.nodes.get(id)
            && (cur_fp.parent != prev_fp.parent || cur_fp.kind != prev_fp.kind)
        {
            emit(id);
        }
    }
    // Dependency-edge drift. Schema v2 added per-node outbound-edge tracking; a
    // pre-v2 baseline recorded no edges, so skip until a v2 snapshot is written
    // to avoid flagging every edge as new on the first scan after upgrade.
    if previous.version >= 2 {
        for (id, cur_fp) in &current.nodes {
            if let Some(prev_fp) = previous.nodes.get(id)
                && cur_fp.edges != prev_fp.edges
            {
                emit(id);
            }
        }
    }
}

pub(crate) fn check_provenance_coverage(graph: &mut Graph, artefacts: &ArtefactSet) {
    if artefacts.decisions.is_empty() {
        return;
    }
    let covered: BTreeSet<&str> = artefacts
        .decisions
        .iter()
        .flat_map(|d| d.nodes.iter().map(String::as_str))
        .collect();
    for node in graph.nodes.values() {
        if node.children.is_empty() && !covered.contains(node.id.as_str()) {
            graph.findings.push(crate::map::graph::Finding {
                code: "CAIRN_PROVENANCE_NO_DECISION".to_owned(),
                severity: crate::map::graph::FindingSeverity::Warning,
                message: format!(
                    "node `{}` has no decision artefact explaining why it exists",
                    node.id
                ),
                node: Some(node.id.clone()),
                target: None,
                path: None,
                deferred_by: None,
                parked_by: None,
            });
        }
    }
}

/// Emits `CAIRN_DECISION_ACCUMULATION` for every graph node carrying more
/// than `threshold` directly-attached accepted decisions.
///
/// Decisions naming a node absent from the graph are
/// `CAIRN_DECISION_ORPHANED`, not accumulation, so they are skipped, and a
/// node repeated inside one decision's `nodes:` list counts once.
pub(crate) fn check_decision_accumulation(
    graph: &mut Graph,
    artefacts: &ArtefactSet,
    threshold: usize,
) {
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for decision in &artefacts.decisions {
        if !matches!(decision.status, DecisionStatus::Accepted) {
            continue;
        }
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for node in &decision.nodes {
            if graph.nodes.contains_key(node) && seen.insert(node.as_str()) {
                *counts.entry(node.as_str()).or_default() += 1;
            }
        }
    }
    let body = crate::copy::lookup("findings.codes.CAIRN_DECISION_ACCUMULATION.body");
    for (node, count) in counts {
        if count <= threshold {
            continue;
        }
        graph.findings.push(crate::map::graph::Finding {
            code: "CAIRN_DECISION_ACCUMULATION".to_owned(),
            severity: crate::map::graph::FindingSeverity::Info,
            message: body
                .replace("{node}", node)
                .replace("{count}", &count.to_string())
                .replace("{threshold}", &threshold.to_string()),
            node: Some(node.to_owned()),
            target: None,
            path: None,
            deferred_by: None,
            parked_by: None,
        });
    }
}

pub(crate) fn check_orphan_beads(graph: &mut Graph, root: &Path) {
    let node_ids: BTreeSet<&str> = graph.nodes.values().map(|node| node.id.as_str()).collect();
    for bead in crate::state::backlog::read(root) {
        let Some(node) = bead.linked_node() else {
            continue;
        };
        if !node_ids.contains(node) {
            graph.findings.push(crate::map::graph::Finding {
                code: "CAIRN_BACKLOG_ORPHAN_NODE".to_owned(),
                severity: crate::map::graph::FindingSeverity::Warning,
                message: format!(
                    "bead `{}` references unknown node `{}` via its cairn-node label",
                    bead.id, node
                ),
                node: Some(node.to_owned()),
                target: None,
                path: None,
                deferred_by: None,
                parked_by: None,
            });
        }
    }
}

pub(crate) fn check_gitignored_paths(graph: &mut Graph, ast: &blueprint::Ast, ignores: &[String]) {
    let mut emit_for = |node: &blueprint::Node| {
        for path in &node.paths {
            let rel = path.trim_start_matches("./").trim_start_matches('/');
            if config::is_ignored(rel, ignores) {
                graph.findings.push(crate::map::graph::Finding {
                    code: "CAIRN_PATH_GITIGNORED".to_owned(),
                    severity: crate::map::graph::FindingSeverity::Warning,
                    message: format!(
                        "node `{}` declares path `{path}` which matches a .gitignore pattern; will appear as a Ghost node",
                        node.id
                    ),
                    node: Some(node.id.clone()),
                    target: None,
                    path: Some(path.clone()),
                                deferred_by: None,
                                parked_by: None,
});
            }
        }
    };
    visit_nodes(&ast.nodes, &mut emit_for);
}

/// Emits an informational finding for every node tag not declared in the
/// project's opt-in registry.
pub(crate) fn check_tag_registry(graph: &mut Graph, registry: &config::TagRegistry) {
    let mut findings = Vec::new();
    for node in graph.nodes.values() {
        let tags: BTreeSet<&str> = node.tags.iter().map(String::as_str).collect();
        for tag in tags {
            if registry.contains(tag) {
                continue;
            }
            findings.push(crate::map::graph::Finding {
                code: "CAIRN_TAG_UNREGISTERED".to_owned(),
                severity: crate::map::graph::FindingSeverity::Info,
                message: format!(
                    "node `{}` uses tag `{tag}` which is not declared in the `tags:` registry",
                    node.id
                ),
                node: Some(node.id.clone()),
                target: None,
                path: None,
                deferred_by: None,
                parked_by: None,
            });
        }
    }
    graph.findings.extend(findings);
}

/// Emits `CAIRN_CONTRACT_INTERFACE_DRIFT` for every contract `interface:`
/// entry that does not match a symbol extracted from the contract's node.
/// Opt-in: contracts with no `interface:` block are never checked, and
/// symbols not listed in `interface:` are never findings (contracts declare
/// a load-bearing subset, not full coverage).
pub(crate) fn check_contract_interface_drift(
    graph: &mut Graph,
    contracts: &crate::artefacts::contract::ContractSet,
) {
    let mut findings = Vec::new();
    for contract in contracts.contracts.values() {
        if contract.interface.is_empty() {
            continue;
        }
        let Some(node) = graph.nodes.get(&contract.node) else {
            continue;
        };
        for entry in &contract.interface {
            let normalized = crate::reconcile::normalize_symbol(entry);
            let matched = node
                .symbols
                .iter()
                .any(|record| record.signature == normalized);
            if !matched {
                findings.push(crate::map::graph::Finding {
                    code: "CAIRN_CONTRACT_INTERFACE_DRIFT".to_owned(),
                    severity: crate::map::graph::FindingSeverity::Warning,
                    message: format!(
                        "contract `{}` declares interface entry `{entry}` not found among `{}`'s extracted symbols",
                        contract.path, contract.node
                    ),
                    node: Some(contract.node.clone()),
                    target: None,
                    path: Some(contract.path.clone()),
                                deferred_by: None,
                                parked_by: None,
});
            }
        }
    }
    graph.findings.extend(findings);
}

fn visit_nodes<F: FnMut(&blueprint::Node)>(nodes: &[blueprint::Node], f: &mut F) {
    let mut stack: Vec<&blueprint::Node> = nodes.iter().collect();
    while let Some(node) = stack.pop() {
        f(node);
        for child in &node.children {
            stack.push(child);
        }
    }
}

#[cfg(test)]
mod interface_drift_tests {
    use std::collections::BTreeMap;

    use super::check_contract_interface_drift;
    use crate::{
        artefacts::contract::{Contract, ContractSet},
        blueprint::{NodeKind, Span},
        map::graph::{Graph, NodeRecord, NodeState},
        reconcile::{SymbolKind, SymbolRecord},
    };

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

    fn symbol(signature: &str) -> SymbolRecord {
        SymbolRecord {
            name: signature.to_owned(),
            kind: SymbolKind::Function,
            signature: signature.to_owned(),
            file: "src/lib.rs".to_owned(),
            line: 1,
            end_line: 1,
        }
    }

    fn contract(node: &str, interface: Vec<&str>) -> Contract {
        Contract {
            path: format!("meta/contracts/{node}.md"),
            declared_by: node.to_owned(),
            node: node.to_owned(),
            body: String::new(),
            interface: interface.into_iter().map(str::to_owned).collect(),
        }
    }

    fn graph_with(node: NodeRecord) -> Graph {
        let mut nodes = BTreeMap::new();
        nodes.insert(node.id.clone(), node);
        Graph {
            nodes,
            names: BTreeMap::new(),
            outbound: BTreeMap::new(),
            inbound: BTreeMap::new(),
            findings: Vec::new(),
        }
    }

    fn contract_set(contracts: Vec<Contract>) -> ContractSet {
        ContractSet {
            contracts: contracts.into_iter().map(|c| (c.path.clone(), c)).collect(),
            findings: Vec::new(),
        }
    }

    #[test]
    fn matching_interface_entry_emits_no_finding() {
        let mut node = bare_node("app.api");
        node.symbols.push(symbol("fn handle()"));
        let mut graph = graph_with(node);
        let contracts = contract_set(vec![contract("app.api", vec!["fn handle()"])]);

        check_contract_interface_drift(&mut graph, &contracts);

        assert!(graph.findings.is_empty());
    }

    #[test]
    fn bogus_interface_entry_emits_drift_finding() {
        let mut node = bare_node("app.api");
        node.symbols.push(symbol("fn handle()"));
        let mut graph = graph_with(node);
        let contracts = contract_set(vec![contract("app.api", vec!["fn does_not_exist()"])]);

        check_contract_interface_drift(&mut graph, &contracts);

        let finding = graph
            .findings
            .iter()
            .find(|f| f.code == "CAIRN_CONTRACT_INTERFACE_DRIFT")
            .expect("bogus interface entry must warn");
        assert_eq!(finding.node.as_deref(), Some("app.api"));
        assert!(finding.message.contains("does_not_exist"));
    }

    #[test]
    fn empty_interface_block_emits_nothing() {
        let mut node = bare_node("app.api");
        node.symbols.push(symbol("fn handle()"));
        let mut graph = graph_with(node);
        let contracts = contract_set(vec![contract("app.api", Vec::new())]);

        check_contract_interface_drift(&mut graph, &contracts);

        assert!(graph.findings.is_empty());
    }

    #[test]
    fn symbol_not_listed_in_interface_is_never_a_finding() {
        let mut node = bare_node("app.api");
        node.symbols.push(symbol("fn handle()"));
        node.symbols.push(symbol("fn undeclared_helper()"));
        let mut graph = graph_with(node);
        let contracts = contract_set(vec![contract("app.api", vec!["fn handle()"])]);

        check_contract_interface_drift(&mut graph, &contracts);

        assert!(graph.findings.is_empty());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blueprint::{Node, NodeKind, Span};

    fn leaf(id: &str) -> Node {
        Node {
            kind: NodeKind::Module,
            name: id.to_owned(),
            description: String::new(),
            id: id.to_owned(),
            tags: Vec::new(),
            paths: Vec::new(),
            owns_files: false,
            contracts: Vec::new(),
            raw_fields: Vec::new(),
            children: Vec::new(),
            span: Span::point("test", 1, 1),
        }
    }

    fn node(id: &str, children: Vec<Node>) -> Node {
        Node {
            children,
            ..leaf(id)
        }
    }

    #[test]
    fn visit_nodes_collects_all_nodes_in_tree() {
        let tree = vec![node(
            "root",
            vec![
                node("child-a", vec![node("grandchild", Vec::new())]),
                node("child-b", Vec::new()),
            ],
        )];

        let mut visited = Vec::new();
        visit_nodes(&tree, &mut |n| visited.push(n.id.clone()));

        // Order is stack-based and not part of the contract; assert the set.
        visited.sort();
        assert_eq!(visited, vec!["child-a", "child-b", "grandchild", "root"]);
    }

    #[test]
    fn visit_nodes_empty_input_invokes_callback_never() {
        let mut visited = Vec::new();
        visit_nodes(&[], &mut |n| visited.push(n.id.clone()));
        assert!(visited.is_empty());
    }
}
