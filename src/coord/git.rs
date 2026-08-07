//! Resolves the git directory a worktree family has in common.
//!
//! `git rev-parse --git-common-dir` is the one resolution that lands every
//! worktree of a checkout family on the same directory; `--git-path` resolves
//! unrecognised paths against the per-worktree gitdir and would give every
//! worktree a private store (see `todo.coord-common-dir-helper`).

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::copy;

/// Resolves the git common directory for `root`.
///
/// A secondary worktree reports its family's shared `.git` directory; a
/// primary checkout reports its own. A relative result is joined onto
/// `root`, mirroring the read-only subprocess discipline of the hook
/// installer.
///
/// # Errors
///
/// Returns the shared `hooks.git-error` copy when git cannot be spawned or
/// exits non-zero (not a repository, git missing).
pub fn git_common_dir(root: &Path) -> Result<PathBuf, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["rev-parse", "--git-common-dir"])
        .output()
        .map_err(|error| format!("{}: {error}", copy::lookup("hooks.git-error")))?;
    if !output.status.success() {
        return Err(copy::lookup("hooks.git-error").to_owned());
    }
    let path = PathBuf::from(String::from_utf8_lossy(&output.stdout).trim().to_owned());
    Ok(if path.is_absolute() {
        path
    } else {
        root.join(path)
    })
}

/// Resolves the per-worktree git directory: `.git` itself in a primary
/// checkout, the directory a worktree's `.git` file names otherwise.
fn worktree_git_dir(root: &Path) -> Option<PathBuf> {
    let dot_git = root.join(".git");
    if dot_git.is_dir() {
        return Some(dot_git);
    }
    let pointer = std::fs::read_to_string(&dot_git).ok()?;
    let target = pointer.strip_prefix("gitdir:")?.trim();
    let path = PathBuf::from(target);
    Some(if path.is_absolute() {
        path
    } else {
        root.join(path)
    })
}

/// Reads the commit hash `root`'s checkout points at, without a subprocess
/// for the common case.
///
/// HEAD is per-worktree; the ref it names lives in the family's shared
/// common dir. Packed refs and detached HEAD return `None`, so callers keep
/// their subprocess fallback.
pub(crate) fn head_hash(root: &Path) -> Option<String> {
    let head_ref = std::fs::read_to_string(worktree_git_dir(root)?.join("HEAD")).ok()?;
    let ref_path = head_ref.trim().strip_prefix("ref: ")?;
    let common = git_common_dir(root).ok()?;
    let hash = std::fs::read_to_string(common.join(ref_path)).ok()?;
    Some(hash.trim().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn git(dir: &Path, args: &[&str]) {
        let status = Command::new("git")
            .arg("-C")
            .arg(dir)
            .args(args)
            .output()
            .expect("git runs");
        assert!(
            status.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&status.stderr)
        );
    }

    fn init_with_commit(dir: &Path) {
        git(dir, &["init", "-q"]);
        git(
            dir,
            &[
                "-c",
                "user.name=t",
                "-c",
                "user.email=t@t",
                "commit",
                "--allow-empty",
                "-q",
                "-m",
                "x",
            ],
        );
    }

    #[test]
    fn primary_checkout_resolves_its_own_git_dir() {
        let root = tempfile::TempDir::new().expect("tempdir");
        init_with_commit(root.path());
        let common = git_common_dir(root.path()).expect("common dir");
        assert_eq!(
            std::fs::canonicalize(&common).expect("canonical common"),
            std::fs::canonicalize(root.path().join(".git")).expect("canonical .git")
        );
    }

    #[test]
    fn secondary_worktree_resolves_the_shared_common_dir() {
        let root = tempfile::TempDir::new().expect("tempdir");
        init_with_commit(root.path());
        let wt = root.path().join("wt");
        git(
            root.path(),
            &["worktree", "add", "-q", wt.to_str().unwrap()],
        );
        let from_worktree = git_common_dir(&wt).expect("worktree common dir");
        assert_eq!(
            std::fs::canonicalize(&from_worktree).expect("canonical worktree common"),
            std::fs::canonicalize(root.path().join(".git")).expect("canonical primary .git")
        );
    }

    fn rev_parse_head(dir: &Path) -> String {
        let output = Command::new("git")
            .arg("-C")
            .arg(dir)
            .args(["rev-parse", "HEAD"])
            .output()
            .expect("git runs");
        assert!(output.status.success());
        String::from_utf8_lossy(&output.stdout).trim().to_owned()
    }

    #[test]
    fn head_hash_resolves_the_worktree_head_not_the_primary_head() {
        let root = tempfile::TempDir::new().expect("tempdir");
        init_with_commit(root.path());
        let wt = root.path().join("wt");
        git(
            root.path(),
            &["worktree", "add", "-q", wt.to_str().unwrap()],
        );
        // Advance the worktree branch so its HEAD differs from the
        // primary's: resolving `<common>/HEAD` instead of the worktree's
        // own HEAD would return the wrong hash here.
        git(
            &wt,
            &[
                "-c",
                "user.name=t",
                "-c",
                "user.email=t@t",
                "commit",
                "--allow-empty",
                "-q",
                "-m",
                "worktree-only",
            ],
        );

        assert_eq!(head_hash(&wt).expect("worktree head"), rev_parse_head(&wt));
        assert_eq!(
            head_hash(root.path()).expect("primary head"),
            rev_parse_head(root.path())
        );
        assert_ne!(
            head_hash(&wt),
            head_hash(root.path()),
            "the two checkouts are on different commits"
        );
    }

    #[test]
    fn non_repository_errors_with_the_shared_copy() {
        let root = tempfile::TempDir::new().expect("tempdir");
        let error = git_common_dir(root.path()).expect_err("not a repository");
        assert_eq!(error, copy::lookup("hooks.git-error"));
    }
}
