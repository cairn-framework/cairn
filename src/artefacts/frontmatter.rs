//! Minimal Markdown frontmatter parser.

use std::collections::BTreeMap;

/// Parsed frontmatter and Markdown body.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Frontmatter {
    /// Key-value frontmatter.
    pub values: BTreeMap<String, String>,
    /// Sequence frontmatter entries, including simple YAML lists and object IDs.
    pub lists: BTreeMap<String, Vec<String>>,
    /// Body after frontmatter.
    pub body: String,
}

/// Parses `---` delimited frontmatter with `key: value` entries.
#[must_use]
pub fn parse(source: &str) -> Frontmatter {
    let mut values = BTreeMap::new();
    let mut lists = BTreeMap::<String, Vec<String>>::new();
    let mut lines = source.lines();
    if lines.next() != Some("---") {
        return Frontmatter {
            values,
            lists,
            body: source.to_owned(),
        };
    }
    let mut body = Vec::new();
    let mut in_frontmatter = true;
    let mut active_list: Option<String> = None;
    for line in lines {
        if in_frontmatter && line.trim() == "---" {
            in_frontmatter = false;
            active_list = None;
            continue;
        }
        if in_frontmatter {
            let trimmed = line.trim();
            if let Some(stripped) = trimmed.strip_prefix("- ") {
                if let Some(key) = &active_list {
                    let is_quoted = stripped.starts_with('"') || stripped.starts_with('\'');
                    if !is_quoted && let Some((item_key, item_value)) = stripped.split_once(':') {
                        if item_key.trim() == "id" {
                            lists
                                .entry(key.clone())
                                .or_default()
                                .push(clean_scalar(item_value));
                        }
                    } else {
                        lists
                            .entry(key.clone())
                            .or_default()
                            .push(clean_scalar(stripped));
                    }
                }
                continue;
            }
            if let Some((nested_key, nested_value)) = trimmed.split_once(':')
                && nested_key.trim() == "id"
                && line.starts_with(char::is_whitespace)
            {
                if let Some(key) = &active_list {
                    lists
                        .entry(key.clone())
                        .or_default()
                        .push(clean_scalar(nested_value));
                }
                continue;
            }
            if let Some((key, value)) = trimmed.split_once(':') {
                let key = key.trim().to_owned();
                let value = value.trim();
                if value.is_empty() {
                    active_list = Some(key.clone());
                    lists.entry(key).or_default();
                } else {
                    active_list = None;
                    values.insert(key.clone(), clean_scalar(value));
                    if let Some(items) = parse_inline_list(value) {
                        lists.insert(key, items);
                    }
                }
            }
        } else {
            body.push(line);
        }
    }
    Frontmatter {
        values,
        lists,
        body: body.join("\n"),
    }
}

fn parse_inline_list(value: &str) -> Option<Vec<String>> {
    let trimmed = value.trim();
    let inner = trimmed.strip_prefix('[')?.strip_suffix(']')?;
    Some(
        inner
            .split(',')
            .map(clean_scalar)
            .filter(|item| !item.is_empty())
            .collect(),
    )
}

fn clean_scalar(value: &str) -> String {
    value
        .split_once('#')
        .map_or(value, |(before_comment, _)| before_comment)
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .to_owned()
}
