//! Tests for todo relationship validation.

use super::*;
use crate::map::FindingSeverity;

fn relation_todo(slug: &str, status: TodoStatus) -> Todo {
    Todo {
        path: format!("meta/todos/todo.{slug}.md"),
        node: "app".to_owned(),
        status,
        created: "2026-01-01".to_owned(),
        satisfies: None,
        blocked_by: Vec::new(),
        parent: None,
        related: Vec::new(),
        defers: Vec::new(),
        body: String::new(),
    }
}

fn run(todos: Vec<Todo>) -> Vec<Finding> {
    let mut set = ArtefactSet {
        todos,
        ..ArtefactSet::default()
    };
    validate_todo_relations(&mut set);
    set.findings
}

fn by_code<'a>(findings: &'a [Finding], code: &str) -> Vec<&'a Finding> {
    findings.iter().filter(|f| f.code == code).collect()
}

fn make_decision(id: &str) -> Decision {
    Decision {
        id: id.to_owned(),
        path: format!("meta/decisions/{id}.md"),
        nodes: Vec::new(),
        status: DecisionStatus::Accepted,
        ratification: RatificationTier::Binding,
        affects: Vec::new(),
        ratified_by_machine: false,
        receipts: Vec::new(),
        date: "2026-01-01".to_owned(),
        revisited: None,
        revisit_triggers: Vec::new(),
        informed_by: Vec::new(),
        supersedes: Vec::new(),
        refines: Vec::new(),
        refined_by: Vec::new(),
        superseded_by: Vec::new(),
        related: Vec::new(),
        orphaned: false,
        orphan_reason: None,
        gap: false,
        claims: None,
        body: String::new(),
    }
}

fn make_research(id: &str) -> Research {
    Research {
        id: id.to_owned(),
        path: format!("meta/research/{id}.md"),
        nodes: Vec::new(),
        date: "2026-01-01".to_owned(),
        sources: Vec::new(),
        method: ResearchMethod::Secondary,
        tags: Vec::new(),
        body: String::new(),
    }
}

fn make_source(id: &str) -> Source {
    Source {
        id: id.to_owned(),
        path: format!("meta/sources/{id}.md"),
        file: "docs/e.txt".to_owned(),
        sha256: None,
        verification: SourceVerification::Unverified,
        source_type: "note".to_owned(),
        date: "2026-01-01".to_owned(),
        tags: Vec::new(),
        description: String::new(),
        body: String::new(),
    }
}

#[test]
fn test_relations_dangling_blocker_emits_one_warning() {
    let mut a = relation_todo("a", TodoStatus::Open);
    a.blocked_by = vec!["todo.x".to_owned()];
    let findings = run(vec![a]);
    let unknown = by_code(&findings, "CAIRN_TODO_RELATION_UNKNOWN");
    assert_eq!(unknown.len(), 1, "exactly one dangling-reference warning");
    assert_eq!(unknown[0].severity, FindingSeverity::Warning);
    assert!(unknown[0].message.contains("blocked_by target `todo.x`"));
    // The dangling blocker has no status, so neither contradiction form fires.
    assert!(by_code(&findings, "CAIRN_TODO_STATUS_CONTRADICTION").is_empty());
    assert!(by_code(&findings, "CAIRN_TODO_RELATION_CYCLE").is_empty());
}

#[test]
fn test_relations_distinct_dangling_refs_carry_distinct_targets() {
    // The scanner deduplicates on (code, node, path, target); two bad
    // references in one file must survive as two findings.
    let mut a = relation_todo("a", TodoStatus::InProgress);
    a.blocked_by = vec!["todo.x".to_owned()];
    a.related = vec!["dec.y".to_owned()];
    let findings = run(vec![a]);
    let unknown = by_code(&findings, "CAIRN_TODO_RELATION_UNKNOWN");
    assert_eq!(unknown.len(), 2);
    let targets: Vec<_> = unknown.iter().map(|f| f.target.as_deref()).collect();
    assert!(targets.contains(&Some("blocked_by:todo.x")));
    assert!(targets.contains(&Some("related:dec.y")));
}

