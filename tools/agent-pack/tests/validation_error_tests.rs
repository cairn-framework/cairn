//! Manifest collision and lexical-validation failures.

use cairn_agent_pack::{run_check, run_write};
use tempfile::TempDir;

fn validation_error(manifest: &str) -> String {
    let temp = TempDir::new().unwrap();
    let manifest_path = temp.path().join("manifest.toml");
    std::fs::write(&manifest_path, manifest).unwrap();
    run_check(&manifest_path, temp.path())
        .unwrap_err()
        .to_string()
}

#[test]
fn duplicate_destination_is_rejected() {
    let error = validation_error(
        r#"
schema_version = 1
bundle_version = "1.0.0"

[[canonical]]
entry = "entry-1"
mode = "default"
source = "src1.txt"

[[canonical]]
entry = "entry-2"
mode = "default"
source = "src2.txt"

[[adapters]]
harness = "claude"
entry = "entry-1"
mode = "default"
destination = "output/./file.txt"

[[adapters]]
harness = "claude"
entry = "entry-2"
mode = "default"
destination = "output/file.txt"
"#,
    );

    assert!(error.contains("filesystem-equivalent destinations 'output/file.txt'"));
    assert!(error.contains("indexes 0 and 1"));
    assert!(error.contains("change one destination or remove the duplicate row"));
}

#[test]
fn duplicate_harness_entry_mode_is_rejected() {
    let error = validation_error(
        r#"
schema_version = 1
bundle_version = "1.0.0"

[[canonical]]
entry = "entry-1"
mode = "default"
source = "src1.txt"

[[adapters]]
harness = "claude"
entry = "entry-1"
mode = "default"
destination = "dest1.txt"

[[adapters]]
harness = "claude"
entry = "entry-1"
mode = "default"
destination = "dest2.txt"
"#,
    );

    assert!(error.contains("harness 'claude' entry 'entry-1' mode 'default'"));
    assert!(error.contains("indexes 0 and 1"));
    assert!(error.contains("distinct harness entry-mode key"));
}

#[test]
fn lexical_escape_is_rejected_with_the_bad_row_and_path() {
    let error = validation_error(
        r#"
schema_version = 1
bundle_version = "1.0.0"

[[canonical]]
entry = "entry-1"
mode = "default"
source = "src1.txt"

[[adapters]]
harness = "claude"
entry = "entry-1"
mode = "default"
destination = "../escaping/dest.txt"
"#,
    );

    assert!(error.contains("row [[adapters]] index 0"));
    assert!(error.contains("../escaping/dest.txt"));
    assert!(error.contains("parent traversal"));
    assert!(error.contains("non-empty project-relative path"));
}

#[test]
fn duplicate_canonical_entry_mode_is_rejected() {
    let error = validation_error(
        r#"
schema_version = 1
bundle_version = "1.0.0"

[[canonical]]
entry = "entry-1"
mode = "default"
source = "src1.txt"

[[canonical]]
entry = "entry-1"
mode = "default"
source = "src2.txt"
"#,
    );

    assert!(error.contains("canonical owner for entry 'entry-1' mode 'default'"));
    assert!(error.contains("indexes 0 and 1"));
    assert!(error.contains("distinct entry-mode key"));
}

#[test]
fn distinct_harnesses_can_adapt_the_same_opaque_entry_mode() {
    let temp = TempDir::new().unwrap();
    let manifest_path = temp.path().join("manifest.toml");
    std::fs::write(temp.path().join("source.txt"), b"shared bytes").unwrap();
    std::fs::write(
        &manifest_path,
        r#"
schema_version = 1
bundle_version = "2.0.0"

[[canonical]]
entry = "entry-1"
mode = "opaque-future-mode"
source = "source.txt"

[[adapters]]
harness = "claude"
entry = "entry-1"
mode = "opaque-future-mode"
destination = "claude/dest.txt"

[[adapters]]
harness = "omp"
entry = "entry-1"
mode = "opaque-future-mode"
destination = "omp/dest.txt"
"#,
    )
    .unwrap();

    run_write(&manifest_path, temp.path()).unwrap();
    assert_eq!(
        std::fs::read(temp.path().join("claude/dest.txt")).unwrap(),
        b"shared bytes"
    );
    assert_eq!(
        std::fs::read(temp.path().join("omp/dest.txt")).unwrap(),
        b"shared bytes"
    );
}

