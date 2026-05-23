//! Project scanner orchestration.

pub mod config;
pub mod outputs;
pub mod state;

use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

use crate::{
    artefacts::{
        contract::{ContractSet, load_contracts},
        registry::{ArtefactSet, DecisionStatus, load_artefacts},
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
        let mut aggregated_files = BTreeMap::<String, Vec<String>>::new();
        let mut aggregated_symbols = Vec::new();
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
                let owned_symbols = report.symbols.clone();
                for (owner, files) in report.claimed_files {
                    aggregated_files.entry(owner).or_default().extend(files);
                }
                aggregated_symbols.extend(report.symbols);
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

/// Deduplicate findings that share the same code, node, and message.
/// Preserves the first occurrence and order.
fn dedup_findings(findings: &mut Vec<crate::map::graph::Finding>) {
    let mut seen = std::collections::HashSet::new();
    findings.retain(|f| {
        let key = (f.code.clone(), f.node.clone(), f.message.clone());
        seen.insert(key)
    });
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
                    node: Some(node_id.clone()),
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
    let contracts = load_contracts(root, &ast);
    let artefacts = load_artefacts(root, &ast, contracts.clone());

    let targets = build_targets(&ast, &config);
    let (target_reports, reconcile_findings) =
        reconcile_targets(&targets, root, &config.ignores, &ast, &config);
    let mut target_hashes = state::TargetHashes::new();
    let mut all_findings = contracts.findings.clone();
    all_findings.extend(artefacts.findings.clone());
    all_findings.extend(reconcile_findings);
    dedup_findings(&mut all_findings);
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
    let mut graph = build_graph(&ast, root, &contracts, &claimed_files, all_findings);
    check_provenance_coverage(&mut graph, &artefacts);
    check_claims(&mut graph, &artefacts, root);
    check_gitignored_paths(&mut graph, &ast, &config.ignores);
    let current_snapshot = compute_blueprint_snapshot(&ast);
    let previous_snapshot =
        state::read_blueprint_snapshot(root).map_err(|error| error.to_string())?;
    check_blueprint_change_decisions(
        &mut graph,
        &artefacts,
        &current_snapshot,
        &previous_snapshot,
    );
    Ok(ScanResult {
        graph,
        artefacts,
        contracts,
        interface_hash,
        target_reports,
        target_hashes,
        blueprint_snapshot: current_snapshot,
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
    state::write_blueprint_snapshot(root, &result.blueprint_snapshot)
        .map_err(|error| error.to_string())?;
    outputs::write_map(root, &result.graph).map_err(|error| error.to_string())?;
    outputs::append_log(root, &result.graph).map_err(|error| error.to_string())?;
    Ok(result)
}

fn walk_blueprint_nodes(
    nodes: &[blueprint::Node],
    parent: Option<&str>,
    snapshot: &mut state::BlueprintSnapshot,
) {
    for node in nodes {
        let mut paths = node.paths.clone();
        paths.sort();
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
            },
        );
        walk_blueprint_nodes(&node.children, Some(&node.id), snapshot);
    }
}

fn compute_blueprint_snapshot(ast: &blueprint::Ast) -> state::BlueprintSnapshot {
    let mut snapshot = state::BlueprintSnapshot::new();
    walk_blueprint_nodes(&ast.nodes, None, &mut snapshot);
    snapshot
}

fn check_blueprint_change_decisions(
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
                DecisionStatus::Proposed | DecisionStatus::Accepted
            )
        })
        .flat_map(|d| d.nodes.iter().map(String::as_str))
        .collect();

    let mut emit = |node_id: &str| {
        if !covered.contains(node_id) {
            graph.findings.push(crate::map::graph::Finding {

                code: "CAIRN_BLUEPRINT_CHANGE_NO_DECISION".to_owned(),
                severity: crate::map::graph::FindingSeverity::Error,
                message: format!(
                    "blueprint shape changed for node `{node_id}` but no decision artefact covers it"
                ),
                node: Some(node_id.to_owned()),
                target: None,
                path: None,
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
}

fn check_provenance_coverage(graph: &mut Graph, artefacts: &ArtefactSet) {
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
            });
        }
    }
}

fn check_claims(graph: &mut Graph, artefacts: &ArtefactSet, root: &Path) {
    use std::collections::BTreeSet;
    for decision in &artefacts.decisions {
        let Some(claims) = &decision.claims else {
            continue;
        };
        if !matches!(claims.mode, crate::artefacts::ClaimsMode::Exhaustive) {
            continue;
        }
        let folder = root.join(&claims.folder);
        let actual: BTreeSet<String> = if let Ok(entries) = std::fs::read_dir(&folder) {
            entries
                .flatten()
                .filter(|e| e.file_type().is_ok_and(|ft| ft.is_file()))
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .collect()
        } else {
            graph.findings.push(crate::map::graph::Finding {
                code: "CA003".to_owned(),
                severity: crate::map::graph::FindingSeverity::Error,
                message: format!(
                    "decision `{}` claims exhaustive file list for folder `{}` which does not exist or is unreadable",
                    decision.id, claims.folder
                ),
                node: Some(decision.nodes.first().cloned().unwrap_or_default()),
                target: None,
                path: Some(decision.path.clone()),
            });
            continue;
        };
        let claimed: BTreeSet<String> = claims.items.iter().cloned().collect();
        let missing: Vec<_> = actual.difference(&claimed).cloned().collect();
        let extra: Vec<_> = claimed.difference(&actual).cloned().collect();
        if !missing.is_empty() || !extra.is_empty() {
            let mut parts = Vec::new();
            if !missing.is_empty() {
                parts.push(format!("missing from claim: {}", missing.join(", ")));
            }
            if !extra.is_empty() {
                parts.push(format!("extra in claim: {}", extra.join(", ")));
            }
            graph.findings.push(crate::map::graph::Finding {
                code: "CA003".to_owned(),
                severity: crate::map::graph::FindingSeverity::Error,
                message: format!(
                    "decision `{}` exhaustive file claim for `{}` does not match actual contents: {}",
                    decision.id,
                    claims.folder,
                    parts.join("; ")
                ),
                node: Some(decision.nodes.first().cloned().unwrap_or_default()),
                target: None,
                path: Some(decision.path.clone()),
            });
        }
    }
}

fn check_gitignored_paths(graph: &mut Graph, ast: &blueprint::Ast, ignores: &[String]) {
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
                });
            }
        }
    };
    visit_nodes(&ast.nodes, &mut emit_for);
}

fn visit_nodes<F: FnMut(&blueprint::Node)>(nodes: &[blueprint::Node], f: &mut F) {
    for node in nodes {
        f(node);
        visit_nodes(&node.children, f);
    }
}
