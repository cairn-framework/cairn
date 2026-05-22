//! Brownfield onboard: groups orphan findings into directory clusters
//! and classifies each as an ignore candidate or blueprint node candidate.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use crate::map::graph::Finding;

const ORPHAN_CODE: &str = "CAIRN_RECONCILE_ORPHANED_FILE";

const IGNORE_PATTERNS: &[&str] = &[
    "target",
    "dist",
    "build",
    "node_modules",
    "vendor",
    "__pycache__",
    ".cache",
    "generated",
    "backup",
    "tmp",
    "out",
    ".next",
    ".nuxt",
    "coverage",
    ".turbo",
];

/// Grouped analysis of orphaned files.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OnboardReport {
    /// Orphan clusters grouped by directory.
    pub clusters: Vec<OrphanCluster>,
    /// Total number of orphaned files across all clusters.
    pub total_orphaned_files: usize,
}

/// A group of orphaned files sharing a parent directory.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrphanCluster {
    /// Parent directory path.
    pub directory: String,
    /// Orphaned file paths.
    pub files: Vec<String>,
    /// Suggested action for this cluster.
    pub suggestion: ClusterSuggestion,
}

/// Suggested action for an orphan cluster.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClusterSuggestion {
    /// Directory matches a generated/cache/vendor pattern; suggest adding to ignore list.
    Ignore(String),
    /// Directory contains meaningful source; suggest adding a blueprint node.
    Node {
        /// Path-derived candidate ID.
        id: String,
    },
}

/// Groups orphan findings by directory and classifies each cluster.
#[must_use]
pub fn analyze(findings: &[Finding]) -> OnboardReport {
    let mut seen = std::collections::BTreeSet::new();
    let orphan_paths: Vec<&str> = findings
        .iter()
        .filter(|f| f.code == ORPHAN_CODE)
        .filter_map(|f| f.path.as_deref())
        .filter(|p| seen.insert(*p))
        .collect();

    let total_orphaned_files = orphan_paths.len();
    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for path in orphan_paths {
        let dir = parent_dir(path);
        groups.entry(dir).or_default().push(path.to_owned());
    }

    let clusters = groups
        .into_iter()
        .map(|(directory, files)| {
            let suggestion = classify(&directory);
            OrphanCluster {
                directory,
                files,
                suggestion,
            }
        })
        .collect();

    OnboardReport {
        clusters,
        total_orphaned_files,
    }
}

fn parent_dir(path: &str) -> String {
    match path.rfind('/') {
        Some(pos) => path[..pos].to_owned(),
        None => ".".to_owned(),
    }
}

fn classify(directory: &str) -> ClusterSuggestion {
    let segments: Vec<&str> = directory.split('/').collect();
    for (i, segment) in segments.iter().enumerate() {
        let lower = segment.to_ascii_lowercase();
        if IGNORE_PATTERNS.iter().any(|p| *p == lower) {
            let glob = segments[..=i].join("/");
            return ClusterSuggestion::Ignore(glob);
        }
    }
    let id = crate::brownfield::heuristics::path_derived_id(directory);
    ClusterSuggestion::Node { id }
}

/// Renders the report as human-readable text.
#[must_use]
pub fn render_human(report: &OnboardReport) -> String {
    if report.clusters.is_empty() {
        return "No orphaned files found. Blueprint coverage is complete.\n".to_owned();
    }

    let mut out = format!(
        "Orphan clusters ({} directories, {} files):\n",
        report.clusters.len(),
        report.total_orphaned_files,
    );

    let ignore_clusters: Vec<&OrphanCluster> = report
        .clusters
        .iter()
        .filter(|c| matches!(c.suggestion, ClusterSuggestion::Ignore(_)))
        .collect();
    let node_clusters: Vec<&OrphanCluster> = report
        .clusters
        .iter()
        .filter(|c| matches!(c.suggestion, ClusterSuggestion::Node { .. }))
        .collect();

    if !ignore_clusters.is_empty() {
        out.push_str("\n--- Suggested ignores ---\n");
        out.push_str("Add to cairn.config.yaml under `ignore:`\n");
        for cluster in &ignore_clusters {
            if let ClusterSuggestion::Ignore(glob) = &cluster.suggestion {
                let _ = write!(
                    out,
                    "\n  - \"{}\"  ({} files in {})\n",
                    glob,
                    cluster.files.len(),
                    cluster.directory,
                );
            }
        }
    }

    if !node_clusters.is_empty() {
        out.push_str("\n--- Suggested blueprint nodes ---\n");
        out.push_str("Add to cairn.blueprint inside your System block:\n");
        for cluster in &node_clusters {
            if let ClusterSuggestion::Node { id } = &cluster.suggestion {
                let name = cluster
                    .directory
                    .rsplit('/')
                    .next()
                    .unwrap_or(&cluster.directory);
                let cap_name = capitalize(name);
                let _ = write!(
                    out,
                    "\n    Module {} \"...\" id \"{}\" {{\n        path \"./{}\"\n    }}\n",
                    cap_name, id, cluster.directory,
                );
            }
        }
    }

    out
}

