//! Project scanner orchestration.

pub(crate) mod cache;
pub(crate) mod checks;
pub mod config;
pub mod outputs;
pub mod state;
#[cfg(test)]
mod tests;

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
    pub symbols: std::sync::Arc<Vec<String>>,
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
    /// Blueprint node fingerprints for change detection.
    pub blueprint_snapshot: state::BlueprintSnapshot,
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
    let cache_key = cache::compute_reconciler_cache_key(ast, ignores, targets, root);
    if let Some(cached) = cache::try_load_reconciler_cache(root, &cache_key) {
        return cache::build_reports_from_cache(&cached, targets, config);
    }

    let mut reports = Vec::new();
    let mut all_findings = Vec::new();
    let rust_reconciler = RustCodeReconciler::new(ast);
    // Each language reconciler scans the entire project root; calling it
    // once per target produces duplicate orphaned-file findings. Cache
    // results by language so each reconciler runs exactly once globally.
    let mut reconciler_cache: BTreeMap<Language, crate::reconcile::ReconcileReport> =
        BTreeMap::new();
    for target in targets {
        let report = reconciler_cache.entry(target.language).or_insert_with(|| {
            let req = ReconcileRequest { root, ignores };
            match target.language {
                Language::Rust => rust_reconciler.reconcile(req).unwrap(),
                Language::TypeScript => {
                    let reconciler = crate::reconcile::typescript::TypeScriptReconciler::new(ast);
                    reconciler.reconcile(req).unwrap()
                }
                Language::Python => {
                    let reconciler = crate::reconcile::python::PythonReconciler::new(ast);
                    reconciler.reconcile(req).unwrap()
                }
                Language::Go => {
                    let reconciler = crate::reconcile::go::GoReconciler::new(ast);
                    reconciler.reconcile(req).unwrap()
                }
            }
        });
        let owned_files = report
            .claimed_files
            .get(&target.id.node_id)
            .cloned()
            .unwrap_or_default();
        let owned_symbols = report
            .node_symbols
            .get(&target.id.node_id)
            .cloned()
            .unwrap_or_default();
        let hash =
            crate::reconcile::fingerprint::InterfaceFingerprint::from_symbols(&owned_symbols).hash;
        reports.push(TargetReport {
            target_id: target.id.clone(),
            language: target.language,
            reconciler_id: target.reconciler_id.clone(),
            claimed_files: owned_files,
            symbols: std::sync::Arc::new(owned_symbols),
            hash,
        });
    }
    // Collect findings once per cached reconciler run, not per target.
    for report in reconciler_cache.values() {
        all_findings.extend(report.findings.clone());
    }
    let divergence_findings = detect_divergence(&reports, targets, config);
    all_findings.extend(divergence_findings);

    // Persist cache for next run — convert BTreeMap<Language, _> to BTreeMap<String, _>.
    let serializable: BTreeMap<String, crate::reconcile::ReconcileReport> = reconciler_cache
        .into_iter()
        .map(|(lang, report)| (lang.as_str().to_owned(), report))
        .collect();
    cache::write_reconciler_cache(root, &cache_key, &serializable);

    (reports, all_findings)
}

/// Deduplicate findings that share the same semantic identity:
/// `(code, node, path, target)`. Two findings with identical identity but
/// different messages are still the same issue — the message is display-only.
/// Preserves the first occurrence and order.
fn dedup_findings(findings: &mut Vec<crate::map::graph::Finding>) {
    let keep: Vec<bool> = {
        let mut seen = std::collections::HashSet::with_capacity(findings.len());
        findings
            .iter()
            .map(|f| {
                let key = (
                    f.code.as_str(),
                    f.node.as_deref(),
                    f.path.as_deref(),
                    f.target.as_deref(),
                );
                seen.insert(key)
            })
            .collect()
    };
    let mut write_idx = 0;
    for (read_idx, should_keep) in keep.into_iter().enumerate() {
        if should_keep {
            if write_idx != read_idx {
                findings.swap(write_idx, read_idx);
            }
            write_idx += 1;
        }
    }
    findings.truncate(write_idx);
}

