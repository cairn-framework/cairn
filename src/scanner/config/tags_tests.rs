//! Integration tests for scanner tag registry findings.

#[test]
fn tag_registry_finding_reaches_load_project_as_info() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("cairn.blueprint"),
        r#"System App "app" id "app" @unknown {
}
"#,
    )
    .unwrap();
    std::fs::write(
        dir.path().join("cairn.config.yaml"),
        "tags:\n  known:\n    description: A known tag\n",
    )
    .unwrap();

    let result =
        crate::scanner::load_project(dir.path(), &dir.path().join("cairn.blueprint")).unwrap();
    let findings: Vec<_> = result
        .graph
        .findings
        .iter()
        .filter(|finding| finding.code == "CAIRN_TAG_UNREGISTERED")
        .collect();
    assert_eq!(findings.len(), 1);
    assert_eq!(
        findings[0].severity,
        crate::map::graph::FindingSeverity::Info
    );
    assert_eq!(findings[0].node.as_deref(), Some("app"));
}

#[test]
fn absent_tag_registry_emits_no_tag_findings() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("cairn.blueprint"),
        r#"System App "app" id "app" @unknown {
}
"#,
    )
    .unwrap();

    let result =
        crate::scanner::load_project(dir.path(), &dir.path().join("cairn.blueprint")).unwrap();
    assert!(
        !result
            .graph
            .findings
            .iter()
            .any(|finding| finding.code == "CAIRN_TAG_UNREGISTERED")
    );
}
