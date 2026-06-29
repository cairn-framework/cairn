//! Typed artefact registry and Phase 2 loaders.

use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

use crate::blueprint::Ast;

use super::{contract::ContractSet, frontmatter};
mod changes;
mod io;
mod parse;
mod sha256;
/// Artefact type definitions.
pub mod types;
mod validate;

use changes::load_changes;
use io::list;
use io::{collect_ids, markdown_paths, optional, parse_file, path_string, pointers, required};
use parse::{
    parse_decision_status, parse_research_method, parse_review_type, parse_source_verification,
    parse_todo_status,
};
pub use types::*;
use validate::validate_integrity;

#[must_use]
/// Loads all non-contract Phase 2 artefacts from retained blueprint pointers.
pub fn load_artefacts(root: &Path, ast: &Ast, contracts: ContractSet) -> ArtefactSet {
    let ids = collect_ids(ast);
    let mut set = ArtefactSet {
        contracts,
        ..ArtefactSet::default()
    };
    load_todos(root, ast, &mut set);
    load_decisions(root, ast, &mut set);
    load_reviews(root, ast, &mut set);
    load_research(root, ast, &mut set);
    load_sources(root, ast, &mut set);
    load_changes(root, &mut set);
    validate_integrity(root, &ids, &mut set);
    set
}
fn load_todos(root: &Path, ast: &Ast, set: &mut ArtefactSet) {
    for pointer in pointers(ast, "todos") {
        for path in markdown_paths(root, &pointer, set) {
            if let Some(parsed) = parse_file(&path, &pointer, set) {
                let Some(node) = required(&parsed.values, "node", path_string(&path), set) else {
                    continue;
                };
                let Some(status) = required(&parsed.values, "status", path_string(&path), set)
                    .and_then(|value| parse_todo_status(&value, &path, set))
                else {
                    continue;
                };
                let Some(created) = required(&parsed.values, "created", path_string(&path), set)
                else {
                    continue;
                };
                set.todos.push(Todo {
                    path: path_string(&path),
                    node,
                    status,
                    created,
                    satisfies: optional(&parsed.values, "satisfies"),
                    body: parsed.body,
                });
            }
        }
    }
}

/// Load decision artefacts from all `decisions` pointers declared in `ast`.
pub(crate) fn load_decisions(root: &Path, ast: &Ast, set: &mut ArtefactSet) {
    for pointer in pointers(ast, "decisions") {
        for path in markdown_paths(root, &pointer, set) {
            if let Some(parsed) = parse_file(&path, &pointer, set) {
                let Some(id) = required(&parsed.values, "id", path_string(&path), set) else {
                    continue;
                };
                let Some(status) = required(&parsed.values, "status", path_string(&path), set)
                    .and_then(|value| parse_decision_status(&value, &path, set))
                else {
                    continue;
                };
                let Some(date) = required(&parsed.values, "date", path_string(&path), set) else {
                    continue;
                };
                set.decisions.push(Decision {
                    id,
                    path: path_string(&path),
                    nodes: list(&parsed, "nodes"),
                    status,
                    date,
                    revisited: optional(&parsed.values, "revisited"),
                    revisit_triggers: list(&parsed, "revisit_triggers"),
                    informed_by: list(&parsed, "informed_by"),
                    supersedes: list(&parsed, "supersedes"),
                    refines: list(&parsed, "refines"),
                    related: list(&parsed, "related"),
                    orphaned: optional(&parsed.values, "orphaned")
                        .is_some_and(|value| value == "true"),
                    orphan_reason: optional(&parsed.values, "orphan_reason"),

                    claims: parse_claims(&parsed.values, &parsed.lists, &path),
                    body: parsed.body,
                });
            }
        }
    }
}

