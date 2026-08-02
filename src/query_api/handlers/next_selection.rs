//! Selects the next recommended unit of work, shared by `cairn next`
//! (`cli::render::remediate::render_next`) and the `status` query
//! (`project::status_json`) so both surfaces agree on what to do next.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use super::remediate::{health_json, remediate_actions_raw};
use super::work_item::WorkItem;

/// The next unit of work to recommend, per `dec.native-todos-first`:
/// outstanding remediation findings take priority (returned as the top
/// remediation action), else the top open native todo, else the top ready
/// bead from the beads backlog.
pub(crate) enum NextSelection<'a> {
    /// The project has outstanding findings; carries the top remediation
    /// action, or `None` if `remediate_json` produced no actions (it
    /// currently always emits a placeholder action, but this stays
    /// defensive rather than assuming that).
    Dirty(Option<Value>),
    /// The project is clean of findings; carries the selected clean-state
    /// item.
    Clean(CleanItem<'a>),
}

/// The selected item when the project is clean of findings.
pub(crate) enum CleanItem<'a> {
    /// Top open native todo.
    NativeTodo(&'a Todo),
    /// Top ready bead from the backlog.
    Bead(crate::state::backlog::BacklogItem),
    /// Nothing to do.
    None,
}

/// Ranks already-resolved candidates per `dec.native-todos-first`: a
/// remediation action (dirty project) outranks a native todo, which
/// outranks a bead. Pure and side-effect free so the priority order is
/// unit-testable without a full lint/hooks fixture; `select_next` below
/// resolves the candidates and calls this.
fn rank_next(
    clean: bool,
    action: Option<Value>,
    todo: Option<&Todo>,
    bead: Option<crate::state::backlog::BacklogItem>,
) -> NextSelection<'_> {
    if !clean {
        return NextSelection::Dirty(action);
    }
    match todo {
        Some(top) => NextSelection::Clean(CleanItem::NativeTodo(top)),
        None => NextSelection::Clean(bead.map_or(CleanItem::None, CleanItem::Bead)),
    }
}

/// Selects the next recommended unit of work per `dec.native-todos-first`.
/// This is the single source of truth for "what should I do next": callers
/// render the selection, they never re-derive it.
#[must_use]
pub(crate) fn select_next<'a>(
    root: &Path,
    changes_dir: &Path,
    scan_result: &'a scanner::ScanResult,
) -> NextSelection<'a> {
    let health = health_json(root, changes_dir, scan_result);
    let clean = health
        .get("clean")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if !clean {
        let action = remediate_actions_raw(root, changes_dir, scan_result)
            .into_iter()
            .next();
        return rank_next(false, action, None, None);
    }
    let todo = open_native_todos(scan_result).into_iter().next();
    if todo.is_some() {
        return rank_next(true, None, todo, None);
    }
    let backlog = crate::state::backlog::read(root);
    let bead = crate::state::backlog::ready(&backlog)
        .into_iter()
        .next()
        .cloned();
    rank_next(true, None, None, bead)
}

/// Returns open native todos from the scan result, sorted by creation date
/// then path so ties resolve deterministically.
#[must_use]
pub(crate) fn open_native_todos(scan_result: &scanner::ScanResult) -> Vec<&Todo> {
    let mut todos: Vec<&Todo> = scan_result
        .artefacts
        .todos
        .iter()
        .filter(|todo| todo.status == TodoStatus::Open)
        .collect();
    todos.sort_by(|a, b| a.created.cmp(&b.created).then_with(|| a.path.cmp(&b.path)));
    todos
}

/// Extracts a concise title from a markdown artefact body.
pub(crate) fn decision_summary(body: &str) -> String {
    let line = body
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .unwrap_or("");
    let cleaned = line.trim_start_matches('#').trim();
    if cleaned.chars().count() > 100 {
        let truncated: String = cleaned.chars().take(97).collect();
        format!("{truncated}...")
    } else {
        cleaned.to_owned()
    }
}

/// Projects a resolved selection into the shared wire shape.
pub(crate) fn work_item_for_selection(selection: &NextSelection<'_>) -> Option<WorkItem> {
    match selection {
        NextSelection::Dirty(action) => action
            .as_ref()
            .and_then(super::work_item::from_finding_action),
        NextSelection::Clean(CleanItem::NativeTodo(todo)) => Some(WorkItem::from_todo(todo)),
        NextSelection::Clean(CleanItem::Bead(bead)) => Some(WorkItem::from_bead(bead)),
        NextSelection::Clean(CleanItem::None) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn todo_fixture() -> Todo {
        Todo {
            path: "./todo.md".to_owned(),
            node: "app".to_owned(),
            status: TodoStatus::Open,
            created: "2026-01-01".to_owned(),
            satisfies: None,
            blocked_by: Vec::new(),
            parent: None,
            related: Vec::new(),
            defers: Vec::new(),
            body: "# Wire the thing".to_owned(),
        }
    }

    fn bead_fixture() -> crate::state::backlog::BacklogItem {
        crate::state::backlog::BacklogItem {
            id: "cairn-aaa".to_owned(),
            title: "Do thing".to_owned(),
            status: "open".to_owned(),
            priority: 2,
            issue_type: "task".to_owned(),
            description: String::new(),
            labels: Vec::new(),
            dependencies: Vec::new(),
        }
    }

    #[test]
    fn rank_next_prefers_action_over_todo_and_bead_when_dirty() {
        let action = json!({"action": "refine"});
        let todo = todo_fixture();
        let selection = rank_next(
            false,
            Some(action.clone()),
            Some(&todo),
            Some(bead_fixture()),
        );
        let NextSelection::Dirty(picked) = selection else {
            panic!("expected a Dirty selection when the project has findings");
        };
        assert_eq!(picked, Some(action));
    }

    #[test]
    fn rank_next_prefers_todo_over_bead_when_clean() {
        let todo = todo_fixture();
        let selection = rank_next(true, None, Some(&todo), Some(bead_fixture()));
        let NextSelection::Clean(CleanItem::NativeTodo(picked)) = selection else {
            panic!("expected the native todo to win over the bead");
        };
        assert_eq!(picked.node, "app");
    }

    #[test]
    fn rank_next_falls_back_to_bead_when_clean_and_no_todo() {
        let selection = rank_next(true, None, None, Some(bead_fixture()));
        let NextSelection::Clean(CleanItem::Bead(picked)) = selection else {
            panic!("expected the bead to win when no native todo is open");
        };
        assert_eq!(picked.id, "cairn-aaa");
    }

    #[test]
    fn rank_next_none_when_clean_and_nothing_ready() {
        let selection = rank_next(true, None, None, None);
        assert!(matches!(selection, NextSelection::Clean(CleanItem::None)));
    }
}
