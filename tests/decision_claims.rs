//! Integration tests for decision artefact `claims:` frontmatter drift
//! detection (issue #67).

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_root(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("cairn-{name}-{suffix}"));
    fs::create_dir_all(&root)?;
    Ok(root)
}

fn write_project(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(root.join("src/reconcile"))?;
    fs::write(root.join("src/reconcile/mod.rs"), "")?;
    fs::write(root.join("src/reconcile/code.rs"), "")?;
    fs::write(root.join("src/reconcile/go.rs"), "")?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "app" id "app" {
    Module Reconcile "reconcile" id "app.reconcile" {
        path "./src/reconcile"
        decisions "./meta/decisions"
    }
}"#,
    )?;
    fs::create_dir_all(root.join("meta/decisions"))?;
    Ok(())
}

fn write_decision(root: &Path, name: &str, claims: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = root.join(format!("meta/decisions/{name}.md"));
    let content = format!(
        "---\nid: {name}\nnode: app.reconcile\nstatus: accepted\ndate: 2026-05-22\n{claims}---\n\n# Decision\n"
    );
    fs::write(&path, content)?;
    Ok(())
}

#[test]
fn test_exhaustive_claims_match_emits_no_finding() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("claims-match")?;
    write_project(&root)?;
    write_decision(
        &root,
        "mapping",
        "claims_folder: ./src/reconcile\nclaims_mode: exhaustive\nclaims_items:\n  - mod.rs\n  - code.rs\n  - go.rs\n",
    )?;

    let result = cairn::scanner::scan(&root, &root.join("cairn.blueprint"))?;

    let findings: Vec<_> = result
        .graph
        .findings
        .iter()
        .filter(|f| f.code == "CA003")
        .collect();
    assert!(findings.is_empty(), "matching claims should not emit CA003");
    Ok(())
}

#[test]
fn test_exhaustive_claims_missing_file_emits_finding() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("claims-missing")?;
    write_project(&root)?;
    // omits go.rs from the claim list
    write_decision(
        &root,
        "mapping",
        "claims_folder: ./src/reconcile\nclaims_mode: exhaustive\nclaims_items:\n  - mod.rs\n  - code.rs\n",
    )?;

    let result = cairn::scanner::scan(&root, &root.join("cairn.blueprint"))?;

    let findings: Vec<_> = result
        .graph
        .findings
        .iter()
        .filter(|f| f.code == "CA003")
        .collect();
    assert_eq!(findings.len(), 1);
    assert!(findings[0].message.contains("go.rs"));
    Ok(())
}

#[test]
fn test_exhaustive_claims_extra_file_emits_finding() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("claims-extra")?;
    write_project(&root)?;
    // claims a file that does not exist
    write_decision(
        &root,
        "mapping",
        "claims_folder: ./src/reconcile\nclaims_mode: exhaustive\nclaims_items:\n  - mod.rs\n  - code.rs\n  - go.rs\n  - extra.rs\n",
    )?;

    let result = cairn::scanner::scan(&root, &root.join("cairn.blueprint"))?;

    let findings: Vec<_> = result
        .graph
        .findings
        .iter()
        .filter(|f| f.code == "CA003")
        .collect();
    assert_eq!(findings.len(), 1);
    assert!(findings[0].message.contains("extra.rs"));
    Ok(())
}

#[test]
fn test_illustrative_claims_never_emit_finding() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("claims-illustrative")?;
    write_project(&root)?;
    write_decision(
        &root,
        "mapping",
        "claims_folder: ./src/reconcile\nclaims_mode: illustrative\nclaims_items:\n  - mod.rs\n",
    )?;

    let result = cairn::scanner::scan(&root, &root.join("cairn.blueprint"))?;

    let findings: Vec<_> = result
        .graph
        .findings
        .iter()
        .filter(|f| f.code == "CA003")
        .collect();
    assert!(
        findings.is_empty(),
        "illustrative claims should not emit findings"
    );
    Ok(())
}

#[test]
fn test_no_claims_no_finding() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("claims-none")?;
    write_project(&root)?;
    write_decision(&root, "mapping", "")?;

    let result = cairn::scanner::scan(&root, &root.join("cairn.blueprint"))?;

    let findings: Vec<_> = result
        .graph
        .findings
        .iter()
        .filter(|f| f.code == "CA003")
        .collect();
    assert!(
        findings.is_empty(),
        "absent claims should not emit findings"
    );
    Ok(())
}
