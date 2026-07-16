//! Human renderers for `cairn change list` and `cairn change show`.
//!
//! Both commands already have a JSON path via `run_shared_json_command`
//! (`query_api::change_queries`); these renderers cover the human,
//! non-`--json` surface so a fresh user's onboarding (which points them at
//! `cairn change list`) does not error (`dec.native-todos-first`, Decision 7).
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use crate::query_api::QueryRequest;
use serde_json::Value;
use std::collections::BTreeSet;

/// Renders `cairn change list`: one line per active change directory under
/// the resolved changes dir. Mirrors `changes::active_changes_lines`, used
/// for generated `map.md`.
pub(crate) fn render_changes(root: &Path, changes_dir: &Path) -> String {
    match crate::changes::discover(root, changes_dir) {
        Ok(changes) if changes.is_empty() => {
            format!(
                "{}\n{}\n",
                crate::copy::lookup("empty-states.cli-no-changes.body"),
                crate::copy::lookup("empty-states.cli-no-changes.cta")
            )
        }
        Ok(changes) => {
            let lines = crate::changes::active_changes_lines(&changes);
            lines.join("\n") + "\n"
        }
        Err(error) => format!("failed to discover changes: {error}\n"),
    }
}

/// Renders `cairn change show <change-id>`: proposal, design (if present), and
/// findings for one change.
pub(crate) fn render_show(parsed: &ParsedArgs, root: &Path) -> Result<String, Finding> {
    let change_id = parsed.command_args.get(1).ok_or_else(|| Finding {
        code: "CAIRN_CLI_MISSING_CHANGE".to_owned(),
        severity: FindingSeverity::Error,
        message: "change id argument is required".to_owned(),
        node: None,
        target: None,
        path: None,
        deferred_by: None,
    })?;
    let request = QueryRequest {
        tool: "show".to_owned(),
        node: None,
        symbol: None,
        change: Some(change_id.clone()),
        old_id: None,
        new_id: None,
        status: None,
        language: None,
        flags: BTreeSet::new(),
        mutating: false,
    };
    let data = crate::query_api::execute(
        root,
        &parsed.file,
        &root.join(&parsed.changes_dir),
        &request,
    )
    .map_err(super::query_error_to_finding)?
    .data;
    Ok(show_change_text(&data))
}

