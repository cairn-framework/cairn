//! Shared file-persistence helpers: atomic writes and versioned JSON state files.
//!
//! Centralises the small, repeated patterns for reading/writing state files
//! that were previously hand-rolled in the scanner, summariser, changes, and
//! workspace modules.

use std::{
    fs,
    io::{self, Write},
    path::Path,
};

/// Writes `content` to `path` atomically by creating a temporary file in the
/// same directory and renaming it into place.
///
/// Creates parent directories if they do not exist. The temporary file is
/// removed whether the write succeeds or fails.
///
/// # Errors
///
/// Returns an I/O error when the temporary file cannot be written, renamed,
/// or the parent directory cannot be created.
pub fn atomic_write(path: &Path, content: &str) -> io::Result<()> {
    atomic_write_bytes(path, content.as_bytes())
}

/// Writes `content` to `path` atomically as raw bytes.
///
/// Creates parent directories if they do not exist. The temporary file is
/// removed whether the write succeeds or fails.
///
/// # Errors
///
/// Returns an I/O error when the temporary file cannot be written, renamed,
/// or the parent directory cannot be created.
pub fn atomic_write_bytes(path: &Path, content: &[u8]) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let permissions = fs::metadata(path)
        .ok()
        .map(|metadata| metadata.permissions());
    #[cfg(unix)]
    let builder = {
        let mut builder = tempfile::Builder::new();
        builder.permissions(std::os::unix::fs::PermissionsExt::from_mode(0o666));
        builder
    };
    #[cfg(not(unix))]
    let builder = tempfile::Builder::new();
    let mut tmp = builder.tempfile_in(parent)?;
    tmp.write_all(content)?;
    #[cfg(not(windows))]
    if let Some(permissions) = permissions.clone() {
        tmp.as_file().set_permissions(permissions)?;
    }
    tmp.persist(path).map(|_| ()).map_err(|error| error.error)?;
    #[cfg(windows)]
    if let Some(permissions) = permissions {
        fs::set_permissions(path, permissions)?;
    }
    Ok(())
}

/// Serialises `value` as pretty-printed JSON and writes it atomically to `path`.
///
/// A trailing newline is appended so the on-disk format matches the project's
/// text-file conventions. When the file already holds identical content the
/// write is skipped, preserving the file's inode and mtime (state files are
/// rewritten every scan; watch mode would otherwise churn them on every tick).
///
/// # Errors
///
/// Returns an I/O error when serialisation fails or the file cannot be written.
pub fn write_json<T: serde::Serialize>(path: &Path, value: &T) -> io::Result<()> {
    let mut body = serde_json::to_string_pretty(value).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("failed to serialise {}: {error}", path.display()),
        )
    })?;
    body.push('\n');
    if matches!(fs::read_to_string(path), Ok(existing) if existing == body) {
        return Ok(());
    }
    atomic_write(path, &body)
}

/// Reads `path` and deserialises it as JSON.
///
/// # Errors
///
/// Returns an I/O error when the file cannot be read or the contents cannot be
/// parsed as JSON. Parse failures carry [`io::ErrorKind::InvalidData`].
pub fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> io::Result<T> {
    let content = fs::read_to_string(path)?;
    parse_json(&content, path)
}

/// Deserialises already-read JSON `content`, labelling errors with `path`.
///
/// Use this instead of [`read_json`] when the file has already been read (for
/// example after [`read_versioned_json`]), so the version peek and the payload
/// parse operate on the same bytes.
///
/// # Errors
///
/// Returns [`io::ErrorKind::InvalidData`] when the contents cannot be parsed.
pub fn parse_json<T: serde::de::DeserializeOwned>(content: &str, path: &Path) -> io::Result<T> {
    serde_json::from_str(content).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("failed to parse {}: {error}", path.display()),
        )
    })
}

/// Reads `path` and deserialises it as TOML.
///
/// # Errors
///
/// Returns an I/O error when the file cannot be read or the contents cannot be
/// parsed as TOML. Parse failures carry [`io::ErrorKind::InvalidData`].
pub fn read_toml<T: serde::de::DeserializeOwned>(path: &Path) -> io::Result<T> {
    let content = fs::read_to_string(path)?;
    toml::from_str(&content).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("failed to parse {}: {error}", path.display()),
        )
    })
}

