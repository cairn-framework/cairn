//! Tests for the remediation-plan handlers.

use super::*;
use std::collections::BTreeMap;

#[test]
fn emitted_remediation_description_uses_copy_entry() {
    let mut scan_result = scanner::ScanResult {
        graph: crate::map::Graph {
            nodes: BTreeMap::new(),
            names: BTreeMap::new(),
            outbound: BTreeMap::new(),
            inbound: BTreeMap::new(),
            findings: Vec::new(),
        },
        artefacts: crate::artefacts::registry::ArtefactSet::default(),
        contracts: crate::artefacts::contract::ContractSet::default(),
        interface_hash: String::new(),
        target_reports: Vec::new(),
        target_hashes: scanner::state::TargetHashes::default(),
        blueprint_snapshot: scanner::state::BlueprintSnapshot::default(),
    };
    let actions = remediate_actions_raw(Path::new("."), Path::new("meta/changes"), &scan_result);
    let description = actions
        .iter()
        .find(|action| action["action"] == "none")
        .and_then(|action| action["description"].as_str());

    assert_eq!(
        description,
        Some(crate::copy::lookup("remediate.actions.none"))
    );
    assert_ne!(description, Some("remediate.actions.none"));

    for action in [
        "fix_blueprint",
        "init_from_code",
        "refine",
        "fix_gitignored_path",
        "summarise",
        "fix_contracts",
        "add_decision",
        "fix_decisions",
        "consolidate_decisions",
        "fix_todos",
        "fix_sources",
        "fix_research",
        "rename_artefacts",
        "fix_order",
        "split_module",
        "none",
    ] {
        let key = format!("remediate.actions.{action}");
        assert_ne!(crate::copy::lookup(&key), key);
    }

    scan_result.graph.findings.push(crate::map::Finding {
        code: "CAIRN_ORDER_CYCLE".to_owned(),
        severity: crate::map::FindingSeverity::Error,
        message: "cycle".to_owned(),
        node: None,
        target: None,
        path: None,
        deferred_by: None,
        parked_by: None,
    });
    let actions = remediate_actions_raw(Path::new("."), Path::new("meta/changes"), &scan_result);
    let description = actions
        .iter()
        .find(|action| action["action"] == "fix_order")
        .and_then(|action| action["description"].as_str());
    assert_eq!(
        description,
        Some(crate::copy::lookup("remediate.actions.fix_order"))
    );
}
