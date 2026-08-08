//! Ratification gate for newly accepted local decisions.

use std::{
    collections::BTreeSet,
    path::{Component, Path, PathBuf},
};

use crate::{
    artefacts::registry::{
        ArtefactSet, Decision, Review,
        manifest::{
            compute_decision_subject_hash, governed_canonical_files, normalise_repo_entry,
            normalise_repo_path, parse_allowlist, rule_matches,
        },
    },
    map::{Finding, FindingSeverity},
};

mod git;

use git::{
    candidate_accepted_local, candidate_decision_pointers, candidate_pointer_configuration_matches,
    changed_paths, decision_was_not_local, git_output, inside_work_tree,
};

const ALLOWLIST_PATH: &str = "docs/registries/binding-surface.md";

/// Selects the git tree compared against the merge base.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RatificationMode {
    /// Compare the merge base with the staged index for a pre-commit hook.
    Index,
    /// Compare the merge base with the checked-out commit in CI.
    Head,
}

/// Checks newly accepted local decisions against their declared subject.
#[must_use]
pub fn ratification_findings(
    root: &Path,
    artefacts: &ArtefactSet,
    mode: RatificationMode,
) -> Vec<Finding> {
    ratification_findings_with_blueprint(root, artefacts, mode, Path::new("cairn.blueprint"))
}

#[must_use]
pub(super) fn ratification_findings_with_blueprint(
    root: &Path,
    artefacts: &ArtefactSet,
    mode: RatificationMode,
    blueprint_path: &Path,
) -> Vec<Finding> {
    let filesystem_root = match root.canonicalize() {
        Ok(root) => root,
        Err(_) => {
            return vec![finding(
                "CAIRN_HOOK_AFFECTS_SUBSET",
                "cannot resolve scan root while checking ratification evidence",
                None,
            )];
        }
    };
    // The trigger is read from the CANDIDATE tree (index in pre-commit, HEAD
    // in CI), never from the worktree: staging an acceptance and then editing
    // the unstaged copy back to `proposed` would otherwise empty this set and
    // skip the gate for a commit that does accept the decision. Only an
    // accepted local decision gates anything, so a repository that merely
    // proposes one still needs no merge base.
    // Outside a Git work tree nothing can be committed, so there is nothing to
    // gate; that is the ONLY silent case. Inside one, an unanswerable Git may
    // be hiding an acceptance, so enumeration failure always fails closed
    // (never inferred from worktree contents, which the candidate tree is
    // free to contradict).
    if !inside_work_tree(&filesystem_root) {
        return Vec::new();
    }
    let Some((candidate_pointers, git_prefix)) =
        candidate_decision_pointers(&filesystem_root, blueprint_path, mode)
    else {
        return vec![finding(
            "CAIRN_HOOK_AFFECTS_SUBSET",
            "cannot read or parse the candidate blueprint while checking ratification evidence",
            None,
        )];
    };
    let Some(pointer_configuration_matches) =
        candidate_pointer_configuration_matches(&artefacts.decision_pointers, &candidate_pointers)
    else {
        return vec![finding(
            "CAIRN_HOOK_AFFECTS_SUBSET",
            "cannot validate candidate decisions pointer configuration while checking ratification evidence",
            None,
        )];
    };
    if !pointer_configuration_matches {
        return vec![finding(
            "CAIRN_HOOK_AFFECTS_SUBSET",
            "candidate and worktree decisions pointer configurations differ",
            None,
        )];
    }
    let Some(candidates) =
        candidate_accepted_local(&filesystem_root, &candidate_pointers, &git_prefix, mode)
    else {
        return vec![finding(
            "CAIRN_HOOK_AFFECTS_SUBSET",
            "cannot read or reconcile candidate decisions while checking ratification evidence",
            None,
        )];
    };
    if candidates.is_empty() {
        return Vec::new();
    }
    let mut local_decisions = Vec::new();
    let mut missing = Vec::new();
    for path in &candidates {
        match artefacts
            .decisions
            .iter()
            .find(|decision| &repository_path(root, &decision.path) == path)
        {
            Some(decision) => local_decisions.push(decision),
            None => missing.push(path.clone()),
        }
    }
    if !missing.is_empty() {
        return missing
            .into_iter()
            .map(|path| {
                finding(
                    "CAIRN_HOOK_MANIFEST_MISMATCH",
                    &format!(
                        "candidate tree accepts local decision `{path}` that the working tree does not load, so its subject cannot be validated"
                    ),
                    Some(path),
                )
            })
            .collect();
    }

    let Some(base) = git_output(&filesystem_root, ["merge-base", "origin/main", "HEAD"]) else {
        return vec![finding(
            "CAIRN_HOOK_AFFECTS_SUBSET",
            "cannot resolve merge-base for required ref `origin/main`",
            None,
        )];
    };
    let Some(changed) = changed_paths(&filesystem_root, base.trim(), mode, &git_prefix) else {
        return vec![finding(
            "CAIRN_HOOK_AFFECTS_SUBSET",
            "cannot read changed paths from ratification range",
            None,
        )];
    };

    local_decisions
        .into_iter()
        .filter(|decision| {
            decision_was_not_local(&filesystem_root, root, base.trim(), decision, &git_prefix)
        })
        .filter(|decision| changed.contains(&repository_path(root, &decision.path)))
        .flat_map(|decision| {
            index_governed_overlap_findings(
                root,
                &filesystem_root,
                decision,
                &artefacts.reviews,
                mode,
                &git_prefix,
            )
            .into_iter()
            .chain(decision_findings(
                root,
                &filesystem_root,
                base.trim(),
                decision,
                &artefacts.reviews,
                &changed,
                &git_prefix,
            ))
        })
        .collect()
}

