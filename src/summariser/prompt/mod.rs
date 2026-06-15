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
mod tests;
