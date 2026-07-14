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

/// Returns `source` with the frontmatter field `key` set to `value`, leaving
/// every other byte (ordering, indentation, body, and line endings) untouched.
///
/// Requires the document to open with a `---` fence; the closing `---` fence is
/// the next one. Within that block it replaces the value on the line whose key
/// (the text before the first `:`) is exactly `key` and which is top-level
/// (not indented). Returns `None` when the document is not a valid frontmatter
/// block, the key is absent, or only a nested (indented) key exists. Lines are
/// rejoined with `\n`, so the edit is byte-for-byte identical for both LF and
/// CRLF inputs except for the single changed value: a CRLF target line keeps its
/// trailing `\r`, and every other line keeps its original ending.
#[must_use]
pub fn set_field(source: &str, key: &str, value: &str) -> Option<String> {
    let lines: Vec<&str> = source.split('\n').collect();
    let is_fence = |l: &&str| l.strip_suffix('\r').unwrap_or(*l) == "---";
    if !lines.first().is_some_and(is_fence) {
        return None;
    }
    let open = 0;
    let close = lines[open + 1..].iter().position(is_fence)?;
    let close = open + 1 + close;
    let mut target: Option<usize> = None;
    for (i, line) in lines[open + 1..close].iter().enumerate() {
        let stripped = line.strip_suffix('\r').unwrap_or(*line);
        if stripped.starts_with(char::is_whitespace) {
            continue; // only top-level keys are eligible
        }
        let Some((k, _)) = stripped.split_once(':') else {
            continue;
        };
        if k == key {
            target = Some(open + 1 + i);
            break;
        }
    }
    let target = target?;
    let target_line = lines[target];
    let indent = target_line.len() - target_line.trim_start().len();
    let mut new_line = format!("{}{key}: {value}", &target_line[..indent]);
    if target_line.ends_with('\r') {
        new_line.push('\r');
    }
    let mut out: Vec<&str> = Vec::with_capacity(lines.len());
    for (i, line) in lines.iter().enumerate() {
        if i == target {
            out.push(new_line.as_str());
        } else {
            out.push(line);
        }
    }
    Some(out.join("\n"))
}
