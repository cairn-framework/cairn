//! Prompt input builder for the summariser.
//!
//! Constructs a `SummariserRequest` from live project state: map facts,
//! contract excerpts, interface findings, project context, rules, and
//! bounded code samples.  Applies `max_sample_bytes_per_file` and
//! `max_prompt_bytes` limits via truncation and sample dropping.

use std::path::Path;

use crate::{
    artefacts::contract::ContractSet,
    map::graph::{Graph, NodeRecord},
    scanner::config::Config,
    summariser::request::{CodeSample, SUMMARISER_SCHEMA_VERSION, SummariserRequest},
};

/// Error during prompt input construction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PromptError {
    /// Target node was not found in the graph.
    NodeNotFound(String),
}

impl std::fmt::Display for PromptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NodeNotFound(id) => write!(f, "node `{id}` not found in graph"),
        }
    }
}

impl std::error::Error for PromptError {}

/// Builds a `SummariserRequest` grounded in the current project state.
///
/// # Errors
///
/// Returns `PromptError::NodeNotFound` when `node_id` is absent from
/// `graph`.
///
/// # Panics
///
/// Never panics.
// Reason: prompt construction needs graph, config, contracts, and metadata
// that are all required to build the context; splitting would not improve clarity.
#[allow(clippy::too_many_arguments)]
pub fn build_request(
    node_id: &str,
    draft_type: &str,
    request_id: &str,
    graph: &Graph,
    config: &Config,
    root: &Path,
    contracts: &ContractSet,
    max_prompt_bytes: usize,
    max_sample_bytes_per_file: usize,
) -> Result<SummariserRequest, PromptError> {
    let node = graph
        .nodes
        .get(node_id)
        .ok_or_else(|| PromptError::NodeNotFound(node_id.to_owned()))?;

    let mut request = SummariserRequest {
        schema_version: SUMMARISER_SCHEMA_VERSION,
        request_id: request_id.to_owned(),
        draft_type: draft_type.to_owned(),
        target_node: node_id.to_owned(),
        map_facts: build_map_facts(node, graph),
        contract_excerpt: build_contract_excerpt(node, root, contracts),
        interface_findings: build_interface_findings(node_id, graph),
        docstring_findings: Vec::new(),
        project_context: config.context.clone(),
        rules: config.rules.values().cloned().collect(),
        code_samples: build_code_samples(node, root, max_sample_bytes_per_file),
    };

    enforce_max_prompt_bytes(&mut request, max_prompt_bytes);

    Ok(request)
}

fn build_map_facts(node: &NodeRecord, graph: &Graph) -> Vec<String> {
    let mut facts = Vec::new();
    facts.push(format!("kind: {:?}", node.kind));
    facts.push(format!("name: {}", node.name));
    if !node.description.is_empty() {
        facts.push(format!("description: {}", node.description));
    }
    if !node.tags.is_empty() {
        facts.push(format!("tags: {}", node.tags.join(", ")));
    }
    if !node.paths.is_empty() {
        facts.push(format!("paths: {}", node.paths.join(", ")));
    }
    if let Some(ref parent) = node.parent {
        facts.push(format!("parent: {parent}"));
    }
    if !node.children.is_empty() {
        facts.push(format!("children: {}", node.children.join(", ")));
    }
    if let Some(edges) = graph.outbound.get(&node.id) {
        for edge in edges {
            facts.push(format!("outbound: {} ({})", edge.to, edge.description));
        }
    }
    if let Some(edges) = graph.inbound.get(&node.id) {
        for edge in edges {
            facts.push(format!("inbound: {} ({})", edge.from, edge.description));
        }
    }
    facts
}

fn build_contract_excerpt(
    node: &NodeRecord,
    root: &Path,
    contracts: &ContractSet,
) -> Option<String> {
    node.contracts.iter().find_map(|path| {
        contracts
            .contracts
            .get(path)
            .map(|c| c.body.clone())
            .or_else(|| {
                std::fs::read_to_string(root.join(path))
                    .ok()
                    .map(|source| crate::artefacts::frontmatter::parse(&source).body)
            })
    })
}

