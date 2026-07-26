// Reason: `tempfile` supplies cross-platform atomic replacement; its target-specific
// transitive crates coexist with versions already locked by the main workspace.
#![allow(clippy::multiple_crate_versions)]

//! Crate `cairn-agent-pack` provides harness-neutral canonical source management,
//! manifest validation, path containment checks, and deterministic raw-byte rendering
//! for Cairn agent packs.

mod containment;
mod manifest;
mod manifest_error;
mod plan;
mod plan_error;

pub use containment::{
    ContainmentError, validate_lexical_containment, validate_resolved_containment,
    validate_resolved_path,
};

use manifest::PackManifest;
use plan::{build_render_plan, check_drift, write_plan};
use plan_error::PlanError;
use std::fs;
use std::path::Path;

/// Runs check mode for one harness against a pack manifest file and target
/// repository root.
///
/// Parses the manifest at `manifest_path`, builds the render plan for `harness`
/// from canonical source files relative to the manifest directory, and verifies
/// disk content under `repo_root`.
///
/// # Errors
///
/// Returns an error if manifest parsing, source loading, or drift verification fails,
/// or if missing/drifted files are detected.
pub fn run_check(
    manifest_path: &Path,
    repo_root: &Path,
    harness: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(manifest_path)?;
    let manifest = PackManifest::parse_str(&content)?;
    let base_dir = manifest_directory(manifest_path);
    let plan = build_render_plan(&manifest, base_dir, harness)?;
    let drift = check_drift(&plan, repo_root)?;

    if !drift.is_clean() {
        return Err(Box::new(PlanError::DriftDetected(Box::new(
            plan_error::DriftReport {
                missing: drift.missing,
                drifted: drift.drifted,
                manifest_path: manifest_path.to_path_buf(),
                repo_root: repo_root.to_path_buf(),
                harness: harness.to_owned(),
            },
        ))));
    }

    Ok(())
}

/// Runs write mode for one harness against a pack manifest file and target
/// repository root.
///
/// Parses the manifest at `manifest_path`, builds the render plan for `harness`
/// from canonical source files relative to the manifest directory, validates
/// containment for all targets, and performs atomic raw-byte disk writes under
/// `repo_root`.
///
/// # Errors
///
/// Returns an error if manifest parsing, source loading, containment check, or disk write fails.
pub fn run_write(
    manifest_path: &Path,
    repo_root: &Path,
    harness: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(manifest_path)?;
    let manifest = PackManifest::parse_str(&content)?;
    let base_dir = manifest_directory(manifest_path);
    let plan = build_render_plan(&manifest, base_dir, harness)?;
    write_plan(&plan, repo_root)?;

    Ok(())
}

fn manifest_directory(manifest_path: &Path) -> &Path {
    manifest_path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
}

#[cfg(test)]
mod tests {
    use super::manifest_directory;
    use std::path::Path;

    #[test]
    fn bare_manifest_uses_current_directory() {
        assert_eq!(
            manifest_directory(Path::new("manifest.toml")),
            Path::new(".")
        );
        assert_eq!(
            manifest_directory(Path::new("dir/manifest.toml")),
            Path::new("dir")
        );
    }
}
