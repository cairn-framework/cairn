//! Behavioural tests for `todo.output-token-efficiency`, Part B.
//!
//! `cairn status --brief` renders a token-lean view (todo count + capped list,
//! a finding summary, and a deduplicated log tail) whose size does not grow
//! with backlog. Default `status` is unchanged.

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
fn temp_root(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("cairn-ote-{name}-{suffix}"));
    fs::create_dir_all(&root)?;
    Ok(root)
}
fn run_git(root: &Path, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let out = Command::new("git").current_dir(root).args(args).output()?;
    if !out.status.success() {
        return Err(format!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr)
        )
        .into());
    }
    Ok(())
}
fn git_init(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    run_git(root, &["init", "--quiet"])?;
    run_git(root, &["config", "user.email", "test@example.com"])?;
    run_git(root, &["config", "user.name", "Test"])?;
    run_git(root, &["config", "commit.gpgsign", "false"])?;
    Ok(())
}
fn git_commit(root: &Path, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
    run_git(root, &["add", "-A"])?;
    run_git(root, &["commit", "--quiet", "-m", msg])?;
    Ok(())
}
fn run(root: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(root)
        .args(args)
        .output()
        .expect("cairn binary executes")
}
fn run_stdout(root: &Path, args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let out = run(root, args);
    Ok(String::from_utf8(out.stdout)?)
}
/// Spec-rule registry row used by the finding-summary scenario.
const DEFERRED_ROW: &str =
    "| R | spec:634 | - | pending | dec.revisit-trigger-correlator-deferred |
";

fn write_spec_project(root: &Path, registry_rows: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(root.join("src"))?;
    fs::write(root.join("src/lib.rs"), "pub fn placeholder() {}\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        "System App \"desc\" id \"app\" {}\n",
    )?;
    fs::create_dir_all(root.join("docs/registries"))?;
    fs::write(
        root.join("docs/registries/spec-rules.md"),
        format!(
            "| Rule | Spec | Code | Status | Deferred-by |\n|---|---|---|---|---|\n{registry_rows}"
        ),
    )?;
    Ok(())
}
fn write_status_project(root: &Path, todo_count: usize) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(root.join("src"))?;
    fs::write(root.join("src/lib.rs"), "pub fn placeholder() {}\n")?;
    fs::create_dir_all(root.join("meta/todos"))?;
    fs::write(
        root.join("cairn.blueprint"),
        "System App \"desc\" id \"app\" {\n    todos \"./meta/todos\"\n}\n",
    )?;
    for i in 0..todo_count {
        let content =
            format!("---\nnode: app\nstatus: open\ncreated: 2026-07-14\n---\n\n# Todo {i}\n");
        fs::write(root.join(format!("meta/todos/todo.{i}.md")), content)?;
    }
    Ok(())
}

/// Extracts the open-todos block (from `Open todos:` up to `Recent log entries:`).
fn extract_todo_block(out: &str) -> &str {
    let start = out.find("Open todos:").expect("open todos header present");
    let rest = &out[start..];
    let end = rest
        .find("Recent log entries:")
        .expect("log header present");
    &rest[..end]
}
/// Extracts the recent-log block (content after the `Recent log entries:` header,
/// up to `Next recommended:`).
fn extract_log_block(out: &str) -> &str {
    let marker = "Recent log entries:";
    let start = out.find(marker).expect("log header present");
    let after = &out[start + marker.len()..];
    let after_line = after.find('\n').map_or("", |i| &after[i + 1..]);
    let end = after_line
        .find("Next recommended:")
        .unwrap_or(after_line.len());
    &after_line[..end]
}
// ── Part B: status --brief ────────────────────────────────────────────────────

