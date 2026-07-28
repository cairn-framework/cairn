//! End-to-end cover for `CAIRN_DECISION_ACCUMULATION` (CA039).
//!
//! Three surfaces have to agree: the configured threshold has to reach the
//! scanner, the finding it raises stays advisory (Info), and `remediate`
//! must hand back the consolidation plan rather than its catch-all
//! "good shape" arm.

use std::fs;
use std::path::Path;
use std::process::Command;

use tempfile::TempDir;

const BLUEPRINT: &str = r#"System App "app" id "app" {
    decisions "./meta/decisions"
    Module Lib "lib" id "app.lib" {
        path "./src"
    }
}"#;

fn write_project(
    decisions: usize,
    config: Option<&str>,
) -> Result<TempDir, Box<dyn std::error::Error>> {
    let dir = TempDir::new()?;
    let root = dir.path();
    fs::create_dir_all(root.join("meta/decisions"))?;
    fs::create_dir_all(root.join("src"))?;
    fs::write(root.join("src/lib.rs"), "pub fn go() {}\n")?;
    fs::write(root.join("cairn.blueprint"), BLUEPRINT)?;
    if let Some(config) = config {
        fs::write(root.join("cairn.config.yaml"), config)?;
    }
    for i in 0..decisions {
        fs::write(
            root.join("meta/decisions").join(format!("rule-{i}.md")),
            format!(
                "---\nid: dec.rule-{i}\nnodes: [app.lib]\nstatus: accepted\ndate: 2026-07-28\n---\n# Rule {i}\n"
            ),
        )?;
    }
    Ok(dir)
}

/// Runs the CLI and returns stdout. A nonzero exit is an error, so a negative
/// assertion can never pass merely because the command fell over; stderr is
/// kept for the diagnostic and out of the JSON.
fn run(root: &Path, args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let out = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(root)
        .args(args)
        .output()?;
    if !out.status.success() {
        return Err(format!(
            "cairn {args:?} exited {:?}: {}",
            out.status.code(),
            String::from_utf8_lossy(&out.stderr)
        )
        .into());
    }
    Ok(String::from_utf8(out.stdout)?)
}

fn accumulation_finding(
    lint: &str,
) -> Result<Option<serde_json::Value>, Box<dyn std::error::Error>> {
    let parsed: serde_json::Value = serde_json::from_str(lint)?;
    let findings = parsed["findings"]
        .as_array()
        .ok_or("lint --json must carry a findings array")?;
    Ok(findings
        .iter()
        .find(|f| f["code"] == "CAIRN_DECISION_ACCUMULATION")
        .cloned())
}

#[test]
fn configured_threshold_flags_the_node_and_yields_a_consolidation_plan()
-> Result<(), Box<dyn std::error::Error>> {
    let dir = write_project(3, Some("decision_accumulation_threshold: 2\n"))?;

    let lint = run(dir.path(), &["lint", "--json"])?;
    assert!(
        !lint.contains("CAIRN_CONFIG_UNKNOWN_KEY"),
        "the threshold key must be recognised by the config parser; got: {lint}"
    );
    let finding = accumulation_finding(&lint)?
        .ok_or_else(|| format!("3 accepted decisions over a threshold of 2 must flag: {lint}"))?;
    assert_eq!(
        finding["severity"], "info",
        "accumulation is advisory, never a blocking severity: {finding}"
    );
    assert_eq!(finding["node"], "app.lib");

    let plan = run(dir.path(), &["remediate"])?;
    assert!(
        plan.contains("consolidate_decisions"),
        "remediate must plan the consolidation; got: {plan}"
    );
    assert!(
        plan.contains("app.lib"),
        "the plan must name the accumulating node; got: {plan}"
    );

    Ok(())
}

#[test]
fn default_threshold_is_ten() -> Result<(), Box<dyn std::error::Error>> {
    // No config file: both cases exercise the compiled-in fallback, so raising
    // or disabling it fails here rather than silently widening the check.
    let at = write_project(10, None)?;
    assert!(
        accumulation_finding(&run(at.path(), &["lint", "--json"])?)?.is_none(),
        "10 accepted decisions is at the default threshold, not over it"
    );

    let over = write_project(11, None)?;
    let finding = accumulation_finding(&run(over.path(), &["lint", "--json"])?)?
        .ok_or("11 accepted decisions must exceed the default threshold of 10")?;
    assert!(
        finding["message"]
            .as_str()
            .is_some_and(|m| m.contains("11") && m.contains("threshold 10")),
        "the message must report the count and the default threshold: {finding}"
    );

    let plan = run(over.path(), &["remediate"])?;
    assert!(
        plan.contains("consolidate_decisions"),
        "the default-threshold breach must reach remediate too; got: {plan}"
    );

    Ok(())
}
