//! `cairn init --from-code` implementation.
//!
//! Runs brownfield discovery against the project root and writes the
//! results as a change proposal under `openspec/changes/brownfield-init/`.

use std::path::Path;

use crate::error::CairnError;

use super::write_change;

/// Fixed change ID for the initial brownfield extraction.
const CHANGE_ID: &str = "brownfield-init";

/// Run brownfield init, creating a change directory with discovered
/// candidates and a blueprint delta.
///
/// # Errors
///
/// Returns `CairnError::ChangeDiscovery` when the change directory
/// already exists (unless `force` is true) or when filesystem
/// operations fail.
pub fn run_init_from_code(root: &Path, force: bool) -> Result<String, CairnError> {
    let change_dir = root.join("openspec/changes").join(CHANGE_ID);
    if change_dir.exists() && !force {
        return Err(CairnError::ChangeDiscovery {
            path: change_dir.to_string_lossy().to_string(),
            detail: "change directory already exists; use --force to overwrite".to_owned(),
        });
    }
    if change_dir.exists() && force {
        // Guard: force must not silently wipe unreviewed suggestions.
        let queue_path = change_dir.join("suggested-edges.json");
        if queue_path.exists()
            && let Ok(Some(queue)) = crate::suggested_edges::read_queue(&queue_path)
            && crate::suggested_edges::count_pending(&queue) > 0
        {
            return Err(CairnError::ChangeDiscovery {
                path: change_dir.to_string_lossy().to_string(),
                detail: "change directory contains pending suggested edges; triage or remove them before forcing".to_owned(),
            });
        }
        std::fs::remove_dir_all(&change_dir).map_err(|e| CairnError::ChangeDiscovery {
            path: change_dir.to_string_lossy().to_string(),
            detail: e.to_string(),
        })?;
    }
    let extraction = super::discovery::discover(root)?;
    let templates = super::templates::load_templates(root);
    write_change(root, CHANGE_ID, &extraction, &templates)?;
    Ok(CHANGE_ID.to_owned())
}