fn parse_claims(
    values: &std::collections::BTreeMap<String, String>,
    lists: &std::collections::BTreeMap<String, Vec<String>>,
    _path: &std::path::Path,
) -> Option<crate::artefacts::Claims> {
    let folder = values.get("claims_folder")?;
    let mode = match values.get("claims_mode").map(String::as_str) {
        Some("exhaustive") => crate::artefacts::ClaimsMode::Exhaustive,
        Some("illustrative") => crate::artefacts::ClaimsMode::Illustrative,
        _ => return None,
    };
    let items = lists.get("claims_items").cloned().unwrap_or_default();
    Some(crate::artefacts::Claims {
        folder: folder.clone(),
        mode,
        items,
    })
}
fn load_reviews(root: &Path, ast: &Ast, set: &mut ArtefactSet) {
    for pointer in pointers(ast, "reviews") {
        for path in markdown_paths(root, &pointer, set) {
            if let Some(parsed) = parse_file(&path, &pointer, set) {
                let Some(node) = required(&parsed.values, "node", path_string(&path), set) else {
                    continue;
                };
                let Some(date) = required(&parsed.values, "date", path_string(&path), set) else {
                    continue;
                };
                let Some(reviewer) = required(&parsed.values, "reviewer", path_string(&path), set)
                else {
                    continue;
                };
                let review_type = optional(&parsed.values, "review_type")
                    .map_or(Some(ReviewType::Human), |value| {
                        parse_review_type(&value, &path, set)
                    });
                let Some(review_type) = review_type else {
                    continue;
                };
                set.reviews.push(Review {
                    path: path_string(&path),
                    node,
                    review_type,
                    date,
                    reviewer,
                    related_change: optional(&parsed.values, "related_change"),
                    body: parsed.body,
                });
            }
        }
    }
}

fn load_research(root: &Path, ast: &Ast, set: &mut ArtefactSet) {
    for pointer in pointers(ast, "research") {
        for path in markdown_paths(root, &pointer, set) {
            if let Some(parsed) = parse_file(&path, &pointer, set) {
                let Some(id) = required(&parsed.values, "id", path_string(&path), set) else {
                    continue;
                };
                let Some(date) = required(&parsed.values, "date", path_string(&path), set) else {
                    continue;
                };
                let method = optional(&parsed.values, "method")
                    .and_then(|value| parse_research_method(&value, &path, set))
                    .unwrap_or_default();
                set.research.push(Research {
                    id,
                    path: path_string(&path),
                    nodes: list(&parsed, "nodes"),
                    date,
                    sources: list(&parsed, "sources"),
                    method,
                    tags: list(&parsed, "tags"),
                    body: parsed.body,
                });
            }
        }
    }
}

