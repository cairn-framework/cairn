//! End-to-end cover for `CAIRN_DECISION_ACCUMULATION` (CA039).
//!
//! Three surfaces have to agree: the configured threshold has to reach the
//! scanner, the finding it raises stays advisory (Info), and `remediate`
//! must hand back the consolidation plan rather than its catch-all
//! "good shape" arm.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const BLUEPRINT: &str = r#"System App "app" id "app" {
    decisions "./meta/decisions"
    Module Lib "lib" id "app.lib" {
        path "./src"
    }
}"#;

fn write_project(
    decisions: usize,
    config: Option<&str>,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Tests in one binary run concurrently and `SystemTime::now()` is not
    // guaranteed to differ between them, so the counter, not the clock, is
    // what keeps two roots apart.
    static SEQ: AtomicUsize = AtomicUsize::new(0);
    let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let seq = SEQ.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!("cairn-ca039-{suffix}-{seq}"));
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
    Ok(root)
}

fn run(root: &Path, args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let out = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(root)
        .args(args)
        .output()?;
    let mut text = String::from_utf8(out.stdout)?;
    text.push_str(&String::from_utf8(out.stderr)?);
    Ok(text)
}

#[test]
fn configured_threshold_flags_the_node_and_yields_a_consolidation_plan()
-> Result<(), Box<dyn std::error::Error>> {
    let root = write_project(3, Some("decision_accumulation_threshold: 2\n"))?;

    let lint = run(&root, &["lint", "--json"])?;
    assert!(
        !lint.contains("CAIRN_CONFIG_UNKNOWN_KEY"),
        "the threshold key must be recognised by the config parser; got: {lint}"
    );
    let parsed: serde_json::Value = serde_json::from_str(&lint)?;
    let finding = parsed["findings"]
        .as_array()
        .and_then(|findings| {
            findings
                .iter()
                .find(|f| f["code"] == "CAIRN_DECISION_ACCUMULATION")
        })
        .unwrap_or_else(|| panic!("3 accepted decisions over a threshold of 2 must flag: {lint}"));
    assert_eq!(
        finding["severity"], "info",
        "accumulation is advisory, never a blocking severity: {finding}"
    );
    assert_eq!(finding["node"], "app.lib");

    let plan = run(&root, &["remediate"])?;
    assert!(
        plan.contains("consolidate_decisions"),
        "remediate must plan the consolidation; got: {plan}"
    );
    assert!(
        plan.contains("app.lib"),
        "the plan must name the accumulating node; got: {plan}"
    );

    fs::remove_dir_all(&root)?;
    Ok(())
}

#[test]
fn default_threshold_leaves_a_small_decision_set_clean() -> Result<(), Box<dyn std::error::Error>> {
    let root = write_project(3, None)?;

    // Parsing first: a negative substring assertion also passes on an error
    // page or empty output, so the shape has to be proven before it counts.
    let lint = run(&root, &["lint", "--json"])?;
    let parsed: serde_json::Value = serde_json::from_str(&lint)?;
    let codes = parsed["findings"]
        .as_array()
        .ok_or("lint --json must carry a findings array")?;
    assert!(
        !codes
            .iter()
            .any(|f| f["code"] == "CAIRN_DECISION_ACCUMULATION"),
        "3 decisions is under the default threshold of 10; got: {lint}"
    );

    let plan = run(&root, &["remediate"])?;
    assert!(
        !plan.contains("consolidate_decisions"),
        "no accumulation means no consolidation action; got: {plan}"
    );

    fs::remove_dir_all(&root)?;
    Ok(())
}
