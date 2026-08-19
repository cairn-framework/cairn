//! Convention-enforcement tests.

use std::fs;
use std::path::PathBuf;

#[test]
fn test_every_allow_attr_has_reason_comment() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut violations = Vec::new();
    scan_dir(&manifest.join("src"), &manifest, &mut violations);
    scan_dir(&manifest.join("tests"), &manifest, &mut violations);

    assert!(
        violations.is_empty(),
        "Found {} #[allow(...)] without // Reason: comment:\n{}",
        violations.len(),
        violations.join("\n")
    );
}

fn scan_dir(dir: &std::path::Path, manifest: &std::path::Path, violations: &mut Vec<String>) {
    let self_path = manifest.join("tests/conventions.rs");
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            scan_dir(&path, manifest, violations);
        } else if path.extension().is_some_and(|ext| ext == "rs") && path != self_path {
            let content = fs::read_to_string(&path).unwrap();
            let lines: Vec<&str> = content.lines().collect();
            for (idx, line) in lines.iter().enumerate() {
                let trimmed = line.trim();
                // Skip the scanner logic inside this test file when it appears
                // in other files (e.g. copy-pasted test utilities).
                if trimmed.contains("starts_with(\"#[allow(\")")
                    || trimmed.contains("starts_with(\"#![allow(\")")
                {
                    continue;
                }
                if trimmed.starts_with("#[allow(") || trimmed.starts_with("#![allow(") {
                    let has_reason = find_reason_in_preceding_comments(&lines, idx)
                        || trimmed.contains("// Reason:")
                        || lines
                            .get(idx + 1)
                            .is_some_and(|next| next.trim().starts_with("// Reason:"));
                    if !has_reason {
                        let rel = path.strip_prefix(manifest).unwrap().display();
                        violations.push(format!("{}:{} {}", rel, idx + 1, trimmed));
                    }
                }
            }
        }
    }
}

/// Walk backwards through consecutive `//` comment lines and return true if
/// any of them starts with `// Reason:`.
fn find_reason_in_preceding_comments(lines: &[&str], allow_idx: usize) -> bool {
    for i in (0..allow_idx).rev() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("// Reason:") {
            return true;
        }
        if !trimmed.starts_with("//") {
            break;
        }
    }
    false
}

#[test]
fn test_every_source_file_has_module_doc() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut violations = Vec::new();
    scan_src_for_module_docs(&manifest.join("src"), &manifest, &mut violations);

    assert!(
        violations.is_empty(),
        "Found {} source files without a //! module doc in the first 5 lines:\n{}",
        violations.len(),
        violations.join("\n")
    );
}

fn scan_src_for_module_docs(
    dir: &std::path::Path,
    manifest: &std::path::Path,
    violations: &mut Vec<String>,
) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            scan_src_for_module_docs(&path, manifest, violations);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            let content = fs::read_to_string(&path).unwrap();
            let has_doc = content
                .lines()
                .take(5)
                .any(|line| line.trim_start().starts_with("//!"));
            if !has_doc {
                let rel = path.strip_prefix(manifest).unwrap().display();
                violations.push(rel.to_string());
            }
        }
    }
}

/// Dogfood guard: every `contract` pointer wired into cairn's own blueprint
/// must resolve to a real file whose `node:` frontmatter matches the declaring
/// node. A broken pointer (missing file, missing/mismatched node) surfaces as
/// an error-severity finding from the contract loader; this test fails on any.
#[test]
fn test_blueprint_contract_pointers_resolve() {
    use cairn::artefacts::contract::load_contracts;
    use cairn::blueprint::parser::parse_file;
    use cairn::map::graph::FindingSeverity;

    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let ast = parse_file(manifest.join("cairn.blueprint")).expect("parse cairn.blueprint");
    let set = load_contracts(&manifest, &ast);

    let errors: Vec<String> = set
        .findings
        .iter()
        .filter(|f| f.severity == FindingSeverity::Error)
        .map(|f| format!("{}: {}", f.code, f.message))
        .collect();
    assert!(
        errors.is_empty(),
        "blueprint contract pointers produced {} error finding(s):\n{}",
        errors.len(),
        errors.join("\n")
    );
    assert!(
        !set.contracts.is_empty(),
        "expected cairn's blueprint to declare at least one contract pointer"
    );
}

