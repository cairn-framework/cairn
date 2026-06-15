//! Parse artefact operations from change directories.

// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::delta::clean_scalar;
use super::*;

pub(super) fn parse_artefact_operations(
    root: &Path,
    change_path: &Path,
    findings: &mut Vec<String>,
) -> Vec<ArtefactOperation> {
    let mut operations = Vec::new();
    for dir in [
        "contracts",
        "todos",
        "decisions",
        "research",
        "sources",
        "reviews",
    ] {
        let path = change_path.join(dir);
        if path.exists() {
            collect_artefact_operations(root, change_path, &path, findings, &mut operations);
        }
    }
    operations.sort_by(|left, right| left.target_path.cmp(&right.target_path));
    operations
}

pub(super) fn collect_artefact_operations(
    root: &Path,
    change_path: &Path,
    path: &Path,
    findings: &mut Vec<String>,
    operations: &mut Vec<ArtefactOperation>,
) {
    let Ok(entries) = fs::read_dir(path) else {
        findings.push(format!("failed to read `{}`", path.display()));
        return;
    };
    for entry in entries.flatten() {
        let entry_path = entry.path();
        if entry_path.is_dir() {
            collect_artefact_operations(root, change_path, &entry_path, findings, operations);
            continue;
        }
        if entry_path
            .extension()
            .is_none_or(|extension| extension != "md")
        {
            continue;
        }
        let content = read_to_string(&entry_path, findings);
        let parsed = frontmatter::parse(&content);
        let Some(operation) = parsed
            .values
            .get("operation")
            .and_then(|value| parse_operation(value))
        else {
            findings.push(format!(
                "artefact `{}` is missing valid operation frontmatter",
                entry_path.display()
            ));
            continue;
        };
        let Ok(relative) = entry_path.strip_prefix(change_path) else {
            findings.push(format!(
                "artefact `{}` is outside change",
                entry_path.display()
            ));
            continue;
        };
        let target_path = root.join("meta").join(relative);
        let renamed_from = parsed
            .values
            .get("renamed_from")
            .map(|value| root.join(clean_scalar(value)));
        operations.push(ArtefactOperation {
            operation,
            change_path: entry_path,
            target_path,
            renamed_from,
            content,
        });
    }
}

pub(super) fn parse_operation(value: &str) -> Option<ChangeOperation> {
    match value {
        "added" => Some(ChangeOperation::Added),
        "modified" => Some(ChangeOperation::Modified),
        "removed" => Some(ChangeOperation::Removed),
        "renamed" => Some(ChangeOperation::Renamed),
        _ => None,
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operation_all_variants() {
        assert_eq!(parse_operation("added"), Some(ChangeOperation::Added));
        assert_eq!(parse_operation("modified"), Some(ChangeOperation::Modified));
        assert_eq!(parse_operation("removed"), Some(ChangeOperation::Removed));
        assert_eq!(parse_operation("renamed"), Some(ChangeOperation::Renamed));
    }

    #[test]
    fn test_parse_operation_unknown_is_none() {
        assert_eq!(parse_operation("created"), None);
        assert_eq!(parse_operation("deleted"), None);
        assert_eq!(
            parse_operation("ADDED"),
            None,
            "case-sensitive: uppercase must not match"
        );
        assert_eq!(parse_operation(""), None);
    }
}
