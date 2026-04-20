//! Rust semantic extraction helpers for dependency and docstring reconciliation.

use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use crate::blueprint::Node;

use super::{
    DependencyObservation, DocstringFact, DocstringFacts, ObservationConfidence, ReconcileError,
};

/// File owner metadata used for resolving observed dependencies.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct OwnerIndex {
    owners: Vec<(String, String)>,
    file_to_owner: BTreeMap<String, String>,
}

impl OwnerIndex {
    /// Creates an owner index from eligible owner paths and claimed files.
    #[must_use]
    pub(super) fn new(
        owners: Vec<(String, String)>,
        claimed_files: &BTreeMap<String, Vec<String>>,
    ) -> Self {
        let file_to_owner = claimed_files
            .iter()
            .flat_map(|(owner, files)| files.iter().map(|file| (file.clone(), owner.clone())))
            .collect();
        Self {
            owners,
            file_to_owner,
        }
    }

    fn owner_for_file(&self, rel: &str) -> Option<&str> {
        self.file_to_owner.get(rel).map(String::as_str)
    }

    fn candidates_for_segments(&self, from: &str, segments: &[String]) -> Vec<String> {
        let segment_set = segments.iter().map(String::as_str).collect::<BTreeSet<_>>();
        self.owners
            .iter()
            .filter(|(id, path)| id != from && owner_matches(path, id, &segment_set))
            .map(|(id, _)| id.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }
}

/// Extracts dependency observations from a parsed Rust tree.
pub(super) fn dependency_observations(
    tree: &tree_sitter::Tree,
    source: &str,
    rel: &str,
    owner: &str,
    index: &OwnerIndex,
) -> Result<Vec<DependencyObservation>, ReconcileError> {
    let mut observations = Vec::new();
    collect_dependencies(
        tree.root_node(),
        source.as_bytes(),
        rel,
        owner,
        index,
        &mut observations,
    )?;
    Ok(observations)
}

fn collect_dependencies(
    node: tree_sitter::Node<'_>,
    source: &[u8],
    rel: &str,
    owner: &str,
    index: &OwnerIndex,
    observations: &mut Vec<DependencyObservation>,
) -> Result<(), ReconcileError> {
    match node.kind() {
        "use_declaration" => {
            let text = node_text(node, source)?;
            push_use_observation(node, &text, rel, owner, index, observations);
        }
        "mod_item" => {
            let text = node_text(node, source)?;
            push_mod_observation(node, &text, rel, owner, index, observations);
        }
        _ => {}
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_dependencies(child, source, rel, owner, index, observations)?;
    }
    Ok(())
}

fn push_use_observation(
    node: tree_sitter::Node<'_>,
    text: &str,
    rel: &str,
    owner: &str,
    index: &OwnerIndex,
    observations: &mut Vec<DependencyObservation>,
) {
    let segments = rust_path_segments(text);
    let candidates = index.candidates_for_segments(owner, &segments);
    push_observation(node, text, rel, owner, candidates, observations);
}

fn push_mod_observation(
    node: tree_sitter::Node<'_>,
    text: &str,
    rel: &str,
    owner: &str,
    index: &OwnerIndex,
    observations: &mut Vec<DependencyObservation>,
) {
    let Some(module) = mod_name(text) else {
        return;
    };
    let mut candidates = module_file_candidates(rel, &module)
        .iter()
        .filter_map(|candidate| index.owner_for_file(candidate).map(ToOwned::to_owned))
        .filter(|candidate| candidate != owner)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if candidates.is_empty() {
        candidates = index.candidates_for_segments(owner, &[module]);
    }
    push_observation(node, text, rel, owner, candidates, observations);
}

fn push_observation(
    node: tree_sitter::Node<'_>,
    text: &str,
    rel: &str,
    owner: &str,
    candidates: Vec<String>,
    observations: &mut Vec<DependencyObservation>,
) {
    if candidates.is_empty() {
        return;
    }
    let position = node.start_position();
    let to = (candidates.len() == 1).then(|| candidates[0].clone());
    let confidence = if to.is_some() {
        ObservationConfidence::High
    } else {
        ObservationConfidence::Low
    };
    observations.push(DependencyObservation {
        from: owner.to_owned(),
        to,
        candidates,
        reference: normalize_source_text(text),
        path: rel.to_owned(),
        line: position.row + 1,
        column: position.column + 1,
        confidence,
    });
}

/// Extracts supported Cairn fact lines from Rust module doc comments.
#[must_use]
pub(super) fn docstring_facts(source: &str, rel: &str, owner: &str) -> Option<DocstringFacts> {
    let mut facts = Vec::new();
    if module_level_doc_file(rel) {
        collect_inner_doc_facts(source, &mut facts);
    }
    collect_outer_mod_doc_facts(source, &mut facts);
    (!facts.is_empty()).then(|| DocstringFacts {
        owner: owner.to_owned(),
        path: rel.to_owned(),
        facts,
    })
}

fn collect_inner_doc_facts(source: &str, facts: &mut Vec<DocstringFact>) {
    for (index, line) in source.lines().enumerate() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with("//!") {
            if !trimmed.is_empty() {
                break;
            }
            continue;
        }
        let column = line.len() - trimmed.len() + 4;
        collect_fact_line(&trimmed[3..], index + 1, column, facts);
    }
}

