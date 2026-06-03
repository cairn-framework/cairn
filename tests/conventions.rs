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
                if trimmed.contains("starts_with(\"#[allow(\")") {
                    continue;
                }
                if trimmed.starts_with("#[allow(") {
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
