//! Contract-baseline state persistence.
//!
//! Two writers are sanctioned: draft acceptance (`summariser::accept`) and the
//! non-generative record/drop surface. The drift enforcer reads this state and
//! never writes it.

use crate::persist;
use std::{
    collections::BTreeMap,
    io,
    path::{Path, PathBuf},
};

/// Schema version of `.cairn/state/contract-baselines.json`.
const VERSION: u32 = 1;

/// Relative path of the state file, from the project root.
const STATE_PATH: &str = ".cairn/state/contract-baselines.json";

/// A node's structural shape at the moment its contract was last reviewed.
///
/// Deliberately a reduced record rather than
/// [`NodeFingerprint`](super::state::NodeFingerprint): `paths` is excluded
/// because path-only edits are ungated, and a mandatory `paths` field would be
/// both written and required on read for a value nothing compares.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractBaseline {
    /// Node kind (e.g. "System", "Module").
    pub kind: String,
    /// Parent node ID, or `null` for a root node.
    pub parent: Option<String>,
    /// Sorted, deduplicated outbound dependency-edge target node IDs.
    pub edges: Vec<String>,
}

/// Versioned map of node ID to reviewed contract baseline.
///
/// Shares the `version`/`nodes` envelope of
/// [`BlueprintSnapshot`](super::state::BlueprintSnapshot), with `version`
/// serialised first so a reader can inspect the schema version without a full
/// parse (conventions section 3).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractBaselines {
    /// Schema version. Current: 1.
    pub version: u32,
    /// Node-ID-to-baseline mapping.
    pub nodes: BTreeMap<String, ContractBaseline>,
}

impl ContractBaselines {
    /// Creates an empty baseline set at the current schema version.
    #[must_use]
    pub fn new() -> Self {
        Self {
            version: VERSION,
            nodes: BTreeMap::new(),
        }
    }
}

impl Default for ContractBaselines {
    fn default() -> Self {
        Self::new()
    }
}

/// Path of the baseline state file under `root`.
///
/// Exposed so a caller that must restore the file byte-for-byte on rollback,
/// including deleting one it created, can address it without duplicating the
/// relative path.
#[must_use]
pub fn state_path(root: &Path) -> PathBuf {
    root.join(STATE_PATH)
}

/// Writes contract-baseline state JSON.
///
/// # Errors
///
/// Returns an I/O error when the state directory or JSON file cannot be written.
pub fn write(root: &Path, baselines: &ContractBaselines) -> io::Result<()> {
    persist::write_json(&state_path(root), baselines)
}

/// Reads contract-baseline state from JSON.
///
/// Returns an empty set when the state file does not exist: a repository that
/// has never recorded a baseline is not in error, and no backfill is performed.
///
/// # Errors
///
/// Returns an I/O error when the file exists but cannot be read, parsed, or
/// carries an unsupported schema version.
pub fn read(root: &Path) -> io::Result<ContractBaselines> {
    let path = state_path(root);
    let Some((version, content)) = persist::read_versioned_json(&path)? else {
        return Ok(ContractBaselines::default());
    };
    if version == VERSION {
        return persist::parse_json(&content, &path);
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("contract-baselines.json: unsupported version {version} (expected 1)"),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absent_file_yields_an_empty_set() {
        let root = tempfile::tempdir().unwrap();
        let baselines = read(root.path()).unwrap();
        assert!(baselines.nodes.is_empty());
        assert_eq!(baselines.version, VERSION);
    }

    #[test]
    fn baselines_round_trip() {
        let root = tempfile::tempdir().unwrap();
        let mut baselines = ContractBaselines::new();
        baselines.nodes.insert(
            "app".to_owned(),
            ContractBaseline {
                kind: "System".to_owned(),
                parent: None,
                edges: Vec::new(),
            },
        );
        baselines.nodes.insert(
            "app.api".to_owned(),
            ContractBaseline {
                kind: "Module".to_owned(),
                parent: Some("app".to_owned()),
                edges: vec!["app.core".to_owned()],
            },
        );
        write(root.path(), &baselines).unwrap();

        assert_eq!(read(root.path()).unwrap(), baselines);
    }

    #[test]
    fn unsupported_version_returns_error() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join(STATE_PATH);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, r#"{"version":2,"nodes":{}}"#).unwrap();

        let err = read(root.path()).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("unsupported version"), "{msg}");
        assert!(msg.contains('2'), "{msg}");
    }

    #[test]
    fn unknown_fields_are_rejected() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join(STATE_PATH);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        for raw in [
            r#"{"version":1,"nodes":{},"extra":true}"#,
            r#"{"version":1,"nodes":{"app":{"kind":"System","parent":null,"edges":[],"extra":true}}}"#,
        ] {
            std::fs::write(&path, raw).unwrap();
            assert!(read(root.path()).is_err(), "{raw}");
            assert_eq!(std::fs::read_to_string(&path).unwrap(), raw);
        }
    }
}
