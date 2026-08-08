//! Git plumbing for the ratification range gate.

// Reason: the parent module owns the finding shapes these helpers feed.
#![allow(clippy::wildcard_imports)]
use std::{collections::BTreeSet, path::Path, process::Command};

use super::*;

/// True when `root` sits inside a Git work tree.
///
/// The discriminator for silence is "nothing can be committed here", never
/// "the worktree shows no local decision": the candidate tree is exactly what
/// a worktree is free to contradict.
pub(super) fn inside_work_tree(root: &Path) -> bool {
    git_output(root, ["rev-parse", "--is-inside-work-tree"])
        .is_some_and(|value| value.trim() == "true")
}
const BLUEPRINT_PATH: &str = "cairn.blueprint";

/// Repository-relative paths of decisions the CANDIDATE tree accepts at tier
/// local: the index in pre-commit mode, `HEAD` in CI mode.
///
/// Reading the candidate rather than the worktree is what makes the trigger
/// honest. `None` means Git could not be questioned, which the caller treats
/// as a refusal.
pub(super) fn candidate_accepted_local(
    root: &Path,
    mode: RatificationMode,
) -> Option<BTreeSet<String>> {
    let candidate_pointers = normalise_pointers(&candidate_decision_pointers(root, mode)?)?;
    for pointer in &candidate_pointers {
        if candidate_pointer_contains_symlink(root, pointer, mode)? {
            return None;
        }
    }
    if candidate_pointers.is_empty() {
        return Some(BTreeSet::new());
    }
    let pathspecs = candidate_pointers
        .iter()
        .map(|pointer| literal_pathspec(pointer))
        .collect::<Vec<_>>();
    let mut args = match mode {
        RatificationMode::Index => vec!["ls-files", "-z", "--"],
        RatificationMode::Head => vec!["ls-tree", "-r", "--name-only", "-z", "HEAD", "--"],
    };
    args.extend(pathspecs.iter().map(String::as_str));
    let listing = git_output(root, args)?;
    let mut accepted = BTreeSet::new();
    for path in listing.split('\0').filter(|path| !path.is_empty()) {
        let spec = match mode {
            RatificationMode::Index => format!(":{path}"),
            RatificationMode::Head => format!("HEAD:{path}"),
        };
        let Some(raw) = git_output(root, ["show", &spec]) else {
            continue;
        };
        let frontmatter = crate::artefacts::frontmatter::parse(&raw);
        if matches!(
            (
                frontmatter.values.get("status").map(String::as_str),
                frontmatter.values.get("ratification").map(String::as_str),
            ),
            (Some("accepted"), Some("local"))
        ) {
            accepted.insert(path.to_owned());
        }
    }
    Some(accepted)
}

fn candidate_decision_pointers(root: &Path, mode: RatificationMode) -> Option<Vec<String>> {
    let spec = match mode {
        RatificationMode::Index => format!(":{BLUEPRINT_PATH}"),
        RatificationMode::Head => format!("HEAD:{BLUEPRINT_PATH}"),
    };
    let source = git_output(root, ["show", &spec])?;
    let ast = crate::blueprint::parser::parse_str(BLUEPRINT_PATH, &source).ok()?;
    Some(crate::artefacts::registry::decision_pointers(&ast))
}

pub(super) fn candidate_pointer_configuration_matches(
    root: &Path,
    worktree_pointers: &[String],
    mode: RatificationMode,
) -> Option<bool> {
    let worktree = normalise_pointers(worktree_pointers)?;
    let candidate = normalise_pointers(&candidate_decision_pointers(root, mode)?)?;
    Some(worktree == candidate)
}

fn normalise_pointers(raw: &[String]) -> Option<Vec<String>> {
    let mut pointers = raw
        .iter()
        .map(|pointer| crate::artefacts::registry::manifest::normalise_repo_pointer(pointer))
        .collect::<Option<Vec<_>>>()?;
    pointers.sort();
    pointers.dedup();
    Some(pointers)
}

fn literal_pathspec(pointer: &str) -> String {
    format!(":(top,literal){pointer}")
}

fn candidate_pointer_contains_symlink(
    root: &Path,
    pointer: &str,
    mode: RatificationMode,
) -> Option<bool> {
    let mut prefix = String::new();
    for component in pointer.split('/') {
        if !prefix.is_empty() {
            prefix.push('/');
        }
        prefix.push_str(component);
        let pathspec = literal_pathspec(&prefix);
        let mut args = match mode {
            RatificationMode::Index => vec!["ls-files", "-s", "-z", "--"],
            RatificationMode::Head => vec!["ls-tree", "-z", "--full-tree", "HEAD", "--"],
        };
        args.push(pathspec.as_str());
        let listing = git_output(root, args)?;
        let found = listing.split('\0').any(|entry| {
            if mode == RatificationMode::Index {
                let Some((metadata, path)) = entry.split_once('\t') else {
                    return false;
                };
                metadata.starts_with("120000 ") && path == prefix
            } else {
                entry.starts_with("120000 ")
            }
        });
        if found {
            return Some(true);
        }
    }
    Some(false)
}
pub(super) fn changed_paths(
    root: &Path,
    base: &str,
    mode: RatificationMode,
) -> Option<BTreeSet<String>> {
    // `-z` is mandatory: without it Git C-quotes paths holding control or
    // non-ASCII bytes, the quoted spelling never equals the artefact path, and
    // the whole ratification gate silently skips that decision.
    let args = match mode {
        RatificationMode::Index => vec![
            "diff",
            "-z",
            "--name-only",
            "--no-renames",
            "--cached",
            base,
        ],
        RatificationMode::Head => vec!["diff", "-z", "--name-only", "--no-renames", base, "HEAD"],
    };
    let output = git_output(root, args)?;
    Some(
        output
            .split('\0')
            .filter(|path| !path.is_empty())
            .map(str::to_owned)
            .collect(),
    )
}
pub(super) fn decision_was_not_local(root: &Path, base: &str, decision: &Decision) -> bool {
    let path = repository_path(root, &decision.path);
    let Some(raw) = git_output(root, ["show", &format!("{base}:{path}")]) else {
        return true;
    };
    let frontmatter = crate::artefacts::frontmatter::parse(&raw);
    !matches!(
        (
            frontmatter.values.get("status").map(String::as_str),
            frontmatter.values.get("ratification").map(String::as_str),
        ),
        (Some("accepted"), Some("local"))
    )
}
pub(super) fn git_output<'a>(
    root: &Path,
    args: impl IntoIterator<Item = &'a str>,
) -> Option<String> {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8(output.stdout).ok())
        .flatten()
}
