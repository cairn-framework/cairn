//! Reconciler result cache: key computation, source-root walking, cache load/write, and report reconstruction.

use std::{
    collections::BTreeMap,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};

use super::{Target, TargetReport, blueprint, config, detect_divergence};

/// Cache version for reconciler cache format.
const RECONCILER_CACHE_VERSION: u32 = 3;

/// Persistent cache entry for reconciler results.
#[derive(serde::Serialize, serde::Deserialize)]
struct ReconcilerCacheEntry {
    /// Schema version.
    version: u32,
    /// Cache key (hex hash of inputs).
    key: String,
    /// Reconcile reports keyed by `Language::as_str()`.
    reports: BTreeMap<String, crate::reconcile::ReconcileReport>,
}

/// Source file extensions tracked for cache invalidation.
const CACHE_EXTENSIONS: &[&str] = &["rs", "ts", "tsx", "py", "go"];

/// Computes a cache key that captures every input to reconciliation.
pub(crate) fn compute_reconciler_cache_key(
    ast: &blueprint::Ast,
    ignores: &[String],
    targets: &[Target],
    root: &Path,
) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();

    // Cache version.
    RECONCILER_CACHE_VERSION.hash(&mut hasher);

    // Blueprint AST — Hash derive is deterministic for these types within a version.
    // Avoids the ~3KB heap allocation of format!("{ast:?}").
    ast.hash(&mut hasher);

    // Ignore patterns.
    ignores.hash(&mut hasher);

    // Target identities (order-sensitive, matching build_targets output).
    for target in targets {
        target.language.as_str().hash(&mut hasher);
        target.id.node_id.hash(&mut hasher);
        target.id.path.to_string_lossy().hash(&mut hasher);
        target.reconciler_id.0.hash(&mut hasher);
    }

    // Walk only blueprint-declared source roots instead of the entire project.
    // Most projects have all source files within declared node paths; walking from
    // root would visit many directories (docs/, meta/, scripts/, etc.) that contain
    // no source files, wasting ~3ms per scan. Blueprint roots are derived from the
    // AST, so changes to the blueprint already invalidate the key via the AST hash.
    // Trade-off: source files added outside all blueprint paths won't invalidate the
    // cache immediately; they appear only on the next full reconcile (cache miss).
    let mut file_entries: Vec<(String, u64)> = Vec::with_capacity(256);
    for scan_root in blueprint_source_roots(ast, root) {
        collect_source_file_mtimes(root, &scan_root, ignores, &mut file_entries);
    }
    file_entries.sort_unstable();
    file_entries.dedup();
    file_entries.hash(&mut hasher);

    format!("{:016x}", hasher.finish())
}

/// Recursively collects `(relative_path, mtime_secs)` for source files.
fn collect_source_file_mtimes(
    root: &Path,
    dir: &Path,
    ignores: &[String],
    entries: &mut Vec<(String, u64)>,
) {
    let Ok(read_dir) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in read_dir.flatten() {
        let path = entry.path();
        let rel = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        if config::is_ignored(&rel, ignores) {
            continue;
        }
        let Ok(ft) = entry.file_type() else { continue };
        if ft.is_dir() {
            collect_source_file_mtimes(root, &path, ignores, entries);
        } else if ft.is_file() {
            let has_ext = path
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| CACHE_EXTENSIONS.contains(&e));
            if has_ext {
                let mtime_secs = entry
                    .metadata()
                    .ok()
                    .and_then(|m| m.modified().ok())
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .map_or(0, |d| d.as_secs());
                entries.push((rel, mtime_secs));
            }
        }
    }
}

