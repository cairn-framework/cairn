//! CLI todo-artefact scaffolding command.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use super::decision::{flag_values, is_kebab_slug, title_from_slug, today_utc, write_new_artefact};

/// Dispatches `cairn todo <subcommand>`.
pub(crate) fn run_todo_command(parsed: &ParsedArgs, root: &Path) -> CliResult {
    match parsed.command_args.get(1).map(String::as_str) {
        Some("new") => {
            let Some(slug) = parsed.command_args.get(2) else {
                return err(1, copy::lookup("todo.usage"));
            };
            let nodes = flag_values(&parsed.command_args, "--node");
            run_todo_new(root, slug, &nodes)
        }
        _ => err(1, copy::lookup("todo.usage")),
    }
}

/// Scaffolds `meta/todos/todo.<slug>.md` with deterministic frontmatter
/// (spec §8.2: `node`, `status`, `created`) and an empty body. This is
/// artefact scaffolding, exactly symmetric with `cairn decision new`
/// (`dec.native-todos-first`): it writes declared state for cairn to
/// validate, not a claim/sequence/close verb.
fn run_todo_new(root: &Path, slug: &str, nodes: &[String]) -> CliResult {
    if !is_kebab_slug(slug) {
        return err(1, copy::lookup("todo.invalid-slug"));
    }
    let [node] = nodes else {
        return err(1, copy::lookup("todo.missing-node"));
    };
    let content = todo_stub(slug, node, &today_utc());
    write_new_artefact(
        root,
        "meta/todos",
        &format!("todo.{slug}.md"),
        &content,
        &copy::lookup("todo.exists").replace("{slug}", slug),
        &copy::lookup("todo.created").replace("{slug}", slug),
    )
}

/// Builds the todo artefact body. Pure: the date is injected so the output
/// is deterministic for a given input.
fn todo_stub(slug: &str, node: &str, date: &str) -> String {
    format!(
        "---\nnode: {node}\nstatus: open\ncreated: {date}\n---\n\n# {}\n\n",
        title_from_slug(slug)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_stub_frontmatter_is_well_formed() {
        let stub = todo_stub("wire-jwt", "saas.api.auth", "2026-06-26");
        assert!(stub.starts_with("---\nnode: saas.api.auth\n"));
        assert!(stub.contains("status: open\n"));
        assert!(stub.contains("created: 2026-06-26\n"));
        assert!(stub.contains("# Wire Jwt"));
    }

    #[test]
    fn test_run_todo_new_rejects_bad_slug() {
        let dir = tempfile::tempdir().unwrap();
        let result = run_todo_new(dir.path(), "Bad Slug", &["app.core".to_owned()]);
        assert_eq!(result.code, 1);
    }

    #[test]
    fn test_run_todo_new_requires_exactly_one_node() {
        let dir = tempfile::tempdir().unwrap();
        let missing = run_todo_new(dir.path(), "my-task", &[]);
        assert_eq!(missing.code, 1, "missing --node must be rejected");
        let repeated = run_todo_new(
            dir.path(),
            "my-task",
            &["app.core".to_owned(), "app.other".to_owned()],
        );
        assert_eq!(repeated.code, 1, "repeated --node must be rejected");
    }

    #[test]
    fn test_run_todo_new_writes_file_and_refuses_overwrite() {
        let dir = tempfile::tempdir().unwrap();
        let first = run_todo_new(dir.path(), "my-task", &["app.core".to_owned()]);
        assert_eq!(first.code, 0, "scaffold should succeed: {}", first.stderr);
        assert!(dir.path().join("meta/todos/todo.my-task.md").exists());
        let second = run_todo_new(dir.path(), "my-task", &["app.core".to_owned()]);
        assert_eq!(second.code, 1, "must refuse to overwrite an existing todo");
    }

    #[test]
    fn test_run_todo_new_creates_meta_dir_on_fresh_project() {
        let dir = tempfile::tempdir().unwrap();
        assert!(!dir.path().join("meta").exists());
        let result = run_todo_new(dir.path(), "first-task", &["app.core".to_owned()]);
        assert_eq!(result.code, 0, "scaffold should succeed: {}", result.stderr);
        assert!(dir.path().join("meta/todos/todo.first-task.md").exists());
    }
}
