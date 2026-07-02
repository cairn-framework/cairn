//! Architecture decision gate: blueprint architectural mutations require
//! paired decision artefacts.

use std::collections::BTreeSet;
use std::path::Path;

use crate::artefacts::registry::{Decision, DecisionStatus};
use crate::blueprint::{Ast, Node, NodeKind};
use crate::map::{Finding, FindingSeverity};

/// A module's location in the blueprint hierarchy.
#[derive(Clone, Debug, Eq, PartialEq)]
struct ModuleLocation {
    id: String,
    paths: Vec<String>,
    container_id: String,
}

/// Returns findings for blueprint architectural mutations that lack a paired
/// decision artefact.
///
/// The gate fires on:
/// - module-add (new module ID appears)
/// - module-remove (existing module ID disappears)
/// - path-reassign-across-containers (module moves to a different container)
///
/// The gate ignores:
/// - path-rename-within-container (path changes, container stays)
/// - reordering modules within a container
/// - casing fixes
///
/// If `escape_hatch` is `true`, the gate is bypassed entirely.
pub fn architecture_findings(
    old_ast: &Ast,
    new_ast: &Ast,
    decisions: &[Decision],
    escape_hatch: bool,
) -> Vec<Finding> {
    if escape_hatch {
        return Vec::new();
    }

    let old_modules = collect_modules(old_ast);
    let new_modules = collect_modules(new_ast);

    let old_ids: BTreeSet<&str> = old_modules.iter().map(|m| m.id.as_str()).collect();
    let new_ids: BTreeSet<&str> = new_modules.iter().map(|m| m.id.as_str()).collect();

    let mut findings = Vec::new();

    // Module additions.
    for module in &new_modules {
        if !old_ids.contains(module.id.as_str()) && !has_decision_for(decisions, &module.id) {
            findings.push(Finding {
                code: "CH001".to_owned(),
                severity: FindingSeverity::Error,
                message: format!(
                    "module `{}` was added without a paired decision artefact referencing `{}`",
                    module.id, module.id
                ),
                node: Some(module.id.clone()),
                target: None,
                path: Some("cairn.blueprint".to_owned()),
            });
        }
    }

    // Module removals.
    for module in &old_modules {
        if !new_ids.contains(module.id.as_str()) && !has_decision_for(decisions, &module.id) {
            findings.push(Finding {
                code: "CH001".to_owned(),
                severity: FindingSeverity::Error,
                message: format!(
                    "module `{}` was removed without a paired decision artefact referencing `{}`",
                    module.id, module.id
                ),
                node: Some(module.id.clone()),
                target: None,
                path: Some("cairn.blueprint".to_owned()),
            });
        }
    }

    // Path reassign across containers.
    let old_by_id: std::collections::HashMap<&str, &ModuleLocation> =
        old_modules.iter().map(|m| (m.id.as_str(), m)).collect();
    let new_by_id: std::collections::HashMap<&str, &ModuleLocation> =
        new_modules.iter().map(|m| (m.id.as_str(), m)).collect();

    for id in old_ids.intersection(&new_ids) {
        let old_mod = old_by_id[id];
        let new_mod = new_by_id[id];
        if old_mod.container_id != new_mod.container_id && !has_decision_for(decisions, id) {
            findings.push(Finding {
                code: "CH001".to_owned(),
                severity: FindingSeverity::Error,
                message: format!(
                    "module `{id}` was reassigned from container `{}` to `{}` without a paired decision artefact referencing `{id}`",
                    old_mod.container_id, new_mod.container_id
                ),
                node: Some(id.to_string()),
                target: None,
                path: Some("cairn.blueprint".to_owned()),
            });
        }
    }

    findings
}

/// Collect all module nodes from the AST, tracking their parent container.
fn collect_modules(ast: &Ast) -> Vec<ModuleLocation> {
    let mut modules = Vec::new();
    for node in &ast.nodes {
        collect_modules_recursive(node, &node.id, &mut modules);
    }
    modules
}

fn collect_modules_recursive(node: &Node, container_id: &str, modules: &mut Vec<ModuleLocation>) {
    if node.kind == NodeKind::Module {
        modules.push(ModuleLocation {
            id: node.id.clone(),
            paths: node.paths.clone(),
            container_id: container_id.to_owned(),
        });
    }
    let next_container = if node.kind == NodeKind::Container || node.kind == NodeKind::System {
        &node.id
    } else {
        container_id
    };
    for child in &node.children {
        collect_modules_recursive(child, next_container, modules);
    }
}

