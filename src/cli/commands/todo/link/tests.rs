//! Tests for `cairn todo link`/`unlink`.

use super::*;
use std::path::PathBuf;
use tempfile::tempdir;

fn args(blocked_by: &[&str], parents: &[&str], related: &[&str]) -> Args {
    Args {
        blocked_by: blocked_by.iter().map(ToString::to_string).collect(),
        parents: parents.iter().map(ToString::to_string).collect(),
        related: related.iter().map(ToString::to_string).collect(),
        json: false,
    }
}

fn write_todo(root: &Path, slug: &str, content: &str) -> PathBuf {
    let dir = root.join("meta/todos");
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join(format!("todo.{slug}.md"));
    fs::write(&path, content).unwrap();
    path
}

#[test]
fn test_strict_flag_values_rejects_valueless_occurrence() {
    // A trailing flag, or one followed by another flag, must error rather
    // than silently dropping the requested edit.
    let trailing: Vec<String> = [
        "todo",
        "link",
        "sample",
        "--related",
        "dec.rule",
        "--parent",
    ]
    .iter()
    .map(ToString::to_string)
    .collect();
    assert!(strict_flag_values(&trailing, "--parent").is_err());
    assert_eq!(
        strict_flag_values(&trailing, "--related").ok().unwrap(),
        vec!["dec.rule".to_owned()]
    );
    let flag_follows: Vec<String> = ["todo", "link", "sample", "--blocked-by", "--related", "x"]
        .iter()
        .map(ToString::to_string)
        .collect();
    assert!(strict_flag_values(&flag_follows, "--blocked-by").is_err());
}

#[test]
fn test_unlink_removes_malformed_inline_parent() {
    // `parent: [todo.a]` is malformed (the scanner warns about it); the
    // sanctioned write path must still remove exactly that raw value.
    let tmp = tempdir().unwrap();
    let path = write_todo(
        tmp.path(),
        "sample",
        "---\nnode: app.core\nstatus: open\ncreated: 2026-08-01\nparent: [todo.a]\n---\nBody.\n",
    );
    let result = run(
        tmp.path(),
        "sample",
        &args(&[], &["[todo.a]"], &[]),
        Mode::Unlink,
    );
    assert_eq!(result.code, 0, "{}", result.stderr);
    let written = fs::read_to_string(&path).unwrap();
    assert!(!written.contains("parent"), "raw malformed scalar removed");
}
fn sample_todo(root: &Path, slug: &str) -> PathBuf {
    write_todo(
        root,
        slug,
        "---\nnode: app.core\nstatus: open\ncreated: 2026-08-01\n---\n\n# Title\n\nBody.\n",
    )
}

#[test]
fn test_link_adds_fields_and_preserves_body() {
    let tmp = tempdir().unwrap();
    let path = sample_todo(tmp.path(), "sample");
    let result = run(
        tmp.path(),
        "sample",
        &args(&["todo.dep"], &["todo.epic"], &["dec.rule"]),
        Mode::Link,
    );
    assert_eq!(result.code, 0, "{}", result.stderr);
    let written = fs::read_to_string(&path).unwrap();
    assert!(written.contains("blocked_by: [todo.dep]\n"));
    assert!(written.contains("parent: todo.epic\n"));
    assert!(written.contains("related: [dec.rule]\n"));
    assert!(written.ends_with("# Title\n\nBody.\n"), "body untouched");
}

#[test]
fn test_link_touches_only_named_fields() {
    // An untouched block-list field must keep its exact authored bytes;
    // only the invocation-named field line may change.
    let tmp = tempdir().unwrap();
    let path = write_todo(
        tmp.path(),
        "sample",
        "---\nnode: app.core\nstatus: open\ncreated: 2026-08-01\nrelated:\n  - dec.rule\n  - res.study\n---\nBody.\n",
    );
    let result = run(
        tmp.path(),
        "sample",
        &args(&["todo.dep"], &[], &[]),
        Mode::Link,
    );
    assert_eq!(result.code, 0, "{}", result.stderr);
    let written = fs::read_to_string(&path).unwrap();
    assert!(
        written.contains("related:\n  - dec.rule\n  - res.study\n"),
        "untouched block list must keep its authored form: {written}"
    );
    assert!(written.contains("blocked_by: [todo.dep]\n"));
}

#[test]
fn test_link_unions_without_duplicates() {
    let tmp = tempdir().unwrap();
    let path = sample_todo(tmp.path(), "sample");
    run(
        tmp.path(),
        "sample",
        &args(&["todo.a"], &[], &[]),
        Mode::Link,
    );
    let result = run(
        tmp.path(),
        "sample",
        &args(&["todo.a", "todo.b"], &[], &[]),
        Mode::Link,
    );
    assert_eq!(result.code, 0);
    let written = fs::read_to_string(&path).unwrap();
    assert!(written.contains("blocked_by: [todo.a, todo.b]\n"));
}

