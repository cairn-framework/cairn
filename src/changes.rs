// cairn:allow-large-module reason: scheduled-for-phase-7.5b-split
//! Change directory discovery, delta parsing, validation, and archive support.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

use crate::{
    artefacts::frontmatter,
    blueprint::{Ast, Edge, Node, NodeKind, parser::parse_str},
    map::Graph,
    scanner,
};

/// Parsed active change metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Change {
    /// Change ID, derived from the directory name.
    pub id: String,
    /// Change directory path.
    pub path: PathBuf,
    /// Proposal title.
    pub title: String,
    /// Proposal markdown.
    pub proposal: String,
    /// Optional design markdown.
    pub design: Option<String>,
    /// Parsed blueprint delta.
    pub delta: BlueprintDelta,
    /// Parsed artefact operations.
    pub artefacts: Vec<ArtefactOperation>,
    /// Validation messages collected while loading the change.
    pub findings: Vec<String>,
}

/// Blueprint delta operations.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BlueprintDelta {
    /// Added nodes.
    pub added_nodes: Vec<Node>,
    /// Modified node declarations.
    pub modified_nodes: Vec<Node>,
    /// Removed node IDs.
    pub removed_nodes: Vec<String>,
    /// Renamed node IDs.
    pub renamed_nodes: Vec<Rename>,
    /// Added edges.
    pub added_edges: Vec<Edge>,
    /// Modified edges.
    pub modified_edges: Vec<Edge>,
    /// Removed edges.
    pub removed_edges: Vec<Edge>,
    /// Renamed edge endpoints.
    pub renamed_edges: Vec<EdgeRename>,
}

/// Old and new ID pair.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rename {
    /// Existing ID.
    pub from: String,
    /// Proposed ID.
    pub to: String,
}

/// Edge replacement pair.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdgeRename {
    /// Existing edge.
    pub from: Edge,
    /// Proposed edge.
    pub to: Edge,
}

/// Artefact operation parsed from mirrored change directories.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtefactOperation {
    /// Operation type.
    pub operation: ChangeOperation,
    /// Path inside the change directory.
    pub change_path: PathBuf,
    /// Target path in the main tree.
    pub target_path: PathBuf,
    /// Source path for rename operations.
    pub renamed_from: Option<PathBuf>,
    /// File content.
    pub content: String,
}

/// Supported operation kinds.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ChangeOperation {
    /// Add a new item.
    Added,
    /// Modify an existing item.
    Modified,
    /// Remove an existing item.
    Removed,
    /// Rename an existing item.
    Renamed,
}

/// Archive outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArchiveReport {
    /// Archive directory path.
    pub archive_path: PathBuf,
    /// Human-readable operation summary.
    pub summary: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Snapshot {
    path: PathBuf,
    content: Option<Vec<u8>>,
}

/// Discovers active changes under `meta/changes`.
///
/// # Errors
///
/// Returns an I/O error if `meta/changes` exists but cannot be read.
pub fn discover(root: &Path) -> io::Result<Vec<Change>> {
    let changes_root = root.join("meta/changes");
    if !changes_root.exists() {
        return Ok(Vec::new());
    }
    let mut changes = Vec::new();
    for entry in fs::read_dir(changes_root)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if !file_type.is_dir() {
            continue;
        }
        let path = entry.path();
        if path.file_name().is_some_and(|name| name == "archive") {
            continue;
        }
        if !path.join("proposal.md").exists() {
            continue;
        }
        changes.push(load_change(root, path));
    }
    changes.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(changes)
}

/// Loads one active change from disk.
#[must_use]
pub fn load_change(root: &Path, path: PathBuf) -> Change {
    let id = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_owned();
    let mut findings = Vec::new();
    let proposal = read_to_string(&path.join("proposal.md"), &mut findings);
    let title = proposal_title(&proposal).unwrap_or_else(|| id.clone());
    let design_path = path.join("design.md");
    let design = design_path
        .exists()
        .then(|| read_to_string(&design_path, &mut findings));
    let delta_path = path.join("blueprint.delta");
    let delta_source = if delta_path.exists() {
        read_to_string(&delta_path, &mut findings)
    } else {
        String::new()
    };
    let delta = parse_blueprint_delta(&delta_path.display().to_string(), &delta_source)
        .map_err(|message| findings.push(message))
        .unwrap_or_default();
    let artefacts = parse_artefact_operations(root, &path, &mut findings);
    Change {
        id,
        path,
        title,
        proposal,
        design,
        delta,
        artefacts,
        findings,
    }
}

/// Parses a `blueprint.delta` document.
///
/// # Errors
///
/// Returns a human-readable parse error when a section contains invalid syntax.
pub fn parse_blueprint_delta(file: &str, source: &str) -> Result<BlueprintDelta, String> {
    let sections = delta_sections(source);
    Ok(BlueprintDelta {
        added_nodes: parse_node_section(file, sections.get("ADDED Nodes").map(String::as_str))?,
        modified_nodes: parse_node_section(
            file,
            sections.get("MODIFIED Nodes").map(String::as_str),
        )?,
        removed_nodes: parse_id_lines(sections.get("REMOVED Nodes").map(String::as_str)),
        renamed_nodes: parse_rename_lines(sections.get("RENAMED Nodes").map(String::as_str))?,
        added_edges: parse_edge_section(file, sections.get("ADDED Edges").map(String::as_str))?,
        modified_edges: parse_edge_section(
            file,
            sections.get("MODIFIED Edges").map(String::as_str),
        )?,
        removed_edges: parse_edge_section(file, sections.get("REMOVED Edges").map(String::as_str))?,
        renamed_edges: parse_edge_renames(file, sections.get("RENAMED Edges").map(String::as_str))?,
    })
}

