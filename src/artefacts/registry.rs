//! Typed artefact registry and Phase 2 loaders.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
    fs, io,
    path::{Path, PathBuf},
};

use crate::{
    blueprint::{Ast, Field, Node},
    map::graph::{Finding, FindingSeverity},
};

use super::{contract::ContractSet, frontmatter};

/// Supported v1 artefact types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtefactType {
    /// Contract artefact.
    Contract,
    /// Todo artefact.
    Todo,
    /// Decision artefact.
    Decision,
    /// Review artefact.
    Review,
    /// Research artefact.
    Research,
    /// Source artefact.
    Source,
}

/// Generic artefact loader request.
#[derive(Clone, Copy, Debug)]
pub struct ArtefactLoadRequest<'a> {
    /// Project root.
    pub root: &'a Path,
    /// Parsed blueprint.
    pub ast: &'a Ast,
}

/// Generic loaded artefact record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtefactRecord {
    /// Artefact type.
    pub artefact_type: ArtefactType,
    /// Stable artefact ID, or path for path-keyed records.
    pub id: String,
    /// Declared path.
    pub path: String,
}

/// Artefact loader error.
pub type ArtefactError = String;

/// Common interface for typed artefact loaders.
pub trait ArtefactLoader {
    /// Artefact type handled by the loader.
    fn artefact_type(&self) -> ArtefactType;
    /// Loads records for the request.
    ///
    /// # Errors
    ///
    /// Returns a loader-level error when the filesystem cannot be traversed.
    fn load(&self, request: ArtefactLoadRequest<'_>) -> Result<Vec<ArtefactRecord>, ArtefactError>;
}

/// Todo status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TodoStatus {
    /// Open todo.
    Open,
    /// In progress todo.
    InProgress,
    /// Completed todo.
    Done,
    /// Blocked todo.
    Blocked,
}

/// Parsed todo.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Todo {
    /// Source path.
    pub path: String,
    /// Referenced node.
    pub node: String,
    /// Status.
    pub status: TodoStatus,
    /// Creation date.
    pub created: String,
    /// Optional satisfied contract clause.
    pub satisfies: Option<String>,
    /// Markdown body.
    pub body: String,
}

/// Decision status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecisionStatus {
    /// Proposed decision.
    Proposed,
    /// Accepted decision.
    Accepted,
    /// Deprecated decision.
    Deprecated,
    /// Superseded decision.
    Superseded,
}

/// Parsed decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Decision {
    /// Stable decision ID.
    pub id: String,
    /// Source path.
    pub path: String,
    /// Referenced nodes.
    pub nodes: Vec<String>,
    /// Status.
    pub status: DecisionStatus,
    /// Decision date.
    pub date: String,
    /// Last revisited date.
    pub revisited: Option<String>,
    /// Revisit triggers.
    pub revisit_triggers: Vec<String>,
    /// Referenced research/source IDs.
    pub informed_by: Vec<String>,
    /// Superseded decision IDs.
    pub supersedes: Vec<String>,
    /// Refined decision IDs.
    pub refines: Vec<String>,
    /// Related decision IDs.
    pub related: Vec<String>,
    /// Whether all node references are intentionally orphaned.
    pub orphaned: bool,
    /// Orphan reason.
    pub orphan_reason: Option<String>,
    /// Markdown body.
    pub body: String,
}

/// Review subtype.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReviewType {
    /// Human-authored review.
    Human,
    /// Implementing agent self-review.
    AgentIntrospective,
    /// Cross-model agent review.
    AgentCrossModel,
}

/// Parsed review.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Review {
    /// Source path.
    pub path: String,
    /// Referenced node.
    pub node: String,
    /// Review subtype.
    pub review_type: ReviewType,
    /// Review date.
    pub date: String,
    /// Reviewer identifier.
    pub reviewer: String,
    /// Optional related change.
    pub related_change: Option<String>,
    /// Markdown body.
    pub body: String,
}

