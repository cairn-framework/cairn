//! Decision ratification-tier scanner checks.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use super::ArtefactSet;
use crate::{
    artefacts::registry::{
        Decision, DecisionStatus, RatificationTier, Review,
        manifest::{self, RepoPathRule},
    },
    blueprint::NodeKind,
    map::{
        Graph,
        graph::{Finding, FindingSeverity},
    },
};

const ALLOWLIST_PATH: &str = "docs/registries/binding-surface.md";

mod convergence;
use convergence::{convergence_leg, machine_debate_record};

pub(super) fn check_ratification(graph: &mut Graph, artefacts: &ArtefactSet, root: &Path) {
    let canonical_root = root.canonicalize().ok();
    // The allowlist is load-bearing only for local-tier claims: a repository
    // with no local decision never pays for the file's absence, while a
    // repository holding one fails closed the moment the list is missing or
    // malformed (the tier claim cannot be validated without it).
    let has_local = artefacts.decisions.iter().any(is_local);
    let allowlist = if has_local {
        match load_allowlist(root, canonical_root.as_deref()) {
            Ok(rules) => Some(rules),
            Err(message) => {
                graph.findings.push(finding(
                    "CAIRN_DECISION_TIER_BINDING_PATH",
                    FindingSeverity::Error,
                    message,
                    None,
                    Some(ALLOWLIST_PATH.to_owned()),
                ));
                None
            }
        }
    } else {
        None
    };

    let mut manifests = BTreeMap::new();
    for decision in artefacts
        .decisions
        .iter()
        .filter(|decision| is_local(decision))
    {
        match manifest::compute_decision_subject_hash(root, decision) {
            Ok(hash) => {
                manifests.insert(decision.id.as_str(), hash);
            }
            Err(error) if decision.status == DecisionStatus::Accepted => {
                graph.findings.push(convergence_finding(
                    decision,
                    &format!("subject manifest unavailable: {}", error.message),
                ));
            }
            Err(_) => {}
        }
    }

    for decision in &artefacts.decisions {
        if is_local(decision) {
            check_local_scope(
                graph,
                decision,
                allowlist.as_deref(),
                canonical_root.as_deref(),
                root,
            );
        }
        if decision.ratification == RatificationTier::Binding && decision.ratified_by_machine {
            graph.findings.push(binding_machine_finding(decision));
        }
        if decision.status == DecisionStatus::Accepted
            && is_local(decision)
            && decision.ratified_by_machine
            && let Err(leg) = machine_debate_record(&decision.body)
        {
            graph.findings.push(convergence_finding(decision, leg));
        }
        if decision.status == DecisionStatus::Accepted
            && is_local(decision)
            && let Some(manifest) = manifests.get(decision.id.as_str())
            && let Some(leg) = convergence_leg(
                canonical_root.as_deref().unwrap_or(root),
                decision,
                artefacts,
                manifest,
            )
        {
            graph.findings.push(convergence_finding(decision, &leg));
        }
    }

    // Audit pointer: subject_hash-only. A stale receipt-like review with the
    // wrong review_type still matches nothing and deserves the pointer; wrong
    // type is the convergence check's business when referenced.
    for review in artefacts
        .reviews
        .iter()
        .filter(|review| review.subject_hash.is_some())
    {
        if !manifests
            .values()
            .any(|hash| review.subject_hash.as_ref() == Some(hash))
        {
            graph.findings.push(unmatched_subject_finding(review));
        }
    }
}
fn load_allowlist(root: &Path, canonical_root: Option<&Path>) -> Result<Vec<RepoPathRule>, String> {
    let source = fs::read_to_string(root.join(ALLOWLIST_PATH)).map_err(|error| {
        format!("Cannot read binding-surface allowlist `{ALLOWLIST_PATH}`: {error}")
    })?;
    let parsed = manifest::parse_allowlist(&source)
        .map_err(|reason| format!("Binding-surface allowlist `{ALLOWLIST_PATH}` has {reason}"))?;
    let mut rules = BTreeSet::new();
    for rule in parsed {
        let row = match &rule {
            RepoPathRule::File(path) => path.clone(),
            RepoPathRule::Dir(path) => format!("{path}/"),
        };
        rules.insert(rule.clone());
        match canonical_root.map(|root| canonical_repo_rule(root, &rule)) {
            Some(CanonicalOutcome::Other(resolved)) => {
                rules.insert(resolved);
            }
            Some(CanonicalOutcome::Escapes) => {
                return Err(format!(
                    "Binding-surface allowlist `{ALLOWLIST_PATH}` has a rule resolving outside the repository: `{row}`"
                ));
            }
            _ => {}
        }
    }
    if rules.is_empty() {
        return Err(format!(
            "Binding-surface allowlist `{ALLOWLIST_PATH}` has no valid rules"
        ));
    }
    Ok(rules.into_iter().collect())
}