#[derive(serde::Deserialize)]
struct VersionOnly {
    version: u32,
}

/// Reads a versioned JSON state file once, returning its top-level `version`
/// and the raw content for a subsequent [`parse_json`] against the same bytes.
///
/// Returns `Ok(None)` when the file does not exist.
///
/// # Errors
///
/// Returns an I/O error when the file exists but cannot be read or does not
/// contain a valid top-level `version` integer.
pub fn read_versioned_json(path: &Path) -> io::Result<Option<(u32, String)>> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };
    let version = serde_json::from_str::<VersionOnly>(&content)
        .map(|value| value.version)
        .map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("failed to parse version from {}: {error}", path.display()),
            )
        })?;
    Ok(Some((version, content)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atomic_write_creates_parent_dirs_and_replaces_content() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("file.txt");
        atomic_write(&path, "first").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "first");
        atomic_write(&path, "second").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "second");
    }

    #[test]
    fn atomic_write_leaves_no_temp_file_after_rename() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("out.json");
        atomic_write(&path, "content").unwrap();
        for entry in fs::read_dir(dir.path()).unwrap() {
            let name = entry.unwrap().file_name().to_string_lossy().into_owned();
            assert!(
                std::path::Path::new(&name).extension() != Some(std::ffi::OsStr::new("tmp")),
                "temporary file must be removed after rename"
            );
        }
    }

    #[test]
    fn write_json_and_read_json_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("data.json");
        let value = serde_json::json!({"version": 1, "items": ["a", "b"]});
        write_json(&path, &value).unwrap();
        let back: serde_json::Value = read_json(&path).unwrap();
        assert_eq!(back, value);
    }

    #[test]
    fn write_json_emits_pretty_body_with_trailing_newline() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("data.json");
        let value = serde_json::json!({"version": 1, "items": ["a"]});
        write_json(&path, &value).unwrap();
        let on_disk = fs::read_to_string(&path).unwrap();
        let expected = serde_json::to_string_pretty(&value).unwrap() + "\n";
        assert_eq!(on_disk, expected);
    }

    #[test]
    fn write_json_skips_rewrite_when_content_is_identical() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("data.json");
        let value = serde_json::json!({"version": 1});
        write_json(&path, &value).unwrap();
        let before = fs::metadata(&path).unwrap().modified().unwrap();
        // A rewrite with identical content must not replace the file.
        std::thread::sleep(std::time::Duration::from_millis(20));
        write_json(&path, &value).unwrap();
        let after = fs::metadata(&path).unwrap().modified().unwrap();
        assert_eq!(before, after, "identical content must skip the rewrite");
    }

    #[test]
    fn read_json_malformed_input_is_invalid_data() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.json");
        fs::write(&path, "not json").unwrap();
        let error = read_json::<serde_json::Value>(&path).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn read_toml_malformed_input_is_invalid_data() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.toml");
        fs::write(&path, "= broken").unwrap();
        let error = read_toml::<toml::Value>(&path).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn read_versioned_json_returns_version_and_content() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("state.json");
        let value = serde_json::json!({"version": 7, "nodes": {"a": {"deep": "nesting"}}});
        write_json(&path, &value).unwrap();
        let (version, content) = read_versioned_json(&path).unwrap().unwrap();
        assert_eq!(version, 7);
        let back: serde_json::Value = parse_json(&content, &path).unwrap();
        assert_eq!(back, value);
    }

    #[test]
    fn read_versioned_json_missing_file_returns_none() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("missing.json");
        assert!(read_versioned_json(&path).unwrap().is_none());
    }

    #[test]
    fn read_versioned_json_non_integer_version_is_invalid_data() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("state.json");
        fs::write(&path, "{\"version\": \"x\"}").unwrap();
        let error = read_versioned_json(&path).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn read_toml_parses_document() {
        #[derive(serde::Deserialize)]
        struct Cfg {
            name: String,
        }
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");
        fs::write(&path, "name = \"x\"\n").unwrap();
        let cfg: Cfg = read_toml(&path).unwrap();
        assert_eq!(cfg.name, "x");
    }
}
