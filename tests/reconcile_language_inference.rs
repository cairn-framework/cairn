//! Integration tests for directory language inference findings.

use std::fs;
use std::path::PathBuf;

fn temp_root(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let dir = std::env::temp_dir().join(format!(
        "cairn-reconcile-lang-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// A directory target with only unsupported files produces a
/// `CAIRN_RECONCILE_LANGUAGE_UNKNOWN` warning naming the path.
#[test]
fn unknown_language_emits_warning_finding() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("unknown-finding")?;
    fs::create_dir_all(root.join("lib"))?;
    fs::write(root.join("lib/readme.txt"), "not source code\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module Lib "lib" id "app.lib" {
        path "./lib"
    }
}
"#,
    )?;

    let result = cairn::scanner::scan(&root, &root.join("cairn.blueprint"))?;
    let findings: Vec<_> = result
        .graph
        .findings
        .iter()
        .filter(|f| f.code == "CAIRN_RECONCILE_LANGUAGE_UNKNOWN")
        .collect();
    assert_eq!(
        findings.len(),
        1,
        "expected one LANGUAGE_UNKNOWN finding, got: {:?}",
        result.graph.findings
    );
    let finding = findings[0];
    assert_eq!(
        finding.severity,
        cairn::map::graph::FindingSeverity::Warning
    );
    assert_eq!(finding.path.as_deref(), Some("lib"));
    assert_eq!(finding.node.as_deref(), Some("app.lib"));

    // No interface hash for the unknown target.
    let report = result
        .target_reports
        .iter()
        .find(|r| r.target_id.node_id == "app.lib")
        .expect("app.lib target report");
    assert!(report.hash.is_none(), "unknown target must not emit a hash");

    let _ = fs::remove_dir_all(&root);
    Ok(())
}
