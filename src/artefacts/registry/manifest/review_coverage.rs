//! Review-coverage matching: does a decision's `affects:` rule set cover a
//! review artefact's on-disk path.

use std::path::Path;

use super::{normalise_repo_entry, rule_matches};

/// Returns whether a review artefact's path is covered by a decision's
/// `affects:` rules: the review path is canonicalised and stripped to its
/// repo-relative form, then matched against each normalised entry (exact
/// file or directory rule).
#[must_use]
pub(crate) fn review_path_covered(
    root: &Path,
    decision: &crate::artefacts::registry::Decision,
    review: &crate::artefacts::registry::Review,
) -> bool {
    let review_path = Path::new(&review.path);
    let canonical_path = review_path.canonicalize().ok();
    let path = canonical_path
        .as_deref()
        .and_then(|path| path.strip_prefix(root).ok())
        .or_else(|| review_path.strip_prefix(root).ok())
        .unwrap_or(review_path);
    // Join components with `/` so Windows separators match repo-rule syntax.
    let path = path
        .components()
        .filter_map(|component| component.as_os_str().to_str())
        .collect::<Vec<_>>()
        .join("/");
    decision
        .affects
        .iter()
        .filter_map(|entry| normalise_repo_entry(entry))
        .any(|rule| rule_matches(&rule, &path))
}