/// Entry point for the hook runner. Reads the current blueprint from disk,
/// the previous blueprint from `git show HEAD:cairn.blueprint`, and delegates
/// to `architecture_findings`.
pub fn architecture_findings_from_project(root: &Path) -> Vec<Finding> {
    let blueprint_path = root.join("cairn.blueprint");
    let Ok(new_source) = std::fs::read_to_string(&blueprint_path) else {
        return Vec::new();
    };
    let Ok(new_ast) = crate::blueprint::parser::parse_str("cairn.blueprint", &new_source) else {
        return Vec::new();
    };

    let escape_hatch = new_source
        .lines()
        .any(|line| line.trim() == "# decision: trivial");
    let Some(old_ast) = read_head_blueprint(root) else {
        return Vec::new();
    };

    architecture_findings(
        &old_ast,
        &new_ast,
        &load_decisions(root, &new_ast),
        escape_hatch,
    )
}

fn current_head_hash(root: &Path) -> Option<String> {
    let head_ref = std::fs::read_to_string(root.join(".git/HEAD")).ok()?;
    let ref_path = head_ref.trim().strip_prefix("ref: ")?;
    let hash = std::fs::read_to_string(root.join(".git").join(ref_path)).ok()?;
    Some(hash.trim().to_owned())
}

