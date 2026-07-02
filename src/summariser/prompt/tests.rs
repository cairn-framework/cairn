// cairn:allow-large-module reason: dedicated test submodule extracted from src/summariser/prompt.rs; source module is now under the limit.
//! Tests for summariser prompt construction and size limits.

use super::*;
use crate::blueprint::NodeKind;
use crate::map::graph::{EdgeRef, Finding, FindingSeverity, Graph, NodeRecord, NodeState};
use std::collections::BTreeMap;

fn empty_graph() -> Graph {
    Graph {
        nodes: BTreeMap::new(),
        names: BTreeMap::new(),
        outbound: BTreeMap::new(),
        inbound: BTreeMap::new(),
        findings: Vec::new(),
    }
}

fn sample_node(id: &str) -> NodeRecord {
    NodeRecord {
        kind: NodeKind::Module,
        id: id.to_owned(),
        name: "Auth".to_owned(),
        description: "Authentication service".to_owned(),
        tags: vec!["core".to_owned()],
        parent: Some("app".to_owned()),
        children: vec!["app.auth.login".to_owned()],
        paths: vec!["src/auth".to_owned()],
        owns_files: true,
        contracts: vec!["contracts/auth.md".to_owned()],
        state: NodeState::Synced,
        files: vec![],
        symbols: vec![],
        span: crate::blueprint::Span::point("test", 1, 1),
    }
}

#[test]
fn test_build_request_node_not_found_errors() {
    let graph = empty_graph();
    let config = Config::default();
    let result = build_request(
        "missing",
        "contract",
        "req-1",
        &graph,
        &config,
        Path::new("."),
        &ContractSet::default(),
        10_000,
        1_000,
    );
    assert!(
        matches!(result, Err(PromptError::NodeNotFound(ref id)) if id == "missing"),
        "expected NodeNotFound, got {result:?}"
    );
}

#[test]
fn test_build_request_populates_map_facts() {
    let mut graph = empty_graph();
    graph
        .nodes
        .insert("app.auth".to_owned(), sample_node("app.auth"));
    graph.outbound.insert(
        "app.auth".to_owned(),
        vec![EdgeRef {
            from: "app.auth".to_owned(),
            to: "app.db".to_owned(),
            description: "persists sessions".to_owned(),
        }],
    );
    graph.inbound.insert(
        "app.auth".to_owned(),
        vec![EdgeRef {
            from: "app.api".to_owned(),
            to: "app.auth".to_owned(),
            description: "delegates login".to_owned(),
        }],
    );
    let config = Config::default();
    let req = build_request(
        "app.auth",
        "contract",
        "req-1",
        &graph,
        &config,
        Path::new("."),
        &ContractSet::default(),
        10_000,
        1_000,
    )
    .unwrap();

    assert!(req.map_facts.iter().any(|f| f.contains("kind: Module")));
    assert!(req.map_facts.iter().any(|f| f.contains("name: Auth")));
    assert!(
        req.map_facts
            .iter()
            .any(|f| f.contains("description: Authentication service"))
    );
    assert!(req.map_facts.iter().any(|f| f.contains("tags: core")));
    assert!(req.map_facts.iter().any(|f| f.contains("paths: src/auth")));
    assert!(req.map_facts.iter().any(|f| f.contains("parent: app")));
    assert!(
        req.map_facts
            .iter()
            .any(|f| f.contains("children: app.auth.login"))
    );
    assert!(
        req.map_facts
            .iter()
            .any(|f| f.contains("outbound: app.db (persists sessions)"))
    );
    assert!(
        req.map_facts
            .iter()
            .any(|f| f.contains("inbound: app.api (delegates login)"))
    );
}

#[test]
fn test_build_request_uses_contract_body_from_set() {
    let mut graph = empty_graph();
    let mut node = sample_node("app.auth");
    node.contracts = vec!["contracts/auth.md".to_owned()];
    graph.nodes.insert("app.auth".to_owned(), node);

    let mut contracts = ContractSet::default();
    contracts.contracts.insert(
        "contracts/auth.md".to_owned(),
        crate::artefacts::contract::Contract {
            path: "contracts/auth.md".to_owned(),
            declared_by: "app.auth".to_owned(),
            node: "app.auth".to_owned(),
            body: "# Auth\n\nHandles login.".to_owned(),
            interface: Vec::new(),
        },
    );

    let config = Config::default();
    let req = build_request(
        "app.auth",
        "contract",
        "req-1",
        &graph,
        &config,
        Path::new("."),
        &contracts,
        10_000,
        1_000,
    )
    .unwrap();

    assert_eq!(
        req.contract_excerpt,
        Some("# Auth\n\nHandles login.".to_owned())
    );
}

