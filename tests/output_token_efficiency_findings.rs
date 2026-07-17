//! Behavioural tests for `todo.output-token-efficiency`.
//!
//! Part A: deferred findings collapse to one summary line in scan/lint/hook
//! output, expand via `--verbose`, and resurface in full when the deferral no
//! longer applies (decision cleared or rule promoted pending->enforced while
//! the emitter is still missing). `--json` wire format is unchanged.

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

/// Spec-rule registry rows.
const DEFERRED_ROW: &str =
    "| R | spec:634 | - | pending | dec.revisit-trigger-correlator-deferred |\n";
const CLEARED_ROW: &str = "| R | spec:634 | - | pending | |\n";
const ENFORCED_ROW: &str =
    "| R | spec:634 | - | enforced | dec.revisit-trigger-correlator-deferred |\n";
const TWO_DEFERRED_ROWS: &str = "| R | spec:634 | - | pending | dec.revisit-trigger-correlator-deferred |\n\
     | S | spec:635 | - | pending | dec.revisit-trigger-correlator-deferred |\n";

const DEFERRED_SUFFIX: &str = "(deferred by dec.revisit-trigger-correlator-deferred)";
const DEFERRED_DECISION: &str = "dec.revisit-trigger-correlator-deferred";
const DEFERRED_DECISION_A: &str = "dec.alpha";
const DEFERRED_DECISION_B: &str = "dec.beta";
const TWO_DIFFERENT_DEFERRED_ROWS: &str = "| R | spec:634 | - | pending | dec.alpha |\n\
     | S | spec:635 | - | pending | dec.beta |\n";

/// Resolves the `findings.deferred-collapsed` copy template (the source of
/// truth for the collapsed line) so assertions track copy edits rather than
/// hardcoding English.
fn collapsed_line(count: usize, decision: &str) -> String {
    let toml_src = include_str!("../docs/design-system/copy.toml");
    let table: toml::Table = toml_src.parse().expect("copy.toml must parse as TOML");
    let tmpl = table
        .get("findings")
        .and_then(|f| f.as_table())
        .and_then(|f| f.get("deferred-collapsed"))
        .and_then(|v| v.as_str())
        .expect("findings.deferred-collapsed key present");
    let noun = if count == 1 { "finding" } else { "findings" };
    tmpl.replace("{count}", &count.to_string())
        .replace("{noun}", noun)
        .replace("{decision}", decision)
}

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

fn count_substring(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
}

// ── Part A: scan ─────────────────────────────────────────────────────────────

#[test]
fn scan_collapses_deferred_by_default() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("scan-collapse")?;
    git_init(&root)?;
    write_spec_project(&root, DEFERRED_ROW)?;
    git_commit(&root, "init")?;

    let stdout = run_stdout(&root, &["scan"])?;
    assert!(
        stdout.contains("deferred finding"),
        "expected collapsed summary line, got:\n{stdout}"
    );
    assert!(
        stdout.contains(DEFERRED_DECISION),
        "collapsed summary must name the decision, got:\n{stdout}"
    );
    assert!(
        !stdout.contains(DEFERRED_SUFFIX),
        "deferred finding must NOT render full text by default, got:\n{stdout}"
    );
    Ok(())
}

#[test]
fn scan_verbose_renders_full() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("scan-verbose")?;
    git_init(&root)?;
    write_spec_project(&root, DEFERRED_ROW)?;
    git_commit(&root, "init")?;

    let stdout = run_stdout(&root, &["--verbose", "scan"])?;
    assert!(
        stdout.contains(DEFERRED_SUFFIX),
        "expected full deferred text with --verbose, got:\n{stdout}"
    );
    assert!(
        !stdout.contains("deferred finding deferred by"),
        "--verbose must not collapse, got:\n{stdout}"
    );
    Ok(())
}

#[test]
fn scan_json_contains_full_message() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("scan-json")?;
    git_init(&root)?;
    write_spec_project(&root, DEFERRED_ROW)?;
    git_commit(&root, "init")?;

    let out = run(&root, &["--json", "scan"]);
    assert!(out.status.success(), "scan --json exits 0");
    let stdout = String::from_utf8(out.stdout)?;
    let value: serde_json::Value = serde_json::from_str(&stdout)?;
    let findings = value["findings"].as_array().expect("findings array");
    let finding = findings
        .iter()
        .find(|f| f["code"] == "CAIRN_SPEC_RULE_UNIMPLEMENTED")
        .expect("deferred finding present in json");
    let message = finding["message"].as_str().unwrap();
    assert!(
        message.contains(DEFERRED_SUFFIX),
        "json message must keep full deferred text, got: {message}"
    );
    Ok(())
}

