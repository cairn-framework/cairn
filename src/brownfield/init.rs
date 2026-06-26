//! `cairn init --from-code` implementation.
//!
//! Runs brownfield discovery against the project root and writes the
//! results as a change proposal under `meta/changes/brownfield-init/`.

use std::path::Path;

use crate::error::CairnError;

use super::write_change;

/// Fixed change ID for the initial brownfield extraction.
const CHANGE_ID: &str = "brownfield-init";

/// Comment-only starter blueprint written when `init --from-code` runs in a
/// project that has no `cairn.blueprint` yet. It parses to an empty graph, so
/// archiving the brownfield change merges the discovered modules into it.
const BASE_BLUEPRINT: &str = "# cairn.blueprint\n\
# Seeded by `cairn init --from-code`. The discovered modules are merged in when\n\
# you archive the brownfield extraction:\n\
#\n\
#     cairn archive brownfield-init\n\
#\n\
# Refine the map afterwards: nest modules under Systems and Containers, and add\n\
# decisions, contracts, and edges as the architecture clarifies.\n";

/// Run brownfield init, creating a change directory with discovered
/// candidates and a blueprint delta.
///
/// # Errors
///
/// Returns `CairnError::ChangeDiscovery` when the change directory
/// already exists (unless `force` is true) or when filesystem
/// operations fail.
pub fn run_init_from_code(root: &Path, force: bool) -> Result<String, CairnError> {
    let change_dir = root.join("meta/changes").join(CHANGE_ID);
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
    // Seed a base blueprint only after discovery succeeds, so a failed
    // traversal does not leave a stray truth file behind.
    let blueprint_path = root.join("cairn.blueprint");
    if !blueprint_path.exists() {
        std::fs::write(&blueprint_path, BASE_BLUEPRINT).map_err(|e| CairnError::WriteOutput {
            path: blueprint_path.to_string_lossy().to_string(),
            detail: e.to_string(),
        })?;
    }
    write_change(root, CHANGE_ID, &extraction, &templates)?;
    Ok(CHANGE_ID.to_owned())
}
