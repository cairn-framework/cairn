//! Receipt convergence legs for accepted local decisions.

use std::{collections::BTreeSet, path::Path};

use crate::artefacts::registry::{ArtefactSet, Decision, Review, ReviewType, manifest};

pub(super) fn convergence_leg(
    root: &Path,
    decision: &Decision,
    artefacts: &ArtefactSet,
    manifest: &str,
) -> Option<String> {
    let reviews: Vec<_> = decision
        .receipts
        .iter()
        .filter_map(|receipt| {
            artefacts
                .reviews
                .iter()
                .find(|review| review_stem(review) == Some(receipt.as_str()))
        })
        .collect();
    if reviews.len() < 2 {
        return Some("fewer than two resolved receipts".to_owned());
    }
    if reviews.iter().any(|review| review.subject_hash.is_none()) {
        return Some("receipt is not receipt-grade".to_owned());
    }
    if reviews
        .iter()
        .any(|review| review.review_type != ReviewType::AgentCrossModel)
    {
        return Some("receipt review_type is not agent_cross_model".to_owned());
    }
    if reviews.iter().any(|review| !clean_verdict(&review.body)) {
        return Some("receipt verdict is not a clean PASS".to_owned());
    }
    let identities: BTreeSet<_> = reviews.iter().map(|review| &review.reviewer).collect();
    if identities.len() < 2 {
        return Some("fewer than two independent reviewer identities".to_owned());
    }
    for review in &reviews {
        let (lens_path, expected_hash) = match lens_prompt_hash(root, &review.reviewer) {
            Ok(binding) => binding,
            Err(leg) => return Some(leg),
        };
        if review.lens_prompt_hash.as_deref() != Some(expected_hash.as_str()) {
            return Some(format!(
                "lens prompt hash mismatch for reviewer `{}` at `{lens_path}`",
                review.reviewer
            ));
        }
    }
    let hashes: BTreeSet<_> = reviews
        .iter()
        .filter_map(|review| review.subject_hash.as_deref())
        .collect();
    if hashes.len() > 1 {
        return Some("receipt subject_hash values are not all equal".to_owned());
    }
    if reviews
        .iter()
        .any(|review| review.subject_hash.as_deref() != Some(manifest))
    {
        return Some("receipt subject_hash does not equal the recomputed manifest".to_owned());
    }
    if reviews
        .iter()
        .any(|review| !manifest::review_path_covered(root, decision, review))
    {
        return Some("receipt path is not covered by affects".to_owned());
    }
    None
}

fn lens_prompt_hash(root: &Path, reviewer: &str) -> Result<(String, String), String> {
    let lens_id = reviewer
        .rsplit_once('/')
        .map(|(_, lens)| lens)
        .filter(|id| !id.is_empty())
        .ok_or_else(|| format!("lens prompt file is missing for reviewer `{reviewer}`"))?;
    let path = format!("docs/agent/lenses/{lens_id}.md");
    manifest::contained_file_sha256(root, &path)
        .map(|hash| (path.clone(), hash))
        .map_err(|_| format!("lens prompt file `{path}` is missing for reviewer `{reviewer}`"))
}

fn review_stem(review: &Review) -> Option<&str> {
    Path::new(&review.path).file_stem()?.to_str()
}

pub(super) fn clean_verdict(body: &str) -> bool {
    let mut verdict_open = false;
    for line in body.lines() {
        if verdict_open && !line.trim().is_empty() {
            return line.starts_with("PASS");
        }
        if line == "## Verdict" {
            verdict_open = true;
        }
    }
    false
}

pub(super) fn machine_debate_record(body: &str) -> Result<(), &'static str> {
    let headings = ["## For", "## Against", "## Verdict"];
    let mut positions = [None; 3];
    for (index, line) in body.lines().enumerate() {
        for (heading_index, heading) in headings.iter().enumerate() {
            if line == *heading {
                positions[heading_index] = Some(index);
            }
        }
    }
    for (index, heading) in headings.iter().enumerate() {
        if positions[index].is_none() {
            return Err(match *heading {
                "## For" => "machine debate record missing ## For section",
                "## Against" => "machine debate record missing ## Against section",
                _ => "machine debate record missing ## Verdict section",
            });
        }
    }
    let [
        Some(for_position),
        Some(against_position),
        Some(verdict_position),
    ] = positions
    else {
        unreachable!("missing headings returned above");
    };
    if !(for_position < against_position && against_position < verdict_position) {
        return Err("machine debate record sections are not in required order");
    }
    for (heading, position) in headings.iter().zip(positions) {
        let content = body
            .lines()
            .skip(position.expect("positions checked above") + 1)
            .find(|line| !line.trim().is_empty());
        if content.is_none_or(|line| line.starts_with('#')) {
            return Err(match *heading {
                "## For" => "machine debate record has empty ## For section",
                "## Against" => "machine debate record has empty ## Against section",
                _ => "machine debate record has empty ## Verdict section",
            });
        }
    }
    Ok(())
}
