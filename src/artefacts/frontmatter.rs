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

/// Failure modes of [`set_field`], surfaced to the user with distinct messages.
#[derive(Debug, PartialEq)]
pub enum SetFieldError {
    /// The document has no valid frontmatter fence: either it does not open
    /// with `---`, or no closing `---` appears before EOF.
    NoFrontmatter,
    /// A valid frontmatter block exists, but it contains no top-level `key`.
    KeyNotFound,
}

/// Returns `source` with the frontmatter field `key` set to `value`, leaving
/// every other byte (ordering, indentation, body, and line endings) untouched.
///
/// Requires the document to open with a `---` fence; the closing `---` fence is
/// the next one. Within that block it replaces the value on the line whose key
/// (the text before the first `:`) is exactly `key` and which is top-level
/// (not indented).
///
/// On success the edited string is byte-for-byte identical for both LF and CRLF
/// inputs except for the single changed value: a CRLF target line keeps its
/// trailing `\r`, and every other line keeps its original ending.
///
/// # Errors
///
/// Returns `Err(NoFrontmatter)` when the document is not a valid frontmatter
/// block (no opening fence, or no closing fence before EOF), and
/// `Err(KeyNotFound)` when a valid frontmatter block exists but contains no
/// top-level `key` (a nested/indented key does not count).
pub fn set_field(source: &str, key: &str, value: &str) -> Result<String, SetFieldError> {
    let lines: Vec<&str> = source.split('\n').collect();
    let close = closing_fence(&lines)?;
    let Some(target) = find_top_level_key(&lines, close, key) else {
        return Err(SetFieldError::KeyNotFound);
    };
    let new_line = match_line_ending(&format!("{key}: {value}"), lines[target]);
    let mut out: Vec<&str> = Vec::with_capacity(lines.len());
    for (i, line) in lines.iter().enumerate() {
        if i == target {
            out.push(new_line.as_str());
        } else {
            out.push(line);
        }
    }
    Ok(out.join("\n"))
}

/// Returns `source` with the frontmatter field `key` set to `value`,
/// inserting `key: value` as the last frontmatter line when the key is
/// absent. A present key is replaced together with any indented block-list
/// lines beneath it, so a YAML block list collapses to the single inline
/// value rather than leaving orphaned `- item` lines. Every other byte is
/// preserved ([`set_field`] semantics).
///
/// # Errors
///
/// Returns `Err(NoFrontmatter)` when the document is not a valid
/// frontmatter block.
pub fn upsert_field(source: &str, key: &str, value: &str) -> Result<String, SetFieldError> {
    let lines: Vec<&str> = source.split('\n').collect();
    let close = closing_fence(&lines)?;
    // A replaced key keeps its own line ending; an inserted line borrows
    // the last frontmatter line's (the fence element carries no `\r` when
    // the document ends at the fence without a trailing newline).
    let (replace_from, replace_to, template) = match find_top_level_key(&lines, close, key) {
        Some(target) => (target, block_extent(&lines, target, close), lines[target]),
        None => (close, close, lines[close - 1]),
    };
    let new_line = match_line_ending(&format!("{key}: {value}"), template);
    let mut out: Vec<&str> = Vec::with_capacity(lines.len() + 1);
    for (i, line) in lines.iter().enumerate() {
        if i == replace_from {
            out.push(new_line.as_str());
        }
        if i < replace_from || i >= replace_to {
            out.push(line);
        }
    }
    Ok(out.join("\n"))
}

/// Returns `source` with the top-level frontmatter field `key` removed,
/// together with any indented block-list lines beneath it, leaving every
/// other byte untouched.
///
/// # Errors
///
/// Returns `Err(NoFrontmatter)` when the document is not a valid
/// frontmatter block, and `Err(KeyNotFound)` when no top-level `key`
/// exists in it.
pub fn remove_field(source: &str, key: &str) -> Result<String, SetFieldError> {
    let lines: Vec<&str> = source.split('\n').collect();
    let close = closing_fence(&lines)?;
    let Some(target) = find_top_level_key(&lines, close, key) else {
        return Err(SetFieldError::KeyNotFound);
    };
    let end = block_extent(&lines, target, close);
    let mut out: Vec<&str> = Vec::with_capacity(lines.len());
    for (i, line) in lines.iter().enumerate() {
        if i < target || i >= end {
            out.push(line);
        }
    }
    Ok(out.join("\n"))
}

/// Index of the closing `---` fence, or `NoFrontmatter` when either fence
/// is missing.
fn closing_fence(lines: &[&str]) -> Result<usize, SetFieldError> {
    let is_fence = |l: &&str| l.strip_suffix('\r').unwrap_or(*l) == "---";
    if !lines.first().is_some_and(is_fence) {
        return Err(SetFieldError::NoFrontmatter);
    }
    match lines[1..].iter().position(is_fence) {
        Some(c) => Ok(1 + c),
        None => Err(SetFieldError::NoFrontmatter),
    }
}

/// Index of the top-level `key:` line inside the frontmatter block, if any.
fn find_top_level_key(lines: &[&str], close: usize, key: &str) -> Option<usize> {
    for (i, line) in lines[1..close].iter().enumerate() {
        let stripped = line.strip_suffix('\r').unwrap_or(*line);
        if stripped.starts_with(char::is_whitespace) {
            continue; // only top-level keys are eligible
        }
        let Some((k, _)) = stripped.split_once(':') else {
            continue;
        };
        if k == key {
            return Some(1 + i);
        }
    }
    None
}

/// One past the last line belonging to the key at `target`: the key line
/// plus every following indented line (a YAML block list or nested map)
/// up to the next top-level key or the closing fence.
fn block_extent(lines: &[&str], target: usize, close: usize) -> usize {
    let mut end = target + 1;
    let mut last_member = target;
    while end < close {
        let stripped = lines[end].strip_suffix('\r').unwrap_or(lines[end]);
        if stripped.trim().is_empty() {
            end += 1; // provisional separator: kept only if a member follows
        } else if stripped.starts_with(char::is_whitespace) {
            end += 1;
            last_member = end - 1;
        } else {
            break;
        }
    }
    // Trailing blank lines are separators, not block members; leave them.
    last_member + 1
}

/// Copies the line ending of `template` onto `line` (a `\r` when the
/// template line ends in one, else nothing; the `\n` separator is owned by
/// the caller's join).
fn match_line_ending(line: &str, template: &str) -> String {
    if template.ends_with('\r') {
        format!("{line}\r")
    } else {
        line.to_owned()
    }
}
