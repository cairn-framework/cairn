//! Contract Markdown artefact loading.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use crate::{
    blueprint::{Ast, Node},
    map::graph::{Finding, FindingSeverity},
};

use super::frontmatter;

/// Parsed contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Contract {
    /// Contract file path as declared.
    pub path: String,
    /// Node ID that declared the contract pointer.
    pub declared_by: String,
    /// Referenced node ID.
    pub node: String,
    /// Markdown body.
    pub body: String,
}

/// Contract loader result.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ContractSet {
    /// Contracts keyed by declared path.
    pub contracts: BTreeMap<String, Contract>,
    /// Loading findings.
    pub findings: Vec<Finding>,
}

/// Loads all Phase 1 contract pointers declared by the AST.
#[must_use]
pub fn load_contracts(root: &Path, ast: &Ast) -> ContractSet {
    let ids = collect_ids(ast);
    let mut set = ContractSet::default();
    for (node_id, pointer) in collect_contract_pointers(ast) {
        let path = root.join(&pointer);
        match fs::read_to_string(&path) {
            Ok(source) => {
                let parsed = frontmatter::parse(&source);
                let Some(contract_node) = parsed.values.get("node").cloned() else {
                    set.findings.push(Finding {
                        code: "CAIRN_CONTRACT_MISSING_NODE".to_owned(),
                        severity: FindingSeverity::Error,
                        message: format!("contract `{pointer}` lacks node frontmatter"),
                        node: Some(node_id.clone()),
                        target: None,
                        path: Some(pointer.clone()),
                    });
                    continue;
                };
                if !ids.contains(&contract_node) {
                    set.findings.push(Finding {
                        code: "CAIRN_CONTRACT_UNKNOWN_NODE".to_owned(),
                        severity: FindingSeverity::Error,
                        message: format!(
                            "contract `{pointer}` references unknown node `{contract_node}`"
                        ),
                        node: Some(contract_node.clone()),
                        target: None,
                        path: Some(pointer.clone()),
                    });
                }
                if contract_node != node_id {
                    set.findings.push(Finding {

                        code: "CAIRN_CONTRACT_WRONG_NODE".to_owned(),
                        severity: FindingSeverity::Error,
                        message: format!(
                            "contract `{pointer}` declared by `{node_id}` references `{contract_node}`"
                        ),
                        node: Some(node_id.clone()),
                        target: None,
                        path: Some(pointer.clone()),
                    });
                }
                set.contracts.insert(
                    pointer.clone(),
                    Contract {
                        path: pointer,
                        declared_by: node_id,
                        node: contract_node,
                        body: parsed.body,
                    },
                );
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => set.findings.push(Finding {
                code: "CAIRN_CONTRACT_READ_FAILED".to_owned(),
                severity: FindingSeverity::Error,
                message: format!("failed to read contract `{pointer}`: {error}"),
                node: Some(node_id),
                target: None,
                path: Some(pointer),
            }),
        }
    }
    set
}

fn collect_ids(ast: &Ast) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    for node in &ast.nodes {
        collect_node_id(node, &mut ids);
    }
    ids
}

fn collect_node_id(node: &Node, ids: &mut BTreeSet<String>) {
    ids.insert(node.id.clone());
    for child in &node.children {
        collect_node_id(child, ids);
    }
}

fn collect_contract_pointers(ast: &Ast) -> Vec<(String, String)> {
    let mut pointers = Vec::new();
    for node in &ast.nodes {
        collect_node_contracts(node, &mut pointers);
    }
    pointers
}

fn collect_node_contracts(node: &Node, pointers: &mut Vec<(String, String)>) {
    for contract in &node.contracts {
        pointers.push((node.id.clone(), contract.clone()));
    }
    for child in &node.children {
        collect_node_contracts(child, pointers);
    }
}