/// Validates change references against current truth.
#[must_use]
pub fn validate_change(change: &Change, graph: &Graph) -> Vec<String> {
    let mut errors = change.findings.clone();
    let mut existing_nodes = graph.nodes.keys().cloned().collect::<BTreeSet<_>>();
    let mut added_nodes = BTreeSet::new();
    let mut touched_nodes = BTreeSet::new();
    for node in &change.delta.added_nodes {
        if !added_nodes.insert(node.id.clone()) || existing_nodes.contains(&node.id) {
            errors.push(format!("node `{}` has duplicate add operation", node.id));
        }
    }
    for rename in &change.delta.renamed_nodes {
        if !existing_nodes.contains(&rename.from) {
            errors.push(format!("renamed node `{}` does not exist", rename.from));
        }
        if existing_nodes.contains(&rename.to) || added_nodes.contains(&rename.to) {
            errors.push(format!(
                "renamed node target `{}` already exists",
                rename.to
            ));
        }
        mark_node_touch(&mut touched_nodes, &rename.from, &mut errors);
        existing_nodes.remove(&rename.from);
        existing_nodes.insert(rename.to.clone());
    }
    for id in &change.delta.removed_nodes {
        if !existing_nodes.contains(id) {
            errors.push(format!("removed node `{id}` does not exist"));
        }
        mark_node_touch(&mut touched_nodes, id, &mut errors);
        existing_nodes.remove(id);
    }
    for node in &change.delta.modified_nodes {
        if !existing_nodes.contains(&node.id) {
            errors.push(format!("modified node `{}` does not exist", node.id));
        }
        mark_node_touch(&mut touched_nodes, &node.id, &mut errors);
    }
    existing_nodes.extend(added_nodes);
    validate_edges(&change.delta, &existing_nodes, graph, &mut errors);
    validate_artefacts(&change.artefacts, graph, &mut errors);
    errors
}

/// Applies and archives an active change.
///
/// # Errors
///
/// Returns an error when validation, mutation, scan, or archive movement fails.
pub fn archive(
    root: &Path,
    blueprint_path: &Path,
    change_id: &str,
) -> Result<ArchiveReport, String> {
    let changes = discover(root).map_err(|error| error.to_string())?;
    let Some(change) = changes.into_iter().find(|change| change.id == change_id) else {
        return Err(format!("change `{change_id}` was not found"));
    };
    let before = scanner::load_project(root, blueprint_path)?;
    let validation = validate_change(&change, &before.graph);
    if !validation.is_empty() {
        return Err(validation.join("; "));
    }
    let mut mutated = mutation_paths(root, blueprint_path, &change);
    mutated.sort();
    mutated.dedup();
    let snapshots = snapshot_paths(&mutated).map_err(|error| error.to_string())?;
    if let Err(error) = apply_archive(root, blueprint_path, &change).and_then(|()| {
        let scan = scanner::load_project(root, blueprint_path)?;
        if scan.graph.has_errors() {
            let messages = scan
                .graph
                .findings
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("; ");
            Err(messages)
        } else {
            Ok(())
        }
    }) {
        restore_snapshots(&snapshots)
            .map_err(|restore| format!("archive failed: {error}; rollback failed: {restore}"))?;
        return Err(error);
    }
    let archive_path = archive_path(root, &change.id);
    if archive_path.exists() {
        restore_snapshots(&snapshots).map_err(|restore| {
            format!(
                "archive destination `{}` already exists; rollback failed: {restore}",
                archive_path.display()
            )
        })?;
        return Err(format!(
            "archive destination `{}` already exists",
            archive_path.display()
        ));
    }
    if let Some(parent) = archive_path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::rename(&change.path, &archive_path).map_err(|error| error.to_string())?;
    scanner::scan(root, blueprint_path)?;
    append_archive_log(root, &change)?;
    Ok(ArchiveReport {
        archive_path,
        summary: operation_summary(&change),
    })
}

