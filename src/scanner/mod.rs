//! Project scanner orchestration.

pub mod config;
pub mod outputs;
pub mod state;

use std::path::Path;

use crate::{
    artefacts::{
        contract::{ContractSet, load_contracts},
        registry::{ArtefactSet, load_artefacts},
    },
    blueprint,
    map::{Graph, build_graph},
    reconcile::{ReconcileRequest, Reconciler, code::RustCodeReconciler},
};

/// Result of a scan or graph load.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScanResult {
    /// Built graph.
    pub graph: Graph,
    /// Loaded typed artefacts.
    pub artefacts: ArtefactSet,
    /// Loaded contracts.
    pub contracts: ContractSet,
    /// Interface hash.
    pub interface_hash: String,
}

/// Loads project state without writing generated outputs.
///
/// # Errors
///
/// Returns an error string when config loading, blueprint parsing, or reconciliation
/// fails.
pub fn load_project(root: &Path, blueprint_path: &Path) -> Result<ScanResult, String> {
    let config = config::load(root).map_err(|error| error.message)?;
    let ast = blueprint::parse_file(blueprint_path).map_err(|error| error.to_string())?;
    let contracts = load_contracts(root, &ast);
    let artefacts = load_artefacts(root, &ast, contracts.clone());
    let reconciler = RustCodeReconciler::new(&ast);
    let report = reconciler
        .reconcile(ReconcileRequest {
            root,
            ignores: &config.ignores,
        })
        .map_err(|error| error.to_string())?;
    let mut findings = contracts.findings.clone();
    findings.extend(artefacts.findings.clone());
    findings.extend(report.findings.clone());
    let mut graph = build_graph(&ast, root, &contracts, &report.claimed_files, findings);
    let semantic_findings = crate::reconcile::semantic_findings(&graph, &report);
    graph.findings.extend(semantic_findings);
    Ok(ScanResult {
        graph,
        artefacts,
        contracts,
        interface_hash: report.fingerprint.hash,
    })
}

/// Runs scanner and writes generated outputs.
///
/// # Errors
///
/// Returns an error string when project loading succeeds but generated output
/// persistence fails, or when project loading itself fails.
pub fn scan(root: &Path, blueprint_path: &Path) -> Result<ScanResult, String> {
    let result = load_project(root, blueprint_path)?;
    state::write_interface_hash(root, &result.interface_hash).map_err(|error| error.to_string())?;
    outputs::write_map(root, &result.graph).map_err(|error| error.to_string())?;
    outputs::append_log(root, &result.graph).map_err(|error| error.to_string())?;
    Ok(result)
}