#[test]
fn status_brief_constant_todo_block() -> Result<(), Box<dyn std::error::Error>> {
    let root5 = temp_root("brief-5")?;
    git_init(&root5)?;
    write_status_project(&root5, 5)?;
    git_commit(&root5, "init")?;
    let s5 = run_stdout(&root5, &["--brief", "status"])?;
    assert!(s5.contains("Open todos: 5"), "expected count 5, got:\n{s5}");
    assert!(
        s5.contains("(+0 more)"),
        "expected (+0 more) when N<=cap, got:\n{s5}"
    );

    let root1k = temp_root("brief-1k")?;
    git_init(&root1k)?;
    write_status_project(&root1k, 1000)?;
    git_commit(&root1k, "init")?;
    let s1k = run_stdout(&root1k, &["--brief", "status"])?;
    assert!(
        s1k.contains("Open todos: 1000"),
        "expected count 1000, got:\n{s1k}"
    );
    assert!(
        s1k.contains("(+995 more)"),
        "expected (+995 more) overflow, got:\n{s1k}"
    );

    let block5 = extract_todo_block(&s5);
    let block1k = extract_todo_block(&s1k);
    assert_eq!(
        block5.lines().count(),
        block1k.lines().count(),
        "todo block line count must be constant across backlog sizes:\n--N=5--\n{block5}\n--N=1000--\n{block1k}"
    );

    assert!(
        s1k.len() < s5.len() + 64,
        "brief todo block must not grow materially with backlog (N=5={} bytes, N=1000={} bytes)",
        s5.len(),
        s1k.len()
    );
    Ok(())
}

#[test]
fn status_brief_finding_summary_present() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("brief-findings")?;
    git_init(&root)?;
    write_spec_project(&root, DEFERRED_ROW)?;
    git_commit(&root, "init")?;

    let s = run_stdout(&root, &["--brief", "status"])?;
    assert!(
        s.lines().any(|line| {
            line.starts_with("Findings:")
                && line.contains("errors")
                && line.contains("warnings")
                && line.contains("info")
        }),
        "brief status must show a Findings: <T> (<E> errors, <W> warnings, <I> info) line, got:\n{s}"
    );
    Ok(())
}

#[test]
fn status_brief_deduped_capped_log_tail() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("brief-log")?;
    git_init(&root)?;
    write_status_project(&root, 5)?;
    git_commit(&root, "init")?;

    let mut log = String::new();
    for i in 0..10 {
        let line = if i % 2 == 0 {
            "build passed"
        } else {
            "scan clean"
        };
        log.push_str(line);
        log.push('\n');
    }
    fs::create_dir_all(root.join(".cairn"))?;
    fs::write(root.join(".cairn/log.md"), log)?;

    let s = run_stdout(&root, &["--brief", "status"])?;
    let block = extract_log_block(&s);
    let lines: Vec<&str> = block.lines().filter(|l| !l.trim().is_empty()).collect();
    assert!(
        lines.len() <= 5,
        "log tail must be capped at 5 unique lines, got:\n{block}"
    );
    let mut seen = std::collections::HashSet::new();
    for line in &lines {
        assert!(
            seen.insert(*line),
            "log tail must deduplicate, duplicate line: {line}\n{block}"
        );
    }
    assert_eq!(
        lines.first(),
        Some(&"scan clean"),
        "log tail must be newest-first, got:\n{block}"
    );
    Ok(())
}

#[test]
fn status_default_unchanged_full_listing() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("default-1k")?;
    git_init(&root)?;
    write_status_project(&root, 1000)?;
    git_commit(&root, "init")?;

    let s = run_stdout(&root, &["status"])?;
    let block = extract_todo_block(&s);
    let detail = block.lines().filter(|l| l.contains(" [open] ")).count();
    assert_eq!(
        detail, 1000,
        "default status must list every open todo (not capped), got:\n{block}"
    );
    assert!(
        !s.contains("Open todos: 1000"),
        "default status must not use the brief count header, got:\n{s}"
    );
    assert!(
        !s.contains("Findings:"),
        "default status must not show the brief findings summary, got:\n{s}"
    );
    Ok(())
}

#[test]
fn status_default_small_fixture_unchanged() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("default-snapshot")?;
    git_init(&root)?;
    write_status_project(&root, 2)?;
    git_commit(&root, "init")?;

    let stdout = run_stdout(&root, &["status"])?;
    let expected = "\
Status:
Active changes:
None
Open todos:
- app [open] meta/todos/todo.0.md
- app [open] meta/todos/todo.1.md
Recent log entries:
None
Next recommended:
Todo 0 (native todo, node: app)
";
    assert_eq!(
        stdout, expected,
        "default status human output must stay byte-for-byte unchanged"
    );
    Ok(())
}