/// Returns the minimal set of directories to walk for cache key computation.
///
/// Extracts declared path roots from the blueprint AST. For each declared path:
/// - Directory paths are used directly.
/// - File paths contribute their parent directory.
///
/// Redundant entries are pruned: if directory A is a sub-path of directory B,
/// A is removed because a walk of B already covers A.
fn blueprint_source_roots(ast: &blueprint::Ast, root: &Path) -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = Vec::new();
    collect_node_source_roots(&ast.nodes, root, &mut dirs);
    // Prune directories that are already covered by an ancestor in the list.
    let all = dirs;
    let mut minimal: Vec<PathBuf> = Vec::new();
    'outer: for candidate in &all {
        for other in &all {
            if other == candidate {
                continue;
            }
            if candidate.starts_with(other) {
                // candidate is a sub-directory of other; skip it.
                continue 'outer;
            }
        }
        if !minimal.contains(candidate) {
            minimal.push(candidate.clone());
        }
    }
    minimal
}

fn collect_node_source_roots(nodes: &[blueprint::Node], root: &Path, dirs: &mut Vec<PathBuf>) {
    for node in nodes {
        for path_str in &node.paths {
            let rel = path_str.trim_start_matches("./").trim_start_matches('/');
            // Use path extension as a heuristic instead of stat()-ing every path:
            // paths with a source-file extension (e.g. "src/main.rs") are files —
            // take their parent; paths without an extension are directories.
            // Blueprint convention: declared file paths always have extensions.
            let is_file_path = Path::new(rel).extension().is_some();
            let abs = root.join(rel);
            if is_file_path {
                if let Some(parent) = abs.parent() {
                    dirs.push(parent.to_path_buf());
                }
            } else {
                dirs.push(abs);
            }
        }
        collect_node_source_roots(&node.children, root, dirs);
    }
}

/// Attempts to load a cached reconciler result matching `key`.
pub(crate) fn try_load_reconciler_cache(
    root: &Path,
    key: &str,
) -> Option<BTreeMap<String, crate::reconcile::ReconcileReport>> {
    let path = root.join(".cairn/state/reconciler-cache.json");
    let content = std::fs::read_to_string(path).ok()?;
    let entry: ReconcilerCacheEntry = serde_json::from_str(&content).ok()?;
    if entry.version != RECONCILER_CACHE_VERSION || entry.key != key {
        return None;
    }
    Some(entry.reports)
}

/// Writes the reconciler cache to disk. Silently ignores errors.
pub(crate) fn write_reconciler_cache(
    root: &Path,
    key: &str,
    reports: &BTreeMap<String, crate::reconcile::ReconcileReport>,
) {
    let entry = ReconcilerCacheEntry {
        version: RECONCILER_CACHE_VERSION,
        key: key.to_owned(),
        reports: reports.clone(),
    };
    let Ok(json) = serde_json::to_string(&entry) else {
        return;
    };
    let dir = root.join(".cairn/state");
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let path = dir.join("reconciler-cache.json");
    // Skip write when content is identical (same pattern as state.rs).
    if let Ok(existing) = std::fs::read_to_string(&path)
        && existing == json
    {
        return;
    }
    let _ = std::fs::write(path, json);
}

/// Rebuilds the `(Vec<TargetReport>, Vec<Finding>)` from cached reconciler reports.
pub(crate) fn build_reports_from_cache(
    cached: &BTreeMap<String, crate::reconcile::ReconcileReport>,
    targets: &[Target],
    config: &config::Config,
) -> (Vec<TargetReport>, Vec<crate::map::graph::Finding>) {
    let mut reports = Vec::new();
    let mut all_findings = Vec::new();

    for target in targets {
        let lang_key = target.language.as_str();
        if let Some(report) = cached.get(lang_key) {
            let hash = report.fingerprint.hash.clone();
            let owned_files = report
                .claimed_files
                .get(&target.id.node_id)
                .cloned()
                .unwrap_or_default();
            let owned_symbols = report.symbols.clone();
            reports.push(TargetReport {
                target_id: target.id.clone(),
                language: target.language,
                reconciler_id: target.reconciler_id.clone(),
                claimed_files: owned_files,
                symbols: owned_symbols,
                hash,
            });
        }
    }
    // Collect findings once per cached language, not per target.
    for report in cached.values() {
        all_findings.extend(report.findings.clone());
    }
    let divergence_findings = detect_divergence(&reports, targets, config);
    all_findings.extend(divergence_findings);
    (reports, all_findings)
}
