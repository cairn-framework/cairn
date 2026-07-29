//! Contract node-shape drift: compares a node's recorded contract baseline
//! against the shape the blueprint declares now.

use std::collections::BTreeSet;

use super::{Graph, contract_baselines::ContractBaselines, state};

/// Emits `CAIRN_CONTRACT_NODE_SHAPE_DRIFT` for every node whose recorded
/// baseline no longer matches the shape the blueprint declares now.
///
/// Opt-in by construction: a node is compared only when it holds a baseline
/// entry, the blueprint declares it, and its contract currently loads. A
/// repository that has recorded no baseline can never trip this check, and no
/// entry is ever backfilled. The scanner is never a writer here: refreshing a
/// baseline during a scan would silently re-accept the drift the finding
/// exists to report.
pub(crate) fn check_contract_node_shape_drift(
    graph: &mut Graph,
    contracts: &crate::artefacts::contract::ContractSet,
    baselines: &ContractBaselines,
    current: &state::BlueprintSnapshot,
) {
    if baselines.nodes.is_empty() {
        return;
    }
    // Keyed by the pointer-owning node: the contract a node declares is what a
    // reviewer re-reads, and a node whose pointer was removed is inert even if
    // some other contract's frontmatter still names it.
    let contracted: BTreeSet<&str> = contracts
        .contracts
        .values()
        .map(|contract| contract.declared_by.as_str())
        .collect();
    for (node_id, baseline) in &baselines.nodes {
        if !contracted.contains(node_id.as_str()) {
            continue;
        }
        let Some(fingerprint) = current.nodes.get(node_id) else {
            continue;
        };
        // Canonical field order. `paths` is deliberately not compared, matching
        // `check_blueprint_change_decisions`, which leaves path-only edits ungated.
        let mut changed = Vec::with_capacity(3);
        if fingerprint.kind != baseline.kind {
            changed.push("kind");
        }
        if fingerprint.parent != baseline.parent {
            changed.push("parent");
        }
        if fingerprint.edges != baseline.edges {
            changed.push("edges");
        }
        if changed.is_empty() {
            continue;
        }
        let target = changed.join(", ");
        graph.findings.push(crate::map::graph::Finding {
            code: "CAIRN_CONTRACT_NODE_SHAPE_DRIFT".to_owned(),
            severity: crate::map::graph::FindingSeverity::Warning,
            message: crate::copy::lookup("findings.codes.CAIRN_CONTRACT_NODE_SHAPE_DRIFT.body")
                .replace("{node}", node_id)
                .replace("{target}", &target),
            node: Some(node_id.clone()),
            target: Some(target),
            path: None,
            deferred_by: None,
            parked_by: None,
        });
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{ContractBaselines, check_contract_node_shape_drift, state};
    use crate::{
        artefacts::contract::{Contract, ContractSet},
        map::graph::{FindingSeverity, Graph},
        scanner::contract_baselines::ContractBaseline,
    };

    fn empty_graph() -> Graph {
        Graph {
            nodes: BTreeMap::new(),
            names: BTreeMap::new(),
            outbound: BTreeMap::new(),
            inbound: BTreeMap::new(),
            findings: Vec::new(),
        }
    }

    fn contract_set(nodes: &[&str]) -> ContractSet {
        ContractSet {
            contracts: nodes
                .iter()
                .map(|node| {
                    let path = format!("meta/contracts/{node}.md");
                    (
                        path.clone(),
                        Contract {
                            path,
                            declared_by: (*node).to_owned(),
                            node: (*node).to_owned(),
                            body: String::new(),
                            interface: Vec::new(),
                        },
                    )
                })
                .collect(),
            findings: Vec::new(),
        }
    }

    fn baseline(kind: &str, parent: Option<&str>, edges: &[&str]) -> ContractBaseline {
        ContractBaseline {
            kind: kind.to_owned(),
            parent: parent.map(str::to_owned),
            edges: edges.iter().map(|e| (*e).to_owned()).collect(),
        }
    }

    fn baselines(entries: Vec<(&str, ContractBaseline)>) -> ContractBaselines {
        let mut set = ContractBaselines::new();
        for (node, entry) in entries {
            set.nodes.insert(node.to_owned(), entry);
        }
        set
    }

    fn fingerprint(
        kind: &str,
        parent: Option<&str>,
        paths: &[&str],
        edges: &[&str],
    ) -> state::NodeFingerprint {
        state::NodeFingerprint {
            kind: kind.to_owned(),
            parent: parent.map(str::to_owned),
            paths: paths.iter().map(|p| (*p).to_owned()).collect(),
            edges: edges.iter().map(|e| (*e).to_owned()).collect(),
        }
    }

    fn snapshot(entries: Vec<(&str, state::NodeFingerprint)>) -> state::BlueprintSnapshot {
        let mut snap = state::BlueprintSnapshot::new();
        for (node, entry) in entries {
            snap.nodes.insert(node.to_owned(), entry);
        }
        snap
    }

    #[test]
    fn no_baselines_recorded_emits_nothing() {
        let mut graph = empty_graph();
        let current = snapshot(vec![(
            "app.api",
            fingerprint("Module", Some("app"), &[], &[]),
        )]);
        check_contract_node_shape_drift(
            &mut graph,
            &contract_set(&["app.api"]),
            &ContractBaselines::new(),
            &current,
        );
        assert!(
            graph.findings.is_empty(),
            "upgrading with no baseline file must never flag"
        );
    }

    #[test]
    fn matching_baseline_emits_nothing() {
        let mut graph = empty_graph();
        let current = snapshot(vec![(
            "app.api",
            fingerprint("Module", Some("app"), &["./src/api"], &["app.core"]),
        )]);
        check_contract_node_shape_drift(
            &mut graph,
            &contract_set(&["app.api"]),
            &baselines(vec![(
                "app.api",
                baseline("Module", Some("app"), &["app.core"]),
            )]),
            &current,
        );
        assert!(
            graph.findings.is_empty(),
            "a re-recorded baseline must clear the finding"
        );
    }

    #[test]
    fn path_only_change_emits_nothing() {
        let mut graph = empty_graph();
        let current = snapshot(vec![(
            "app.api",
            fingerprint("Module", Some("app"), &["./src/moved"], &[]),
        )]);
        check_contract_node_shape_drift(
            &mut graph,
            &contract_set(&["app.api"]),
            &baselines(vec![("app.api", baseline("Module", Some("app"), &[]))]),
            &current,
        );
        assert!(
            graph.findings.is_empty(),
            "path-only edits stay ungated, as for blueprint-change decisions"
        );
    }

    #[test]
    fn shape_edit_emits_exactly_one_warning_naming_the_node() {
        for current_fp in [
            fingerprint("Container", Some("app"), &[], &[]),
            fingerprint("Module", Some("app.other"), &[], &[]),
            fingerprint("Module", Some("app"), &[], &["app.core"]),
        ] {
            let mut graph = empty_graph();
            check_contract_node_shape_drift(
                &mut graph,
                &contract_set(&["app.api"]),
                &baselines(vec![("app.api", baseline("Module", Some("app"), &[]))]),
                &snapshot(vec![("app.api", current_fp)]),
            );
            assert_eq!(graph.findings.len(), 1, "{:?}", graph.findings);
            assert_eq!(graph.findings[0].code, "CAIRN_CONTRACT_NODE_SHAPE_DRIFT");
            assert_eq!(graph.findings[0].severity, FindingSeverity::Warning);
            assert_eq!(graph.findings[0].node.as_deref(), Some("app.api"));
        }
    }

    #[test]
    fn changed_fields_are_named_in_canonical_order() {
        let mut graph = empty_graph();
        check_contract_node_shape_drift(
            &mut graph,
            &contract_set(&["app.api"]),
            &baselines(vec![("app.api", baseline("Module", Some("app"), &[]))]),
            &snapshot(vec![(
                "app.api",
                fingerprint("Container", Some("app.other"), &[], &[]),
            )]),
        );
        assert_eq!(graph.findings[0].target.as_deref(), Some("kind, parent"));
    }

    #[test]
    fn message_resolves_from_copy_with_no_unsubstituted_slots() {
        let mut graph = empty_graph();
        check_contract_node_shape_drift(
            &mut graph,
            &contract_set(&["app.api"]),
            &baselines(vec![("app.api", baseline("Module", Some("app"), &[]))]),
            &snapshot(vec![(
                "app.api",
                fingerprint("Container", Some("app"), &[], &[]),
            )]),
        );
        let message = &graph.findings[0].message;
        assert!(!message.contains("{node}"), "{message}");
        assert!(!message.contains("{target}"), "{message}");
        assert!(message.contains("app.api"), "{message}");
        assert_eq!(
            *message,
            crate::copy::lookup("findings.codes.CAIRN_CONTRACT_NODE_SHAPE_DRIFT.body")
                .replace("{node}", "app.api")
                .replace("{target}", "kind"),
        );
    }

    #[test]
    fn entry_for_an_undeclared_node_is_inert() {
        let mut graph = empty_graph();
        check_contract_node_shape_drift(
            &mut graph,
            &contract_set(&["app.api"]),
            &baselines(vec![("app.removed", baseline("Module", None, &[]))]),
            &snapshot(vec![(
                "app.api",
                fingerprint("Module", Some("app"), &[], &[]),
            )]),
        );
        assert!(
            graph.findings.is_empty(),
            "a baseline the blueprint no longer declares must not be compared"
        );
    }

    #[test]
    fn eligibility_follows_the_pointer_owner_not_contract_frontmatter() {
        // A contract declared by `app.api` whose frontmatter names another
        // node: the owner stays reviewable, the named node does not become so.
        let mut contracts = contract_set(&["app.api"]);
        for contract in contracts.contracts.values_mut() {
            contract.node = "app.other".to_owned();
        }
        let drifted = fingerprint("Container", Some("app"), &[], &[]);
        let current = snapshot(vec![("app.api", drifted.clone()), ("app.other", drifted)]);

        let mut graph = empty_graph();
        check_contract_node_shape_drift(
            &mut graph,
            &contracts,
            &baselines(vec![
                ("app.api", baseline("Module", Some("app"), &[])),
                ("app.other", baseline("Module", Some("app"), &[])),
            ]),
            &current,
        );
        assert_eq!(graph.findings.len(), 1, "{:?}", graph.findings);
        assert_eq!(graph.findings[0].node.as_deref(), Some("app.api"));
    }

    #[test]
    fn entry_for_a_node_without_a_loading_contract_is_inert() {
        let mut graph = empty_graph();
        check_contract_node_shape_drift(
            &mut graph,
            &contract_set(&[]),
            &baselines(vec![("app.api", baseline("Module", Some("app"), &[]))]),
            &snapshot(vec![(
                "app.api",
                fingerprint("Container", Some("app.other"), &[], &["app.core"]),
            )]),
        );
        assert!(
            graph.findings.is_empty(),
            "with no contract to review there is nothing to flag"
        );
    }
}
