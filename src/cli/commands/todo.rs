//! CLI todo-artefact scaffolding command.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use super::decision::{flag_values, is_kebab_slug, title_from_slug, today_utc, write_new_artefact};
use crate::artefacts::frontmatter::{SetFieldError, set_field};
use crate::artefacts::registry::types::TodoStatus;
use std::fs;

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
        Some("set") => {
            let Some(slug) = parsed.command_args.get(2) else {
                return err(1, copy::lookup("todo.set-usage"));
            };
            let Some(status) = parsed.command_args.get(3) else {
                return err(1, copy::lookup("todo.set-usage"));
            };
            run_todo_set(root, slug, status, parsed.json)
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
/// Writes `status:` for `todo.<slug>.md` via a surgical frontmatter edit
/// (spec `dec.todo-write-surface`): only the `status:` line changes; every
/// other byte is preserved. This is state stewardship, not a workflow verb.
fn run_todo_set(root: &Path, slug: &str, status: &str, json: bool) -> CliResult {
    if !is_kebab_slug(slug) {
        return err(1, copy::lookup("todo.invalid-slug"));
    }
    let Some(parsed_status) = TodoStatus::from_cli(status) else {
        return err(
            1,
            &copy::lookup("todo.invalid-status").replace("{status}", status),
        );
    };
    let token = parsed_status.as_str();
    let path = root.join("meta/todos").join(format!("todo.{slug}.md"));
    if !path.exists() {
        return err(1, &copy::lookup("todo.not-found").replace("{slug}", slug));
    }
    let source = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            return err(
                1,
                &copy::lookup("todo.io-read-error")
                    .replace("{path}", &path.display().to_string())
                    .replace("{error}", &e.to_string()),
            );
        }
    };
    let updated = match set_field(&source, "status", token) {
        Ok(s) => s,
        Err(SetFieldError::NoFrontmatter) => {
            return err(
                1,
                &copy::lookup("todo.malformed-frontmatter").replace("{slug}", slug),
            );
        }
        Err(SetFieldError::KeyNotFound) => {
            return err(
                1,
                &copy::lookup("todo.missing-status-field").replace("{slug}", slug),
            );
        }
    };
    if let Err(e) = fs::write(&path, &updated) {
        return err(
            1,
            &copy::lookup("todo.io-write-error")
                .replace("{path}", &path.display().to_string())
                .replace("{error}", &e.to_string()),
        );
    }
    if json {
        ok(serde_json::json!({
            "slug": slug,
            "status": token,
            "path": path.to_string_lossy(),
        })
        .to_string())
    } else {
        ok(copy::lookup("todo.set-success")
            .replace("{slug}", slug)
            .replace("{status}", token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::{Path, PathBuf};
    use tempfile::tempdir;

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
    fn sample_todo(root: &Path, slug: &str) -> PathBuf {
        let dir = root.join("meta/todos");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join(format!("todo.{slug}.md"));
        let content = "---\nnode: cairn.kernel.cli\nstatus: open\ncreated: 2026-07-11\n---\n\n# Title\n\nBody line.\nThe status: open default is fine.\n";
        fs::write(&path, content).unwrap();
        path
    }
    fn sample_todo_crlf(root: &Path, slug: &str) -> PathBuf {
        let dir = root.join("meta/todos");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join(format!("todo.{slug}.md"));
        let content = "---\r\nnode: cairn.kernel.cli\r\nstatus: open\r\ncreated: 2026-07-11\r\n---\r\n\r\n# Title\r\n\r\nBody line.\r\nThe status: open default is fine.\r\n";
        fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn test_run_todo_set_flips_each_valid_status_surgically() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();
        let dir = root.join("meta/todos");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("todo.sample.md");
        // Frontmatter carries a nested (indented) `status:` under `details:` that
        // must NEVER be rewritten by a top-level status flip.
        let fm_tail = "details:\n  status: open\n";
        let body = "\n\n# Title\n\nBody line.\nThe status: open default is fine.\n";
        fs::write(
            &path,
            format!(
                "---\nnode: cairn.kernel.cli\nstatus: open\ncreated: 2026-07-11\n{fm_tail}---{body}"
            ),
        )
        .unwrap();
        for status in ["in_progress", "done", "blocked", "open"] {
            let res = run_todo_set(root, "sample", status, false);
            assert_eq!(res.code, 0, "expected success for status {status}");
            let after = fs::read_to_string(&path).unwrap();
            let expected = format!(
                "---\nnode: cairn.kernel.cli\nstatus: {status}\ncreated: 2026-07-11\n{fm_tail}---{body}"
            );
            assert_eq!(
                after, expected,
                "only the top-level status value may change, byte-for-byte, for status {status}; nested frontmatter status must be untouched"
            );
        }
    }

    #[test]
    fn test_run_todo_set_rejects_invalid_status() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();
        let path = sample_todo(root, "sample");
        let res = run_todo_set(root, "sample", "frobnicate", false);
        assert_eq!(res.code, 1, "invalid status must error");
        assert!(
            res.stderr
                .to_lowercase()
                .contains("open|in_progress|done|blocked")
                || res.stderr.to_lowercase().contains("invalid"),
            "unexpected msg: {}",
            res.stderr
        );
        let after = fs::read_to_string(&path).unwrap();
        assert!(
            after.contains("status: open"),
            "file must be unchanged on invalid status"
        );
    }

    #[test]
    fn test_run_todo_set_missing_file_errors() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();
        let res = run_todo_set(root, "ghost", "done", false);
        assert_eq!(res.code, 1);
        assert!(
            res.stderr.to_lowercase().contains("ghost")
                || res.stderr.to_lowercase().contains("no todo"),
            "unexpected: {}",
            res.stderr
        );
    }
    #[test]
    fn test_set_field_requires_leading_fence() {
        // A body with thematic breaks but no leading frontmatter must be left
        // untouched (return Err) so we never mutate a non-frontmatter region.
        let body = "# Title\n\n---\nstatus: open\n---\n";
        assert_eq!(
            set_field(body, "status", "done"),
            Err(SetFieldError::NoFrontmatter)
        );
        // A proper frontmatter block without the key returns Err(KeyNotFound).
        let fm = "---\nnode: x\n---\nbody\n";
        assert_eq!(
            set_field(fm, "status", "done"),
            Err(SetFieldError::KeyNotFound)
        );
    }

    #[test]
    fn test_run_todo_set_json_shape() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();
        let _path = sample_todo(root, "sample");
        let res = run_todo_set(root, "sample", "done", true);
        assert_eq!(res.code, 0);
        let v: serde_json::Value =
            serde_json::from_str(&res.stdout).expect("stdout must be valid json");
        assert_eq!(v["slug"], "sample");
        assert_eq!(v["status"], "done");
        assert!(
            v["path"].as_str().unwrap().ends_with("todo.sample.md"),
            "path: {}",
            v["path"]
        );
    }
    #[test]
    fn test_run_todo_set_rejects_bad_slug() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();
        let res = run_todo_set(root, "../escape", "done", false);
        assert_eq!(res.code, 1, "non-kebab slug must be rejected");
        assert!(
            !root.join("meta/todos").exists(),
            "no todo dir should be created for a bad slug"
        );
    }
    #[test]
    fn test_run_todo_set_crlf_roundtrip() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();
        let path = sample_todo_crlf(root, "sample");
        let res = run_todo_set(root, "sample", "done", false);
        assert_eq!(res.code, 0, "crlf todo must be settable: {}", res.stderr);
        let after = fs::read_to_string(&path).unwrap();
        let expected = "---\r\nnode: cairn.kernel.cli\r\nstatus: done\r\ncreated: 2026-07-11\r\n---\r\n\r\n# Title\r\n\r\nBody line.\r\nThe status: open default is fine.\r\n";
        assert_eq!(
            after, expected,
            "crlf document must change only the status value, preserving every \\r"
        );
    }

    #[test]
    fn test_set_field_crlf_byte_for_byte() {
        let src = "---\r\nnode: cairn.kernel.cli\r\nstatus: open\r\ncreated: 2026-07-11\r\n---\r\n\r\n# Title\r\n\r\nBody line.\r\nThe status: open default is fine.\r\n";
        let out = set_field(src, "status", "done").expect("must rewrite crlf document");
        assert_eq!(
            out,
            "---\r\nnode: cairn.kernel.cli\r\nstatus: done\r\ncreated: 2026-07-11\r\n---\r\n\r\n# Title\r\n\r\nBody line.\r\nThe status: open default is fine.\r\n"
        );
    }

    #[test]
    fn test_set_field_ignores_nested_status() {
        // A nested (indented) status under a map key with NO top-level status
        // field must return Err(KeyNotFound), never rewrite the nested line.
        let fm = "---\nnode: x\nmap:\n  status: open\n---\nbody\n";
        assert_eq!(
            set_field(fm, "status", "done"),
            Err(SetFieldError::KeyNotFound)
        );
    }

    #[test]
    fn test_set_field_rewrites_only_top_level_key() {
        let fm = "---\nstatus: open\nstatus_foo: bar\n---\nbody\n";
        let out = set_field(fm, "status", "done").expect("must rewrite top-level");
        assert_eq!(out, "---\nstatus: done\nstatus_foo: bar\n---\nbody\n");
    }

    #[test]
    fn test_run_todo_set_missing_status_field_errors() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();
        let dir = root.join("meta/todos");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("todo.nostatus.md");
        let content = "---\nnode: cairn.kernel.cli\ncreated: 2026-07-11\n---\n\n# Title\n";
        fs::write(&path, content).unwrap();
        let res = run_todo_set(root, "nostatus", "done", false);
        assert_eq!(res.code, 1, "missing status field must error");
        assert!(
            res.stderr.to_lowercase().contains("status"),
            "unexpected: {}",
            res.stderr
        );
        let after = fs::read_to_string(&path).unwrap();
        assert_eq!(after, content, "file must be unchanged on missing field");
    }

    #[test]
    fn test_run_todo_set_preserves_nested_status() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();
        let dir = root.join("meta/todos");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("todo.nested.md");
        let content = "---\nnode: x\nstatus: open\nsub:\n  status: open\n---\n\n# Title\n\nBody with status: open inline.\n";
        fs::write(&path, content).unwrap();
        let res = run_todo_set(root, "nested", "done", false);
        assert_eq!(res.code, 0, "must succeed: {}", res.stderr);
        let after = fs::read_to_string(&path).unwrap();
        assert_eq!(
            after.matches("status: done").count(),
            1,
            "only the top-level status should flip:\n{after}"
        );
        assert!(
            after.contains("  status: open"),
            "nested status must be untouched:\n{after}"
        );
        assert!(
            after.contains("Body with status: open inline."),
            "body must be untouched:\n{after}"
        );
    }
    #[test]
    fn test_set_field_no_closing_fence() {
        // An opening fence with no closing fence is malformed frontmatter.
        let bad = "---\nnode: x\nstatus: open\nbody without close\n";
        assert_eq!(
            set_field(bad, "status", "done"),
            Err(SetFieldError::NoFrontmatter)
        );
    }

    #[test]
    fn test_run_todo_set_malformed_frontmatter_errors() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();
        let dir = root.join("meta/todos");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("todo.broken.md");
        // Opening fence present, but no closing fence -> malformed frontmatter.
        let content = "---\nnode: cairn.kernel.cli\nstatus: open\nbody has no closing fence\n";
        fs::write(&path, content).unwrap();
        let res = run_todo_set(root, "broken", "done", false);
        assert_eq!(res.code, 1, "malformed frontmatter must error");
        assert!(
            res.stderr.to_lowercase().contains("frontmatter"),
            "unexpected: {}",
            res.stderr
        );
        let after = fs::read_to_string(&path).unwrap();
        assert_eq!(
            after, content,
            "file must be unchanged on malformed frontmatter"
        );
    }
}
