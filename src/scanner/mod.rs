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
    // Each language reconciler scans the entire project root; calling it
    // once per target produces duplicate orphaned-file findings. Cache
    // results by language so each reconciler runs exactly once globally.
    let mut reconciler_cache: BTreeMap<Language, crate::reconcile::ReconcileReport> =
        BTreeMap::new();
    for (node_id, node_targets) in by_node {
        let mut node_reports = Vec::new();
        for target in node_targets {
            let report = reconciler_cache.entry(target.language).or_insert_with(|| {
                let req = ReconcileRequest { root, ignores };
                match target.language {
                    Language::Rust => rust_reconciler.reconcile(req).unwrap(),
                    Language::TypeScript => {
                        let reconciler =
                            crate::reconcile::typescript::TypeScriptReconciler::new(ast);
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
            let hash = report.fingerprint.hash.clone();
            let owned_files = report
                .claimed_files
                .get(&node_id)
                .cloned()
                .unwrap_or_default();
            let owned_symbols = report.symbols.clone();
            node_reports.push(TargetReport {
                target_id: target.id.clone(),
                language: target.language,
                reconciler_id: target.reconciler_id.clone(),
                claimed_files: owned_files,
                symbols: owned_symbols,
                hash,
            });
        }
        reports.extend(node_reports);
    }
    // Collect findings once per cached reconciler run, not per target or node.
    for (_, report) in reconciler_cache {
        all_findings.extend(report.findings);
    }
    let divergence_findings = detect_divergence(&reports, targets, config);
    all_findings.extend(divergence_findings);
    (reports, all_findings)
}

/// Deduplicate findings that share the same semantic identity:
/// `(code, node, path, target)`. Two findings with identical identity but
/// different messages are still the same issue — the message is display-only.
/// Preserves the first occurrence and order.
fn dedup_findings(findings: &mut Vec<crate::map::graph::Finding>) {
    let keep: Vec<bool> = {
        let mut seen = std::collections::HashSet::new();
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
    let mut contracts = load_contracts(root, &ast);
    let mut artefacts = load_artefacts(root, &ast, contracts.clone());
    let targets = build_targets(&ast, &config);
    let (target_reports, reconcile_findings) =
        reconcile_targets(&targets, root, &config.ignores, &ast, &config);
    let mut target_hashes = state::TargetHashes::new();
    let mut all_findings = std::mem::take(&mut contracts.findings);
    all_findings.extend(std::mem::take(&mut artefacts.findings));
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
                DecisionStatus::Proposed | DecisionStatus::Accepted | DecisionStatus::Superseded
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
#[cfg(test)]
mod tests {
    use crate::{
        artefacts::registry::types::Decision,
        artefacts::registry::{ArtefactSet, DecisionStatus},
        blueprint::{NodeKind, ast::Span},
        map::graph::{Finding, FindingSeverity, Graph, NodeRecord, NodeState},
        scanner::{
            config::Config,
            state::{BlueprintSnapshot, NodeFingerprint},
        },
    };
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use super::{
        check_blueprint_change_decisions, check_provenance_coverage, dedup_findings,
        detect_divergence,
    };

    fn finding(
        code: &str,
        node: Option<&str>,
        path: Option<&str>,
        target: Option<&str>,
        message: &str,
    ) -> Finding {
        Finding {
            code: code.to_owned(),
            severity: FindingSeverity::Warning,
            message: message.to_owned(),
            node: node.map(str::to_owned),
            path: path.map(str::to_owned),
            target: target.map(str::to_owned),
        }
    }

    #[test]
    fn test_dedup_drops_exact_duplicate() {
        let mut findings = vec![
            finding("CC001", Some("app.api"), None, None, "msg"),
            finding("CC001", Some("app.api"), None, None, "msg"),
        ];
        dedup_findings(&mut findings);
        assert_eq!(findings.len(), 1, "exact duplicate must be dropped");
    }

    #[test]
    fn test_dedup_keeps_different_targets() {
        // Same code, node, path, message — but different dependency target.
        // Previously these were incorrectly collapsed because the key was
        // (code, node, message) and did not include `target`.
        let mut findings = vec![
            finding("CC002", Some("app.api"), None, Some("db"), "missing edge"),
            finding(
                "CC002",
                Some("app.api"),
                None,
                Some("cache"),
                "missing edge",
            ),
        ];
        dedup_findings(&mut findings);
        assert_eq!(
            findings.len(),
            2,
            "findings for different targets must both be kept"
        );
    }

    #[test]
    fn test_dedup_keeps_different_paths() {
        let mut findings = vec![
            finding(
                "CAIRN_RECONCILE_ORPHANED_FILE",
                Some("app.api"),
                Some("src/a.rs"),
                None,
                "msg",
            ),
            finding(
                "CAIRN_RECONCILE_ORPHANED_FILE",
                Some("app.api"),
                Some("src/b.rs"),
                None,
                "msg",
            ),
        ];
        dedup_findings(&mut findings);
        assert_eq!(
            findings.len(),
            2,
            "findings for different file paths must both be kept"
        );
    }

    #[test]
    fn test_dedup_merges_same_issue_different_message() {
        // Same issue (code + node + path + target) with a different message text
        // — the second is redundant; the first occurrence is preserved.
        let mut findings = vec![
            finding(
                "CC001",
                Some("app.api"),
                Some("src/lib.rs"),
                None,
                "first message",
            ),
            finding(
                "CC001",
                Some("app.api"),
                Some("src/lib.rs"),
                None,
                "second message",
            ),
        ];
        dedup_findings(&mut findings);
        assert_eq!(
            findings.len(),
            1,
            "same issue with different message texts must be deduplicated"
        );
        assert_eq!(
            findings[0].message, "first message",
            "first occurrence must be kept"
        );
    }

    #[test]
    fn test_dedup_preserves_order_and_first_occurrence() {
        let mut findings = vec![
            finding("CC001", Some("app.api"), None, None, "alpha"),
            finding("CC002", Some("app.db"), None, None, "beta"),
            finding("CC001", Some("app.api"), None, None, "alpha"),
        ];
        dedup_findings(&mut findings);
        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0].code, "CC001");
        assert_eq!(findings[1].code, "CC002");
    }

    #[test]
    fn test_dedup_empty_is_noop() {
        let mut findings: Vec<Finding> = Vec::new();
        dedup_findings(&mut findings);
        assert!(findings.is_empty());
    }

    // ── helpers ───────────────────────────────────────────────────────────────

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
            span: Span::point("test", 1, 1),
        }
    }

    fn graph_with_leaf(id: &str) -> Graph {
        let mut g = empty_graph();
        g.nodes.insert(id.to_owned(), bare_node(id));
        g
    }

    fn graph_with_parent(parent_id: &str, child_id: &str) -> Graph {
        let mut g = empty_graph();
        let mut parent = bare_node(parent_id);
        parent.children = vec![child_id.to_owned()];
        g.nodes.insert(parent_id.to_owned(), parent);
        g.nodes.insert(child_id.to_owned(), bare_node(child_id));
        g
    }

    fn empty_graph() -> Graph {
        Graph {
            nodes: BTreeMap::new(),
            names: BTreeMap::new(),
            outbound: BTreeMap::new(),
            inbound: BTreeMap::new(),
            findings: Vec::new(),
        }
    }

    fn snap(items: &[(&str, &str)]) -> BlueprintSnapshot {
        let mut s = BlueprintSnapshot::new();
        for (id, kind) in items {
            s.nodes.insert(
                id.to_string(),
                NodeFingerprint {
                    kind: kind.to_string(),
                    parent: None,
                    paths: Vec::new(),
                },
            );
        }
        s
    }

    fn decision(id: &str, nodes: &[&str], status: DecisionStatus) -> Decision {
        Decision {
            id: id.to_owned(),
            path: "meta/decisions/test.md".to_owned(),
            nodes: nodes.iter().map(ToString::to_string).collect(),
            status,
            date: "2024-01-01".to_owned(),
            revisited: None,
            revisit_triggers: Vec::new(),
            informed_by: Vec::new(),
            supersedes: Vec::new(),
            refines: Vec::new(),
            related: Vec::new(),
            orphaned: false,
            orphan_reason: None,
            claims: None,
            body: String::new(),
        }
    }

    fn artefacts_with(decisions: Vec<Decision>) -> ArtefactSet {
        ArtefactSet {
            decisions,
            ..Default::default()
        }
    }

    fn report_and_target(
        node_id: &str,
        path: &str,
        role: &str,
        hash: &str,
    ) -> (super::TargetReport, crate::reconcile::target::Target) {
        use crate::reconcile::{
            ReconcilerId,
            target::{Language, Target, TargetId},
        };
        let path_buf = PathBuf::from(path);
        let report = super::TargetReport {
            target_id: TargetId {
                node_id: node_id.to_owned(),
                path: path_buf.clone(),
            },
            language: Language::Rust,
            reconciler_id: ReconcilerId("rust-code".to_owned()),
            claimed_files: Vec::new(),
            symbols: Vec::new(),
            hash: hash.to_owned(),
        };
        let target = Target::new(node_id.to_owned(), path_buf, Language::Rust)
            .with_contract_role(role.to_owned());
        (report, target)
    }

    // ── check_blueprint_change_decisions ──────────────────────────────────────

    #[test]
    fn test_blueprint_change_no_finding_when_previous_is_empty() {
        let mut g = empty_graph();
        let current = snap(&[("app.new", "Module")]);
        let previous = BlueprintSnapshot::new(); // empty
        let artefacts = artefacts_with(vec![]);
        check_blueprint_change_decisions(&mut g, &artefacts, &current, &previous);
        assert!(g.findings.is_empty(), "empty previous must skip all checks");
    }

    #[test]
    fn test_blueprint_change_no_finding_when_no_decisions() {
        let mut g = empty_graph();
        let previous = snap(&[("app.existing", "Module")]);
        let current = snap(&[("app.existing", "Module"), ("app.new", "Module")]);
        let artefacts = artefacts_with(vec![]); // no decisions
        check_blueprint_change_decisions(&mut g, &artefacts, &current, &previous);
        assert!(g.findings.is_empty(), "no decisions must skip all checks");
    }

    #[test]
    fn test_blueprint_change_added_uncovered_node_emits_finding() {
        let mut g = empty_graph();
        let previous = snap(&[("app.existing", "Module")]);
        let current = snap(&[("app.existing", "Module"), ("app.new", "Module")]);
        // decision covers only "app.existing", not "app.new"
        let artefacts = artefacts_with(vec![decision(
            "d1",
            &["app.existing"],
            DecisionStatus::Accepted,
        )]);
        check_blueprint_change_decisions(&mut g, &artefacts, &current, &previous);
        assert_eq!(g.findings.len(), 1);
        assert_eq!(g.findings[0].code, "CAIRN_BLUEPRINT_CHANGE_NO_DECISION");
        assert_eq!(g.findings[0].node.as_deref(), Some("app.new"));
    }

    #[test]
    fn test_blueprint_change_covered_added_node_no_finding() {
        let mut g = empty_graph();
        let previous = snap(&[("app.existing", "Module")]);
        let current = snap(&[("app.existing", "Module"), ("app.new", "Module")]);
        let artefacts =
            artefacts_with(vec![decision("d1", &["app.new"], DecisionStatus::Accepted)]);
        check_blueprint_change_decisions(&mut g, &artefacts, &current, &previous);
        assert!(g.findings.is_empty());
    }

    #[test]
    fn test_blueprint_change_removed_node_uncovered_emits_finding() {
        let mut g = empty_graph();
        let previous = snap(&[("app.existing", "Module"), ("app.removed", "Module")]);
        let current = snap(&[("app.existing", "Module")]);
        let artefacts = artefacts_with(vec![decision(
            "d1",
            &["app.existing"],
            DecisionStatus::Accepted,
        )]);
        check_blueprint_change_decisions(&mut g, &artefacts, &current, &previous);
        assert_eq!(g.findings.len(), 1);
        assert_eq!(g.findings[0].node.as_deref(), Some("app.removed"));
    }

    #[test]
    fn test_blueprint_change_path_only_no_finding() {
        // Path-only changes are explicitly not gated (comment in implementation).
        let mut g = empty_graph();
        let mut previous = BlueprintSnapshot::new();
        previous.nodes.insert(
            "app.api".to_owned(),
            NodeFingerprint {
                kind: "Module".to_owned(),
                parent: None,
                paths: vec!["src/old".to_owned()],
            },
        );
        let mut current = BlueprintSnapshot::new();
        current.nodes.insert(
            "app.api".to_owned(),
            NodeFingerprint {
                kind: "Module".to_owned(),
                parent: None,
                paths: vec!["src/new".to_owned()], // different path, same kind/parent
            },
        );
        let artefacts = artefacts_with(vec![decision("d1", &["x"], DecisionStatus::Accepted)]);
        check_blueprint_change_decisions(&mut g, &artefacts, &current, &previous);
        assert!(g.findings.is_empty(), "path-only change must not be gated");
    }

    #[test]
    fn test_blueprint_change_superseded_decision_covers_added_node() {
        // The function message says "no decision artefact covers it" — a
        // Superseded decision IS still a decision artefact.  The filter
        // `Proposed | Accepted` wrongly excludes Superseded, causing the
        // gate to fire even though the node was legitimately decided.
        let mut g = empty_graph();
        let previous = snap(&[("app.existing", "Module")]);
        let current = snap(&[("app.existing", "Module"), ("app.new", "Module")]);
        let artefacts = artefacts_with(vec![decision(
            "d1",
            &["app.new"],
            DecisionStatus::Superseded, // the only covering decision is Superseded
        )]);
        check_blueprint_change_decisions(&mut g, &artefacts, &current, &previous);
        assert!(
            g.findings.is_empty(),
            "superseded decision must count as coverage; got: {:?}",
            g.findings
        );
    }

    // ── check_provenance_coverage ─────────────────────────────────────────────

    #[test]
    fn test_provenance_coverage_no_decisions_no_findings() {
        let mut g = graph_with_leaf("app.api");
        let artefacts = artefacts_with(vec![]);
        check_provenance_coverage(&mut g, &artefacts);
        assert!(
            g.findings.is_empty(),
            "no decisions → early return, no warnings"
        );
    }

    #[test]
    fn test_provenance_coverage_uncovered_leaf_emits_warning() {
        let mut g = graph_with_leaf("app.api");
        let artefacts = artefacts_with(vec![decision(
            "d1",
            &["app.other"],
            DecisionStatus::Accepted,
        )]);
        check_provenance_coverage(&mut g, &artefacts);
        assert_eq!(g.findings.len(), 1);
        assert_eq!(g.findings[0].code, "CAIRN_PROVENANCE_NO_DECISION");
        assert_eq!(g.findings[0].severity, FindingSeverity::Warning);
        assert_eq!(g.findings[0].node.as_deref(), Some("app.api"));
    }

    #[test]
    fn test_provenance_coverage_covered_leaf_no_warning() {
        let mut g = graph_with_leaf("app.api");
        let artefacts =
            artefacts_with(vec![decision("d1", &["app.api"], DecisionStatus::Accepted)]);
        check_provenance_coverage(&mut g, &artefacts);
        assert!(g.findings.is_empty());
    }

    #[test]
    fn test_provenance_coverage_parent_node_exempt_from_warning() {
        // Only leaf nodes (children.is_empty()) are checked for provenance.
        let mut g = graph_with_parent("app.system", "app.api");
        let artefacts =
            artefacts_with(vec![decision("d1", &["app.api"], DecisionStatus::Accepted)]);
        check_provenance_coverage(&mut g, &artefacts);
        // app.system has children → exempt. app.api is covered → no warning.
        assert!(g.findings.is_empty());
    }

    // ── detect_divergence ─────────────────────────────────────────────────────

    #[test]
    fn test_divergence_single_report_no_finding() {
        let (r, t) = report_and_target("app.api", "src/api.rs", "public_api", "abc");
        let findings = detect_divergence(&[r], &[t], &Config::default());
        assert!(findings.is_empty(), "one report cannot diverge");
    }

    #[test]
    fn test_divergence_two_reports_same_hash_no_finding() {
        let (r1, t1) = report_and_target("app.api", "src/v1.rs", "public_api", "abc");
        let (r2, t2) = report_and_target("app.api", "src/v2.rs", "public_api", "abc");
        let findings = detect_divergence(&[r1, r2], &[t1, t2], &Config::default());
        assert!(findings.is_empty(), "identical hashes must not diverge");
    }

    #[test]
    fn test_divergence_two_reports_different_hash_emits_ct001() {
        let (r1, t1) = report_and_target("app.api", "src/v1.rs", "public_api", "abc");
        let (r2, t2) = report_and_target("app.api", "src/v2.rs", "public_api", "xyz");
        let findings = detect_divergence(&[r1, r2], &[t1, t2], &Config::default());
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].code, "CT001");
        assert_eq!(findings[0].severity, FindingSeverity::Error);
        assert_eq!(findings[0].node.as_deref(), Some("app.api"));
    }

    #[test]
    fn test_divergence_different_roles_no_finding() {
        // Each role has only one target → len < 2 per role → no divergence.
        let (r1, t1) = report_and_target("app.api", "src/public.rs", "public_api", "abc");
        let (r2, t2) = report_and_target("app.api", "src/internal.rs", "internal", "xyz");
        let findings = detect_divergence(&[r1, r2], &[t1, t2], &Config::default());
        assert!(findings.is_empty(), "different roles must not be compared");
    }
}