fn load_sources(root: &Path, ast: &Ast, set: &mut ArtefactSet) {
    for pointer in pointers(ast, "sources") {
        for path in markdown_paths(root, &pointer, set) {
            if let Some(parsed) = parse_file(&path, &pointer, set) {
                let Some(id) = required(&parsed.values, "id", path_string(&path), set) else {
                    continue;
                };
                let Some(file) = required(&parsed.values, "file", path_string(&path), set) else {
                    continue;
                };
                let Some(verification) =
                    required(&parsed.values, "verification", path_string(&path), set)
                        .and_then(|value| parse_source_verification(&value, &path, set))
                else {
                    continue;
                };
                let Some(source_type) = required(&parsed.values, "type", path_string(&path), set)
                else {
                    continue;
                };
                let Some(date) = required(&parsed.values, "date", path_string(&path), set) else {
                    continue;
                };
                set.sources.push(Source {
                    id,
                    path: path_string(&path),
                    file,
                    sha256: optional(&parsed.values, "sha256").filter(|value| value != "null"),
                    verification,
                    source_type,
                    date,
                    tags: list(&parsed, "tags"),
                    description: optional(&parsed.values, "description").unwrap_or_default(),
                    body: parsed.body,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blueprint::parser::parse_str;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_load_artefacts_loads_known_records() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("loads-known-records")?;
        let ast = write_project(&root)?;

        let set = load_artefacts(&root, &ast, ContractSet::default());

        assert_eq!(set.todos.len(), 1);
        assert_eq!(set.decisions.len(), 1);
        assert_eq!(set.research.len(), 1);
        assert_eq!(set.sources.len(), 1);
        assert!(set.findings.is_empty(), "{:?}", set.findings);
        Ok(())
    }

    #[test]
    fn test_load_artefacts_reports_unknown_node_references()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("unknown-node")?;
        let ast = parse_str(
            "cairn.blueprint",
            r#"System App "desc" id "app" {
    Module Api "api" id "app.api" {
        todos "./meta/todos"
    }
}
"#,
        )?;
        fs::create_dir_all(root.join("meta/todos"))?;
        fs::write(
            root.join("meta/todos/todo.api.md"),
            "---\nnode: ghost.node\nstatus: open\ncreated: 2026-04-01\n---\n# Todo\n",
        )?;

        let set = load_artefacts(&root, &ast, ContractSet::default());

        assert!(
            set.findings
                .iter()
                .any(|finding| finding.code == "CAIRN_TODO_ORPHAN_NODE")
        );
        Ok(())
    }

    #[test]
    fn test_load_artefacts_reports_duplicate_or_invalid_provenance()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("invalid-provenance")?;
        let ast = parse_str(
            "cairn.blueprint",
            r#"System App "desc" id "app" {
    Module Api "api" id "app.api" {
        decisions "./meta/decisions"
    }
}
"#,
        )?;
        fs::create_dir_all(root.join("meta/decisions"))?;
        fs::write(
            root.join("meta/decisions/dec.api.md"),
            "---\nid: dec.api\nnodes: [app.api]\nstatus: accepted\ndate: 2026-04-01\ninformed_by: [missing.ref]\n---\n# Decision\n",
        )?;

        let set = load_artefacts(&root, &ast, ContractSet::default());

        assert!(
            set.findings
                .iter()
                .any(|finding| finding.code == "CAIRN_DECISION_UNKNOWN_PROVENANCE")
        );
        Ok(())
    }

    #[test]
    fn test_parse_status_kinds_reject_unknown_values() {
        let mut set = ArtefactSet::default();
        let path = Path::new("bad.md");

        assert!(parse_todo_status("bad", path, &mut set).is_none());
        assert!(parse_decision_status("bad", path, &mut set).is_none());
        assert!(parse_review_type("bad", path, &mut set).is_none());
        assert!(parse_source_verification("bad", path, &mut set).is_none());
        assert_eq!(set.findings.len(), 4);
    }

    fn write_project(root: &Path) -> Result<Ast, Box<dyn std::error::Error>> {
        fs::create_dir_all(root.join("meta/todos"))?;
        fs::create_dir_all(root.join("meta/decisions"))?;
        fs::create_dir_all(root.join("meta/research"))?;
        fs::create_dir_all(root.join("meta/sources"))?;
        fs::write(root.join("docs-source.txt"), "source\n")?;
        fs::write(
            root.join("meta/todos/todo.api.md"),
            "---\nnode: app.api\nstatus: open\ncreated: 2026-04-01\n---\n# Todo\n",
        )?;
        fs::write(
            root.join("meta/decisions/dec.api.md"),
            "---\nid: dec.api\nnodes: [app.api]\nstatus: accepted\ndate: 2026-04-01\ninformed_by: [res.api]\n---\n# Decision\n",
        )?;
        fs::write(
            root.join("meta/research/res.api.md"),
            "---\nid: res.api\nnodes: [app.api]\ndate: 2026-03-20\nsources: [src.api]\n---\n# Research\n",
        )?;
        fs::write(
            root.join("meta/sources/src.api.md"),
            "---\nid: src.api\nfile: docs-source.txt\nsha256: b8bb034f9b63bd0254fbc7c157cae746c75853f4643d6cea844dc48ddb57f522\nverification: verified\ntype: note\ndate: 2026-03-19\n---\n# Source\n",
        )?;
        parse_str(
            "cairn.blueprint",
            r#"System App "desc" id "app" {
    Module Api "api" id "app.api" {
        todos "./meta/todos"
        decisions "./meta/decisions"
        research "./meta/research"
        sources "./meta/sources"
    }
}
"#,
        )
        .map_err(Into::into)
    }

    fn temp_root(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let root = std::env::temp_dir().join(format!("cairn-artefacts-tests-{name}-{suffix}"));
        fs::create_dir_all(&root)?;
        Ok(root)
    }

    // ── parse_claims ──────────────────────────────────────────────────────────

    fn claims_values(folder: &str, mode: &str) -> std::collections::BTreeMap<String, String> {
        let mut m = std::collections::BTreeMap::new();
        m.insert("claims_folder".to_owned(), folder.to_owned());
        m.insert("claims_mode".to_owned(), mode.to_owned());
        m
    }

    fn claims_lists(items: &[&str]) -> std::collections::BTreeMap<String, Vec<String>> {
        let mut m = std::collections::BTreeMap::new();
        m.insert(
            "claims_items".to_owned(),
            items.iter().map(ToString::to_string).collect(),
        );
        m
    }

    fn no_lists() -> std::collections::BTreeMap<String, Vec<String>> {
        std::collections::BTreeMap::new()
    }

    #[test]
    fn test_parse_claims_absent_when_no_folder_key() {
        let result = parse_claims(
            &std::collections::BTreeMap::new(),
            &no_lists(),
            Path::new("test.md"),
        );
        assert!(result.is_none(), "no claims_folder → None");
    }

    #[test]
    fn test_parse_claims_exhaustive_with_items_returns_some() {
        let result = parse_claims(
            &claims_values("meta/decisions", "exhaustive"),
            &claims_lists(&["a.md", "b.md"]),
            Path::new("test.md"),
        );
        let claims = result.expect("exhaustive + items must return Some");
        assert_eq!(claims.folder, "meta/decisions");
        assert!(matches!(
            claims.mode,
            crate::artefacts::ClaimsMode::Exhaustive
        ));
        assert_eq!(claims.items, vec!["a.md", "b.md"]);
    }

    #[test]
    fn test_parse_claims_illustrative_mode_returns_some() {
        let result = parse_claims(
            &claims_values("meta/decisions", "illustrative"),
            &claims_lists(&["a.md"]),
            Path::new("test.md"),
        );
        let claims = result.expect("illustrative + items must return Some");
        assert!(matches!(
            claims.mode,
            crate::artefacts::ClaimsMode::Illustrative
        ));
    }

    #[test]
    fn test_parse_claims_unknown_mode_returns_none() {
        // Unknown mode silently returns None (documents the current behavior;
        // a future improvement could emit an error finding instead).
        let result = parse_claims(
            &claims_values("meta/decisions", "Exhaustive"), // wrong case
            &claims_lists(&["a.md"]),
            Path::new("test.md"),
        );
        assert!(result.is_none(), "unknown mode must return None");
    }

    #[test]
    fn test_parse_claims_missing_items_defaults_to_empty_not_none() {
        // When claims_folder and a valid claims_mode are present but
        // claims_items is absent, the CA003 check should still run —
        // an exhaustive claim with no listed files means every file in
        // the folder is "missing from claim". Returning None silently
        // disables the check entirely.
        let result = parse_claims(
            &claims_values("meta/decisions", "exhaustive"),
            &no_lists(), // no claims_items key at all
            Path::new("test.md"),
        );
        assert!(
            result.is_some(),
            "missing claims_items must not disable claims checking; got None"
        );
        assert_eq!(
            result.unwrap().items,
            Vec::<String>::new(),
            "absent claims_items must default to empty list"
        );
    }
}
