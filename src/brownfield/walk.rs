//! Bounded filesystem survey behind brownfield candidate discovery.
//!
//! Records the source files each directory holds directly, and which
//! directories are package roots (they hold a package manifest). Depth is
//! bounded per package rather than per repository: a package root restarts
//! the budget, so a package whose sources sit several levels below its
//! manifest is surveyed rather than pruned.

use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

use crate::error::CairnError;

/// Supported source file extensions for candidate discovery.
const SOURCE_EXTS: &[&str] = &["rs", "ts", "js", "py", "go"];

/// Maximum directory depth below the enclosing package root, or below the
/// repository root for code that sits outside every package.
const MAX_DEPTH: usize = 4;

/// Absolute ceiling on traversal depth below the repository root. A package
/// root restarts `MAX_DEPTH`, so manifests nested all the way down would
/// otherwise let the walk recurse without bound.
const MAX_TOTAL_DEPTH: usize = 32;

/// Filenames that mark a directory as the root of a package.
const MANIFEST_FILES: &[&str] = &["package.json", "pyproject.toml", "Cargo.toml", "go.mod"];

/// What one filesystem survey observed.
#[derive(Debug, Default)]
pub(super) struct Survey {
    /// Source files held directly by each directory that holds any.
    dir_files: BTreeMap<PathBuf, Vec<PathBuf>>,
    /// Directories holding a package manifest.
    package_roots: BTreeSet<PathBuf>,
}

/// Survey `root`, recording source files per directory and package roots.
///
/// # Errors
///
/// Returns `CairnError::ChangeDiscovery` when directory traversal fails.
pub(super) fn survey(root: &Path) -> Result<Survey, CairnError> {
    let mut observed = Survey::default();
    walk(root, &mut observed, 0, 0)?;
    Ok(observed)
}

impl Survey {
    /// Directories that become candidates, each paired with the source files
    /// it accounts for, keyed and ordered by path.
    ///
    /// A package root accounts for every source file below it that no nearer
    /// package root claims, the innermost package wins where roots nest, and a
    /// directory no package claims qualifies on holding `min_files` source
    /// files directly. Rule and rationale: `dec.brownfield-package-root-discovery`.
    ///
    /// Files under a package root dropped for enclosing another are claimed by
    /// nothing: the reconciler reports them as orphans, which is the intended
    /// outcome for a workspace root's own stray sources.
    pub(super) fn candidates<'a>(
        &'a self,
        root: &Path,
        min_files: usize,
    ) -> BTreeMap<&'a Path, Vec<&'a Path>> {
        let mut planned: BTreeMap<&Path, Vec<&Path>> = BTreeMap::new();
        for (dir, files) in &self.dir_files {
            let Some(package) = self.package_of(dir) else {
                continue;
            };
            if package == root {
                continue;
            }
            planned
                .entry(package)
                .or_default()
                .extend(files.iter().map(PathBuf::as_path));
        }

        let enclosing: Vec<&Path> = planned
            .keys()
            .copied()
            .filter(|package| {
                planned
                    .keys()
                    .any(|other| other != package && other.starts_with(package))
            })
            .collect();
        for package in enclosing {
            planned.remove(package);
        }

        let packages: BTreeSet<&Path> = planned.keys().copied().collect();
        for (dir, files) in &self.dir_files {
            let dir = dir.as_path();
            // Package-root membership is checked against every manifest
            // directory, not the survivors: a root dropped for enclosing
            // another must not return through its own direct file count.
            if files.len() < min_files || self.package_roots.contains(dir) {
                continue;
            }
            if self
                .enclosing_package(dir)
                .is_some_and(|package| packages.contains(package))
            {
                continue;
            }
            planned.insert(dir, files.iter().map(PathBuf::as_path).collect());
        }
        planned
    }

    /// Every source file the survey observed, ordered by directory then path.
    ///
    /// Broader than `candidates`: a directory below the candidate threshold
    /// still holds source files, and a consumer indexing file contents rather
    /// than proposing nodes needs all of them.
    pub(super) fn source_files(&self) -> impl Iterator<Item = &Path> {
        self.dir_files
            .values()
            .flat_map(|files| files.iter().map(PathBuf::as_path))
    }

    /// The package root owning `dir`: `dir` itself when it holds a manifest,
    /// otherwise the nearest ancestor that does.
    fn package_of<'a>(&'a self, dir: &Path) -> Option<&'a Path> {
        dir.ancestors()
            .find_map(|ancestor| self.package_roots.get(ancestor).map(PathBuf::as_path))
    }

    /// The package root strictly above `dir`.
    fn enclosing_package<'a>(&'a self, dir: &Path) -> Option<&'a Path> {
        dir.parent().and_then(|parent| self.package_of(parent))
    }
}

