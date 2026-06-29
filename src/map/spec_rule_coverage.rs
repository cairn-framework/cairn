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
        let message = match &rule.code {
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

/// Parses registry table rows. A data row has exactly four cells and a known
/// status; every other line (prose, headers, separators, the format table) is
/// ignored.
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
        if cells.len() != 4 {
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
mod tests {
    use super::*;
    use crate::map::graph::Graph;
    use std::collections::BTreeMap;
    use std::fs;

    fn empty_graph() -> Graph {
        Graph {
            nodes: BTreeMap::new(),
            names: BTreeMap::new(),
            outbound: BTreeMap::new(),
            inbound: BTreeMap::new(),
            findings: Vec::new(),
        }
    }

    fn codes(g: &Graph) -> Vec<&str> {
        g.findings.iter().map(|f| f.code.as_str()).collect()
    }

    fn write(root: &Path, rel: &str, content: &str) {
        let path = root.join(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    fn registry(rows: &str) -> String {
        format!("| Rule | Spec | Code | Status |\n|---|---|---|---|\n{rows}")
    }

    /// Builds a temp project with the given registry rows and source, runs the
    /// check, returns the graph.
    fn run(rows: &str, src_rel: &str, src: &str) -> (tempfile::TempDir, Graph) {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), REGISTRY, &registry(rows));
        write(dir.path(), src_rel, src);
        let mut g = empty_graph();
        validate_spec_rule_coverage(&mut g, dir.path());
        (dir, g)
    }

    #[test]
    fn absent_registry_no_finding() {
        let dir = tempfile::tempdir().unwrap();
        let mut g = empty_graph();
        validate_spec_rule_coverage(&mut g, dir.path());
        assert!(g.findings.is_empty());
    }

    #[test]
    fn enforced_rule_with_emitted_code_no_finding() {
        let (_d, g) = run(
            "| R | spec:1 | `CAIRN_FOO` | enforced |\n",
            "src/check.rs",
            "fn f() { warning(\n    \"CAIRN_FOO\",\n    msg); }",
        );
        assert!(g.findings.is_empty(), "{:?}", codes(&g));
    }

    #[test]
    fn enforced_rule_missing_code_warns() {
        let (_d, g) = run(
            "| R | spec:1 | `CAIRN_FOO` | enforced |\n",
            "src/check.rs",
            "fn f() {}",
        );
        assert_eq!(codes(&g), vec!["CAIRN_SPEC_RULE_UNIMPLEMENTED"]);
        assert_eq!(g.findings[0].severity, FindingSeverity::Warning);
    }

    #[test]
    fn pending_rule_without_code_is_info() {
        let (_d, g) = run(
            "| R | spec:634 | - | pending |\n",
            "src/check.rs",
            "fn f() {}",
        );
        assert_eq!(codes(&g), vec!["CAIRN_SPEC_RULE_UNIMPLEMENTED"]);
        assert_eq!(g.findings[0].severity, FindingSeverity::Info);
    }

    #[test]
    fn pending_rule_with_emitted_code_no_finding() {
        // Once a pending rule is implemented, its emitter clears the finding.
        let (_d, g) = run(
            "| R | spec:634 | `CAIRN_FOO` | pending |\n",
            "src/check.rs",
            "fn f() { info(\"CAIRN_FOO\", m); }",
        );
        assert!(g.findings.is_empty(), "{:?}", codes(&g));
    }

    #[test]
    fn declared_rule_exempt() {
        let (_d, g) = run(
            "| R | spec:635 | - | declared |\n",
            "src/check.rs",
            "fn f() {}",
        );
        assert!(g.findings.is_empty());
    }

    #[test]
    fn code_only_in_match_arm_is_not_emission() {
        // Bare reference (remediation handler / match arm), not an emission.
        let (_d, g) = run(
            "| R | spec:1 | `CAIRN_FOO` | enforced |\n",
            "src/handler.rs",
            "fn f(c: &str) { match c { \"CAIRN_FOO\" => take(), _ => {} } }",
        );
        assert_eq!(codes(&g), vec!["CAIRN_SPEC_RULE_UNIMPLEMENTED"]);
    }

    #[test]
    fn code_only_in_inline_test_module_is_excluded() {
        // Emission lives only inside an inline #[cfg(test)] module.
        let (_d, g) = run(
            "| R | spec:1 | `CAIRN_FOO` | enforced |\n",
            "src/check.rs",
            "fn f() {}\n#[cfg(test)]\nmod tests { fn t() { warning(\"CAIRN_FOO\", m); } }",
        );
        assert_eq!(codes(&g), vec!["CAIRN_SPEC_RULE_UNIMPLEMENTED"]);
    }

    #[test]
    fn cfg_test_in_string_literal_does_not_truncate_production() {
        // A const marker mentions `#[cfg(test)]` as a string before the real
        // emission; the production source must not truncate at the literal.
        let (_d, g) = run(
            "| R | spec:1 | `CAIRN_FOO` | enforced |\n",
            "src/check.rs",
            "const MARKER: &str = \"#[cfg(test)]\";\nfn f() { warning(\"CAIRN_FOO\", m); }",
        );
        assert!(g.findings.is_empty(), "{:?}", codes(&g));
    }

    #[test]
    fn code_in_separate_tests_file_is_excluded() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            REGISTRY,
            &registry("| R | spec:1 | `CAIRN_FOO` | enforced |\n"),
        );
        write(dir.path(), "src/check.rs", "fn f() {}");
        write(
            dir.path(),
            "src/tests.rs",
            "fn t() { warning(\"CAIRN_FOO\", m); }",
        );
        let mut g = empty_graph();
        validate_spec_rule_coverage(&mut g, dir.path());
        assert_eq!(codes(&g), vec!["CAIRN_SPEC_RULE_UNIMPLEMENTED"]);
    }

    #[test]
    fn struct_literal_code_field_counts_as_emission() {
        let (_d, g) = run(
            "| R | spec:1 | `CAIRN_FOO` | enforced |\n",
            "src/check.rs",
            "fn f() { push(Finding { code: \"CAIRN_FOO\".to_owned() }); }",
        );
        assert!(g.findings.is_empty(), "{:?}", codes(&g));
    }

    #[test]
    fn same_spec_anchor_rules_do_not_dedup_collapse() {
        // Two rules sharing one spec anchor must produce two distinct findings;
        // the scanner dedups on (code, node, path, target).
        let (_d, g) = run(
            "| Rule A | spec:61 | - | enforced |\n| Rule B | spec:61 | - | enforced |\n",
            "src/check.rs",
            "fn f() {}",
        );
        assert_eq!(g.findings.len(), 2);
        let mut targets: Vec<&str> = g
            .findings
            .iter()
            .filter_map(|f| f.target.as_deref())
            .collect();
        targets.sort_unstable();
        assert_eq!(targets, vec!["spec:61 Rule A", "spec:61 Rule B"]);
    }

    #[test]
    fn commented_out_emission_is_not_emission() {
        let (_d, g) = run(
            "| R | spec:1 | `CAIRN_FOO` | enforced |\n",
            "src/check.rs",
            "fn f() { // warning(\"CAIRN_FOO\", m);\n}",
        );
        assert_eq!(codes(&g), vec!["CAIRN_SPEC_RULE_UNIMPLEMENTED"]);
    }

    #[test]
    fn suffix_identifier_call_is_not_emission() {
        let (_d, g) = run(
            "| R | spec:1 | `CAIRN_FOO` | enforced |\n",
            "src/check.rs",
            "fn f() { my_error(\"CAIRN_FOO\", m); }",
        );
        assert_eq!(codes(&g), vec!["CAIRN_SPEC_RULE_UNIMPLEMENTED"]);
    }

    #[test]
    fn error_finding_wrapper_counts_as_emission() {
        let (_d, g) = run(
            "| R | spec:1 | `CAIRN_FOO` | enforced |\n",
            "src/check.rs",
            "fn f() { io::error_finding(\"CAIRN_FOO\", m, None); }",
        );
        assert!(g.findings.is_empty(), "{:?}", codes(&g));
    }

    #[test]
    fn url_in_string_preserves_code_on_same_line() {
        // A `//` inside a string literal must not truncate a real emission.
        let (_d, g) = run(
            "| R | spec:1 | `CAIRN_FOO` | enforced |\n",
            "src/check.rs",
            "fn f() { let u = \"https://x\"; warning(\"CAIRN_FOO\", u); }",
        );
        assert!(g.findings.is_empty(), "{:?}", codes(&g));
    }
}
