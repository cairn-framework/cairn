//! Machine state persistence.

use std::{collections::BTreeMap, fs, io, path::Path};

/// Type alias for target-to-hash mapping.
pub type TargetHashes = BTreeMap<String, String>;

/// Writes interface hash state JSON.
///
/// # Errors
///
/// Returns an I/O error when the state directory or JSON file cannot be written.
pub fn write_interface_hash(root: &Path, hashes: &TargetHashes) -> io::Result<()> {
    let dir = root.join(".cairn/state");
    fs::create_dir_all(&dir)?;
    let json = serde_json::to_string_pretty(hashes).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("serialization failed: {e}"),
        )
    })?;
    fs::write(dir.join("interface-hashes.json"), json)
}

/// Reads interface hash state from JSON.
///
/// # Errors
///
/// Returns an I/O error when the state file cannot be read.
pub fn read_interface_hash(root: &Path) -> io::Result<TargetHashes> {
    let path = root.join(".cairn/state/interface-hashes.json");
    if !path.exists() {
        return Ok(BTreeMap::new());
    }
    let content = fs::read_to_string(path)?;
    let hashes: BTreeMap<String, String> = serde_json::from_str(&content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("invalid JSON: {e}")))?;
    Ok(hashes)
}

/// Migrates a single-hash node entry to target-level hash entries.
pub fn migrate_single_hash(hashes: &mut TargetHashes, node_id: &str, hash: String) {
    let key = format!("{node_id}:");
    if !hashes.keys().any(|k| k.starts_with(&key)) {
        let target_key = format!("{node_id}:default");
        hashes.insert(target_key, hash);
    }
}
