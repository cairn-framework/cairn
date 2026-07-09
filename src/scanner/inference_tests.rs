//! Tests for directory language inference and target reconciliation behaviour.

use std::path::Path;

use crate::reconcile::{ReconcileReport, fingerprint::InterfaceFingerprint, target::Language};

fn temp_dir(prefix: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("cairn-{prefix}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn cleanup(dir: &std::path::Path) {
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn infer_from_directory_typescript() {
    let dir = temp_dir("infer-ts");
    let src = dir.join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join("index.ts"), "export const x = 1;").unwrap();
    let lang = Language::infer_from_directory(&dir, Path::new("src"), &[]);
    assert_eq!(lang, Some(Language::TypeScript));
    cleanup(&dir);
}

#[test]
fn infer_from_directory_mixed_picks_dominant() {
    let dir = temp_dir("infer-mixed");
    let src = dir.join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join("a.rs"), "pub fn a() {}").unwrap();
    std::fs::write(src.join("b.rs"), "pub fn b() {}").unwrap();
    std::fs::write(src.join("c.ts"), "export const c = 1;").unwrap();
    let lang = Language::infer_from_directory(&dir, Path::new("src"), &[]);
    assert_eq!(lang, Some(Language::Rust));
    cleanup(&dir);
}

#[test]
fn infer_from_directory_tie_breaks_by_supported_order() {
    let dir = temp_dir("infer-tie");
    let src = dir.join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join("a.rs"), "pub fn a() {}").unwrap();
    std::fs::write(src.join("b.ts"), "export const b = 1;").unwrap();
    let lang = Language::infer_from_directory(&dir, Path::new("src"), &[]);
    assert_eq!(lang, Some(Language::Rust));
    cleanup(&dir);
}

#[test]
fn infer_from_directory_empty_returns_none() {
    let dir = temp_dir("infer-empty");
    let src = dir.join("src");
    std::fs::create_dir_all(&src).unwrap();
    let lang = Language::infer_from_directory(&dir, Path::new("src"), &[]);
    assert_eq!(lang, None);
    cleanup(&dir);
}

#[test]
fn infer_from_directory_unsupported_returns_none() {
    let dir = temp_dir("infer-unsupported");
    let src = dir.join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join("README.md"), "# readme").unwrap();
    std::fs::write(src.join("config.json"), "{}").unwrap();
    let lang = Language::infer_from_directory(&dir, Path::new("src"), &[]);
    assert_eq!(lang, None);
    cleanup(&dir);
}

#[test]
fn scan_directory_target_infers_typescript() {
    let dir = temp_dir("scan-ts-dir");
    let src = dir.join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join("index.ts"), "export const x = 1;").unwrap();
    std::fs::write(
        dir.join("cairn.blueprint"),
        r#"System App "app" id "app" {
    Module Core "core" id "app.core" {
        path "./src"
    }
}
"#,
    )
    .unwrap();
    let result = super::load_project(&dir, &dir.join("cairn.blueprint")).unwrap();
    let reports: Vec<_> = result.target_reports.iter().collect();
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0].language, Language::TypeScript);
    assert!(reports[0].hash.is_some());
    assert!(
        reports[0]
            .claimed_files
            .iter()
            .any(|f| f.ends_with("index.ts"))
    );
    cleanup(&dir);
}

#[test]
fn scan_empty_directory_target_is_unknown_with_warning() {
    let dir = temp_dir("scan-empty-dir");
    let src = dir.join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(
        dir.join("cairn.blueprint"),
        r#"System App "app" id "app" {
    Module Core "core" id "app.core" {
        path "./src"
    }
}
"#,
    )
    .unwrap();
    let result = super::load_project(&dir, &dir.join("cairn.blueprint")).unwrap();
    let reports: Vec<_> = result.target_reports.iter().collect();
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0].language, Language::Unknown);
    assert!(reports[0].hash.is_none());
    assert!(
        result
            .graph
            .findings
            .iter()
            .any(|f| f.code == "CAIRN_RECONCILE_LANGUAGE_UNKNOWN")
    );
    cleanup(&dir);
}

