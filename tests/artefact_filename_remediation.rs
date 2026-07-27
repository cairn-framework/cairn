//! End-to-end cover for `CAIRN_ARTEFACT_FILENAME_DRIFT` (CA038).
//!
//! The finding is only useful if the two surfaces agree: `cairn scan --strict`
//! must fail on a drifted filename, and `cairn remediate` must hand back a plan
//! for it. An unrouted finding code reaches `remediate`'s catch-all arm and
//! reports "the project is in good shape" while strict scan is failing, which
//! is the exact contradiction this test exists to prevent.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const BLUEPRINT: &str = r#"System App "app" id "app" {
    decisions "./meta/decisions"
    Module Lib "lib" id "app.lib" {
        path "./src"
    }
}"#;

const DECISION: &str = "---\nid: dec.only-rule\nnodes: [app.lib]\nstatus: accepted\ndate: 2026-07-27\n---\n# Only Rule\n";

fn write_project(filename: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("cairn-ca038-{suffix}"));
    fs::create_dir_all(root.join("meta/decisions"))?;
    fs::create_dir_all(root.join("src"))?;
    fs::write(root.join("src/lib.rs"), "pub fn go() {}\n")?;
    fs::write(root.join("cairn.blueprint"), BLUEPRINT)?;
    fs::write(root.join("meta/decisions").join(filename), DECISION)?;
    Ok(root)
}

fn run(root: &Path, args: &[&str]) -> Result<(bool, String), Box<dyn std::error::Error>> {
    let out = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(root)
        .args(args)
        .output()?;
    let mut text = String::from_utf8(out.stdout)?;
    text.push_str(&String::from_utf8(out.stderr)?);
    Ok((out.status.success(), text))
}

#[test]
fn drifted_filename_fails_strict_scan_and_yields_a_rename_plan()
-> Result<(), Box<dyn std::error::Error>> {
    // Filename carries the typed prefix the convention forbids.
    let root = write_project("dec.only-rule.md")?;

    let (_, lint) = run(&root, &["lint", "--json"])?;
    assert!(
        lint.contains("CAIRN_ARTEFACT_FILENAME_DRIFT"),
        "prefixed filename must drift; got: {lint}"
    );
    assert!(
        lint.contains("only-rule.md"),
        "finding must name the expected filename; got: {lint}"
    );

    let (strict_ok, _) = run(&root, &["scan", "--strict"])?;
    assert!(!strict_ok, "a Warning finding must fail --strict");

    // `remediate --json` projects to the shared work-item shape, which keeps
    // the title and command but not the action name; assert on the CLI render.
    let (_, plan) = run(&root, &["remediate"])?;
    assert!(
        plan.contains("rename_artefacts"),
        "remediate must plan the rename rather than report no actions; got: {plan}"
    );
    assert!(
        !plan.contains("good shape"),
        "the `none` action must not be returned while drift exists; got: {plan}"
    );

    fs::remove_dir_all(&root)?;
    Ok(())
}

#[test]
fn conforming_filename_is_clean_and_needs_no_remediation() -> Result<(), Box<dyn std::error::Error>>
{
    let root = write_project("only-rule.md")?;

    let (_, lint) = run(&root, &["lint", "--json"])?;
    assert!(
        !lint.contains("CAIRN_ARTEFACT_FILENAME_DRIFT"),
        "slug-only filename must be clean; got: {lint}"
    );

    let (_, plan) = run(&root, &["remediate"])?;
    assert!(
        !plan.contains("rename_artefacts"),
        "no drift means no rename action; got: {plan}"
    );

    fs::remove_dir_all(&root)?;
    Ok(())
}
