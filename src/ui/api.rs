//! UI API endpoint handlers.

// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::*;
use serialise::*;
use server::{Response, json};

pub(super) fn graph_json(graph: &GraphResponse) -> String {
    let nodes = graph
        .nodes
        .iter()
        .map(node_json)
        .collect::<Vec<_>>()
        .join(",");
    let edges = graph
        .edges
        .iter()
        .map(|edge| {
            format!(
                "{{\"from\":\"{}\",\"to\":\"{}\",\"kind\":\"{}\",\"description\":\"{}\"}}",
                esc(&edge.from),
                esc(&edge.to),
                graph_edge_kind_name(edge.kind),
                esc(&edge.description)
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("{{\"nodes\":[{nodes}],\"edges\":[{edges}]}}")
}

pub(super) fn node_json(node: &NodeRecord) -> String {
    format!(
        "{{\"id\":\"{}\",\"kind\":\"{}\",\"name\":\"{}\",\"description\":\"{}\",\"tags\":{},\"parent\":{},\"children\":{},\"paths\":{},\"contracts\":{},\"state\":\"{}\",\"files\":{}}}",
        esc(&node.id),
        kind_name(node.kind),
        esc(&node.name),
        esc(&node.description),
        string_array_json(&node.tags),
        optional_json(node.parent.as_deref()),
        string_array_json(&node.children),
        string_array_json(&node.paths),
        string_array_json(&node.contracts),
        state_name(node.state),
        string_array_json(&node.files)
    )
}

pub(super) fn dependency_json(graph: &Graph, node: &str, outbound: bool) -> Response {
    let decoded = percent_decode(node);
    let result = if outbound {
        query::depends(graph, &decoded, false)
    } else {
        query::dependents(graph, &decoded, false)
    };
    result.map_or_else(
        |finding| json(404, &finding_json(&finding)),
        |response| {
            let entries = response
                .nodes
                .iter()
                .map(|id| match graph.nodes.get(id) {
                    Some(record) => format!(
                        "{{\"id\":\"{}\",\"name\":\"{}\",\"slug\":\"{}\",\"state\":\"{}\",\"kind\":\"{}\"}}",
                        esc(&record.id),
                        esc(&record.name),
                        esc(&record.id),
                        state_name(record.state),
                        kind_name(record.kind),
                    ),
                    None => format!(
                        "{{\"id\":\"{}\",\"name\":\"{}\",\"slug\":\"{}\",\"state\":\"synced\",\"kind\":\"module\"}}",
                        esc(id),
                        esc(id),
                        esc(id),
                    ),
                })
                .collect::<Vec<_>>()
                .join(",");
            json(
                200,
                &format!(
                    "{{\"node\":\"{}\",\"nodes\":[{entries}]}}",
                    esc(&response.node),
                ),
            )
        },
    )
}

pub(super) fn status_json(project: &scanner::ScanResult) -> String {
    let findings = query::lint(&project.graph).findings;
    let errors = findings
        .iter()
        .filter(|finding| finding.severity == FindingSeverity::Error)
        .count();
    let warnings = findings
        .iter()
        .filter(|finding| finding.severity == FindingSeverity::Warning)
        .count();
    let infos = findings
        .iter()
        .filter(|finding| finding.severity == FindingSeverity::Info)
        .count();
    format!(
        "{{\"nodes\":{},\"edges\":{},\"findings\":{},\"errors\":{errors},\"warnings\":{warnings},\"infos\":{infos},\"interface_hash\":\"{}\"}}",
        project.graph.nodes.len(),
        project.graph.outbound.values().map(Vec::len).sum::<usize>(),
        findings.len(),
        esc(&project.interface_hash)
    )
}

pub(super) fn finding_json(finding: &Finding) -> String {
    format!(
        "{{\"code\":\"{}\",\"severity\":\"{}\",\"message\":\"{}\",\"node\":{},\"path\":{}}}",
        esc(&finding.code),
        severity_name(finding.severity),
        esc(&finding.message),
        optional_json(finding.node.as_deref()),
        optional_json(finding.path.as_deref())
    )
}

pub(super) fn project_finding(message: String) -> Finding {
    Finding {
        code: "CAIRN_UI_PROJECT_LOAD_FAILED".to_owned(),
        severity: FindingSeverity::Error,
        message,
        node: None,
        target: None,
        path: None,
    }
}
