//! Spec-rule coverage integrity check.
//!
//! Reads the spec-rule registry (`docs/registries/spec-rules.md`) and emits
//! [`CAIRN_SPEC_RULE_UNIMPLEMENTED`] (registry code CK004) when a rule's
//! enforcing `CAIRN_*` code is not emitted in non-test `src/` source. This turns
//! a Designed-but-unimplemented spec rule from prose that silently passes scan
//! into tracked cairn state, per spec.md:24.
//!
//! Severity follows the rule's status:
//! - `enforced`: the rule is built, so a missing emitter is a regression and
//!   surfaces a **Warning** (promoted by `cairn scan --strict`).
//! - `pending`: the rule is Designed but not yet built, so a missing emitter
//!   surfaces an **Info** advisory: visible and tracked, but it does not block
//!   `--strict` (which `cairn accept` runs), so an unbuilt rule does not wedge
//!   every future change.
//! - `declared`: exempt (named but not yet designed enough to enforce).
//!
//! See `meta/decisions/dec.ghost-rule-tracking.md` for rationale.

use std::{fs, path::Path};

use super::graph::{Finding, FindingSeverity, Graph};

const REGISTRY: &str = "docs/registries/spec-rules.md";

/// Enforcement status of a registered spec rule.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Status {
    /// Built: enforcer must be emitted; a missing emitter is a regression.
    Enforced,
    /// Designed but not yet built: a missing enforcer is a tracked advisory.
    Pending,
    /// Declared maturity: exempt, listed for completeness.
    Declared,
}

/// A registered spec rule parsed from one registry table row.
struct SpecRule {
    rule: String,
    spec: String,
    /// Enforcing `CAIRN_*` code, or `None` when no enforcer is named.
    code: Option<String>,
    status: Status,
    /// Decision artefact deferring the rule's build, from the optional
    /// `Deferred-by` column, or `None` when no deferral is recorded.
    deferred_by: Option<String>,
}

/// Emits `CAIRN_SPEC_RULE_UNIMPLEMENTED` for spec rules whose enforcer is not
/// emitted in non-test source. No-op when the registry is absent, so projects
/// without a spec-rule registry are unaffected.
pub(crate) fn validate_spec_rule_coverage(graph: &mut Graph, root: &Path) {
    let Ok(registry) = fs::read_to_string(root.join(REGISTRY)) else {
        return;
    };
    let rules = parse_rules(&registry);
    if rules.is_empty() {
        return;
    }
    let corpus = production_source(&root.join("src"));
    for rule in rules {
        let severity = match rule.status {
            Status::Enforced => FindingSeverity::Warning,
            Status::Pending => FindingSeverity::Info,
            Status::Declared => continue,
        };
        let emitted = rule
            .code
            .as_deref()
            .is_some_and(|code| is_emitted(&corpus, code));
        if emitted {
            continue;
        }
        let mut message = match &rule.code {
            Some(code) => format!(
                "spec rule `{}` ({}) is {} but its enforcer `{code}` is not emitted in non-test source",
                rule.rule,
                rule.spec,
                rule.status.label()
            ),
            None => format!(
                "spec rule `{}` ({}) is {} but names no enforcer",
                rule.rule,
                rule.spec,
                rule.status.label()
            ),
        };
        if let Some(dec) = &rule.deferred_by {
            message.push_str(" (deferred by ");
            message.push_str(dec);
            message.push(')');
        }
        graph.findings.push(Finding {
            code: "CAIRN_SPEC_RULE_UNIMPLEMENTED".to_owned(),
            severity,
            message,
            node: None,
            // Per-rule identity (spec + description) keys uniqueness so sibling
            // rules sharing one spec anchor do not dedup-collapse.
            target: Some(format!("{} {}", rule.spec, rule.rule)),
            path: Some(REGISTRY.to_owned()),
        });
    }
}

impl Status {
    const fn label(self) -> &'static str {
        match self {
            Self::Enforced => "enforced",
            Self::Pending => "pending",
            Self::Declared => "declared",
        }
    }
}

