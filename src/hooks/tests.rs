//! Tests for hook helpers: finding classification and change-id parsing.

use super::*;
use crate::map::graph::FindingSeverity;

fn finding(severity: FindingSeverity, code: &str) -> Finding {
    Finding {
        code: code.to_owned(),
        severity,
        message: "test".to_owned(),
        node: None,
        target: None,
        path: None,
    }
}

// ── structural_findings / tension_findings ────────────────────────────────

#[test]
fn test_structural_findings_keeps_only_errors() {
    let findings = vec![
        finding(FindingSeverity::Error, "ERR"),
        finding(FindingSeverity::Warning, "WARN"),
        finding(FindingSeverity::Info, "INFO"),
    ];
    let out = structural_findings(&findings);
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].code, "ERR");
}

#[test]
fn test_tension_findings_keeps_warning_and_info_not_error() {
    let findings = vec![
        finding(FindingSeverity::Error, "ERR"),
        finding(FindingSeverity::Warning, "WARN"),
        finding(FindingSeverity::Info, "INFO"),
    ];
    let out = tension_findings(&findings);
    assert_eq!(out.len(), 2);
    let codes: Vec<&str> = out.iter().map(|f| f.code.as_str()).collect();
    assert!(codes.contains(&"WARN"));
    assert!(codes.contains(&"INFO"));
    assert!(!codes.contains(&"ERR"));
}

// ── is_id_char ────────────────────────────────────────────────────────────

#[test]
fn test_is_id_char_accepts_valid_chars() {
    for ch in "abcz0189.-_".chars() {
        assert!(is_id_char(ch), "expected {ch:?} to be a valid id char");
    }
}

#[test]
fn test_is_id_char_rejects_invalid_chars() {
    for ch in "ABCZ /\\@#".chars() {
        assert!(!is_id_char(ch), "expected {ch:?} to be invalid id char");
    }
}

// ── clean_id ──────────────────────────────────────────────────────────────

#[test]
fn test_clean_id_trims_whitespace() {
    assert_eq!(clean_id("  app.api  "), "app.api");
}

#[test]
fn test_clean_id_strips_double_quotes() {
    assert_eq!(clean_id(r#""app.api""#), "app.api");
}

#[test]
fn test_clean_id_strips_backticks() {
    assert_eq!(clean_id("`app.api`"), "app.api");
}

#[test]
fn test_clean_id_strips_trailing_comma() {
    assert_eq!(clean_id("app.api,"), "app.api");
}

#[test]
fn test_clean_id_plain_id_unchanged() {
    assert_eq!(clean_id("app.api"), "app.api");
}

// ── field_value ───────────────────────────────────────────────────────────

#[test]
fn test_field_value_extracts_quoted_value() {
    assert_eq!(
        field_value(r#"Module Api id "app.api" {"#, "id"),
        Some("app.api".to_owned())
    );
}

#[test]
fn test_field_value_absent_field_returns_none() {
    assert_eq!(field_value("Module Api {}", "id"), None);
}

// ── parse_edge ────────────────────────────────────────────────────────────

#[test]
fn test_parse_edge_with_spaces() {
    assert_eq!(
        parse_edge("app.api -> app.db"),
        Some(("app.api".to_owned(), "app.db".to_owned()))
    );
}

#[test]
fn test_parse_edge_without_spaces() {
    assert_eq!(
        parse_edge("app.api->app.db"),
        Some(("app.api".to_owned(), "app.db".to_owned()))
    );
}

#[test]
fn test_parse_edge_with_description_ignores_description() {
    assert_eq!(
        parse_edge(r#"app.api -> app.db "dep""#),
        Some(("app.api".to_owned(), "app.db".to_owned()))
    );
}

#[test]
fn test_parse_edge_no_arrow_returns_none() {
    assert_eq!(parse_edge("app.api app.db"), None);
}

// ── ids_from_text ─────────────────────────────────────────────────────────

#[test]
fn test_ids_from_text_extracts_dotted_id_from_bullet() {
    let ids = ids_from_text("- app.api");
    assert_eq!(ids, vec!["app.api"]);
}

#[test]
fn test_ids_from_text_extracts_id_field() {
    let ids = ids_from_text(r#"Module Api id "app.api" {"#);
    assert_eq!(ids, vec!["app.api"]);
}

#[test]
fn test_ids_from_text_no_dotted_id_returns_empty() {
    let ids = ids_from_text("# ADDED Nodes");
    assert!(ids.is_empty(), "heading must produce no ids: {ids:?}");
}

#[test]
fn test_ids_from_text_empty_id_field_not_included() {
    // field_value returns Some("") for id "".
    // The empty string must not appear in the output — it is not a
    // valid node ID and would insert "node:" into the targets set.
    let ids = ids_from_text(r#"Module id """#);
    assert!(
        !ids.contains(&String::new()),
        "empty string must not be in ids output: {ids:?}"
    );
}
