//! Multi-project workspace aggregation (`cairn.workspace`).
//!
//! `dec.workspace-aggregation`: `cairn.workspace` declares a set of
//! independent member projects, each with its own `cairn.blueprint`.
//! Workspace commands fold N independent [`scanner::load_project`] calls
//! into one aggregate view. There is no shared graph and no cross-project
//! edges; that is a v1 scope boundary, not a placeholder.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::{
    map::{
        graph::{Finding, FindingSeverity},
        query::{self, FrontierEntry},
    },
    scanner,
};

/// Emitted for a workspace member whose root or blueprint fails to load.
pub const MEMBER_MISSING: &str = "CAIRN_WORKSPACE_MEMBER_MISSING";

/// One declared workspace member.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceProject {
    /// Member name, as declared in `cairn.workspace`.
    pub name: String,
    /// Member project root, resolved relative to the workspace file.
    pub root: PathBuf,
}

/// A parsed `cairn.workspace` file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workspace {
    /// Declared member projects, in file order.
    pub projects: Vec<WorkspaceProject>,
}

#[derive(Deserialize)]
struct WorkspaceToml {
    project: Vec<ProjectToml>,
}

#[derive(Deserialize)]
struct ProjectToml {
    name: String,
    root: String,
}

impl Workspace {
    /// Loads and parses a `cairn.workspace` TOML file. Each member's `root`
    /// is resolved relative to the workspace file's own directory.
    ///
    /// # Errors
    ///
    /// Returns an error string when the file cannot be read, or does not
    /// parse as a `cairn.workspace` document (`[[project]] name / root`).
    pub fn load(path: &Path) -> Result<Self, String> {
        let source = std::fs::read_to_string(path)
            .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
        let parsed: WorkspaceToml = toml::from_str(&source)
            .map_err(|error| format!("failed to parse {}: {error}", path.display()))?;
        let base = path.parent().unwrap_or_else(|| Path::new("."));
        let projects = parsed
            .project
            .into_iter()
            .map(|project| WorkspaceProject {
                name: project.name,
                root: base.join(project.root),
            })
            .collect();
        Ok(Self { projects })
    }
}

/// Loads one member's scan, or synthesises a [`MEMBER_MISSING`] finding when
/// its root or blueprint cannot be loaded.
fn load_member(project: &WorkspaceProject) -> Result<scanner::ScanResult, Finding> {
    let blueprint_path = project.root.join("cairn.blueprint");
    scanner::load_project(&project.root, &blueprint_path).map_err(|error| Finding {
        code: "CAIRN_WORKSPACE_MEMBER_MISSING".to_owned(),
        severity: FindingSeverity::Error,
        message: format!(
            "workspace member '{}' failed to load: {error}",
            project.name
        ),
        node: None,
        target: None,
        path: Some(project.root.display().to_string()),
    })
}

fn count_severities(findings: &[Finding]) -> (usize, usize, usize) {
    let mut errors = 0;
    let mut warnings = 0;
    let mut info = 0;
    for finding in findings {
        match finding.severity {
            FindingSeverity::Error => errors += 1,
            FindingSeverity::Warning => warnings += 1,
            FindingSeverity::Info => info += 1,
        }
    }
    (errors, warnings, info)
}

/// One member's row in a [`status`] response (or the summed totals row).
pub struct MemberStatus {
    /// Member name, or `"TOTAL"` for the summed row.
    pub name: String,
    /// Node count (0 when the member failed to load).
    pub nodes: usize,
    /// Dependency edge count (0 when the member failed to load).
    pub edges: usize,
    /// Error-severity finding count.
    pub errors: usize,
    /// Warning-severity finding count.
    pub warnings: usize,
    /// Info-severity finding count.
    pub info: usize,
    /// Set when the member failed to load; other counts are zero except
    /// `errors`, which is 1 (the synthesised [`MEMBER_MISSING`] finding).
    pub missing: Option<Finding>,
}

/// `cairn workspace status`: per-member counts plus a summed totals row.
pub struct WorkspaceStatus {
    /// One row per declared member, in workspace file order.
    pub members: Vec<MemberStatus>,
    /// Sum of every member's counts.
    pub totals: MemberStatus,
}