fn read_head_blueprint(root: &Path) -> Option<Ast> {
    // Fast path: if a cached copy exists for the current HEAD, use it.
    let cache_path = root.join(".cairn/state/head-blueprint.cache");
    if let (Some(head), Ok(cache)) = (
        current_head_hash(root),
        std::fs::read_to_string(&cache_path),
    ) {
        let mut lines = cache.lines();
        if lines.next() == Some(&head) {
            let source = lines.collect::<Vec<_>>().join("\n");
            if let Ok(ast) = crate::blueprint::parser::parse_str("HEAD:cairn.blueprint", &source) {
                return Some(ast);
            }
        }
    }

    let output = std::process::Command::new("git")
        .current_dir(root)
        .args(["show", "HEAD:cairn.blueprint"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let source = String::from_utf8(output.stdout).ok()?;
    let ast = crate::blueprint::parser::parse_str("HEAD:cairn.blueprint", &source).ok()?;

    // Write cache for subsequent runs.
    if let Some(head) = current_head_hash(root) {
        let cache_dir = root.join(".cairn/state");
        let _ = std::fs::create_dir_all(&cache_dir);
        let cache_content = format!("{head}\n{source}");
        let _ = std::fs::write(&cache_path, cache_content);
    }

    Some(ast)
}

/// Load parsed decisions from all `decisions` pointers declared in `ast`.
fn load_decisions(root: &Path, ast: &Ast) -> Vec<Decision> {
    let mut set = crate::artefacts::registry::ArtefactSet::default();
    crate::artefacts::registry::load_decisions(root, ast, &mut set);
    set.decisions
}

/// Check if any parsed accepted decision covers the given node ID.
fn has_decision_for(decisions: &[Decision], node_id: &str) -> bool {
    decisions
        .iter()
        .any(|d| d.status == DecisionStatus::Accepted && d.nodes.iter().any(|n| n == node_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blueprint::parser::parse_str;

    fn accepted_for(node_id: &str) -> Decision {
        Decision {
            id: format!("dec.{node_id}"),
            path: format!("meta/decisions/dec.{node_id}.md"),
            nodes: vec![node_id.to_owned()],
            status: DecisionStatus::Accepted,
            date: "2026-06-16".to_owned(),
            revisited: None,
            revisit_triggers: Vec::new(),
            informed_by: Vec::new(),
            supersedes: Vec::new(),
            refines: Vec::new(),
            related: Vec::new(),
            orphaned: false,
            orphan_reason: None,
            gap: false,
            claims: None,
            body: String::new(),
        }
    }

    #[test]
    fn test_gate_fires_on_module_add_without_decision() {
        let old = parse_str("old", "System App \"desc\" id \"app\" {}").unwrap();
        let new = parse_str(
            "new",
            r#"System App "desc" id "app" {
    Module One "one" id "app.one" {
        path "./src/one"
    }
}"#,
        )
        .unwrap();
        let findings = architecture_findings(&old, &new, &[], false);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].code, "CH001");
        assert!(findings[0].message.contains("app.one"));
        assert!(findings[0].message.contains("added"));
    }

    #[test]
    fn test_gate_passes_on_module_add_with_decision() {
        let old = parse_str("old", "System App \"desc\" id \"app\" {}").unwrap();
        let new = parse_str(
            "new",
            r#"System App "desc" id "app" {
    Module One "one" id "app.one" {
        path "./src/one"
    }
}"#,
        )
        .unwrap();
        let decisions = vec![accepted_for("app.one")];
        let findings = architecture_findings(&old, &new, &decisions, false);
        assert!(findings.is_empty());
    }

    #[test]
    fn test_gate_passes_with_escape_hatch() {
        let old = parse_str("old", "System App \"desc\" id \"app\" {}").unwrap();
        let new = parse_str(
            "new",
            r#"System App "desc" id "app" {
    Module One "one" id "app.one" {
        path "./src/one"
    }
}"#,
        )
        .unwrap();
        let findings = architecture_findings(&old, &new, &[], true);
        assert!(findings.is_empty());
    }

    #[test]
    fn test_gate_ignores_reorder_and_casing() {
        let old = parse_str(
            "old",
            r#"System App "desc" id "app" {
    Module One "one" id "app.one" {
        path "./src/one"
    }
    Module Two "two" id "app.two" {
        path "./src/two"
    }
}"#,
        )
        .unwrap();
        let new = parse_str(
            "new",
            r#"System App "desc" id "app" {
    Module Two "two" id "app.two" {
        path "./src/two"
    }
    Module One "one" id "app.one" {
        path "./src/one"
    }
}"#,
        )
        .unwrap();
        let findings = architecture_findings(&old, &new, &[], false);
        assert!(findings.is_empty());
    }

    #[test]
    fn test_gate_fires_on_module_remove_without_decision() {
        let old = parse_str(
            "old",
            r#"System App "desc" id "app" {
    Module One "one" id "app.one" {
        path "./src/one"
    }
}"#,
        )
        .unwrap();
        let new = parse_str("new", "System App \"desc\" id \"app\" {}").unwrap();
        let findings = architecture_findings(&old, &new, &[], false);
        assert_eq!(findings.len(), 1);
        assert!(findings[0].message.contains("removed"));
    }

    #[test]
    fn test_gate_fires_on_reassign_across_containers() {
        let old = parse_str(
            "old",
            r#"System App "desc" id "app" {
    Container A "a" id "app.a" {
        Module One "one" id "app.one" {
            path "./src/one"
        }
    }
}"#,
        )
        .unwrap();
        let new = parse_str(
            "new",
            r#"System App "desc" id "app" {
    Container A "a" id "app.a" {}
    Container B "b" id "app.b" {
        Module One "one" id "app.one" {
            path "./src/one"
        }
    }
}"#,
        )
        .unwrap();
        let findings = architecture_findings(&old, &new, &[], false);
        assert_eq!(findings.len(), 1);
        assert!(findings[0].message.contains("reassigned"));
        assert!(findings[0].message.contains("app.a"));
        assert!(findings[0].message.contains("app.b"));
    }

    #[test]
    fn test_gate_ignores_path_rename_within_container() {
        let old = parse_str(
            "old",
            r#"System App "desc" id "app" {
    Module One "one" id "app.one" {
        path "./src/old"
    }
}"#,
        )
        .unwrap();
        let new = parse_str(
            "new",
            r#"System App "desc" id "app" {
    Module One "one" id "app.one" {
        path "./src/new"
    }
}"#,
        )
        .unwrap();
        let findings = architecture_findings(&old, &new, &[], false);
        assert!(findings.is_empty());
    }

    #[test]
    fn test_decision_multi_node_covers_multiple_changes() {
        let old = parse_str(
            "old",
            r#"System App "desc" id "app" {
    Module One "one" id "app.one" {
        path "./src/one"
    }
}"#,
        )
        .unwrap();
        let new = parse_str(
            "new",
            r#"System App "desc" id "app" {
    Module One "one" id "app.one" {
        path "./src/one"
    }
    Module Two "two" id "app.two" {
        path "./src/two"
    }
}"#,
        )
        .unwrap();
        let mut decision = accepted_for("placeholder");
        decision.nodes = vec!["app.one".to_owned(), "app.two".to_owned()];
        let findings = architecture_findings(&old, &new, &[decision], false);
        assert!(findings.is_empty());
    }
}
