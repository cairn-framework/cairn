//! Non-generative contract-baseline record and drop surface.
//!

use std::path::Path;

use crate::{
    artefacts::contract::load_contracts,
    blueprint, scanner,
    scanner::contract_baselines::{ContractBaseline, ContractBaselines},
};

/// Error from a contract-baseline record or drop.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum BaselineError {
    /// The blueprint could not be parsed. Carries the declared path and the
    /// underlying cause; the CLI supplies the sentence.
    BlueprintUnreadable {
        /// Blueprint path as given.
        path: String,
        /// Underlying parse error.
        error: String,
    },
    /// The blueprint does not declare the node.
    NodeNotDeclared(String),
    /// The node is declared but carries no contract pointer.
    NoContract(String),
    /// The node's contract pointer does not resolve to a readable file.
    ContractUnreadable {
        /// Node ID whose contract failed to load.
        node: String,
        /// Declared contract path, relative to the project root.
        path: String,
        /// Underlying I/O error.
        error: String,
    },
    /// No baseline is recorded for the node.
    NotRecorded(String),
    /// The node is still declared with a loadable contract, so its baseline is
    /// live and dropping it would silence a finding without the review the
    /// finding asks for.
    StillLive(String),
    /// The baseline state file could not be read.
    StateReadFailed(String),
    /// The baseline state file could not be written.
    StateWriteFailed(String),
}

/// Records the node's current declared shape as its contract baseline,
/// overwriting any existing entry.
///
/// Read-modify-write: exactly this node's entry changes, and every other entry
/// round-trips unchanged, including inert ones for nodes the blueprint no
/// longer declares. Requires a declared node
/// whose contract loads, so a contractless node can never acquire a baseline
/// and can never trip the drift enforcer.
///
/// # Errors
///
/// Returns [`BaselineError`] when the blueprint fails to parse, the node is not
/// declared, it has no contract pointer, its contract cannot be read, or the
/// state file cannot be read or written. The state file is left untouched on
/// every one of those paths.
pub(crate) fn record_baseline(
    root: &Path,
    blueprint_path: &Path,
    node_id: &str,
) -> Result<ContractBaseline, BaselineError> {
    let ast = parse_blueprint(blueprint_path)?;
    let mut snapshot = scanner::compute_blueprint_snapshot(&ast);
    let Some(fingerprint) = snapshot.nodes.remove(node_id) else {
        return Err(BaselineError::NodeNotDeclared(node_id.to_owned()));
    };
    require_loadable_contract(root, &ast, node_id)?;

    let entry = ContractBaseline {
        kind: fingerprint.kind,
        parent: fingerprint.parent,
        edges: fingerprint.edges,
    };
    let mut baselines = read_baselines(root)?;
    baselines.nodes.insert(node_id.to_owned(), entry.clone());
    write_baselines(root, &baselines)?;
    Ok(entry)
}

/// Drops the node's contract baseline.
///
/// Restricted to inert entries: the node is absent from the blueprint, or its
/// contract does not load. A live entry is refused so drop can never silence a
/// drift finding in place of reviewing it. The scanner never prunes, so without
/// this operation an entry left behind by a removed node would be unremovable
/// except by hand.
///
/// # Errors
///
/// Returns [`BaselineError`] when the blueprint fails to parse, no entry exists
/// for the node, the entry is still live, or the state file cannot be read or
/// written. The state file is left untouched on every one of those paths.
pub(crate) fn drop_baseline(
    root: &Path,
    blueprint_path: &Path,
    node_id: &str,
) -> Result<(), BaselineError> {
    let mut baselines = read_baselines(root)?;
    if !baselines.nodes.contains_key(node_id) {
        return Err(BaselineError::NotRecorded(node_id.to_owned()));
    }
    let ast = parse_blueprint(blueprint_path)?;
    let declared = contract_paths(&ast, node_id).is_some();
    if declared && require_loadable_contract(root, &ast, node_id).is_ok() {
        return Err(BaselineError::StillLive(node_id.to_owned()));
    }
    baselines.nodes.remove(node_id);
    write_baselines(root, &baselines)
}

