//! The agent-pack ownership ledger: what the packager owns, and what state
//! each owned file is in.
//!
//! Split from the verbs so the rules that decide whether a byte may be written
//! are readable on their own. Everything here is decision logic over the
//! manifest and the disk; `pack.rs` owns dispatch and rendering.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use super::pack_assets::PackAsset;
use super::wire::atomic_write;
use sha2::{Digest, Sha256};
use std::fs;

/// Ownership ledger location inside the target project.
pub(crate) const MANIFEST_PATH: &str = ".cairn/state/agent-pack.json";

/// Version of the pack content compiled into this binary. Pinned to
/// `tools/agent-pack/manifest.toml`; `tests/pack_lifecycle.rs` fails if the two
/// drift apart.
pub(crate) const BUNDLE_VERSION: &str = "1.0.0";

/// Manifest schema version. Bump only on an incompatible ledger change.
pub(crate) const SCHEMA_VERSION: u32 = 1;

/// A note shown when updating across a bundle-version boundary. Compiled in,
/// never fetched (`dec.agent-pack-packaging` clause 6). Empty until a bundle
/// version ships that needs one.
pub(crate) struct MigrationNote {
    /// Bundle version that introduced the change.
    pub(crate) upto: &'static str,
    /// What the reader has to do, if anything.
    pub(crate) body: &'static str,
}

pub(crate) const MIGRATION_NOTES: &[MigrationNote] = &[];

/// The installed ownership ledger.
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct InstalledManifest {
    /// Ledger schema version.
    pub(crate) schema_version: u32,
    /// Version of the cairn binary that last wrote this ledger.
    pub(crate) cli_version: String,
    /// Version of the pack content that binary carried.
    pub(crate) bundle_version: String,
    /// Harness whose adapter produced these destinations.
    pub(crate) harness: String,
    /// Every file the packager owns, sorted by path.
    pub(crate) files: Vec<InstalledFile>,
}

/// One owned file and the content hash it had when the packager wrote it.
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct InstalledFile {
    /// Destination relative to the project root.
    pub(crate) path: String,
    /// Lowercase hex SHA-256 of the bytes the packager wrote.
    pub(crate) sha256: String,
}

/// What one bundled asset needs, given the disk and the ledger.
#[derive(PartialEq, Eq)]
pub(crate) enum Action {
    /// Absent from disk: write it.
    Backfill,
    /// Owned, pristine, and the bundle carries different bytes: refresh it.
    Refresh,
    /// Present, unowned, and byte-identical to the bundle: record it.
    Adopt,
    /// Owned or unowned but edited: report, never touch.
    Modified,
    /// Owned, pristine, and already the bundle bytes.
    Unchanged,
}

/// Lowercase hex SHA-256 of `bytes`.
pub(crate) fn digest(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .fold(String::with_capacity(64), |mut acc, byte| {
            use std::fmt::Write;
            let _ = write!(acc, "{byte:02x}");
            acc
        })
}

/// Read the ledger, or `None` when this is a fresh or legacy install.
pub(crate) fn read_manifest(root: &Path) -> Result<Option<InstalledManifest>, CliResult> {
    let path = root.join(MANIFEST_PATH);
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(err(
                1,
                &copy::lookup("pack.err-manifest-parse")
                    .replace("{file}", MANIFEST_PATH)
                    .replace("{detail}", &error.to_string()),
            ));
        }
    };
    serde_json::from_str(&text).map(Some).map_err(|error| {
        err(
            1,
            &copy::lookup("pack.err-manifest-parse")
                .replace("{file}", MANIFEST_PATH)
                .replace("{detail}", &error.to_string()),
        )
    })
}

