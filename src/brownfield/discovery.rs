//! Repository-wide candidate discovery for cold-start extraction.
//!
//! Walks the filesystem from a project root and identifies directories
//! with enough source files to be plausible module candidates. Works
//! without an existing blueprint. Edges are derived only from imports
//! observed in the candidates' source files, in import direction; an
//! edge discovery cannot observe in the code is not proposed.

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use crate::error::CairnError;

use super::heuristics::path_derived_id;
use super::import_edges;

/// Supported source file extensions for candidate discovery.
const SOURCE_EXTS: &[&str] = &["rs", "ts", "js", "py", "go"];

/// Minimum source-file count for a directory to become a candidate.
const MIN_FILES: usize = 3;

/// Maximum depth below root for candidate directories.
const MAX_DEPTH: usize = 4;

/// A discovered module candidate from filesystem traversal.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DiscoveredCandidate {
    /// Proposed node ID (path-derived).
    pub id: String,
    /// Proposed human-readable name.
    pub name: String,
    /// Proposed description.
    pub description: String,
    /// Source directory path (relative to project root).
    pub path: String,
    /// Detected tags.
    pub tags: Vec<String>,
    /// Confidence score (higher is better).
    pub confidence: f64,
    /// Evidence paths that contributed to this candidate.
    pub evidence: Vec<String>,
    /// Observed outbound edges to other candidate IDs.
    pub edges: Vec<DiscoveredEdge>,
}

/// An observed edge between discovered candidates.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DiscoveredEdge {
    /// Target candidate ID.
    pub target: String,
    /// Edge description.
    pub description: String,
    /// Edge confidence.
    pub confidence: f64,
}

/// Result of a brownfield discovery pass.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Extraction {
    /// Discovered candidates.
    pub candidates: Vec<DiscoveredCandidate>,
    /// Schema version.
    pub schema_version: u32,
}

impl Default for Extraction {
    fn default() -> Self {
        Self {
            candidates: Vec::new(),
            schema_version: 1,
        }
    }
}

/// Discover candidates in a repository root.
///
/// Walks the filesystem up to `MAX_DEPTH`, collecting directories that
/// contain at least `MIN_FILES` source files. Each qualifying directory
/// becomes a `DiscoveredCandidate`. Edges are derived only from observed
/// imports, in import direction: proposing all-pairs "sibling"
/// dependencies guaranteed a `CAIRN_ORDER_CYCLE` on first scan for any
/// repo with two or more co-located modules.
///
/// # Errors
///
/// Returns `CairnError::ChangeDiscovery` when directory traversal fails.
pub fn discover(root: &Path) -> Result<Extraction, CairnError> {
    let mut candidates = Vec::new();
    let mut dir_counts: BTreeMap<PathBuf, Vec<PathBuf>> = BTreeMap::new();
    collect_source_files(root, root, &mut dir_counts, 0)?;

    for (dir, files) in &dir_counts {
        if files.len() >= MIN_FILES {
            let rel = dir.strip_prefix(root).unwrap_or(dir);
            let rel_str = rel.to_string_lossy().to_string();
            if rel_str.is_empty() {
                continue;
            }
            let id = node_id_from_path(&rel_str);
            let name = name_from_path(&rel_str);
            let confidence = compute_confidence(files.len());
            let mut evidence: Vec<String> = files
                .iter()
                .map(|p| {
                    p.strip_prefix(root)
                        .unwrap_or(p)
                        .to_string_lossy()
                        .to_string()
                })
                .collect();
            evidence.sort();
            candidates.push(DiscoveredCandidate {
                id,
                name,
                description: format!("Discovered module at {rel_str}"),
                path: rel_str,
                tags: Vec::new(),
                confidence,
                evidence,
                edges: Vec::new(),
            });
        }
    }
    import_edges::derive_import_edges(root, &mut candidates);

    Ok(Extraction {
        candidates,
        schema_version: 1,
    })
}

// Reason: `root` is used by the caller for strip_prefix at lines 93/104
// but clippy's flow-insensitive analysis flags it as recursion-only.
#[allow(clippy::only_used_in_recursion)]
fn collect_source_files(
    root: &Path,
    current: &Path,
    dir_counts: &mut BTreeMap<PathBuf, Vec<PathBuf>>,
    depth: usize,
) -> Result<(), CairnError> {
    if depth > MAX_DEPTH {
        return Ok(());
    }
    let mut entries = Vec::new();
    for entry in std::fs::read_dir(current).map_err(|e| CairnError::ChangeDiscovery {
        path: current.to_string_lossy().to_string(),
        detail: e.to_string(),
    })? {
        let entry = entry.map_err(|e| CairnError::ChangeDiscovery {
            path: current.to_string_lossy().to_string(),
            detail: e.to_string(),
        })?;
        entries.push(entry.path());
    }

    let mut files_here = Vec::new();
    for path in &entries {
        if path.is_file() && is_source_file(path) {
            files_here.push(path.clone());
        }
    }
    if !files_here.is_empty() {
        dir_counts.insert(current.to_path_buf(), files_here);
    }

    for path in &entries {
        if path.is_dir() && !is_ignored_dir(path) && !is_symlink(path) {
            collect_source_files(root, path, dir_counts, depth + 1)?;
        }
    }
    Ok(())
}