#[test]
fn test_build_request_reads_contract_fallback_when_not_in_set() {
    let dir = tempfile::tempdir().unwrap();
    let contract_path = dir.path().join("contracts/auth.md");
    std::fs::create_dir_all(contract_path.parent().unwrap()).unwrap();
    std::fs::write(
        &contract_path,
        "---\nnode: app.auth\n---\n# Auth\n\nFallback.",
    )
    .unwrap();

    let mut graph = empty_graph();
    let mut node = sample_node("app.auth");
    node.contracts = vec!["contracts/auth.md".to_owned()];
    graph.nodes.insert("app.auth".to_owned(), node);

    let config = Config::default();
    let req = build_request(
        "app.auth",
        "contract",
        "req-1",
        &graph,
        &config,
        dir.path(),
        &ContractSet::default(),
        10_000,
        1_000,
    )
    .unwrap();

    assert_eq!(req.contract_excerpt, Some("# Auth\n\nFallback.".to_owned()));
}

#[test]
fn test_build_request_no_contract_returns_none() {
    let mut graph = empty_graph();
    let mut node = sample_node("app.auth");
    node.contracts = vec![];
    graph.nodes.insert("app.auth".to_owned(), node);

    let config = Config::default();
    let req = build_request(
        "app.auth",
        "contract",
        "req-1",
        &graph,
        &config,
        Path::new("."),
        &ContractSet::default(),
        10_000,
        1_000,
    )
    .unwrap();

    assert!(req.contract_excerpt.is_none());
}

#[test]
fn test_build_request_filters_interface_findings() {
    let mut graph = empty_graph();
    graph
        .nodes
        .insert("app.auth".to_owned(), sample_node("app.auth"));
    graph.findings.push(Finding {
        code: "CT001".to_owned(),
        severity: FindingSeverity::Error,
        message: "Interface contradiction".to_owned(),
        node: Some("app.auth".to_owned()),
        target: None,
        path: None,
    });
    graph.findings.push(Finding {
        code: "CT002".to_owned(),
        severity: FindingSeverity::Warning,
        message: "Other node issue".to_owned(),
        node: Some("app.db".to_owned()),
        target: None,
        path: None,
    });

    let config = Config::default();
    let req = build_request(
        "app.auth",
        "contract",
        "req-1",
        &graph,
        &config,
        Path::new("."),
        &ContractSet::default(),
        10_000,
        1_000,
    )
    .unwrap();

    assert_eq!(req.interface_findings.len(), 1);
    assert!(req.interface_findings[0].contains("CT001"));
    assert!(req.interface_findings[0].contains("Interface contradiction"));
}

#[test]
fn test_build_request_populates_context_and_rules() {
    let mut graph = empty_graph();
    graph
        .nodes
        .insert("app.auth".to_owned(), sample_node("app.auth"));

    let mut config = Config {
        context: "Auth context".to_owned(),
        ..Config::default()
    };
    config
        .rules
        .insert("style".to_owned(), "use present tense".to_owned());
    config
        .rules
        .insert("tone".to_owned(), "be concise".to_owned());
    let req = build_request(
        "app.auth",
        "contract",
        "req-1",
        &graph,
        &config,
        Path::new("."),
        &ContractSet::default(),
        10_000,
        1_000,
    )
    .unwrap();

    assert_eq!(req.project_context, "Auth context");
    assert_eq!(req.rules.len(), 2);
    assert!(req.rules.contains(&"use present tense".to_owned()));
    assert!(req.rules.contains(&"be concise".to_owned()));
}

#[test]
fn test_build_request_includes_code_samples() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("src")).unwrap();
    std::fs::write(dir.path().join("src/auth.rs"), "fn login() {}").unwrap();

    let mut graph = empty_graph();
    let mut node = sample_node("app.auth");
    node.files = vec!["src/auth.rs".to_owned()];
    graph.nodes.insert("app.auth".to_owned(), node);

    let config = Config::default();
    let req = build_request(
        "app.auth",
        "contract",
        "req-1",
        &graph,
        &config,
        dir.path(),
        &ContractSet::default(),
        10_000,
        1_000,
    )
    .unwrap();

    assert_eq!(req.code_samples.len(), 1);
    assert_eq!(req.code_samples[0].path, "src/auth.rs");
    assert_eq!(req.code_samples[0].content, "fn login() {}");
}

#[test]
fn test_build_request_skips_missing_code_files() {
    let mut graph = empty_graph();
    let mut node = sample_node("app.auth");
    node.files = vec!["src/missing.rs".to_owned()];
    graph.nodes.insert("app.auth".to_owned(), node);

    let config = Config::default();
    let req = build_request(
        "app.auth",
        "contract",
        "req-1",
        &graph,
        &config,
        Path::new("."),
        &ContractSet::default(),
        10_000,
        1_000,
    )
    .unwrap();

    assert!(req.code_samples.is_empty());
}