/// `docs/conventions.md` once required `thiserror::Error` for all error types
/// while nothing in the workspace used it
/// (`todo.conventions-thiserror-divergence`). Guard both sides of the resolved
/// state: the Error Types section states only the hand-written convention, and
/// no workspace package declares `thiserror`.
#[test]
fn test_conventions_error_types_match_code() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let conventions =
        fs::read_to_string(manifest_dir.join("docs/conventions.md")).expect("read conventions.md");
    let error_types = conventions
        .split("### Error Types")
        .nth(1)
        .and_then(|tail| tail.split("### ").next())
        .expect("docs/conventions.md must contain an Error Types section");
    assert!(
        error_types.contains("implement `fmt::Display` and `std::error::Error` by hand"),
        "docs/conventions.md Error Types section must state the hand-written \
         convention (dec.conventions-error-types)"
    );
    assert!(
        !error_types.contains("MUST use `thiserror::Error`"),
        "docs/conventions.md Error Types section must not restore the superseded \
         `thiserror` mandate (dec.conventions-error-types)"
    );

    let metadata = std::process::Command::new(env!("CARGO"))
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .current_dir(&manifest_dir)
        .output()
        .expect("run cargo metadata");
    assert!(
        metadata.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&metadata.stderr)
    );
    let metadata: serde_json::Value =
        serde_json::from_slice(&metadata.stdout).expect("cargo metadata output must be JSON");
    let thiserror_users = metadata["packages"]
        .as_array()
        .expect("cargo metadata packages must be an array")
        .iter()
        .filter_map(|package| {
            let uses_thiserror = package["dependencies"]
                .as_array()
                .is_some_and(|dependencies| {
                    dependencies
                        .iter()
                        .any(|dependency| dependency["name"] == "thiserror")
                });
            uses_thiserror.then(|| package["name"].as_str().unwrap_or("<unnamed>"))
        })
        .collect::<Vec<_>>();
    assert!(
        thiserror_users.is_empty(),
        "workspace packages must not declare `thiserror` while the hand-written \
         error convention stands: {}",
        thiserror_users.join(", ")
    );
}

#[test]
fn cargo_package_list_stays_within_expected_roots() {
    let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let manifest: toml::Value =
        toml::from_str(&fs::read_to_string(manifest_path).expect("read Cargo.toml"))
            .expect("Cargo.toml must parse");
    let include = manifest["package"]["include"]
        .as_array()
        .expect("package.include must be an array");
    assert!(
        include.iter().all(|pattern| pattern
            .as_str()
            .is_some_and(|pattern| pattern.starts_with('/'))),
        "every package.include pattern must be anchored at the package root"
    );

    let output = std::process::Command::new(env!("CARGO"))
        .args(["package", "--list", "--allow-dirty"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run cargo package --list");
    assert!(
        output.status.success(),
        "cargo package --list failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let expected_roots = [
        "src/",
        "schemas/",
        "docs/design-system/",
        ".claude/skills/",
        ".claude/commands/",
        "Cargo.lock",
        "Cargo.toml",
        "Cargo.toml.orig",
        ".cargo_vcs_info.json",
        "README.md",
        "LICENSE-APACHE",
        "LICENSE-MIT",
        "CHANGELOG.md",
    ];

    let packaged_paths = String::from_utf8(output.stdout).expect("package list is UTF-8");
    for required in [
        "src/lib.rs",
        "schemas/finding.schema.json",
        "docs/design-system/copy.toml",
        ".claude/skills/cairn-dev/SKILL.md",
        ".claude/skills/cairn-loop-scope/SKILL.md",
        ".claude/skills/cairn-loop-implement/SKILL.md",
        ".claude/skills/cairn-loop-recovery/SKILL.md",
        ".claude/skills/cairn-loop-reconcile/SKILL.md",
        ".claude/skills/cairn-loop-landing/SKILL.md",
        ".claude/commands/cairn-loop.md",
        "README.md",
    ] {
        assert!(
            packaged_paths.lines().any(|path| path == required),
            "cargo package omitted required compile-time asset {required}"
        );
    }
    for path in packaged_paths.lines() {
        assert!(
            expected_roots.iter().any(|root| {
                root.ends_with('/') && path.starts_with(root)
                    || !root.ends_with('/') && path == *root
            }),
            "cargo package included unexpected path {path}"
        );
    }
}