/// Renders the report as a JSON envelope.
#[must_use]
pub fn render_json(report: &OnboardReport) -> String {
    let clusters: Vec<serde_json::Value> = report
        .clusters
        .iter()
        .map(|c| {
            let (suggestion_type, suggestion_value) = match &c.suggestion {
                ClusterSuggestion::Ignore(glob) => ("ignore", glob.clone()),
                ClusterSuggestion::Node { id } => ("node", id.clone()),
            };
            serde_json::json!({
                "directory": c.directory,
                "file_count": c.files.len(),
                "files": c.files,
                "suggestion": suggestion_type,
                "suggestion_value": suggestion_value,
            })
        })
        .collect();

    let ignore_suggestions: Vec<&str> = report
        .clusters
        .iter()
        .filter_map(|c| match &c.suggestion {
            ClusterSuggestion::Ignore(glob) => Some(glob.as_str()),
            ClusterSuggestion::Node { .. } => None,
        })
        .collect();

    let node_suggestions: Vec<serde_json::Value> = report
        .clusters
        .iter()
        .filter_map(|c| match &c.suggestion {
            ClusterSuggestion::Node { id } => Some(serde_json::json!({
                "id": id,
                "directory": c.directory,
                "file_count": c.files.len(),
            })),
            ClusterSuggestion::Ignore(_) => None,
        })
        .collect();

    let envelope = serde_json::json!({
        "clusters": clusters,
        "ignore_suggestions": ignore_suggestions,
        "node_suggestions": node_suggestions,
        "total_orphaned_files": report.total_orphaned_files,
    });

    format!("{envelope}\n")
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let upper: String = first.to_uppercase().collect();
            upper + chars.as_str()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::graph::FindingSeverity;

    fn orphan_finding(path: &str) -> Finding {
        Finding {
            code: ORPHAN_CODE.to_owned(),
            severity: FindingSeverity::Info,
            message: format!("Rust file `{path}` is not owned by any eligible node"),
            node: None,
            target: None,
            path: Some(path.to_owned()),
        }
    }

    #[test]
    fn groups_by_parent_directory() {
        let findings = vec![
            orphan_finding("src/auth/login.rs"),
            orphan_finding("src/auth/session.rs"),
            orphan_finding("src/db/pool.rs"),
        ];
        let report = analyze(&findings);
        assert_eq!(report.total_orphaned_files, 3);
        assert_eq!(report.clusters.len(), 2);
        assert_eq!(report.clusters[0].directory, "src/auth");
        assert_eq!(report.clusters[0].files.len(), 2);
        assert_eq!(report.clusters[1].directory, "src/db");
    }

    #[test]
    fn classifies_generated_dirs_as_ignore() {
        let findings = vec![
            orphan_finding("generated/cache/output.rs"),
            orphan_finding("target/debug/build.rs"),
        ];
        let report = analyze(&findings);
        assert!(matches!(
            &report.clusters[0].suggestion,
            ClusterSuggestion::Ignore(g) if g == "generated"
        ));
        assert!(matches!(
            &report.clusters[1].suggestion,
            ClusterSuggestion::Ignore(g) if g == "target"
        ));
    }

    #[test]
    fn classifies_source_dirs_as_node() {
        let findings = vec![orphan_finding("src/auth/login.rs")];
        let report = analyze(&findings);
        assert!(matches!(
            &report.clusters[0].suggestion,
            ClusterSuggestion::Node { id } if id == "src.auth"
        ));
    }

    #[test]
    fn empty_findings_produce_empty_report() {
        let report = analyze(&[]);
        assert!(report.clusters.is_empty());
        assert_eq!(report.total_orphaned_files, 0);
    }

    #[test]
    fn nested_ignore_pattern_matches_ancestor() {
        let findings = vec![orphan_finding("dist/assets/chunk.rs")];
        let report = analyze(&findings);
        assert!(matches!(
            &report.clusters[0].suggestion,
            ClusterSuggestion::Ignore(g) if g == "dist"
        ));
    }

    #[test]
    fn json_output_parses_as_valid_json() {
        let findings = vec![
            orphan_finding("src/auth/login.rs"),
            orphan_finding("generated/out.rs"),
        ];
        let report = analyze(&findings);
        let json_str = render_json(&report);
        let parsed: serde_json::Value = serde_json::from_str(&json_str).expect("valid JSON");
        assert_eq!(parsed["total_orphaned_files"], 2);
        assert_eq!(parsed["clusters"].as_array().unwrap().len(), 2);
        assert_eq!(parsed["ignore_suggestions"].as_array().unwrap().len(), 1);
        assert_eq!(parsed["node_suggestions"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn human_output_shows_both_sections() {
        let findings = vec![
            orphan_finding("src/auth/login.rs"),
            orphan_finding("generated/out.rs"),
        ];
        let report = analyze(&findings);
        let text = render_human(&report);
        assert!(text.contains("Suggested ignores"));
        assert!(text.contains("Suggested blueprint nodes"));
        assert!(text.contains("Module Auth"));
    }

    #[test]
    fn root_level_files_cluster_under_dot() {
        let findings = vec![orphan_finding("stray.rs")];
        let report = analyze(&findings);
        assert_eq!(report.clusters[0].directory, ".");
    }
}
