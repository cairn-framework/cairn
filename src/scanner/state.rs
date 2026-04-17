//! Machine state persistence.

use std::{fs, io, path::Path};

/// Writes interface hash state JSON.
///
/// # Errors
///
/// Returns an I/O error when the state directory or JSON file cannot be written.
pub fn write_interface_hash(root: &Path, hash: &str) -> io::Result<()> {
    let dir = root.join(".cairn/state");
    fs::create_dir_all(&dir)?;
    fs::write(
        dir.join("interface-hashes.json"),
        format!("{{\n  \"rust-code\": \"{hash}\"\n}}\n"),
    )
}
