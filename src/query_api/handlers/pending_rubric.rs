//! Parses the four-part rubric bullets on proposed decision bodies.
//!
//! Split from `pending_brief` at its natural seam: rubric accumulation is
//! independent of ruling and paragraph extraction, and both stay under the
//! proactive module-size threshold this way.

use super::pending_brief::{
    clean_markdown, find_section, nonempty, nonempty_items, normalise_label, starts_list_item,
    top_level_bullet,
};

/// The four named parts of a decision rubric.
#[derive(Clone, Debug, serde::Serialize, schemars::JsonSchema)]
pub struct PendingRubric {
    /// Decision tier and its supporting facts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// What signing the decision unblocks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unblocks: Option<Vec<String>>,
    /// How the decision fits the project mission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<Vec<String>>,
    /// Choices considered by the author.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

pub(super) fn parse_rubric(body: &str) -> Option<PendingRubric> {
    let lines: Vec<&str> = body.lines().collect();
    let (start, end) = find_section(&lines, |level, heading| {
        level >= 2 && heading.contains("rubric")
    })?;
    let mut tier = None;
    let mut sections: [Option<Vec<String>>; 3] = [None, None, None];
    let mut current: Option<usize> = None;

    for line in &lines[start..end] {
        let trimmed = line.trim();
        let is_top_level = !line.chars().next().is_some_and(char::is_whitespace);
        let Some((label, value)) = is_top_level.then(|| top_level_bullet(trimmed)).flatten() else {
            if let Some(index) = current {
                if index == 3 {
                    append_tier(&mut tier, trimmed);
                } else if starts_list_item(trimmed) {
                    append_value(&mut sections[index], trimmed);
                } else {
                    append_continuation(&mut sections[index], trimmed);
                }
            }
            continue;
        };
        let key = normalise_label(label);
        let body_value = clean_markdown(value);
        match key.as_str() {
            "tier" => {
                tier = nonempty(body_value);
                current = Some(3);
            }
            "unblocks" => {
                current = Some(0);
                append_value(&mut sections[0], &body_value);
            }
            "alignment" => {
                current = Some(1);
                append_value(&mut sections[1], &body_value);
            }
            value if value.starts_with("options") => {
                current = Some(2);
                append_value(&mut sections[2], &body_value);
            }
            _ => current = None,
        }
    }

    let options = sections[2]
        .take()
        .and_then(nonempty_items)
        .and_then(|values| {
            let mut parsed = None;
            append_options(&mut parsed, &values.join(" "));
            parsed
        });
    let rubric = PendingRubric {
        tier,
        unblocks: sections[0].take().and_then(nonempty_items),
        alignment: sections[1].take().and_then(nonempty_items),
        options,
    };
    (rubric.tier.is_some()
        || rubric.unblocks.is_some()
        || rubric.alignment.is_some()
        || rubric.options.is_some())
    .then_some(rubric)
}
fn append_tier(slot: &mut Option<String>, value: &str) {
    let value = clean_markdown(value);
    if value.is_empty() {
        return;
    }
    if let Some(current) = slot {
        current.push(' ');
        current.push_str(&value);
    } else {
        *slot = Some(value);
    }
}

fn append_continuation(slot: &mut Option<Vec<String>>, value: &str) {
    let value = clean_markdown(value);
    if value.is_empty() {
        return;
    }
    if let Some(last) = slot.as_mut().and_then(|values| values.last_mut()) {
        last.push(' ');
        last.push_str(&value);
    } else {
        slot.get_or_insert_with(Vec::new).push(value);
    }
}

fn append_value(slot: &mut Option<Vec<String>>, value: &str) {
    let value = clean_markdown(value);
    if value.is_empty() {
        return;
    }
    slot.get_or_insert_with(Vec::new).push(value);
}

fn append_options(slot: &mut Option<Vec<String>>, value: &str) {
    let value = clean_markdown(value);
    let mut cursor = 0;
    let mut items = Vec::new();
    while let Some((_open, close, marker)) = find_option_marker(&value, cursor) {
        let next = find_option_marker(&value, close + 1)
            .map_or(value.len(), |(next_open, _, _)| next_open);
        let item = value[close + 1..next]
            .trim()
            .trim_start_matches([';', ','])
            .trim()
            .trim_end_matches(['.', ';', ','])
            .trim();
        if !item.is_empty() {
            items.push(format!("({marker}) {item}"));
        }
        cursor = next;
    }
    if items.is_empty() {
        append_value(slot, &value);
    } else {
        slot.get_or_insert_with(Vec::new).extend(items);
    }
}

fn find_option_marker(value: &str, start: usize) -> Option<(usize, usize, char)> {
    for (offset, ch) in value[start..].char_indices() {
        if ch != '(' {
            continue;
        }
        let open = start + offset;
        let close = value[open + 1..].find(')')? + open + 1;
        let marker = value[open + 1..close].chars().next()?;
        if value[open + 1..close].chars().count() != 1
            || !marker.is_ascii_alphabetic()
            || !value[..open]
                .chars()
                .rev()
                .find(|ch| !ch.is_whitespace())
                .is_none_or(|ch| matches!(ch, ';' | ','))
        {
            continue;
        }
        return Some((open, close, marker));
    }
    None
}
