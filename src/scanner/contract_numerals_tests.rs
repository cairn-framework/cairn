//! Focused tests for the contract asserted-numeral drift check
//! (`CAIRN_CONTRACT_NUMERAL_DRIFT`), split from `contract_numerals.rs`
//! to respect the module-size guideline.

use std::collections::BTreeMap;

use super::contract_numerals::check_contract_numeral_drift;
use crate::{
    artefacts::contract::{Contract, ContractSet},
    blueprint::{Ast, Node, NodeKind, Span},
    map::{build_graph, graph::FindingSeverity},
};

fn leaf(id: &str) -> Node {
    Node {
        kind: NodeKind::Module,
        id: id.to_owned(),
        name: id.to_owned(),
        description: String::new(),
        tags: Vec::new(),
        paths: Vec::new(),
        owns_files: false,
        contracts: Vec::new(),
        raw_fields: Vec::new(),
        children: Vec::new(),
        span: Span::point("test.blueprint", 1, 1),
    }
}

fn contract_set(node: &str, body: &str) -> ContractSet {
    let path = format!("meta/contracts/{node}.md");
    ContractSet {
        contracts: BTreeMap::from([(
            path.clone(),
            Contract {
                path,
                declared_by: node.to_owned(),
                node: node.to_owned(),
                body: body.to_owned(),
                interface: Vec::new(),
            },
        )]),
        findings: Vec::new(),
    }
}

fn scan(root: &std::path::Path, body: &str) -> crate::map::graph::Graph {
    let ast = Ast {
        nodes: vec![leaf("app.api")],
        edges: Vec::new(),
    };
    let mut claimed = BTreeMap::from([("app.api".to_owned(), vec!["lib.rs".to_owned()])]);
    let contracts = contract_set("app.api", body);
    let mut graph = build_graph(
        &ast,
        root,
        &ContractSet::default(),
        &mut claimed,
        Vec::new(),
    );
    check_contract_numeral_drift(&mut graph, &contracts, root);
    graph
}

fn drift_targets(graph: &crate::map::graph::Graph) -> Vec<&str> {
    graph
        .findings
        .iter()
        .filter(|f| f.code == "CAIRN_CONTRACT_NUMERAL_DRIFT")
        .map(|f| f.target.as_deref().unwrap_or_default())
        .collect()
}

#[test]
fn stale_assertion_emits_warning_finding() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("lib.rs"),
        "pub const WIRE_VERSION: u32 = 2;\n",
    )
    .unwrap();
    let graph = scan(dir.path(), "The wire schema is `WIRE_VERSION = 1`.");
    let finding = graph
        .findings
        .iter()
        .find(|f| f.code == "CAIRN_CONTRACT_NUMERAL_DRIFT")
        .expect("stale assertion must be flagged");
    assert_eq!(finding.severity, FindingSeverity::Warning);
    assert_eq!(finding.node.as_deref(), Some("app.api"));
    assert_eq!(finding.target.as_deref(), Some("WIRE_VERSION"));
    assert_eq!(finding.path.as_deref(), Some("meta/contracts/app.api.md"));
    assert!(
        finding.message.contains('2'),
        "message must name the declared value: {}",
        finding.message
    );
}

#[test]
fn current_assertion_emits_nothing() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("lib.rs"),
        "pub(crate) const WIRE_VERSION: u32 = 1_000;\n",
    )
    .unwrap();
    let graph = scan(dir.path(), "The wire schema is `WIRE_VERSION = 1000`.");
    assert!(
        drift_targets(&graph).is_empty(),
        "a source-backed assertion must not be flagged: {:?}",
        graph.findings
    );
}

#[test]
fn vanished_constant_emits_finding() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("lib.rs"), "pub fn run() {}\n").unwrap();
    let graph = scan(dir.path(), "The wire schema is `OLD_VERSION = 3`.");
    let finding = graph
        .findings
        .iter()
        .find(|f| f.code == "CAIRN_CONTRACT_NUMERAL_DRIFT")
        .expect("an assertion with no source constant is stale");
    assert_eq!(finding.target.as_deref(), Some("OLD_VERSION"));
    assert!(
        finding.message.contains("no `const OLD_VERSION`"),
        "missing-constant copy must render: {}",
        finding.message
    );
}