/// Creates a reviewable rename change without mutating current truth.
///
/// # Errors
///
/// Returns an error when the old node is missing, the target exists, or files
/// cannot be written.
pub fn create_rename_change(
    root: &Path,
    blueprint_path: &Path,
    old_id: &str,
    new_id: &str,
) -> Result<Change, String> {
    let scan = scanner::load_project(root, blueprint_path)?;
    if !scan.graph.nodes.contains_key(old_id) {
        return Err(format!("node `{old_id}` was not found"));
    }
    if scan.graph.nodes.contains_key(new_id) {
        return Err(format!("node `{new_id}` already exists"));
    }
    let change_id = format!("rename-{old_id}-to-{new_id}");
    let change_path = root.join("meta/changes").join(&change_id);
    if change_path.exists() {
        return Err(format!("change `{change_id}` already exists"));
    }
    fs::create_dir_all(&change_path).map_err(|error| error.to_string())?;
    atomic_write(
        &change_path.join("proposal.md"),
        &format!("# Proposal: Rename {old_id} to {new_id}\n\nRename `{old_id}` to `{new_id}`.\n"),
    )?;
    let mut delta = format!("## RENAMED Nodes\n- {old_id} -> {new_id}\n");
    let mut changed_edges = Vec::new();
    for edges in scan.graph.outbound.values() {
        for edge in edges {
            if edge.from == old_id || edge.to == old_id {
                changed_edges.push(edge.clone());
            }
        }
    }
    if !changed_edges.is_empty() {
        delta.push_str("\n## RENAMED Edges\n");
        for edge in changed_edges {
            let new_from = replace_exact_id(&edge.from, old_id, new_id);
            let new_to = replace_exact_id(&edge.to, old_id, new_id);
            let _ = writeln!(
                delta,
                "{} -> {} {:?} => {} -> {} {:?}",
                edge.from, edge.to, edge.description, new_from, new_to, edge.description
            );
        }
    }
    atomic_write(&change_path.join("blueprint.delta"), &delta)?;
    copy_referencing_artefacts(root, &change_path, old_id, new_id)?;
    Ok(load_change(root, change_path))
}

/// Returns a human-readable operation count summary.
#[must_use]
pub fn operation_summary(change: &Change) -> String {
    let mut counts = BTreeMap::<String, usize>::new();
    for (label, count) in [
        ("added_nodes", change.delta.added_nodes.len()),
        ("modified_nodes", change.delta.modified_nodes.len()),
        ("removed_nodes", change.delta.removed_nodes.len()),
        ("renamed_nodes", change.delta.renamed_nodes.len()),
        ("added_edges", change.delta.added_edges.len()),
        ("modified_edges", change.delta.modified_edges.len()),
        ("removed_edges", change.delta.removed_edges.len()),
        ("renamed_edges", change.delta.renamed_edges.len()),
    ] {
        if count > 0 {
            counts.insert(label.to_owned(), count);
        }
    }
    for artefact in &change.artefacts {
        let key = format!("{:?}_artefacts", artefact.operation).to_lowercase();
        *counts.entry(key).or_default() += 1;
    }
    if counts.is_empty() {
        "no operations".to_owned()
    } else {
        counts
            .into_iter()
            .map(|(label, count)| format!("{count} {label}"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// Returns proposed operations related to a node or direct neighbour.
#[must_use]
pub fn operations_for_nodes(changes: &[Change], nodes: &BTreeSet<String>) -> Vec<String> {
    let mut lines = Vec::new();
    for change in changes {
        for node in &change.delta.added_nodes {
            if nodes.contains(&node.id) {
                lines.push(format!("{}: added node {}", change.id, node.id));
            }
        }
        for node in &change.delta.modified_nodes {
            if nodes.contains(&node.id) {
                lines.push(format!("{}: modified node {}", change.id, node.id));
            }
        }
        for id in &change.delta.removed_nodes {
            if nodes.contains(id) {
                lines.push(format!("{}: removed node {id}", change.id));
            }
        }
        for rename in &change.delta.renamed_nodes {
            if nodes.contains(&rename.from) || nodes.contains(&rename.to) {
                lines.push(format!(
                    "{}: renamed node {} -> {}",
                    change.id, rename.from, rename.to
                ));
            }
        }
        for edge in change
            .delta
            .added_edges
            .iter()
            .chain(change.delta.modified_edges.iter())
            .chain(change.delta.removed_edges.iter())
        {
            if nodes.contains(&edge.from) || nodes.contains(&edge.to) {
                lines.push(format!(
                    "{}: edge {} -> {} ({})",
                    change.id, edge.from, edge.to, edge.description
                ));
            }
        }
        for artefact in &change.artefacts {
            if artefact_content_refs(&artefact.content, nodes) {
                lines.push(format!(
                    "{}: {:?} artefact {}",
                    change.id,
                    artefact.operation,
                    artefact.target_path.display()
                ));
            }
        }
    }
    lines
}

/// Renders active changes for generated `map.md`.
#[must_use]
pub fn active_changes_lines(changes: &[Change]) -> Vec<String> {
    changes
        .iter()
        .map(|change| {
            format!(
                "{} - {} ({})",
                change.id,
                change.title,
                operation_summary(change)
            )
        })
        .collect()
}

fn read_to_string(path: &Path, findings: &mut Vec<String>) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| {
        findings.push(format!("failed to read `{}`: {error}", path.display()));
        String::new()
    })
}

fn proposal_title(source: &str) -> Option<String> {
    source.lines().find_map(|line| {
        line.strip_prefix("# Proposal:")
            .or_else(|| line.strip_prefix("# "))
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
    })
}

fn delta_sections(source: &str) -> BTreeMap<String, String> {
    let mut sections = BTreeMap::<String, String>::new();
    let mut current: Option<String> = None;
    for line in source.lines() {
        if let Some(title) = line.trim().strip_prefix("## ") {
            current = Some(title.trim().to_owned());
            sections.entry(title.trim().to_owned()).or_default();
            continue;
        }
        if let Some(title) = &current {
            sections.entry(title.clone()).or_default().push_str(line);
            sections.entry(title.clone()).or_default().push('\n');
        }
    }
    sections
}

fn parse_node_section(file: &str, section: Option<&str>) -> Result<Vec<Node>, String> {
    let Some(section) = section else {
        return Ok(Vec::new());
    };
    let source = uncomment_lines(section);
    if source.trim().is_empty() {
        return Ok(Vec::new());
    }
    let ast = parse_str(file, &source).map_err(|error| error.to_string())?;
    if !ast.edges.is_empty() {
        return Err(format!(
            "{file}: node delta section contains edge operations"
        ));
    }
    Ok(flatten_nodes(ast.nodes))
}

fn parse_edge_section(file: &str, section: Option<&str>) -> Result<Vec<Edge>, String> {
    let Some(section) = section else {
        return Ok(Vec::new());
    };
    let source = uncomment_lines(section);
    if source.trim().is_empty() {
        return Ok(Vec::new());
    }
    let ast = parse_str(file, &source).map_err(|error| error.to_string())?;
    if !ast.nodes.is_empty() {
        return Err(format!(
            "{file}: edge delta section contains node operations"
        ));
    }
    Ok(ast.edges)
}

fn parse_id_lines(section: Option<&str>) -> Vec<String> {
    section
        .into_iter()
        .flat_map(str::lines)
        .map(clean_list_line)
        .filter(|line| !line.is_empty())
        .collect()
}

fn parse_rename_lines(section: Option<&str>) -> Result<Vec<Rename>, String> {
    section
        .into_iter()
        .flat_map(str::lines)
        .map(clean_list_line)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let Some((from, to)) = line.split_once("->") else {
                return Err(format!("malformed rename operation `{line}`"));
            };
            Ok(Rename {
                from: clean_scalar(from),
                to: clean_scalar(to),
            })
        })
        .collect()
}

