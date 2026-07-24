//! Determinism and checked-in drift behavior for the agent-pack renderer.

use cairn_agent_pack::{run_check, run_write};
use std::collections::BTreeSet;
use tempfile::TempDir;

#[test]
fn adapter_row_order_does_not_change_rendered_bytes() {
    let temp = TempDir::new().unwrap();
    std::fs::write(temp.path().join("a.txt"), b"content A").unwrap();
    std::fs::write(temp.path().join("b.txt"), b"content B").unwrap();
    let first_manifest = temp.path().join("first.toml");
    let second_manifest = temp.path().join("second.toml");
    std::fs::write(&first_manifest, manifest_with_order("b", "a")).unwrap();
    std::fs::write(&second_manifest, manifest_with_order("a", "b")).unwrap();
    let first_root = temp.path().join("first");
    let second_root = temp.path().join("second");
    std::fs::create_dir_all(&first_root).unwrap();
    std::fs::create_dir_all(&second_root).unwrap();

    run_write(&first_manifest, &first_root).unwrap();
    run_write(&second_manifest, &second_root).unwrap();

    for destination in ["a.out", "b.out"] {
        assert_eq!(
            std::fs::read(first_root.join(destination)).unwrap(),
            std::fs::read(second_root.join(destination)).unwrap()
        );
    }
}

fn manifest_with_order(first: &str, second: &str) -> String {
    format!(
        r#"
schema_version = 1
bundle_version = "1.0.0"

[[canonical]]
entry = "a"
mode = "default"
source = "a.txt"

[[canonical]]
entry = "b"
mode = "default"
source = "b.txt"

[[adapters]]
harness = "claude"
entry = "{first}"
mode = "default"
destination = "{first}.out"

[[adapters]]
harness = "claude"
entry = "{second}"
mode = "default"
destination = "{second}.out"
"#
    )
}

