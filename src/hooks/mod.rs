//! Hook engine and active-change conflict detection.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    time::Instant,
};

mod architecture;
mod render;

pub use render::{render_human, render_json};

use crate::{
    map::{FindingSeverity, graph::Finding, query},
    scanner,
};

/// Hook enforcement class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HookKind {
    /// Blocks on structural errors.
    Structural,
    /// Blocks on interface contradictions.
    Interface,
    /// Reports rationale tensions without blocking.
    Tension,
    /// Blocks on blueprint architectural mutations lacking paired decisions.
    ArchitectureDecision,
    /// Runs all hook classes with combined blocking semantics.
    All,
}

/// Final hook exit decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExitDecision {
    /// Hook passes.
    Pass,
    /// Hook blocks the caller.
    Block,
}

/// Hook execution report.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HookReport {
    /// Requested hook kind.
    pub kind: HookKind,
    /// Scanner and lint findings selected for this hook.
    pub findings: Vec<Finding>,
    /// Active-change conflict findings.
    pub conflict_findings: Vec<Finding>,
    /// Exit decision.
    pub decision: ExitDecision,
    /// Elapsed runtime in milliseconds.
    pub elapsed_ms: u128,
    /// Output paths touched by the hook engine.
    pub output_paths: Vec<String>,
}

impl HookReport {
    /// Process exit code for this report.
    #[must_use]
    pub const fn exit_code(&self) -> u8 {
        match self.decision {
            ExitDecision::Pass => 0,
            ExitDecision::Block => 1,
        }
    }
}

/// Runs a hook against an already loaded project.
#[must_use]
pub fn run(
    kind: HookKind,
    root: &Path,
    changes_dir: &Path,
    scan_result: &scanner::ScanResult,
) -> HookReport {
    let started = Instant::now();
    let lint_findings = query::lint(&scan_result.graph).findings;
    let structural = structural_findings(&lint_findings);
    let interface = interface_findings(root, &scan_result.target_hashes);
    let tensions = tension_findings(&lint_findings);
    let conflict_findings = detect_active_change_conflicts(changes_dir);
    let architecture = architecture::architecture_findings_from_project(root);
    let findings = match kind {
        HookKind::Structural => structural
            .iter()
            .cloned()
            .chain(conflict_findings.iter().cloned())
            .collect(),
        HookKind::Interface => interface.clone(),
        HookKind::Tension => tensions.clone(),
        HookKind::ArchitectureDecision => architecture.clone(),
        HookKind::All => structural
            .iter()
            .cloned()
            .chain(interface.iter().cloned())
            .chain(tensions.iter().cloned())
            .chain(conflict_findings.iter().cloned())
            .chain(architecture.iter().cloned())
            .collect(),
    };
    let blocks = match kind {
        HookKind::Structural => !structural.is_empty() || !conflict_findings.is_empty(),
        HookKind::Interface => !interface.is_empty(),
        HookKind::Tension => false,
        HookKind::ArchitectureDecision => !architecture.is_empty(),
        HookKind::All => {
            !structural.is_empty()
                || !interface.is_empty()
                || !conflict_findings.is_empty()
                || !architecture.is_empty()
        }
    };
    HookReport {
        kind,
        findings,
        conflict_findings,
        decision: if blocks {
            ExitDecision::Block
        } else {
            ExitDecision::Pass
        },
        elapsed_ms: started.elapsed().as_millis(),
        output_paths: Vec::new(),
    }
}

/// Returns active-change conflicts as structural findings.
#[must_use]
pub fn detect_active_change_conflicts(changes_dir: &Path) -> Vec<Finding> {
    let changes = discover_changes(changes_dir);
    let mut findings = Vec::new();
    detect_duplicate_targets(
        changes.iter().flat_map(|change| {
            change
                .blueprint_targets
                .iter()
                .map(move |target| (target, change))
        }),
        "CAIRN_CHANGE_BLUEPRINT_CONFLICT",
        "blueprint operation",
        &mut findings,
    );
    detect_duplicate_targets(
        changes.iter().flat_map(|change| {
            change
                .artefact_targets
                .iter()
                .map(move |target| (target, change))
        }),
        "CAIRN_CHANGE_ARTEFACT_CONFLICT",
        "artefact operation",
        &mut findings,
    );
    detect_duplicate_targets(
        changes.iter().flat_map(|change| {
            change
                .rename_targets
                .iter()
                .map(move |target| (target, change))
        }),
        "CAIRN_CHANGE_RENAME_CONFLICT",
        "rename operation",
        &mut findings,
    );
    findings
}

