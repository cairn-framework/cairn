//! `cairn refine` implementation.
//!
//! Re-runs brownfield discovery and writes results to a timestamped
//! change directory under `openspec/changes/brownfield-refine-{secs}/`.

use std::path::Path;

use crate::error::CairnError;

use super::write_change;

/// Run brownfield refine, creating a timestamped change directory
/// with the latest discovery results.
///
/// # Errors
///
/// Returns `CairnError::ChangeDiscovery` when filesystem operations fail.
pub fn run_refine(root: &Path) -> Result<String, CairnError> {
    let extraction = super::discovery::discover(root)?;
    let change_id = format!("brownfield-refine-{}", timestamp());
    write_change(root, &change_id, &extraction)?;
    Ok(change_id)
}

fn timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{secs}")
}
