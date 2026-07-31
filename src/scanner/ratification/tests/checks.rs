//! Check-level fixtures driving `check_ratification` end to end.

// Reason: the parent test module owns the shared builders and imports.
#![allow(clippy::wildcard_imports)]
use super::*;

#[test]
fn test_check_ratification_local_span_two_containers_errors() {
    let mut graph = graph_with_containers();
    let mut item = decision();
    item.nodes = vec!["one.child".to_owned(), "two.child".to_owned()];
    check_ratification(
        &mut graph,
        &ArtefactSet {
            decisions: vec![item],
            ..Default::default()
        },
        &temp_root("span"),
    );
    assert_eq!(findings(&graph, "CAIRN_DECISION_TIER_SPAN"), 1);
}

#[test]
fn test_check_ratification_local_span_parent_link_without_prefix_errors() {
    let mut graph = graph_with_containers();
    let mut detached = graph.nodes.get("one.child").expect("source node").clone();
    detached.id = "detached".to_owned();
    detached.parent = Some("one".to_owned());
    graph.nodes.insert("detached".to_owned(), detached);
    let mut item = decision();
    item.nodes = vec!["detached".to_owned(), "two.child".to_owned()];
    check_ratification(
        &mut graph,
        &ArtefactSet {
            decisions: vec![item],
            ..Default::default()
        },
        &temp_root("structural-span"),
    );
    assert_eq!(findings(&graph, "CAIRN_DECISION_TIER_SPAN"), 1);
}

#[test]
fn test_check_ratification_local_supersedes_errors() {
    let mut graph = empty_graph();
    let mut item = decision();
    item.supersedes.push("dec.old".to_owned());
    check_ratification(
        &mut graph,
        &ArtefactSet {
            decisions: vec![item],
            ..Default::default()
        },
        &temp_root("supersedes"),
    );
    assert_eq!(findings(&graph, "CAIRN_DECISION_TIER_SUPERSEDES"), 1);
}

#[test]
fn test_check_ratification_local_binding_path_errors() {
    let root = allowlist_root("binding-path");
    let mut item = decision();
    item.affects = vec!["docs/registries/".to_owned()];
    let mut graph = empty_graph();
    check_ratification(
        &mut graph,
        &ArtefactSet {
            decisions: vec![item],
            ..Default::default()
        },
        &root,
    );
    assert_eq!(findings(&graph, "CAIRN_DECISION_TIER_BINDING_PATH"), 1);
    fs::remove_dir_all(root).expect("cleanup");
}

#[cfg(unix)]
#[test]
fn test_check_ratification_symlink_laundering_hits_allowlist() {
    use std::os::unix::fs::symlink;

    let root = allowlist_root("symlink-laundering");
    fs::create_dir_all(root.join("src/artefacts/registry")).expect("registry directory");
    fs::write(root.join("src/artefacts/registry/file.rs"), "").expect("registry file");
    symlink("src/artefacts/registry", root.join("laundered")).expect("laundering symlink");
    let mut item = decision();
    item.affects = vec!["laundered/".to_owned()];
    let mut graph = empty_graph();
    check_ratification(
        &mut graph,
        &ArtefactSet {
            decisions: vec![item],
            ..Default::default()
        },
        &root,
    );
    assert_eq!(findings(&graph, "CAIRN_DECISION_TIER_BINDING_PATH"), 1);
    fs::remove_dir_all(root).expect("cleanup");
}

#[cfg(unix)]
#[test]
fn test_check_ratification_allowlisted_symlink_dir_hits_canonical_affects() {
    use std::os::unix::fs::symlink;

    let root = allowlist_root("allowlist-symlink");
    fs::create_dir_all(root.join("src/target")).expect("target directory");
    symlink("src/target", root.join("allowlisted")).expect("allowlisted symlink");
    fs::write(
        root.join("docs/registries/binding-surface.md"),
        "- allowlisted/\n",
    )
    .expect("allowlist");
    let mut item = decision();
    item.affects = vec!["src/target/".to_owned()];
    let mut graph = empty_graph();
    check_ratification(
        &mut graph,
        &ArtefactSet {
            decisions: vec![item],
            ..Default::default()
        },
        &root,
    );
    assert_eq!(findings(&graph, "CAIRN_DECISION_TIER_BINDING_PATH"), 1);
    fs::remove_dir_all(root).expect("cleanup");
}

