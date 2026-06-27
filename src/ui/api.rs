//! UI API endpoint handlers.

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
    format!("{{\"available_commands\":[{commands}]}}")
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

pub(super) fn beads_response_json(root: &Path, node: &str) -> String {
    let items = backlog::read(root);
    let beads = backlog::for_node(&items, node)
        .iter()
        .map(|item| item.to_json().to_string())
        .collect::<Vec<_>>()
        .join(",");
    format!("{{\"node\":\"{}\",\"beads\":[{beads}]}}", esc(node))
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
        if !frontmatter_mentions_node(&parsed.values, &parsed.lists, node) {
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

pub(super) fn frontmatter_mentions_node(
    values: &BTreeMap<String, String>,
    lists: &BTreeMap<String, Vec<String>>,
    node: &str,
) -> bool {
    for key in ["node", "nodes"] {
        if values.get(key).is_some_and(|v| v == node) {
            return true;
        }
        if lists
            .get(key)
            .is_some_and(|items| items.iter().any(|id| id == node))
        {
            return true;
        }
    }
    false
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
        target: None,
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
#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::frontmatter_mentions_node;
    use crate::artefacts::frontmatter;

    // ── frontmatter_mentions_node ────────────────────────────────────────────

    #[test]
    fn test_fmn_exact_scalar_node_key_matches() {
        let fm = frontmatter::parse("---\nnode: app.api\n---\n");
        assert!(frontmatter_mentions_node(&fm.values, &fm.lists, "app.api"));
    }

    #[test]
    fn test_fmn_prefix_node_key_does_not_match_child() {
        // "app" must NOT match a file with `node: app.api`.
        // This was the false-positive bug: "app.api".contains("app") == true.
        let fm = frontmatter::parse("---\nnode: app.api\n---\n");
        assert!(!frontmatter_mentions_node(&fm.values, &fm.lists, "app"));
    }

    #[test]
    fn test_fmn_prefix_nodes_inline_list_does_not_false_positive() {
        // "app" must NOT match `nodes: [app.api, app.db]`.
        let fm = frontmatter::parse("---\nnodes: [app.api, app.db]\n---\n");
        assert!(!frontmatter_mentions_node(&fm.values, &fm.lists, "app"));
    }

    #[test]
    fn test_fmn_inline_list_exact_match() {
        // "app.api" MUST match `nodes: [app.api, app.db]`.
        let fm = frontmatter::parse("---\nnodes: [app.api, app.db]\n---\n");
        assert!(frontmatter_mentions_node(&fm.values, &fm.lists, "app.api"));
    }

    #[test]
    fn test_fmn_block_list_is_matched() {
        // Block-form YAML list is stored only in `lists`, not `values`.
        // The old code only checked `values` and silently missed these.
        let fm = frontmatter::parse("---\nnodes:\n  - app.api\n  - app.db\n---\n");
        assert!(frontmatter_mentions_node(&fm.values, &fm.lists, "app.api"));
    }

    #[test]
    fn test_fmn_block_list_no_false_positive_for_prefix() {
        let fm = frontmatter::parse("---\nnodes:\n  - app.api\n  - app.db\n---\n");
        assert!(!frontmatter_mentions_node(&fm.values, &fm.lists, "app"));
    }

    #[test]
    fn test_fmn_absent_node_key_returns_false() {
        let mut values = BTreeMap::new();
        values.insert("title".to_owned(), "something".to_owned());
        assert!(!frontmatter_mentions_node(
            &values,
            &BTreeMap::new(),
            "app.api"
        ));
    }

    // ── title_from_body ──────────────────────────────────────────────────────

    #[test]
    fn test_title_from_body_extracts_h1() {
        use super::title_from_body;
        assert_eq!(title_from_body("# My Title\nbody", "fallback"), "My Title");
    }

    #[test]
    fn test_title_from_body_uses_fallback_when_no_h1() {
        use super::title_from_body;
        assert_eq!(title_from_body("just body text", "fallback"), "fallback");
    }

    #[test]
    fn test_title_from_body_empty_h1_uses_fallback() {
        use super::title_from_body;
        assert_eq!(title_from_body("# \nbody", "fallback"), "fallback");
    }

    // ── beads_response_json ──────────────────────────────────────────────────

    #[test]
    fn test_beads_response_json_filters_by_node() {
        use super::beads_response_json;
        let dir = std::env::temp_dir().join(format!("cairn-ui-beads-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(".beads")).unwrap();
        std::fs::write(
            dir.join(".beads").join("issues.jsonl"),
            [
                r#"{"id":"cairn-ui1","title":"View","status":"open","priority":1,"issue_type":"task","labels":["cairn-node:cairn.ui"]}"#,
                r#"{"id":"cairn-elsewhere","title":"Other","status":"open","priority":0,"issue_type":"task","labels":["cairn-node:cairn.kernel.cli"]}"#,
            ]
            .join("\n"),
        )
        .unwrap();
        let json = beads_response_json(&dir, "cairn.ui");
        assert!(json.contains("\"node\":\"cairn.ui\""));
        assert!(json.contains("\"id\":\"cairn-ui1\""));
        assert!(json.contains("\"status\":\"open\""));
        assert!(json.contains("\"priority\":1"));
        assert!(!json.contains("cairn-elsewhere"), "other nodes excluded");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
