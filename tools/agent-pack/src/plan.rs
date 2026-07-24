//! Render plan building, drift verification, and atomic disk writes.

use crate::containment::validate_resolved_containment;
use crate::manifest::PackManifest;
use crate::plan_error::PlanError;
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub(crate) type RenderPlan = BTreeMap<PathBuf, PlannedOutput>;
#[derive(Debug)]
pub(crate) struct PlannedOutput {
    adapter_index: usize,
    content: Vec<u8>,
}

/// Result of evaluating a render plan against disk.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct DriftResult {
    /// Destination paths missing from disk.
    pub(crate) missing: Vec<PathBuf>,
    /// Destination paths whose contents differ on disk.
    pub(crate) drifted: Vec<PathBuf>,
}

impl DriftResult {
    /// Returns true if no files are missing or drifted.
    #[must_use]
    pub(crate) fn is_clean(&self) -> bool {
        self.missing.is_empty() && self.drifted.is_empty()
    }
}

/// Builds a deterministic, BTreeMap-ordered render plan from a validated manifest.
///
/// # Errors
///
/// Returns [`PlanError::SourceReadError`] if a canonical source file cannot be loaded.
pub(crate) fn build_render_plan(
    manifest: &PackManifest,
    base_dir: &Path,
) -> Result<RenderPlan, PlanError> {
    let mut canonical_sources = std::collections::HashMap::new();
    for (index, canonical) in manifest.canonical.iter().enumerate() {
        let source_full =
            validate_resolved_containment(base_dir, &canonical.source).map_err(|error| {
                PlanError::SourceContainment {
                    index,
                    path: canonical.source.clone(),
                    error,
                }
            })?;
        let content = fs::read(&source_full).map_err(|error| PlanError::SourceReadError {
            index,
            path: canonical.source.clone(),
            details: error.to_string(),
        })?;
        canonical_sources.insert((canonical.entry.clone(), canonical.mode.clone()), content);
    }

    let mut plan = BTreeMap::new();
    for (adapter_index, adapter) in manifest.adapters.iter().enumerate() {
        let content = canonical_sources
            .get(&(adapter.entry.clone(), adapter.mode.clone()))
            .ok_or_else(|| PlanError::MissingCanonical {
                entry: adapter.entry.clone(),
                mode: adapter.mode.clone(),
            })?;
        plan.insert(
            adapter.destination.clone(),
            PlannedOutput {
                adapter_index,
                content: content.clone(),
            },
        );
    }
    Ok(plan)
}

/// Evaluates a render plan against disk files under `repo_root`.
///
/// # Errors
///
/// Returns [`PlanError::DestinationIo`] or a containment error when a
/// destination cannot be read safely.
pub(crate) fn check_drift(plan: &RenderPlan, repo_root: &Path) -> Result<DriftResult, PlanError> {
    let mut result = DriftResult::default();

    for (dest_rel, output) in plan {
        let full_dest = validate_resolved_containment(repo_root, dest_rel).map_err(|error| {
            PlanError::DestinationContainment {
                index: output.adapter_index,
                destination: dest_rel.clone(),
                error,
            }
        })?;
        if !full_dest.exists() {
            result.missing.push(dest_rel.clone());
            continue;
        }

        let existing = fs::read(&full_dest).map_err(|error| PlanError::DestinationIo {
            index: output.adapter_index,
            destination: dest_rel.clone(),
            details: error.to_string(),
        })?;

        if existing != output.content {
            result.drifted.push(dest_rel.clone());
        }
    }

    Ok(result)
}

/// Validates containment for all plan targets, then writes them atomically using temp files.
///
/// All target destinations are validated before creating or modifying any file on disk.
///
/// # Errors
///
/// Returns [`PlanError`] if containment validation fails or disk IO fails.
pub(crate) fn write_plan(plan: &RenderPlan, repo_root: &Path) -> Result<(), PlanError> {
    for (destination, output) in plan {
        validate_resolved_containment(repo_root, destination).map_err(|error| {
            PlanError::DestinationContainment {
                index: output.adapter_index,
                destination: destination.clone(),
                error,
            }
        })?;
    }
    for (destination, output) in plan {
        atomic_write(&repo_root.join(destination), &output.content).map_err(|details| {
            PlanError::DestinationIo {
                index: output.adapter_index,
                destination: destination.clone(),
                details,
            }
        })?;
    }
    Ok(())
}

fn atomic_write(destination: &Path, content: &[u8]) -> Result<(), String> {
    let parent = destination.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)
        .map_err(|error| format!("create parent '{}': {error}", parent.display()))?;
    let mut temporary = tempfile::NamedTempFile::new_in(parent)
        .map_err(|error| format!("create temporary file in '{}': {error}", parent.display()))?;
    temporary
        .write_all(content)
        .and_then(|()| temporary.as_file().sync_all())
        .map_err(|error| format!("write temporary file: {error}"))?;
    set_output_permissions(temporary.as_file(), destination)
        .map_err(|error| format!("set output permissions: {error}"))?;
    temporary
        .persist(destination)
        .map_err(|error| format!("replace '{}': {}", destination.display(), error.error))?;
    Ok(())
}

fn set_output_permissions(file: &fs::File, destination: &Path) -> std::io::Result<()> {
    if let Ok(metadata) = fs::metadata(destination) {
        return file.set_permissions(metadata.permissions());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        file.set_permissions(fs::Permissions::from_mode(0o644))?;
    }
    Ok(())
}
