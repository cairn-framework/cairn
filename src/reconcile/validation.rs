//! Semantic reconciliation checks that need the built Cairn graph.

use std::collections::{BTreeMap, BTreeSet};

use crate::map::graph::{Finding, FindingSeverity, Graph};

use super::{DocstringFact, DocstringFacts, ObservationConfidence, ReconcileReport};

/// Builds advisory semantic reconciliation findings from observed source facts.
#[must_use]
pub(crate) fn semantic_findings(graph: &Graph, report: &ReconcileReport) -> Vec<Finding> {
    let mut findings = Vec::new();
    findings.extend(edge_divergence_findings(graph, report));
    findings.extend(docstring_drift_findings(graph, report));
    findings
}

fn edge_divergence_findings(graph: &Graph, report: &ReconcileReport) -> Vec<Finding> {
    let observed = report
        .dependencies
        .iter()
        .filter_map(|observation| {
            observation
                .to
                .as_ref()
                .map(|to| (observation.from.clone(), to.clone(), observation))
        })
        .collect::<Vec<_>>();
    let observed_pairs = observed
        .iter()
        .map(|(from, to, _)| (from.clone(), to.clone()))
        .collect::<BTreeSet<_>>();
    let declared_pairs = graph
        .outbound
        .values()
        .flat_map(|edges| {
            edges
                .iter()
                .map(|edge| (edge.from.clone(), edge.to.clone()))
        })
        .collect::<BTreeSet<_>>();
    let mut findings = Vec::new();
    for (from, to) in &declared_pairs {
        if !observed_pairs.contains(&(from.clone(), to.clone())) {
            findings.push(warning(
                "CE001",
                format!("declared edge `{from}` -> `{to}` has no observed Rust dependency"),
                Some(from.clone()),
                None,
            ));
        }
    }
    for (from, to, observation) in observed {
        if !declared_pairs.contains(&(from.clone(), to.clone())) {
            findings.push(warning(
                "CE002",
                format!(
                    "observed Rust dependency `{from}` -> `{to}` from `{}` at {}:{} has no declared edge",
                    observation.path, observation.line, observation.column
                ),
                Some(from),
                Some(observation.path.clone()),
            ));
        }
    }
    for observation in &report.dependencies {
        if observation.confidence == ObservationConfidence::Low {
            findings.push(warning(
                "CE003",
                format!(
                    "ambiguous Rust dependency `{}` from `{}` at {}:{} could target: {}",
                    observation.reference,
                    observation.path,
                    observation.line,
                    observation.column,
                    observation.candidates.join(", ")
                ),
                Some(observation.from.clone()),
                Some(observation.path.clone()),
            ));
        }
    }
    findings
}

fn docstring_drift_findings(graph: &Graph, report: &ReconcileReport) -> Vec<Finding> {
    report
        .docstrings
        .iter()
        .flat_map(|docstring| validate_docstring(graph, docstring))
        .collect()
}

fn validate_docstring(graph: &Graph, docstring: &DocstringFacts) -> Vec<Finding> {
    let grouped = grouped_facts(&docstring.facts);
    let node_id = grouped
        .get("ID")
        .and_then(|facts| facts.first())
        .map_or_else(|| docstring.owner.clone(), |fact| fact.value.clone());
    let mut findings = Vec::new();
    if !graph.nodes.contains_key(&node_id) {
        findings.push(doc_warning(
            "CE004",
            format!("docstring references unknown Cairn node `{node_id}`"),
            &node_id,
            docstring,
            grouped.get("ID").and_then(|facts| facts.first()),
        ));
        return findings;
    }
    if let Some(facts) = grouped.get("ID") {
        for fact in facts {
            if !graph.nodes.contains_key(&fact.value) {
                findings.push(doc_warning(
                    "CE004",
                    format!("docstring references unknown Cairn node `{}`", fact.value),
                    &node_id,
                    docstring,
                    Some(fact),
                ));
            }
        }
    }
    findings.extend(compare_docstring_facts(
        graph, docstring, &node_id, &grouped,
    ));
    findings
}

