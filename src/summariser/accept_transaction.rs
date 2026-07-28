//! Rollback primitives for the accept commit sequence.
//!
//! `accept()` installs a contract, records the node's baseline, and marks the
//! draft accepted. Every step after the post-write scan is fallible and the
//! sequence is collectively atomic, so each one is paired with a restore of
//! everything written before it.

use std::path::Path;

use super::accept::AcceptError;
use crate::scanner::contract_baselines::state_path;

/// Reads the baseline state file verbatim for rollback. `None` means the file
/// does not exist, which a restore reproduces by deleting it.
pub(super) fn read_baseline_file(root: &Path) -> Result<Option<Vec<u8>>, AcceptError> {
    match std::fs::read(state_path(root)) {
        Ok(bytes) => Ok(Some(bytes)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(AcceptError::Io(
            crate::copy::lookup("baseline.state-read-failed").replace("{error}", &e.to_string()),
        )),
    }
}

fn restore_baseline_file(root: &Path, prior: Option<&[u8]>) -> Result<(), AcceptError> {
    let path = state_path(root);
    let result = match prior {
        Some(bytes) => crate::persist::atomic_write_bytes(&path, bytes),
        None => match std::fs::remove_file(&path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            other => other,
        },
    };
    result.map_err(|e| {
        AcceptError::Io(
            crate::copy::lookup("baseline.state-restore-failed").replace("{error}", &e.to_string()),
        )
    })
}

/// Undoes every write a commit step made: the contract text and the baseline
/// file. A failing restore never stops the other from being attempted, so one
/// broken device cannot leave the untouched half committed by omission; the
/// first error is reported.
pub(super) fn rollback(
    target_path: &Path,
    original: &str,
    root: &Path,
    baseline_backup: Option<&[u8]>,
) -> Result<(), AcceptError> {
    let contract = crate::persist::atomic_write(target_path, original).map_err(|e| {
        AcceptError::Io(
            crate::copy::lookup("baseline.contract-restore-failed")
                .replace("{path}", &target_path.display().to_string())
                .replace("{error}", &e.to_string()),
        )
    });
    let baseline = restore_baseline_file(root, baseline_backup);
    contract.and(baseline)
}

#[cfg(test)]
mod tests {
    use super::{read_baseline_file, rollback};

    fn temp_root() -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(".cairn/state")).unwrap();
        dir
    }

    #[test]
    fn absent_baseline_file_snapshots_as_none() {
        let dir = temp_root();
        assert_eq!(read_baseline_file(dir.path()).unwrap(), None);
    }

    #[test]
    fn rollback_restores_the_contract_and_the_prior_baseline_bytes() {
        let dir = temp_root();
        let root = dir.path();
        let contract = root.join("contract.md");
        std::fs::write(&contract, "original").unwrap();
        let state = root.join(".cairn/state/contract-baselines.json");
        std::fs::write(&state, "{\"version\":1,\"nodes\":{}}").unwrap();

        let backup = read_baseline_file(root).unwrap().expect("snapshot");
        // The commit steps run: both files are overwritten.
        std::fs::write(&contract, "installed").unwrap();
        std::fs::write(&state, "{\"version\":1,\"nodes\":{\"app\":{}}}").unwrap();

        rollback(&contract, "original", root, Some(&backup)).unwrap();

        assert_eq!(std::fs::read_to_string(&contract).unwrap(), "original");
        assert_eq!(std::fs::read(&state).unwrap(), backup);
    }

    #[test]
    fn rollback_deletes_a_baseline_file_the_call_created() {
        let dir = temp_root();
        let root = dir.path();
        let contract = root.join("contract.md");
        std::fs::write(&contract, "original").unwrap();

        let backup = read_baseline_file(root).unwrap();
        assert_eq!(backup, None, "no baseline file exists yet");
        std::fs::write(&contract, "installed").unwrap();
        let state = root.join(".cairn/state/contract-baselines.json");
        std::fs::write(&state, "{\"version\":1,\"nodes\":{\"app\":{}}}").unwrap();

        rollback(&contract, "original", root, backup.as_deref()).unwrap();

        assert_eq!(std::fs::read_to_string(&contract).unwrap(), "original");
        assert!(!state.exists(), "a file this call created must be removed");
    }

    #[test]
    fn rollback_is_idempotent_when_nothing_was_written() {
        let dir = temp_root();
        let root = dir.path();
        let contract = root.join("contract.md");
        std::fs::write(&contract, "original").unwrap();

        rollback(&contract, "original", root, None).unwrap();
        rollback(&contract, "original", root, None).unwrap();

        assert_eq!(std::fs::read_to_string(&contract).unwrap(), "original");
    }
}