fn parse_edge_renames(file: &str, section: Option<&str>) -> Result<Vec<EdgeRename>, String> {
    section
        .into_iter()
        .flat_map(str::lines)
        .map(clean_list_line)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let Some((from, to)) = line.split_once("=>") else {
                return Err(format!("malformed edge rename operation `{line}`"));
            };
            let from_edges = parse_edge_section(file, Some(from))?;
            let to_edges = parse_edge_section(file, Some(to))?;
            let [from_edge] = from_edges.as_slice() else {
                return Err(format!(
                    "edge rename source must contain one edge: `{line}`"
                ));
            };
            let [to_edge] = to_edges.as_slice() else {
                return Err(format!(
                    "edge rename target must contain one edge: `{line}`"
                ));
            };
            Ok(EdgeRename {
                from: from_edge.clone(),
                to: to_edge.clone(),
            })
        })
        .collect()
}

fn uncomment_lines(source: &str) -> String {
    source
        .lines()
        .map(clean_list_line)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn clean_list_line(line: &str) -> String {
    let trimmed = line.trim();
    let trimmed = trimmed.strip_prefix("- ").unwrap_or(trimmed);
    trimmed.trim().to_owned()
}

fn clean_scalar(value: &str) -> String {
    value
        .trim()
        .trim_matches('`')
        .trim_matches('"')
        .trim_matches('\'')
        .to_owned()
}

fn flatten_nodes(nodes: Vec<Node>) -> Vec<Node> {
    let mut flattened = Vec::new();
    for node in nodes {
        flattened.push(node.clone());
        flattened.extend(flatten_nodes(node.children));
    }
    flattened
}

fn parse_artefact_operations(
    root: &Path,
    change_path: &Path,
    findings: &mut Vec<String>,
) -> Vec<ArtefactOperation> {
    let mut operations = Vec::new();
    for dir in [
        "contracts",
        "todos",
        "decisions",
        "research",
        "sources",
        "reviews",
    ] {
        let path = change_path.join(dir);
        if path.exists() {
            collect_artefact_operations(root, change_path, &path, findings, &mut operations);
        }
    }
    operations.sort_by(|left, right| left.target_path.cmp(&right.target_path));
    operations
}

fn collect_artefact_operations(
    root: &Path,
    change_path: &Path,
    path: &Path,
    findings: &mut Vec<String>,
    operations: &mut Vec<ArtefactOperation>,
) {
    let Ok(entries) = fs::read_dir(path) else {
        findings.push(format!("failed to read `{}`", path.display()));
        return;
    };
    for entry in entries.flatten() {
        let entry_path = entry.path();
        if entry_path.is_dir() {
            collect_artefact_operations(root, change_path, &entry_path, findings, operations);
            continue;
        }
        if entry_path
            .extension()
            .is_none_or(|extension| extension != "md")
        {
            continue;
        }
        let content = read_to_string(&entry_path, findings);
        let parsed = frontmatter::parse(&content);
        let Some(operation) = parsed
            .values
            .get("operation")
            .and_then(|value| parse_operation(value))
        else {
            findings.push(format!(
                "artefact `{}` is missing valid operation frontmatter",
                entry_path.display()
            ));
            continue;
        };
        let Ok(relative) = entry_path.strip_prefix(change_path) else {
            findings.push(format!(
                "artefact `{}` is outside change",
                entry_path.display()
            ));
            continue;
        };
        let target_path = root.join("meta").join(relative);
        let renamed_from = parsed
            .values
            .get("renamed_from")
            .map(|value| root.join(clean_scalar(value)));
        operations.push(ArtefactOperation {
            operation,
            change_path: entry_path,
            target_path,
            renamed_from,
            content,
        });
    }
}

fn parse_operation(value: &str) -> Option<ChangeOperation> {
    match value {
        "added" => Some(ChangeOperation::Added),
        "modified" => Some(ChangeOperation::Modified),
        "removed" => Some(ChangeOperation::Removed),
        "renamed" => Some(ChangeOperation::Renamed),
        _ => None,
    }
}

fn validate_edges(
    delta: &BlueprintDelta,
    available_nodes: &BTreeSet<String>,
    graph: &Graph,
    errors: &mut Vec<String>,
) {
    for edge in delta
        .added_edges
        .iter()
        .chain(delta.modified_edges.iter())
        .chain(delta.renamed_edges.iter().map(|rename| &rename.to))
    {
        if !available_nodes.contains(&edge.from) || !available_nodes.contains(&edge.to) {
            errors.push(format!(
                "edge `{}` -> `{}` references missing endpoint",
                edge.from, edge.to
            ));
        }
    }
    for edge in delta
        .removed_edges
        .iter()
        .chain(delta.modified_edges.iter())
        .chain(delta.renamed_edges.iter().map(|rename| &rename.from))
    {
        if !graph_edge_exists(graph, edge) {
            errors.push(format!(
                "edge `{}` -> `{}` ({}) does not exist",
                edge.from, edge.to, edge.description
            ));
        }
    }
}

fn graph_edge_exists(graph: &Graph, edge: &Edge) -> bool {
    graph.outbound.get(&edge.from).is_some_and(|edges| {
        edges
            .iter()
            .any(|existing| existing.to == edge.to && existing.description == edge.description)
    })
}

fn mark_node_touch(touched: &mut BTreeSet<String>, id: &str, errors: &mut Vec<String>) {
    if !touched.insert(id.to_owned()) {
        errors.push(format!("node `{id}` has conflicting operations"));
    }
}

fn validate_artefacts(artefacts: &[ArtefactOperation], graph: &Graph, errors: &mut Vec<String>) {
    let mut targets = BTreeSet::<PathBuf>::new();
    for artefact in artefacts {
        if !targets.insert(artefact.target_path.clone()) {
            errors.push(format!(
                "artefact `{}` has duplicate operations",
                artefact.target_path.display()
            ));
        }
        match artefact.operation {
            ChangeOperation::Added => {
                if artefact.target_path.exists() {
                    errors.push(format!(
                        "added artefact target `{}` already exists",
                        artefact.target_path.display()
                    ));
                }
            }
            ChangeOperation::Modified | ChangeOperation::Removed => {
                if !artefact.target_path.exists() {
                    errors.push(format!(
                        "{:?} artefact target `{}` does not exist",
                        artefact.operation,
                        artefact.target_path.display()
                    ));
                }
            }
            ChangeOperation::Renamed => {
                if artefact.renamed_from.is_none() {
                    errors.push(format!(
                        "renamed artefact `{}` is missing renamed_from",
                        artefact.change_path.display()
                    ));
                }
                if let Some(source) = &artefact.renamed_from
                    && !source.exists()
                {
                    errors.push(format!(
                        "renamed artefact source `{}` does not exist",
                        source.display()
                    ));
                }
            }
        }
        validate_artefact_refs(artefact, graph, errors);
    }
}

fn validate_artefact_refs(artefact: &ArtefactOperation, graph: &Graph, errors: &mut Vec<String>) {
    let parsed = frontmatter::parse(&artefact.content);
    for key in ["node", "nodes"] {
        let ids = if key == "node" {
            parsed
                .values
                .get(key)
                .map(|value| vec![value.clone()])
                .unwrap_or_default()
        } else {
            parsed.lists.get(key).cloned().unwrap_or_default()
        };
        for id in ids {
            if !graph.nodes.contains_key(&id) {
                errors.push(format!(
                    "artefact `{}` references unknown node `{id}`",
                    artefact.change_path.display()
                ));
            }
        }
    }
}

fn mutation_paths(root: &Path, blueprint_path: &Path, change: &Change) -> Vec<PathBuf> {
    let mut paths = vec![root.join(blueprint_path)];
    paths.extend(change.artefacts.iter().flat_map(|artefact| {
        let mut paths = vec![artefact.target_path.clone()];
        if let Some(source) = &artefact.renamed_from {
            paths.push(source.clone());
        }
        paths
    }));
    paths.push(root.join("map.md"));
    paths.push(root.join(".cairn/log.md"));
    paths.push(root.join(".cairn/state/interface-hashes.json"));
    paths
}

fn snapshot_paths(paths: &[PathBuf]) -> io::Result<Vec<Snapshot>> {
    paths
        .iter()
        .map(|path| {
            let content = if path.exists() {
                Some(fs::read(path)?)
            } else {
                None
            };
            Ok(Snapshot {
                path: path.clone(),
                content,
            })
        })
        .collect()
}

fn restore_snapshots(snapshots: &[Snapshot]) -> io::Result<()> {
    for snapshot in snapshots {
        match &snapshot.content {
            Some(content) => {
                if let Some(parent) = snapshot.path.parent() {
                    fs::create_dir_all(parent)?;
                }
                atomic_write_bytes(&snapshot.path, content)?;
            }
            None if snapshot.path.exists() => {
                if snapshot.path.is_dir() {
                    fs::remove_dir_all(&snapshot.path)?;
                } else {
                    fs::remove_file(&snapshot.path)?;
                }
            }
            None => {}
        }
    }
    Ok(())
}

fn apply_archive(root: &Path, blueprint_path: &Path, change: &Change) -> Result<(), String> {
    let full_blueprint = root.join(blueprint_path);
    let source = fs::read_to_string(&full_blueprint).map_err(|error| error.to_string())?;
    let next = apply_blueprint_delta(&source, &change.delta)?;
    atomic_write(&full_blueprint, &next)?;
    apply_artefact_operations(&change.artefacts)?;
    Ok(())
}

fn apply_blueprint_delta(source: &str, delta: &BlueprintDelta) -> Result<String, String> {
    let ast = parse_str("cairn.blueprint", source).map_err(|error| error.to_string())?;
    let mut nodes = ast.nodes;
    for rename in &delta.renamed_nodes {
        rename_node_id(&mut nodes, &rename.from, &rename.to);
    }
    for id in &delta.removed_nodes {
        remove_node(&mut nodes, id);
    }
    for node in &delta.modified_nodes {
        replace_node(&mut nodes, node)?;
    }
    nodes.extend(delta.added_nodes.clone());
    let mut edges = ast.edges;
    for rename in &delta.renamed_nodes {
        for edge in &mut edges {
            edge.from = replace_exact_id(&edge.from, &rename.from, &rename.to);
            edge.to = replace_exact_id(&edge.to, &rename.from, &rename.to);
        }
    }
    for edge in &delta.removed_edges {
        edges.retain(|candidate| !same_edge(candidate, edge));
    }
    for rename in &delta.renamed_edges {
        edges.retain(|candidate| !same_edge(candidate, &rename.from));
        edges.push(rename.to.clone());
    }
    for edge in &delta.modified_edges {
        edges.retain(|candidate| !(candidate.from == edge.from && candidate.to == edge.to));
        edges.push(edge.clone());
    }
    edges.extend(delta.added_edges.clone());
    Ok(serialize_ast(&Ast { nodes, edges }))
}

fn rename_node_id(nodes: &mut [Node], from: &str, to: &str) {
    for node in nodes {
        if node.id == from {
            to.clone_into(&mut node.id);
        }
        rename_node_id(&mut node.children, from, to);
    }
}

fn remove_node(nodes: &mut Vec<Node>, id: &str) {
    nodes.retain(|node| node.id != id);
    for node in nodes {
        remove_node(&mut node.children, id);
    }
}

fn replace_node(nodes: &mut [Node], replacement: &Node) -> Result<(), String> {
    for node in nodes {
        if node.id == replacement.id {
            *node = replacement.clone();
            return Ok(());
        }
        if replace_node(&mut node.children, replacement).is_ok() {
            return Ok(());
        }
    }
    Err(format!("modified node `{}` was not found", replacement.id))
}

fn same_edge(left: &Edge, right: &Edge) -> bool {
    left.from == right.from && left.to == right.to && left.description == right.description
}

fn serialize_ast(ast: &Ast) -> String {
    let mut output = String::new();
    for node in &ast.nodes {
        serialize_node(node, 0, &mut output);
    }
    for edge in &ast.edges {
        let _ = writeln!(
            output,
            "{} -> {} {:?}",
            edge.from, edge.to, edge.description
        );
    }
    output
}

fn serialize_node(node: &Node, indent: usize, output: &mut String) {
    let pad = " ".repeat(indent);
    let _ = write!(
        output,
        "{}{} {} {:?} id {:?}",
        pad,
        node_kind_name(node.kind),
        node.name,
        node.description,
        node.id
    );
    for tag in &node.tags {
        let _ = write!(output, " @{tag}");
    }
    output.push_str(" {\n");
    for path in &node.paths {
        let _ = writeln!(output, "{pad}    path {path:?}");
    }
    if node.owns_files {
        let _ = writeln!(output, "{pad}    owns-files: true");
    }
    for contract in &node.contracts {
        let _ = writeln!(output, "{pad}    contract {contract:?}");
    }
    for field in &node.raw_fields {
        let values = serialize_field_values(&field.values);
        let _ = writeln!(output, "{}    {} {}", pad, field.name, values);
    }
    for child in &node.children {
        serialize_node(child, indent + 4, output);
    }
    let _ = writeln!(output, "{pad}}}");
}

fn node_kind_name(kind: NodeKind) -> &'static str {
    match kind {
        NodeKind::System => "System",
        NodeKind::Container => "Container",
        NodeKind::Module => "Module",
        NodeKind::Actor => "Actor",
    }
}

