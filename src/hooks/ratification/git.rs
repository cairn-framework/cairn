//! Git plumbing for the ratification range gate.

// Reason: the parent module owns the finding shapes these helpers feed.
#![allow(clippy::wildcard_imports)]
use crate::artefacts::registry::manifest::{governed_canonical_files, parse_allowlist};
use std::{collections::BTreeSet, fs, path::Path, process::Command};

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
/// Repository-relative paths of decisions the CANDIDATE tree accepts at tier
/// local: the index in pre-commit mode, `HEAD` in CI mode.
///
/// Reading the candidate rather than the worktree is what makes the trigger
/// honest. `None` means Git could not be questioned, which the caller treats
/// as a refusal.
pub(super) fn candidate_accepted_local(
    root: &Path,
    raw_pointers: &[String],
    git_prefix: &str,
    mode: RatificationMode,
) -> Option<BTreeSet<String>> {
    let candidate_pointers = normalise_pointers(raw_pointers)?;
    for pointer in &candidate_pointers {
        let candidate_path = git_path(git_prefix, pointer);
        if candidate_pointer_contains_unsafe_entry(root, &candidate_path, mode)? {
            return None;
        }
    }
    if candidate_pointers.is_empty() {
        return Some(BTreeSet::new());
    }
    let candidate_paths = candidate_pointers
        .iter()
        .map(|pointer| git_path(git_prefix, pointer))
        .collect::<Vec<_>>();
    let listing = candidate_listing(root, &candidate_paths, mode)?;
    accepted_local_from_listing(root, &listing, &candidate_paths, git_prefix, mode)
}

fn candidate_listing(root: &Path, paths: &[String], mode: RatificationMode) -> Option<String> {
    let pathspecs = paths
        .iter()
        .map(|pointer| literal_pathspec(pointer))
        .collect::<Vec<_>>();
    let mut args = match mode {
        RatificationMode::Index => vec!["ls-files", "--full-name", "-z", "--"],
        RatificationMode::Head => vec![
            "ls-tree",
            "-r",
            "--full-tree",
            "--name-only",
            "-z",
            "HEAD",
            "--",
        ],
    };
    args.extend(pathspecs.iter().map(String::as_str));
    git_output(root, args)
}