#[test]
fn test_relations_dangling_parent_emits_warning() {
    let mut a = relation_todo("a", TodoStatus::Open);
    a.parent = Some("todo.ghost".to_owned());
    let findings = run(vec![a]);
    let unknown = by_code(&findings, "CAIRN_TODO_RELATION_UNKNOWN");
    assert_eq!(unknown.len(), 1);
    assert!(unknown[0].message.contains("parent target `todo.ghost`"));
}

#[test]
fn test_relations_two_todo_blocked_by_cycle_emits_one_error() {
    let mut a = relation_todo("a", TodoStatus::Open);
    a.blocked_by = vec!["todo.b".to_owned()];
    let mut b = relation_todo("b", TodoStatus::Open);
    b.blocked_by = vec!["todo.a".to_owned()];
    let findings = run(vec![a, b]);
    let cycles = by_code(&findings, "CAIRN_TODO_RELATION_CYCLE");
    assert_eq!(cycles.len(), 1, "one finding per cyclic component");
    assert_eq!(cycles[0].severity, FindingSeverity::Error);
    assert!(
        cycles[0]
            .message
            .contains("`blocked_by` cycle involving: todo.a, todo.b"),
        "unexpected message: {}",
        cycles[0].message
    );
}

#[test]
fn test_relations_overlapping_cycles_report_one_component() {
    // a <-> b and a <-> c share `a`: one knot, one finding naming every
    // member, so no cycle is ever silently missing.
    let mut a = relation_todo("a", TodoStatus::Open);
    a.blocked_by = vec!["todo.b".to_owned(), "todo.c".to_owned()];
    let mut b = relation_todo("b", TodoStatus::Open);
    b.blocked_by = vec!["todo.a".to_owned()];
    let mut c = relation_todo("c", TodoStatus::Open);
    c.blocked_by = vec!["todo.a".to_owned()];
    let findings = run(vec![a, b, c]);
    let cycles = by_code(&findings, "CAIRN_TODO_RELATION_CYCLE");
    assert_eq!(cycles.len(), 1, "{cycles:?}");
    assert!(
        cycles[0]
            .message
            .contains("involving: todo.a, todo.b, todo.c"),
        "unexpected message: {}",
        cycles[0].message
    );
}

#[test]
fn test_relations_disjoint_cycles_survive_dedup_with_distinct_targets() {
    let mut a = relation_todo("a", TodoStatus::Open);
    a.blocked_by = vec!["todo.b".to_owned()];
    let mut b = relation_todo("b", TodoStatus::Open);
    b.blocked_by = vec!["todo.a".to_owned()];
    let mut c = relation_todo("c", TodoStatus::Open);
    c.blocked_by = vec!["todo.d".to_owned()];
    let mut d = relation_todo("d", TodoStatus::Open);
    d.blocked_by = vec!["todo.c".to_owned()];
    let findings = run(vec![a, b, c, d]);
    let cycles = by_code(&findings, "CAIRN_TODO_RELATION_CYCLE");
    assert_eq!(cycles.len(), 2);
    assert_eq!(
        cycles[0].target.as_deref(),
        Some("blocked_by:todo.a, todo.b")
    );
    assert_eq!(
        cycles[1].target.as_deref(),
        Some("blocked_by:todo.c, todo.d")
    );
}

#[test]
fn test_relations_self_loop_blocked_by_emits_cycle() {
    let mut a = relation_todo("a", TodoStatus::Open);
    a.blocked_by = vec!["todo.a".to_owned()];
    let findings = run(vec![a]);
    assert_eq!(by_code(&findings, "CAIRN_TODO_RELATION_CYCLE").len(), 1);
}

#[test]
fn test_relations_parent_cycle_emits_one_error() {
    let mut a = relation_todo("a", TodoStatus::Open);
    a.parent = Some("todo.b".to_owned());
    let mut b = relation_todo("b", TodoStatus::Open);
    b.parent = Some("todo.a".to_owned());
    let findings = run(vec![a, b]);
    let cycles = by_code(&findings, "CAIRN_TODO_RELATION_CYCLE");
    assert_eq!(cycles.len(), 1);
    assert!(cycles[0].message.contains("`parent` cycle"));
}

