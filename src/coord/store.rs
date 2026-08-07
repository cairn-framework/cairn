//! Store root resolution and lazy initialisation.
//!
//! The store lives at `<git-common-dir>/cairn/coord/` and is created on the
//! FIRST append only: reads never create anything, and a missing store reads
//! as `uninitialised` rather than as an empty fact set.

use std::path::{Path, PathBuf};

use crate::persist;

use super::envelope::STORE_FORMAT;
use super::git::git_common_dir;

/// Resolves the store root for the checkout family containing `root`.
///
/// # Errors
///
/// Returns the git resolution error when `root` is not inside a repository.
pub fn store_root(root: &Path) -> Result<PathBuf, String> {
    Ok(git_common_dir(root)?.join("cairn/coord"))
}

/// Returns true when the store has been initialised by a prior append.
#[must_use]
pub fn is_initialised(store: &Path) -> bool {
    store.join("format").is_file()
}

/// Reads and checks the store format marker.
///
/// # Errors
///
/// Fails closed on an unreadable or unknown format, mirroring the
/// `read_versioned_json` discipline.
pub(crate) fn check_format(store: &Path) -> Result<(), String> {
    let raw = std::fs::read_to_string(store.join("format"))
        .map_err(|error| format!("coordination store format unreadable: {error}"))?;
    if raw.trim() != STORE_FORMAT.to_string() {
        return Err(format!(
            "coordination store format `{}` is not the supported `{STORE_FORMAT}`",
            raw.trim()
        ));
    }
    Ok(())
}

/// Initialises the store layout when absent; called from the append path
/// only.
///
/// # Errors
///
/// Returns an error when a directory or the format marker cannot be written,
/// or when an existing store carries an unknown format.
pub(crate) fn ensure_initialised(store: &Path) -> Result<(), String> {
    if is_initialised(store) {
        return check_format(store);
    }
    for dir in ["facts", "leases", "singleton", "cache", "archive"] {
        std::fs::create_dir_all(store.join(dir))
            .map_err(|error| format!("cannot create coordination store `{dir}`: {error}"))?;
    }
    persist::atomic_write(&store.join("format"), &format!("{STORE_FORMAT}\n"))
        .map_err(|error| format!("cannot write coordination store format: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialisation_writes_layout_and_format_once() {
        let dir = tempfile::TempDir::new().expect("tempdir");
        let store = dir.path().join("coord");
        assert!(!is_initialised(&store));
        ensure_initialised(&store).expect("initialises");
        for sub in ["facts", "leases", "singleton", "cache", "archive"] {
            assert!(store.join(sub).is_dir(), "{sub} exists");
        }
        assert_eq!(
            std::fs::read_to_string(store.join("format")).expect("format"),
            "1\n"
        );
        ensure_initialised(&store).expect("idempotent");
    }

    #[test]
    fn unknown_format_fails_closed() {
        let dir = tempfile::TempDir::new().expect("tempdir");
        let store = dir.path().join("coord");
        std::fs::create_dir_all(&store).expect("mkdir");
        std::fs::write(store.join("format"), "2\n").expect("format");
        assert!(check_format(&store).is_err());
        assert!(ensure_initialised(&store).is_err());
    }
}
