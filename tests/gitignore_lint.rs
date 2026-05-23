//! Integration tests for `CAIRN_PATH_GITIGNORED` lint finding (issue #45 option 3).
//!
//! When a blueprint declares a `path` that matches a `.gitignore` pattern,
//! the scanner emits a Warning finding so authors catch the mistake before
//! the path silently becomes a Ghost node.

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_root(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("cairn-{name}-{suffix}"));
    fs::create_dir_all(&root)?;
    Ok(root)
}

fn minimal_blueprint(path: &str) -> String {
    format!(
        r#"System App "app" id "app" {{
    Module Lib "lib" id "app.lib" {{
        path "{path}"
    }}
}}"#
    )
}

#[test]
fn test_gitignored_path_emits_warning() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("gitignored-warn")?;
    // Declare a path that is present on disk but gitignored.
    fs::create_dir_all(root.join("dist"))?;
    fs::write(root.join("dist/main.rs"), "")?;
    fs::write(root.join(".gitignore"), "dist\n")?;
    fs::write(root.join("cairn.blueprint"), minimal_blueprint("./dist"))?;

    let result = cairn::scanner::scan(&root, &root.join("cairn.blueprint"))?;

    let findings: Vec<_> = result
        .graph
        .findings
        .iter()
        .filter(|f| f.code == "CAIRN_PATH_GITIGNORED")
        .collect();
    assert_eq!(
        findings.len(),
        1,
        "expected one gitignored finding, got: {findings:?}"
    );
    assert_eq!(findings[0].severity, cairn::map::FindingSeverity::Warning);
    assert!(findings[0].message.contains("dist"));
    Ok(())
}

#[test]
fn test_non_gitignored_path_no_warning() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("gitignored-no-warn")?;
    fs::create_dir_all(root.join("src/lib"))?;
    fs::write(root.join("src/lib/mod.rs"), "")?;
    fs::write(root.join(".gitignore"), "dist\nbuild\n")?;
    fs::write(root.join("cairn.blueprint"), minimal_blueprint("./src/lib"))?;

    let result = cairn::scanner::scan(&root, &root.join("cairn.blueprint"))?;

    let findings: Vec<_> = result
        .graph
        .findings
        .iter()
        .filter(|f| f.code == "CAIRN_PATH_GITIGNORED")
        .collect();
    assert!(
        findings.is_empty(),
        "non-gitignored path should not emit warning"
    );
    Ok(())
}

#[test]
fn test_no_gitignore_no_warning() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("gitignored-no-file")?;
    fs::create_dir_all(root.join("src"))?;
    fs::write(root.join("src/lib.rs"), "")?;
    // No .gitignore file at all.
    fs::write(root.join("cairn.blueprint"), minimal_blueprint("./src"))?;

    let result = cairn::scanner::scan(&root, &root.join("cairn.blueprint"))?;

    let findings: Vec<_> = result
        .graph
        .findings
        .iter()
        .filter(|f| f.code == "CAIRN_PATH_GITIGNORED")
        .collect();
    assert!(
        findings.is_empty(),
        "absent .gitignore should not emit warnings"
    );
    Ok(())
}

#[test]
fn test_wildcard_gitignore_pattern_matches() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("gitignored-wildcard")?;
    fs::create_dir_all(root.join("target/debug"))?;
    fs::write(root.join("target/debug/cairn"), "")?;
    fs::write(root.join(".gitignore"), "target\n*.lock\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        minimal_blueprint("./target/debug"),
    )?;

    let result = cairn::scanner::scan(&root, &root.join("cairn.blueprint"))?;

    let findings: Vec<_> = result
        .graph
        .findings
        .iter()
        .filter(|f| f.code == "CAIRN_PATH_GITIGNORED")
        .collect();
    assert_eq!(
        findings.len(),
        1,
        "wildcard-matched path should emit warning"
    );
    Ok(())
}