fn build_interface_findings(node_id: &str, graph: &Graph) -> Vec<String> {
    graph
        .findings
        .iter()
        .filter(|f| f.node.as_deref() == Some(node_id))
        .map(|f| format!("{}: {}", f.code, f.message))
        .collect()
}

fn build_code_samples(
    node: &NodeRecord,
    root: &Path,
    max_sample_bytes_per_file: usize,
) -> Vec<CodeSample> {
    let mut samples = Vec::new();
    for path in &node.files {
        let full = root.join(path);
        let Ok(content) = std::fs::read(&full) else {
            continue;
        };
        let Ok(text) = String::from_utf8(content) else {
            continue;
        };
        let truncated = if text.len() > max_sample_bytes_per_file {
            let mut end = max_sample_bytes_per_file;
            while !text.is_char_boundary(end) && end > 0 {
                end -= 1;
            }
            text[..end].to_owned()
        } else {
            text
        };
        samples.push(CodeSample {
            path: path.clone(),
            content: truncated,
        });
    }
    samples
}

fn enforce_max_prompt_bytes(request: &mut SummariserRequest, max_prompt_bytes: usize) {
    let over_limit = |req: &SummariserRequest| {
        serde_json::to_string(req).map_or(true, |s| s.len() > max_prompt_bytes)
    };

    // Drop samples one-by-one (largest first) until the JSON fits.
    while over_limit(request) {
        if request.code_samples.is_empty() {
            break;
        }
        let largest_idx = request
            .code_samples
            .iter()
            .enumerate()
            .max_by_key(|(_, s)| s.content.len())
            .map_or(0, |(i, _)| i);
        request.code_samples.swap_remove(largest_idx);
    }

    // If still over limit, truncate contract_excerpt.
    let mut excerpt = request.contract_excerpt.clone().unwrap_or_default();
    while over_limit(request) {
        if excerpt.is_empty() {
            request.contract_excerpt = None;
            break;
        }
        let new_len = excerpt.len().saturating_sub(100);
        let mut end = new_len;
        while !excerpt.is_char_boundary(end) && end > 0 {
            end -= 1;
        }
        excerpt.truncate(end);
        request.contract_excerpt = Some(excerpt.clone());
    }

    // If still over limit, truncate project_context.
    let mut context = request.project_context.clone();
    while over_limit(request) {
        if context.is_empty() {
            request.project_context.clear();
            break;
        }
        let new_len = context.len().saturating_sub(100);
        let mut end = new_len;
        while !context.is_char_boundary(end) && end > 0 {
            end -= 1;
        }
        context.truncate(end);
        request.project_context.clone_from(&context);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blueprint::NodeKind;
    use crate::map::graph::{EdgeRef, Finding, FindingSeverity, Graph, NodeRecord, NodeState};
    use std::collections::BTreeMap;

    fn empty_graph() -> Graph {
        Graph {
            nodes: BTreeMap::new(),
            names: BTreeMap::new(),
            outbound: BTreeMap::new(),
            inbound: BTreeMap::new(),
            findings: Vec::new(),
        }
    }

    fn sample_node(id: &str) -> NodeRecord {
        NodeRecord {
            kind: NodeKind::Module,
            id: id.to_owned(),
            name: "Auth".to_owned(),
            description: "Authentication service".to_owned(),
            tags: vec!["core".to_owned()],
            parent: Some("app".to_owned()),
            children: vec!["app.auth.login".to_owned()],
            paths: vec!["src/auth".to_owned()],
            owns_files: true,
            contracts: vec!["contracts/auth.md".to_owned()],
            state: NodeState::Synced,
            files: vec![],
            span: crate::blueprint::Span::point("test", 1, 1),
        }
    }

    #[test]
    fn test_build_request_node_not_found_errors() {
        let graph = empty_graph();
        let config = Config::default();
        let result = build_request(
            "missing",
            "contract",
            "req-1",
            &graph,
            &config,
            Path::new("."),
            &ContractSet::default(),
            10_000,
            1_000,
        );
        assert!(
            matches!(result, Err(PromptError::NodeNotFound(ref id)) if id == "missing"),
            "expected NodeNotFound, got {result:?}"
        );
    }

    #[test]
    fn test_build_request_populates_map_facts() {
        let mut graph = empty_graph();
        graph
            .nodes
            .insert("app.auth".to_owned(), sample_node("app.auth"));
        graph.outbound.insert(
            "app.auth".to_owned(),
            vec![EdgeRef {
                from: "app.auth".to_owned(),
                to: "app.db".to_owned(),
                description: "persists sessions".to_owned(),
            }],
        );
        graph.inbound.insert(
            "app.auth".to_owned(),
            vec![EdgeRef {
                from: "app.api".to_owned(),
                to: "app.auth".to_owned(),
                description: "delegates login".to_owned(),
            }],
        );
        let config = Config::default();
        let req = build_request(
            "app.auth",
            "contract",
            "req-1",
            &graph,
            &config,
            Path::new("."),
            &ContractSet::default(),
            10_000,
            1_000,
        )
        .unwrap();

        assert!(req.map_facts.iter().any(|f| f.contains("kind: Module")));
        assert!(req.map_facts.iter().any(|f| f.contains("name: Auth")));
        assert!(
            req.map_facts
                .iter()
                .any(|f| f.contains("description: Authentication service"))
        );
        assert!(req.map_facts.iter().any(|f| f.contains("tags: core")));
        assert!(req.map_facts.iter().any(|f| f.contains("paths: src/auth")));
        assert!(req.map_facts.iter().any(|f| f.contains("parent: app")));
        assert!(
            req.map_facts
                .iter()
                .any(|f| f.contains("children: app.auth.login"))
        );
        assert!(
            req.map_facts
                .iter()
                .any(|f| f.contains("outbound: app.db (persists sessions)"))
        );
        assert!(
            req.map_facts
                .iter()
                .any(|f| f.contains("inbound: app.api (delegates login)"))
        );
    }

    #[test]
    fn test_build_request_uses_contract_body_from_set() {
        let mut graph = empty_graph();
        let mut node = sample_node("app.auth");
        node.contracts = vec!["contracts/auth.md".to_owned()];
        graph.nodes.insert("app.auth".to_owned(), node);

        let mut contracts = ContractSet::default();
        contracts.contracts.insert(
            "contracts/auth.md".to_owned(),
            crate::artefacts::contract::Contract {
                path: "contracts/auth.md".to_owned(),
                declared_by: "app.auth".to_owned(),
                node: "app.auth".to_owned(),
                body: "# Auth\n\nHandles login.".to_owned(),
            },
        );

        let config = Config::default();
        let req = build_request(
            "app.auth",
            "contract",
            "req-1",
            &graph,
            &config,
            Path::new("."),
            &contracts,
            10_000,
            1_000,
        )
        .unwrap();

        assert_eq!(
            req.contract_excerpt,
            Some("# Auth\n\nHandles login.".to_owned())
        );
    }

    #[test]
    fn test_build_request_reads_contract_fallback_when_not_in_set() {
        let dir = tempfile::tempdir().unwrap();
        let contract_path = dir.path().join("contracts/auth.md");
        std::fs::create_dir_all(contract_path.parent().unwrap()).unwrap();
        std::fs::write(
            &contract_path,
            "---\nnode: app.auth\n---\n# Auth\n\nFallback.",
        )
        .unwrap();

        let mut graph = empty_graph();
        let mut node = sample_node("app.auth");
        node.contracts = vec!["contracts/auth.md".to_owned()];
        graph.nodes.insert("app.auth".to_owned(), node);

        let config = Config::default();
        let req = build_request(
            "app.auth",
            "contract",
            "req-1",
            &graph,
            &config,
            dir.path(),
            &ContractSet::default(),
            10_000,
            1_000,
        )
        .unwrap();

        assert_eq!(req.contract_excerpt, Some("# Auth\n\nFallback.".to_owned()));
    }

    #[test]
    fn test_build_request_no_contract_returns_none() {
        let mut graph = empty_graph();
        let mut node = sample_node("app.auth");
        node.contracts = vec![];
        graph.nodes.insert("app.auth".to_owned(), node);

        let config = Config::default();
        let req = build_request(
            "app.auth",
            "contract",
            "req-1",
            &graph,
            &config,
            Path::new("."),
            &ContractSet::default(),
            10_000,
            1_000,
        )
        .unwrap();

        assert!(req.contract_excerpt.is_none());
    }

    #[test]
    fn test_build_request_filters_interface_findings() {
        let mut graph = empty_graph();
        graph
            .nodes
            .insert("app.auth".to_owned(), sample_node("app.auth"));
        graph.findings.push(Finding {
            code: "CT001".to_owned(),
            severity: FindingSeverity::Error,
            message: "Interface contradiction".to_owned(),
            node: Some("app.auth".to_owned()),
            target: None,
            path: None,
        });
        graph.findings.push(Finding {
            code: "CT002".to_owned(),
            severity: FindingSeverity::Warning,
            message: "Other node issue".to_owned(),
            node: Some("app.db".to_owned()),
            target: None,
            path: None,
        });

        let config = Config::default();
        let req = build_request(
            "app.auth",
            "contract",
            "req-1",
            &graph,
            &config,
            Path::new("."),
            &ContractSet::default(),
            10_000,
            1_000,
        )
        .unwrap();

        assert_eq!(req.interface_findings.len(), 1);
        assert!(req.interface_findings[0].contains("CT001"));
        assert!(req.interface_findings[0].contains("Interface contradiction"));
    }

    #[test]
    fn test_build_request_populates_context_and_rules() {
        let mut graph = empty_graph();
        graph
            .nodes
            .insert("app.auth".to_owned(), sample_node("app.auth"));

        let mut config = Config {
            context: "Auth context".to_owned(),
            ..Config::default()
        };
        config
            .rules
            .insert("style".to_owned(), "use present tense".to_owned());
        config
            .rules
            .insert("tone".to_owned(), "be concise".to_owned());
        let req = build_request(
            "app.auth",
            "contract",
            "req-1",
            &graph,
            &config,
            Path::new("."),
            &ContractSet::default(),
            10_000,
            1_000,
        )
        .unwrap();

        assert_eq!(req.project_context, "Auth context");
        assert_eq!(req.rules.len(), 2);
        assert!(req.rules.contains(&"use present tense".to_owned()));
        assert!(req.rules.contains(&"be concise".to_owned()));
    }

    #[test]
    fn test_build_request_includes_code_samples() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("src")).unwrap();
        std::fs::write(dir.path().join("src/auth.rs"), "fn login() {}").unwrap();

        let mut graph = empty_graph();
        let mut node = sample_node("app.auth");
        node.files = vec!["src/auth.rs".to_owned()];
        graph.nodes.insert("app.auth".to_owned(), node);

        let config = Config::default();
        let req = build_request(
            "app.auth",
            "contract",
            "req-1",
            &graph,
            &config,
            dir.path(),
            &ContractSet::default(),
            10_000,
            1_000,
        )
        .unwrap();

        assert_eq!(req.code_samples.len(), 1);
        assert_eq!(req.code_samples[0].path, "src/auth.rs");
        assert_eq!(req.code_samples[0].content, "fn login() {}");
    }

    #[test]
    fn test_build_request_skips_missing_code_files() {
        let mut graph = empty_graph();
        let mut node = sample_node("app.auth");
        node.files = vec!["src/missing.rs".to_owned()];
        graph.nodes.insert("app.auth".to_owned(), node);

        let config = Config::default();
        let req = build_request(
            "app.auth",
            "contract",
            "req-1",
            &graph,
            &config,
            Path::new("."),
            &ContractSet::default(),
            10_000,
            1_000,
        )
        .unwrap();

        assert!(req.code_samples.is_empty());
    }

    #[test]
    fn test_build_request_truncates_code_samples_per_file_limit() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("src")).unwrap();
        let long_content = "a".repeat(500);
        std::fs::write(dir.path().join("src/auth.rs"), &long_content).unwrap();

        let mut graph = empty_graph();
        let mut node = sample_node("app.auth");
        node.files = vec!["src/auth.rs".to_owned()];
        graph.nodes.insert("app.auth".to_owned(), node);

        let config = Config::default();
        let req = build_request(
            "app.auth",
            "contract",
            "req-1",
            &graph,
            &config,
            dir.path(),
            &ContractSet::default(),
            10_000,
            100,
        )
        .unwrap();

        assert_eq!(req.code_samples.len(), 1);
        assert_eq!(req.code_samples[0].content.len(), 100);
    }

    #[test]
    fn test_build_request_drops_samples_when_prompt_too_large() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("src")).unwrap();
        std::fs::write(dir.path().join("src/a.rs"), "x").unwrap();
        std::fs::write(dir.path().join("src/b.rs"), "y").unwrap();

        let mut graph = empty_graph();
        let mut node = sample_node("app.auth");
        node.files = vec!["src/a.rs".to_owned(), "src/b.rs".to_owned()];
        graph.nodes.insert("app.auth".to_owned(), node);

        let config = Config::default();
        let req = build_request(
            "app.auth",
            "contract",
            "req-1",
            &graph,
            &config,
            dir.path(),
            &ContractSet::default(),
            200,
            1_000,
        )
        .unwrap();

        // With only 200 bytes total, both samples should have been dropped.
        assert!(req.code_samples.is_empty());
    }

    #[test]
    fn test_build_request_truncates_contract_when_prompt_too_large() {
        let mut graph = empty_graph();
        let mut node = sample_node("app.auth");
        node.contracts = vec!["contracts/auth.md".to_owned()];
        graph.nodes.insert("app.auth".to_owned(), node);

        let mut contracts = ContractSet::default();
        contracts.contracts.insert(
            "contracts/auth.md".to_owned(),
            crate::artefacts::contract::Contract {
                path: "contracts/auth.md".to_owned(),
                declared_by: "app.auth".to_owned(),
                node: "app.auth".to_owned(),
                body: "a".repeat(1_000),
            },
        );

        let config = Config::default();
        let req = build_request(
            "app.auth",
            "contract",
            "req-1",
            &graph,
            &config,
            Path::new("."),
            &contracts,
            300,
            1_000,
        )
        .unwrap();

        // The contract excerpt should have been truncated to fit.
        assert!(
            req.contract_excerpt.as_ref().unwrap().len() < 1_000,
            "expected truncated excerpt, got {} chars",
            req.contract_excerpt.as_ref().unwrap().len()
        );
    }

    #[test]
    fn test_build_request_omits_empty_description_from_map_facts() {
        let mut graph = empty_graph();
        let mut node = sample_node("app.auth");
        node.description = String::new();
        graph.nodes.insert("app.auth".to_owned(), node);

        let config = Config::default();
        let req = build_request(
            "app.auth",
            "contract",
            "req-1",
            &graph,
            &config,
            Path::new("."),
            &ContractSet::default(),
            10_000,
            1_000,
        )
        .unwrap();

        assert!(!req.map_facts.iter().any(|f| f.starts_with("description:")));
    }

    #[test]
    fn test_build_request_omits_empty_tags_from_map_facts() {
        let mut graph = empty_graph();
        let mut node = sample_node("app.auth");
        node.tags = vec![];
        graph.nodes.insert("app.auth".to_owned(), node);

        let config = Config::default();
        let req = build_request(
            "app.auth",
            "contract",
            "req-1",
            &graph,
            &config,
            Path::new("."),
            &ContractSet::default(),
            10_000,
            1_000,
        )
        .unwrap();

        assert!(!req.map_facts.iter().any(|f| f.starts_with("tags:")));
    }

    #[test]
    fn test_build_request_request_id_and_target_preserved() {
        let mut graph = empty_graph();
        graph
            .nodes
            .insert("app.auth".to_owned(), sample_node("app.auth"));

        let config = Config::default();
        let req = build_request(
            "app.auth",
            "contract",
            "custom-req-42",
            &graph,
            &config,
            Path::new("."),
            &ContractSet::default(),
            10_000,
            1_000,
        )
        .unwrap();

        assert_eq!(req.request_id, "custom-req-42");
        assert_eq!(req.target_node, "app.auth");
        assert_eq!(req.draft_type, "contract");
    }
}