fn compare_docstring_facts(
    graph: &Graph,
    docstring: &DocstringFacts,
    node_id: &str,
    grouped: &BTreeMap<String, Vec<DocstringFact>>,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    let node = &graph.nodes[node_id];
    if let Some(fact) = grouped.get("Name").and_then(|facts| facts.first())
        && fact.value != node.name
    {
        findings.push(doc_warning(
            "CE005",
            format!(
                "docstring name `{}` contradicts map name `{}` for `{node_id}`",
                fact.value, node.name
            ),
            node_id,
            docstring,
            Some(fact),
        ));
    }
    findings.extend(compare_dependency_facts(graph, docstring, node_id, grouped));
    findings.extend(compare_list_fact(
        docstring, node_id, grouped, "Tags", &node.tags, "CE007", "tags",
    ));
    findings.extend(compare_list_fact(
        docstring,
        node_id,
        grouped,
        "Contract",
        &node.contracts,
        "CE009",
        "contracts",
    ));
    for (key, facts) in grouped {
        if !matches!(
            key.as_str(),
            "ID" | "Name" | "Depends" | "Tags" | "Contract"
        ) {
            for fact in facts {
                findings.push(doc_warning(
                    "CE008",
                    format!("unknown Cairn docstring fact key `Cairn-{key}`"),
                    node_id,
                    docstring,
                    Some(fact),
                ));
            }
        }
    }
    findings
}

fn compare_dependency_facts(
    graph: &Graph,
    docstring: &DocstringFacts,
    node_id: &str,
    grouped: &BTreeMap<String, Vec<DocstringFact>>,
) -> Vec<Finding> {
    let declared = graph
        .outbound
        .get(node_id)
        .into_iter()
        .flatten()
        .map(|edge| edge.to.clone())
        .collect::<BTreeSet<_>>();
    grouped
        .get("Depends")
        .into_iter()
        .flatten()
        .filter_map(|fact| {
            if !graph.nodes.contains_key(&fact.value) {
                return Some(doc_warning(
                    "CE004",
                    format!(
                        "docstring dependency references unknown node `{}`",
                        fact.value
                    ),
                    node_id,
                    docstring,
                    Some(fact),
                ));
            }
            (!declared.contains(&fact.value)).then(|| {
                doc_warning(
                    "CE006",
                    format!(
                        "docstring dependency `{}` is not declared from `{node_id}`",
                        fact.value
                    ),
                    node_id,
                    docstring,
                    Some(fact),
                )
            })
        })
        .collect()
}

fn compare_list_fact(
    docstring: &DocstringFacts,
    node_id: &str,
    grouped: &BTreeMap<String, Vec<DocstringFact>>,
    key: &str,
    expected: &[String],
    code: &str,
    label: &str,
) -> Vec<Finding> {
    let actual = grouped
        .get(key)
        .into_iter()
        .flatten()
        .flat_map(|fact| split_list(&fact.value))
        .collect::<BTreeSet<_>>();
    if actual.is_empty() {
        return Vec::new();
    }
    let expected = expected.iter().cloned().collect::<BTreeSet<_>>();
    if actual == expected {
        return Vec::new();
    }
    vec![doc_warning(
        code,
        format!(
            "docstring {label} `{}` contradict map {label} `{}` for `{node_id}`",
            actual.into_iter().collect::<Vec<_>>().join(", "),
            expected.into_iter().collect::<Vec<_>>().join(", ")
        ),
        node_id,
        docstring,
        grouped.get(key).and_then(|facts| facts.first()),
    )]
}

fn grouped_facts(facts: &[DocstringFact]) -> BTreeMap<String, Vec<DocstringFact>> {
    let mut grouped = BTreeMap::<String, Vec<DocstringFact>>::new();
    for fact in facts {
        grouped
            .entry(fact.key.clone())
            .or_default()
            .push(fact.clone());
    }
    grouped
}

fn split_list(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim_ascii)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn doc_warning(
    code: &str,
    message: impl Into<String>,
    node: &str,
    docstring: &DocstringFacts,
    fact: Option<&DocstringFact>,
) -> Finding {
    let message = message.into();
    let location = fact.map_or_else(String::new, |fact| {
        format!(" at {}:{}", fact.line, fact.column)
    });
    warning(
        code,
        format!("{message} in `{}`{location}", docstring.path),
        Some(node.to_owned()),
        Some(docstring.path.clone()),
    )
}

fn warning(
    code: &str,
    message: impl Into<String>,
    node: Option<String>,
    path: Option<String>,
) -> Finding {
    Finding {
        code: code.to_owned(),
        severity: FindingSeverity::Warning,
        message: message.into(),
        node,
        path,
    }
}