fn is_source_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| SOURCE_EXTS.contains(&ext))
}

fn is_symlink(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok_and(|m| m.file_type().is_symlink())
}

fn is_ignored_dir(path: &Path) -> bool {
    let name = path.file_name().map_or("", |n| n.to_str().unwrap_or(""));
    matches!(
        name,
        "target" | "node_modules" | ".git" | ".cairn" | "openspec" | "meta" | "dist" | "build"
    )
}

fn node_id_from_path(path: &str) -> String {
    path_derived_id(path)
}

fn name_from_path(path: &str) -> String {
    path.rsplit(&['/', '\\'])
        .next()
        .unwrap_or(path)
        .to_owned()
        .replace(['-', '_'], " ")
}

fn compute_confidence(file_count: usize) -> f64 {
    if file_count >= 5 {
        1.0
    } else if file_count >= 3 {
        0.7
    } else {
        0.3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_id_replaces_separators() {
        assert_eq!(node_id_from_path("src/auth/login"), "src.auth.login");
    }

    #[test]
    fn name_extracts_last_segment() {
        assert_eq!(name_from_path("src/user_auth"), "user auth");
    }

    #[test]
    fn confidence_tiers() {
        assert!((compute_confidence(5) - 1.0).abs() < f64::EPSILON);
        assert!((compute_confidence(3) - 0.7).abs() < f64::EPSILON);
        assert!((compute_confidence(2) - 0.3).abs() < f64::EPSILON);
    }

    #[test]
    fn ignored_dirs_matched() {
        assert!(is_ignored_dir(Path::new("/repo/target")));
        assert!(is_ignored_dir(Path::new("/repo/node_modules")));
        assert!(!is_ignored_dir(Path::new("/repo/src")));
    }
    // ── derive_import_edges ─────────────────────────────────────────────────

    fn write(root: &Path, rel: &str, content: &str) {
        let path = root.join(rel);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }

    /// Mirror of docs/assets/demo/brownfield-setup.sh: api imports auth,
    /// auth imports db. Discovery must emit exactly those directed edges.
    #[test]
    fn import_edges_follow_observed_direction() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write(
            root,
            "src/main.rs",
            "mod api;\nmod auth;\nmod db;\nfn main() { api::serve(); }\n",
        );
        write(
            root,
            "src/db/mod.rs",
            "pub mod pool;\npub mod schema;\n\npub fn connect() {}\n",
        );
        write(root, "src/db/pool.rs", "pub fn get_pool() {}\n");
        write(root, "src/db/schema.rs", "pub fn migrate() {}\n");
        write(
            root,
            "src/auth/mod.rs",
            "pub mod tokens;\npub mod session;\nuse crate::db;\n\npub fn login() { db::connect(); }\n",
        );
        write(root, "src/auth/tokens.rs", "pub fn issue() {}\n");
        write(root, "src/auth/session.rs", "pub fn refresh() {}\n");
        write(
            root,
            "src/api/mod.rs",
            "pub mod routes;\npub mod handlers;\nuse crate::auth;\n\npub fn serve() { auth::login(); }\n",
        );
        write(root, "src/api/routes.rs", "pub fn register() {}\n");
        write(root, "src/api/handlers.rs", "pub fn users() {}\n");

        let extraction = discover(root).unwrap();
        let mut edges: Vec<(String, String)> = extraction
            .candidates
            .iter()
            .flat_map(|c| {
                c.edges
                    .iter()
                    .map(|e| (c.id.clone(), e.target.clone()))
                    .collect::<Vec<_>>()
            })
            .collect();
        edges.sort();
        assert_eq!(
            edges,
            vec![
                ("src.api".to_owned(), "src.auth".to_owned()),
                ("src.auth".to_owned(), "src.db".to_owned()),
            ],
            "expected exactly api->auth and auth->db, no fabricated or reverse edges"
        );
        let auth = extraction
            .candidates
            .iter()
            .find(|c| c.id == "src.auth")
            .unwrap();
        assert!((auth.edges[0].confidence - 0.6).abs() < f64::EPSILON);
    }

    #[test]
    fn relative_ts_imports_resolve_to_candidate_paths() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        for f in ["index.ts", "routes.ts", "handlers.ts"] {
            let body = if f == "index.ts" {
                "import { login } from \"../auth/session\";\nexport function serve() {}\n"
            } else {
                "export function x() {}\n"
            };
            write(root, &format!("src/api/{f}"), body);
        }
        for f in ["session.ts", "tokens.ts", "guard.ts"] {
            write(root, &format!("src/auth/{f}"), "export function y() {}\n");
        }
        let extraction = discover(root).unwrap();
        let api = extraction
            .candidates
            .iter()
            .find(|c| c.id == "src.api")
            .unwrap();
        assert_eq!(api.edges.len(), 1);
        assert_eq!(api.edges[0].target, "src.auth");
    }

    #[test]
    fn ambiguous_segment_names_are_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        // Two candidates both named "db": segment match must not guess.
        for parent in ["a", "b"] {
            for f in ["x.rs", "y.rs", "z.rs"] {
                write(root, &format!("{parent}/db/{f}"), "pub fn f() {}\n");
            }
        }
        for f in ["m.rs", "n.rs", "o.rs"] {
            write(
                root,
                &format!("core/{f}"),
                "use crate::db;\npub fn g() {}\n",
            );
        }
        let extraction = discover(root).unwrap();
        let core = extraction
            .candidates
            .iter()
            .find(|c| c.id == "core")
            .unwrap();
        assert!(core.edges.is_empty(), "ambiguous name must produce no edge");
    }

    // ── is_source_file ────────────────────────────────────────────────────────

    #[test]
    fn test_is_source_file_accepts_known_extensions() {
        for ext in &["rs", "ts", "js", "py", "go"] {
            let path = PathBuf::from(format!("foo.{ext}"));
            assert!(
                is_source_file(&path),
                ".{ext} must be recognised as a source file"
            );
        }
    }

    #[test]
    fn test_is_source_file_rejects_unknown_extensions() {
        for ext in &["md", "json", "toml", "txt", "tsx"] {
            let path = PathBuf::from(format!("foo.{ext}"));
            assert!(
                !is_source_file(&path),
                ".{ext} must not be recognised as a source file"
            );
        }
    }

    #[test]
    fn test_is_source_file_rejects_no_extension() {
        assert!(!is_source_file(Path::new("Makefile")));
        assert!(!is_source_file(Path::new("Dockerfile")));
    }

    // ── name_from_path ────────────────────────────────────────────────────────

    #[test]
    fn test_name_from_path_converts_hyphens_to_spaces() {
        assert_eq!(name_from_path("src/user-auth"), "user auth");
    }

    #[test]
    fn test_name_from_path_single_segment_no_separator() {
        assert_eq!(name_from_path("auth"), "auth");
    }

    // ── discover() ────────────────────────────────────────────────────────────

    #[test]
    fn test_discover_creates_candidate_for_directory_with_enough_source_files() {
        let dir = tempfile::tempdir().expect("temp dir");
        let src = dir.path().join("src");
        std::fs::create_dir(&src).unwrap();
        for name in &["a.rs", "b.rs", "c.rs"] {
            std::fs::write(src.join(name), "// test").unwrap();
        }
        let result = discover(dir.path()).expect("discover");
        assert_eq!(result.candidates.len(), 1);
        assert_eq!(result.candidates[0].id, "src");
    }

    #[test]
    fn test_discover_skips_directory_with_too_few_source_files() {
        let dir = tempfile::tempdir().expect("temp dir");
        let src = dir.path().join("src");
        std::fs::create_dir(&src).unwrap();
        // Only 2 files — below MIN_FILES threshold of 3.
        for name in &["a.rs", "b.rs"] {
            std::fs::write(src.join(name), "// test").unwrap();
        }
        let result = discover(dir.path()).expect("discover");
        assert!(
            result.candidates.is_empty(),
            "2 source files must not produce a candidate"
        );
    }

    #[test]
    fn test_discover_ignores_target_directory() {
        let dir = tempfile::tempdir().expect("temp dir");
        let target = dir.path().join("target");
        std::fs::create_dir(&target).unwrap();
        for name in &["a.rs", "b.rs", "c.rs", "d.rs"] {
            std::fs::write(target.join(name), "// test").unwrap();
        }
        let result = discover(dir.path()).expect("discover");
        assert!(
            result.candidates.is_empty(),
            "target/ must be excluded even with enough source files"
        );
    }

    #[test]
    fn test_discover_sibling_directories_get_no_fabricated_edges() {
        let dir = tempfile::tempdir().expect("temp dir");
        let web = dir.path().join("web");
        let api = dir.path().join("api");
        std::fs::create_dir(&web).unwrap();
        std::fs::create_dir(&api).unwrap();
        for name in &["a.rs", "b.rs", "c.rs"] {
            std::fs::write(web.join(name), "").unwrap();
            std::fs::write(api.join(name), "").unwrap();
        }
        let result = discover(dir.path()).expect("discover");
        assert_eq!(result.candidates.len(), 2);
        // Discovery must not invent dependency edges between co-located
        // modules: all-pairs sibling edges made every first scan report a
        // dependency cycle.
        for cand in &result.candidates {
            assert!(
                cand.edges.is_empty(),
                "no fabricated edges for {}: {:?}",
                cand.id,
                cand.edges
            );
        }
    }

    #[test]
    fn test_discover_non_source_files_do_not_count_toward_threshold() {
        let dir = tempfile::tempdir().expect("temp dir");
        let src = dir.path().join("docs");
        std::fs::create_dir(&src).unwrap();
        // 5 markdown files — none are source files.
        for name in &["a.md", "b.md", "c.md", "d.md", "e.md"] {
            std::fs::write(src.join(name), "# doc").unwrap();
        }
        let result = discover(dir.path()).expect("discover");
        assert!(
            result.candidates.is_empty(),
            "non-source files must not trigger candidate creation"
        );
    }
}
