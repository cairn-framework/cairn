//! CLI-only `cairn workspace <status|lint|frontier>` command family.
//!
//! `dec.workspace-aggregation`: `cairn.workspace` declares independent member
//! projects; these commands fold N independent `scanner::load_project` calls
//! into one aggregate view. No shared graph, no cross-project edges (v1
//! scope boundary).
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use std::fmt::Write as _;

use serde_json::json;

use super::super::*;
use crate::{
    map::query::FrontierEntry,
    workspace::{self, MemberStatus, Workspace},
};

const USAGE: &str = "usage: cairn workspace <status|lint|frontier>";

/// Dispatches `cairn workspace <status|lint|frontier>`. Loads `cairn.workspace`
/// from `root` (not `--file`, which names a `cairn.blueprint`).
pub(crate) fn run_workspace_command(parsed: &ParsedArgs, root: &Path) -> CliResult {
    let Some(subcommand) = parsed.command_args.get(1).map(String::as_str) else {
        return err(2, USAGE);
    };
    let workspace_path = root.join("cairn.workspace");
    let workspace = match Workspace::load(&workspace_path) {
        Ok(workspace) => workspace,
        Err(error) => return error_output(parsed.json, "CAIRN_COMMAND_FAILED", &error),
    };
    match subcommand {
        "status" => render_workspace_status(parsed, &workspace),
        "lint" => render_workspace_lint(parsed, &workspace),
        "frontier" => render_workspace_frontier(parsed, &workspace),
        _ => err(2, USAGE),
    }
}

fn member_json(member: &MemberStatus) -> serde_json::Value {
    json!({
        "name": member.name,
        "nodes": member.nodes,
        "edges": member.edges,
        "errors": member.errors,
        "warnings": member.warnings,
        "info": member.info,
        "missing": member.missing.as_ref().map(|finding| finding.message.clone()),
    })
}

fn render_workspace_status(parsed: &ParsedArgs, workspace: &Workspace) -> CliResult {
    let response = workspace::status(workspace);
    if parsed.json {
        let members: Vec<_> = response.members.iter().map(member_json).collect();
        let data = json!({ "members": members, "totals": member_json(&response.totals) });
        return ok(format!("{data}\n"));
    }
    let mut out = String::new();
    for member in &response.members {
        if let Some(finding) = &member.missing {
            let _ = writeln!(out, "{}: MISSING ({})", member.name, finding.message);
        } else {
            let _ = writeln!(
                out,
                "{}: {} nodes, {} edges, {} errors, {} warnings, {} info",
                member.name,
                member.nodes,
                member.edges,
                member.errors,
                member.warnings,
                member.info
            );
        }
    }
    let _ = writeln!(
        out,
        "TOTAL: {} nodes, {} edges, {} errors, {} warnings, {} info",
        response.totals.nodes,
        response.totals.edges,
        response.totals.errors,
        response.totals.warnings,
        response.totals.info
    );
    ok(out)
}

fn render_workspace_lint(parsed: &ParsedArgs, workspace: &Workspace) -> CliResult {
    let findings = workspace::lint(workspace);
    let has_error = findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Error);
    let has_warning = findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Warning);
    let code = if parsed.strict {
        u8::from(has_error || has_warning)
    } else {
        u8::from(has_error)
    };
    CliResult {
        code,
        stdout: render_findings(&findings, parsed.json),
        stderr: String::new(),
    }
}

fn frontier_entry_json(entry: &FrontierEntry) -> serde_json::Value {
    json!({
        "node": entry.node,
        "name": entry.name,
        "tier": entry.tier,
        "has_contract": entry.has_contract,
        "blocking": entry.blocking,
    })
}