#[test]
fn scan_json_identical_with_or_without_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("scan-json-verbose")?;
    git_init(&root)?;
    write_spec_project(&root, DEFERRED_ROW)?;
    git_commit(&root, "init")?;

    let a = run(&root, &["--json", "scan"]);
    let b = run(&root, &["--json", "--verbose", "scan"]);
    assert!(a.status.success() && b.status.success());
    assert_eq!(
        a.stdout, b.stdout,
        "JSON wire format must not change with --verbose"
    );
    Ok(())
}

#[test]
fn scan_collapses_multiple_same_decision_to_one_line() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("multi-collapse")?;
    git_init(&root)?;
    write_spec_project(&root, TWO_DEFERRED_ROWS)?;
    git_commit(&root, "init")?;

    let stdout = run_stdout(&root, &["scan"])?;
    let expected = collapsed_line(2, DEFERRED_DECISION);
    assert_eq!(
        count_substring(&stdout, &expected),
        1,
        "two deferred findings sharing one decision must collapse to exactly one summary line, got:\n{stdout}"
    );
    assert_eq!(
        count_substring(&stdout, DEFERRED_SUFFIX),
        0,
        "no full deferred text may appear by default, got:\n{stdout}"
    );
    Ok(())
}

#[test]
fn scan_verbose_shows_all_deferred_findings() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("multi-verbose")?;
    git_init(&root)?;
    write_spec_project(&root, TWO_DEFERRED_ROWS)?;
    git_commit(&root, "init")?;

    let stdout = run_stdout(&root, &["--verbose", "scan"])?;
    assert_eq!(
        count_substring(&stdout, DEFERRED_SUFFIX),
        2,
        "--verbose must surface every full deferred finding, got:\n{stdout}"
    );
    assert!(
        !stdout.contains("2 deferred finding"),
        "--verbose must not collapse, got:\n{stdout}"
    );
    Ok(())
}

#[test]
fn scan_collapses_different_decisions_to_separate_lines() -> Result<(), Box<dyn std::error::Error>>
{
    let root = temp_root("multi-different")?;
    git_init(&root)?;
    write_spec_project(&root, TWO_DIFFERENT_DEFERRED_ROWS)?;
    git_commit(&root, "init")?;

    let stdout = run_stdout(&root, &["scan"])?;
    let line_a = collapsed_line(1, DEFERRED_DECISION_A);
    let line_b = collapsed_line(1, DEFERRED_DECISION_B);
    assert_eq!(
        count_substring(&stdout, &line_a),
        1,
        "each distinct deciding decision must collapse to its own summary line, got:\n{stdout}"
    );
    assert_eq!(
        count_substring(&stdout, &line_b),
        1,
        "each distinct deciding decision must collapse to its own summary line, got:\n{stdout}"
    );

    let verbose = run_stdout(&root, &["--verbose", "scan"])?;
    assert!(
        verbose.contains(&format!("(deferred by {DEFERRED_DECISION_A})")),
        "--verbose must render every full deferred finding, got:\n{verbose}"
    );
    assert!(
        verbose.contains(&format!("(deferred by {DEFERRED_DECISION_B})")),
        "--verbose must render every full deferred finding, got:\n{verbose}"
    );
    assert!(
        !verbose.contains(&line_a),
        "--verbose must not collapse, got:\n{verbose}"
    );
    assert!(
        !verbose.contains(&line_b),
        "--verbose must not collapse, got:\n{verbose}"
    );
    Ok(())
}

// ── Part A: lint ─────────────────────────────────────────────────────────────

#[test]
fn lint_collapses_deferred_by_default() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("lint-collapse")?;
    git_init(&root)?;
    write_spec_project(&root, DEFERRED_ROW)?;
    git_commit(&root, "init")?;

    let stdout = run_stdout(&root, &["lint"])?;
    assert!(
        stdout.contains("deferred finding"),
        "expected collapsed summary line, got:\n{stdout}"
    );
    assert!(
        !stdout.contains(DEFERRED_SUFFIX),
        "deferred finding must NOT render full text by default, got:\n{stdout}"
    );
    Ok(())
}

