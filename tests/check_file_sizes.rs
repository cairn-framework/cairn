//! Integration tests for the Rust file-size gate script.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn test_check_file_sizes_script_behaviour() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root()?;
    let src = root.join("src");
    fs::create_dir_all(&src)?;

    write_lines(&src.join("exact.rs"), 500, None)?;
    assert!(run_script(&root).status.success());

    write_lines(&src.join("too_big.rs"), 501, None)?;
    let failed = run_script(&root);
    assert!(!failed.status.success());
    let stderr = String::from_utf8(failed.stderr)?;
    assert!(stderr.contains("too_big.rs"));
    assert!(stderr.contains("501"));

    write_lines(
        &src.join("allowed.rs"),
        501,
        Some("// cairn:allow-large-module reason: scheduled-for-phase-7.5b-split"),
    )?;
    fs::remove_file(src.join("too_big.rs"))?;
    assert!(run_script(&root).status.success());

    for empty_reason in [
        "// cairn:allow-large-module reason:",
        "// cairn:allow-large-module reason: ",
    ] {
        write_lines(&src.join("missing_reason.rs"), 501, Some(empty_reason))?;
        let missing_reason = run_script(&root);
        assert!(
            !missing_reason.status.success(),
            "expected failure for {empty_reason:?}"
        );
        let stderr = String::from_utf8(missing_reason.stderr)?;
        assert!(stderr.contains("missing_reason.rs"));
        assert!(
            stderr.contains("missing non-empty allow-list reason"),
            "expected missing-reason diagnostic for {empty_reason:?}, got: {stderr}",
        );
    }

    Ok(())
}

fn run_script(root: &Path) -> std::process::Output {
    Command::new("sh")
        .arg("scripts/check-file-sizes.sh")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("CAIRN_FILE_SIZE_ROOT", root)
        .output()
        .expect("script should execute")
}

fn write_lines(
    path: &Path,
    count: usize,
    first_line: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = Vec::with_capacity(count);
    if let Some(first_line) = first_line {
        lines.push(first_line.to_owned());
    }
    while lines.len() < count {
        lines.push(format!("fn line_{}() {{}}", lines.len()));
    }
    fs::write(path, format!("{}\n", lines.join("\n")))?;
    Ok(())
}

fn temp_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("cairn-file-sizes-{suffix}"));
    fs::create_dir_all(&root)?;
    Ok(root)
}
