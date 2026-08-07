//! Epoch-succession exclusion tokens.
//!
//! The two records needing mutual exclusion (the driver singleton and a unit
//! lease grant) acquire by reading the highest `epoch-NNNNNN.json` and
//! creating the successor with `create_new`, so exactly one writer wins.
//! Tokens are never deleted: release is a fact, and a crashed holder's token
//! stays as the audit trail while staleness is reader-derived.

use std::io::Write;
use std::path::Path;

/// Bounded retries against fresh maxima before giving up.
const MAX_ATTEMPTS: u32 = 16;

/// Highest existing epoch number under `dir`, or 0 when none.
fn highest_epoch(dir: &Path) -> Result<u64, String> {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        Err(error) => return Err(format!("cannot list epoch dir: {error}")),
    };
    let mut highest = 0;
    for entry in entries {
        let entry = entry.map_err(|error| format!("cannot read epoch entry: {error}"))?;
        let name = entry.file_name();
        let Some(name) = name.to_str() else { continue };
        if let Some(number) = name
            .strip_prefix("epoch-")
            .and_then(|rest| rest.strip_suffix(".json"))
            .and_then(|digits| digits.parse::<u64>().ok())
        {
            highest = highest.max(number);
        }
    }
    Ok(highest)
}

/// Acquires the next epoch token under `dir`, creating the directory when
/// absent. Returns the epoch number won.
///
/// # Errors
///
/// Returns an error when the directory cannot be created or listed, or when
/// `MAX_ATTEMPTS` successive creations lose the `create_new` race.
pub fn acquire_epoch(dir: &Path) -> Result<u64, String> {
    std::fs::create_dir_all(dir).map_err(|error| format!("cannot create epoch dir: {error}"))?;
    for _ in 0..MAX_ATTEMPTS {
        let next = highest_epoch(dir)? + 1;
        let path = dir.join(format!("epoch-{next:06}.json"));
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(mut file) => {
                let _ = file.write_all(b"{}\n");
                return Ok(next);
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(format!("cannot create epoch token: {error}")),
        }
    }
    Err(format!(
        "lost the epoch race {MAX_ATTEMPTS} times in a row; a writer storm is in progress"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concurrent_acquisitions_produce_one_winner_per_epoch() {
        let dir = tempfile::TempDir::new().expect("tempdir");
        let path = dir.path().join("singleton");
        let barrier = std::sync::Barrier::new(2);
        let epochs = std::thread::scope(|scope| {
            let handles = [
                scope.spawn(|| {
                    barrier.wait();
                    acquire_epoch(&path).expect("acquire a")
                }),
                scope.spawn(|| {
                    barrier.wait();
                    acquire_epoch(&path).expect("acquire b")
                }),
            ];
            handles.map(|handle| handle.join().expect("thread joins"))
        });
        let mut sorted = epochs;
        sorted.sort_unstable();
        assert_eq!(sorted.to_vec(), vec![1, 2], "each epoch has one winner");
        assert!(path.join("epoch-000001.json").is_file());
        assert!(path.join("epoch-000002.json").is_file());
    }

    #[test]
    fn succession_continues_from_the_highest_token() {
        let dir = tempfile::TempDir::new().expect("tempdir");
        let path = dir.path().join("leases/todo.x");
        assert_eq!(acquire_epoch(&path).expect("first"), 1);
        assert_eq!(acquire_epoch(&path).expect("second"), 2);
        assert_eq!(acquire_epoch(&path).expect("third"), 3);
    }
}
