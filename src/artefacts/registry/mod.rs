//! Typed artefact registry and Phase 2 loaders.

// Reason: sibling modules (`io`, `parse`, `validate`, `kinds`) use `use super::*`
// and inherit this parent import surface; several names are only consumed there.
#![allow(unused_imports)]

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use crate::blueprint::Ast;

use super::{contract::ContractSet, frontmatter};
mod changes;
pub(crate) mod dates;
mod io;
mod kinds;
pub(crate) mod manifest;
mod parse;
pub(crate) mod sha256;
#[cfg(test)]
mod tests;
/// Artefact type definitions.
pub mod types;
mod validate;

use changes::load_changes;
use io::{
    collect_ids, list, markdown_paths, optional, parse_file, path_string, pointers, required,
};
use kinds::{decisions_kind, load_kind, load_kinds, parse_claims};
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
        decision_pointers: pointers(ast, "decisions"),
        ..ArtefactSet::default()
    };
    load_kinds(root, ast, &mut set);
    compute_reverse_provenance(&mut set.decisions);
    load_changes(root, &mut set);
    validate_integrity(root, &ids, &mut set);
    set
}

/// Load decision artefacts from all `decisions` pointers declared in `ast`.
///
/// Thin wrapper over the kind table for callers (e.g. architecture hooks) that
/// only need decisions without a full artefact load.
pub(crate) fn load_decisions(root: &Path, ast: &Ast, set: &mut ArtefactSet) {
    set.decision_pointers = pointers(ast, "decisions");
    load_kind(root, ast, decisions_kind(), set);
    compute_reverse_provenance(&mut set.decisions);
}

/// Derive reverse provenance from the loaded forward decision references.
///
/// Unknown targets are omitted here and remain the responsibility of validation.
pub(crate) fn compute_reverse_provenance(decisions: &mut [Decision]) {
    let known_ids = decisions
        .iter()
        .map(|decision| decision.id.as_str())
        .collect::<BTreeSet<_>>();
    let mut refined_by = BTreeMap::<String, BTreeSet<String>>::new();
    let mut superseded_by = BTreeMap::<String, BTreeSet<String>>::new();
    for decision in decisions.iter() {
        for target in &decision.refines {
            if known_ids.contains(target.as_str()) {
                refined_by
                    .entry(target.clone())
                    .or_default()
                    .insert(decision.id.clone());
            }
        }
        for target in &decision.supersedes {
            if known_ids.contains(target.as_str()) {
                superseded_by
                    .entry(target.clone())
                    .or_default()
                    .insert(decision.id.clone());
            }
        }
    }
    for decision in decisions {
        decision.refined_by = refined_by
            .remove(&decision.id)
            .map_or_else(Vec::new, |ids| ids.into_iter().collect());
        decision.superseded_by = superseded_by
            .remove(&decision.id)
            .map_or_else(Vec::new, |ids| ids.into_iter().collect());
    }
}