fn walk(
    current: &Path,
    observed: &mut Survey,
    depth: usize,
    total_depth: usize,
) -> Result<(), CairnError> {
    if depth > MAX_DEPTH || total_depth > MAX_TOTAL_DEPTH {
        return Ok(());
    }
    let entries = read_dir(current)?;

    let mut files_here = Vec::new();
    let mut is_package_root = false;
    for path in &entries {
        if !path.is_file() {
            continue;
        }
        if is_source_file(path) {
            files_here.push(path.clone());
        }
        if is_manifest(path) {
            is_package_root = true;
        }
    }
    if is_package_root {
        observed.package_roots.insert(current.to_path_buf());
    }
    if !files_here.is_empty() {
        observed.dir_files.insert(current.to_path_buf(), files_here);
    }

    // A package root restarts the depth budget: its sources may sit below the
    // repository-relative bound and still belong to it.
    let child_depth = if is_package_root { 1 } else { depth + 1 };
    for path in &entries {
        if path.is_dir() && !is_ignored_dir(path) && !is_symlink(path) {
            walk(path, observed, child_depth, total_depth + 1)?;
        }
    }
    Ok(())
}

fn read_dir(current: &Path) -> Result<Vec<PathBuf>, CairnError> {
    let mut entries = Vec::new();
    for entry in std::fs::read_dir(current).map_err(|e| traversal_failed(current, &e))? {
        entries.push(entry.map_err(|e| traversal_failed(current, &e))?.path());
    }
    Ok(entries)
}

fn traversal_failed(path: &Path, error: &std::io::Error) -> CairnError {
    CairnError::ChangeDiscovery {
        path: path.to_string_lossy().to_string(),
        detail: error.to_string(),
    }
}

fn file_name(path: &Path) -> &str {
    path.file_name().map_or("", |n| n.to_str().unwrap_or(""))
}