fn decision_findings(
    scan_root: &Path,
    filesystem_root: &Path,
    base: &str,
    decision: &Decision,
    reviews: &[Review],
    changed: &BTreeSet<String>,
    git_prefix: &str,
) -> Vec<Finding> {
    let path = repository_path(scan_root, &decision.path);
    let rules = decision
        .affects
        .iter()
        .filter_map(|affect| normalise_repo_entry(affect))
        .collect::<Vec<_>>();
    let uncovered = changed
        .iter()
        .chain(std::iter::once(&path))
        .find(|candidate| !rules.iter().any(|rule| rule_matches(rule, candidate)));
    let mut findings = Vec::new();
    if let Some(path) = uncovered {
        findings.push(finding(
            "CAIRN_HOOK_AFFECTS_SUBSET",
            &format!(
                "accepted local decision `{}` does not cover changed path `{path}` in affects",
                decision.id
            ),
            Some(decision.path.clone()),
        ));
    }

    let receipt_hashes = decision
        .receipts
        .iter()
        .map(|stem| reviews.iter().find(|review| review_stem(review) == stem))
        .collect::<Vec<_>>();
    let actual = compute_decision_subject_hash(scan_root, decision).ok();
    let receipts_match = !decision.receipts.is_empty()
        && actual.as_ref().is_some_and(|actual| {
            receipt_hashes.iter().all(|review| {
                review.and_then(|review| review.subject_hash.as_ref()) == Some(actual)
            })
        });
    findings.extend(base_binding_surface_findings(
        filesystem_root,
        base,
        decision,
        git_prefix,
    ));

    if !receipts_match {
        findings.push(finding(
            "CAIRN_HOOK_MANIFEST_MISMATCH",
            &format!(
                "accepted local decision `{}` has receipts that do not match its manifest",
                decision.id
            ),
            Some(decision.path.clone()),
        ));
    }
    findings
}

fn index_governed_overlap_findings(
    scan_root: &Path,
    filesystem_root: &Path,
    decision: &Decision,
    reviews: &[Review],
    mode: RatificationMode,
    git_prefix: &str,
) -> Vec<Finding> {
    if mode != RatificationMode::Index {
        return Vec::new();
    }
    let Some(unstaged) = git_output(
        filesystem_root,
        ["diff", "--no-relative", "-z", "--name-only", "--no-renames"],
    ) else {
        return vec![finding(
            "CAIRN_HOOK_MANIFEST_MISMATCH",
            "cannot read unstaged paths while checking ratification evidence",
            Some(decision.path.clone()),
        )];
    };
    let rules = decision
        .affects
        .iter()
        .filter_map(|affect| normalise_repo_entry(affect))
        .collect::<Vec<_>>();
    let Some(tracked) = git_path_set(
        filesystem_root,
        ["ls-files", "--full-name", "-z"],
        git_prefix,
    ) else {
        return vec![finding(
            "CAIRN_HOOK_MANIFEST_MISMATCH",
            "cannot read index paths while checking ratification evidence",
            Some(decision.path.clone()),
        )];
    };
    let Some(untracked) = git_path_set(
        filesystem_root,
        [
            "ls-files",
            "--full-name",
            "-z",
            "--others",
            "--exclude-standard",
        ],
        git_prefix,
    ) else {
        return vec![finding(
            "CAIRN_HOOK_MANIFEST_MISMATCH",
            "cannot read untracked paths while checking ratification evidence",
            Some(decision.path.clone()),
        )];
    };
    let Some(ignored) = git_path_set(
        filesystem_root,
        [
            "ls-files",
            "--full-name",
            "-z",
            "--others",
            "-i",
            "--exclude-standard",
        ],
        git_prefix,
    ) else {
        return vec![finding(
            "CAIRN_HOOK_MANIFEST_MISMATCH",
            "cannot read ignored paths while checking ratification evidence",
            Some(decision.path.clone()),
        )];
    };
    let known_paths = tracked
        .iter()
        .chain(untracked.iter())
        .chain(ignored.iter())
        .cloned()
        .collect::<BTreeSet<_>>();
    let governed = governed_paths(scan_root, decision, reviews, &rules, &known_paths);
    if let Some(path) = governed.iter().find(|path| {
        !tracked.contains(*path) || untracked.contains(*path) || ignored.contains(*path)
    }) {
        return vec![finding(
            "CAIRN_HOOK_MANIFEST_MISMATCH",
            &format!(
                "governed path `{path}` is absent from the index; the commit cannot contain the reviewed bytes"
            ),
            Some(decision.path.clone()),
        )];
    }
    let overlaps = unstaged
        .split('\0')
        .filter(|path| !path.is_empty())
        .filter_map(|path| project_relative_path(path, git_prefix))
        .filter_map(normalise_repo_path)
        .any(|path| governed.contains(&path) || rules.iter().any(|rule| rule_matches(rule, &path)));
    overlaps
        .then(|| {
            finding(
                "CAIRN_HOOK_MANIFEST_MISMATCH",
                &format!(
                    "unstaged changes overlap the governed subject for accepted local decision `{}`; stage or stash them before accepting",
                    decision.id
                ),
                Some(decision.path.clone()),
            )
        })
        .into_iter()
        .collect()
}