/// Parses registry table rows. A data row has four cells (plus an optional
/// fifth `Deferred-by` cell) and a known status; every other line (prose,
/// headers, separators, the format table) is ignored.
fn parse_rules(registry: &str) -> Vec<SpecRule> {
    let mut rules = Vec::new();
    for line in registry.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = trimmed
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect();
        if cells.len() != 4 && cells.len() != 5 {
            continue;
        }
        let Some(status) = parse_status(cells[3]) else {
            continue;
        };
        rules.push(SpecRule {
            rule: cells[0].to_owned(),
            spec: cells[1].to_owned(),
            code: parse_code(cells[2]),
            status,
            // The Deferred-by cell is a pending-row contract: an enforced rule
            // regression must never render as deliberately deferred.
            deferred_by: (status == Status::Pending)
                .then(|| cells.get(4).and_then(|cell| parse_code(cell)))
                .flatten(),
        });
    }
    rules
}

fn parse_status(cell: &str) -> Option<Status> {
    match cell.to_ascii_lowercase().as_str() {
        "enforced" => Some(Status::Enforced),
        "pending" => Some(Status::Pending),
        "declared" => Some(Status::Declared),
        _ => None,
    }
}

/// Strips backticks/whitespace from a code cell. An empty or `-` cell means no
/// enforcer is named.
fn parse_code(cell: &str) -> Option<String> {
    let code = cell.trim().trim_matches('`').trim();
    if code.is_empty() || code == "-" {
        None
    } else {
        Some(code.to_owned())
    }
}

/// True when `code` appears as a string literal at an emission site in the
/// corpus: the `"code"` literal is immediately preceded (ignoring whitespace) by
/// a finding-emitting call (`error(`, `warning(`, `info(`, `error_finding(`) or
/// a `code:` struct field, with an identifier boundary so `my_error(` does not
/// match. Line comments are stripped before scanning, so a commented-out emit
/// and a bare reference (match arm, remediation handler, doc comment) do not
/// count.
fn is_emitted(corpus: &str, code: &str) -> bool {
    const ANCHORS: [&str; 5] = ["error(", "warning(", "info(", "error_finding(", "code:"];
    let needle = format!("\"{code}\"");
    let mut from = 0;
    while let Some(rel) = corpus[from..].find(&needle) {
        let idx = from + rel;
        let prefix = corpus[..idx].trim_end();
        if ANCHORS
            .iter()
            .any(|anchor| ends_with_anchor(prefix, anchor))
        {
            return true;
        }
        from = idx + needle.len();
    }
    false
}

/// True when `prefix` ends with `anchor` and the character before the anchor is
/// not an identifier character, so `error(` matches `push(error(` but not
/// `my_error(`.
fn ends_with_anchor(prefix: &str, anchor: &str) -> bool {
    let Some(head) = prefix.strip_suffix(anchor) else {
        return false;
    };
    head.chars()
        .next_back()
        .is_none_or(|ch| !ch.is_alphanumeric() && ch != '_')
}

/// Concatenates non-test Rust source under `src_dir`: skips any `tests/`
/// directory and `tests.rs` file, and truncates each file at its first
/// `#[cfg(test)]` so inline test modules (which reference codes without
/// emitting them) are excluded.
fn production_source(src_dir: &Path) -> String {
    let mut corpus = String::new();
    collect_production(src_dir, &mut corpus);
    corpus
}

fn collect_production(dir: &Path, corpus: &mut String) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|name| name == "tests") {
                continue;
            }
            collect_production(&path, corpus);
            continue;
        }
        if path.extension().is_none_or(|ext| ext != "rs") {
            continue;
        }
        if path.file_name().is_some_and(|name| name == "tests.rs") {
            continue;
        }
        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };
        for line in content.lines() {
            // Stop at the first inline `#[cfg(test)]` attribute (line-anchored
            // so a `"#[cfg(test)]"` string literal, e.g. a const marker, does
            // not prematurely truncate the production source).
            if line.trim_start().starts_with("#[cfg(test)]") {
                break;
            }
            corpus.push_str(strip_line_comment(line));
            corpus.push('\n');
        }
    }
}

/// Returns `line` up to the first `//` that is not inside a double-quoted string
/// literal, so a commented-out emission is dropped while a code literal sharing
/// a line with a `//`-bearing string (e.g. a URL) is preserved.
fn strip_line_comment(line: &str) -> &str {
    let bytes = line.as_bytes();
    let mut in_string = false;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' if in_string => i += 1,
            b'"' => in_string = !in_string,
            b'/' if !in_string && bytes.get(i + 1) == Some(&b'/') => return &line[..i],
            _ => {}
        }
        i += 1;
    }
    line
}

#[cfg(test)]
mod tests;
