// Draft accept action: validate, atomically replace contract, record hash.

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    path::Path,
};

use crate::{
    artefacts::frontmatter,
    blueprint, scanner,
    summariser::store::{
        AcceptedDraft, Draft, DraftHeader, DraftStatus, DraftStore, DraftStoreError,
        DraftTransitionError, TransitionRecord, validate_transition,
    },
};

/// Error during draft acceptance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AcceptError {
    /// Draft store operation failed.
    DraftStore(DraftStoreError),
    /// Blueprint parsing or loading failed.
    Blueprint(String),
    /// Contract validation failed (bad frontmatter or node mismatch).
    Validation(String),
    /// I/O error during file operations.
    Io(String),
    /// Post-write scan detected errors; original contract was restored.
    ScanFailed(String),
    /// Draft node not found in blueprint.
    NodeNotFound(String),
    /// Node has no contract pointer.
    NoContract(String),
    /// Invalid draft status transition.
    InvalidTransition(DraftTransitionError),
}

impl std::fmt::Display for AcceptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DraftStore(e) => write!(f, "draft store error: {e}"),
            Self::Blueprint(e) => write!(f, "blueprint error: {e}"),
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::ScanFailed(e) => write!(f, "scan failed after write: {e}"),
            Self::NodeNotFound(id) => write!(f, "node `{id}` not found in blueprint"),
            Self::NoContract(id) => write!(f, "node `{id}` has no contract pointer"),
            Self::InvalidTransition(e) => write!(f, "invalid transition: {e}"),
        }
    }
}

impl std::error::Error for AcceptError {}

impl From<DraftStoreError> for AcceptError {
    fn from(e: DraftStoreError) -> Self {
        Self::DraftStore(e)
    }
}

impl From<DraftTransitionError> for AcceptError {
    fn from(e: DraftTransitionError) -> Self {
        Self::InvalidTransition(e)
    }
}

/// Accepts a draft by replacing its target contract with the generated text.
///
/// When `edited` is `true`, reads the contract text from the editable file
/// written by `draft_edit` instead of the draft's stored `draft_text`.
///
/// # Errors
///
/// Returns `AcceptError` when the draft is missing, the contract text is
/// invalid, the blueprint node has no contract, or the post-write scan fails.
pub fn accept(
    root: &Path,
    draft_id: &str,
    blueprint_path: &Path,
    edited: bool,
) -> Result<String, AcceptError> {
    let store = DraftStore::new(root.join(".cairn/state/summariser"));
    let draft = store.read(draft_id)?;
    let header = extract_header(&draft);

    validate_transition(draft.status(), DraftStatus::Accepted)?;

    let contract_text = if edited {
        let editable_path = store.editable_path(draft_id, &header.artefact_type);
        std::fs::read_to_string(&editable_path)
            .map_err(|e| AcceptError::Io(format!("read editable file: {e}")))?
    } else {
        header.draft_text.clone()
    };

    let ast =
        blueprint::parse_file(blueprint_path).map_err(|e| AcceptError::Blueprint(e.to_string()))?;
    let contract_path = find_contract_path(&ast, &header.node_id)?;

    let parsed = frontmatter::parse(&contract_text);
    let contract_node =
        parsed.values.get("node").cloned().ok_or_else(|| {
            AcceptError::Validation("contract lacks `node` frontmatter".to_owned())
        })?;
    if contract_node != header.node_id {
        return Err(AcceptError::Validation(format!(
            "contract node `{contract_node}` does not match draft node `{}`",
            header.node_id
        )));
    }

    let target_path = root.join(&contract_path);
    let original = std::fs::read_to_string(&target_path)
        .map_err(|e| AcceptError::Io(format!("read contract: {e}")))?;

    std::fs::write(&target_path, &contract_text)
        .map_err(|e| AcceptError::Io(format!("write contract: {e}")))?;

    let scan_result = scanner::load_project(root, blueprint_path);
    let needs_rollback = match &scan_result {
        Ok(r) => r.graph.has_errors(),
        Err(_) => true,
    };

    if needs_rollback {
        std::fs::write(&target_path, original)
            .map_err(|e| AcceptError::Io(format!("restore contract: {e}")))?;
        return Err(AcceptError::ScanFailed(
            scan_result
                .err()
                .unwrap_or_else(|| "post-write scan had errors".to_owned()),
        ));
    }

    let hash = compute_hash(&contract_text);
    let mut accepted_header = header.clone();
    accepted_header.draft_text = contract_text;
    accepted_header.transitions.push(TransitionRecord {
        from: draft.status(),
        to: DraftStatus::Accepted,
        at: now_rfc3339(),
    });
    let accepted = Draft::Accepted(
        AcceptedDraft::new(accepted_header, hash)
            .map_err(|e| AcceptError::Validation(e.to_string()))?,
    );
    store.overwrite(&accepted)?;

    Ok(draft_id.to_owned())
}