#[cfg(unix)]
#[test]
fn test_check_ratification_symlink_escape_is_affects_invalid() {
    use std::os::unix::fs::symlink;

    let root = allowlist_root("symlink-escape");
    let outside = tempfile::tempdir().expect("outside root");
    symlink(outside.path(), root.join("escape")).expect("escaping symlink");
    let mut item = decision();
    item.affects = vec!["escape/".to_owned()];
    let mut graph = empty_graph();
    check_ratification(
        &mut graph,
        &ArtefactSet {
            decisions: vec![item],
            ..Default::default()
        },
        &root,
    );
    assert_eq!(findings(&graph, "CAIRN_DECISION_AFFECTS_INVALID"), 1);
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn test_check_ratification_machine_on_binding_errors() {
    let mut item = decision();
    item.ratification = RatificationTier::Binding;
    item.ratified_by_machine = true;
    let mut graph = empty_graph();
    check_ratification(
        &mut graph,
        &ArtefactSet {
            decisions: vec![item],
            ..Default::default()
        },
        &temp_root("machine"),
    );
    assert_eq!(findings(&graph, "CAIRN_DECISION_MACHINE_BINDING"), 1);
}

#[test]
fn test_check_ratification_accepted_local_happy_fixture_no_findings() {
    let root = allowlist_root("happy");
    fs::create_dir_all(root.join("meta/decisions")).expect("decision directory");
    fs::create_dir_all(root.join("meta/reviews")).expect("review directory");
    fs::create_dir_all(root.join("src")).expect("source directory");
    fs::write(
        root.join("src/governed.rs"),
        "pub const RULE: bool = true;\n",
    )
    .expect("governed file");
    let raw = "---\nid: dec.local\nstatus: accepted\ndate: 2026-01-01\nratification: local\naffects:\n  - meta/decisions/dec.local.md\n  - src/governed.rs\n  - meta/reviews/rev.one.md\n  - meta/reviews/rev.two.md\nreceipts:\n  - rev.one\n  - rev.two\nnodes:\n  - app.module\n---\nRule body\n";
    let affects = vec![
        "meta/decisions/dec.local.md".to_owned(),
        "src/governed.rs".to_owned(),
        "meta/reviews/rev.one.md".to_owned(),
        "meta/reviews/rev.two.md".to_owned(),
    ];
    fs::write(root.join("meta/decisions/dec.local.md"), raw).expect("decision file");
    fs::write(root.join("meta/reviews/rev.one.md"), "## Verdict\nPASS\n").expect("first receipt");
    fs::write(root.join("meta/reviews/rev.two.md"), "## Verdict\nPASS\n").expect("second receipt");
    let hash = compute_subject_hash(&root, "meta/decisions/dec.local.md", raw, &affects)
        .expect("subject hash");
    let mut item = decision();
    item.path = root
        .join("meta/decisions/dec.local.md")
        .to_string_lossy()
        .into_owned();
    item.affects = affects;
    let first = receipt(&root, "rev.one", "model-a/correctness", &hash);
    let second = receipt(&root, "rev.two", "model-b/security", &hash);
    let mut graph = empty_graph();
    check_ratification(
        &mut graph,
        &ArtefactSet {
            decisions: vec![item],
            reviews: vec![first, second],
            ..Default::default()
        },
        &root,
    );
    assert!(graph.findings.is_empty(), "{:?}", graph.findings);
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn test_check_ratification_accepted_machine_local_debate_fixture_no_findings() {
    let root = allowlist_root("machine-happy");
    fs::create_dir_all(root.join("meta/decisions")).expect("decision directory");
    fs::create_dir_all(root.join("meta/reviews")).expect("review directory");
    fs::create_dir_all(root.join("src")).expect("source directory");
    fs::write(
        root.join("src/governed.rs"),
        "pub const RULE: bool = true;\n",
    )
    .expect("governed file");
    let raw = "---\nid: dec.local\nstatus: accepted\ndate: 2026-01-01\nratification: local\nratified_by: machine\naffects:\n  - meta/decisions/dec.local.md\n  - src/governed.rs\n  - meta/reviews/rev.one.md\n  - meta/reviews/rev.two.md\nreceipts:\n  - rev.one\n  - rev.two\nnodes:\n  - app.module\n---\n## For\nBenefits exceed costs.\n## Against\nRisks remain bounded.\n## Verdict\nProceed.\n";
    let affects = vec![
        "meta/decisions/dec.local.md".to_owned(),
        "src/governed.rs".to_owned(),
        "meta/reviews/rev.one.md".to_owned(),
        "meta/reviews/rev.two.md".to_owned(),
    ];
    fs::write(root.join("meta/decisions/dec.local.md"), raw).expect("decision file");
    fs::write(root.join("meta/reviews/rev.one.md"), "## Verdict\nPASS\n").expect("first receipt");
    fs::write(root.join("meta/reviews/rev.two.md"), "## Verdict\nPASS\n").expect("second receipt");
    let hash = compute_subject_hash(&root, "meta/decisions/dec.local.md", raw, &affects)
        .expect("subject hash");
    let mut item = decision();
    item.path = root
        .join("meta/decisions/dec.local.md")
        .to_string_lossy()
        .into_owned();
    item.affects = affects;
    item.ratified_by_machine = true;
    item.body =
        "## For\nBenefits exceed costs.\n## Against\nRisks remain bounded.\n## Verdict\nProceed.\n"
            .to_owned();
    let first = receipt(&root, "rev.one", "model-a/correctness", &hash);
    let second = receipt(&root, "rev.two", "model-b/security", &hash);
    let mut graph = empty_graph();
    check_ratification(
        &mut graph,
        &ArtefactSet {
            decisions: vec![item],
            reviews: vec![first, second],
            ..Default::default()
        },
        &root,
    );
    assert!(graph.findings.is_empty(), "{:?}", graph.findings);
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn test_check_ratification_unmatched_receipt_grade_review_is_info() {
    let root = temp_root("unmatched");
    let mut graph = empty_graph();
    check_ratification(
        &mut graph,
        &ArtefactSet {
            reviews: vec![receipt(
                &root,
                "rev.orphan",
                "model-a/correctness",
                &format!("sha256:{}", "b".repeat(64)),
            )],
            ..Default::default()
        },
        &root,
    );
    let finding = graph
        .findings
        .iter()
        .find(|finding| finding.code == "CAIRN_REVIEW_SUBJECT_UNMATCHED")
        .expect("unmatched finding");
    assert_eq!(finding.severity, crate::map::graph::FindingSeverity::Info);
}
