//! The scratch workspace, and the one definition of a usable response path.
//!
//! The checked-in fixture is never the workspace. Every run copies it into a
//! fresh scratch directory and mutates only the copy, so a run can never leave
//! the fixture different from how it found it. The directory is owned by a
//! `TempDir`, so it is created uniquely and removed on drop; nothing here ever
//! deletes a guessed path.
//!
//! [`canonical_relative`] is the single authority on whether a path is usable
//! and on how two spellings of one path compare. Everything that reasons about
//! response paths goes through it: the prompt's expected paths, the runner's
//! coverage check, and the writes themselves. A second, slightly different
//! notion of validity anywhere else turns a backend's malformed path into a
//! failed run rather than the recorded protocol violation it is.

use std::fs;

use camino::{Utf8Path, Utf8PathBuf};
use tempfile::TempDir;

use super::backend::FileEdit;
use crate::error::CairnError;

/// Validates a workspace-relative file path and returns its canonical spelling.
///
/// The canonical form is the path's components joined by `/`, so redundant
/// separators compare equal. Everything else is rejected, with the reason:
/// empty, NUL-bearing, absolute, `..`-bearing, `.`-bearing, directory-shaped,
/// or naming no file at all.
///
/// # Errors
///
/// Returns the human-readable reason the path is unusable.
pub(crate) fn canonical_relative(path: &str) -> Result<String, String> {
    if path.is_empty() {
        return Err("is empty".to_owned());
    }
    if path.contains('\0') {
        return Err("contains a NUL byte".to_owned());
    }
    if path.ends_with('/') || path.ends_with('\\') {
        return Err("is directory-shaped".to_owned());
    }
    if path.starts_with('/') || path.starts_with('\\') || Utf8Path::new(path).is_absolute() {
        return Err("is absolute".to_owned());
    }
    // A Windows drive-relative path such as `C:out.md` is NOT absolute, yet
    // joining it onto the workspace root replaces the root outright. Checked on
    // the raw string rather than through `Utf8Component::Prefix`, which only
    // parses on Windows: the rule has to hold wherever this runs, or the hole
    // is invisible to every test on a Unix CI machine.
    if drive_prefixed(path) {
        return Err("escapes the scratch workspace".to_owned());
    }

    // Split the raw string rather than using `Utf8Path::components`, which
    // silently drops interior `.` segments: `a/./b` would then look canonical
    // and pass a check the writes later reject. Both separators are treated as
    // separators so a Windows-style spelling cannot smuggle one past.
    let mut segments = Vec::new();
    for segment in path.split(['/', '\\']) {
        match segment {
            "" => {}
            "." => return Err("is not a plain relative file path".to_owned()),
            ".." => return Err("escapes the scratch workspace".to_owned()),
            other => segments.push(other),
        }
    }
    if segments.is_empty() {
        return Err("names no file".to_owned());
    }
    Ok(segments.join("/"))
}

/// Whether `path` opens with a Windows drive letter, as in `C:` or `c:out.md`.
fn drive_prefixed(path: &str) -> bool {
    let mut chars = path.chars();
    chars.next().is_some_and(char::is_alphabetic) && chars.next() == Some(':')
}

/// A scratch copy of the fixture, removed when dropped.
#[derive(Debug)]
pub(crate) struct Workspace {
    // Reason: held for its Drop impl, which removes the scratch directory.
    _dir: TempDir,
    root: Utf8PathBuf,
}

impl Workspace {
    /// Copies `fixture` into a fresh scratch directory.
    pub(crate) fn from_fixture(fixture: &Utf8Path) -> Result<Self, CairnError> {
        if !fixture.is_dir() {
            return Err(CairnError::AuthorEval {
                message: format!("fixture `{fixture}` is not a directory"),
            });
        }

        let dir = tempfile::Builder::new()
            .prefix("cairn-authoreval-")
            .tempdir()
            .map_err(|e| CairnError::AuthorEval {
                message: format!("failed to create a scratch workspace: {e}"),
            })?;

        let root = Utf8PathBuf::from_path_buf(dir.path().to_path_buf()).map_err(|path| {
            CairnError::AuthorEval {
                message: format!("scratch workspace `{}` is not utf-8", path.display()),
            }
        })?;

        copy_dir(fixture, &root)?;
        Ok(Self { _dir: dir, root })
    }

    /// The workspace root.
    pub(crate) fn root(&self) -> &Utf8Path {
        &self.root
    }