fn check_local_scope(
    graph: &mut Graph,
    decision: &Decision,
    allowlist: Option<&[RepoPathRule]>,
    canonical_root: Option<&Path>,
    root_for_members: &Path,
) {
    let containers: BTreeSet<_> = decision
        .nodes
        .iter()
        .filter_map(|node| container_for(graph, node))
        .collect();
    if containers.len() > 1 {
        graph
            .findings
            .push(copy_finding("CAIRN_DECISION_TIER_SPAN", decision, None));
    }
    if !decision.supersedes.is_empty() {
        graph.findings.push(copy_finding(
            "CAIRN_DECISION_TIER_SUPERSEDES",
            decision,
            None,
        ));
    }
    for affects in &decision.affects {
        let Some(affects_rule) = manifest::normalise_repo_entry(affects) else {
            continue;
        };
        let canonical = canonical_root.map_or(CanonicalOutcome::Unresolvable, |root| {
            canonical_repo_rule(root, &affects_rule)
        });
        if matches!(canonical, CanonicalOutcome::Escapes) {
            graph.findings.push(finding(
                "CAIRN_DECISION_AFFECTS_INVALID",
                FindingSeverity::Error,
                format!(
                    "decision `{}` affects entry `{affects}` resolves outside the repository (symlinks resolve before matching)",
                    decision.id
                ),
                None,
                Some(decision.path.clone()),
            ));
            continue;
        }
        let Some(allowlist) = allowlist else {
            continue;
        };
        // Classify what the entry really governs: a nested alias inside a
        // directory rule can resolve into a binding surface the entry's own
        // canonical form never reveals. An expansion failure is a refusal,
        // never an empty governed set (escapes are Errors, not misses).
        let governed = match manifest::governed_canonical_files(root_for_members, &affects_rule) {
            Ok(paths) => paths,
            Err(error) => {
                graph.findings.push(finding(
                    "CAIRN_DECISION_AFFECTS_INVALID",
                    FindingSeverity::Error,
                    format!(
                        "decision `{}` affects entry `{affects}` cannot be classified: {}",
                        decision.id, error.message
                    ),
                    None,
                    Some(decision.path.clone()),
                ));
                continue;
            }
        };
        let member_hit = governed.iter().any(|file| {
            allowlist
                .iter()
                .any(|allow| manifest::rule_matches(allow, file))
        });
        let lexical_hit = member_hit
            || allowlist
                .iter()
                .any(|allow| rules_overlap(allow, &affects_rule));
        let canonical_hit = !lexical_hit
            && match &canonical {
                CanonicalOutcome::Other(resolved) => {
                    allowlist.iter().any(|allow| rules_overlap(allow, resolved))
                }
                _ => false,
            };
        if lexical_hit || canonical_hit {
            graph.findings.push(copy_finding(
                "CAIRN_DECISION_TIER_BINDING_PATH",
                decision,
                Some(affects),
            ));
        }
    }
}

