use super::*;

pub(super) fn mutation_paths(root: &Path, blueprint_path: &Path, change: &Change) -> Vec<PathBuf> {
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

pub(super) fn apply_archive(
    root: &Path,
    blueprint_path: &Path,
    change: &Change,
) -> Result<(), String> {
    let full_blueprint = root.join(blueprint_path);
    let source = fs::read_to_string(&full_blueprint).map_err(|error| error.to_string())?;
    let next = apply_blueprint_delta(&source, &change.delta)?;
    atomic_write(&full_blueprint, &next)?;
    apply_artefact_operations(&change.artefacts)?;
    Ok(())
}

pub(super) fn apply_blueprint_delta(
    source: &str,
    delta: &BlueprintDelta,
) -> Result<String, String> {
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

pub(super) fn serialize_ast(ast: &Ast) -> String {
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
mod tests {
    use super::*;
    use crate::blueprint::{NodeKind, Span};

    fn span() -> Span {
        Span::point("test", 1, 1)
    }

    fn leaf(id: &str) -> Node {
        Node {
            kind: NodeKind::Module,
            id: id.to_owned(),
            name: id.to_owned(),
            description: String::new(),
            tags: Vec::new(),
            paths: Vec::new(),
            owns_files: false,
            contracts: Vec::new(),
            raw_fields: Vec::new(),
            children: Vec::new(),
            span: span(),
        }
    }

    fn mk_edge(from: &str, to: &str, desc: &str) -> Edge {
        Edge {
            from: from.to_owned(),
            to: to.to_owned(),
            description: desc.to_owned(),
            span: span(),
        }
    }

    // ── same_edge ─────────────────────────────────────────────────────────────

    #[test]
    fn test_same_edge_equal() {
        let e = mk_edge("a", "b", "calls");
        assert!(same_edge(&e, &e.clone()));
    }

    #[test]
    fn test_same_edge_different_description_is_not_equal() {
        let e1 = mk_edge("a", "b", "calls");
        let e2 = mk_edge("a", "b", "queries");
        assert!(!same_edge(&e1, &e2));
    }

    #[test]
    fn test_same_edge_different_from_is_not_equal() {
        assert!(!same_edge(&mk_edge("a", "b", "c"), &mk_edge("x", "b", "c")));
    }

    // ── replace_exact_id ──────────────────────────────────────────────────────

    #[test]
    fn test_replace_exact_id_exact_match() {
        assert_eq!(replace_exact_id("app.api", "app.api", "app.new"), "app.new");
    }

    #[test]
    fn test_replace_exact_id_no_substring_replacement() {
        // "app" must not be replaced inside "app.api".
        assert_eq!(replace_exact_id("app.api", "app", "x"), "app.api");
    }

    #[test]
    fn test_replace_exact_id_no_match() {
        assert_eq!(replace_exact_id("app.api", "other", "x"), "app.api");
    }

    // ── node_kind_name ────────────────────────────────────────────────────────

    #[test]
    fn test_node_kind_name_all_variants() {
        assert_eq!(node_kind_name(NodeKind::System), "System");
        assert_eq!(node_kind_name(NodeKind::Container), "Container");
        assert_eq!(node_kind_name(NodeKind::Module), "Module");
        assert_eq!(node_kind_name(NodeKind::Actor), "Actor");
    }

    // ── serialize_field_values ────────────────────────────────────────────────

    #[test]
    fn test_serialize_field_values_empty_produces_brackets() {
        assert_eq!(serialize_field_values(&[]), "[]");
    }

    #[test]
    fn test_serialize_field_values_single_produces_quoted_string() {
        let v = vec!["src/api".to_owned()];
        assert_eq!(serialize_field_values(&v), r#""src/api""#);
    }

    #[test]
    fn test_serialize_field_values_multiple_produces_bracketed_list() {
        let v = vec!["a".to_owned(), "b".to_owned()];
        assert_eq!(serialize_field_values(&v), r#"["a", "b"]"#);
    }

    // ── serialize_node ────────────────────────────────────────────────────────

    #[test]
    fn test_serialize_node_basic_structure() {
        let n = Node {
            kind: NodeKind::Module,
            id: "app.api".to_owned(),
            name: "Api".to_owned(),
            description: "The API".to_owned(),
            tags: vec!["public".to_owned()],
            ..leaf("app.api")
        };
        let mut out = String::new();
        serialize_node(&n, 0, &mut out);
        assert!(out.contains("Module Api"), "kind and name: {out:?}");
        assert!(out.contains(r#""The API""#), "description quoted: {out:?}");
        assert!(out.contains(r#"id "app.api""#), "id quoted: {out:?}");
        assert!(out.contains("@public"), "tag present: {out:?}");
        assert!(out.starts_with("Module"), "no indent at level 0: {out:?}");
    }

    #[test]
    fn test_serialize_node_with_path_emits_path_line() {
        let n = Node {
            paths: vec!["src/api".to_owned()],
            ..leaf("app.api")
        };
        let mut out = String::new();
        serialize_node(&n, 0, &mut out);
        assert!(out.contains(r#"path "src/api""#), "path line: {out:?}");
    }

    #[test]
    fn test_serialize_node_owns_files_true_emits_flag() {
        let n = Node {
            owns_files: true,
            ..leaf("app.api")
        };
        let mut out = String::new();
        serialize_node(&n, 0, &mut out);
        assert!(out.contains("owns-files: true"), "owns-files flag: {out:?}");
    }

    #[test]
    fn test_serialize_node_owns_files_false_does_not_emit_flag() {
        let mut out = String::new();
        serialize_node(&leaf("app.api"), 0, &mut out);
        assert!(
            !out.contains("owns-files"),
            "no owns-files when false: {out:?}"
        );
    }

    #[test]
    fn test_serialize_node_indent_applied_at_nonzero_level() {
        let mut out = String::new();
        serialize_node(&leaf("a"), 4, &mut out);
        assert!(
            out.starts_with("    "),
            "4-space indent at level 4: {out:?}"
        );
    }

    // ── rename_node_id ────────────────────────────────────────────────────────

    #[test]
    fn test_rename_node_id_renames_top_level_node() {
        let mut nodes = vec![leaf("old")];
        rename_node_id(&mut nodes, "old", "new");
        assert_eq!(nodes[0].id, "new");
    }

    #[test]
    fn test_rename_node_id_renames_nested_child() {
        let child = leaf("child");
        let mut parent = leaf("parent");
        parent.children = vec![child];
        let mut nodes = vec![parent];
        rename_node_id(&mut nodes, "child", "renamed");
        assert_eq!(nodes[0].children[0].id, "renamed");
    }

    #[test]
    fn test_rename_node_id_no_match_is_noop() {
        let mut nodes = vec![leaf("a")];
        rename_node_id(&mut nodes, "missing", "x");
        assert_eq!(nodes[0].id, "a");
    }

    // ── remove_node ───────────────────────────────────────────────────────────

    #[test]
    fn test_remove_node_removes_top_level() {
        let mut nodes = vec![leaf("a"), leaf("b")];
        remove_node(&mut nodes, "a");
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].id, "b");
    }

    #[test]
    fn test_remove_node_removes_nested_child() {
        let mut parent = leaf("parent");
        parent.children = vec![leaf("child")];
        let mut nodes = vec![parent];
        remove_node(&mut nodes, "child");
        assert!(nodes[0].children.is_empty(), "child must be removed");
    }

    #[test]
    fn test_remove_node_no_match_is_noop() {
        let mut nodes = vec![leaf("a")];
        remove_node(&mut nodes, "missing");
        assert_eq!(nodes.len(), 1);
    }

    // ── replace_node ──────────────────────────────────────────────────────────

    #[test]
    fn test_replace_node_replaces_top_level() {
        let replacement = Node {
            description: "replaced".to_owned(),
            ..leaf("a")
        };
        let mut nodes = vec![leaf("a")];
        replace_node(&mut nodes, &replacement).unwrap();
        assert_eq!(nodes[0].description, "replaced");
    }

    #[test]
    fn test_replace_node_replaces_nested_child() {
        let replacement = Node {
            description: "new desc".to_owned(),
            ..leaf("child")
        };
        let mut parent = leaf("parent");
        parent.children = vec![leaf("child")];
        let mut nodes = vec![parent];
        replace_node(&mut nodes, &replacement).unwrap();
        assert_eq!(nodes[0].children[0].description, "new desc");
    }

    #[test]
    fn test_replace_node_not_found_returns_err() {
        let mut nodes = vec![leaf("a")];
        let result = replace_node(&mut nodes, &leaf("missing"));
        assert!(result.is_err(), "not-found must return Err");
        let msg = result.unwrap_err();
        assert!(msg.contains("missing"), "error message names the id: {msg}");
    }

    // ── strip_change_frontmatter ──────────────────────────────────────────────

    #[test]
    fn test_strip_frontmatter_no_frontmatter_passes_through() {
        let src = "# Title\n\nbody text\n";
        let result = strip_change_frontmatter(src);
        assert!(result.contains("# Title"));
        assert!(result.contains("body text"));
    }

    #[test]
    fn test_strip_frontmatter_removes_operation_field() {
        let src = "---\noperation: add\ntitle: foo\n---\nbody\n";
        let result = strip_change_frontmatter(src);
        assert!(
            !result.contains("operation: add"),
            "operation line stripped: {result:?}"
        );
        assert!(
            result.contains("title: foo"),
            "other fields kept: {result:?}"
        );
        assert!(result.contains("body"), "body kept: {result:?}");
    }

    #[test]
    fn test_strip_frontmatter_removes_renamed_from_field() {
        let src = "---\noperation: rename\nrenamed_from: old.md\ntitle: bar\n---\nbody\n";
        let result = strip_change_frontmatter(src);
        assert!(
            !result.contains("renamed_from:"),
            "renamed_from stripped: {result:?}"
        );
        assert!(result.contains("title: bar"), "title kept: {result:?}");
    }

    #[test]
    fn test_strip_frontmatter_operation_in_body_not_stripped() {
        // A body line starting with "operation:" must not be removed.
        // Only lines inside the frontmatter block are candidates for stripping.
        let src = "---\ntitle: foo\n---\noperation: something\n";
        let result = strip_change_frontmatter(src);
        assert!(
            result.contains("operation: something"),
            "body line must not be stripped: {result:?}"
        );
    }

    #[test]
    fn test_strip_frontmatter_always_ends_with_newline() {
        // Input with no trailing newline must still produce one.
        let result = strip_change_frontmatter("no newline at end");
        assert!(result.ends_with('\n'), "must end with newline: {result:?}");
    }
}
