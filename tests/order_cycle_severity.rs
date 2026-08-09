//! CLI acceptance tests for provenance-aware dependency cycle severity.
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_repo(name: &str) -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock is after epoch")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("cairn-order-cycle-{name}-{suffix}"));
    fs::create_dir_all(&root).expect("temporary repository directory");
    root
}

fn write_project(root: &Path, blueprint: &str) {
    fs::write(root.join("cairn.blueprint"), blueprint).expect("blueprint");
    fs::write(
        root.join("cairn.config.yaml"),
        "ignore:\n  - target\ncontext: \"\"\nrules: {}\n",
    )
    .expect("configuration");
}

fn scan(root: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(root)
        .args(["scan", "--json"])
        .output()
        .expect("scan command")
}

#[test]
fn scan_reports_mixed_component_severity_and_exits_nonzero() {
    let root = temp_repo("mixed");
    write_project(
        &root,
        r#"System App "app" id "app" {
    Module A "a" id "a" {}
    Module B "b" id "b" {}
    Module C "c" id "c" {}
    Module D "d" id "d" {}
}
a -> b "observed" @inferred
b -> a "observed" @inferred
c -> d "declared"
d -> c "declared"
"#,
    );
    let output = scan(&root);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !output.status.success(),
        "a hand-declared cycle must block: {stdout}"
    );
    assert_eq!(
        stdout.matches("\"code\":\"CAIRN_ORDER_CYCLE\"").count(),
        2,
        "both cyclic SCCs must be reported: {stdout}"
    );
    assert!(stdout.contains("\"severity\":\"info\""), "{stdout}");
    assert!(stdout.contains("\"severity\":\"error\""), "{stdout}");
    assert!(stdout.contains("dependency cycle: a -> b -> a"), "{stdout}");
    assert!(stdout.contains("dependency cycle: c -> d -> c"), "{stdout}");
}

#[test]
fn scan_reports_inferred_cycle_and_hand_containment_contradiction() {
    let root = temp_repo("containment");
    write_project(
        &root,
        r#"System App "app" id "app" {
    Container Ancestor "ancestor" id "ancestor" {
        Module Child "child" id "child" {}
    }
    Module CycleA "cycle-a" id "cycle-a" {}
    Module CycleB "cycle-b" id "cycle-b" {}
}
cycle-a -> cycle-b "observed" @inferred
cycle-b -> cycle-a "observed" @inferred
child -> ancestor "declared"
"#,
    );
    let output = scan(&root);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !output.status.success(),
        "the containment contradiction must block: {stdout}"
    );
    assert_eq!(
        stdout.matches("\"code\":\"CAIRN_ORDER_CYCLE\"").count(),
        2,
        "cycle and contradiction must both be reported: {stdout}"
    );
    assert!(
        stdout.contains("dependency cycle: cycle-a -> cycle-b -> cycle-a"),
        "{stdout}"
    );
    assert!(
        stdout.contains("containment and dependency constraints are cyclic among: ancestor, child"),
        "{stdout}"
    );
    assert!(stdout.contains("\"severity\":\"info\""), "{stdout}");
    assert!(stdout.contains("\"severity\":\"error\""), "{stdout}");
}