fn render_workspace_frontier(parsed: &ParsedArgs, workspace: &Workspace) -> CliResult {
    let response = workspace::frontier(workspace);
    let has_error = response
        .findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Error);
    let code = u8::from(has_error);
    if parsed.json {
        let data = json!({
            "ready": response.ready.iter().map(frontier_entry_json).collect::<Vec<_>>(),
            "blocked": response.blocked.iter().map(frontier_entry_json).collect::<Vec<_>>(),
            "findings": response.findings.iter().map(|finding| json!({
                "code": finding.code,
                "severity": finding.severity.name(),
                "message": finding.message,
            })).collect::<Vec<_>>(),
        });
        return CliResult {
            code,
            stdout: format!("{data}\n"),
            stderr: String::new(),
        };
    }
    let mut out = String::from("Ready:\n");
    if response.ready.is_empty() {
        out.push_str("  (none)\n");
    } else {
        for entry in &response.ready {
            let _ = writeln!(out, "  {} (tier {})", entry.node, entry.tier);
        }
    }
    out.push_str("\nBlocked:\n");
    if response.blocked.is_empty() {
        out.push_str("  (none)\n");
    } else {
        for entry in &response.blocked {
            let _ = writeln!(
                out,
                "  {}: blocked by {}",
                entry.node,
                entry.blocking.join(", ")
            );
        }
    }
    if !response.findings.is_empty() {
        out.push_str("\nFindings:\n");
        for finding in &response.findings {
            let _ = writeln!(out, "  [{}] {}", finding.code, finding.message);
        }
    }
    CliResult {
        code,
        stdout: out,
        stderr: String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SINGLE_MODULE_BLUEPRINT: &str = r#"System Demo "Demo" id "demo" {
    Module Api "Api" id "demo.api" {
        path "./src/api.rs"
    }
}
"#;

    fn workspace_args(args: &[&str]) -> ParsedArgs {
        ParsedArgs {
            json: false,
            strict: false,
            file: PathBuf::from("cairn.blueprint"),
            changes_dir: PathBuf::from("meta/changes"),
            command: "workspace".to_owned(),
            command_args: args.iter().map(|s| (*s).to_owned()).collect(),
        }
    }

    fn write_project(root: &Path) {
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("cairn.blueprint"), SINGLE_MODULE_BLUEPRINT).unwrap();
        fs::write(root.join("src/api.rs"), "fn api() {}\n").unwrap();
    }

    #[test]
    fn run_workspace_command_missing_workspace_file_reports_error() {
        let dir = tempfile::tempdir().unwrap();
        let parsed = workspace_args(&["workspace", "status"]);
        let result = run_workspace_command(&parsed, dir.path());
        assert_eq!(result.code, 1);
        assert!(result.stdout.contains("CAIRN_COMMAND_FAILED"));
    }

    #[test]
    fn run_workspace_command_no_subcommand_is_usage_error() {
        let dir = tempfile::tempdir().unwrap();
        let parsed = workspace_args(&["workspace"]);
        let result = run_workspace_command(&parsed, dir.path());
        assert_eq!(result.code, 2);
        assert!(result.stderr.contains("usage: cairn workspace"));
    }

    #[test]
    fn run_workspace_command_status_single_member_root_dot() {
        let dir = tempfile::tempdir().unwrap();
        write_project(dir.path());
        fs::write(
            dir.path().join("cairn.workspace"),
            "[[project]]\nname = \"root\"\nroot = \".\"\n",
        )
        .unwrap();
        let parsed = workspace_args(&["workspace", "status"]);
        let result = run_workspace_command(&parsed, dir.path());
        assert_eq!(result.code, 0, "stdout: {}", result.stdout);
        assert!(result.stdout.contains("root: 2 nodes"));
        assert!(result.stdout.contains("TOTAL: 2 nodes"));
    }

    #[test]
    fn run_workspace_command_status_json_reports_missing_member() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(
            dir.path().join("cairn.workspace"),
            "[[project]]\nname = \"gone\"\nroot = \"does-not-exist\"\n",
        )
        .unwrap();
        let mut parsed = workspace_args(&["workspace", "status"]);
        parsed.json = true;
        let result = run_workspace_command(&parsed, dir.path());
        assert_eq!(result.code, 0);
        assert!(
            result
                .stdout
                .contains("\"missing\":\"workspace member 'gone' failed to load"),
            "stdout: {}",
            result.stdout
        );
    }

    #[test]
    fn run_workspace_command_frontier_qualifies_nodes() {
        let dir = tempfile::tempdir().unwrap();
        let member_root = dir.path().join("proj");
        fs::create_dir_all(member_root.join("src")).unwrap();
        fs::write(
            member_root.join("cairn.blueprint"),
            "System Demo \"d\" id \"demo\" {\n    Module Db \"d\" id \"demo.db\" {\n        path \"./src/db.rs\"\n    }\n}\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("cairn.workspace"),
            "[[project]]\nname = \"proj\"\nroot = \"proj\"\n",
        )
        .unwrap();
        let parsed = workspace_args(&["workspace", "frontier"]);
        let result = run_workspace_command(&parsed, dir.path());
        assert_eq!(result.code, 0, "stdout: {}", result.stdout);
        assert!(result.stdout.contains("proj:demo.db"));
    }
}