#[test]
fn lint_verbose_renders_full() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("lint-verbose")?;
    git_init(&root)?;
    write_spec_project(&root, DEFERRED_ROW)?;
    git_commit(&root, "init")?;

    let stdout = run_stdout(&root, &["--verbose", "lint"])?;
    assert!(
        stdout.contains(DEFERRED_SUFFIX),
        "expected full deferred text with --verbose, got:\n{stdout}"
    );
    Ok(())
}

#[test]
fn lint_json_identical_with_or_without_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("lint-json-verbose")?;
    git_init(&root)?;
    write_spec_project(&root, DEFERRED_ROW)?;
    git_commit(&root, "init")?;

    let a = run(&root, &["--json", "lint"]);
    let b = run(&root, &["--json", "--verbose", "lint"]);
    assert!(a.status.success() && b.status.success());
    assert_eq!(
        a.stdout, b.stdout,
        "JSON wire format must not change with --verbose"
    );
    Ok(())
}

// ── Part A: hook tension (includes Info findings) ─────────────────────────────

#[test]
fn hook_tension_collapses_deferred_by_default() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("hook-collapse")?;
    git_init(&root)?;
    write_spec_project(&root, DEFERRED_ROW)?;
    git_commit(&root, "init")?;

    let stdout = run_stdout(&root, &["hook", "tension"])?;
    assert!(
        stdout.contains("deferred finding"),
        "expected collapsed summary line, got:\n{stdout}"
    );
    assert!(
        !stdout.contains(DEFERRED_SUFFIX),
        "deferred finding must NOT render full text by default, got:\n{stdout}"
    );
    Ok(())
}

#[test]
fn hook_tension_verbose_renders_full() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("hook-verbose")?;
    git_init(&root)?;
    write_spec_project(&root, DEFERRED_ROW)?;
    git_commit(&root, "init")?;

    let stdout = run_stdout(&root, &["--verbose", "hook", "tension"])?;
    assert!(
        stdout.contains(DEFERRED_SUFFIX),
        "expected full deferred text with --verbose, got:\n{stdout}"
    );
    Ok(())
}

#[test]
fn hook_json_identical_with_or_without_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("hook-json-verbose")?;
    git_init(&root)?;
    write_spec_project(&root, DEFERRED_ROW)?;
    git_commit(&root, "init")?;

    let a = run(&root, &["--json", "hook", "tension"]);
    let b = run(&root, &["--json", "--verbose", "hook", "tension"]);
    assert!(a.status.success() && b.status.success());
    assert_eq!(
        a.stdout, b.stdout,
        "JSON wire format must not change with --verbose"
    );
    Ok(())
}

// ── Part A: resurface (finding still fires, deferral no longer applies) ────────

#[test]
fn resurface_when_deferral_cleared() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("resurface-clear")?;
    git_init(&root)?;
    write_spec_project(&root, DEFERRED_ROW)?;
    git_commit(&root, "init")?;

    // Clear the Deferred-by cell: finding still fires as Info, no longer deferred.
    write_spec_project(&root, CLEARED_ROW)?;

    let stdout = run_stdout(&root, &["scan"])?;
    assert!(
        stdout.contains("is pending but names no enforcer"),
        "full Info message must be present after deferral cleared, got:\n{stdout}"
    );
    assert!(
        !stdout.contains("deferred by"),
        "must not be collapsed/deferred after clearing, got:\n{stdout}"
    );
    Ok(())
}

#[test]
fn resurface_when_rule_enforced_without_emitter() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("resurface-enforced")?;
    git_init(&root)?;
    // Promote pending->enforced while the emitter is still missing.
    // Deferral cell may remain but must be ignored: finding fires as Warning.
    write_spec_project(&root, ENFORCED_ROW)?;
    git_commit(&root, "init")?;

    let stdout = run_stdout(&root, &["scan"])?;
    assert!(
        stdout.contains("is enforced but names no enforcer"),
        "full Warning message must be present when enforced without emitter, got:\n{stdout}"
    );
    assert!(
        !stdout.contains("deferred by"),
        "must not be collapsed/deferred when enforced, got:\n{stdout}"
    );
    Ok(())
}