fn structural_findings(findings: &[Finding]) -> Vec<Finding> {
    findings
        .iter()
        .filter(|finding| finding.severity == FindingSeverity::Error)
        .cloned()
        .collect()
}

fn tension_findings(findings: &[Finding]) -> Vec<Finding> {
    // Cycle 3: include both Warning and Info severities so the
    // advisory channel still surfaces non-blocking signals (e.g.,
    // CAIRN_SOURCE_UNVERIFIED, which phase 7.7 demoted from Warning to
    // Info per the FindingSeverity::Info kernel addition).
    findings
        .iter()
        .filter(|finding| {
            matches!(
                finding.severity,
                FindingSeverity::Warning | FindingSeverity::Info
            )
        })
        .cloned()
        .collect()
}

fn interface_findings(root: &Path, current: &scanner::state::TargetHashes) -> Vec<Finding> {
    let state_path = root.join(".cairn/state/interface-hashes.json");
    let Ok(recorded) = scanner::state::read_interface_hash(root) else {
        return Vec::new();
    };
    if recorded.is_empty() || &recorded == current {
        Vec::new()
    } else {
        vec![Finding {
            code: "CAIRN_INTERFACE_HASH_CHANGED".to_owned(),
            severity: FindingSeverity::Error,
            message: "current interface hash differs from recorded state".to_owned(),
            node: None,
            target: None,
            path: Some(path_string(&state_path)),
        }]
    }
}

#[derive(Clone, Debug)]
struct ChangeSummary {
    id: String,
    path: PathBuf,
    blueprint_targets: BTreeSet<String>,
    artefact_targets: BTreeSet<String>,
    rename_targets: BTreeSet<String>,
}

fn discover_changes(changes_dir: &Path) -> Vec<ChangeSummary> {
    let Ok(entries) = fs::read_dir(changes_dir) else {
        return Vec::new();
    };
    let mut changes = entries
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            if !path.is_dir()
                || path.file_name().and_then(|name| name.to_str()) == Some("archive")
                || !path.join("proposal.md").exists()
            {
                return None;
            }
            Some(parse_change(&path))
        })
        .collect::<Vec<_>>();
    changes.sort_by(|left, right| left.id.cmp(&right.id));
    changes
}

fn parse_change(path: &Path) -> ChangeSummary {
    let id = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown")
        .to_owned();
    let (blueprint_targets, rename_targets) = parse_blueprint_delta(&path.join("blueprint.delta"));
    let mut summary = ChangeSummary {
        id,
        path: path.to_path_buf(),
        blueprint_targets,
        artefact_targets: BTreeSet::new(),
        rename_targets,
    };
    parse_artefacts(path, &mut summary);
    summary
}

fn parse_blueprint_delta(path: &Path) -> (BTreeSet<String>, BTreeSet<String>) {
    let Ok(content) = fs::read_to_string(path) else {
        return (BTreeSet::new(), BTreeSet::new());
    };
    let mut section = String::new();
    let mut targets = BTreeSet::new();
    let mut rename_targets = BTreeSet::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("## ") {
            section = trimmed.trim_start_matches('#').trim().to_ascii_lowercase();
            continue;
        }
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if section.contains("renamed") {
            let pair = parse_edge(trimmed).or_else(|| {
                let ids = ids_from_text(trimmed);
                (ids.len() >= 2).then(|| (ids[0].clone(), ids[1].clone()))
            });
            if let Some((old_id, new_id)) = pair {
                rename_targets.insert(format!("rename-old:{old_id}"));
                rename_targets.insert(format!("rename-new:{new_id}"));
                targets.insert(format!("node:{old_id}"));
                targets.insert(format!("node:{new_id}"));
            }
        } else if section.contains("edge") {
            if let Some((from, to)) = parse_edge(trimmed) {
                targets.insert(format!("edge:{from}->{to}"));
            }
        } else {
            for id in ids_from_text(trimmed) {
                targets.insert(format!("node:{id}"));
            }
        }
    }
    (targets, rename_targets)
}

