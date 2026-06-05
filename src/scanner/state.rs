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
    let path = dir.join("interface-hashes.json");
    let json = serde_json::to_string(hashes).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("serialization failed: {e}"),
        )
    })?;
    if let Ok(existing) = fs::read_to_string(&path)
        && existing == json
    {
        return Ok(());
    }
    fs::write(path, json)
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

/// Fingerprint for a blueprint node's structural identity.
/// Changes to kind or parent constitute a gated "shape change."
/// Paths are tracked for completeness but path-only changes are not gated.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NodeFingerprint {
    /// Node kind (e.g. "System", "Module").
    pub kind: String,
    /// Parent node ID, if any.
    pub parent: Option<String>,
    /// Sorted declared paths.
    pub paths: Vec<String>,
}

/// Versioned snapshot of blueprint node fingerprints.
///
/// The `version` field is serialized first so readers can inspect the schema
/// version without a full parse (conventions §3). Current schema version: 1.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BlueprintSnapshot {
    /// Schema version. Current: 1.
    pub version: u32,
    /// Node-ID-to-fingerprint mapping.
    pub nodes: BTreeMap<String, NodeFingerprint>,
}

impl BlueprintSnapshot {
    /// Creates an empty snapshot at the current schema version.
    #[must_use]
    pub fn new() -> Self {
        Self {
            version: 1,
            nodes: BTreeMap::new(),
        }
    }

    /// Returns `true` when no nodes are recorded.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

impl Default for BlueprintSnapshot {
    fn default() -> Self {
        Self::new()
    }
}

/// Writes blueprint snapshot state JSON.
///
/// # Errors
///
/// Returns an I/O error when the state directory or JSON file cannot be written.
pub fn write_blueprint_snapshot(root: &Path, snapshot: &BlueprintSnapshot) -> io::Result<()> {
    let dir = root.join(".cairn/state");
    fs::create_dir_all(&dir)?;
    let path = dir.join("blueprint-snapshot.json");
    let json = serde_json::to_string(snapshot).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("serialization failed: {e}"),
        )
    })?;
    if let Ok(existing) = fs::read_to_string(&path)
        && existing == json
    {
        return Ok(());
    }
    fs::write(path, json)
}

/// Reads blueprint snapshot state from JSON.
///
/// Returns an empty snapshot when the state file does not exist.
///
/// # Errors
///
/// Returns an I/O error when the file exists but cannot be read, parsed, or
/// carries an unsupported schema version.
pub fn read_blueprint_snapshot(root: &Path) -> io::Result<BlueprintSnapshot> {
    let path = root.join(".cairn/state/blueprint-snapshot.json");
    if !path.exists() {
        return Ok(BlueprintSnapshot::default());
    }
    let content = fs::read_to_string(path)?;
    let snapshot: BlueprintSnapshot = serde_json::from_str(&content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("invalid JSON: {e}")))?;
    if snapshot.version != 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "blueprint-snapshot.json: unsupported version {} (expected 1)",
                snapshot.version
            ),
        ));
    }
    Ok(snapshot)
}

/// Migrates a single-hash node entry to target-level hash entries.
pub fn migrate_single_hash(hashes: &mut TargetHashes, node_id: &str, hash: String) {
    let key = format!("{node_id}:");
    if !hashes.keys().any(|k| k.starts_with(&key)) {
        let target_key = format!("{node_id}:default");
        hashes.insert(target_key, hash);
    }
}
