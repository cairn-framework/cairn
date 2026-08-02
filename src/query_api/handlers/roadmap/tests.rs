//! Tests for the roadmap projection.

use super::*;

fn todo(slug: &str, status: TodoStatus) -> Todo {
    Todo {
        path: format!("/project/meta/todos/todo.{slug}.md"),
        node: "app".to_owned(),
        status,
        created: "2026-01-01".to_owned(),
        satisfies: None,
        blocked_by: Vec::new(),
        parent: None,
        related: Vec::new(),
        defers: Vec::new(),
        body: format!("# {slug}\n"),
    }
}

fn stems(response: &RoadmapResponse) -> Vec<(u32, Vec<&str>)> {
    response
        .tiers
        .iter()
        .map(|tier| {
            (
                tier.tier,
                tier.items.iter().map(|item| item.stem.as_str()).collect(),
            )
        })
        .collect()
}

#[test]
fn test_roadmap_tiers_follow_blocked_by_chain() {
    let a = todo("a", TodoStatus::Open);
    let mut b = todo("b", TodoStatus::Blocked);
    b.blocked_by = vec!["todo.a".to_owned()];
    let mut c = todo("c", TodoStatus::Blocked);
    c.blocked_by = vec!["todo.b".to_owned()];
    let response = roadmap_response(std::path::Path::new("/project"), &[c, a, b]);
    assert_eq!(
        stems(&response),
        vec![
            (1, vec!["todo.a"]),
            (2, vec!["todo.b"]),
            (3, vec!["todo.c"]),
        ]
    );
}

#[test]
fn test_roadmap_zero_edges_degenerates_to_one_tier() {
    let response = roadmap_response(
        std::path::Path::new("/project"),
        &[todo("a", TodoStatus::Open), todo("b", TodoStatus::Blocked)],
    );
    assert_eq!(stems(&response), vec![(1, vec!["todo.a", "todo.b"])]);
}

#[test]
fn test_roadmap_excludes_done_and_ignores_satisfied_blockers() {
    // A done blocker gates nothing: its dependant sits in tier 1, and the
    // done todo itself never appears.
    let done = todo("done-dep", TodoStatus::Done);
    let mut open = todo("open", TodoStatus::Open);
    open.blocked_by = vec!["todo.done-dep".to_owned()];
    let response = roadmap_response(std::path::Path::new("/project"), &[done, open]);
    assert_eq!(stems(&response), vec![(1, vec!["todo.open"])]);
}

#[test]
fn test_roadmap_dangling_blocker_gates_nothing() {
    let mut open = todo("open", TodoStatus::Open);
    open.blocked_by = vec!["todo.ghost".to_owned()];
    let response = roadmap_response(std::path::Path::new("/project"), &[open]);
    assert_eq!(stems(&response), vec![(1, vec!["todo.open"])]);
}

#[test]
fn test_roadmap_parent_groups_within_tier_without_ordering() {
    // `parent` groups members inside a tier; it never lifts or sinks them.
    let epic = todo("epic", TodoStatus::Open);
    let mut child_b = todo("child-b", TodoStatus::Open);
    child_b.parent = Some("todo.epic".to_owned());
    let mut child_a = todo("child-a", TodoStatus::Open);
    child_a.parent = Some("todo.epic".to_owned());
    let lone = todo("alone", TodoStatus::Open);
    let response = roadmap_response(
        std::path::Path::new("/project"),
        &[child_b, lone, epic, child_a],
    );
    assert_eq!(
        stems(&response),
        vec![(
            1,
            vec!["todo.alone", "todo.child-a", "todo.child-b", "todo.epic"],
        )],
        "parent groups cluster deterministically inside the tier"
    );
    let items = &response.tiers[0].items;
    assert_eq!(items[1].parent.as_deref(), Some("todo.epic"));
    assert_eq!(items[2].parent.as_deref(), Some("todo.epic"));
}

#[test]
fn test_roadmap_cycle_members_share_one_tier_as_a_unit() {
    // A cyclic component has no order among its members; it occupies one
    // tier as a unit at the depth of its external blockers (none here).
    let mut a = todo("a", TodoStatus::Open);
    a.blocked_by = vec!["todo.b".to_owned()];
    let mut b = todo("b", TodoStatus::Open);
    b.blocked_by = vec!["todo.a".to_owned()];
    let free = todo("free", TodoStatus::Open);
    let response = roadmap_response(std::path::Path::new("/project"), &[a, b, free]);
    assert_eq!(
        stems(&response),
        vec![(1, vec!["todo.a", "todo.b", "todo.free"])],
        "an unblocked cycle sits in tier 1 beside unblocked todos"
    );
}

#[test]
fn test_roadmap_cycle_dependant_tiers_after_the_cycle() {
    // A todo blocked by a cycle member is not part of the knot; it tiers
    // strictly after the whole component.
    let mut a = todo("a", TodoStatus::Open);
    a.blocked_by = vec!["todo.b".to_owned()];
    let mut b = todo("b", TodoStatus::Open);
    b.blocked_by = vec!["todo.a".to_owned()];
    let mut c = todo("c", TodoStatus::Open);
    c.blocked_by = vec!["todo.a".to_owned()];
    let response = roadmap_response(std::path::Path::new("/project"), &[a, b, c]);
    assert_eq!(
        stems(&response),
        vec![(1, vec!["todo.a", "todo.b"]), (2, vec!["todo.c"])],
        "cycle dependants keep topological depth over the condensation"
    );
}

#[test]
fn test_roadmap_self_loop_occupies_its_own_tier_unit() {
    let mut a = todo("a", TodoStatus::Open);
    a.blocked_by = vec!["todo.a".to_owned()];
    let mut c = todo("c", TodoStatus::Open);
    c.blocked_by = vec!["todo.a".to_owned()];
    let response = roadmap_response(std::path::Path::new("/project"), &[a, c]);
    assert_eq!(
        stems(&response),
        vec![(1, vec!["todo.a"]), (2, vec!["todo.c"])],
        "a self-loop is a one-member component; dependants tier after it"
    );
}

#[test]
fn test_roadmap_item_carries_wire_fields() {
    let mut item_todo = todo("alpha", TodoStatus::InProgress);
    item_todo.parent = Some("todo.epic".to_owned());
    item_todo.blocked_by = vec!["todo.done-thing".to_owned()];
    let response = roadmap_response(std::path::Path::new("/project"), &[item_todo]);
    let item = &response.tiers[0].items[0];
    assert_eq!(item.stem, "todo.alpha");
    assert_eq!(item.title, "alpha");
    assert_eq!(item.status, "in_progress");
    assert_eq!(item.path, "meta/todos/todo.alpha.md");
    assert_eq!(item.parent.as_deref(), Some("todo.epic"));
    assert_eq!(item.blocked_by, vec!["todo.done-thing"]);
    assert_eq!(item.rank, 100, "WorkItem todo rank");
}