#[test]
fn drift_check_names_the_file_and_regeneration_command() {
    let temp = TempDir::new().unwrap();
    let manifest_path = temp.path().join("manifest.toml");
    std::fs::write(temp.path().join("source.txt"), b"canonical data").unwrap();
    std::fs::write(
        &manifest_path,
        r#"
schema_version = 1
bundle_version = "1.0.0"

[[canonical]]
entry = "test-entry"
mode = "default"
source = "source.txt"

[[adapters]]
harness = "claude"
entry = "test-entry"
mode = "default"
destination = "output.txt"
"#,
    )
    .unwrap();

    let missing = run_check(&manifest_path, temp.path())
        .unwrap_err()
        .to_string();
    assert!(missing.contains("missing: output.txt"));
    assert!(missing.contains("cargo run -p cairn-agent-pack -- --write"));

    run_write(&manifest_path, temp.path()).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(temp.path().join("output.txt"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(mode, 0o644);
    }
    run_check(&manifest_path, temp.path()).unwrap();
    std::fs::write(temp.path().join("output.txt"), b"tampered data").unwrap();

    let drifted = run_check(&manifest_path, temp.path())
        .unwrap_err()
        .to_string();
    assert!(drifted.contains("drifted: output.txt"));
    assert!(drifted.contains("cargo run -p cairn-agent-pack -- --write"));
    assert!(drifted.contains("--manifest <MANIFEST> --root <ROOT>"));
    assert!(drifted.contains(&manifest_path.display().to_string()));
    assert!(drifted.contains(&temp.path().display().to_string()));

    run_write(&manifest_path, temp.path()).unwrap();
    assert_eq!(
        std::fs::read(temp.path().join("output.txt")).unwrap(),
        b"canonical data"
    );
}

const EXPECTED_CANONICAL: [(&str, &str, &str); 11] = [
    (
        "cairn-apply",
        "default",
        "content/skills/cairn-apply/SKILL.md",
    ),
    (
        "cairn-archive",
        "default",
        "content/skills/cairn-archive/SKILL.md",
    ),
    ("cairn-dev", "default", "content/skills/cairn-dev/SKILL.md"),
    (
        "cairn-dev-reference-artefact-schemas",
        "default",
        "content/skills/cairn-dev/references/artefact-schemas.md",
    ),
    (
        "cairn-dev-reference-blueprint-syntax",
        "default",
        "content/skills/cairn-dev/references/blueprint-syntax.md",
    ),
    (
        "cairn-dev-reference-finding-codes",
        "default",
        "content/skills/cairn-dev/references/finding-codes.md",
    ),
    (
        "cairn-explore",
        "default",
        "content/skills/cairn-explore/SKILL.md",
    ),
    ("cairn-loop", "default", "content/commands/cairn-loop.md"),
    (
        "cairn-loop-landing",
        "default",
        "content/skills/cairn-loop-landing/SKILL.md",
    ),
    (
        "cairn-loop-recovery",
        "default",
        "content/skills/cairn-loop-recovery/SKILL.md",
    ),
    (
        "cairn-propose",
        "default",
        "content/skills/cairn-propose/SKILL.md",
    ),
];

const EXPECTED_CLAUDE: [(&str, &str, &str); 11] = [
    (
        "cairn-apply",
        "default",
        ".claude/skills/cairn-apply/SKILL.md",
    ),
    (
        "cairn-archive",
        "default",
        ".claude/skills/cairn-archive/SKILL.md",
    ),
    ("cairn-dev", "default", ".claude/skills/cairn-dev/SKILL.md"),
    (
        "cairn-dev-reference-artefact-schemas",
        "default",
        ".claude/skills/cairn-dev/references/artefact-schemas.md",
    ),
    (
        "cairn-dev-reference-blueprint-syntax",
        "default",
        ".claude/skills/cairn-dev/references/blueprint-syntax.md",
    ),
    (
        "cairn-dev-reference-finding-codes",
        "default",
        ".claude/skills/cairn-dev/references/finding-codes.md",
    ),
    (
        "cairn-explore",
        "default",
        ".claude/skills/cairn-explore/SKILL.md",
    ),
    ("cairn-loop", "default", ".claude/commands/cairn-loop.md"),
    (
        "cairn-loop-landing",
        "default",
        ".claude/skills/cairn-loop-landing/SKILL.md",
    ),
    (
        "cairn-loop-recovery",
        "default",
        ".claude/skills/cairn-loop-recovery/SKILL.md",
    ),
    (
        "cairn-propose",
        "default",
        ".claude/skills/cairn-propose/SKILL.md",
    ),
];

#[test]
fn checked_in_claude_outputs_match_the_real_manifest() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest_path = manifest_dir.join("manifest.toml");
    let repo_root = manifest_dir.join("../..");
    run_check(&manifest_path, &repo_root).unwrap();

    let manifest_text = std::fs::read_to_string(&manifest_path).unwrap();
    let manifest: toml::Value = toml::from_str(&manifest_text).unwrap();
    let expected_canonical = BTreeSet::from(EXPECTED_CANONICAL);
    let actual_canonical: BTreeSet<_> = manifest["canonical"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            (
                row["entry"].as_str().unwrap(),
                row["mode"].as_str().unwrap(),
                row["source"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(actual_canonical, expected_canonical);

    let expected_claude = BTreeSet::from(EXPECTED_CLAUDE);
    let actual_claude: BTreeSet<_> = manifest["adapters"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|row| row["harness"].as_str() == Some("claude"))
        .map(|row| {
            (
                row["entry"].as_str().unwrap(),
                row["mode"].as_str().unwrap(),
                row["destination"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(actual_claude, expected_claude);

    let expected_destinations: BTreeSet<_> =
        expected_claude.iter().map(|(_, _, path)| *path).collect();
    let attributes = std::fs::read_to_string(repo_root.join(".gitattributes")).unwrap();
    let generated: BTreeSet<_> = attributes
        .lines()
        .filter_map(|line| line.strip_suffix(" linguist-generated=true"))
        .filter(|path| path.starts_with(".claude/"))
        .collect();
    assert_eq!(generated, expected_destinations);
}