/// Parsed research.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Research {
    /// Stable research ID.
    pub id: String,
    /// Source path.
    pub path: String,
    /// Referenced nodes.
    pub nodes: Vec<String>,
    /// Research date.
    pub date: String,
    /// Referenced source IDs.
    pub sources: Vec<String>,
    /// Tags.
    pub tags: Vec<String>,
    /// Markdown body.
    pub body: String,
}

/// Source verification state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceVerification {
    /// Local file hash is verified.
    Verified,
    /// External URL reference.
    External,
    /// Unverified source.
    Unverified,
}

/// Parsed source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Source {
    /// Stable source ID.
    pub id: String,
    /// Source manifest path.
    pub path: String,
    /// Local file path or URL.
    pub file: String,
    /// Optional expected SHA-256.
    pub sha256: Option<String>,
    /// Verification state.
    pub verification: SourceVerification,
    /// Source type.
    pub source_type: String,
    /// Source date.
    pub date: String,
    /// Tags.
    pub tags: Vec<String>,
    /// Description.
    pub description: String,
    /// Markdown body.
    pub body: String,
}

/// Loaded Phase 2 artefacts.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ArtefactSet {
    /// Contract set.
    pub contracts: ContractSet,
    /// Todos.
    pub todos: Vec<Todo>,
    /// Decisions.
    pub decisions: Vec<Decision>,
    /// Reviews.
    pub reviews: Vec<Review>,
    /// Research records.
    pub research: Vec<Research>,
    /// Sources.
    pub sources: Vec<Source>,
    /// Loading and validation findings.
    pub findings: Vec<Finding>,
}

/// Loads all non-contract Phase 2 artefacts from retained blueprint pointers.
#[must_use]
pub fn load_artefacts(root: &Path, ast: &Ast, contracts: ContractSet) -> ArtefactSet {
    let ids = collect_ids(ast);
    let mut set = ArtefactSet {
        contracts,
        ..ArtefactSet::default()
    };
    load_todos(root, ast, &mut set);
    load_decisions(root, ast, &mut set);
    load_reviews(root, ast, &mut set);
    load_research(root, ast, &mut set);
    load_sources(root, ast, &mut set);
    validate_integrity(root, &ids, &mut set);
    set
}

fn load_todos(root: &Path, ast: &Ast, set: &mut ArtefactSet) {
    for pointer in pointers(ast, "todos") {
        for path in markdown_paths(root, &pointer, set) {
            if let Some(parsed) = parse_file(&path, &pointer, set) {
                let Some(node) = required(&parsed.values, "node", path_string(&path), set) else {
                    continue;
                };
                let Some(status) = required(&parsed.values, "status", path_string(&path), set)
                    .and_then(|value| parse_todo_status(&value, &path, set))
                else {
                    continue;
                };
                let Some(created) = required(&parsed.values, "created", path_string(&path), set)
                else {
                    continue;
                };
                set.todos.push(Todo {
                    path: path_string(&path),
                    node,
                    status,
                    created,
                    satisfies: optional(&parsed.values, "satisfies"),
                    body: parsed.body,
                });
            }
        }
    }
}

fn load_decisions(root: &Path, ast: &Ast, set: &mut ArtefactSet) {
    for pointer in pointers(ast, "decisions") {
        for path in markdown_paths(root, &pointer, set) {
            if let Some(parsed) = parse_file(&path, &pointer, set) {
                let Some(id) = required(&parsed.values, "id", path_string(&path), set) else {
                    continue;
                };
                let Some(status) = required(&parsed.values, "status", path_string(&path), set)
                    .and_then(|value| parse_decision_status(&value, &path, set))
                else {
                    continue;
                };
                let Some(date) = required(&parsed.values, "date", path_string(&path), set) else {
                    continue;
                };
                set.decisions.push(Decision {
                    id,
                    path: path_string(&path),
                    nodes: list(&parsed, "nodes"),
                    status,
                    date,
                    revisited: optional(&parsed.values, "revisited"),
                    revisit_triggers: list(&parsed, "revisit_triggers"),
                    informed_by: list(&parsed, "informed_by"),
                    supersedes: list(&parsed, "supersedes"),
                    refines: list(&parsed, "refines"),
                    related: list(&parsed, "related"),
                    orphaned: optional(&parsed.values, "orphaned")
                        .is_some_and(|value| value == "true"),
                    orphan_reason: optional(&parsed.values, "orphan_reason"),
                    body: parsed.body,
                });
            }
        }
    }
}

