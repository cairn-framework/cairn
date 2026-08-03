//! Parses the human briefing carried by proposed decision bodies.
//!
//! The pending queue keeps this extraction deliberately small and deterministic:
//! it reads the rubric bullets, the first ruling paragraph, and local review
//! receipts without interpreting the decision beyond those named sections.

use super::pending_evidence::{PendingEvidence, assemble};
pub use super::pending_rubric::PendingRubric;
use super::pending_rubric::parse_rubric;
use crate::artefacts::registry::{Decision, Review};

/// Parsed body fields shared by the CLI, context query, and webui.
#[derive(Clone, Debug, Default)]
pub struct PendingBrief {
    /// Short ruling paragraph.
    pub ruling_summary: Option<String>,
    /// Rubric sections, when a rubric heading was authored.
    pub rubric: Option<PendingRubric>,
    /// Local review state, when the row is self-approvable.
    pub evidence: Option<PendingEvidence>,
    /// True when any comparable review hash differs from the current
    /// subject hash; uncomparable receipts never set it.
    pub changed_since_review: bool,
}

/// Parse the briefing fields for one proposed decision.
pub(crate) fn parse_pending_brief(
    root: &std::path::Path,
    decision: &Decision,
    reviews: &[Review],
    current_subject_hash: Option<&str>,
    local_tier: bool,
) -> PendingBrief {
    let ruling_summary =
        first_ruling_paragraph(&decision.body).map(|text| limit_sentences(&text, 2));
    let rubric = parse_rubric(&decision.body);
    if !local_tier {
        return PendingBrief {
            ruling_summary,
            rubric,
            ..PendingBrief::default()
        };
    }

    let (evidence, changed_since_review) = assemble(root, decision, reviews, current_subject_hash);
    PendingBrief {
        ruling_summary,
        rubric,
        evidence: Some(evidence),
        changed_since_review,
    }
}

pub(super) fn nonempty(value: String) -> Option<String> {
    (!value.is_empty()).then_some(value)
}

pub(super) fn nonempty_items(values: Vec<String>) -> Option<Vec<String>> {
    (!values.is_empty()).then_some(values)
}

fn first_ruling_paragraph(body: &str) -> Option<String> {
    let lines: Vec<&str> = body.lines().collect();
    find_section(&lines, |level, heading| {
        // Level 2+ only: an H1 title mentioning "decision" is the document
        // name, never the ruling section.
        level >= 2 && (heading.starts_with("ruling") || heading.starts_with("decision"))
    })
    .and_then(|(start, end)| first_paragraph(&lines[start..end]))
    .or_else(|| first_body_paragraph(&lines))
    .map(|paragraph| clean_markdown(&paragraph))
    .filter(|paragraph| !paragraph.is_empty())
}

fn first_body_paragraph(lines: &[&str]) -> Option<String> {
    let mut index = 0;
    while index < lines.len() {
        if heading_level(lines[index]).is_some() {
            index += 1;
            continue;
        }
        if !lines[index].trim().is_empty() {
            return first_paragraph(&lines[index..]);
        }
        index += 1;
    }
    None
}

fn first_paragraph(lines: &[&str]) -> Option<String> {
    let mut values = Vec::new();
    let mut list_item = false;
    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if values.is_empty() {
                continue;
            }
            break;
        }
        if heading_level(trimmed).is_some() {
            break;
        }
        if list_item && starts_list_item(trimmed) {
            break;
        }
        list_item |= starts_list_item(trimmed);
        values.push(trimmed.to_owned());
    }
    (!values.is_empty()).then(|| values.join(" "))
}

pub(super) fn find_section(
    lines: &[&str],
    wanted: impl Fn(usize, &str) -> bool,
) -> Option<(usize, usize)> {
    let (heading_index, (_, level)) = lines.iter().enumerate().find_map(|(index, line)| {
        heading_level(line)
            .filter(|(level, text)| wanted(*level, text))
            .map(|(level, text)| (index, (text, level)))
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
    let text = trimmed[level..]
        .trim()
        .trim_end_matches('#')
        .trim()
        .to_lowercase();
    Some((level, text))
}

pub(super) fn top_level_bullet(line: &str) -> Option<(&str, &str)> {
    let value = line.strip_prefix("- ")?.trim();
    let colon = value.find(':')?;
    let label = value[..colon].trim().trim_matches('*').trim();
    Some((label, value[colon + 1..].trim()))
}

pub(super) fn normalise_label(label: &str) -> String {
    label
        .trim_matches(|ch: char| !ch.is_ascii_alphanumeric())
        .to_lowercase()
}

pub(super) fn starts_list_item(value: &str) -> bool {
    value.starts_with("- ")
        || value.starts_with("* ")
        || value.split_once('.').is_some_and(|(prefix, _)| {
            !prefix.is_empty() && prefix.chars().all(|ch| ch.is_ascii_digit())
        })
}

pub(super) fn clean_markdown(value: &str) -> String {
    let value = value.trim();
    let value = value.strip_prefix("- ").unwrap_or(value).trim();
    let value = value
        .split_once('.')
        .filter(|(prefix, _)| !prefix.is_empty() && prefix.chars().all(|ch| ch.is_ascii_digit()))
        .map_or(value, |(_, rest)| rest.trim());
    value.replace("**", "").replace("__", "").replace('`', "")
}

fn limit_sentences(value: &str, max: usize) -> String {
    let mut count = 0;
    let mut end = value.len();
    for (index, ch) in value.char_indices() {
        if !matches!(ch, '.' | '!' | '?') {
            continue;
        }
        // A terminator mid-token (`dec.foo`, `src/file.rs`) is not a
        // sentence end; only end-of-text or a following space counts, with
        // closing quotes and brackets allowed between the two.
        let after = &value[index + ch.len_utf8()..];
        let closers = after.len() - after.trim_start_matches(['"', '\'', ')', ']', '`']).len();
        let rest = &after[closers..];
        if !rest.is_empty() && !rest.starts_with(char::is_whitespace) {
            continue;
        }
        count += 1;
        if count == max {
            end = index + ch.len_utf8() + closers;
            break;
        }
    }
    value[..end].trim().to_owned()
}
#[cfg(test)]
#[path = "pending_brief_live_tests.rs"]
mod live_tests;
#[cfg(test)]
#[path = "pending_brief_tests.rs"]
mod tests;
