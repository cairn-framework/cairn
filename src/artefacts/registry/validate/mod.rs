//! Integrity validation for loaded artefacts.

// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::*;
use io::{error, info, is_url, warning};
use sha256::sha256_hex;
use std::collections::BTreeMap;

pub(super) fn validate_integrity(root: &Path, node_ids: &BTreeSet<String>, set: &mut ArtefactSet) {
    let research_ids = set
        .research
        .iter()
        .map(|item| item.id.clone())
        .collect::<BTreeSet<_>>();
    let source_ids = set
        .sources
        .iter()
        .map(|item| item.id.clone())
        .collect::<BTreeSet<_>>();
    let decisions = set
        .decisions
        .iter()
        .map(|item| (item.id.clone(), item.status))
        .collect::<BTreeMap<_, _>>();
    validate_nodes(node_ids, set);
    validate_decision_refs(&decisions, set);
    validate_provenance_refs(&research_ids, &source_ids, set);
    validate_sources(root, &source_ids, set);
    validate_decision_claims(root, set);
}

pub(super) fn validate_nodes(node_ids: &BTreeSet<String>, set: &mut ArtefactSet) {
    for todo in &set.todos {
        if !node_ids.contains(&todo.node) {
            set.findings.push(warning(
                "CAIRN_TODO_ORPHAN_NODE",
                format!(
                    "todo `{}` references unknown node `{}`",
                    todo.path, todo.node
                ),
                Some(todo.node.clone()),
                Some(todo.path.clone()),
            ));
        }
    }
    for review in &set.reviews {
        if !node_ids.contains(&review.node) {
            set.findings.push(error(
                "CAIRN_REVIEW_UNKNOWN_NODE",
                format!(
                    "review `{}` references unknown node `{}`",
                    review.path, review.node
                ),
                Some(review.node.clone()),
                Some(review.path.clone()),
            ));
        }
    }
    let research_records = set.research.clone();
    for research in &research_records {
        validate_node_list(
            node_ids,
            &research.nodes,
            "research",
            &research.id,
            &research.path,
            set,
        );
    }
    let decision_records = set.decisions.clone();
    for decision in &decision_records {
        if decision.nodes.is_empty() {
            set.findings.push(error(
                "CAIRN_DECISION_MISSING_NODES",
                format!("decision `{}` has no nodes", decision.id),
                None,
                Some(decision.path.clone()),
            ));
            continue;
        }
        let known = decision
            .nodes
            .iter()
            .filter(|node| node_ids.contains(*node))
            .count();
        if known == 0
            && (!decision.orphaned || decision.orphan_reason.as_deref().unwrap_or("").is_empty())
        {
            set.findings.push(error(
                "CAIRN_DECISION_ORPHANED",
                format!("decision `{}` references only unknown nodes", decision.id),
                None,
                Some(decision.path.clone()),
            ));
        }
    }
}

pub(super) fn validate_node_list(
    node_ids: &BTreeSet<String>,
    nodes: &[String],
    kind: &str,
    id: &str,
    path: &str,
    set: &mut ArtefactSet,
) {
    if nodes.is_empty() {
        set.findings.push(error(
            "CAIRN_ARTEFACT_MISSING_NODES",
            format!("{kind} `{id}` has no nodes"),
            None,
            Some(path.to_owned()),
        ));
        return;
    }
    for node in nodes {
        if !node_ids.contains(node) {
            set.findings.push(error(
                "CAIRN_ARTEFACT_UNKNOWN_NODE",
                format!("{kind} `{id}` references unknown node `{node}`"),
                Some(node.clone()),
                Some(path.to_owned()),
            ));
        }
    }
}

pub(super) fn validate_decision_refs(
    decisions: &BTreeMap<String, DecisionStatus>,
    set: &mut ArtefactSet,
) {
    for decision in &set.decisions {
        for target in decision
            .supersedes
            .iter()
            .chain(decision.refines.iter())
            .chain(decision.related.iter())
        {
            let Some(status) = decisions.get(target) else {
                set.findings.push(warning(
                    "CAIRN_DECISION_REFERENCE_UNKNOWN",
                    format!(
                        "decision `{}` references unknown decision `{target}`",
                        decision.id
                    ),
                    None,
                    Some(decision.path.clone()),
                ));
                continue;
            };
            if decision.supersedes.contains(target) && *status != DecisionStatus::Superseded {
                set.findings.push(warning(
                    "CAIRN_DECISION_SUPERSEDES_STATUS",
                    format!(
                        "decision `{}` supersedes `{target}` but target is not superseded",
                        decision.id
                    ),
                    None,
                    Some(decision.path.clone()),
                ));
            }
        }
    }
}

pub(super) fn validate_provenance_refs(
    research_ids: &BTreeSet<String>,
    source_ids: &BTreeSet<String>,
    set: &mut ArtefactSet,
) {
    for research in &set.research {
        if research.sources.is_empty() && research.method != ResearchMethod::Primary {
            set.findings.push(error(
                "CAIRN_RESEARCH_MISSING_SOURCES",
                format!("research `{}` has no sources", research.id),
                None,
                Some(research.path.clone()),
            ));
        }
        for source in &research.sources {
            if !source_ids.contains(source) {
                set.findings.push(warning(
                    "CAIRN_RESEARCH_UNKNOWN_SOURCE",
                    format!(
                        "research `{}` references unknown source `{source}`",
                        research.id
                    ),
                    None,
                    Some(research.path.clone()),
                ));
            }
        }
    }
    for decision in &set.decisions {
        for reference in &decision.informed_by {
            if !research_ids.contains(reference) && !source_ids.contains(reference) {
                set.findings.push(warning(
                    "CAIRN_DECISION_UNKNOWN_PROVENANCE",
                    format!(
                        "decision `{}` references unknown provenance `{reference}`",
                        decision.id
                    ),
                    None,
                    Some(decision.path.clone()),
                ));
            }
        }
    }
}