#[test]
fn version_semantics_prose_is_never_flagged() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("lib.rs"), "pub fn run() {}\n").unwrap();
    let body = "State migrates v1-to-v2 until a schema-v2 snapshot exists.\n\
        Protocol version reported is `2024-11-05`.\n\
        `TRACE_VERSION: u32` is the wire schema version.\n\
        ```\n`IGNORED_IN_FENCE = 9`\n```\n\
        ~~~\n`ALSO_IGNORED = 8`\n~~~\n";
    let graph = scan(dir.path(), body);
    assert!(
        drift_targets(&graph).is_empty(),
        "stable version semantics must not be flagged: {:?}",
        graph.findings
    );
}

#[test]
fn unclosed_backtick_tail_is_not_a_span() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("lib.rs"), "pub fn run() {}\n").unwrap();
    let graph = scan(dir.path(), "A stray backtick `OLD_VERSION = 1");
    assert!(
        drift_targets(&graph).is_empty(),
        "an unclosed backtick opens no span: {:?}",
        graph.findings
    );
}

#[test]
fn const_generic_parameter_is_not_a_source() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("lib.rs"),
        "pub fn make<const WIRE_VERSION: usize>() { let x = 2; }\n",
    )
    .unwrap();
    let graph = scan(dir.path(), "The wire schema is `WIRE_VERSION = 2`.");
    let finding = graph
        .findings
        .iter()
        .find(|f| f.code == "CAIRN_CONTRACT_NUMERAL_DRIFT")
        .expect("a const-generic parameter is not a deterministic source");
    assert!(
        finding.message.contains("no `const WIRE_VERSION`"),
        "generic parameter must report as missing: {}",
        finding.message
    );
}

#[test]
fn non_decimal_initialiser_is_not_a_source() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("lib.rs"),
        "pub const FLAG_MASK: u32 = 0x10;\npub const SUM_LIMIT: u32 = 1 + 2;\n",
    )
    .unwrap();
    let graph = scan(
        dir.path(),
        "Bits `FLAG_MASK = 16` and total `SUM_LIMIT = 3`.",
    );
    let targets = drift_targets(&graph);
    assert_eq!(
        targets,
        vec!["FLAG_MASK", "SUM_LIMIT"],
        "hex and expression initialisers are skipped, never mis-parsed"
    );
    assert!(
        graph
            .findings
            .iter()
            .all(|f| f.code != "CAIRN_CONTRACT_NUMERAL_DRIFT" || f.message.contains("no `const")),
        "skipped initialisers must report as missing, not as a wrong value: {:?}",
        graph.findings
    );
}

#[test]
fn suffixed_literal_is_a_source() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("lib.rs"),
        "pub const WIRE_VERSION: u32 = 7u32;\n",
    )
    .unwrap();
    let graph = scan(dir.path(), "The wire schema is `WIRE_VERSION = 7`.");
    assert!(
        drift_targets(&graph).is_empty(),
        "a type-suffixed decimal literal is a valid source: {:?}",
        graph.findings
    );
}

#[test]
fn duplicate_assertions_emit_one_finding() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("lib.rs"), "pub fn run() {}\n").unwrap();
    let graph = scan(
        dir.path(),
        "`OLD_VERSION = 1` here and `OLD_VERSION = 1` again.",
    );
    assert_eq!(
        drift_targets(&graph),
        vec!["OLD_VERSION"],
        "one name is flagged once per contract"
    );
}

#[test]
fn pseudo_const_in_string_or_comment_is_not_a_source() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("lib.rs"),
        "// const WIRE_VERSION: u32 = 1;\n\
         pub fn fixture() -> &'static str {\n\
             \"pub const WIRE_VERSION: u32 = 1;\"\n\
         }\n",
    )
    .unwrap();
    let graph = scan(dir.path(), "The wire schema is `WIRE_VERSION = 1`.");
    let finding = graph
        .findings
        .iter()
        .find(|f| f.code == "CAIRN_CONTRACT_NUMERAL_DRIFT")
        .expect("a pseudo-const in a string or comment must not satisfy the assertion");
    assert!(
        finding.message.contains("no `const WIRE_VERSION`"),
        "pseudo-const must report as missing: {}",
        finding.message
    );
}

#[test]
fn mismatched_fence_delimiters_do_not_close_a_fence() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("lib.rs"), "pub fn run() {}\n").unwrap();
    let body = "```\n\
        ~~~\n\
        `IN_BACKTICK_FENCE = 9`\n\
        ```\n\
        ````\n\
        ```\n\
        `IN_LONG_FENCE = 9`\n\
        ````\n";
    let graph = scan(dir.path(), body);
    assert!(
        drift_targets(&graph).is_empty(),
        "content inside a fence must never assert: {:?}",
        graph.findings
    );
}