fn load_reviews(root: &Path, ast: &Ast, set: &mut ArtefactSet) {
    for pointer in pointers(ast, "reviews") {
        for path in markdown_paths(root, &pointer, set) {
            if let Some(parsed) = parse_file(&path, &pointer, set) {
                let Some(node) = required(&parsed.values, "node", path_string(&path), set) else {
                    continue;
                };
                let Some(date) = required(&parsed.values, "date", path_string(&path), set) else {
                    continue;
                };
                let Some(reviewer) = required(&parsed.values, "reviewer", path_string(&path), set)
                else {
                    continue;
                };
                let review_type = optional(&parsed.values, "review_type")
                    .map_or(Some(ReviewType::Human), |value| {
                        parse_review_type(&value, &path, set)
                    });
                let Some(review_type) = review_type else {
                    continue;
                };
                set.reviews.push(Review {
                    path: path_string(&path),
                    node,
                    review_type,
                    date,
                    reviewer,
                    related_change: optional(&parsed.values, "related_change"),
                    body: parsed.body,
                });
            }
        }
    }
}

fn load_research(root: &Path, ast: &Ast, set: &mut ArtefactSet) {
    for pointer in pointers(ast, "research") {
        for path in markdown_paths(root, &pointer, set) {
            if let Some(parsed) = parse_file(&path, &pointer, set) {
                let Some(id) = required(&parsed.values, "id", path_string(&path), set) else {
                    continue;
                };
                let Some(date) = required(&parsed.values, "date", path_string(&path), set) else {
                    continue;
                };
                set.research.push(Research {
                    id,
                    path: path_string(&path),
                    nodes: list(&parsed, "nodes"),
                    date,
                    sources: list(&parsed, "sources"),
                    tags: list(&parsed, "tags"),
                    body: parsed.body,
                });
            }
        }
    }
}

fn load_sources(root: &Path, ast: &Ast, set: &mut ArtefactSet) {
    for pointer in pointers(ast, "sources") {
        for path in markdown_paths(root, &pointer, set) {
            if let Some(parsed) = parse_file(&path, &pointer, set) {
                let Some(id) = required(&parsed.values, "id", path_string(&path), set) else {
                    continue;
                };
                let Some(file) = required(&parsed.values, "file", path_string(&path), set) else {
                    continue;
                };
                let Some(verification) =
                    required(&parsed.values, "verification", path_string(&path), set)
                        .and_then(|value| parse_source_verification(&value, &path, set))
                else {
                    continue;
                };
                let Some(source_type) = required(&parsed.values, "type", path_string(&path), set)
                else {
                    continue;
                };
                let Some(date) = required(&parsed.values, "date", path_string(&path), set) else {
                    continue;
                };
                set.sources.push(Source {
                    id,
                    path: path_string(&path),
                    file,
                    sha256: optional(&parsed.values, "sha256").filter(|value| value != "null"),
                    verification,
                    source_type,
                    date,
                    tags: list(&parsed, "tags"),
                    description: optional(&parsed.values, "description").unwrap_or_default(),
                    body: parsed.body,
                });
            }
        }
    }
}

