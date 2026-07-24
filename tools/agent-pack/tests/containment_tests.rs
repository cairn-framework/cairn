//! Filesystem containment behavior for agent-pack paths.

use cairn_agent_pack::{
    ContainmentError, run_check, validate_lexical_containment, validate_resolved_containment,
    validate_resolved_path,
};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

#[test]
fn test_lexical_containment_validation() {
    assert!(validate_lexical_containment(Path::new("valid/relative/path.txt")).is_ok());
    assert!(validate_lexical_containment(Path::new("./valid.txt")).is_ok());

    assert_eq!(
        validate_lexical_containment(Path::new("")),
        Err(ContainmentError::EmptyPath)
    );

    #[cfg(unix)]
    assert_eq!(
        validate_lexical_containment(Path::new("/abs/path.txt")),
        Err(ContainmentError::NotRelative {
            path: PathBuf::from("/abs/path.txt")
        })
    );

    assert_eq!(
        validate_lexical_containment(Path::new("foo/../bar.txt")),
        Err(ContainmentError::ParentTraversal {
            path: PathBuf::from("foo/../bar.txt")
        })
    );
}

#[test]
fn test_resolved_containment_escape_rejected() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("root");
    let outside = temp.path().join("outside");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::create_dir_all(&outside).unwrap();

    let contained = validate_resolved_containment(&root, Path::new("sub/file.txt")).unwrap();
    assert_eq!(contained, root.join("sub/file.txt"));

    let err = validate_resolved_path(&root, &outside.join("file.txt")).unwrap_err();
    assert!(matches!(err, ContainmentError::ResolvedEscape { .. }));
    assert!(err.to_string().contains("escapes root directory"));
}

#[test]
#[cfg(unix)]
fn test_symlink_containment_rejected() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("root");
    let outside = temp.path().join("outside");

    std::fs::create_dir_all(&root).unwrap();
    std::fs::create_dir_all(&outside).unwrap();

    let link_path = root.join("symlink_dir");
    std::os::unix::fs::symlink(&outside, &link_path).unwrap();

    let rel_target = Path::new("symlink_dir/file.txt");
    let res = validate_resolved_containment(&root, rel_target);

    assert!(res.is_err());
    let err = res.unwrap_err();
    match err {
        ContainmentError::SymlinkDetected { ref symlink_path } => {
            assert_eq!(
                symlink_path,
                &std::fs::canonicalize(&root).unwrap().join("symlink_dir")
            );
        }
        _ => panic!("unexpected error variant: {err:?}"),
    }

    assert!(err.to_string().contains("symlink detected"));
}

#[test]
#[cfg(unix)]
fn drift_check_rejects_a_symlinked_destination() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("root");
    let outside = temp.path().join("outside.txt");
    let manifest = temp.path().join("manifest.toml");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(temp.path().join("source.txt"), b"canonical").unwrap();
    std::fs::write(&outside, b"canonical").unwrap();
    std::os::unix::fs::symlink(&outside, root.join("output.txt")).unwrap();
    std::fs::write(
        &manifest,
        r#"
schema_version = 1
bundle_version = "1.0.0"

[[canonical]]
entry = "entry"
mode = "default"
source = "source.txt"

[[adapters]]
harness = "claude"
entry = "entry"
mode = "default"
destination = "output.txt"
"#,
    )
    .unwrap();

    let error = run_check(&manifest, &root).unwrap_err().to_string();
    assert!(error.contains("symlink detected"));
    assert!(error.contains("remove symlinked ancestors"));
}

#[test]
#[cfg(unix)]
fn unadapted_canonical_source_rejects_symlink_escape() {
    let temp = TempDir::new().unwrap();
    let outside = temp.path().join("outside.txt");
    let source_link = temp.path().join("source.txt");
    let manifest = temp.path().join("manifest.toml");
    std::fs::write(&outside, b"outside").unwrap();
    std::os::unix::fs::symlink(&outside, &source_link).unwrap();
    std::fs::write(
        &manifest,
        r#"
schema_version = 1
bundle_version = "1.0.0"

[[canonical]]
entry = "future-entry"
mode = "default"
source = "source.txt"
"#,
    )
    .unwrap();

    let error = run_check(&manifest, temp.path()).unwrap_err().to_string();
    assert!(error.contains("row [[canonical]] index 0"));
    assert!(error.contains("symlink detected"));
    assert!(error.contains("remove symlinked ancestors"));
}

#[test]
fn resolved_path_rejects_unresolved_parent_traversal() {
    let temp = TempDir::new().unwrap();
    let candidate = temp.path().join("missing/../../outside.txt");
    let error = validate_resolved_path(temp.path(), &candidate).unwrap_err();
    assert!(matches!(error, ContainmentError::ParentTraversal { .. }));
}

#[test]
fn destination_io_error_names_adapter_row_and_path() {
    let temp = TempDir::new().unwrap();
    let manifest = temp.path().join("manifest.toml");
    std::fs::write(temp.path().join("source.txt"), b"canonical").unwrap();
    std::fs::write(temp.path().join("blocked"), b"not a directory").unwrap();
    std::fs::write(
        &manifest,
        r#"
schema_version = 1
bundle_version = "1.0.0"

[[canonical]]
entry = "entry"
mode = "default"
source = "source.txt"

[[adapters]]
harness = "claude"
entry = "entry"
mode = "default"
destination = "blocked/output.txt"
"#,
    )
    .unwrap();

    let error = cairn_agent_pack::run_write(&manifest, temp.path())
        .unwrap_err()
        .to_string();
    assert!(error.contains("row [[adapters]] index 0"));
    assert!(error.contains("blocked/output.txt"));
    assert!(error.contains("I/O failure"));
}