pub(super) fn validate_sources(root: &Path, source_ids: &BTreeSet<String>, set: &mut ArtefactSet) {
    let used_sources = set
        .research
        .iter()
        .flat_map(|item| item.sources.iter().cloned())
        .chain(
            set.decisions
                .iter()
                .flat_map(|item| item.informed_by.iter().cloned()),
        )
        .collect::<BTreeSet<_>>();
    let source_records = set.sources.clone();
    for source in &source_records {
        if !used_sources.contains(&source.id) {
            set.findings.push(warning(
                "CAIRN_SOURCE_ORPHAN",
                format!("source `{}` is not referenced", source.id),
                None,
                Some(source.path.clone()),
            ));
        }
        match source.verification {
            SourceVerification::Verified => validate_verified_source(root, source, set),
            SourceVerification::External => {
                if !is_url(&source.file) {
                    set.findings.push(error(
                        "CAIRN_SOURCE_EXTERNAL_URL",
                        format!("external source `{}` file is not a URL", source.id),
                        None,
                        Some(source.path.clone()),
                    ));
                }
            }
            SourceVerification::Unverified => set.findings.push(info(
                "CAIRN_SOURCE_UNVERIFIED",
                format!("source `{}` is unverified", source.id),
                None,
                Some(source.path.clone()),
            )),
        }
    }
    for source in source_ids {
        if !set.sources.iter().any(|item| &item.id == source) {
            set.findings.push(warning(
                "CAIRN_SOURCE_INDEX_GAP",
                format!("source `{source}` is indexed but missing"),
                None,
                None,
            ));
        }
    }
}

pub(super) fn validate_verified_source(root: &Path, source: &Source, set: &mut ArtefactSet) {
    let Some(expected) = &source.sha256 else {
        set.findings.push(error(
            "CAIRN_SOURCE_SHA256_MISSING",
            format!("verified source `{}` lacks sha256", source.id),
            None,
            Some(source.path.clone()),
        ));
        return;
    };
    match fs::read(root.join(&source.file)) {
        Ok(bytes) => {
            let actual = sha256_hex(&bytes);
            if &actual != expected {
                set.findings.push(error(
                    "CAIRN_SOURCE_SHA256_MISMATCH",
                    format!("verified source `{}` sha256 mismatch", source.id),
                    None,
                    Some(source.path.clone()),
                ));
            }
        }
        Err(read_error) => set.findings.push(error(
            "CAIRN_SOURCE_READ_FAILED",
            format!(
                "failed to read verified source `{}`: {read_error}",
                source.id
            ),
            None,
            Some(source.path.clone()),
        )),
    }
}

/// Cross-check decision prose that claims to close or resolve a spec
/// open-question (`Q-NN`) against the declared-items registry. If a decision
/// body claims closure but the registry still lists the question as anything
/// other than `resolved` (or omits it), that is silent prose drift the manual
/// ratify-a-convention procedure exists to prevent. CA004.
pub(super) fn validate_decision_claims(root: &Path, set: &mut ArtefactSet) {
    let registry_path = root.join("docs/registries/declared-items.md");
    let Ok(registry) = fs::read_to_string(&registry_path) else {
        // No registry to cross-check against: nothing to validate.
        return;
    };
    let statuses = question_statuses(&registry);
    let decisions = set.decisions.clone();
    for decision in &decisions {
        for question in claimed_closed_questions(&decision.body) {
            let detail = match statuses.get(&question) {
                Some(status) if status == "resolved" => continue,
                Some(status) => {
                    format!("the declared-items registry lists it as `{status}`")
                }
                None => "it is absent from the declared-items registry".to_owned(),
            };
            set.findings.push(error(
                "CAIRN_DECISION_CLAIM_UNRESOLVED",
                format!(
                    "decision `{}` claims to close `{question}` but {detail}",
                    decision.id
                ),
                None,
                Some(decision.path.clone()),
            ));
        }
    }
}

/// Parse `| Q-NN | item | source | status | ... |` rows from the declared-items
/// registry into a question-id to status map.
fn question_statuses(registry: &str) -> BTreeMap<String, String> {
    let mut statuses = BTreeMap::new();
    for line in registry.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("| Q-") {
            continue;
        }
        let cells: Vec<&str> = trimmed
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect();
        // Columns: ID | Item | Source | Status | Resolving Phase | Notes
        if let [id, _item, _source, status, ..] = cells.as_slice() {
            statuses.insert((*id).to_owned(), (*status).to_owned());
        }
    }
    statuses
}

/// Extract the spec open-question ids a decision body claims to close or
/// resolve: a `Q-NN` token on a line that also carries a `clos*`/`resolv*` verb.
fn claimed_closed_questions(body: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    for line in body.lines() {
        let has_close_verb = line.split_whitespace().any(|word| {
            let stem = word
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase();
            stem.starts_with("clos") || stem.starts_with("resolv")
        });
        if has_close_verb {
            collect_question_ids(line, &mut found);
        }
    }
    found
}

/// Collect `Q-<digits>` tokens from a single line.
fn collect_question_ids(line: &str, found: &mut BTreeSet<String>) {
    let bytes = line.as_bytes();
    let mut i = 0;
    while i + 2 < bytes.len() {
        if bytes[i] == b'Q' && bytes[i + 1] == b'-' && bytes[i + 2].is_ascii_digit() {
            let mut j = i + 2;
            while j < bytes.len() && bytes[j].is_ascii_digit() {
                j += 1;
            }
            found.insert(line[i..j].to_owned());
            i = j;
        } else {
            i += 1;
        }
    }
}
#[cfg(test)]
mod tests;
