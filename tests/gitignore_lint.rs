//! Integration tests for `CAIRN_PATH_GITIGNORED` lint finding (issue #45 option 3).
//!
//! When a blueprint declares a `path` that matches a `.gitignore` pattern,
//! the scanner emits a Warning finding so authors catch the mistake before
//! the path silently becomes a Ghost node.

use std::fs;
use std::path::PathBuf;
use std::process::Command;
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
fn test_gitignored_path_surfaces_remediation_action() -> Result<(), Box<dyn std::error::Error>> {
    // A gitignored declared path is a Warning that `cairn lint` reports. The
    // remediation driver must map it to a concrete, node-specific action rather
    // than silently reporting "good shape".
    let root = temp_root("gitignored-remediate")?;
    fs::create_dir_all(root.join("dist"))?;
    fs::write(root.join("dist/main.rs"), "")?;
    fs::write(root.join(".gitignore"), "dist\n")?;
    fs::write(root.join("cairn.blueprint"), minimal_blueprint("./dist"))?;

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["remediate", "--json"])
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let parsed: serde_json::Value = serde_json::from_str(&stdout)?;
    let actions = parsed["actions"]
        .as_array()
        .ok_or("remediate output missing actions array")?;

    let gitignore_action = actions
        .iter()
        .find(|a| a["action"] == "fix_gitignored_path")
        .ok_or_else(|| format!("expected a fix_gitignored_path action, got: {stdout}"))?;
    let nodes: Vec<&str> = gitignore_action["nodes"]
        .as_array()
        .map(|arr| arr.iter().filter_map(serde_json::Value::as_str).collect())
        .unwrap_or_default();
    assert!(
        nodes.contains(&"app.lib"),
        "action should name the affected node app.lib, got: {nodes:?}"
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