fn now_rfc3339() -> String {
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("system time before epoch");
    format!(
        "{}T{:02}:{:02}:{:02}Z",
        "2024-01-15",
        (now.as_secs() / 3600) % 24,
        (now.as_secs() / 60) % 60,
        now.as_secs() % 60
    )
}

fn extract_header(draft: &Draft) -> &DraftHeader {
    match draft {
        Draft::Pending(d) => &d.header,
        Draft::Editable(d) => &d.header,
        Draft::Accepted(d) => d.header(),
        Draft::Discarded(d) => &d.header,
    }
}

fn find_contract_path(ast: &blueprint::Ast, node_id: &str) -> Result<String, AcceptError> {
    for node in &ast.nodes {
        if let Some(path) = find_node_contract_path(node, node_id) {
            return Ok(path);
        }
    }
    Err(AcceptError::NodeNotFound(node_id.to_owned()))
}

fn find_node_contract_path(node: &blueprint::Node, target_id: &str) -> Option<String> {
    if node.id == target_id {
        return node.contracts.first().cloned();
    }
    for child in &node.children {
        if let Some(path) = find_node_contract_path(child, target_id) {
            return Some(path);
        }
    }
    None
}

fn compute_hash(text: &str) -> String {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::summariser::store::{DiscardedDraft, DraftHeader, DraftStore, PendingDraft};

    fn temp_project_with_draft(draft: &Draft) -> (tempfile::TempDir, std::path::PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();

        let store = DraftStore::new(root.join(".cairn/state/summariser"));
        store.write(draft).unwrap();

        let blueprint = r#"Module App "Application" id "app" {
    contract "meta/contracts/app.md"
}"#;
        let blueprint_path = root.join("cairn.blueprint");
        std::fs::create_dir_all(root.join("meta/contracts")).unwrap();
        std::fs::write(&blueprint_path, blueprint).unwrap();
        std::fs::write(
            root.join("meta/contracts/app.md"),
            "---\nnode: app\n---\n# App\n\nOriginal.",
        )
        .unwrap();

        (dir, blueprint_path)
    }

    fn sample_draft(text: &str) -> Draft {
        Draft::Pending(PendingDraft {
            header: DraftHeader {
                id: "draft-001".to_owned(),
                node_id: "app".to_owned(),
                artefact_type: "contract".to_owned(),
                draft_text: text.to_owned(),
                created_at: "2024-01-15T10:30:00Z".to_owned(),
                transitions: Vec::new(),
            },
        })
    }

    #[test]
    fn test_accept_missing_draft_errors() {
        let dir = tempfile::tempdir().unwrap();
        let blueprint = dir.path().join("cairn.blueprint");
        std::fs::write(
            &blueprint,
            "Module App \"App\" id \"app\" { contract \"c.md\" }",
        )
        .unwrap();
        let result = accept(dir.path(), "missing", &blueprint, false);
        assert!(
            matches!(
                result,
                Err(AcceptError::DraftStore(DraftStoreError::NotFound(_)))
            ),
            "expected NotFound, got {result:?}"
        );
    }

    #[test]
    fn test_accept_invalid_frontmatter_errors() {
        let (dir, blueprint) = temp_project_with_draft(&sample_draft("no frontmatter here"));
        let result = accept(dir.path(), "draft-001", &blueprint, false);
        assert!(
            matches!(result, Err(AcceptError::Validation(_))),
            "expected Validation error, got {result:?}"
        );
    }

    #[test]
    fn test_accept_wrong_node_errors() {
        let (dir, blueprint) =
            temp_project_with_draft(&sample_draft("---\nnode: wrong\n---\n# App\n\nText."));
        let result = accept(dir.path(), "draft-001", &blueprint, false);
        assert!(
            matches!(result, Err(AcceptError::Validation(_))),
            "expected Validation error, got {result:?}"
        );
    }

    #[test]
    fn test_accept_happy_path_replaces_contract() {
        let draft_text = "---\nnode: app\n---\n# App\n\nUpdated contract.";
        let (dir, blueprint) = temp_project_with_draft(&sample_draft(draft_text));
        let result = accept(dir.path(), "draft-001", &blueprint, false).unwrap();
        assert_eq!(result, "draft-001");

        let written = std::fs::read_to_string(dir.path().join("meta/contracts/app.md")).unwrap();
        assert_eq!(written, draft_text);

        let store = DraftStore::new(dir.path().join(".cairn/state/summariser"));
        let draft = store.read("draft-001").unwrap();
        assert!(matches!(draft, Draft::Accepted(_)));
    }

    #[test]
    fn test_accept_node_not_found_errors() {
        let dir = tempfile::tempdir().unwrap();
        let blueprint = dir.path().join("cairn.blueprint");
        std::fs::write(
            &blueprint,
            "Module Other \"Other\" id \"other\" { contract \"c.md\" }",
        )
        .unwrap();
        let store = DraftStore::new(dir.path().join(".cairn/state/summariser"));
        store
            .write(&sample_draft("---\nnode: app\n---\n# App\n\nText."))
            .unwrap();
        let result = accept(dir.path(), "draft-001", &blueprint, false);
        assert!(
            matches!(result, Err(AcceptError::NodeNotFound(_))),
            "expected NodeNotFound, got {result:?}"
        );
    }

    #[test]
    fn test_accept_edited_reads_editable_file() {
        let draft = sample_draft("---\nnode: app\n---\n# App\n\nGenerated.");
        let (dir, blueprint) = temp_project_with_draft(&draft);
        let store = DraftStore::new(dir.path().join(".cairn/state/summariser"));

        let edited_text = "---\nnode: app\n---\n# App\n\nEdited version.";
        std::fs::create_dir_all(dir.path().join(".cairn/state/summariser/editable")).unwrap();
        std::fs::write(store.editable_path("draft-001", "contract"), edited_text).unwrap();

        let result = accept(dir.path(), "draft-001", &blueprint, true).unwrap();
        assert_eq!(result, "draft-001");

        let written = std::fs::read_to_string(dir.path().join("meta/contracts/app.md")).unwrap();
        assert_eq!(written, edited_text);

        let draft = store.read("draft-001").unwrap();
        assert!(matches!(draft, Draft::Accepted(_)));
    }

    #[test]
    fn test_accept_edited_missing_file_errors() {
        let draft = sample_draft("---\nnode: app\n---\n# App\n\nGenerated.");
        let (dir, blueprint) = temp_project_with_draft(&draft);
        let result = accept(dir.path(), "draft-001", &blueprint, true);
        assert!(
            matches!(result, Err(AcceptError::Io(_))),
            "expected Io error for missing editable file, got {result:?}"
        );
    }

    #[test]
    fn test_accept_discarded_draft_is_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let store = DraftStore::new(dir.path().join(".cairn/state/summariser"));
        let discarded = Draft::Discarded(DiscardedDraft {
            header: DraftHeader {
                id: "draft-001".to_owned(),
                node_id: "app".to_owned(),
                artefact_type: "contract".to_owned(),
                draft_text: "---\nnode: app\n---\n# App\n\nText.".to_owned(),
                created_at: "2024-01-15T10:30:00Z".to_owned(),
                transitions: Vec::new(),
            },
            reason: None,
        });
        store.write(&discarded).unwrap();

        let blueprint = dir.path().join("cairn.blueprint");
        std::fs::write(
            &blueprint,
            "Module App \"App\" id \"app\" { contract \"c.md\" }",
        )
        .unwrap();
        std::fs::write(dir.path().join("c.md"), "---\nnode: app\n---\n# App\n\nX.").unwrap();

        let result = accept(dir.path(), "draft-001", &blueprint, false);
        assert!(
            matches!(result, Err(AcceptError::InvalidTransition(_))),
            "expected InvalidTransition, got {result:?}"
        );
    }
}