fn is_manifest(path: &Path) -> bool {
    MANIFEST_FILES.contains(&file_name(path))
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
    matches!(
        file_name(path),
        "target" | "node_modules" | ".git" | ".cairn" | "openspec" | "meta" | "dist" | "build"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(root: &Path, rel: &str, content: &str) {
        let path = root.join(rel);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }

    #[test]
    fn every_manifest_name_anchors_a_package() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        for (pkg, manifest, source) in [
            ("node", "package.json", "one.ts"),
            ("py", "pyproject.toml", "one.py"),
            ("rust", "Cargo.toml", "one.rs"),
            ("go", "go.mod", "one.go"),
        ] {
            write(root, &format!("{pkg}/{manifest}"), "");
            write(root, &format!("{pkg}/src/deep/{source}"), "");
        }

        let observed = survey(root).unwrap();
        let planned = observed.candidates(root, 3);
        let mut names: Vec<&str> = planned
            .keys()
            .map(|p| p.file_name().unwrap().to_str().unwrap())
            .collect();
        names.sort_unstable();
        assert_eq!(
            names,
            vec!["go", "node", "py", "rust"],
            "each manifest name must anchor its package on one source file"
        );
    }

    #[test]
    fn ignored_dirs_matched() {
        assert!(is_ignored_dir(Path::new("/repo/target")));
        assert!(is_ignored_dir(Path::new("/repo/node_modules")));
        assert!(!is_ignored_dir(Path::new("/repo/src")));
    }

    #[test]
    fn manifest_files_matched() {
        for name in MANIFEST_FILES {
            assert!(
                is_manifest(&Path::new("/repo").join(name)),
                "{name} must anchor a package root"
            );
        }
        assert!(!is_manifest(Path::new("/repo/tsconfig.json")));
        assert!(!is_manifest(Path::new("/repo/package.json.bak")));
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

    #[test]
    fn package_root_restarts_the_depth_budget() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        // Five levels below the repository root, but only two below the
        // manifest at a/b/c.
        write(root, "a/b/c/package.json", "{}\n");
        write(root, "a/b/c/src/tools/one.ts", "");

        let observed = survey(root).unwrap();
        assert!(
            observed
                .dir_files
                .contains_key(&root.join("a/b/c/src/tools")),
            "sources below a package manifest must be surveyed: {:?}",
            observed.dir_files.keys().collect::<Vec<_>>()
        );
    }

    #[test]
    fn code_outside_any_package_stays_bounded() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write(root, "a/b/c/d/e/deep.rs", "");

        let observed = survey(root).unwrap();
        assert!(
            observed.dir_files.is_empty(),
            "no manifest means the repository-relative bound still prunes"
        );
    }

    #[test]
    fn a_package_root_covers_its_qualifying_descendants() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write(root, "pkg/package.json", "{}\n");
        for name in ["one.ts", "two.ts", "three.ts"] {
            write(root, &format!("pkg/src/{name}"), "");
        }

        let observed = survey(root).unwrap();
        let planned = observed.candidates(root, 3);
        assert_eq!(
            planned.keys().collect::<Vec<_>>(),
            vec![&root.join("pkg").as_path()],
            "the package root is the only candidate"
        );
        assert_eq!(planned[root.join("pkg").as_path()].len(), 3);
    }

    #[test]
    fn an_enclosing_workspace_root_yields_to_its_packages() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write(root, "ws/package.json", "{}\n");
        write(root, "ws/loose.ts", "");
        write(root, "ws/inner/package.json", "{}\n");
        write(root, "ws/inner/one.ts", "");

        let observed = survey(root).unwrap();
        let planned = observed.candidates(root, 3);
        assert_eq!(
            planned.keys().collect::<Vec<_>>(),
            vec![&root.join("ws/inner").as_path()],
            "a workspace root that encloses a package must not swallow it"
        );
    }

    /// A workspace root dropped for enclosing a package must not return
    /// through the direct-count rule: that would restore the nesting the
    /// drop exists to prevent.
    #[test]
    fn a_dropped_workspace_root_is_not_readmitted_by_its_own_file_count() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write(root, "ws/package.json", "{}\n");
        for name in ["a.ts", "b.ts", "c.ts"] {
            write(root, &format!("ws/{name}"), "");
        }
        write(root, "ws/inner/package.json", "{}\n");
        write(root, "ws/inner/one.ts", "");

        let observed = survey(root).unwrap();
        let planned = observed.candidates(root, 3);
        assert_eq!(
            planned.keys().collect::<Vec<_>>(),
            vec![&root.join("ws/inner").as_path()],
            "the dropped workspace root must not reappear as a direct candidate"
        );
    }

    /// A dense directory that merely contains a package keeps its own
    /// candidate. `dec.brownfield-discovery-cycle-severity` clause 2 rules
    /// that package roots and subpackages stay flat sibling Modules, which is
    /// about emitted shape, not path containment. Dropping the parent instead
    /// orphans every file it holds: measured on `AutoDocs`, suppressing dense
    /// ancestors of a package took orphan findings from 2 to 20.
    #[test]
    fn a_plain_directory_containing_a_package_keeps_its_own_candidate() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        for name in ["a.ts", "b.ts", "c.ts"] {
            write(root, &format!("tools/{name}"), "");
        }
        write(root, "tools/pkg/package.json", "{}\n");
        write(root, "tools/pkg/one.ts", "");

        let observed = survey(root).unwrap();
        let planned = observed.candidates(root, 3);
        assert_eq!(
            planned.keys().collect::<Vec<_>>(),
            vec![
                &root.join("tools").as_path(),
                &root.join("tools/pkg").as_path()
            ],
            "the package is anchored without orphaning the files around it"
        );
    }

    #[test]
    fn nested_manifests_cannot_outrun_the_absolute_ceiling() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let mut rel = String::new();
        for _ in 0..(MAX_TOTAL_DEPTH + 4) {
            rel.push_str("n/");
            write(root, &format!("{rel}package.json"), "{}\n");
        }
        write(root, &format!("{rel}deep.ts"), "");

        let observed = survey(root).unwrap();
        assert!(
            !observed.dir_files.contains_key(&root.join(&rel)),
            "a manifest at every level must not defeat the absolute ceiling"
        );
    }
}