fn collect_outer_mod_doc_facts(source: &str, facts: &mut Vec<DocstringFact>) {
    let lines = source.lines().collect::<Vec<_>>();
    let mut index = 0;
    while index < lines.len() {
        let trimmed = lines[index].trim_start();
        if !trimmed.starts_with("///") {
            index += 1;
            continue;
        }
        let start = index;
        while index < lines.len() && lines[index].trim_start().starts_with("///") {
            index += 1;
        }
        if lines
            .get(index)
            .is_some_and(|line| looks_like_mod_declaration(line.trim_start()))
        {
            for (fact_index, line) in lines.iter().enumerate().take(index).skip(start) {
                let doc = line.trim_start();
                let column = line.len() - doc.len() + 4;
                collect_fact_line(&doc[3..], fact_index + 1, column, facts);
            }
        }
    }
}

fn collect_fact_line(text: &str, line: usize, column: usize, facts: &mut Vec<DocstringFact>) {
    let stripped = text.trim_start();
    if let Some(rest) = stripped.strip_prefix("Cairn-")
        && let Some((key, value)) = rest.split_once(':')
    {
        facts.push(DocstringFact {
            key: key.to_owned(),
            value: value.trim_ascii().to_owned(),
            line,
            column,
        });
    }
}

/// Returns true when the path is a Rust module root file.
#[must_use]
pub(super) fn module_level_doc_file(rel: &str) -> bool {
    Path::new(rel)
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| matches!(name, "lib.rs" | "main.rs" | "mod.rs"))
}

/// Collects eligible owner paths from a node tree.
pub(super) fn collect_owner(node: &Node, owners: &mut Vec<(String, String)>) {
    let is_internal = !node.children.is_empty();
    if !is_internal || node.owns_files {
        for path in &node.paths {
            owners.push((node.id.clone(), trim_dot(path)));
        }
    }
    for child in &node.children {
        collect_owner(child, owners);
    }
}

fn owner_matches(path: &str, id: &str, segments: &BTreeSet<&str>) -> bool {
    path.rsplit('/')
        .next()
        .is_some_and(|last| segments.contains(last))
        || id
            .rsplit('.')
            .next()
            .is_some_and(|last| segments.contains(last))
}

fn module_file_candidates(rel: &str, module: &str) -> Vec<String> {
    let base = Path::new(rel)
        .parent()
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .unwrap_or_default();
    if base.is_empty() {
        vec![format!("{module}.rs"), format!("{module}/mod.rs")]
    } else {
        vec![
            format!("{base}/{module}.rs"),
            format!("{base}/{module}/mod.rs"),
        ]
    }
}

fn rust_path_segments(text: &str) -> Vec<String> {
    text.split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
        .filter(|part| !part.is_empty())
        .filter(|part| !matches!(*part, "use" | "pub" | "crate" | "self" | "super" | "as"))
        .map(ToOwned::to_owned)
        .collect()
}

fn mod_name(text: &str) -> Option<String> {
    let mut iter = text
        .split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
        .filter(|part| !part.is_empty())
        .filter(|part| !matches!(*part, "pub" | "mod"));
    iter.next().map(ToOwned::to_owned)
}

fn looks_like_mod_declaration(line: &str) -> bool {
    line.starts_with("mod ") || line.starts_with("pub mod ")
}

fn node_text(node: tree_sitter::Node<'_>, source: &[u8]) -> Result<String, ReconcileError> {
    node.utf8_text(source)
        .map(normalize_source_text)
        .map_err(|error| ReconcileError {
            code: "CAIRN_RECONCILE_SOURCE_TEXT".to_owned(),
            message: error.to_string(),
        })
}

fn normalize_source_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn trim_dot(path: &str) -> String {
    path.trim_start_matches("./").to_owned()
}