fn validate_integrity(root: &Path, node_ids: &BTreeSet<String>, set: &mut ArtefactSet) {
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

fn validate_nodes(node_ids: &BTreeSet<String>, set: &mut ArtefactSet) {
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

fn validate_node_list(
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

fn validate_decision_refs(decisions: &BTreeMap<String, DecisionStatus>, set: &mut ArtefactSet) {
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

fn validate_provenance_refs(
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

fn validate_sources(root: &Path, source_ids: &BTreeSet<String>, set: &mut ArtefactSet) {
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
            SourceVerification::Unverified => set.findings.push(warning(
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

fn validate_verified_source(root: &Path, source: &Source, set: &mut ArtefactSet) {
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

fn pointers(ast: &Ast, field_name: &str) -> Vec<String> {
    let mut result = Vec::new();
    for node in &ast.nodes {
        collect_pointers(node, field_name, &mut result);
    }
    result.sort();
    result.dedup();
    result
}

fn collect_pointers(node: &Node, field_name: &str, result: &mut Vec<String>) {
    for Field { name, values, .. } in &node.raw_fields {
        if name == field_name {
            result.extend(values.iter().cloned());
        }
    }
    for child in &node.children {
        collect_pointers(child, field_name, result);
    }
}

fn collect_ids(ast: &Ast) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    for node in &ast.nodes {
        collect_node_id(node, &mut ids);
    }
    ids
}

fn collect_node_id(node: &Node, ids: &mut BTreeSet<String>) {
    ids.insert(node.id.clone());
    for child in &node.children {
        collect_node_id(child, ids);
    }
}

fn markdown_paths(root: &Path, pointer: &str, set: &mut ArtefactSet) -> Vec<PathBuf> {
    let path = root.join(pointer);
    if path.is_dir() {
        return read_dir_markdown(&path).unwrap_or_else(|error| {
            set.findings.push(error_finding(
                "CAIRN_ARTEFACT_DIR_READ_FAILED",
                format!("failed to read artefact directory `{pointer}`: {error}"),
                Some(pointer.to_owned()),
            ));
            Vec::new()
        });
    }
    if path.exists() {
        vec![path]
    } else {
        set.findings.push(warning(
            "CAIRN_ARTEFACT_POINTER_MISSING",
            format!("artefact pointer `{pointer}` is missing"),
            None,
            Some(pointer.to_owned()),
        ));
        Vec::new()
    }
}

fn read_dir_markdown(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut paths = fs::read_dir(path)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|entry| entry.extension().is_some_and(|ext| ext == "md"))
        .collect::<Vec<_>>();
    paths.sort();
    Ok(paths)
}

fn parse_file(
    path: &Path,
    pointer: &str,
    set: &mut ArtefactSet,
) -> Option<frontmatter::Frontmatter> {
    fs::read_to_string(path)
        .map(|source| frontmatter::parse(&source))
        .map_err(|error| {
            set.findings.push(error_finding(
                "CAIRN_ARTEFACT_READ_FAILED",
                format!(
                    "failed to read artefact `{}` from `{pointer}`: {error}",
                    path.display()
                ),
                Some(path_string(path)),
            ));
        })
        .ok()
}

fn required(
    values: &BTreeMap<String, String>,
    key: &str,
    path: String,
    set: &mut ArtefactSet,
) -> Option<String> {
    values
        .get(key)
        .filter(|value| !value.is_empty())
        .cloned()
        .or_else(|| {
            set.findings.push(error_finding(
                "CAIRN_ARTEFACT_MISSING_FIELD",
                format!("artefact `{path}` lacks required `{key}` frontmatter"),
                Some(path),
            ));
            None
        })
}

fn optional(values: &BTreeMap<String, String>, key: &str) -> Option<String> {
    values.get(key).filter(|value| !value.is_empty()).cloned()
}

fn list(parsed: &frontmatter::Frontmatter, key: &str) -> Vec<String> {
    parsed.lists.get(key).cloned().unwrap_or_default()
}

fn parse_todo_status(value: &str, path: &Path, set: &mut ArtefactSet) -> Option<TodoStatus> {
    match value {
        "open" => Some(TodoStatus::Open),
        "in_progress" => Some(TodoStatus::InProgress),
        "done" => Some(TodoStatus::Done),
        "blocked" => Some(TodoStatus::Blocked),
        _ => {
            set.findings.push(error_finding(
                "CAIRN_TODO_STATUS_INVALID",
                format!("todo `{}` has invalid status `{value}`", path.display()),
                Some(path_string(path)),
            ));
            None
        }
    }
}

fn parse_decision_status(
    value: &str,
    path: &Path,
    set: &mut ArtefactSet,
) -> Option<DecisionStatus> {
    match value {
        "proposed" => Some(DecisionStatus::Proposed),
        "accepted" => Some(DecisionStatus::Accepted),
        "deprecated" => Some(DecisionStatus::Deprecated),
        "superseded" => Some(DecisionStatus::Superseded),
        _ => {
            set.findings.push(error_finding(
                "CAIRN_DECISION_STATUS_INVALID",
                format!("decision `{}` has invalid status `{value}`", path.display()),
                Some(path_string(path)),
            ));
            None
        }
    }
}

fn parse_review_type(value: &str, path: &Path, set: &mut ArtefactSet) -> Option<ReviewType> {
    match value {
        "human" => Some(ReviewType::Human),
        "agent_introspective" => Some(ReviewType::AgentIntrospective),
        "agent_cross_model" => Some(ReviewType::AgentCrossModel),
        _ => {
            set.findings.push(error_finding(
                "CAIRN_REVIEW_TYPE_INVALID",
                format!(
                    "review `{}` has invalid review_type `{value}`",
                    path.display()
                ),
                Some(path_string(path)),
            ));
            None
        }
    }
}

fn parse_source_verification(
    value: &str,
    path: &Path,
    set: &mut ArtefactSet,
) -> Option<SourceVerification> {
    match value {
        "verified" => Some(SourceVerification::Verified),
        "external" => Some(SourceVerification::External),
        "unverified" => Some(SourceVerification::Unverified),
        _ => {
            set.findings.push(error_finding(
                "CAIRN_SOURCE_VERIFICATION_INVALID",
                format!(
                    "source `{}` has invalid verification `{value}`",
                    path.display()
                ),
                Some(path_string(path)),
            ));
            None
        }
    }
}

fn error(code: &str, message: String, node: Option<String>, path: Option<String>) -> Finding {
    Finding {
        code: code.to_owned(),
        severity: FindingSeverity::Error,
        message,
        node,
        path,
    }
}

fn warning(code: &str, message: String, node: Option<String>, path: Option<String>) -> Finding {
    Finding {
        code: code.to_owned(),
        severity: FindingSeverity::Warning,
        message,
        node,
        path,
    }
}

fn error_finding(code: &str, message: String, path: Option<String>) -> Finding {
    error(code, message, None, path)
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn is_url(value: &str) -> bool {
    value.starts_with("http://") || value.starts_with("https://")
}

const SHA256_INITIAL_STATE: [u32; 8] = [
    0x6a09_e667,
    0xbb67_ae85,
    0x3c6e_f372,
    0xa54f_f53a,
    0x510e_527f,
    0x9b05_688c,
    0x1f83_d9ab,
    0x5be0_cd19,
];

const SHA256_ROUND_CONSTANTS: [u32; 64] = [
    0x428a_2f98,
    0x7137_4491,
    0xb5c0_fbcf,
    0xe9b5_dba5,
    0x3956_c25b,
    0x59f1_11f1,
    0x923f_82a4,
    0xab1c_5ed5,
    0xd807_aa98,
    0x1283_5b01,
    0x2431_85be,
    0x550c_7dc3,
    0x72be_5d74,
    0x80de_b1fe,
    0x9bdc_06a7,
    0xc19b_f174,
    0xe49b_69c1,
    0xefbe_4786,
    0x0fc1_9dc6,
    0x240c_a1cc,
    0x2de9_2c6f,
    0x4a74_84aa,
    0x5cb0_a9dc,
    0x76f9_88da,
    0x983e_5152,
    0xa831_c66d,
    0xb003_27c8,
    0xbf59_7fc7,
    0xc6e0_0bf3,
    0xd5a7_9147,
    0x06ca_6351,
    0x1429_2967,
    0x27b7_0a85,
    0x2e1b_2138,
    0x4d2c_6dfc,
    0x5338_0d13,
    0x650a_7354,
    0x766a_0abb,
    0x81c2_c92e,
    0x9272_2c85,
    0xa2bf_e8a1,
    0xa81a_664b,
    0xc24b_8b70,
    0xc76c_51a3,
    0xd192_e819,
    0xd699_0624,
    0xf40e_3585,
    0x106a_a070,
    0x19a4_c116,
    0x1e37_6c08,
    0x2748_774c,
    0x34b0_bcb5,
    0x391c_0cb3,
    0x4ed8_aa4a,
    0x5b9c_ca4f,
    0x682e_6ff3,
    0x748f_82ee,
    0x78a5_636f,
    0x84c8_7814,
    0x8cc7_0208,
    0x90be_fffa,
    0xa450_6ceb,
    0xbef9_a3f7,
    0xc671_78f2,
];

fn sha256_hex(bytes: &[u8]) -> String {
    let mut data = bytes.to_vec();
    let bit_len = (data.len() as u64) * 8;
    data.push(0x80);
    while data.len() % 64 != 56 {
        data.push(0);
    }
    data.extend_from_slice(&bit_len.to_be_bytes());
    let mut state = SHA256_INITIAL_STATE;
    for chunk in data.chunks_exact(64) {
        compress_sha256_block(&mut state, chunk);
    }
    state.iter().fold(String::new(), |mut output, word| {
        let _ = write!(output, "{word:08x}");
        output
    })
}

fn compress_sha256_block(state: &mut [u32; 8], chunk: &[u8]) {
    let schedule = sha256_schedule(chunk);
    let mut work = *state;
    for index in 0..64 {
        let big_sigma1 =
            work[4].rotate_right(6) ^ work[4].rotate_right(11) ^ work[4].rotate_right(25);
        let choose = (work[4] & work[5]) ^ ((!work[4]) & work[6]);
        let temp1 = work[7]
            .wrapping_add(big_sigma1)
            .wrapping_add(choose)
            .wrapping_add(SHA256_ROUND_CONSTANTS[index])
            .wrapping_add(schedule[index]);
        let big_sigma0 =
            work[0].rotate_right(2) ^ work[0].rotate_right(13) ^ work[0].rotate_right(22);
        let majority = (work[0] & work[1]) ^ (work[0] & work[2]) ^ (work[1] & work[2]);
        let temp2 = big_sigma0.wrapping_add(majority);
        work = [
            temp1.wrapping_add(temp2),
            work[0],
            work[1],
            work[2],
            work[3].wrapping_add(temp1),
            work[4],
            work[5],
            work[6],
        ];
    }
    for (slot, value) in state.iter_mut().zip(work) {
        *slot = slot.wrapping_add(value);
    }
}

fn sha256_schedule(chunk: &[u8]) -> [u32; 64] {
    let mut schedule = [0_u32; 64];
    for (index, word) in schedule.iter_mut().take(16).enumerate() {
        let offset = index * 4;
        *word = u32::from_be_bytes([
            chunk[offset],
            chunk[offset + 1],
            chunk[offset + 2],
            chunk[offset + 3],
        ]);
    }
    for index in 16..64 {
        let sigma0 = schedule[index - 15].rotate_right(7)
            ^ schedule[index - 15].rotate_right(18)
            ^ (schedule[index - 15] >> 3);
        let sigma1 = schedule[index - 2].rotate_right(17)
            ^ schedule[index - 2].rotate_right(19)
            ^ (schedule[index - 2] >> 10);
        schedule[index] = schedule[index - 16]
            .wrapping_add(sigma0)
            .wrapping_add(schedule[index - 7])
            .wrapping_add(sigma1);
    }
    schedule
}
