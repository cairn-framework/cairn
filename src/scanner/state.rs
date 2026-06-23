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
/// Changes to kind, parent, or the outbound dependency-edge set constitute a
/// gated "shape change."
/// Paths are tracked for completeness but path-only changes are not gated.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NodeFingerprint {
    /// Node kind (e.g. "System", "Module").
    pub kind: String,
    /// Parent node ID, if any.
    pub parent: Option<String>,
    /// Sorted declared paths.
    pub paths: Vec<String>,
    /// Sorted outbound dependency-edge target node IDs. Schema v2+; defaults
    /// empty when reading a v1 snapshot, which predates edge-drift tracking.
    #[serde(default)]
    pub edges: Vec<String>,
}

/// Versioned snapshot of blueprint node fingerprints.
///
/// The `version` field is serialized first so readers can inspect the schema
/// version without a full parse (conventions §3). Current schema version: 2.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BlueprintSnapshot {
    /// Schema version. Current: 2.
    pub version: u32,
    /// Node-ID-to-fingerprint mapping.
    pub nodes: BTreeMap<String, NodeFingerprint>,
}

impl BlueprintSnapshot {
    /// Creates an empty snapshot at the current schema version.
    #[must_use]
    pub fn new() -> Self {
        Self {
            version: 2,
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

/// Version-1 blueprint snapshot, kept for explicit migration to v2.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct BlueprintSnapshotV1 {
    version: u32,
    nodes: BTreeMap<String, NodeFingerprintV1>,
}

/// Version-1 node fingerprint, kept for explicit migration to v2.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct NodeFingerprintV1 {
    kind: String,
    parent: Option<String>,
    paths: Vec<String>,
}

/// Migrates a v1 blueprint snapshot to v2, adding an empty outbound-edge set
/// to every node (v1 predates edge tracking).
fn migrate_v1_to_v2(snapshot: BlueprintSnapshotV1) -> BlueprintSnapshot {
    let mut nodes = BTreeMap::new();
    for (id, fp) in snapshot.nodes {
        nodes.insert(
            id,
            NodeFingerprint {
                kind: fp.kind,
                parent: fp.parent,
                paths: fp.paths,
                edges: Vec::new(),
            },
        );
    }
    BlueprintSnapshot { version: 2, nodes }
}

/// Peeks the `version` field from a JSON object without fully deserializing
/// the payload.
fn peek_version(content: &str) -> io::Result<u32> {
    #[derive(serde::Deserialize)]
    struct VersionOnly {
        version: u32,
    }
    serde_json::from_str::<VersionOnly>(content)
        .map(|v| v.version)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("invalid JSON: {e}")))
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
    let version = peek_version(&content)?;
    match version {
        1 => {
            let snapshot: BlueprintSnapshotV1 = serde_json::from_str(&content).map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, format!("invalid JSON: {e}"))
            })?;
            Ok(migrate_v1_to_v2(snapshot))
        }
        2 => serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("invalid JSON: {e}"))),
        other => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("blueprint-snapshot.json: unsupported version {other} (expected 1 or 2)"),
        )),
    }
}

/// Migrates a single-hash node entry to target-level hash entries.
pub fn migrate_single_hash(hashes: &mut TargetHashes, node_id: &str, hash: String) {
    let key = format!("{node_id}:");
    if !hashes.keys().any(|k| k.starts_with(&key)) {
        let target_key = format!("{node_id}:default");
        hashes.insert(target_key, hash);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn read_missing_snapshot_returns_empty_v2() {
        let root = tempfile::tempdir().unwrap();
        let snapshot = read_blueprint_snapshot(root.path()).unwrap();
        assert!(snapshot.is_empty());
        assert_eq!(snapshot.version, 2);
    }

    #[test]
    fn v1_snapshot_migrates_to_v2_with_empty_edges() {
        let root = tempfile::tempdir().unwrap();
        let content =
            r#"{"version":1,"nodes":{"app.api":{"kind":"Module","parent":null,"paths":["src"]}}}"#;
        let path = root.path().join(".cairn/state/blueprint-snapshot.json");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, content).unwrap();

        let snapshot = read_blueprint_snapshot(root.path()).unwrap();
        assert_eq!(snapshot.version, 2);
        let fp = snapshot.nodes.get("app.api").unwrap();
        assert_eq!(fp.kind, "Module");
        assert!(fp.edges.is_empty());
    }

    #[test]
    fn v2_snapshot_round_trips_with_edges() {
        let root = tempfile::tempdir().unwrap();
        let mut nodes = BTreeMap::new();
        nodes.insert(
            "app.api".to_owned(),
            NodeFingerprint {
                kind: "Module".to_owned(),
                parent: None,
                paths: vec!["src".to_owned()],
                edges: vec!["app.db".to_owned()],
            },
        );
        let snapshot = BlueprintSnapshot { version: 2, nodes };
        write_blueprint_snapshot(root.path(), &snapshot).unwrap();

        let read = read_blueprint_snapshot(root.path()).unwrap();
        assert_eq!(read.version, 2);
        assert_eq!(read.nodes["app.api"].edges, vec!["app.db"]);
    }

    #[test]
    fn unsupported_version_returns_error() {
        let root = tempfile::tempdir().unwrap();
        let content = r#"{"version":99,"nodes":{}}"#;
        let path = root.path().join(".cairn/state/blueprint-snapshot.json");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, content).unwrap();

        let err = read_blueprint_snapshot(root.path()).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("unsupported version"), "{msg}");
        assert!(msg.contains("99"), "{msg}");
    }
}
