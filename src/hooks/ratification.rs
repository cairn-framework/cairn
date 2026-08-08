//! Ratification gate for newly accepted local decisions.

use std::{collections::BTreeSet, path::Path};

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
    candidate_accepted_local, changed_paths, decision_was_not_local, git_output, inside_work_tree,
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
    if !inside_work_tree(root) {
        return Vec::new();
    }
    let Some(candidates) = candidate_accepted_local(root, &artefacts.decision_pointers, mode)
    else {
        return vec![finding(
            "CAIRN_HOOK_AFFECTS_SUBSET",
            "cannot read candidate decisions while checking ratification evidence",
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

    let Some(base) = git_output(root, ["merge-base", "origin/main", "HEAD"]) else {
        return vec![finding(
            "CAIRN_HOOK_AFFECTS_SUBSET",
            "cannot resolve merge-base for required ref `origin/main`",
            None,
        )];
    };
    let Some(changed) = changed_paths(root, base.trim(), mode) else {
        return vec![finding(
            "CAIRN_HOOK_AFFECTS_SUBSET",
            "cannot read changed paths from ratification range",
            None,
        )];
    };

    local_decisions
        .into_iter()
        .filter(|decision| decision_was_not_local(root, base.trim(), decision))
        .filter(|decision| changed.contains(&repository_path(root, &decision.path)))
        .flat_map(|decision| {
            index_governed_overlap_findings(root, decision, &artefacts.reviews, mode)
                .into_iter()
                .chain(decision_findings(
                    root,
                    base.trim(),
                    decision,
                    &artefacts.reviews,
                    &changed,
                ))
        })
        .collect()
}

fn decision_findings(
    root: &Path,
    base: &str,
    decision: &Decision,
    reviews: &[Review],
    changed: &BTreeSet<String>,
) -> Vec<Finding> {
    let path = repository_path(root, &decision.path);
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
    let actual = compute_decision_subject_hash(root, decision).ok();
    let receipts_match = !decision.receipts.is_empty()
        && actual.as_ref().is_some_and(|actual| {
            receipt_hashes.iter().all(|review| {
                review.and_then(|review| review.subject_hash.as_ref()) == Some(actual)
            })
        });
    findings.extend(base_binding_surface_findings(root, base, decision));

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
    root: &Path,
    decision: &Decision,
    reviews: &[Review],
    mode: RatificationMode,
) -> Vec<Finding> {
    if mode != RatificationMode::Index {
        return Vec::new();
    }

    let Some(unstaged) = git_output(root, ["diff", "-z", "--name-only", "--no-renames"]) else {
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
    let Some(tracked) = git_path_set(root, ["ls-files", "-z"]) else {
        return vec![finding(
            "CAIRN_HOOK_MANIFEST_MISMATCH",
            "cannot read index paths while checking ratification evidence",
            Some(decision.path.clone()),
        )];
    };
    let Some(untracked) = git_path_set(root, ["ls-files", "-z", "--others", "--exclude-standard"])
    else {
        return vec![finding(
            "CAIRN_HOOK_MANIFEST_MISMATCH",
            "cannot read untracked paths while checking ratification evidence",
            Some(decision.path.clone()),
        )];
    };
    let Some(ignored) = git_path_set(
        root,
        ["ls-files", "-z", "--others", "-i", "--exclude-standard"],
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
    let governed = governed_paths(root, decision, reviews, &rules, &known_paths);
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
) -> Option<BTreeSet<String>> {
    let output = git_output(root, args)?;
    Some(
        output
            .split('\0')
            .filter(|path| !path.is_empty())
            .filter_map(normalise_repo_path)
            .collect(),
    )
}

/// Classifies an accepted local decision against the MERGE-BASE allowlist.
///
/// The candidate range must not be able to weaken its own gate: reading the
/// allowlist from the base tree means a commit that rewrites
/// `docs/registries/binding-surface.md` is still judged by the rules that
/// stood before it. Fails closed when the base copy is unavailable, since a
/// local acceptance cannot be validated without the surface it must avoid.
fn base_binding_surface_findings(root: &Path, base: &str, decision: &Decision) -> Vec<Finding> {
    let Some(source) = git_output(root, ["show", &format!("{base}:{ALLOWLIST_PATH}")]) else {
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
    Path::new(path)
        .strip_prefix(root)
        .unwrap_or_else(|_| Path::new(path))
        .to_string_lossy()
        .replace('\\', "/")
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
