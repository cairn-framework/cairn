//! Minimal Markdown frontmatter parser.

use std::collections::BTreeMap;

/// Parsed frontmatter and Markdown body.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Frontmatter {
    /// Key-value frontmatter.
    pub values: BTreeMap<String, String>,
    /// Body after frontmatter.
    pub body: String,
}

/// Parses `---` delimited frontmatter with `key: value` entries.
#[must_use]
pub fn parse(source: &str) -> Frontmatter {
    let mut values = BTreeMap::new();
    let mut lines = source.lines();
    if lines.next() != Some("---") {
        return Frontmatter {
            values,
            body: source.to_owned(),
        };
    }
    let mut body = Vec::new();
    let mut in_frontmatter = true;
    for line in lines {
        if in_frontmatter && line.trim() == "---" {
            in_frontmatter = false;
            continue;
        }
        if in_frontmatter {
            if let Some((key, value)) = line.split_once(':') {
                values.insert(key.trim().to_owned(), value.trim().to_owned());
            }
        } else {
            body.push(line);
        }
    }
    Frontmatter {
        values,
        body: body.join("\n"),
    }
}
