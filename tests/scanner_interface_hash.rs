//! Scanner-level regression test for per-node interface hashes.
//!
//! The reconciler-level tests verify that symbols are partitioned per node, but
//! the degenerate-hash bug lived in `scanner::mod.rs::reconcile_targets` (and
//! the cache reconstruction path), where the per-language global fingerprint
//! was stamped onto every target. This test drives the public `scanner::scan`
//! entry point so a regression in either the scanner aggregation or the cache
//! path fails the suite.

use std::fs;

use cairn::scanner;
use tempfile::tempdir;

#[test]
fn test_scan_produces_distinct_interface_hashes_for_distinct_nodes() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir(root.join("src")).unwrap();
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "app" id "app" {
    Module Alpha "alpha" id "app.alpha" {
        path "./src/alpha.rs"
        owns-files: true
    }
    Module Beta "beta" id "app.beta" {
        path "./src/beta.rs"
        owns-files: true
    }
}
"#,
    )
    .unwrap();
    fs::write(
        root.join("src/alpha.rs"),
        "pub fn alpha() {}\npub struct Alpha;\n",
    )
    .unwrap();
    fs::write(
        root.join("src/beta.rs"),
        "pub fn beta() {}\npub enum Beta {}\n",
    )
    .unwrap();

    let result = scanner::scan(root, root.join("cairn.blueprint").as_path()).unwrap();

    let alpha_key = "app.alpha:src/alpha.rs";
    let beta_key = "app.beta:src/beta.rs";

    let alpha_hash = result
        .target_hashes
        .get(alpha_key)
        .unwrap_or_else(|| panic!("missing target hash for {alpha_key}"));
    let beta_hash = result
        .target_hashes
        .get(beta_key)
        .unwrap_or_else(|| panic!("missing target hash for {beta_key}"));

    assert_ne!(
        alpha_hash, beta_hash,
        "distinct nodes with distinct public symbols must have distinct interface hashes"
    );
}
