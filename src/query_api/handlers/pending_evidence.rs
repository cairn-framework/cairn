//! Assembles local pending-decision review evidence.
//!
//! Receipt discovery and hash comparison stay separate from Markdown briefing
//! extraction so local-only evidence can evolve without enlarging the parser.

use std::path::Path;

use crate::artefacts::registry::{Decision, Review};

fn nullable_bool_schema(
    generator: &mut schemars::r#gen::SchemaGenerator,
) -> schemars::schema::Schema {
    generator.subschema_for::<Option<bool>>()
}
/// One review receipt attached to a local-tier pending decision.
#[derive(Clone, Debug, serde::Serialize, schemars::JsonSchema)]
pub struct PendingReceipt {
    /// Review stem from the decision's `receipts:` list or an `affects:`
    /// rule (exact review files even when absent; directories via loaded
    /// reviews).
    pub stem: String,
    /// Reviewer name, when the review artefact is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewer: Option<String>,
    /// First line under the review's Verdict heading, when present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verdict: Option<String>,
    /// Whether this review covered the decision's current subject manifest;
    /// null when either hash is unavailable, so no comparison was made.
    #[schemars(required, schema_with = "nullable_bool_schema")]
    pub subject_hash_matches: Option<bool>,
}

/// Evidence state for a local-tier pending decision.
#[derive(Clone, Debug, serde::Serialize, schemars::JsonSchema)]
pub struct PendingEvidence {
    /// Review stems named by the decision or linked through `affects:`.
    pub receipts: Vec<PendingReceipt>,
}

/// Assemble the evidence rows and whether any available review is stale.
#[must_use]
pub(crate) fn assemble(
    root: &std::path::Path,
    decision: &Decision,
    reviews: &[Review],
    current_subject_hash: Option<&str>,
) -> (PendingEvidence, bool) {
    use crate::artefacts::registry::manifest;
    let mut stems = decision.receipts.clone();
    // An exact review-file rule in `affects:` seeds a receipt row even when
    // the artefact is absent, so a dangling pointer renders as unverified
    // instead of vanishing. Directory rules expand from loaded reviews only.
    for entry in &decision.affects {
        let Some(manifest::RepoPathRule::File(path)) = manifest::normalise_repo_entry(entry) else {
            continue;
        };
        let file = std::path::Path::new(&path);
        let in_reviews = file
            .parent()
            .is_some_and(|dir| dir == std::path::Path::new(manifest::REVIEWS_DIR));
        let is_markdown = file
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("md"));
        let Some(stem) = file.file_stem().and_then(|stem| stem.to_str()) else {
            continue;
        };
        if in_reviews && is_markdown && !stems.iter().any(|existing| existing == stem) {
            stems.push(stem.to_owned());
        }
    }
    for review in reviews {
        if manifest::review_path_covered(root, decision, review) {
            let stem = review_stem(review);
            if !stem.is_empty() && !stems.contains(&stem) {
                stems.push(stem);
            }
        }
    }
    let receipts = stems
        .iter()
        .map(|stem| {
            let review = reviews.iter().find(|review| review_stem(review) == *stem);
            let subject_hash_matches = match (
                review.and_then(|review| review.subject_hash.as_deref()),
                current_subject_hash,
            ) {
                (Some(reviewed), Some(current)) => Some(reviewed == current),
                _ => None,
            };
            PendingReceipt {
                stem: stem.clone(),
                reviewer: review.map(|review| review.reviewer.clone()),
                verdict: review.and_then(|review| first_verdict_line(&review.body)),
                subject_hash_matches,
            }
        })
        .collect::<Vec<_>>();
    let changed_since_review = receipts
        .iter()
        .any(|receipt| receipt.subject_hash_matches == Some(false));
    (PendingEvidence { receipts }, changed_since_review)
}

fn review_stem(review: &Review) -> String {
    Path::new(&review.path)
        .file_stem()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or_default()
        .to_owned()
}
fn first_verdict_line(body: &str) -> Option<String> {
    let lines: Vec<&str> = body.lines().collect();
    let (start, end) = find_verdict_section(&lines)?;
    lines[start..end]
        .iter()
        .map(|line| clean_review_text(line.trim()))
        .find(|line| !line.is_empty())
}

fn find_verdict_section(lines: &[&str]) -> Option<(usize, usize)> {
    let (heading_index, level) = lines.iter().enumerate().find_map(|(index, line)| {
        heading_level(line)
            .filter(|(_, text)| text.starts_with("verdict"))
            .map(|(level, _)| (index, level))
    })?;
    let end = lines[heading_index + 1..]
        .iter()
        .position(|line| heading_level(line).is_some_and(|(next_level, _)| next_level <= level))
        .map_or(lines.len(), |offset| heading_index + 1 + offset);
    Some((heading_index + 1, end))
}

fn heading_level(line: &str) -> Option<(usize, String)> {
    let trimmed = line.trim();
    let level = trimmed.chars().take_while(|ch| *ch == '#').count();
    if level == 0 || !trimmed[level..].starts_with(' ') {
        return None;
    }
    Some((
        level,
        trimmed[level..]
            .trim()
            .trim_end_matches('#')
            .trim()
            .to_lowercase(),
    ))
}

fn clean_review_text(value: &str) -> String {
    value
        .strip_prefix("- ")
        .unwrap_or(value)
        .trim()
        .replace("**", "")
        .replace("__", "")
        .replace('`', "")
}

#[cfg(test)]
#[path = "pending_evidence_tests.rs"]
mod tests;