fn detect_divergence(
    reports: &[TargetReport],
    targets: &[Target],
    config: &config::Config,
) -> Vec<crate::map::graph::Finding> {
    let mut findings = Vec::new();
    let mut by_node: BTreeMap<&str, Vec<&TargetReport>> = BTreeMap::new();
    for report in reports {
        by_node
            .entry(report.target_id.node_id.as_str())
            .or_default()
            .push(report);
    }
    for (node_id, node_reports) in by_node {
        if node_reports.len() < 2 {
            continue;
        }
        let mut by_role: BTreeMap<String, Vec<&PathBuf>> = BTreeMap::new();
        let mut hash_by_path: BTreeMap<PathBuf, String> = BTreeMap::new();
        for report in &node_reports {
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

            if let Some(asymmetry) = config.is_intentional_asymmetry(node_id, &role, &paths) {
                findings.push(crate::map::graph::Finding {
                    code: "CT002".to_owned(),
                    severity: crate::map::graph::FindingSeverity::Warning,
                    message: format!(
                        "Intentional asymmetry in `{}` for contract role `{}`: {}",
                        node_id, role, asymmetry.reason
                    ),
                    node: Some(node_id.to_owned()),
                    target: Some(role.clone()),
                    path: None,
                });
            } else {
                findings.push(crate::map::graph::Finding {

                    code: "CT001".to_owned(),
                    severity: crate::map::graph::FindingSeverity::Error,
                    message: format!(
                        "Interface contradiction: targets for `{node_id}` with contract role `{role}` have divergent interfaces"
                    ),
                    node: Some(node_id.to_owned()),
                    target: Some(role.clone()),
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
    let mut contracts = load_contracts(root, &ast);
    let mut artefacts = load_artefacts(root, &ast, contracts.clone());
    let targets = build_targets(&ast, &config);
    let (target_reports, reconcile_findings) =
        reconcile_targets(&targets, root, &config.ignores, &ast, &config);
    let mut target_hashes = state::TargetHashes::new();
    let mut all_findings = Vec::with_capacity(
        contracts.findings.len() + artefacts.findings.len() + reconcile_findings.len(),
    );
    all_findings.extend(std::mem::take(&mut contracts.findings));
    all_findings.extend(std::mem::take(&mut artefacts.findings));
    all_findings.extend(reconcile_findings);
    dedup_findings(&mut all_findings);
    for report in &target_reports {
        let mut key = String::with_capacity(
            report.target_id.node_id.len() + 1 + report.target_id.path.as_os_str().len(),
        );
        key.push_str(&report.target_id.node_id);
        key.push(':');
        key.push_str(&report.target_id.path.to_string_lossy());
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
        files.sort_unstable();
        files.dedup();
    }
    let mut graph = build_graph(&ast, root, &contracts, &mut claimed_files, all_findings);
    checks::check_provenance_coverage(&mut graph, &artefacts);
    checks::check_claims(&mut graph, &artefacts, root);
    checks::check_gitignored_paths(&mut graph, &ast, &config.ignores);
    checks::check_orphan_beads(&mut graph, root);
    let current_snapshot = compute_blueprint_snapshot(&ast);
    let previous_snapshot =
        state::read_blueprint_snapshot(root).map_err(|error| error.to_string())?;
    checks::check_blueprint_change_decisions(
        &mut graph,
        &artefacts,
        &current_snapshot,
        &previous_snapshot,
    );
    Ok(ScanResult {
        graph,
        target_hashes,
        interface_hash,
        blueprint_snapshot: current_snapshot,
        target_reports,
        contracts,
        artefacts,
    })
}
///
/// # Errors
///
/// Returns an error string when project loading succeeds but generated output
/// persistence fails, or when project loading itself fails.
#[allow(clippy::missing_panics_doc)] // Reason: Mutex is never poisoned inside thread::scope
pub fn scan(root: &Path, blueprint_path: &Path) -> Result<ScanResult, String> {
    let result = load_project(root, blueprint_path)?;
    let errs = std::sync::Mutex::new(Vec::new());
    std::thread::scope(|s| {
        s.spawn(|| {
            if let Err(e) = state::write_interface_hash(root, &result.target_hashes) {
                errs.lock().unwrap().push(format!("{e}"));
            }
        });
        s.spawn(|| {
            if let Err(e) = state::write_blueprint_snapshot(root, &result.blueprint_snapshot) {
                errs.lock().unwrap().push(format!("{e}"));
            }
        });
        s.spawn(|| {
            if let Err(e) = outputs::write_map(root, &result.graph) {
                errs.lock().unwrap().push(format!("{e}"));
            }
        });
        s.spawn(|| {
            if let Err(e) = outputs::append_log(root, &result.graph) {
                errs.lock().unwrap().push(format!("{e}"));
            }
        });
    });
    let errs = errs.into_inner().unwrap();
    if let Some(first) = errs.into_iter().next() {
        return Err(first);
    }
    Ok(result)
}

fn walk_blueprint_nodes(
    nodes: &[blueprint::Node],
    parent: Option<&str>,
    edges: &BTreeMap<String, Vec<String>>,
    snapshot: &mut state::BlueprintSnapshot,
) {
    for node in nodes {
        let mut paths = node.paths.clone();
        paths.sort_unstable();
        snapshot.nodes.insert(
            node.id.clone(),
            state::NodeFingerprint {
                kind: match node.kind {
                    blueprint::NodeKind::System => "System",
                    blueprint::NodeKind::Container => "Container",
                    blueprint::NodeKind::Module => "Module",
                    blueprint::NodeKind::Actor => "Actor",
                }
                .to_owned(),
                parent: parent.map(String::from),
                paths,
                edges: edges.get(&node.id).cloned().unwrap_or_default(),
            },
        );
        walk_blueprint_nodes(&node.children, Some(&node.id), edges, snapshot);
    }
}

fn compute_blueprint_snapshot(ast: &blueprint::Ast) -> state::BlueprintSnapshot {
    let mut edges: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for edge in &ast.edges {
        edges
            .entry(edge.from.clone())
            .or_default()
            .push(edge.to.clone());
    }
    for targets in edges.values_mut() {
        targets.sort_unstable();
        targets.dedup();
    }
    let mut snapshot = state::BlueprintSnapshot::new();
    walk_blueprint_nodes(&ast.nodes, None, &edges, &mut snapshot);
    snapshot
}
