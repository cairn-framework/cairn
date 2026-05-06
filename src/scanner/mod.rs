//! Project scanner orchestration.

pub mod config;
pub mod outputs;
pub mod state;

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use crate::{
    artefacts::{
        contract::{ContractSet, load_contracts},
        registry::{ArtefactSet, load_artefacts},
    },
    blueprint,
    map::{Graph, build_graph},
    reconcile::{
        ReconcileRequest, Reconciler, ReconcilerId,
        code::RustCodeReconciler,
        target::{Language, Target, TargetId},
    },
};

/// Report for a single reconciled target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetReport {
    /// Target identifier with node ID and path.
    pub target_id: TargetId,
    /// Language detected or configured for this target.
    pub language: Language,
    /// Reconciler identifier.
    pub reconciler_id: ReconcilerId,
    /// Files claimed by this target.
    pub claimed_files: Vec<String>,
    /// Public symbols exported by this target.
    pub symbols: Vec<String>,
    /// Interface hash for this target.
    pub hash: String,
}

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
    /// Per-target reconciliation reports.
    pub target_reports: Vec<TargetReport>,
    /// Interface hashes keyed by target ID.
    pub target_hashes: state::TargetHashes,
}

fn build_targets(ast: &blueprint::Ast, config: &config::Config) -> Vec<Target> {
    let mut targets = Vec::new();
    for node in &ast.nodes {
        collect_targets(node, &mut targets);
    }
    for target_config in &config.targets {
        if let Some(target) = targets
            .iter_mut()
            .find(|t| t.id.node_id == target_config.node_id)
        {
            if let Some(lang) = Language::from_language_str(&target_config.language) {
                target.language = lang;
                target.reconciler_id = lang.reconciler_id();
            }
            target
                .contract_role
                .clone_from(&target_config.contract_role);
        }
    }
    targets
}

fn collect_targets(node: &blueprint::Node, targets: &mut Vec<Target>) {
    let is_internal = !node.children.is_empty();
    if !is_internal || node.owns_files {
        for path_str in &node.paths {
            let path = std::path::PathBuf::from(path_str.trim_start_matches("./"));
            let language = Language::from_extension(&path).unwrap_or(Language::Rust);
            let target = Target::new(node.id.clone(), path, language);
            targets.push(target);
        }
    }
    for child in &node.children {
        collect_targets(child, targets);
    }
}

fn reconcile_targets(
    targets: &[Target],
    root: &Path,
    ignores: &[String],
    ast: &blueprint::Ast,
    config: &config::Config,
) -> (Vec<TargetReport>, Vec<crate::map::graph::Finding>) {
    let mut reports = Vec::new();
    let mut all_findings = Vec::new();
    let rust_reconciler = RustCodeReconciler::new(ast);
    let mut by_node: BTreeMap<String, Vec<&Target>> = BTreeMap::new();
    for target in targets {
        by_node
            .entry(target.id.node_id.clone())
            .or_default()
            .push(target);
    }
    for (node_id, node_targets) in by_node {
        let mut node_reports = Vec::new();
        for target in node_targets {
            let request = ReconcileRequest { root, ignores };
            let result = match target.language {
                Language::Rust => rust_reconciler.reconcile(request),
                Language::TypeScript => {
                    let reconciler = crate::reconcile::typescript::TypeScriptReconciler::new(ast);
                    reconciler.reconcile(request)
                }
                Language::Python => {
                    let reconciler = crate::reconcile::python::PythonReconciler::new(ast);
                    reconciler.reconcile(request)
                }
                Language::Go => {
                    let reconciler = crate::reconcile::go::GoReconciler::new(ast);
                    reconciler.reconcile(request)
                }
            };
            if let Ok(report) = result {
                let hash = report.fingerprint.hash.clone();
                let owned_files = report
                    .claimed_files
                    .get(&node_id)
                    .cloned()
                    .unwrap_or_default();
                let owned_symbols = report.symbols;
                all_findings.extend(report.findings);
                node_reports.push(TargetReport {
                    target_id: target.id.clone(),
                    language: target.language,
                    reconciler_id: target.reconciler_id.clone(),
                    claimed_files: owned_files,
                    symbols: owned_symbols,
                    hash,
                });
            }
        }
        reports.extend(node_reports);
    }
    let divergence_findings = detect_divergence(&reports, targets, config);
    all_findings.extend(divergence_findings);
    (reports, all_findings)
}

