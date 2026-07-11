// cairn:allow-large-module reason: extracted test module for spec-rule coverage (convention: tests.rs split from parent when parent exceeds the line limit)
//! Tests for spec-rule coverage: registry parsing, emission detection, and
//! finding severity/message rendering (including deferral suffixes).
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
    // No Deferred-by cell: message renders unchanged, no deferral suffix.
    assert_eq!(
        g.findings[0].message,
        "spec rule `R` (spec:634) is pending but names no enforcer"
    );
}

#[test]
fn pending_rule_with_deferral_names_decision() {
    let (_d, g) = run(
        "| R | spec:634 | - | pending | dec.revisit-trigger-correlator-deferred |\n",
        "src/check.rs",
        "fn f() {}",
    );
    assert_eq!(codes(&g), vec!["CAIRN_SPEC_RULE_UNIMPLEMENTED"]);
    assert_eq!(g.findings[0].severity, FindingSeverity::Info);
    assert_eq!(
        g.findings[0].message,
        "spec rule `R` (spec:634) is pending but names no enforcer \
         (deferred by dec.revisit-trigger-correlator-deferred)"
    );
}

#[test]
fn dash_deferred_by_cell_renders_unchanged() {
    let (_d, g) = run(
        "| R | spec:634 | - | pending | - |\n",
        "src/check.rs",
        "fn f() {}",
    );
    assert_eq!(
        g.findings[0].message,
        "spec rule `R` (spec:634) is pending but names no enforcer"
    );
}

#[test]
fn enforced_rule_with_deferral_cell_ignores_it() {
    // Deferred-by is a pending-row contract: a stale fifth cell on an
    // enforced row must not render a regression as deliberately deferred.
    let (_d, g) = run(
        "| R | spec:1 | `CAIRN_FOO` | enforced | dec.stale-deferral |\n",
        "src/check.rs",
        "fn f() {}",
    );
    assert_eq!(codes(&g), vec!["CAIRN_SPEC_RULE_UNIMPLEMENTED"]);
    assert_eq!(g.findings[0].severity, FindingSeverity::Warning);
    assert!(
        !g.findings[0].message.contains("deferred by"),
        "{}",
        g.findings[0].message
    );
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