#[test]
fn windows_path_syntax_is_rejected_on_every_host() {
    let error = validation_error(
        r#"
schema_version = 1
bundle_version = "1.0.0"

[[canonical]]
entry = "entry-1"
mode = "default"
source = 'C:\outside\source.md'
"#,
    );

    assert!(error.contains("row [[canonical]] index 0"));
    assert!(error.contains(r"C:\outside\source.md"));
    assert!(error.contains("forward-slash project-relative syntax"));
}

#[test]
fn destination_hierarchy_collisions_are_rejected_in_either_order() {
    for (first, second) in [("a", "a/b"), ("a/b", "a")] {
        let error = validation_error(&format!(
            r#"
schema_version = 1
bundle_version = "1.0.0"

[[canonical]]
entry = "entry-1"
mode = "default"
source = "src1.txt"

[[canonical]]
entry = "entry-2"
mode = "default"
source = "src2.txt"

[[adapters]]
harness = "claude"
entry = "entry-1"
mode = "default"
destination = "{first}"

[[adapters]]
harness = "claude"
entry = "entry-2"
mode = "default"
destination = "{second}"
"#
        ));

        assert!(error.contains("destination hierarchy collision"));
        assert!(error.contains("indexes 0 and 1"));
        assert!(error.contains("not ancestors of one another"));
    }
}

#[test]
fn unadapted_canonical_sources_are_still_validated() {
    let temp = TempDir::new().unwrap();
    let manifest_path = temp.path().join("manifest.toml");
    std::fs::write(
        &manifest_path,
        r#"
schema_version = 1
bundle_version = "1.0.0"

[[canonical]]
entry = "future-entry"
mode = "default"
source = "missing.txt"
"#,
    )
    .unwrap();

    let error = run_check(&manifest_path, temp.path())
        .unwrap_err()
        .to_string();
    assert!(error.contains("row [[canonical]] index 0"));
    assert!(error.contains("missing.txt"));
    assert!(error.contains("correct the source path"));
}

#[test]
fn unknown_manifest_fields_are_rejected() {
    let top_level = validation_error(
        r#"
schema_version = 1
bundle_version = "1.0.0"
workflow = "forbidden"
"#,
    );
    assert!(top_level.contains("unknown field `workflow`"));
    assert!(top_level.contains("Correct the TOML syntax"));

    let misspelled_table = validation_error(
        r#"
schema_version = 1
bundle_version = "1.0.0"

[[adapter]]
harness = "claude"
entry = "entry"
mode = "default"
destination = "output.txt"
"#,
    );
    assert!(misspelled_table.contains("unknown field `adapter`"));
}

#[test]
fn case_folded_destination_collisions_are_rejected() {
    let error = validation_error(
        r#"
schema_version = 1
bundle_version = "1.0.0"

[[canonical]]
entry = "entry-1"
mode = "default"
source = "src1.txt"

[[canonical]]
entry = "entry-2"
mode = "default"
source = "src2.txt"

[[adapters]]
harness = "claude"
entry = "entry-1"
mode = "default"
destination = "Assets/File.md"

[[adapters]]
harness = "claude"
entry = "entry-2"
mode = "default"
destination = "assets/file.md"
"#,
    );

    assert!(error.contains("filesystem-equivalent destinations"));
    assert!(error.contains("Assets/File.md"));
    assert!(error.contains("assets/file.md"));
}

#[test]
fn non_ascii_manifest_paths_are_rejected_for_portability() {
    let error = validation_error(
        r#"
schema_version = 1
bundle_version = "1.0.0"

[[canonical]]
entry = "entry"
mode = "default"
source = "café.md"
"#,
    );

    assert!(error.contains("café.md"));
    assert!(error.contains("ASCII forward-slash project-relative syntax"));
}