#[test]
fn test_relations_mixed_parent_blocked_by_chain_stays_silent() {
    // A child blocking the epic that contains it is legal: cycles are
    // detected per graph, never across their union
    // (`dec.todo-relationship-model` ruling 4).
    let mut child = relation_todo("child", TodoStatus::Open);
    child.parent = Some("todo.epic".to_owned());
    let mut epic = relation_todo("epic", TodoStatus::Open);
    epic.blocked_by = vec!["todo.child".to_owned()];
    let findings = run(vec![child, epic]);
    assert!(by_code(&findings, "CAIRN_TODO_RELATION_CYCLE").is_empty());
    assert!(by_code(&findings, "CAIRN_TODO_RELATION_UNKNOWN").is_empty());
}

#[test]
fn test_relations_blocked_without_blockers_stays_silent() {
    // Blockers outside the todo graph are legal and undeclarable; a blocked
    // todo with no `blocked_by:` is out of the advisory's scope.
    let findings = run(vec![relation_todo("a", TodoStatus::Blocked)]);
    assert!(findings.is_empty(), "unexpected findings: {findings:?}");
}

#[test]
fn test_relations_blocked_with_all_done_blockers_emits_info() {
    let mut a = relation_todo("a", TodoStatus::Blocked);
    a.blocked_by = vec!["todo.b".to_owned(), "todo.c".to_owned()];
    let b = relation_todo("b", TodoStatus::Done);
    let c = relation_todo("c", TodoStatus::Done);
    let findings = run(vec![a, b, c]);
    let contradictions = by_code(&findings, "CAIRN_TODO_STATUS_CONTRADICTION");
    assert_eq!(contradictions.len(), 1);
    assert_eq!(contradictions[0].severity, FindingSeverity::Info);
    assert!(
        contradictions[0]
            .message
            .contains("blocked while every declared blocker is done")
    );
}

#[test]
fn test_relations_blocked_with_unresolved_blocker_stays_silent() {
    let mut a = relation_todo("a", TodoStatus::Blocked);
    a.blocked_by = vec!["todo.b".to_owned()];
    let b = relation_todo("b", TodoStatus::Open);
    let findings = run(vec![a, b]);
    assert!(findings.is_empty(), "unexpected findings: {findings:?}");
}

#[test]
fn test_relations_open_with_unresolved_blocker_emits_info() {
    let mut a = relation_todo("a", TodoStatus::Open);
    a.blocked_by = vec!["todo.b".to_owned()];
    let b = relation_todo("b", TodoStatus::InProgress);
    let findings = run(vec![a, b]);
    let contradictions = by_code(&findings, "CAIRN_TODO_STATUS_CONTRADICTION");
    assert_eq!(contradictions.len(), 1);
    assert!(
        contradictions[0]
            .message
            .contains("open while declared blocker `todo.b` is not done")
    );
}

#[test]
fn test_relations_in_progress_with_unresolved_blocker_stays_silent() {
    // An in-progress todo may legitimately carry an unresolved dependency
    // mid-flight; only `open` is flagged on the downstream side.
    let mut a = relation_todo("a", TodoStatus::InProgress);
    a.blocked_by = vec!["todo.b".to_owned()];
    let b = relation_todo("b", TodoStatus::Open);
    let findings = run(vec![a, b]);
    assert!(findings.is_empty(), "unexpected findings: {findings:?}");
}

#[test]
fn test_relations_related_resolves_across_artefact_kinds() {
    let mut a = relation_todo("a", TodoStatus::Open);
    a.related = vec![
        "todo.b".to_owned(),
        "dec.rule".to_owned(),
        "res.study".to_owned(),
        "src.evidence".to_owned(),
    ];
    let b = relation_todo("b", TodoStatus::Open);
    let mut set = ArtefactSet {
        todos: vec![a, b],
        decisions: vec![make_decision("dec.rule")],
        research: vec![make_research("res.study")],
        sources: vec![make_source("src.evidence")],
        ..ArtefactSet::default()
    };
    validate_todo_relations(&mut set);
    assert!(
        set.findings.is_empty(),
        "unexpected findings: {:?}",
        set.findings
    );
}

#[test]
fn test_relations_related_unknown_prefix_emits_warning() {
    let mut a = relation_todo("a", TodoStatus::Open);
    a.related = vec!["rev.someone".to_owned()];
    let findings = run(vec![a]);
    let unknown = by_code(&findings, "CAIRN_TODO_RELATION_UNKNOWN");
    assert_eq!(unknown.len(), 1);
    assert!(unknown[0].message.contains("related target `rev.someone`"));
}
