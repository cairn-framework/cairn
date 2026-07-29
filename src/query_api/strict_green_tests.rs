//! Tests for the published `strict_green` verdict on the lint/scan wire.
//!
//! `dec.loop-selection-strict-green-fold`: the lint/scan `data` payload
//! publishes `strict_green`, `true` exactly when `--strict` would exit zero
//! over the emitted finding set. Kept beside `tests.rs` rather than inside it
//! so the shared execute-test file stays under the module size gate.

use super::*;

/// Runs the `lint` query against a fixture root and returns its `data`
/// payload.
fn lint_data(tmp: &std::path::Path) -> Value {
    let request = QueryRequest {
        tool: "lint".to_owned(),
        ..QueryRequest::default()
    };
    execute(
        tmp,
        &tmp.join("cairn.blueprint"),
        &tmp.join("meta/changes"),
        &request,
    )
    .expect("lint must succeed")
    .data
}

/// Projects the payload's findings to their severity labels.
fn severities(data: &Value) -> Vec<String> {
    data["findings"]
        .as_array()
        .expect("findings array")
        .iter()
        .map(|finding| finding["severity"].as_str().unwrap_or_default().to_owned())
        .collect()
}

#[test]
fn test_execute_lint_publishes_strict_green_true_on_info_only_set() {
    let tmp = std::env::temp_dir().join(format!("cairn-strictgreen-ok-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(tmp.join("meta/sources"));
    let _ = std::fs::create_dir_all(tmp.join("meta/research"));
    // An unverified source (anchored by a citing research artefact so it is
    // not an orphan) is the cheapest deterministic Info finding, the same
    // class as the live standing set this rule folds.
    let _ = std::fs::write(
        tmp.join("cairn.blueprint"),
        "System Test \"T\" id \"t\" {\n    Container Api \"api\" id \"t.api\" {\n        research \"./meta/research\"\n        sources \"./meta/sources\"\n    }\n}\n",
    );
    let _ = std::fs::write(tmp.join("probe.txt"), "probe\n");
    let _ = std::fs::write(
        tmp.join("meta/sources/probe.md"),
        "---\nid: src.probe\nfile: probe.txt\nverification: unverified\ntype: note\ndate: 2026-07-29\n---\n# Probe\n",
    );
    let _ = std::fs::write(
        tmp.join("meta/research/probe.md"),
        "---\nid: res.probe\nnodes: [t.api]\ndate: 2026-07-29\nsources: [src.probe]\n---\n# Probe research\n",
    );
    let data = lint_data(&tmp);
    let severities = severities(&data);
    assert!(
        severities.iter().any(|s| s == "info"),
        "fixture drifted: expected an Info finding so the Info-only case is exercised, got {severities:?}"
    );
    assert!(
        !severities.iter().any(|s| s == "error" || s == "warning"),
        "fixture drifted: expected no blocking findings, got {severities:?}"
    );
    assert_eq!(
        data.get("strict_green").and_then(Value::as_bool),
        Some(true),
        "an Info-only set the strict gate tolerates must publish strict_green true"
    );
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn test_execute_lint_publishes_strict_green_false_on_blocking_finding() {
    // A dependency cycle is an Error, so the strict verdict must go false;
    // Info findings alone never do this (the true-case test above).
    let tmp = std::env::temp_dir().join(format!("cairn-strictgreen-red-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);
    let _ = std::fs::write(
        tmp.join("cairn.blueprint"),
        "System App \"d\" id \"app\" {\n    Module A \"a\" id \"app.a\" {\n    }\n    Module B \"b\" id \"app.b\" {\n    }\n}\napp.a -> app.b \"x\"\napp.b -> app.a \"y\"\n",
    );
    let data = lint_data(&tmp);
    let severities = severities(&data);
    assert!(
        severities.iter().any(|s| s == "error" || s == "warning"),
        "fixture drifted: expected a blocking finding, got {severities:?}"
    );
    assert_eq!(
        data.get("strict_green").and_then(Value::as_bool),
        Some(false),
        "a set the strict gate rejects must publish strict_green false"
    );
    let _ = std::fs::remove_dir_all(&tmp);
}
