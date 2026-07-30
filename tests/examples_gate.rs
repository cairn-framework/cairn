//! Anti-rot gate for the maintained example corpus.
//!
//! Scans a fresh copy of each corpus. `examples/demo` is compared against the
//! empty baseline committed inside it: it is what a healthy project looks
//! like. `tests/fixtures/cairn-bootstrap` was repaired to a clean scan under
//! `todo.bootstrap-fixture-repair-or-delete` and is asserted clean directly,
//! with no baseline file to drift.
//!
//! The demo baseline sits inside the project it pins, so it is present during
//! the scan it describes. It is plain JSON at the corpus root, which no
//! reconciler claims and no artefact pointer reaches, so it does not perturb
//! its own result.

use std::{fs, path::Path};

use tempfile::TempDir;

/// Baseline filename, relative to a corpus root.
const BASELINE: &str = "expected-findings.json";

/// Directories holding generated state rather than fixture content. Copying them
/// would let a stale reconciler cache decide the finding set.
const GENERATED: [&str; 2] = [".cairn", "target"];

#[test]
fn test_demo_example_scans_clean() -> Result<(), Box<dyn std::error::Error>> {
    let findings = assert_scan_matches_baseline("examples/demo")?;
    assert!(
        findings.is_empty(),
        "the demo example is the healthy-project surface and must scan clean, got: {findings:#?}"
    );
    Ok(())
}

#[test]
fn test_bootstrap_fixture_scans_clean() -> Result<(), Box<dyn std::error::Error>> {
    let findings = scan_fixture_copy("tests/fixtures/cairn-bootstrap")?;
    assert!(
        findings.is_empty(),
        "the bootstrap fixture was repaired to a clean scan under \
         todo.bootstrap-fixture-repair-or-delete; a finding here is fixture rot: {findings:#?}"
    );
    Ok(())
}

/// Scan a throwaway copy of `fixture` and return the findings, folded back to
/// fixture-relative paths and canonically ordered.
///
/// The copy lives in a `TempDir` held for the whole scan: `Drop` removes it and
/// the state the scan writes into it on the success, error, and
/// assertion-failure paths alike.
fn scan_fixture_copy(fixture: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join(fixture);
    let copy = TempDir::new()?;
    copy_tree(&source, copy.path())?;

    let blueprint = copy.path().join("cairn.blueprint");
    let result = cairn::cli::run(&[
        "--file".to_owned(),
        blueprint.to_string_lossy().to_string(),
        "scan".to_owned(),
        "--json".to_owned(),
    ]);
    let response: serde_json::Value = serde_json::from_str(result.stdout.trim()).map_err(|e| {
        format!(
            "scan of {fixture} did not emit JSON ({e}); stdout: {} stderr: {}",
            result.stdout, result.stderr
        )
    })?;
    let findings = response
        .get("findings")
        .ok_or("scan response has no findings array")?;

    // A finding may carry an absolute path. The copy lives under a temporary
    // directory with a generated name, so that prefix is folded back to the
    // fixture it came from or a baseline could never match twice.
    let mut observed: Vec<serde_json::Value> = serde_json::from_str(
        &serde_json::to_string(findings)?
            .replace(&copy.path().to_string_lossy().to_string(), fixture),
    )?;
    // Emission order is a traversal detail, so the set is ordered canonically
    // rather than asserting the scanner's.
    observed.sort_by_key(ToString::to_string);

    assert_eq!(
        result.code, 0,
        "plain scan of {fixture} must stay green, stderr: {}",
        result.stderr
    );

    Ok(observed)
}

/// Scan a throwaway copy of `fixture` and assert its findings equal the
/// baseline committed inside it.
///
/// Returns the observed findings so a caller can assert the shape of the set on
/// top of its exact contents.
fn assert_scan_matches_baseline(
    fixture: &str,
) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let observed = scan_fixture_copy(fixture)?;
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join(fixture);
    let mut expected: Vec<serde_json::Value> =
        serde_json::from_str(&fs::read_to_string(source.join(BASELINE))?)?;
    expected.sort_by_key(ToString::to_string);

    assert_eq!(
        observed,
        expected,
        "scan of {fixture} drifted from its {BASELINE}. Fix the fixture, or, when the \
         change is intended, rewrite the baseline:\n  cairn --file {fixture}/cairn.blueprint \
         scan --json | jq '.findings' > {fixture}/{BASELINE}\nobserved: {}",
        serde_json::to_string_pretty(&observed)?
    );

    Ok(observed)
}

/// Copy fixture content, skipping generated state.
fn copy_tree(from: &Path, to: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let name = entry.file_name();
        if GENERATED.iter().any(|skip| *skip == name) {
            continue;
        }
        let target = to.join(&name);
        if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}
