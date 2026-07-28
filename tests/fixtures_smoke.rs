//! Smoke tests for Phase 0 fixture access and CLI version output.

use std::{ffi::OsStr, fs, path::Path, process::Command};

const FIXTURE_START_TOKENS: [&str; 4] = ["System", "Container", "Module", "Actor"];

const BOOTSTRAP_ROOT: &str = "tests/fixtures/cairn-bootstrap";

/// Sources directory, relative to the fixture root. `file:` values in these
/// artefacts are written against that root, so a self-reference compares
/// against this prefix rather than against a repository path.
const BOOTSTRAP_SOURCES_REL: &str = "meta/sources";

/// Every source the bootstrap fixture ships. Pinned rather than derived from
/// the directory: the fixture declares no `sources` pointer, so nothing loads
/// these files and `CAIRN_ARTEFACT_FILENAME_DRIFT` never sees them. Without an
/// expected set, deleting a source would satisfy a filename check vacuously.
const BOOTSTRAP_SOURCE_IDS: [&str; 9] = [
    "src.adr-tools",
    "src.akash-llm-project-wiki",
    "src.dlthub-map-first",
    "src.dual-graph-codex-compact",
    "src.karpathy-llm-wiki",
    "src.openspec-deepwiki",
    "src.openspec-repo",
    "src.review-adversarial-1",
    "src.structurizr-blueprint",
];

#[test]
fn test_root_fixture_readable_contains_declared_node() -> Result<(), Box<dyn std::error::Error>> {
    assert_fixture_contains_declared_node("tests/fixtures/cairn.blueprint")
}

#[test]
fn test_bootstrap_fixture_readable_contains_declared_node() -> Result<(), Box<dyn std::error::Error>>
{
    assert_fixture_contains_declared_node("tests/fixtures/cairn-bootstrap/cairn.blueprint")
}

/// `dec.artefact-layout-authority` for a corpus no reconciler reaches: each file
/// is its own `id` with the `src.` prefix stripped, and the directory holds
/// exactly the pinned set, so a removal fails as loudly as a bad name.
#[test]
fn test_bootstrap_fixture_sources_are_named_for_their_ids() -> Result<(), Box<dyn std::error::Error>>
{
    let dir = Path::new(BOOTSTRAP_ROOT).join(BOOTSTRAP_SOURCES_REL);

    let mut found: Vec<String> = Vec::new();
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();

        assert!(
            entry.file_type()?.is_file(),
            "artefact directories are flat, so `{}` must not be a directory",
            path.display()
        );
        if path.extension().and_then(OsStr::to_str) != Some("md") {
            continue;
        }

        let stem = path
            .file_stem()
            .and_then(OsStr::to_str)
            .ok_or_else(|| format!("unreadable filename: {}", path.display()))?
            .to_owned();
        let id = frontmatter_value(&fs::read_to_string(&path)?, "id")
            .ok_or_else(|| format!("source declares no id: {}", path.display()))?;

        assert_eq!(
            id.strip_prefix("src."),
            Some(stem.as_str()),
            "source filename must be its id without the `src.` prefix: {}",
            path.display()
        );
        found.push(id);
    }
    found.sort();

    assert_eq!(
        found, BOOTSTRAP_SOURCE_IDS,
        "bootstrap source set changed; renaming is expected, removal is not"
    );

    Ok(())
}

/// A source records where its evidence lives. Pointing `file:` at the artefact
/// itself records nothing, and the id-derived filename makes that collision
/// reachable by rename rather than by typo.
///
/// Driven from the pinned ids rather than from the directory so that a deleted
/// source fails here too, and compared after collapsing `.` and `..` so the
/// guard is not satisfied by respelling the same path.
#[test]
fn test_bootstrap_fixture_sources_do_not_cite_themselves() -> Result<(), Box<dyn std::error::Error>>
{
    for id in BOOTSTRAP_SOURCE_IDS {
        let slug = id.strip_prefix("src.").ok_or("id lacks `src.` prefix")?;
        let own = format!("{BOOTSTRAP_SOURCES_REL}/{slug}.md");
        let path = Path::new(BOOTSTRAP_ROOT).join(&own);

        let contents = fs::read_to_string(&path)
            .map_err(|error| format!("pinned source missing at {}: {error}", path.display()))?;
        // `file:` is required of every source (`required` in
        // `src/artefacts/registry/kinds.rs`), and no loader reaches this
        // directory to say so, hence the explicit error rather than a skip.
        let file = frontmatter_value(&contents, "file")
            .ok_or_else(|| format!("source `{id}` declares no `file:`: {}", path.display()))?;
        let file = file.trim_matches(['"', '\'']);

        if file == "null" || file.contains("://") {
            continue;
        }
        assert_ne!(
            collapse_relative(file),
            own,
            "source `{id}` cites its own artefact as its evidence: {file}"
        );
    }

    Ok(())
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

/// First `key: value` line in the leading frontmatter block, trailing `#`
/// comment stripped. Deliberately naive: these fixtures are flat scalars, and
/// the test must not depend on the loader it guards.
fn frontmatter_value(contents: &str, key: &str) -> Option<String> {
    let prefix = format!("{key}:");
    let body = contents.strip_prefix("---\n")?;
    body.lines()
        .take_while(|line| line.trim() != "---")
        .find_map(|line| line.strip_prefix(&prefix))
        .map(|value| {
            value
                .split_once(" #")
                .map_or(value, |(before, _)| before)
                .trim()
                .to_owned()
        })
}

/// Lexically collapses `.` and `..` segments, so `./meta/sources/x.md`,
/// `meta/sources/x.md`, and `meta/sources/../sources/x.md` compare equal.
///
/// A `..` with nothing to cancel is KEPT, and an absolute path is returned
/// unchanged. Both stay unequal to the fixture-root-relative path they are
/// compared against, so a value pointing outside the fixture is not mistaken
/// for a self-reference. Purely textual: nothing here touches the filesystem.
fn collapse_relative(value: &str) -> String {
    if value.starts_with('/') {
        return value.to_owned();
    }

    let mut parts: Vec<&str> = Vec::new();
    for segment in value.split('/') {
        match segment {
            "" | "." => {}
            ".." if matches!(parts.last(), Some(&last) if last != "..") => {
                parts.pop();
            }
            other => parts.push(other),
        }
    }
    parts.join("/")
}
