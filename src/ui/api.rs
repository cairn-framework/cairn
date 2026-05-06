//! JSON builders for UI endpoints (graph, nodes, contracts, findings, status).
// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::*;
use serialise::*;
use server::{Response, json};

pub(super) fn meta_json() -> String {
    let commands = cli::registry()
        .iter()
        .map(|command| {
            format!(
                "{{\"name\":\"{}\",\"request\":\"{}\",\"response\":\"{}\",\"safety\":\"{:?}\"}}",
                esc(command.cli_name),
                esc(command.request_schema),
                esc(command.response_schema),
                command.safety
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("{{\"schema_version\":{SCHEMA_VERSION},\"available_commands\":[{commands}]}}")
}

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

pub(super) fn lint_json(graph: &Graph) -> String {
    let findings = query::lint(graph)
        .findings
        .iter()
        .map(finding_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("{{\"findings\":[{findings}]}}")
}

pub(super) fn status_json(project: &scanner::ScanResult) -> String {
    let findings = query::lint(&project.graph).findings;
    let errors = findings
        .iter()
        .filter(|finding| finding.severity == FindingSeverity::Error)
        .count();
    let warnings = findings.len().saturating_sub(errors);
    format!(
        "{{\"schema_version\":{SCHEMA_VERSION},\"nodes\":{},\"edges\":{},\"findings\":{},\"errors\":{errors},\"warnings\":{warnings},\"interface_hash\":\"{}\"}}",
        project.graph.nodes.len(),
        project.graph.outbound.values().map(Vec::len).sum::<usize>(),
        findings.len(),
        esc(&project.interface_hash)
    )
}

pub(super) fn contract_response_json(project: &scanner::ScanResult, node: &str) -> String {
    let artefacts = project
        .contracts
        .contracts
        .values()
        .filter(|contract| contract.node == node || contract.declared_by == node)
        .map(contract_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("{{\"node\":\"{}\",\"artefacts\":[{artefacts}]}}", esc(node))
}

pub(super) fn contract_json(contract: &Contract) -> String {
    format!(
        "{{\"type\":\"contract\",\"path\":\"{}\",\"title\":\"{}\",\"frontmatter\":{{\"node\":\"{}\"}},\"body\":\"{}\"}}",
        esc(&contract.path),
        esc(&title_from_body(&contract.body, "Contract")),
        esc(&contract.node),
        esc(&contract.body)
    )
}

pub(super) fn artefact_response_json(root: &Path, kind: &str, node: &str) -> String {
    let artefacts = collect_artefacts(root, kind, node)
        .iter()
        .map(|artefact| artefact_json(kind, artefact))
        .collect::<Vec<_>>()
        .join(",");
    format!("{{\"node\":\"{}\",\"artefacts\":[{artefacts}]}}", esc(node))
}

pub(super) fn rationale_json(root: &Path, node: &str) -> String {
    let artefacts = ["decisions", "research", "sources"]
        .iter()
        .flat_map(|kind| {
            collect_artefacts(root, kind, node)
                .into_iter()
                .map(|artefact| ((*kind).to_owned(), artefact))
        })
        .map(|(kind, artefact)| artefact_json(&kind, &artefact))
        .collect::<Vec<_>>()
        .join(",");
    format!("{{\"node\":\"{}\",\"artefacts\":[{artefacts}]}}", esc(node))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Artefact {
    path: String,
    title: String,
    frontmatter: BTreeMap<String, String>,
    body: String,
}

pub(super) fn collect_artefacts(root: &Path, kind: &str, node: &str) -> Vec<Artefact> {
    let mut artefacts = Vec::new();
    let directory = root.join("meta").join(kind);
    collect_artefacts_from_dir(root, &directory, node, &mut artefacts);
    artefacts.sort_by(|left, right| left.path.cmp(&right.path));
    artefacts
}

pub(super) fn collect_artefacts_from_dir(
    root: &Path,
    directory: &Path,
    node: &str,
    artefacts: &mut Vec<Artefact>,
) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_artefacts_from_dir(root, &path, node, artefacts);
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }
        let Ok(source) = fs::read_to_string(&path) else {
            continue;
        };
        let parsed = frontmatter::parse(&source);
        if !frontmatter_mentions_node(&parsed.values, node) {
            continue;
        }
        let relative = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();
        artefacts.push(Artefact {
            path: relative,
            title: title_from_body(&parsed.body, "Artefact"),
            frontmatter: parsed.values,
            body: parsed.body,
        });
    }
}

pub(super) fn frontmatter_mentions_node(values: &BTreeMap<String, String>, node: &str) -> bool {
    ["node", "nodes"]
        .iter()
        .filter_map(|key| values.get(*key))
        .any(|value| value.contains(node))
}

pub(super) fn artefact_json(kind: &str, artefact: &Artefact) -> String {
    format!(
        "{{\"type\":\"{}\",\"path\":\"{}\",\"title\":\"{}\",\"frontmatter\":{},\"body\":\"{}\"}}",
        esc(kind),
        esc(&artefact.path),
        esc(&artefact.title),
        map_json(&artefact.frontmatter),
        esc(&artefact.body)
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
        path: None,
    }
}

pub(super) fn title_from_body(body: &str, fallback: &str) -> String {
    body.lines()
        .find_map(|line| line.trim().strip_prefix("# "))
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .unwrap_or(fallback)
        .to_owned()
}