fn parse_edge(line: &str) -> Option<(String, String)> {
    let (from, rest) = line.split_once("->")?;
    let to = rest.split_whitespace().next()?;
    Some((clean_id(from), clean_id(to)))
}

fn ids_from_text(text: &str) -> Vec<String> {
    let mut ids = Vec::new();
    if let Some(id) = field_value(text, "id")
        && !id.is_empty()
    {
        ids.push(id);
    }
    ids.extend(
        text.split(|character: char| {
            character.is_whitespace()
                || matches!(
                    character,
                    '"' | '\'' | '[' | ']' | '(' | ')' | ',' | ':' | ';'
                )
        })
        .map(clean_id)
        .filter(|token| token.contains('.') && token.chars().all(is_id_char)),
    );
    ids.sort();
    ids.dedup();
    ids
}

fn field_value(text: &str, field: &str) -> Option<String> {
    let needle = format!("{field} \"");
    let start = text.find(&needle)? + needle.len();
    let end = text[start..].find('"')? + start;
    Some(text[start..end].to_owned())
}

fn clean_id(value: &str) -> String {
    value
        .trim()
        .trim_matches('"')
        .trim_matches('`')
        .trim_matches(',')
        .to_owned()
}

const fn is_id_char(character: char) -> bool {
    character.is_ascii_lowercase()
        || character.is_ascii_digit()
        || character == '.'
        || character == '-'
        || character == '_'
}

fn parse_artefacts(change_path: &Path, summary: &mut ChangeSummary) {
    let meta = change_path.join("meta");
    let mut stack = vec![meta];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
                parse_artefact(change_path, &path, summary);
            }
        }
    }
}

fn parse_artefact(change_path: &Path, path: &Path, summary: &mut ChangeSummary) {
    let relative = path
        .strip_prefix(change_path)
        .map_or_else(|_| path_string(path), path_string);
    let Ok(content) = fs::read_to_string(path) else {
        return;
    };
    let operation =
        frontmatter_value(&content, "operation").unwrap_or_else(|| "modified".to_owned());
    let target = match operation.as_str() {
        "renamed" => frontmatter_value(&content, "renamed_from").map_or_else(
            || format!("artefact:{relative}"),
            |from| format!("artefact:{from}"),
        ),
        _ => format!("artefact:{relative}"),
    };
    if operation == "renamed" {
        summary
            .rename_targets
            .insert(format!("rename-old:{target}"));
        summary
            .rename_targets
            .insert(format!("rename-new:artefact:{relative}"));
    }
    summary.artefact_targets.insert(target);
}

fn frontmatter_value(content: &str, key: &str) -> Option<String> {
    let mut lines = content.lines();
    if lines.next()? != "---" {
        return None;
    }
    for line in lines {
        if line == "---" {
            return None;
        }
        if let Some((candidate, value)) = line.split_once(':')
            && candidate.trim() == key
        {
            return Some(value.trim().trim_matches('"').to_owned());
        }
    }
    None
}