/// Aggregates node/edge/finding counts across every workspace member.
///
/// A member whose root or blueprint fails to load contributes a zeroed row
/// with `missing` set and one error to the totals; the loop continues with
/// the remaining members.
#[must_use]
pub fn status(workspace: &Workspace) -> WorkspaceStatus {
    let members: Vec<MemberStatus> = workspace
        .projects
        .iter()
        .map(|project| match load_member(project) {
            Ok(scan) => {
                let (errors, warnings, info) = count_severities(&scan.graph.findings);
                MemberStatus {
                    name: project.name.clone(),
                    nodes: scan.graph.nodes.len(),
                    edges: scan.graph.outbound.values().map(Vec::len).sum(),
                    errors,
                    warnings,
                    info,
                    missing: None,
                }
            }
            Err(finding) => MemberStatus {
                name: project.name.clone(),
                nodes: 0,
                edges: 0,
                errors: 1,
                warnings: 0,
                info: 0,
                missing: Some(finding),
            },
        })
        .collect();
    let totals = MemberStatus {
        name: "TOTAL".to_owned(),
        nodes: members.iter().map(|m| m.nodes).sum(),
        edges: members.iter().map(|m| m.edges).sum(),
        errors: members.iter().map(|m| m.errors).sum(),
        warnings: members.iter().map(|m| m.warnings).sum(),
        info: members.iter().map(|m| m.info).sum(),
        missing: None,
    };
    WorkspaceStatus { members, totals }
}

/// `cairn workspace lint`: every member's findings, each message prefixed
/// `<project>: `. A member that fails to load contributes one
/// [`MEMBER_MISSING`] finding instead of its (unavailable) findings, and the
/// loop continues with the remaining members.
#[must_use]
pub fn lint(workspace: &Workspace) -> Vec<Finding> {
    let mut findings = Vec::new();
    for project in &workspace.projects {
        match load_member(project) {
            Ok(scan) => {
                for finding in &scan.graph.findings {
                    findings.push(Finding {
                        message: format!("{}: {}", project.name, finding.message),
                        ..finding.clone()
                    });
                }
            }
            Err(finding) => findings.push(finding),
        }
    }
    findings
}

/// `cairn workspace frontier`: every member's [`query::frontier`] result,
/// with entry node IDs (and blocking dependency IDs) qualified
/// `<project>:<node>`.
pub struct WorkspaceFrontier {
    /// Ready entries across every member, project-qualified.
    pub ready: Vec<FrontierEntry>,
    /// Blocked entries across every member, project-qualified.
    pub blocked: Vec<FrontierEntry>,
    /// One finding per member that failed to load or whose graph is
    /// cyclic; the loop continues with the remaining members.
    pub findings: Vec<Finding>,
}

/// Aggregates the buildable-now / blocked frontier across every workspace
/// member. A member that fails to load, or whose dependency graph is
/// cyclic, contributes findings instead of frontier entries and is skipped.
#[must_use]
pub fn frontier(workspace: &Workspace) -> WorkspaceFrontier {
    let mut ready = Vec::new();
    let mut blocked = Vec::new();
    let mut findings = Vec::new();
    for project in &workspace.projects {
        match load_member(project) {
            Ok(scan) => match query::frontier(&scan.graph) {
                Ok(response) => {
                    ready.extend(
                        response
                            .ready
                            .into_iter()
                            .map(|entry| qualify(&project.name, entry)),
                    );
                    blocked.extend(
                        response
                            .blocked
                            .into_iter()
                            .map(|entry| qualify(&project.name, entry)),
                    );
                }
                Err(member_findings) => {
                    findings.extend(member_findings.into_iter().map(|finding| Finding {
                        message: format!("{}: {}", project.name, finding.message),
                        ..finding
                    }));
                }
            },
            Err(finding) => findings.push(finding),
        }
    }
    WorkspaceFrontier {
        ready,
        blocked,
        findings,
    }
}