/// Outcome of resolving an affects rule against the real file system.
enum CanonicalOutcome {
    /// Path resolves to itself (no symlink indirection).
    Same,
    /// Path resolves to a different repo-relative form (symlink inside root).
    Other(RepoPathRule),
    /// Path resolves outside the repository root.
    Escapes,
    /// Path does not exist or cannot be resolved.
    Unresolvable,
}
/// Resolves an affects rule to its canonical on-disk repo-relative form, so a
/// symlink cannot launder a binding-surface path through a non-binding name.
fn canonical_repo_rule(root: &Path, rule: &RepoPathRule) -> CanonicalOutcome {
    let (path, is_dir) = match rule {
        RepoPathRule::File(path) => (path, false),
        RepoPathRule::Dir(path) => (path, true),
    };
    let Ok(canonical) = root.join(path).canonicalize() else {
        return CanonicalOutcome::Unresolvable;
    };
    let Ok(relative) = canonical.strip_prefix(root) else {
        return CanonicalOutcome::Escapes;
    };
    let mut text = String::new();
    for part in relative.components() {
        if !text.is_empty() {
            text.push('/');
        }
        let Some(part) = part.as_os_str().to_str() else {
            return CanonicalOutcome::Unresolvable;
        };
        text.push_str(part);
    }
    if text.is_empty() {
        return CanonicalOutcome::Unresolvable;
    }
    let resolved = if is_dir {
        RepoPathRule::Dir(text)
    } else {
        RepoPathRule::File(text)
    };
    if &resolved == rule {
        CanonicalOutcome::Same
    } else {
        CanonicalOutcome::Other(resolved)
    }
}

fn container_for(graph: &Graph, node_id: &str) -> Option<String> {
    let mut candidate = graph.nodes.get(node_id)?;
    let mut visited = BTreeSet::new();
    loop {
        if !visited.insert(candidate.id.as_str()) {
            return None;
        }
        if matches!(candidate.kind, NodeKind::Container | NodeKind::System) {
            return Some(candidate.id.clone());
        }
        if let Some(parent) = candidate.parent.as_deref() {
            candidate = graph.nodes.get(parent)?;
            continue;
        }
        let parent_id = candidate.id.rsplit_once('.')?.0;
        candidate = graph.nodes.get(parent_id)?;
    }
}

fn rules_overlap(left: &RepoPathRule, right: &RepoPathRule) -> bool {
    match (left, right) {
        (RepoPathRule::File(path), rule) | (rule, RepoPathRule::File(path)) => {
            manifest::rule_matches(rule, path)
        }
        (RepoPathRule::Dir(left), RepoPathRule::Dir(right)) => {
            left == right
                || left
                    .strip_prefix(right)
                    .is_some_and(|rest| rest.starts_with('/'))
                || right
                    .strip_prefix(left)
                    .is_some_and(|rest| rest.starts_with('/'))
        }
    }
}

fn is_local(decision: &Decision) -> bool {
    decision.ratification == RatificationTier::Local
}

fn copy_finding(code: &str, decision: &Decision, path: Option<&str>) -> Finding {
    let mut message = crate::copy::lookup(&format!("findings.codes.{code}.body"))
        .replace("{decision}", &decision.id);
    if let Some(path) = path {
        message = message.replace("{path}", path);
    }
    finding(
        code,
        FindingSeverity::Error,
        message,
        decision.nodes.first().cloned(),
        Some(decision.path.clone()),
    )
}

fn convergence_finding(decision: &Decision, leg: &str) -> Finding {
    let message = crate::copy::lookup("findings.codes.CAIRN_DECISION_CONVERGENCE_UNMET.body")
        .replace("{decision}", &decision.id)
        .replace("{leg}", leg);
    finding(
        "CAIRN_DECISION_CONVERGENCE_UNMET",
        FindingSeverity::Error,
        message,
        decision.nodes.first().cloned(),
        Some(decision.path.clone()),
    )
}

fn binding_machine_finding(decision: &Decision) -> Finding {
    copy_finding("CAIRN_DECISION_MACHINE_BINDING", decision, None)
}

fn unmatched_subject_finding(review: &Review) -> Finding {
    let message = crate::copy::lookup("findings.codes.CAIRN_REVIEW_SUBJECT_UNMATCHED.body")
        .replace("{path}", &review.path);
    finding(
        "CAIRN_REVIEW_SUBJECT_UNMATCHED",
        FindingSeverity::Info,
        message,
        Some(review.node.clone()),
        Some(review.path.clone()),
    )
}

fn finding(
    code: &str,
    severity: FindingSeverity,
    message: String,
    node: Option<String>,
    path: Option<String>,
) -> Finding {
    Finding {
        code: code.to_owned(),
        severity,
        message,
        node,
        target: None,
        path,
        deferred_by: None,
        parked_by: None,
    }
}

#[cfg(test)]
mod tests;
