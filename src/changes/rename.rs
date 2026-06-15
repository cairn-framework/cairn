//! Rename-aware reference rewriting for changes.

// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::apply::atomic_write;
use super::*;

pub(super) fn read_to_string(path: &Path, findings: &mut Vec<String>) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| {
        findings.push(format!("failed to read `{}`: {error}", path.display()));
        String::new()
    })
}

pub(super) fn proposal_title(source: &str) -> Option<String> {
    source.lines().find_map(|line| {
        line.strip_prefix("# Proposal:")
            .or_else(|| line.strip_prefix("# "))
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
    })
}

pub(super) fn copy_referencing_artefacts(
    root: &Path,
    change_path: &Path,
    old_id: &str,
    new_id: &str,
) -> Result<(), String> {
    let meta = root.join("meta");
    if !meta.exists() {
        return Ok(());
    }
    copy_referencing_artefacts_from(root, change_path, &meta, old_id, new_id)
}

pub(super) fn copy_referencing_artefacts_from(
    root: &Path,
    change_path: &Path,
    dir: &Path,
    old_id: &str,
    new_id: &str,
) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            if path
                .strip_prefix(root)
                .is_ok_and(|relative| relative.starts_with("meta/changes"))
            {
                continue;
            }
            copy_referencing_artefacts_from(root, change_path, &path, old_id, new_id)?;
            continue;
        }
        if path.extension().is_none_or(|extension| extension != "md") {
            continue;
        }
        let content = fs::read_to_string(&path).map_err(|error| error.to_string())?;
        if !frontmatter_references(&content, old_id) {
            continue;
        }
        let relative = path
            .strip_prefix(root.join("meta"))
            .map_err(|error| error.to_string())?;
        let target = change_path.join(relative);
        let updated = update_frontmatter_reference(&content, old_id, new_id);
        let updated = insert_operation(&updated, "modified", None);
        atomic_write(&target, &updated)?;
    }
    Ok(())
}

pub(super) fn frontmatter_references(source: &str, id: &str) -> bool {
    let parsed = frontmatter::parse(source);
    parsed.values.values().any(|value| value == id)
        || parsed
            .lists
            .values()
            .any(|values| values.iter().any(|value| value == id))
}

pub(super) fn update_frontmatter_reference(source: &str, old_id: &str, new_id: &str) -> String {
    let mut in_frontmatter = false;
    let mut seen_start = false;
    let mut output = Vec::new();
    for line in source.lines() {
        if !seen_start && line.trim() == "---" {
            seen_start = true;
            in_frontmatter = true;
            output.push(line.to_owned());
            continue;
        }
        if in_frontmatter && line.trim() == "---" {
            in_frontmatter = false;
            output.push(line.to_owned());
            continue;
        }
        if in_frontmatter {
            output.push(line.replace(old_id, new_id));
        } else {
            output.push(line.to_owned());
        }
    }
    format!("{}\n", output.join("\n"))
}

pub(super) fn insert_operation(
    source: &str,
    operation: &str,
    renamed_from: Option<&Path>,
) -> String {
    let mut output = Vec::new();
    let mut inserted = false;
    for line in source.lines() {
        output.push(line.to_owned());
        if !inserted && line.trim() == "---" {
            output.push(format!("operation: {operation}"));
            if let Some(path) = renamed_from {
                output.push(format!("renamed_from: {}", path.display()));
            }
            inserted = true;
        }
    }
    format!("{}\n", output.join("\n"))
}

