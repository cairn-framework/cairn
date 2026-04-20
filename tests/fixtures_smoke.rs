//! Smoke tests for Phase 0 fixture access and CLI version output.

use std::{fs, path::Path, process::Command};

const FIXTURE_START_TOKENS: [&str; 4] = ["System", "Container", "Module", "Actor"];

#[test]
fn test_root_fixture_readable_contains_declared_node() -> Result<(), Box<dyn std::error::Error>> {
    assert_fixture_contains_declared_node("test/fixtures/cairn.blueprint")
}

#[test]
fn test_bootstrap_fixture_readable_contains_declared_node() -> Result<(), Box<dyn std::error::Error>>
{
    assert_fixture_contains_declared_node("test/fixtures/cairn-bootstrap/cairn.blueprint")
}

#[test]
fn test_cli_version_prints_package_name_and_version() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .arg("--version")
        .output()?;

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout)?;
    assert_eq!(stdout.trim(), cairn::version_label());

    Ok(())
}

fn assert_fixture_contains_declared_node(
    path: impl AsRef<Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.as_ref();

    assert!(path.exists(), "fixture does not exist: {}", path.display());

    let contents = fs::read_to_string(path)?;

    assert!(
        !contents.trim().is_empty(),
        "fixture is empty: {}",
        path.display()
    );
    assert!(
        contains_declared_node_line(&contents),
        "fixture lacks a declaration line: {}",
        path.display()
    );

    Ok(())
}

fn contains_declared_node_line(contents: &str) -> bool {
    contents.lines().any(|line| {
        let trimmed = line.trim();

        !trimmed.is_empty()
            && !trimmed.starts_with('#')
            && trimmed
                .split_whitespace()
                .next()
                .is_some_and(|token| FIXTURE_START_TOKENS.contains(&token))
    })
}
