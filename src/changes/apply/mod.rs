//! Apply a change to the blueprint and artefacts.

use super::*;

mod preserve;

pub(super) fn mutation_paths(root: &Path, blueprint_path: &Path, change: &Change) -> Vec<PathBuf> {
    let mut paths = vec![blueprint_path.to_path_buf()];
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

pub(super) fn snapshot_paths(paths: &[PathBuf]) -> io::Result<Vec<Snapshot>> {
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

pub(super) fn restore_snapshots(snapshots: &[Snapshot]) -> io::Result<()> {
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

pub(super) fn apply_archive(blueprint_path: &Path, change: &Change) -> Result<(), String> {
    // An empty delta carries no blueprint operations, so re-serialising and
    // rewriting the file would only strip comments and formatting for no gain.
    // Skip the blueprint mutation entirely and leave the file byte-identical.
    if !change.delta.is_empty() {
        let source = fs::read_to_string(blueprint_path).map_err(|error| error.to_string())?;
        let next = apply_blueprint_delta(&source, &change.delta)?;
        atomic_write(blueprint_path, &next)?;
    }
    apply_artefact_operations(&change.artefacts)?;
    Ok(())
}

pub(super) fn apply_blueprint_delta(
    source: &str,
    delta: &BlueprintDelta,
) -> Result<String, String> {
    preserve::apply_delta_preserving(source, delta)
}

pub(super) fn rename_node_id(nodes: &mut [Node], from: &str, to: &str) {
    for node in nodes {
        if node.id == from {
            to.clone_into(&mut node.id);
        }
        rename_node_id(&mut node.children, from, to);
    }
}

pub(super) fn remove_node(nodes: &mut Vec<Node>, id: &str) {
    nodes.retain(|node| node.id != id);
    for node in nodes {
        remove_node(&mut node.children, id);
    }
}

pub(super) fn replace_node(nodes: &mut [Node], replacement: &Node) -> Result<(), String> {
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

pub(super) fn same_edge(left: &Edge, right: &Edge) -> bool {
    left.from == right.from && left.to == right.to && left.description == right.description
}

pub(super) fn serialize_node(node: &Node, indent: usize, output: &mut String) {
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

pub(super) fn node_kind_name(kind: NodeKind) -> &'static str {
    match kind {
        NodeKind::System => "System",
        NodeKind::Container => "Container",
        NodeKind::Module => "Module",
        NodeKind::Actor => "Actor",
    }
}

pub(super) fn serialize_field_values(values: &[String]) -> String {
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

pub(super) fn replace_exact_id(value: &str, old_id: &str, new_id: &str) -> String {
    if value == old_id {
        new_id.to_owned()
    } else {
        value.to_owned()
    }
}

pub(super) fn apply_artefact_operations(artefacts: &[ArtefactOperation]) -> Result<(), String> {
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

pub(super) fn write_artefact_target(artefact: &ArtefactOperation) -> Result<(), String> {
    let content = strip_change_frontmatter(&artefact.content);
    atomic_write(&artefact.target_path, &content)
}

pub(super) fn strip_change_frontmatter(source: &str) -> String {
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

pub(super) fn archive_path(root: &Path, change_id: &str) -> PathBuf {
    root.join("meta/changes/archive")
        .join(format!("{}-{change_id}", today_utc()))
}

pub(super) fn today_utc() -> String {
    std::process::Command::new("date")
        .args(["-u", "+%F"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "1970-01-01".to_owned())
}

pub(super) fn append_archive_log(root: &Path, change: &Change) -> Result<(), String> {
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

pub(super) fn atomic_write(path: &Path, content: &str) -> Result<(), String> {
    atomic_write_bytes(path, content.as_bytes()).map_err(|error| error.to_string())
}

pub(super) fn atomic_write_bytes(path: &Path, content: &[u8]) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("tmp-cairn-write");
    fs::write(&tmp, content)?;
    fs::rename(tmp, path)
}

#[cfg(test)]
mod tests;