/// Classify one asset against the ledger and the disk.
pub(crate) fn classify(root: &Path, asset: &PackAsset, owned: Option<&str>) -> Action {
    let full = root.join(asset.path);
    let Ok(disk) = fs::read(&full) else {
        return Action::Backfill;
    };
    let on_disk = digest(&disk);
    let bundled = digest(asset.content.as_bytes());
    // Content is identity here. A file that already holds the bundled bytes
    // needs nothing, whoever put them there: an interrupted update that wrote
    // the file before publishing the ledger would otherwise look edited
    // forever, and no verb would ever repair it.
    if on_disk == bundled {
        return if owned.is_some() {
            Action::Unchanged
        } else {
            Action::Adopt
        };
    }
    match owned {
        // Owned and still matching what the packager wrote: safe to refresh.
        Some(recorded) if recorded == on_disk => Action::Refresh,
        // Owned and edited, or unowned and different: report, never touch.
        _ => Action::Modified,
    }
}

/// Publish the ledger last, through an atomic replacement.
pub(crate) fn write_manifest(
    root: &Path,
    harness: &str,
    files: Vec<InstalledFile>,
) -> Result<(), CliResult> {
    let manifest = InstalledManifest {
        schema_version: SCHEMA_VERSION,
        cli_version: env!("CARGO_PKG_VERSION").to_owned(),
        bundle_version: BUNDLE_VERSION.to_owned(),
        harness: harness.to_owned(),
        files,
    };
    let Ok(mut body) = serde_json::to_string_pretty(&manifest) else {
        return Err(err(1, "failed to serialise the pack manifest"));
    };
    body.push('\n');
    let full = root.join(MANIFEST_PATH);
    let Some(parent) = full.parent() else {
        return Err(err(1, "pack manifest has no parent directory"));
    };
    if let Err(error) = fs::create_dir_all(parent) {
        return Err(err(
            1,
            &copy::lookup("pack.err-write")
                .replace("{file}", MANIFEST_PATH)
                .replace("{detail}", &error.to_string()),
        ));
    }
    atomic_write(parent, &full, &body).map_err(|detail| {
        err(
            1,
            &copy::lookup("pack.err-write")
                .replace("{file}", MANIFEST_PATH)
                .replace("{detail}", &detail),
        )
    })
}

/// Does this ledger already own loop-mode assets?
pub(crate) fn has_loop_assets(manifest: &InstalledManifest) -> bool {
    manifest
        .files
        .iter()
        .any(|file| file.path.ends_with("references/loop-mode.md"))
}

/// Notes whose introducing version is newer than what is installed and no
/// newer than what this binary carries. Pure so it can be tested without
/// shipping a fake note.
pub(crate) fn migration_notes<'a>(
    table: &'a [MigrationNote],
    installed: &str,
    current: &str,
) -> Vec<&'a str> {
    let installed = parse_version(installed);
    let current = parse_version(current);
    table
        .iter()
        .filter(|note| {
            let upto = parse_version(note.upto);
            installed < upto && upto <= current
        })
        .map(|note| note.body)
        .collect()
}

/// Parse `major.minor.patch`; unparsable components sort as zero.
fn parse_version(value: &str) -> (u32, u32, u32) {
    let mut parts = value.split('.').map(|part| part.parse().unwrap_or(0));
    (
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
    )
}

#[cfg(test)]
mod tests {
    use super::{MigrationNote, digest, migration_notes, parse_version};

    #[test]
    fn digest_is_the_known_sha256_of_the_empty_input() {
        assert_eq!(
            digest(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn parse_version_tolerates_short_and_broken_input() {
        assert_eq!(parse_version("1.2.3"), (1, 2, 3));
        assert_eq!(parse_version("1.2"), (1, 2, 0));
        assert_eq!(parse_version("nonsense"), (0, 0, 0));
    }

    #[test]
    fn migration_notes_cover_the_range_crossed_by_an_update() {
        let table = [
            MigrationNote {
                upto: "1.1.0",
                body: "one",
            },
            MigrationNote {
                upto: "2.0.0",
                body: "two",
            },
        ];
        assert_eq!(migration_notes(&table, "1.0.0", "1.1.0"), vec!["one"]);
        assert_eq!(migration_notes(&table, "1.1.0", "2.0.0"), vec!["two"]);
        // Both boundaries crossed at once.
        assert_eq!(
            migration_notes(&table, "1.0.0", "2.0.0"),
            vec!["one", "two"]
        );
        // Already current: nothing to say.
        assert!(migration_notes(&table, "2.0.0", "2.0.0").is_empty());
    }
}