fn qualify(project: &str, mut entry: FrontierEntry) -> FrontierEntry {
    entry.node = format!("{project}:{}", entry.node);
    entry.blocking = entry
        .blocking
        .iter()
        .map(|id| format!("{project}:{id}"))
        .collect();
    entry
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn write_project(root: &Path, blueprint: &str) {
        fs::create_dir_all(root).unwrap();
        fs::write(root.join("cairn.blueprint"), blueprint).unwrap();
    }

    const SINGLE_MODULE_BLUEPRINT: &str = r#"System Demo "Demo" id "demo" {
    Module Api "Api" id "demo.api" {
        path "./src/api.rs"
    }
}
"#;

    fn write_workspace(dir: &Path, body: &str) -> PathBuf {
        fs::create_dir_all(dir).unwrap();
        let path = dir.join("cairn.workspace");
        fs::write(&path, body).unwrap();
        path
    }

    #[test]
    fn load_parses_single_member() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_workspace(
            dir.path(),
            r#"[[project]]
name = "root"
root = "."
"#,
        );
        let workspace = Workspace::load(&path).expect("must parse");
        assert_eq!(workspace.projects.len(), 1);
        assert_eq!(workspace.projects[0].name, "root");
        assert_eq!(workspace.projects[0].root, dir.path());
    }

    #[test]
    fn status_two_members_sums_totals() {
        let dir = tempfile::tempdir().unwrap();
        write_project(&dir.path().join("a"), SINGLE_MODULE_BLUEPRINT);
        fs::create_dir_all(dir.path().join("a/src")).unwrap();
        fs::write(dir.path().join("a/src/api.rs"), "fn api() {}\n").unwrap();
        write_project(&dir.path().join("b"), SINGLE_MODULE_BLUEPRINT);
        fs::create_dir_all(dir.path().join("b/src")).unwrap();
        fs::write(dir.path().join("b/src/api.rs"), "fn api() {}\n").unwrap();

        let workspace = Workspace {
            projects: vec![
                WorkspaceProject {
                    name: "a".to_owned(),
                    root: dir.path().join("a"),
                },
                WorkspaceProject {
                    name: "b".to_owned(),
                    root: dir.path().join("b"),
                },
            ],
        };
        let response = status(&workspace);
        assert_eq!(response.members.len(), 2);
        assert!(response.members.iter().all(|m| m.missing.is_none()));
        assert_eq!(response.members[0].nodes, 2);
        assert_eq!(response.members[1].nodes, 2);
        assert_eq!(response.totals.nodes, 4);
    }

    #[test]
    fn status_missing_member_root_reports_error_other_member_still_counted() {
        let dir = tempfile::tempdir().unwrap();
        write_project(&dir.path().join("present"), SINGLE_MODULE_BLUEPRINT);
        fs::create_dir_all(dir.path().join("present/src")).unwrap();
        fs::write(dir.path().join("present/src/api.rs"), "fn api() {}\n").unwrap();

        let workspace = Workspace {
            projects: vec![
                WorkspaceProject {
                    name: "present".to_owned(),
                    root: dir.path().join("present"),
                },
                WorkspaceProject {
                    name: "gone".to_owned(),
                    root: dir.path().join("does-not-exist"),
                },
            ],
        };
        let response = status(&workspace);
        assert_eq!(response.members.len(), 2);
        assert!(response.members[0].missing.is_none());
        assert_eq!(response.members[0].nodes, 2);
        let missing = response.members[1]
            .missing
            .as_ref()
            .expect("gone member must be missing");
        assert_eq!(missing.code, MEMBER_MISSING);
        assert_eq!(response.totals.nodes, 2, "present member still counted");
        assert_eq!(response.totals.errors, 1);
    }

    #[test]
    fn lint_missing_member_emits_member_missing_finding() {
        let dir = tempfile::tempdir().unwrap();
        let workspace = Workspace {
            projects: vec![WorkspaceProject {
                name: "gone".to_owned(),
                root: dir.path().join("does-not-exist"),
            }],
        };
        let findings = lint(&workspace);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].code, MEMBER_MISSING);
        assert_eq!(findings[0].severity, FindingSeverity::Error);
    }

    #[test]
    fn frontier_qualifies_entries_with_project_prefix() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("proj");
        write_project(
            &root,
            r#"System Demo "Demo" id "demo" {
    Module Api "Api" id "demo.api" {
        path "./src/api.rs"
    }
    Module Db "Db" id "demo.db" {
        path "./src/db.rs"
    }
}

demo.db -> demo.api "dep"
"#,
        );
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/api.rs"), "fn api() {}\n").unwrap();
        let workspace = Workspace {
            projects: vec![WorkspaceProject {
                name: "proj".to_owned(),
                root,
            }],
        };
        let response = frontier(&workspace);
        assert!(response.findings.is_empty());
        assert_eq!(response.ready.len(), 1);
        assert_eq!(response.ready[0].node, "proj:demo.db");
        assert!(response.blocked.is_empty());
    }
}
