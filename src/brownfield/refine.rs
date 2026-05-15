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
/// If a change directory with the same timestamp already exists, a
/// numeric suffix is appended to avoid silent overwrites.
///
/// # Errors
///
/// Returns `CairnError::ChangeDiscovery` when filesystem operations fail.
pub fn run_refine(root: &Path) -> Result<String, CairnError> {
    let extraction = super::discovery::discover(root)?;
    let change_id = unique_change_id(root, &timestamp());
    write_change(root, &change_id, &extraction)?;
    Ok(change_id)
}

/// Produces a unique change ID by appending a counter suffix when the
/// directory already exists.
fn unique_change_id(root: &Path, ts: &str) -> String {
    let base = format!("brownfield-refine-{ts}");
    let changes_dir = root.join("openspec/changes");
    if !changes_dir.join(&base).exists() {
        return base;
    }
    let mut counter = 1u32;
    loop {
        let candidate = format!("{base}-{counter}");
        if !changes_dir.join(&candidate).exists() {
            return candidate;
        }
        counter += 1;
    }
}

fn timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{nanos}")
}