#[test]
fn test_build_request_truncates_code_samples_per_file_limit() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("src")).unwrap();
    let long_content = "a".repeat(500);
    std::fs::write(dir.path().join("src/auth.rs"), &long_content).unwrap();

    let mut graph = empty_graph();
    let mut node = sample_node("app.auth");
    node.files = vec!["src/auth.rs".to_owned()];
    graph.nodes.insert("app.auth".to_owned(), node);

    let config = Config::default();
    let req = build_request(
        "app.auth",
        "contract",
        "req-1",
        &graph,
        &config,
        dir.path(),
        &ContractSet::default(),
        10_000,
        100,
    )
    .unwrap();

    assert_eq!(req.code_samples.len(), 1);
    assert_eq!(req.code_samples[0].content.len(), 100);
}

#[test]
fn test_build_request_drops_samples_when_prompt_too_large() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("src")).unwrap();
    std::fs::write(dir.path().join("src/a.rs"), "x").unwrap();
    std::fs::write(dir.path().join("src/b.rs"), "y").unwrap();

    let mut graph = empty_graph();
    let mut node = sample_node("app.auth");
    node.files = vec!["src/a.rs".to_owned(), "src/b.rs".to_owned()];
    graph.nodes.insert("app.auth".to_owned(), node);

    let config = Config::default();
    let req = build_request(
        "app.auth",
        "contract",
        "req-1",
        &graph,
        &config,
        dir.path(),
        &ContractSet::default(),
        200,
        1_000,
    )
    .unwrap();

    // With only 200 bytes total, both samples should have been dropped.
    assert!(req.code_samples.is_empty());
}

#[test]
fn test_build_request_truncates_contract_when_prompt_too_large() {
    let mut graph = empty_graph();
    let mut node = sample_node("app.auth");
    node.contracts = vec!["contracts/auth.md".to_owned()];
    graph.nodes.insert("app.auth".to_owned(), node);

    let mut contracts = ContractSet::default();
    contracts.contracts.insert(
        "contracts/auth.md".to_owned(),
        crate::artefacts::contract::Contract {
            path: "contracts/auth.md".to_owned(),
            declared_by: "app.auth".to_owned(),
            node: "app.auth".to_owned(),
            body: "a".repeat(1_000),
            interface: Vec::new(),
        },
    );

    let config = Config::default();
    let req = build_request(
        "app.auth",
        "contract",
        "req-1",
        &graph,
        &config,
        Path::new("."),
        &contracts,
        300,
        1_000,
    )
    .unwrap();

    // The contract excerpt should have been truncated to fit.
    assert!(
        req.contract_excerpt.as_ref().unwrap().len() < 1_000,
        "expected truncated excerpt, got {} chars",
        req.contract_excerpt.as_ref().unwrap().len()
    );
}

#[test]
fn test_build_request_omits_empty_description_from_map_facts() {
    let mut graph = empty_graph();
    let mut node = sample_node("app.auth");
    node.description = String::new();
    graph.nodes.insert("app.auth".to_owned(), node);

    let config = Config::default();
    let req = build_request(
        "app.auth",
        "contract",
        "req-1",
        &graph,
        &config,
        Path::new("."),
        &ContractSet::default(),
        10_000,
        1_000,
    )
    .unwrap();

    assert!(!req.map_facts.iter().any(|f| f.starts_with("description:")));
}

#[test]
fn test_build_request_omits_empty_tags_from_map_facts() {
    let mut graph = empty_graph();
    let mut node = sample_node("app.auth");
    node.tags = vec![];
    graph.nodes.insert("app.auth".to_owned(), node);

    let config = Config::default();
    let req = build_request(
        "app.auth",
        "contract",
        "req-1",
        &graph,
        &config,
        Path::new("."),
        &ContractSet::default(),
        10_000,
        1_000,
    )
    .unwrap();

    assert!(!req.map_facts.iter().any(|f| f.starts_with("tags:")));
}

#[test]
fn test_build_request_request_id_and_target_preserved() {
    let mut graph = empty_graph();
    graph
        .nodes
        .insert("app.auth".to_owned(), sample_node("app.auth"));

    let config = Config::default();
    let req = build_request(
        "app.auth",
        "contract",
        "custom-req-42",
        &graph,
        &config,
        Path::new("."),
        &ContractSet::default(),
        10_000,
        1_000,
    )
    .unwrap();

    assert_eq!(req.request_id, "custom-req-42");
    assert_eq!(req.target_node, "app.auth");
    assert_eq!(req.draft_type, "contract");
}