/// Renders the canonical `change_json` data (from `show_change`) as human text.
fn show_change_text(data: &Value) -> String {
    let id = data["id"].as_str().unwrap_or_default();
    let title = data["title"].as_str().unwrap_or_default();
    let path = data["path"].as_str().unwrap_or_default();
    let summary = data["summary"].as_str().unwrap_or_default();
    let proposal = data["proposal"].as_str().unwrap_or_default();
    let mut out = vec![
        format!("Change: {id} ({title})"),
        format!("Path: {path}"),
        format!("Summary: {summary}"),
        String::new(),
        proposal.trim().to_owned(),
    ];
    if let Some(design) = data["design"].as_str() {
        out.push(String::new());
        out.push(design.trim().to_owned());
    }
    let findings = data["findings"]
        .as_array()
        .map_or(&[][..], std::ops::Deref::deref);
    if !findings.is_empty() {
        out.push(String::new());
        out.push("Findings:".to_owned());
        for finding in findings {
            out.push(format!("- {}", finding.as_str().unwrap_or_default()));
        }
    }
    out.join("\n") + "\n"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmpdir(tag: &str) -> std::path::PathBuf {
        let dir =
            std::env::temp_dir().join(format!("cairn-changes-view-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn parsed(args: &[&str]) -> ParsedArgs {
        ParsedArgs {
            json: false,
            strict: false,
            file: std::path::PathBuf::from("cairn.blueprint"),
            changes_dir: std::path::PathBuf::from("meta/changes"),
            command: "show".to_owned(),
            command_args: args.iter().map(|arg| (*arg).to_owned()).collect(),
            verbose: false,
            brief: false,
        }
    }

    #[test]
    fn test_render_changes_empty_uses_copy_lookup() {
        let dir = tmpdir("empty");
        let out = render_changes(&dir, &dir.join("meta/changes"));
        assert_eq!(
            out,
            format!(
                "{}\n{}\n",
                crate::copy::lookup("empty-states.cli-no-changes.body"),
                crate::copy::lookup("empty-states.cli-no-changes.cta")
            )
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_render_changes_lists_active_change() {
        let dir = tmpdir("active");
        let change_dir = dir.join("meta/changes/my-change");
        std::fs::create_dir_all(&change_dir).unwrap();
        std::fs::write(
            change_dir.join("proposal.md"),
            "# Proposal: my-change\n\n## Motivation\n\nWhy.\n",
        )
        .unwrap();
        std::fs::write(change_dir.join("design.md"), "# Design\n").unwrap();
        let out = render_changes(&dir, &dir.join("meta/changes"));
        assert!(out.contains("my-change"), "{out}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_render_show_missing_id_errors() {
        let dir = tmpdir("missing-id");
        let result = render_show(&parsed(&["show"]), &dir);
        assert!(result.is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_render_show_unknown_change_errors() {
        let dir = tmpdir("unknown");
        let result = render_show(&parsed(&["show", "nope"]), &dir);
        let finding = result.expect_err("must error");
        assert_eq!(finding.code, "CAIRN_CHANGE_NOT_FOUND");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_render_show_renders_proposal_and_design() {
        let dir = tmpdir("show");
        let change_dir = dir.join("meta/changes/my-change");
        std::fs::create_dir_all(&change_dir).unwrap();
        std::fs::write(
            change_dir.join("proposal.md"),
            "# Proposal: my-change\n\n## Motivation\n\nCloses a gap.\n",
        )
        .unwrap();
        std::fs::write(change_dir.join("design.md"), "# Design\n\nDo the thing.\n").unwrap();
        let out = render_show(&parsed(&["show", "my-change"]), &dir).unwrap();
        assert!(out.contains("Change: my-change"));
        assert!(out.contains("Closes a gap."));
        assert!(out.contains("Do the thing."));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_render_show_renders_findings() {
        let dir = tmpdir("findings");
        let change_dir = dir.join("meta/changes/broken-delta");
        std::fs::create_dir_all(&change_dir).unwrap();
        std::fs::write(
            change_dir.join("proposal.md"),
            "# Proposal: broken-delta\n\n## Motivation\n\nWhy.\n",
        )
        .unwrap();
        // A malformed delta file fails to parse, which `load_change` records
        // as a finding on the `Change` rather than propagating an error.
        std::fs::write(
            change_dir.join("blueprint.delta"),
            "## ADDED Nodes\n\nModule Bad \"Bad\" id \"bad\" {\n",
        )
        .unwrap();
        let out = render_show(&parsed(&["show", "broken-delta"]), &dir).unwrap();
        assert!(out.contains("Findings:"), "{out}");
        assert!(out.contains("- "), "{out}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_render_changes_reports_discover_error() {
        let dir = tmpdir("discover-err");
        // `meta/changes` as a file (not a directory) makes `fs::read_dir`
        // fail inside `crate::changes::discover`.
        std::fs::create_dir_all(dir.join("meta")).unwrap();
        std::fs::write(dir.join("meta/changes"), "not a directory").unwrap();
        let out = render_changes(&dir, &dir.join("meta/changes"));
        assert!(out.contains("failed to discover changes"), "{out}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_render_show_reports_discover_error() {
        let dir = tmpdir("show-discover-err");
        std::fs::create_dir_all(dir.join("meta")).unwrap();
        std::fs::write(dir.join("meta/changes"), "not a directory").unwrap();
        let finding = render_show(&parsed(&["show", "anything"]), &dir).expect_err("must error");
        assert_eq!(finding.code, "CAIRN_CHANGES_DISCOVERY_FAILED");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