fn serialize_field_values(values: &[String]) -> String {
    if let [value] = values {
        format!("{value:?}")
    } else {
        format!(
            "[{}]",
            values
                .iter()
                .map(|value| format!("{value:?}"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

fn replace_exact_id(value: &str, old_id: &str, new_id: &str) -> String {
    if value == old_id {
        new_id.to_owned()
    } else {
        value.to_owned()
    }
}

fn apply_artefact_operations(artefacts: &[ArtefactOperation]) -> Result<(), String> {
    for operation in [
        ChangeOperation::Renamed,
        ChangeOperation::Removed,
        ChangeOperation::Modified,
        ChangeOperation::Added,
    ] {
        for artefact in artefacts
            .iter()
            .filter(|artefact| artefact.operation == operation)
        {
            match operation {
                ChangeOperation::Renamed => {
                    let Some(source) = &artefact.renamed_from else {
                        return Err(format!(
                            "renamed artefact `{}` is missing renamed_from",
                            artefact.change_path.display()
                        ));
                    };
                    if source.exists() {
                        fs::remove_file(source).map_err(|error| error.to_string())?;
                    }
                    write_artefact_target(artefact)?;
                }
                ChangeOperation::Removed => {
                    fs::remove_file(&artefact.target_path).map_err(|error| error.to_string())?;
                }
                ChangeOperation::Modified | ChangeOperation::Added => {
                    write_artefact_target(artefact)?;
                }
            }
        }
    }
    Ok(())
}

fn write_artefact_target(artefact: &ArtefactOperation) -> Result<(), String> {
    let content = strip_change_frontmatter(&artefact.content);
    atomic_write(&artefact.target_path, &content)
}

fn strip_change_frontmatter(source: &str) -> String {
    let mut output = Vec::new();
    let mut in_frontmatter = false;
    let mut seen_start = false;
    for line in source.lines() {
        if !seen_start && line.trim() == "---" {
            seen_start = true;
            in_frontmatter = true;
            output.push(line.to_owned());
            continue;
        }
        if in_frontmatter && line.trim() == "---" {
            in_frontmatter = false;
            output.push(line.to_owned());
            continue;
        }
        if in_frontmatter
            && (line.trim_start().starts_with("operation:")
                || line.trim_start().starts_with("renamed_from:"))
        {
            continue;
        }
        output.push(line.to_owned());
    }
    format!("{}\n", output.join("\n"))
}

fn archive_path(root: &Path, change_id: &str) -> PathBuf {
    root.join("meta/changes/archive")
        .join(format!("{}-{change_id}", today_utc()))
}

fn today_utc() -> String {
    std::process::Command::new("date")
        .args(["-u", "+%F"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "1970-01-01".to_owned())
}

fn append_archive_log(root: &Path, change: &Change) -> Result<(), String> {
    fs::create_dir_all(root.join(".cairn")).map_err(|error| error.to_string())?;
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(root.join(".cairn/log.md"))
        .map_err(|error| error.to_string())?;
    writeln!(
        file,
        "- archive: {} merged; {}",
        change.id,
        operation_summary(change)
    )
    .map_err(|error| error.to_string())
}

fn atomic_write(path: &Path, content: &str) -> Result<(), String> {
    atomic_write_bytes(path, content.as_bytes()).map_err(|error| error.to_string())
}

fn atomic_write_bytes(path: &Path, content: &[u8]) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("tmp-cairn-write");
    fs::write(&tmp, content)?;
    fs::rename(tmp, path)
}

fn copy_referencing_artefacts(
    root: &Path,
    change_path: &Path,
    old_id: &str,
    new_id: &str,
) -> Result<(), String> {
    let meta = root.join("meta");
    if !meta.exists() {
        return Ok(());
    }
    copy_referencing_artefacts_from(root, change_path, &meta, old_id, new_id)
}

fn copy_referencing_artefacts_from(
    root: &Path,
    change_path: &Path,
    dir: &Path,
    old_id: &str,
    new_id: &str,
) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            if path
                .strip_prefix(root)
                .is_ok_and(|relative| relative.starts_with("meta/changes"))
            {
                continue;
            }
            copy_referencing_artefacts_from(root, change_path, &path, old_id, new_id)?;
            continue;
        }
        if path.extension().is_none_or(|extension| extension != "md") {
            continue;
        }
        let content = fs::read_to_string(&path).map_err(|error| error.to_string())?;
        if !frontmatter_references(&content, old_id) {
            continue;
        }
        let relative = path
            .strip_prefix(root.join("meta"))
            .map_err(|error| error.to_string())?;
        let target = change_path.join(relative);
        let updated = update_frontmatter_reference(&content, old_id, new_id);
        let updated = insert_operation(&updated, "modified", None);
        atomic_write(&target, &updated)?;
    }
    Ok(())
}

fn frontmatter_references(source: &str, id: &str) -> bool {
    let parsed = frontmatter::parse(source);
    parsed.values.values().any(|value| value == id)
        || parsed
            .lists
            .values()
            .any(|values| values.iter().any(|value| value == id))
}

fn update_frontmatter_reference(source: &str, old_id: &str, new_id: &str) -> String {
    let mut in_frontmatter = false;
    let mut seen_start = false;
    let mut output = Vec::new();
    for line in source.lines() {
        if !seen_start && line.trim() == "---" {
            seen_start = true;
            in_frontmatter = true;
            output.push(line.to_owned());
            continue;
        }
        if in_frontmatter && line.trim() == "---" {
            in_frontmatter = false;
            output.push(line.to_owned());
            continue;
        }
        if in_frontmatter {
            output.push(line.replace(old_id, new_id));
        } else {
            output.push(line.to_owned());
        }
    }
    format!("{}\n", output.join("\n"))
}

fn insert_operation(source: &str, operation: &str, renamed_from: Option<&Path>) -> String {
    let mut output = Vec::new();
    let mut inserted = false;
    for line in source.lines() {
        output.push(line.to_owned());
        if !inserted && line.trim() == "---" {
            output.push(format!("operation: {operation}"));
            if let Some(path) = renamed_from {
                output.push(format!("renamed_from: {}", path.display()));
            }
            inserted = true;
        }
    }
    format!("{}\n", output.join("\n"))
}

fn artefact_content_refs(source: &str, ids: &BTreeSet<String>) -> bool {
    let parsed = frontmatter::parse(source);
    parsed.values.values().any(|value| ids.contains(value))
        || parsed
            .lists
            .values()
            .any(|values| values.iter().any(|value| ids.contains(value)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_parse_blueprint_delta_added_round_trip() -> Result<(), Box<dyn std::error::Error>> {
        let delta = parse_blueprint_delta(
            "change.delta",
            r#"## ADDED Nodes
System App "desc" id "app" {
    Module Api "api" id "app.api" {}
}

## ADDED Edges
app.api -> app "reports"
"#,
        )?;

        let rendered = apply_blueprint_delta("", &delta)?;

        assert!(rendered.contains("System App"));
        assert!(rendered.contains("app.api -> app \"reports\""));
        Ok(())
    }

    #[test]
    fn test_parse_blueprint_delta_modified_round_trip() -> Result<(), Box<dyn std::error::Error>> {
        let base = r#"System App "desc" id "app" {
    Module Api "old" id "app.api" {}
}
"#;
        let delta = parse_blueprint_delta(
            "change.delta",
            r#"## MODIFIED Nodes
Module Api "new" id "app.api" {}
"#,
        )?;

        let rendered = apply_blueprint_delta(base, &delta)?;

        assert!(rendered.contains("Module Api \"new\" id \"app.api\""));
        Ok(())
    }

    #[test]
    fn test_parse_blueprint_delta_removed_round_trip() -> Result<(), Box<dyn std::error::Error>> {
        let base = r#"System App "desc" id "app" {
    Module Api "old" id "app.api" {}
}
app.api -> app "reports"
"#;
        let delta = parse_blueprint_delta(
            "change.delta",
            r#"## REMOVED Nodes
- app.api

## REMOVED Edges
app.api -> app "reports"
"#,
        )?;

        let rendered = apply_blueprint_delta(base, &delta)?;

        assert!(!rendered.contains("app.api"));
        Ok(())
    }

    #[test]
    fn test_parse_blueprint_delta_renamed_round_trip() -> Result<(), Box<dyn std::error::Error>> {
        let base = r#"System App "desc" id "app" {
    Module Api "old" id "app.api" {}
}
app.api -> app "reports"
"#;
        let delta = parse_blueprint_delta(
            "change.delta",
            r"## RENAMED Nodes
- app.api -> app.http
",
        )?;

        let rendered = apply_blueprint_delta(base, &delta)?;

        assert!(rendered.contains("id \"app.http\""));
        assert!(rendered.contains("app.http -> app \"reports\""));
        Ok(())
    }

    #[test]
    fn test_validate_change_detects_conflicting_operations()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("conflict")?;
        write_project(&root)?;
        let graph = scanner::load_project(&root, &root.join("cairn.blueprint"))?.graph;
        let change = Change {
            id: "phase-7.5a-test-fortification".to_owned(),
            path: root.join("meta/changes/phase-7.5a-test-fortification"),
            title: "test".to_owned(),
            proposal: String::new(),
            design: None,
            delta: BlueprintDelta {
                modified_nodes: vec![Node {
                    kind: NodeKind::Module,
                    name: "Api".to_owned(),
                    description: "desc".to_owned(),
                    id: "app.api".to_owned(),
                    tags: Vec::new(),
                    paths: Vec::new(),
                    owns_files: false,
                    contracts: Vec::new(),
                    raw_fields: Vec::new(),
                    children: Vec::new(),
                    span: crate::blueprint::Span::point("cairn.blueprint", 1, 1),
                }],
                removed_nodes: vec!["app.api".to_owned()],
                ..BlueprintDelta::default()
            },
            artefacts: Vec::new(),
            findings: Vec::new(),
        };

        let errors = validate_change(&change, &graph);

        assert!(
            errors
                .iter()
                .any(|error| error.contains("conflicting operations"))
        );
        Ok(())
    }

    fn write_project(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(root.join("meta/changes"))?;
        fs::write(
            root.join("cairn.blueprint"),
            r#"System App "desc" id "app" {
    Module Api "desc" id "app.api" {}
}
"#,
        )?;
        fs::write(
            root.join("cairn.config.yaml"),
            "reconcilers:\n  - id: rust-code\n    version: phase-1\n    config:\n      ignore:\n        - target\ncontext: \"\"\nrules: {}\n",
        )?;
        Ok(())
    }

    fn temp_root(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let root = std::env::temp_dir().join(format!("cairn-changes-tests-{name}-{suffix}"));
        fs::create_dir_all(&root)?;
        Ok(root)
    }
}