fn accepted_local_from_listing(
    root: &Path,
    listing: &str,
    candidate_paths: &[String],
    git_prefix: &str,
    mode: RatificationMode,
) -> Option<BTreeSet<String>> {
    let mut accepted = BTreeSet::new();
    for git_path in listing.split('\0').filter(|path| !path.is_empty()) {
        let exact_pointer = candidate_paths.iter().any(|pointer| pointer == git_path);
        if !exact_pointer
            && Path::new(git_path)
                .extension()
                .is_none_or(|extension| extension != "md")
        {
            continue;
        }
        let path = strip_git_prefix(git_path, git_prefix)?;
        let spec = match mode {
            RatificationMode::Index => format!(":{git_path}"),
            RatificationMode::Head => format!("HEAD:{git_path}"),
        };
        let raw = git_output(root, ["show", &spec])?;
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

pub(super) fn candidate_decision_pointers(
    root: &Path,
    blueprint_path: &Path,
    mode: RatificationMode,
) -> Option<(Vec<String>, String)> {
    let (git_blueprint_path, git_prefix) = candidate_path_context(root, blueprint_path)?;
    let spec = match mode {
        RatificationMode::Index => format!(":{git_blueprint_path}"),
        RatificationMode::Head => format!("HEAD:{git_blueprint_path}"),
    };
    let source = git_output(root, ["show", &spec])?;
    let ast = crate::blueprint::parser::parse_str(&git_blueprint_path, &source).ok()?;
    Some((
        crate::artefacts::registry::decision_pointers(&ast),
        git_prefix,
    ))
}

pub(super) fn candidate_pointer_configuration_matches(
    worktree_pointers: &[String],
    candidate_pointers: &[String],
) -> Option<bool> {
    let worktree = normalise_pointers(worktree_pointers)?;
    let candidate = normalise_pointers(candidate_pointers)?;
    Some(worktree == candidate)
}

fn candidate_path_context(root: &Path, blueprint_path: &Path) -> Option<(String, String)> {
    let git_root = fs::canonicalize(Path::new(
        git_output(root, ["rev-parse", "--show-toplevel"])?.trim(),
    ))
    .ok()?;
    let canonical_root = fs::canonicalize(root).ok()?;
    let git_prefix = relative_path(&git_root, &canonical_root)?;
    let blueprint_relative = if blueprint_path.is_absolute() {
        let blueprint = lexical_normalize(blueprint_path);
        relative_path(&canonical_root, &blueprint)?
    } else {
        crate::artefacts::registry::manifest::normalise_repo_pointer(blueprint_path.to_str()?)?
    };
    let git_blueprint_path = crate::artefacts::registry::manifest::normalise_repo_pointer(
        &git_path(&git_prefix, &blueprint_relative),
    )?;
    Some((git_blueprint_path, git_prefix))
}

fn relative_path(base: &Path, path: &Path) -> Option<String> {
    path.strip_prefix(base)
        .ok()?
        .to_str()
        .map(|path| path.replace('\\', "/"))
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

fn git_path(prefix: &str, path: &str) -> String {
    if prefix.is_empty() {
        path.to_owned()
    } else {
        format!("{prefix}/{path}")
    }
}

fn strip_git_prefix<'a>(path: &'a str, prefix: &str) -> Option<&'a str> {
    if prefix.is_empty() {
        Some(path)
    } else {
        path.strip_prefix(prefix)?.strip_prefix('/')
    }
}

fn literal_pathspec(pointer: &str) -> String {
    format!(":(top,literal){pointer}")
}

fn candidate_pointer_contains_unsafe_entry(
    root: &Path,
    pointer: &str,
    mode: RatificationMode,
) -> Option<bool> {
    let pathspec = literal_pathspec(pointer);
    let mut args = match mode {
        RatificationMode::Index => vec!["ls-files", "--full-name", "-s", "-z", "--"],
        RatificationMode::Head => vec!["ls-tree", "-r", "-z", "--full-tree", "HEAD", "--"],
    };
    args.push(pathspec.as_str());
    let listing = git_output(root, args)?;
    for entry in listing.split('\0').filter(|entry| !entry.is_empty()) {
        let (metadata, _path) = entry.split_once('\t')?;
        let mode = metadata.split_whitespace().next()?;
        if !matches!(mode, "100644" | "100755") {
            return Some(true);
        }
    }
    Some(false)
}
pub(super) fn changed_paths(
    root: &Path,
    base: &str,
    mode: RatificationMode,
    git_prefix: &str,
) -> Option<BTreeSet<String>> {
    // `-z` is mandatory: without it Git C-quotes paths holding control or
    // non-ASCII bytes, the quoted spelling never equals the artefact path, and
    // the whole ratification gate silently skips that decision.
    let args = match mode {
        RatificationMode::Index => vec![
            "diff",
            "--no-relative",
            "-z",
            "--name-only",
            "--no-renames",
            "--cached",
            base,
        ],
        RatificationMode::Head => vec![
            "diff",
            "--no-relative",
            "-z",
            "--name-only",
            "--no-renames",
            base,
            "HEAD",
        ],
    };
    let output = git_output(root, args)?;
    Some(
        output
            .split('\0')
            .filter(|path| !path.is_empty())
            .filter_map(|path| project_relative_path(path, git_prefix))
            .map(str::to_owned)
            .collect(),
    )
}
pub(super) fn decision_was_not_local(
    filesystem_root: &Path,
    scan_root: &Path,
    base: &str,
    decision: &Decision,
    git_prefix: &str,
) -> bool {
    let path = project_git_path(&repository_path(scan_root, &decision.path), git_prefix);
    let Some(raw) = git_output(filesystem_root, ["show", &format!("{base}:{path}")]) else {
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
/// Classifies an accepted local decision against the MERGE-BASE allowlist.
///
/// The candidate range must not be able to weaken its own gate: reading the
/// allowlist from the base tree means a commit that rewrites
/// `docs/registries/binding-surface.md` is still judged by the rules that
pub(super) fn base_binding_surface_findings(
    root: &Path,
    base: &str,
    decision: &Decision,
    git_prefix: &str,
) -> Vec<Finding> {
    let allowlist_path = project_git_path(ALLOWLIST_PATH, git_prefix);
    let Some(source) = git_output(root, ["show", &format!("{base}:{allowlist_path}")]) else {
        return vec![finding(
            "CAIRN_DECISION_TIER_BINDING_PATH",
            &format!(
                "cannot read `{ALLOWLIST_PATH}` at the merge base, so local decision `{}` cannot be classified",
                decision.id
            ),
            Some(decision.path.clone()),
        )];
    };
    let rules = match parse_allowlist(&source) {
        Ok(rules) => rules,
        Err(reason) => {
            return vec![finding(
                "CAIRN_DECISION_TIER_BINDING_PATH",
                &format!("merge-base `{ALLOWLIST_PATH}` has {reason}"),
                Some(decision.path.clone()),
            )];
        }
    };
    let mut findings = Vec::new();
    for affect in &decision.affects {
        let Some(rule) = normalise_repo_entry(affect) else {
            continue;
        };
        let governed = match governed_canonical_files(root, &rule) {
            Ok(paths) => paths,
            Err(error) => {
                findings.push(finding(
                    "CAIRN_DECISION_TIER_BINDING_PATH",
                    &format!(
                        "affects entry `{affect}` of local decision `{}` cannot be classified: {}",
                        decision.id, error.message
                    ),
                    Some(decision.path.clone()),
                ));
                continue;
            }
        };
        let hit = governed
            .iter()
            .any(|file| rules.iter().any(|allow| rule_matches(allow, file)))
            || rules
                .iter()
                .any(|allow| rule_matches(allow, affect.trim_end_matches('/')));
        if hit {
            findings.push(finding(
                "CAIRN_DECISION_TIER_BINDING_PATH",
                &format!(
                    "local decision `{}` governs binding-surface path `{affect}` under the merge-base allowlist",
                    decision.id
                ),
                Some(decision.path.clone()),
            ));
        }
    }
    findings
}