#[test]
fn test_link_parent_replaces_existing() {
    let tmp = tempdir().unwrap();
    let path = sample_todo(tmp.path(), "sample");
    run(
        tmp.path(),
        "sample",
        &args(&[], &["todo.first"], &[]),
        Mode::Link,
    );
    run(
        tmp.path(),
        "sample",
        &args(&[], &["todo.second"], &[]),
        Mode::Link,
    );
    let written = fs::read_to_string(&path).unwrap();
    assert!(written.contains("parent: todo.second\n"));
    assert!(!written.contains("todo.first"));
}

#[test]
fn test_link_rejects_invalid_reference_shape() {
    let tmp = tempdir().unwrap();
    sample_todo(tmp.path(), "sample");
    for bad in [
        args(&["dep"], &[], &[]),
        args(&["dec.rule"], &[], &[]),
        args(&[], &[], &["rev.person"]),
    ] {
        let result = run(tmp.path(), "sample", &bad, Mode::Link);
        assert_eq!(result.code, 1, "shape must be rejected");
    }
    // A namespaced dotted id is legal for --related.
    let result = run(
        tmp.path(),
        "sample",
        &args(&[], &[], &["res.gas-city.analysis"]),
        Mode::Link,
    );
    assert_eq!(result.code, 0, "{}", result.stderr);
}

#[test]
fn test_link_rejects_multiple_parents() {
    let tmp = tempdir().unwrap();
    sample_todo(tmp.path(), "sample");
    let result = run(
        tmp.path(),
        "sample",
        &args(&[], &["todo.a", "todo.b"], &[]),
        Mode::Link,
    );
    assert_eq!(result.code, 1);
}

#[test]
fn test_link_requires_at_least_one_flag() {
    let tmp = tempdir().unwrap();
    sample_todo(tmp.path(), "sample");
    let result = run(tmp.path(), "sample", &args(&[], &[], &[]), Mode::Link);
    assert_eq!(result.code, 1);
}

#[test]
fn test_unlink_removes_entries_and_drops_empty_lines() {
    let tmp = tempdir().unwrap();
    let path = sample_todo(tmp.path(), "sample");
    run(
        tmp.path(),
        "sample",
        &args(&["todo.a", "todo.b"], &["todo.epic"], &["dec.rule"]),
        Mode::Link,
    );
    let result = run(
        tmp.path(),
        "sample",
        &args(&["todo.a"], &[], &[]),
        Mode::Unlink,
    );
    assert_eq!(result.code, 0, "{}", result.stderr);
    let written = fs::read_to_string(&path).unwrap();
    assert!(written.contains("blocked_by: [todo.b]\n"));
    let result = run(
        tmp.path(),
        "sample",
        &args(&["todo.b"], &["todo.epic"], &["dec.rule"]),
        Mode::Unlink,
    );
    assert_eq!(result.code, 0, "{}", result.stderr);
    let written = fs::read_to_string(&path).unwrap();
    assert!(!written.contains("blocked_by"), "emptied list line dropped");
    assert!(!written.contains("parent:"), "cleared parent line dropped");
    assert!(!written.contains("related"), "emptied list line dropped");
    assert!(written.contains("status: open\n"), "other keys untouched");
}

#[test]
fn test_unlink_removes_invalid_shaped_entry() {
    // The scanner warns about `rev.someone`; the sanctioned write path must
    // still be able to remove it (shape checks apply only when adding).
    let tmp = tempdir().unwrap();
    let path = write_todo(
        tmp.path(),
        "sample",
        "---\nnode: app.core\nstatus: open\ncreated: 2026-08-01\nrelated: [rev.someone]\n---\nBody.\n",
    );
    let result = run(
        tmp.path(),
        "sample",
        &args(&[], &[], &["rev.someone"]),
        Mode::Unlink,
    );
    assert_eq!(result.code, 0, "{}", result.stderr);
    let written = fs::read_to_string(&path).unwrap();
    assert!(!written.contains("rev.someone"));
}

#[test]
fn test_unlink_missing_entry_errors() {
    let tmp = tempdir().unwrap();
    sample_todo(tmp.path(), "sample");
    let result = run(
        tmp.path(),
        "sample",
        &args(&["todo.absent"], &[], &[]),
        Mode::Unlink,
    );
    assert_eq!(result.code, 1);
    let result = run(
        tmp.path(),
        "sample",
        &args(&[], &["todo.absent"], &[]),
        Mode::Unlink,
    );
    assert_eq!(result.code, 1, "unlink of a parent that is not set errors");
}

#[test]
fn test_link_missing_todo_errors() {
    let tmp = tempdir().unwrap();
    let result = run(
        tmp.path(),
        "ghost",
        &args(&["todo.a"], &[], &[]),
        Mode::Link,
    );
    assert_eq!(result.code, 1);
}