/// Confirms the node declares at least one contract pointer that resolves to a
/// loaded contract. A node may declare multiple contracts; one loaded contract
/// is enough to keep its baseline live.
fn require_loadable_contract(
    root: &Path,
    ast: &blueprint::Ast,
    node_id: &str,
) -> Result<(), BaselineError> {
    let Some(paths) = contract_paths(ast, node_id) else {
        return Err(BaselineError::NoContract(node_id.to_owned()));
    };
    let Some(first_path) = paths.first() else {
        return Err(BaselineError::NoContract(node_id.to_owned()));
    };
    let contracts = load_contracts(root, ast);
    if paths.iter().any(|path| {
        contracts
            .contracts
            .get(path)
            .is_some_and(|contract| contract.node == node_id)
    }) {
        return Ok(());
    }
    let error = contracts
        .findings
        .into_iter()
        .find(|finding| {
            finding
                .path
                .as_ref()
                .is_some_and(|path| paths.contains(path))
        })
        .map_or_else(
            || "no declared contract file was found".to_owned(),
            |finding| finding.message,
        );
    Err(BaselineError::ContractUnreadable {
        node: node_id.to_owned(),
        path: first_path.clone(),
        error,
    })
}

/// Returns every contract path declared by the node.
fn contract_paths<'a>(ast: &'a blueprint::Ast, node_id: &str) -> Option<&'a [String]> {
    fn walk<'a>(node: &'a blueprint::Node, target: &str) -> Option<&'a [String]> {
        if node.id == target {
            return Some(&node.contracts);
        }
        node.children.iter().find_map(|child| walk(child, target))
    }
    ast.nodes.iter().find_map(|node| walk(node, node_id))
}

/// Parses the blueprint, tagging failures with the path that was tried.
fn parse_blueprint(blueprint_path: &Path) -> Result<blueprint::Ast, BaselineError> {
    blueprint::parse_file(blueprint_path).map_err(|e| BaselineError::BlueprintUnreadable {
        path: blueprint_path.display().to_string(),
        error: e.to_string(),
    })
}

fn read_baselines(root: &Path) -> Result<ContractBaselines, BaselineError> {
    scanner::contract_baselines::read(root)
        .map_err(|e| BaselineError::StateReadFailed(e.to_string()))
}