    /// Validates the whole batch and resolves it to workspace targets.
    ///
    /// This is the only place a response's paths are judged, and every
    /// rejection is a fact about the response rather than about the
    /// instrument, so the caller records it as a protocol failure. Returned
    /// pairs are the canonical relative spelling and the absolute target, in
    /// input order.
    ///
    /// # Errors
    ///
    /// Returns the human-readable reason the batch is unusable.
    pub(crate) fn validate(
        &self,
        files: &[FileEdit],
    ) -> Result<Vec<(String, Utf8PathBuf)>, String> {
        let mut resolved: Vec<(String, Utf8PathBuf)> = Vec::with_capacity(files.len());

        for file in files {
            let relative = canonical_relative(&file.path)
                .map_err(|reason| format!("path `{}` {reason}", file.path))?;

            // Two edits resolving to one destination make the batch ambiguous:
            // whichever came last would silently win, and the record would
            // claim both were authored.
            if resolved.iter().any(|(seen, _)| *seen == relative) {
                return Err(format!("path `{relative}` is written more than once"));
            }

            // One target nested under another cannot both be a file and a
            // directory. Caught here rather than mid-write, where it would
            // half-apply the batch.
            if let Some((clash, _)) = resolved
                .iter()
                .find(|(seen, _)| nests(seen, &relative) || nests(&relative, seen))
            {
                return Err(format!(
                    "path `{relative}` and path `{clash}` cannot both exist"
                ));
            }

            let target = self.resolve(&relative)?;
            resolved.push((relative, target));
        }

        Ok(resolved)
    }

    /// Writes a batch that [`Workspace::validate`] already accepted.
    ///
    /// Validation, not the writes themselves, is the atomic part. A later I/O
    /// failure (permissions, a full disk) can leave the batch half applied.
    /// That is not worth a staging layer: the caller turns any error from here
    /// into a failed run, no record is emitted, and the scratch workspace is
    /// dropped, so half-applied state is never observed.
    pub(crate) fn write(
        files: &[FileEdit],
        targets: &[(String, Utf8PathBuf)],
    ) -> Result<(), CairnError> {
        for (file, (_, target)) in files.iter().zip(targets) {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent).map_err(|e| CairnError::AuthorEval {
                    message: format!("failed to create `{parent}`: {e}"),
                })?;
            }
            fs::write(target, &file.contents).map_err(|e| CairnError::AuthorEval {
                message: format!("failed to write `{target}`: {e}"),
            })?;
        }
        Ok(())
    }
}

impl Workspace {
    /// Resolves one canonical path, checking the tree can hold it.
    fn resolve(&self, relative: &str) -> Result<Utf8PathBuf, String> {
        let target = self.root.join(relative);
        if target.is_dir() {
            return Err(format!("path `{relative}` is an existing directory"));
        }

        // `write` creates parent directories. An ancestor that already exists
        // as a file makes that fail partway through the batch.
        let segments: Vec<&str> = relative.split('/').collect();
        let mut walked = self.root.clone();
        for segment in &segments[..segments.len() - 1] {
            walked = walked.join(segment);
            if walked.exists() && !walked.is_dir() {
                return Err(format!(
                    "path `{relative}` has an ancestor that already exists as a file"
                ));
            }
        }
        Ok(target)
    }
}

/// Whether `parent` names a directory prefix of `child`.
fn nests(parent: &str, child: &str) -> bool {
    child.len() > parent.len()
        && child.starts_with(parent)
        && child.as_bytes().get(parent.len()) == Some(&b'/')
}

fn copy_dir(from: &Utf8Path, to: &Utf8Path) -> Result<(), CairnError> {
    fs::create_dir_all(to).map_err(|e| CairnError::AuthorEval {
        message: format!("failed to create `{to}`: {e}"),
    })?;

    let entries = fs::read_dir(from).map_err(|e| CairnError::AuthorEval {
        message: format!("failed to read `{from}`: {e}"),
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| CairnError::AuthorEval {
            message: format!("failed to read an entry of `{from}`: {e}"),
        })?;
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            return Err(CairnError::AuthorEval {
                message: format!("fixture entry under `{from}` has a non-utf-8 name"),
            });
        };

        let source = from.join(name);
        let target = to.join(name);

        // `file_type` from `read_dir` does not follow links. A symlinked
        // directory would otherwise be walked, copying content from outside the
        // fixture into the workspace, or recursing forever through a cycle.
        let file_type = entry.file_type().map_err(|e| CairnError::AuthorEval {
            message: format!("failed to read the type of `{source}`: {e}"),
        })?;
        if file_type.is_symlink() {
            return Err(CairnError::AuthorEval {
                message: format!("fixture entry `{source}` is a symlink; fixtures must be plain"),
            });
        }

        if file_type.is_dir() {
            copy_dir(&source, &target)?;
        } else {
            fs::copy(&source, &target).map_err(|e| CairnError::AuthorEval {
                message: format!("failed to copy `{source}`: {e}"),
            })?;
        }
    }
    Ok(())
}
