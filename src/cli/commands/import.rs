//! CLI import-openspec command implementation.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;

/// Migrate legacy openspec changes to the meta/changes directory.
pub(crate) fn run_import_openspec(root: &Path, json: bool) -> CliResult {
    let changes_dir = root.join("meta/changes");
    let meta_dir = root.join("meta/changes");

    if !changes_dir.exists() {
        return err(1, "no meta/changes directory found");
    }

    if let Err(error) = fs::create_dir_all(&meta_dir) {
        return err(1, &format!("failed to create meta/changes: {error}"));
    }

    let mut migrated = Vec::new();
    let mut copied_archive = false;

    let entries = match fs::read_dir(&changes_dir) {
        Ok(entries) => entries,
        Err(error) => return err(1, &format!("failed to read meta/changes: {error}")),
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(error) => {
                eprintln!("warning: failed to read directory entry: {error}");
                continue;
            }
        };

        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        // Skip hidden files and the archive directory (handled separately).
        if name_str.starts_with('.') || name_str == "archive" {
            continue;
        }

        let source = entry.path();
        let target = meta_dir.join(&name);

        if !source.is_dir() {
            continue;
        }

        // Copy directory recursively.
        if let Err(error) = copy_dir_all(&source, &target) {
            return err(1, &format!("failed to copy {name_str}: {error}"));
        }

        migrated.push(name_str.into_owned());
    }

    // Copy archive directory if it exists.
    let archive_source = changes_dir.join("archive");
    if archive_source.exists() {
        let archive_target = meta_dir.join("archive");
        if let Err(error) = copy_dir_all(&archive_source, &archive_target) {
            return err(1, &format!("failed to copy archive: {error}"));
        }
        copied_archive = true;
    }

    if json {
        let response = serde_json::json!({
            "command": "import-openspec",
            "status": "ok",
            "data": {
                "migrated": migrated,
                "archive_copied": copied_archive,
            }
        });
        return ok(format!("{response}\n"));
    }

    let mut out = format!("migrated {} phase(s)\n", migrated.len());
    for name in &migrated {
        let _ = std::fmt::Write::write_str(&mut out, &format!("  {name}\n"));
    }
    if copied_archive {
        let _ = std::fmt::Write::write_str(&mut out, "archive copied\n");
    }
    ok(out)
}

fn copy_dir_all(source: impl AsRef<Path>, target: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&target)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let target_path = target.as_ref().join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(entry.path(), &target_path)?;
        } else {
            fs::copy(entry.path(), &target_path)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_dir_all_creates_target_and_copies_files() {
        let source = tempfile::tempdir().unwrap();
        let target = tempfile::tempdir().unwrap();

        fs::create_dir_all(source.path().join("sub")).unwrap();
        fs::write(source.path().join("a.txt"), "hello").unwrap();
        fs::write(source.path().join("sub/b.txt"), "world").unwrap();

        copy_dir_all(source.path(), target.path()).unwrap();

        assert!(target.path().join("sub").is_dir());
        assert_eq!(
            fs::read_to_string(target.path().join("a.txt")).unwrap(),
            "hello"
        );
        assert_eq!(
            fs::read_to_string(target.path().join("sub/b.txt")).unwrap(),
            "world"
        );
    }

    #[test]
    fn copy_dir_all_copies_empty_directory() {
        let source = tempfile::tempdir().unwrap();
        let target = tempfile::tempdir().unwrap();

        copy_dir_all(source.path(), target.path()).unwrap();

        assert!(target.path().is_dir());
        let entries: Vec<_> = fs::read_dir(target.path()).unwrap().collect();
        assert!(entries.is_empty());
    }

    #[test]
    fn copy_dir_all_overwrites_existing_file_in_target() {
        let source = tempfile::tempdir().unwrap();
        let target = tempfile::tempdir().unwrap();

        fs::write(source.path().join("file.txt"), "new").unwrap();
        fs::write(target.path().join("file.txt"), "old").unwrap();

        copy_dir_all(source.path(), target.path()).unwrap();

        assert_eq!(
            fs::read_to_string(target.path().join("file.txt")).unwrap(),
            "new"
        );
    }
}