fn write_baselines(root: &Path, baselines: &ContractBaselines) -> Result<(), BaselineError> {
    scanner::contract_baselines::write(root, baselines)
        .map_err(|e| BaselineError::StateWriteFailed(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A two-node project: `app` (root, contract, no edges) and `app.api`
    /// (child of `app`, contract, one outbound edge).
    fn temp_project() -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::create_dir_all(root.join("meta/contracts")).unwrap();
        std::fs::create_dir_all(root.join("src/api")).unwrap();
        std::fs::create_dir_all(root.join("src/core")).unwrap();
        std::fs::write(root.join("src/api/lib.rs"), "pub fn serve() {}\n").unwrap();
        std::fs::write(root.join("src/core/lib.rs"), "pub fn core() {}\n").unwrap();
        std::fs::write(
            root.join("cairn.blueprint"),
            r#"System App "app" id "app" {
    contract "./meta/contracts/app.md"
    Module Core "core" id "app.core" {
        path "./src/core"
    }
    Module Api "api" id "app.api" {
        path "./src/api"
        contract "./meta/contracts/api.md"
    }
}
app.api -> app.core "reports"
"#,
        )
        .unwrap();
        std::fs::write(
            root.join("meta/contracts/app.md"),
            "---\nnode: app\n---\n# App\n",
        )
        .unwrap();
        std::fs::write(
            root.join("meta/contracts/api.md"),
            "---\nnode: app.api\n---\n# Api\n",
        )
        .unwrap();
        dir
    }

    fn blueprint_of(dir: &tempfile::TempDir) -> std::path::PathBuf {
        dir.path().join("cairn.blueprint")
    }

    fn baseline_file(dir: &tempfile::TempDir) -> std::path::PathBuf {
        dir.path().join(".cairn/state/contract-baselines.json")
    }

    #[test]
    fn test_record_writes_reduced_shape_without_paths() {
        let dir = temp_project();
        let entry = record_baseline(dir.path(), &blueprint_of(&dir), "app.api").unwrap();
        assert_eq!(entry.kind, "Module");
        assert_eq!(entry.parent.as_deref(), Some("app"));
        assert_eq!(entry.edges, vec!["app.core".to_owned()]);

        let raw = std::fs::read_to_string(baseline_file(&dir)).unwrap();
        let value: serde_json::Value = serde_json::from_str(&raw).unwrap();
        assert_eq!(value["version"], 1);
        let keys: Vec<&str> = value["nodes"]["app.api"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(keys, vec!["edges", "kind", "parent"]);
        assert!(raw.trim_start().starts_with("{\n  \"version\""), "{raw}");
    }

    #[test]
    fn test_record_root_node_round_trips_with_null_parent() {
        let dir = temp_project();
        record_baseline(dir.path(), &blueprint_of(&dir), "app").unwrap();
        let raw = std::fs::read_to_string(baseline_file(&dir)).unwrap();
        let value: serde_json::Value = serde_json::from_str(&raw).unwrap();
        assert!(
            value["nodes"]["app"]["parent"].is_null(),
            "root parent must be JSON null, not absent or empty: {raw}"
        );
        let reread = scanner::contract_baselines::read(dir.path()).unwrap();
        assert_eq!(reread.nodes["app"].parent, None);
    }

    #[test]
    fn test_rerecord_overwrites_only_its_own_node() {
        let dir = temp_project();
        record_baseline(dir.path(), &blueprint_of(&dir), "app").unwrap();
        record_baseline(dir.path(), &blueprint_of(&dir), "app.api").unwrap();

        // Drop the edge, then re-record only `app.api`.
        std::fs::write(
            blueprint_of(&dir),
            std::fs::read_to_string(blueprint_of(&dir))
                .unwrap()
                .replace("app.api -> app.core \"reports\"\n", ""),
        )
        .unwrap();
        let entry = record_baseline(dir.path(), &blueprint_of(&dir), "app.api").unwrap();
        assert!(entry.edges.is_empty());

        let baselines = scanner::contract_baselines::read(dir.path()).unwrap();
        assert!(baselines.nodes["app.api"].edges.is_empty());
        assert_eq!(baselines.nodes["app"].kind, "System");
    }

    #[test]
    fn test_record_rejects_undeclared_node_and_leaves_file_untouched() {
        let dir = temp_project();
        record_baseline(dir.path(), &blueprint_of(&dir), "app").unwrap();
        let before = std::fs::read(baseline_file(&dir)).unwrap();

        let result = record_baseline(dir.path(), &blueprint_of(&dir), "app.nope");
        assert_eq!(
            result,
            Err(BaselineError::NodeNotDeclared("app.nope".to_owned()))
        );
        assert_eq!(std::fs::read(baseline_file(&dir)).unwrap(), before);
    }

    #[test]
    fn test_record_rejects_contractless_node_and_leaves_file_untouched() {
        let dir = temp_project();
        record_baseline(dir.path(), &blueprint_of(&dir), "app").unwrap();
        let before = std::fs::read(baseline_file(&dir)).unwrap();

        // `app.core` is declared but carries no contract pointer.
        let result = record_baseline(dir.path(), &blueprint_of(&dir), "app.core");
        assert_eq!(
            result,
            Err(BaselineError::NoContract("app.core".to_owned()))
        );
        assert_eq!(std::fs::read(baseline_file(&dir)).unwrap(), before);
    }

    #[test]
    fn test_record_rejects_unreadable_contract_and_leaves_file_untouched() {
        let dir = temp_project();
        record_baseline(dir.path(), &blueprint_of(&dir), "app").unwrap();
        let before = std::fs::read(baseline_file(&dir)).unwrap();
        std::fs::remove_file(dir.path().join("meta/contracts/api.md")).unwrap();

        let result = record_baseline(dir.path(), &blueprint_of(&dir), "app.api");
        assert!(
            matches!(result, Err(BaselineError::ContractUnreadable { ref node, .. }) if node == "app.api"),
            "expected ContractUnreadable, got {result:?}"
        );
        assert_eq!(std::fs::read(baseline_file(&dir)).unwrap(), before);
    }

    #[test]
    fn test_record_rejects_contracts_that_fail_to_load_and_leaves_file_untouched() {
        for invalid_contract in ["---\n---\n# Api\n", "---\nnode: app\n---\n# Api\n"] {
            let dir = temp_project();
            record_baseline(dir.path(), &blueprint_of(&dir), "app").unwrap();
            let before = std::fs::read(baseline_file(&dir)).unwrap();
            std::fs::write(dir.path().join("meta/contracts/api.md"), invalid_contract).unwrap();

            let result = record_baseline(dir.path(), &blueprint_of(&dir), "app.api");
            assert!(
                matches!(result, Err(BaselineError::ContractUnreadable { ref node, .. }) if node == "app.api"),
                "expected ContractUnreadable, got {result:?}"
            );
            assert_eq!(std::fs::read(baseline_file(&dir)).unwrap(), before);
        }
    }

    #[test]
    fn test_record_accepts_a_later_loaded_contract() {
        let dir = temp_project();
        let blueprint = std::fs::read_to_string(blueprint_of(&dir))
            .unwrap()
            .replace(
                "        contract \"./meta/contracts/api.md\"",
                "        contract \"./meta/contracts/missing.md\"\n        contract \"./meta/contracts/api.md\"",
            );
        std::fs::write(blueprint_of(&dir), blueprint).unwrap();

        record_baseline(dir.path(), &blueprint_of(&dir), "app.api").unwrap();
        assert!(
            scanner::contract_baselines::read(dir.path())
                .unwrap()
                .nodes
                .contains_key("app.api")
        );
    }

    #[test]
    fn test_drop_prunes_an_entry_for_an_undeclared_node() {
        let dir = temp_project();
        record_baseline(dir.path(), &blueprint_of(&dir), "app").unwrap();
        record_baseline(dir.path(), &blueprint_of(&dir), "app.api").unwrap();

        // Remove `app.api` from the blueprint, making its entry inert.
        std::fs::write(
            blueprint_of(&dir),
            r#"System App "app" id "app" {
    contract "./meta/contracts/app.md"
    Module Core "core" id "app.core" {
        path "./src/core"
    }
}
"#,
        )
        .unwrap();
        drop_baseline(dir.path(), &blueprint_of(&dir), "app.api").unwrap();

        let baselines = scanner::contract_baselines::read(dir.path()).unwrap();
        assert!(!baselines.nodes.contains_key("app.api"));
        assert!(
            baselines.nodes.contains_key("app"),
            "sibling entry survives"
        );
    }

    #[test]
    fn test_drop_prunes_an_entry_whose_contract_no_longer_loads() {
        let dir = temp_project();
        record_baseline(dir.path(), &blueprint_of(&dir), "app.api").unwrap();
        std::fs::remove_file(dir.path().join("meta/contracts/api.md")).unwrap();

        drop_baseline(dir.path(), &blueprint_of(&dir), "app.api").unwrap();
        assert!(
            scanner::contract_baselines::read(dir.path())
                .unwrap()
                .nodes
                .is_empty()
        );
    }

    #[test]
    fn test_drop_prunes_entries_whose_contracts_fail_to_load() {
        for invalid_contract in ["---\n---\n# Api\n", "---\nnode: app\n---\n# Api\n"] {
            let dir = temp_project();
            record_baseline(dir.path(), &blueprint_of(&dir), "app.api").unwrap();
            std::fs::write(dir.path().join("meta/contracts/api.md"), invalid_contract).unwrap();

            drop_baseline(dir.path(), &blueprint_of(&dir), "app.api").unwrap();
            assert!(
                scanner::contract_baselines::read(dir.path())
                    .unwrap()
                    .nodes
                    .is_empty()
            );
        }
    }

    #[test]
    fn test_drop_refuses_a_live_entry_and_leaves_file_untouched() {
        let dir = temp_project();
        record_baseline(dir.path(), &blueprint_of(&dir), "app.api").unwrap();
        let before = std::fs::read(baseline_file(&dir)).unwrap();

        let result = drop_baseline(dir.path(), &blueprint_of(&dir), "app.api");
        assert_eq!(result, Err(BaselineError::StillLive("app.api".to_owned())));
        assert_eq!(std::fs::read(baseline_file(&dir)).unwrap(), before);
    }

    #[test]
    fn test_drop_refuses_when_a_later_contract_loads() {
        let dir = temp_project();
        record_baseline(dir.path(), &blueprint_of(&dir), "app.api").unwrap();
        let before = std::fs::read(baseline_file(&dir)).unwrap();
        let blueprint = std::fs::read_to_string(blueprint_of(&dir))
            .unwrap()
            .replace(
                "        contract \"./meta/contracts/api.md\"",
                "        contract \"./meta/contracts/missing.md\"\n        contract \"./meta/contracts/api.md\"",
            );
        std::fs::write(blueprint_of(&dir), blueprint).unwrap();

        let result = drop_baseline(dir.path(), &blueprint_of(&dir), "app.api");
        assert_eq!(result, Err(BaselineError::StillLive("app.api".to_owned())));
        assert_eq!(std::fs::read(baseline_file(&dir)).unwrap(), before);
    }

    #[test]
    fn test_drop_refuses_an_absent_entry_and_leaves_file_untouched() {
        let dir = temp_project();
        record_baseline(dir.path(), &blueprint_of(&dir), "app").unwrap();
        let before = std::fs::read(baseline_file(&dir)).unwrap();

        let result = drop_baseline(dir.path(), &blueprint_of(&dir), "app.api");
        assert_eq!(
            result,
            Err(BaselineError::NotRecorded("app.api".to_owned()))
        );
        assert_eq!(std::fs::read(baseline_file(&dir)).unwrap(), before);
    }

    #[test]
    fn test_record_never_touches_the_draft_store() {
        // The whole point of this surface: no summariser backend, no draft.
        let dir = temp_project();
        record_baseline(dir.path(), &blueprint_of(&dir), "app.api").unwrap();
        assert!(
            !dir.path().join(".cairn/state/summariser").exists(),
            "recording a baseline must not create a draft store"
        );
    }
}
