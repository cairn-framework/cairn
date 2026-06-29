//! Regression guard for cairn-9ey: the dogfood gate must run the working
//! tree's freshly-built cairn, never a PATH-installed (and possibly stale)
//! binary. With a stale `~/.cargo/bin/cairn`, the pre-push gate can false-green
//! by linting with an old binary that lacks the working tree's newer checks.

use std::path::Path;

fn dogfood_script() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts/dogfood.sh");
    std::fs::read_to_string(path).expect("scripts/dogfood.sh should exist")
}

#[test]
fn dogfood_builds_and_runs_the_working_tree_binary() {
    let script = dogfood_script();
    assert!(
        script.contains("cargo run") && script.contains("--bin cairn"),
        "dogfood.sh must build and run the working-tree cairn (cargo run --bin cairn), \
         not a stale installed binary (cairn-9ey)"
    );
}

#[test]
fn dogfood_never_invokes_path_cairn() {
    // A line invoking `cairn` directly (at any indentation) resolves via PATH
    // to a possibly-stale binary. The gate must reach cairn only through the
    // cargo-built path.
    let script = dogfood_script();
    for line in script.lines() {
        assert!(
            !line.trim_start().starts_with("cairn "),
            "dogfood.sh must not invoke bare `cairn` (PATH can resolve a stale binary); found: {line:?}"
        );
    }
}