fn governed_paths(
    root: &Path,
    decision: &Decision,
    reviews: &[Review],
    rules: &[crate::artefacts::registry::manifest::RepoPathRule],
    known_paths: &BTreeSet<String>,
) -> BTreeSet<String> {
    let mut paths = BTreeSet::from([repository_path(root, &decision.path)]);
    paths.extend(rules.iter().filter_map(|rule| match rule {
        crate::artefacts::registry::manifest::RepoPathRule::File(path) => Some(path.clone()),
        crate::artefacts::registry::manifest::RepoPathRule::Dir(_) => None,
    }));
    paths.extend(
        known_paths
            .iter()
            .filter(|path| rules.iter().any(|rule| rule_matches(rule, path)))
            .cloned(),
    );
    paths.extend(
        decision
            .receipts
            .iter()
            .filter_map(|stem| reviews.iter().find(|review| review_stem(review) == stem))
            .map(|review| repository_path(root, &review.path)),
    );
    paths
}

fn git_path_set<'a>(
    root: &Path,
    args: impl IntoIterator<Item = &'a str>,
    git_prefix: &str,
) -> Option<BTreeSet<String>> {
    let output = git_output(root, args)?;
    Some(
        output
            .split('\0')
            .filter(|path| !path.is_empty())
            .filter_map(|path| project_relative_path(path, git_prefix))
            .filter_map(normalise_repo_path)
            .collect(),
    )
}

/// Classifies an accepted local decision against the MERGE-BASE allowlist.
///
/// The candidate range must not be able to weaken its own gate: reading the
/// allowlist from the base tree means a commit that rewrites
/// `docs/registries/binding-surface.md` is still judged by the rules that
fn base_binding_surface_findings(
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

fn repository_path(root: &Path, path: &str) -> String {
    let root = lexical_normalize(root);
    let path = lexical_normalize(Path::new(path));
    path.strip_prefix(&root)
        .unwrap_or(path.as_path())
        .to_string_lossy()
        .replace('\\', "/")
}

fn lexical_normalize(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir if normalized.file_name().is_some() => {
                normalized.pop();
            }
            Component::ParentDir => normalized.push(".."),
            _ => normalized.push(component.as_os_str()),
        }
    }
    if normalized.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        normalized
    }
}

fn project_relative_path<'a>(path: &'a str, git_prefix: &str) -> Option<&'a str> {
    if git_prefix.is_empty() {
        Some(path)
    } else {
        path.strip_prefix(git_prefix)?.strip_prefix('/')
    }
}

fn project_git_path(path: &str, git_prefix: &str) -> String {
    if git_prefix.is_empty() {
        path.to_owned()
    } else {
        format!("{git_prefix}/{path}")
    }
}

fn review_stem(review: &Review) -> &str {
    Path::new(&review.path)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or_default()
}

fn finding(code: &str, message: &str, path: Option<String>) -> Finding {
    Finding {
        code: code.to_owned(),
        severity: FindingSeverity::Error,
        message: message.to_owned(),
        node: None,
        target: None,
        path,
        deferred_by: None,
        parked_by: None,
    }
}
