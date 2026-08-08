//! Integration tests for scanner tag registry findings.

use super::{Config, parse_config};
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
    assert_eq!(findings[0].target.as_deref(), Some("unknown"));
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

#[test]
fn malformed_tag_registry_is_reported_instead_of_being_disabled() {
    for source in [
        "tags:\n  broken:\n    behavior_affecting: not-a-bool\n",
        "tags: not-a-map\n",
    ] {
        let mut config = Config::default();
        parse_config(source, &mut config);

        assert!(config.tags.is_none());
        let finding = config
            .findings
            .iter()
            .find(|finding| finding.code == "CAIRN_CONFIG_TAGS_INVALID")
            .expect("malformed tags must be reported");
        assert_eq!(
            finding.severity,
            crate::map::graph::FindingSeverity::Warning
        );
        assert!(finding.message.contains("tags"));
    }
}

#[test]
fn absent_tag_registry_does_not_validate_unrelated_malformed_yaml() {
    let mut config = Config::default();
    parse_config("context: [unclosed\n", &mut config);

    assert!(config.tags.is_none());
    assert!(
        config
            .findings
            .iter()
            .all(|finding| finding.code != "CAIRN_CONFIG_TAGS_INVALID")
    );
}

#[test]
fn terminal_gate_is_flushed_before_tag_registry() {
    let mut config = Config::default();
    parse_config(
        "gates:\n  - name: build\n    command: cargo build\n",
        &mut config,
    );

    assert!(config.tags.is_none());
    assert_eq!(config.gates.len(), 1);
    assert_eq!(config.gates[0].name, "build");
    assert_eq!(config.gates[0].command, "cargo build");
}

#[test]
fn reserved_names_inside_tag_registry_do_not_reconfigure_sections() {
    let mut config = Config::default();
    parse_config(
        "rules:\n  context: rule-context\n  rules: rule-rules\n  gates: rule-gates\n  artefact_types: rule-artefacts\n  tags: rule-tags\n\
targets:\n  - node: app.api\n    path: src\n    language: rust\n\
gates:\n  - name: build\n    command: true\n\
tags:\n  context:\n    description: Context tag\n  rules:\n    description: Rules tag\n  gates:\n    description: Gates tag\n  artefact_types:\n    description: Artefact tag\n  nested:\n    description: Nested tag\n    tags:\n      inner:\n        description: Inner tag\n",
        &mut config,
    );

    assert_eq!(
        config.rules.get("context").map(String::as_str),
        Some("rule-context")
    );
    assert_eq!(
        config.rules.get("rules").map(String::as_str),
        Some("rule-rules")
    );
    assert_eq!(config.targets.len(), 1);
    assert!(config.gates_configured);
    assert_eq!(config.gates.len(), 1);
    assert_eq!(config.gates[0].name, "build");
    let registry = config.tags.expect("top-level tags must be registered");
    for tag in ["context", "rules", "gates", "artefact_types", "nested"] {
        assert!(registry.contains(tag), "missing registry entry `{tag}`");
    }
    assert!(config.findings.is_empty(), "{:?}", config.findings);
}
