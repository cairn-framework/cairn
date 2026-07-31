//! Tests for query wire serialisation: status roundtrips, enriched
//! artefact wires, and path/title helpers.
use super::*;
use crate::artefacts::registry::ResearchMethod;

#[test]
fn test_todo_status_roundtrip() {
    for (status, name) in [
        (TodoStatus::Open, "open"),
        (TodoStatus::InProgress, "in_progress"),
        (TodoStatus::Done, "done"),
        (TodoStatus::Blocked, "blocked"),
    ] {
        assert_eq!(todo_status(status), name);
        assert_eq!(parse_todo_status_filter(name), Some(status));
    }
}

#[test]
fn test_decision_status_roundtrip() {
    for (status, name) in [
        (DecisionStatus::Proposed, "proposed"),
        (DecisionStatus::Accepted, "accepted"),
        (DecisionStatus::Deprecated, "deprecated"),
        (DecisionStatus::Superseded, "superseded"),
    ] {
        assert_eq!(decision_status(status), name);
        assert_eq!(parse_decision_status_filter(name), Some(status));
    }
}

#[test]
fn test_parse_todo_status_filter_unknown_returns_none() {
    assert_eq!(parse_todo_status_filter("in-progress"), None);
    assert_eq!(parse_todo_status_filter(""), None);
    assert_eq!(parse_todo_status_filter("Open"), None); // case-sensitive
}

#[test]
fn test_parse_decision_status_filter_unknown_returns_none() {
    assert_eq!(parse_decision_status_filter("Accepted"), None);
    assert_eq!(parse_decision_status_filter(""), None);
}

#[test]
fn test_enriched_artefact_wires_include_title_and_body() {
    let root = std::path::Path::new("/project");
    let todo = Todo {
        path: "/project/meta/todos/todo.md".to_owned(),
        node: "app.api".to_owned(),
        status: TodoStatus::Open,
        created: "2026-04-01".to_owned(),
        satisfies: Some("status.contract".to_owned()),
        defers: Vec::new(),
        body: "# Ship the endpoint\n\nDetails.".to_owned(),
    };
    let value = todo_enriched_json(&todo, root);
    assert_eq!(value["path"], "meta/todos/todo.md");
    assert_eq!(value["title"], "Ship the endpoint");
    assert_eq!(value["body"], todo.body);
    assert_eq!(value["status"], "open");

    let decision = Decision {
        id: "dec.api".to_owned(),
        path: "/project/meta/decisions/api.md".to_owned(),
        nodes: vec!["app.api".to_owned()],
        status: DecisionStatus::Accepted,
        date: "2026-04-01".to_owned(),
        revisited: Some("2026-05-01".to_owned()),
        revisit_triggers: vec!["trigger-a".to_owned()],
        informed_by: vec!["res.api".to_owned()],
        supersedes: vec!["dec.old".to_owned()],
        refines: vec!["dec.parent".to_owned()],
        related: vec!["dec.cousin".to_owned()],
        orphaned: false,
        orphan_reason: None,
        gap: false,
        claims: None,
        body: "# API Decision\nUse stable JSON.".to_owned(),
        ratification: crate::artefacts::registry::RatificationTier::Local,
        affects: Vec::new(),
        ratified_by_machine: true,
        receipts: Vec::new(),
    };
    let value = decision_enriched_json(&decision, root);
    assert_eq!(value["path"], "meta/decisions/api.md");
    assert_eq!(value["title"], "API Decision");
    assert_eq!(value["body"], decision.body);
    assert_eq!(value["date"], "2026-04-01");
    assert_eq!(value["revisited"], "2026-05-01");
    assert_eq!(value["revisit_triggers"], json!(["trigger-a"]));
    assert_eq!(value["ratification"], "local");
    assert_eq!(value["ratified_by"], "machine");

    let mut maintainer_signed = decision.clone();
    maintainer_signed.ratified_by_machine = false;
    let value = decision_enriched_json(&maintainer_signed, root);
    assert_eq!(
        value["ratified_by"], "maintainer",
        "accepted without the marker is maintainer-signed"
    );

    let mut unsigned = decision.clone();
    unsigned.ratified_by_machine = false;
    unsigned.status = DecisionStatus::Proposed;
    let value = decision_enriched_json(&unsigned, root);
    assert_eq!(
        value["ratified_by"],
        json!(null),
        "a proposed decision has no signer yet"
    );

    let research = Research {
        id: "res.api".to_owned(),
        path: "/project/meta/research/api.md".to_owned(),
        nodes: vec!["app.api".to_owned()],
        date: "2026-03-20".to_owned(),
        sources: vec!["src.api".to_owned()],
        method: ResearchMethod::Primary,
        tags: vec!["wire".to_owned()],
        body: "# API Research\nStudied evolution.".to_owned(),
    };
    let value = research_enriched_json(&research, root);
    assert_eq!(value["path"], "meta/research/api.md");
    assert_eq!(value["title"], "API Research");
    assert_eq!(value["body"], research.body);

    let source = Source {
        id: "src.api".to_owned(),
        path: "/project/meta/sources/api.md".to_owned(),
        file: "docs-source.txt".to_owned(),
        sha256: None,
        verification: SourceVerification::Verified,
        source_type: "note".to_owned(),
        date: "2026-03-19".to_owned(),
        tags: vec!["wire".to_owned()],
        description: "bootstrap source".to_owned(),
        body: "# API Source\nBootstrap evidence.".to_owned(),
    };
    let value = source_enriched_json(&source, root);
    assert_eq!(value["path"], "meta/sources/api.md");
    assert_eq!(value["title"], "API Source");
    assert_eq!(value["body"], source.body);
}

#[test]
fn test_source_json_reports_tracked_verification() {
    let source = Source {
        id: "src.live".to_owned(),
        path: "meta/sources/live.md".to_owned(),
        file: "src/lib.rs".to_owned(),
        sha256: None,
        verification: SourceVerification::Tracked,
        source_type: "in-repo code read".to_owned(),
        date: "2026-07-30".to_owned(),
        tags: Vec::new(),
        description: String::new(),
        body: String::new(),
    };
    assert_eq!(source_json(&source)["verification"], "tracked");
}

#[test]
fn test_relative_path_strips_root_when_possible() {
    let root = std::path::Path::new("/project");
    assert_eq!(relative_path("/project/meta/todo.md", root), "meta/todo.md");
    assert_eq!(
        relative_path("/elsewhere/meta/todo.md", root),
        "/elsewhere/meta/todo.md"
    );
    assert_eq!(relative_path("meta/todo.md", root), "meta/todo.md");
}

#[test]
fn test_title_from_body_falls_back_on_missing_heading() {
    assert_eq!(title_from_body("No heading here", "Fallback"), "Fallback");
    assert_eq!(title_from_body("# \n\nBody", "Fallback"), "Fallback");
    assert_eq!(title_from_body("# Title\nBody", "Fallback"), "Title");
}