fn detect_duplicate_targets<'a>(
    items: impl Iterator<Item = (&'a String, &'a ChangeSummary)>,
    code: &str,
    label: &str,
    findings: &mut Vec<Finding>,
) {
    let mut owners: BTreeMap<&str, Vec<&ChangeSummary>> = BTreeMap::new();
    for (target, change) in items {
        owners.entry(target.as_str()).or_default().push(change);
    }
    for (target, changes) in owners {
        if changes.len() < 2 {
            continue;
        }
        let ids = changes
            .iter()
            .map(|change| change.id.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        findings.push(Finding {
            code: code.to_owned(),
            severity: FindingSeverity::Error,
            message: format!("{label} `{target}` is claimed by active changes: {ids}"),
            node: None,
            target: None,
            path: Some(path_string(&changes[0].path)),
        });
    }
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::graph::FindingSeverity;

    fn finding(severity: FindingSeverity, code: &str) -> Finding {
        Finding {
            code: code.to_owned(),
            severity,
            message: "test".to_owned(),
            node: None,
            target: None,
            path: None,
        }
    }

    // ── structural_findings / tension_findings ────────────────────────────────

    #[test]
    fn test_structural_findings_keeps_only_errors() {
        let findings = vec![
            finding(FindingSeverity::Error, "ERR"),
            finding(FindingSeverity::Warning, "WARN"),
            finding(FindingSeverity::Info, "INFO"),
        ];
        let out = structural_findings(&findings);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].code, "ERR");
    }

    #[test]
    fn test_tension_findings_keeps_warning_and_info_not_error() {
        let findings = vec![
            finding(FindingSeverity::Error, "ERR"),
            finding(FindingSeverity::Warning, "WARN"),
            finding(FindingSeverity::Info, "INFO"),
        ];
        let out = tension_findings(&findings);
        assert_eq!(out.len(), 2);
        let codes: Vec<&str> = out.iter().map(|f| f.code.as_str()).collect();
        assert!(codes.contains(&"WARN"));
        assert!(codes.contains(&"INFO"));
        assert!(!codes.contains(&"ERR"));
    }

    // ── is_id_char ────────────────────────────────────────────────────────────

    #[test]
    fn test_is_id_char_accepts_valid_chars() {
        for ch in "abcz0189.-_".chars() {
            assert!(is_id_char(ch), "expected {ch:?} to be a valid id char");
        }
    }

    #[test]
    fn test_is_id_char_rejects_invalid_chars() {
        for ch in "ABCZ /\\@#".chars() {
            assert!(!is_id_char(ch), "expected {ch:?} to be invalid id char");
        }
    }

    // ── clean_id ──────────────────────────────────────────────────────────────

    #[test]
    fn test_clean_id_trims_whitespace() {
        assert_eq!(clean_id("  app.api  "), "app.api");
    }

    #[test]
    fn test_clean_id_strips_double_quotes() {
        assert_eq!(clean_id(r#""app.api""#), "app.api");
    }

    #[test]
    fn test_clean_id_strips_backticks() {
        assert_eq!(clean_id("`app.api`"), "app.api");
    }

    #[test]
    fn test_clean_id_strips_trailing_comma() {
        assert_eq!(clean_id("app.api,"), "app.api");
    }

    #[test]
    fn test_clean_id_plain_id_unchanged() {
        assert_eq!(clean_id("app.api"), "app.api");
    }

    // ── field_value ───────────────────────────────────────────────────────────

    #[test]
    fn test_field_value_extracts_quoted_value() {
        assert_eq!(
            field_value(r#"Module Api id "app.api" {"#, "id"),
            Some("app.api".to_owned())
        );
    }

    #[test]
    fn test_field_value_absent_field_returns_none() {
        assert_eq!(field_value("Module Api {}", "id"), None);
    }

    // ── parse_edge ────────────────────────────────────────────────────────────

    #[test]
    fn test_parse_edge_with_spaces() {
        assert_eq!(
            parse_edge("app.api -> app.db"),
            Some(("app.api".to_owned(), "app.db".to_owned()))
        );
    }

    #[test]
    fn test_parse_edge_without_spaces() {
        assert_eq!(
            parse_edge("app.api->app.db"),
            Some(("app.api".to_owned(), "app.db".to_owned()))
        );
    }

    #[test]
    fn test_parse_edge_with_description_ignores_description() {
        assert_eq!(
            parse_edge(r#"app.api -> app.db "dep""#),
            Some(("app.api".to_owned(), "app.db".to_owned()))
        );
    }

    #[test]
    fn test_parse_edge_no_arrow_returns_none() {
        assert_eq!(parse_edge("app.api app.db"), None);
    }

    // ── ids_from_text ─────────────────────────────────────────────────────────

    #[test]
    fn test_ids_from_text_extracts_dotted_id_from_bullet() {
        let ids = ids_from_text("- app.api");
        assert_eq!(ids, vec!["app.api"]);
    }

    #[test]
    fn test_ids_from_text_extracts_id_field() {
        let ids = ids_from_text(r#"Module Api id "app.api" {"#);
        assert_eq!(ids, vec!["app.api"]);
    }

    #[test]
    fn test_ids_from_text_no_dotted_id_returns_empty() {
        let ids = ids_from_text("# ADDED Nodes");
        assert!(ids.is_empty(), "heading must produce no ids: {ids:?}");
    }

    #[test]
    fn test_ids_from_text_empty_id_field_not_included() {
        // field_value returns Some("") for id "".
        // The empty string must not appear in the output — it is not a
        // valid node ID and would insert "node:" into the targets set.
        let ids = ids_from_text(r#"Module id """#);
        assert!(
            !ids.contains(&String::new()),
            "empty string must not be in ids output: {ids:?}"
        );
    }
}