#[test]
fn scan_targets_override_wins_over_inference() {
    let dir = temp_dir("scan-override");
    let src = dir.join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join("index.ts"), "export const x = 1;").unwrap();
    std::fs::write(src.join("lib.rs"), "pub fn lib() {}").unwrap();
    std::fs::write(
        dir.join("cairn.blueprint"),
        r#"System App "app" id "app" {
    Module Core "core" id "app.core" {
        path "./src"
    }
}
"#,
    )
    .unwrap();
    std::fs::write(
        dir.join("cairn.config.yaml"),
        r"targets:
  - node: app.core
    language: rust
    contract_role: public_api
",
    )
    .unwrap();
    let result = super::load_project(&dir, &dir.join("cairn.blueprint")).unwrap();
    let reports: Vec<_> = result.target_reports.iter().collect();
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0].language, Language::Rust);
    cleanup(&dir);
}

#[test]
fn cached_empty_target_report_has_no_hash() {
    let dir = temp_dir("scan-cache-empty");
    let src = dir.join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(
        dir.join("cairn.blueprint"),
        r#"System App "app" id "app" {
    Module Core "core" id "app.core" {
        path "./src"
    }
}
"#,
    )
    .unwrap();
    std::fs::write(
        dir.join("cairn.config.yaml"),
        r"targets:
  - node: app.core
    language: rust
    contract_role: public_api
",
    )
    .unwrap();
    let first = super::load_project(&dir, &dir.join("cairn.blueprint")).unwrap();
    let second = super::load_project(&dir, &dir.join("cairn.blueprint")).unwrap();
    let first_report = first.target_reports.first().unwrap();
    let second_report = second.target_reports.first().unwrap();
    assert_eq!(first_report.language, Language::Rust);
    assert!(first_report.hash.is_none());
    assert_eq!(second_report.language, Language::Rust);
    assert!(second_report.hash.is_none());
    cleanup(&dir);
}

#[test]
fn build_reports_from_cache_empty_target_has_no_hash() {
    let target = crate::reconcile::target::Target::new(
        "app.core".to_owned(),
        std::path::PathBuf::from("src"),
        Language::Rust,
    );
    let mut cached = std::collections::BTreeMap::new();
    cached.insert(
        "rust".to_owned(),
        ReconcileReport {
            claimed_files: std::collections::BTreeMap::new(),
            symbols: std::sync::Arc::new(Vec::new()),
            node_symbols: std::collections::BTreeMap::new(),
            node_symbol_records: std::collections::BTreeMap::new(),
            fingerprint: InterfaceFingerprint::from_symbols(&[]),
            findings: Vec::new(),
        },
    );
    let config = crate::scanner::config::Config::default();
    let (reports, findings) = super::cache::build_reports_from_cache(&cached, &[target], &config);
    assert_eq!(reports.len(), 1);
    assert!(reports[0].hash.is_none());
    assert!(
        findings
            .iter()
            .any(|f| f.code == "CAIRN_RECONCILE_EMPTY_TARGET")
    );
}

#[test]
fn build_reports_from_cache_unknown_target_has_no_hash() {
    let target = crate::reconcile::target::Target::new(
        "app.core".to_owned(),
        std::path::PathBuf::from("src"),
        Language::Unknown,
    );
    let cached = std::collections::BTreeMap::new();
    let config = crate::scanner::config::Config::default();
    let (reports, findings) = super::cache::build_reports_from_cache(&cached, &[target], &config);
    assert_eq!(reports.len(), 1);
    assert!(reports[0].hash.is_none());
    assert!(
        findings
            .iter()
            .any(|f| f.code == "CAIRN_RECONCILE_LANGUAGE_UNKNOWN")
    );
}