fn detect_divergence(
    reports: &[TargetReport],
    targets: &[Target],
    config: &config::Config,
) -> Vec<crate::map::graph::Finding> {
    let mut findings = Vec::new();
    let mut by_node: BTreeMap<String, Vec<&TargetReport>> = BTreeMap::new();
    for report in reports {
        by_node
            .entry(report.target_id.node_id.clone())
            .or_default()
            .push(report);
    }
    for (node_id, node_reports) in by_node {
        if node_reports.len() < 2 {
            continue;
        }
        let mut by_role: BTreeMap<String, Vec<&PathBuf>> = BTreeMap::new();
        let mut hash_by_path: BTreeMap<PathBuf, String> = BTreeMap::new();
        for report in &node_reports.clone() {
            let target = targets.iter().find(|t| {
                t.id.node_id == report.target_id.node_id && t.id.path == report.target_id.path
            });
            if let Some(t) = target {
                by_role
                    .entry(t.contract_role.clone())
                    .or_default()
                    .push(&t.id.path);
                hash_by_path.insert(report.target_id.path.clone(), report.hash.clone());
            }
        }
        for (role, paths) in by_role {
            if paths.len() < 2 {
                continue;
            }
            let hashes: Vec<_> = paths
                .iter()
                .filter_map(|p| hash_by_path.get(*p).cloned())
                .collect();
            let all_same = hashes.windows(2).all(|w| w[0] == w[1]);
            if all_same {
                continue;
            }

            if let Some(asymmetry) = config.is_intentional_asymmetry(&node_id, &role, &paths) {
                findings.push(crate::map::graph::Finding {
                    code: "CT002".to_owned(),
                    severity: crate::map::graph::FindingSeverity::Warning,
                    message: format!(
                        "Intentional asymmetry in `{}` for contract role `{}`: {}",
                        node_id, role, asymmetry.reason
                    ),
                    node: Some(node_id.clone()),
                    path: None,
                });
            } else {
                findings.push(crate::map::graph::Finding {
                    code: "CT001".to_owned(),
                    severity: crate::map::graph::FindingSeverity::Error,
                    message: format!(
                        "Interface contradiction: targets for `{node_id}` with contract role `{role}` have divergent interfaces"
                    ),
                    node: Some(node_id.clone()),
                    path: None,
                });
            }
        }
    }
    findings
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
    let targets = build_targets(&ast, &config);
    let (target_reports, reconcile_findings) =
        reconcile_targets(&targets, root, &config.ignores, &ast, &config);
    let mut target_hashes = state::TargetHashes::new();
    let mut all_findings = contracts.findings.clone();
    all_findings.extend(artefacts.findings.clone());
    all_findings.extend(reconcile_findings);
    for report in &target_reports {
        let key = format!(
            "{}:{}",
            report.target_id.node_id,
            report.target_id.path.display()
        );
        target_hashes.insert(key, report.hash.clone());
    }
    let interface_hash = target_reports
        .first()
        .map(|r| r.hash.clone())
        .unwrap_or_default();
    let mut claimed_files = BTreeMap::<String, Vec<String>>::new();
    for report in &target_reports {
        claimed_files
            .entry(report.target_id.node_id.clone())
            .or_default()
            .extend(report.claimed_files.clone());
    }
    for files in claimed_files.values_mut() {
        files.sort();
        files.dedup();
    }
    let graph = build_graph(&ast, root, &contracts, &claimed_files, all_findings);
    Ok(ScanResult {
        graph,
        artefacts,
        contracts,
        interface_hash,
        target_reports,
        target_hashes,
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
    state::write_interface_hash(root, &result.target_hashes).map_err(|error| error.to_string())?;
    outputs::write_map(root, &result.graph).map_err(|error| error.to_string())?;
    outputs::append_log(root, &result.graph).map_err(|error| error.to_string())?;
    Ok(result)
}
