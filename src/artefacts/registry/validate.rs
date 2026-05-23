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
        if research.sources.is_empty() {
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
#[cfg(test)]
mod tests {
    #![allow(clippy::field_reassign_with_default)]
    // Reason: two-step Default + field assignment is more readable in test fixtures
    // than the single-expression struct-update alternative when multiple fields vary.
    use std::{collections::BTreeSet, path::Path};

    use super::{sha256::sha256_hex, *};

    // ── fixtures ─────────────────────────────────────────────────────────────

    fn node_ids(ids: &[&str]) -> BTreeSet<String> {
        ids.iter().map(ToString::to_string).collect()
    }

    fn make_todo(node: &str) -> Todo {
        Todo {
            path: "meta/todos/t.md".to_owned(),
            node: node.to_owned(),
            status: TodoStatus::Open,
            created: "2024-01-01".to_owned(),
            satisfies: None,
            body: String::new(),
        }
    }

    fn make_review(node: &str) -> Review {
        Review {
            path: "meta/reviews/r.md".to_owned(),
            node: node.to_owned(),
            review_type: ReviewType::Human,
            date: "2024-01-01".to_owned(),
            reviewer: "alice".to_owned(),
            related_change: None,
            body: String::new(),
        }
    }

    fn make_research(id: &str, nodes: &[&str], sources: &[&str]) -> Research {
        Research {
            id: id.to_owned(),
            path: format!("meta/research/{id}.md"),
            nodes: nodes.iter().map(ToString::to_string).collect(),
            date: "2024-01-01".to_owned(),
            sources: sources.iter().map(ToString::to_string).collect(),
            tags: Vec::new(),
            body: String::new(),
        }
    }

    fn make_decision(id: &str, nodes: &[&str], status: DecisionStatus) -> Decision {
        Decision {
            id: id.to_owned(),
            path: format!("meta/decisions/{id}.md"),
            nodes: nodes.iter().map(ToString::to_string).collect(),
            status,
            date: "2024-01-01".to_owned(),
            revisited: None,
            revisit_triggers: Vec::new(),
            informed_by: Vec::new(),
            supersedes: Vec::new(),
            refines: Vec::new(),
            related: Vec::new(),
            orphaned: false,
            orphan_reason: None,
            claims: None,
            body: String::new(),
        }
    }

    fn make_source(id: &str, verification: SourceVerification, file: &str) -> Source {
        Source {
            id: id.to_owned(),
            path: format!("meta/sources/{id}.md"),
            file: file.to_owned(),
            sha256: None,
            verification,
            source_type: "paper".to_owned(),
            date: "2024-01-01".to_owned(),
            tags: Vec::new(),
            description: String::new(),
            body: String::new(),
        }
    }

    fn finding_codes(set: &ArtefactSet) -> Vec<&str> {
        set.findings.iter().map(|f| f.code.as_str()).collect()
    }

    // ── validate_nodes ────────────────────────────────────────────────────────

    #[test]
    fn test_todo_unknown_node_emits_orphan_warning() {
        let mut set = ArtefactSet::default();
        set.todos = vec![make_todo("app.ghost")];
        validate_nodes(&node_ids(&["app.real"]), &mut set);
        assert!(
            finding_codes(&set).contains(&"CAIRN_TODO_ORPHAN_NODE"),
            "todo referencing unknown node must emit CAIRN_TODO_ORPHAN_NODE; got: {:?}",
            finding_codes(&set)
        );
    }

    #[test]
    fn test_review_unknown_node_emits_error() {
        let mut set = ArtefactSet::default();
        set.reviews = vec![make_review("app.ghost")];
        validate_nodes(&node_ids(&["app.real"]), &mut set);
        assert!(finding_codes(&set).contains(&"CAIRN_REVIEW_UNKNOWN_NODE"));
    }

    #[test]
    fn test_research_empty_nodes_emits_missing_nodes_error() {
        let mut set = ArtefactSet::default();
        set.research = vec![make_research("r1", &[], &["src1"])];
        validate_nodes(&node_ids(&["app.real"]), &mut set);
        assert!(finding_codes(&set).contains(&"CAIRN_ARTEFACT_MISSING_NODES"));
    }

    #[test]
    fn test_research_unknown_node_emits_unknown_node_error() {
        let mut set = ArtefactSet::default();
        set.research = vec![make_research("r1", &["app.ghost"], &["src1"])];
        validate_nodes(&node_ids(&["app.real"]), &mut set);
        assert!(finding_codes(&set).contains(&"CAIRN_ARTEFACT_UNKNOWN_NODE"));
    }

    #[test]
    fn test_decision_empty_nodes_emits_missing_nodes_error() {
        let mut set = ArtefactSet::default();
        set.decisions = vec![make_decision("d1", &[], DecisionStatus::Accepted)];
        validate_nodes(&node_ids(&["app.real"]), &mut set);
        assert!(finding_codes(&set).contains(&"CAIRN_DECISION_MISSING_NODES"));
    }

    #[test]
    fn test_decision_only_unknown_nodes_emits_orphaned_error() {
        let mut set = ArtefactSet::default();
        set.decisions = vec![make_decision(
            "d1",
            &["app.ghost"],
            DecisionStatus::Accepted,
        )];
        validate_nodes(&node_ids(&["app.real"]), &mut set);
        assert!(finding_codes(&set).contains(&"CAIRN_DECISION_ORPHANED"));
    }

    #[test]
    fn test_decision_orphaned_flag_with_reason_suppresses_error() {
        let mut set = ArtefactSet::default();
        let mut d = make_decision("d1", &["app.ghost"], DecisionStatus::Accepted);
        d.orphaned = true;
        d.orphan_reason = Some("node was retired".to_owned());
        set.decisions = vec![d];
        validate_nodes(&node_ids(&["app.real"]), &mut set);
        assert!(
            !finding_codes(&set).contains(&"CAIRN_DECISION_ORPHANED"),
            "orphaned=true with reason must suppress CAIRN_DECISION_ORPHANED"
        );
    }

    #[test]
    fn test_validate_nodes_happy_path_no_findings() {
        let mut set = ArtefactSet::default();
        set.todos = vec![make_todo("app.real")];
        set.reviews = vec![make_review("app.real")];
        set.research = vec![make_research("r1", &["app.real"], &[])];
        set.decisions = vec![make_decision("d1", &["app.real"], DecisionStatus::Accepted)];
        validate_nodes(&node_ids(&["app.real"]), &mut set);
        assert!(
            set.findings.is_empty(),
            "all known refs must produce no findings"
        );
    }

    // ── validate_decision_refs ────────────────────────────────────────────────

    #[test]
    fn test_decision_supersedes_unknown_decision_emits_warning() {
        let mut set = ArtefactSet::default();
        let mut d = make_decision("d1", &["app.real"], DecisionStatus::Accepted);
        d.supersedes = vec!["d.ghost".to_owned()];
        set.decisions = vec![d];
        let decisions = set
            .decisions
            .iter()
            .map(|d| (d.id.clone(), d.status))
            .collect();
        validate_decision_refs(&decisions, &mut set);
        assert!(finding_codes(&set).contains(&"CAIRN_DECISION_REFERENCE_UNKNOWN"));
    }

    #[test]
    fn test_decision_supersedes_non_superseded_emits_status_warning() {
        let mut set = ArtefactSet::default();
        let target = make_decision("d.old", &["app.real"], DecisionStatus::Accepted);
        let mut d = make_decision("d.new", &["app.real"], DecisionStatus::Accepted);
        d.supersedes = vec!["d.old".to_owned()];
        set.decisions = vec![d, target];
        let decisions = set
            .decisions
            .iter()
            .map(|d| (d.id.clone(), d.status))
            .collect();
        validate_decision_refs(&decisions, &mut set);
        assert!(finding_codes(&set).contains(&"CAIRN_DECISION_SUPERSEDES_STATUS"));
    }

    #[test]
    fn test_decision_supersedes_superseded_no_warning() {
        let mut set = ArtefactSet::default();
        let target = make_decision("d.old", &["app.real"], DecisionStatus::Superseded);
        let mut d = make_decision("d.new", &["app.real"], DecisionStatus::Accepted);
        d.supersedes = vec!["d.old".to_owned()];
        set.decisions = vec![d, target];
        let decisions = set
            .decisions
            .iter()
            .map(|d| (d.id.clone(), d.status))
            .collect();
        validate_decision_refs(&decisions, &mut set);
        assert!(
            !finding_codes(&set).contains(&"CAIRN_DECISION_SUPERSEDES_STATUS"),
            "superseding a Superseded decision must produce no warning"
        );
    }

    // ── validate_provenance_refs ──────────────────────────────────────────────

    #[test]
    fn test_research_no_sources_emits_missing_sources_error() {
        let mut set = ArtefactSet::default();
        set.research = vec![make_research("r1", &["app.real"], &[])];
        let source_ids = BTreeSet::new();
        validate_provenance_refs(&BTreeSet::new(), &source_ids, &mut set);
        assert!(finding_codes(&set).contains(&"CAIRN_RESEARCH_MISSING_SOURCES"));
    }

    #[test]
    fn test_research_unknown_source_emits_warning() {
        let mut set = ArtefactSet::default();
        set.research = vec![make_research("r1", &["app.real"], &["src.ghost"])];
        let source_ids = node_ids(&["src.real"]);
        validate_provenance_refs(&BTreeSet::new(), &source_ids, &mut set);
        assert!(finding_codes(&set).contains(&"CAIRN_RESEARCH_UNKNOWN_SOURCE"));
    }

    #[test]
    fn test_decision_unknown_provenance_emits_warning() {
        let mut set = ArtefactSet::default();
        let mut d = make_decision("d1", &["app.real"], DecisionStatus::Accepted);
        d.informed_by = vec!["research.ghost".to_owned()];
        set.decisions = vec![d];
        validate_provenance_refs(&BTreeSet::new(), &BTreeSet::new(), &mut set);
        assert!(finding_codes(&set).contains(&"CAIRN_DECISION_UNKNOWN_PROVENANCE"));
    }

    // ── validate_sources ──────────────────────────────────────────────────────

    #[test]
    fn test_unreferenced_source_emits_orphan_warning() {
        let mut set = ArtefactSet::default();
        set.sources = vec![make_source(
            "src1",
            SourceVerification::Unverified,
            "file.pdf",
        )];
        // No research or decisions reference src1.
        validate_sources(Path::new("/tmp"), &node_ids(&["src1"]), &mut set);
        assert!(finding_codes(&set).contains(&"CAIRN_SOURCE_ORPHAN"));
    }

    #[test]
    fn test_external_source_non_url_emits_error() {
        let mut set = ArtefactSet::default();
        set.sources = vec![make_source(
            "src1",
            SourceVerification::External,
            "./local.pdf",
        )];
        // Reference it so it doesn't also emit CAIRN_SOURCE_ORPHAN.
        set.research = vec![make_research("r1", &["app.real"], &["src1"])];
        validate_sources(Path::new("/tmp"), &node_ids(&["src1"]), &mut set);
        assert!(finding_codes(&set).contains(&"CAIRN_SOURCE_EXTERNAL_URL"));
    }

    #[test]
    fn test_external_source_valid_url_no_error() {
        let mut set = ArtefactSet::default();
        set.sources = vec![make_source(
            "src1",
            SourceVerification::External,
            "https://example.com/paper.pdf",
        )];
        set.research = vec![make_research("r1", &["app.real"], &["src1"])];
        validate_sources(Path::new("/tmp"), &node_ids(&["src1"]), &mut set);
        assert!(
            !finding_codes(&set).contains(&"CAIRN_SOURCE_EXTERNAL_URL"),
            "external source with https URL must not error"
        );
    }

    #[test]
    fn test_unverified_source_emits_info() {
        let mut set = ArtefactSet::default();
        set.sources = vec![make_source(
            "src1",
            SourceVerification::Unverified,
            "file.pdf",
        )];
        set.research = vec![make_research("r1", &["app.real"], &["src1"])];
        validate_sources(Path::new("/tmp"), &node_ids(&["src1"]), &mut set);
        assert!(finding_codes(&set).contains(&"CAIRN_SOURCE_UNVERIFIED"));
    }

    #[test]
    fn test_indexed_but_missing_source_emits_index_gap_warning() {
        let mut set = ArtefactSet::default();
        // source_ids contains "src.missing" but set.sources does not.
        validate_sources(Path::new("/tmp"), &node_ids(&["src.missing"]), &mut set);
        assert!(finding_codes(&set).contains(&"CAIRN_SOURCE_INDEX_GAP"));
    }

    // ── validate_verified_source ──────────────────────────────────────────────

    #[test]
    fn test_verified_source_without_sha256_emits_error() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("paper.pdf"), b"content").unwrap();
        let mut set = ArtefactSet::default();
        let source = make_source("src1", SourceVerification::Verified, "paper.pdf");
        // sha256 field is None — validated source must require it.
        validate_verified_source(dir.path(), &source, &mut set);
        assert!(finding_codes(&set).contains(&"CAIRN_SOURCE_SHA256_MISSING"));
    }

    #[test]
    fn test_verified_source_wrong_sha256_emits_mismatch_error() {
        let dir = tempfile::tempdir().unwrap();
        let content = b"hello world";
        std::fs::write(dir.path().join("paper.pdf"), content).unwrap();
        let mut set = ArtefactSet::default();
        let mut source = make_source("src1", SourceVerification::Verified, "paper.pdf");
        source.sha256 =
            Some("0000000000000000000000000000000000000000000000000000000000000000".to_owned());
        validate_verified_source(dir.path(), &source, &mut set);
        assert!(finding_codes(&set).contains(&"CAIRN_SOURCE_SHA256_MISMATCH"));
    }

    #[test]
    fn test_verified_source_correct_sha256_no_finding() {
        let dir = tempfile::tempdir().unwrap();
        let content = b"hello world";
        std::fs::write(dir.path().join("paper.pdf"), content).unwrap();
        let mut set = ArtefactSet::default();
        let mut source = make_source("src1", SourceVerification::Verified, "paper.pdf");
        source.sha256 = Some(sha256_hex(content));
        validate_verified_source(dir.path(), &source, &mut set);
        assert!(
            set.findings.is_empty(),
            "correct sha256 must produce no findings; got: {:?}",
            set.findings
        );
    }
}