pub(super) fn artefact_content_refs(source: &str, ids: &BTreeSet<String>) -> bool {
    let parsed = frontmatter::parse(source);
    parsed.values.values().any(|value| ids.contains(value))
        || parsed
            .lists
            .values()
            .any(|values| values.iter().any(|value| ids.contains(value)))
}
#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    // ── proposal_title ────────────────────────────────────────────────────────

    #[test]
    fn test_proposal_title_from_proposal_prefix() {
        assert_eq!(
            proposal_title("# Proposal: My Feature\n\nbody"),
            Some("My Feature".to_owned())
        );
    }

    #[test]
    fn test_proposal_title_from_plain_h1() {
        assert_eq!(
            proposal_title("# My Feature\n\nbody"),
            Some("My Feature".to_owned())
        );
    }

    #[test]
    fn test_proposal_title_prefers_first_matching_line() {
        // Finds the first # or # Proposal: line; does not skip non-matching lines.
        assert_eq!(
            proposal_title("## Subheading\n# First Match\n# Second"),
            Some("First Match".to_owned())
        );
    }

    #[test]
    fn test_proposal_title_empty_heading_is_skipped() {
        // "# " with nothing after must be filtered, not returned as empty string.
        assert_eq!(
            proposal_title("# \n# Real Title\n"),
            Some("Real Title".to_owned())
        );
    }

    #[test]
    fn test_proposal_title_no_heading_returns_none() {
        assert_eq!(proposal_title("just body text\nno headings"), None);
    }

    #[test]
    fn test_proposal_title_trims_whitespace() {
        assert_eq!(
            proposal_title("# Proposal:   Padded Title   \n"),
            Some("Padded Title".to_owned())
        );
    }

    // ── frontmatter_references ────────────────────────────────────────────────

    #[test]
    fn test_frontmatter_references_scalar_match() {
        let src = "---\nnode: app.api\n---\nbody";
        assert!(frontmatter_references(src, "app.api"));
    }

    #[test]
    fn test_frontmatter_references_list_match() {
        let src = "---\nnodes:\n- app.api\n- app.db\n---\n";
        assert!(frontmatter_references(src, "app.api"));
    }

    #[test]
    fn test_frontmatter_references_no_match() {
        let src = "---\nnode: app.other\n---\nbody mentions app.api";
        // ID appears in body but not frontmatter — must return false.
        assert!(!frontmatter_references(src, "app.api"));
    }

    #[test]
    fn test_frontmatter_references_body_mention_not_counted() {
        // Ensure body text is not scanned.
        let src = "---\nnode: app.other\n---\napp.target is mentioned here";
        assert!(!frontmatter_references(src, "app.target"));
    }

    // ── update_frontmatter_reference ──────────────────────────────────────────

    #[test]
    fn test_update_frontmatter_reference_scalar_value() {
        let src = "---\nnode: app.old\n---\nbody";
        let result = update_frontmatter_reference(src, "app.old", "app.new");
        assert!(
            result.contains("node: app.new"),
            "scalar value must be updated"
        );
        assert!(result.contains("body"), "body must be preserved");
    }

    #[test]
    fn test_update_frontmatter_reference_list_value() {
        let src = "---\nnodes:\n- app.old\n- app.other\n---\nbody";
        let result = update_frontmatter_reference(src, "app.old", "app.new");
        assert!(result.contains("app.new"), "list item must be updated");
        assert!(
            result.contains("app.other"),
            "other list items must be preserved"
        );
    }

    #[test]
    fn test_update_frontmatter_reference_body_not_modified() {
        // The old ID in the body must NOT be replaced — only frontmatter is touched.
        let src = "---\nnode: app.old\n---\napp.old is mentioned in the body";
        let result = update_frontmatter_reference(src, "app.old", "app.new");
        assert!(
            result.contains("app.old is mentioned in the body"),
            "body occurrences of old ID must not be replaced; got:\n{result}"
        );
    }

    #[test]
    fn test_update_frontmatter_reference_trailing_newline() {
        let src = "---\nnode: app.old\n---\n";
        let result = update_frontmatter_reference(src, "app.old", "app.new");
        assert!(
            result.ends_with('\n'),
            "result must always end with a newline"
        );
    }

    #[test]
    fn test_update_frontmatter_reference_no_match_is_noop() {
        let src = "---\nnode: app.other\n---\nbody";
        let result = update_frontmatter_reference(src, "app.old", "app.new");
        assert!(!result.contains("app.new"), "no match means no replacement");
        assert!(
            result.contains("app.other"),
            "unrelated values must be preserved"
        );
    }

    // ── insert_operation ──────────────────────────────────────────────────────

    #[test]
    fn test_insert_operation_adds_after_opening_dashes() {
        let src = "---\ntitle: T\n---\nbody";
        let result = insert_operation(src, "modified", None);
        // operation: must appear immediately after the first ---
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines[0], "---");
        assert_eq!(
            lines[1], "operation: modified",
            "operation must be first line after ---"
        );
        assert_eq!(lines[2], "title: T");
    }

    #[test]
    fn test_insert_operation_with_renamed_from() {
        let src = "---\ntitle: T\n---\n";
        let path = std::path::Path::new("meta/decisions/d1.md");
        let result = insert_operation(src, "renamed", Some(path));
        assert!(result.contains("operation: renamed"));
        assert!(result.contains("renamed_from: meta/decisions/d1.md"));
    }

    #[test]
    fn test_insert_operation_without_frontmatter_does_not_insert() {
        // A document with no opening --- must not have operation: inserted anywhere.
        let src = "# Title\n\nBody text.\n";
        let result = insert_operation(src, "modified", None);
        assert!(
            !result.contains("operation:"),
            "insert_operation must not insert into a document without frontmatter"
        );
    }

    #[test]
    fn test_insert_operation_trailing_newline() {
        let src = "---\ntitle: T\n---\n";
        let result = insert_operation(src, "modified", None);
        assert!(result.ends_with('\n'));
    }

    // ── artefact_content_refs ─────────────────────────────────────────────────

    #[test]
    fn test_artefact_content_refs_scalar_match() {
        let src = "---\nnode: app.api\n---\n";
        let ids: BTreeSet<String> = ["app.api".to_owned()].into();
        assert!(artefact_content_refs(src, &ids));
    }

    #[test]
    fn test_artefact_content_refs_list_match() {
        let src = "---\nnodes:\n- app.api\n- app.db\n---\n";
        let ids: BTreeSet<String> = ["app.db".to_owned()].into();
        assert!(artefact_content_refs(src, &ids));
    }

    #[test]
    fn test_artefact_content_refs_no_match() {
        let src = "---\nnode: app.other\n---\nbody";
        let ids: BTreeSet<String> = ["app.api".to_owned()].into();
        assert!(!artefact_content_refs(src, &ids));
    }
}
