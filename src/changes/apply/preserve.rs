//! Source-preserving blueprint delta application.
//!
//! Re-serialising the whole AST on archive discards every comment and blank
//! line. This module applies a non-empty [`BlueprintDelta`] against the original
//! source text instead: it rewrites only the declarations the delta actually
//! changes and copies every untouched line (comments and blank lines included)
//! through verbatim, at any nesting depth.
//!
//! The splice assumes each declaration occupies its own source lines (the
//! conventional one-per-line layout). To stay correct for any legal blueprint,
//! the spliced output is re-parsed and compared against a canonical mutation of
//! the AST; if they disagree (e.g. the source packed several declarations onto
//! one line) it falls back to canonical serialisation, which is structurally
//! correct but drops trivia.
//!
//! Tradeoffs, by design:
//! - A node that is itself a `modified` target is re-serialised wholesale, so
//!   comments and blanks *inside* that one declaration are not retained.
//! - A removed node's own preceding/internal trivia is removed with it; a
//!   comment that sat above it is left in place (and may read as orphaned).
//! - Added nodes and edges are appended in canonical serialised form, since new
//!   declarations carry no source trivia of their own.

use std::{collections::BTreeMap, fmt::Write as _};

use crate::blueprint::{
    Ast, Edge, Node,
    lexer::{TokenKind, tokenize},
    parser::parse_str,
};
use crate::changes::BlueprintDelta;

use super::{
    remove_node, rename_node_id, replace_exact_id, replace_node, same_edge, serialize_node,
};

/// The inclusive source line range a node declaration occupies, plus the same
/// for each of its children. Mirrors the AST tree positionally.
struct Layout {
    start: usize,
    end: usize,
    children: Vec<Self>,
}

/// Applies a [`BlueprintDelta`] to `source`, preserving untouched lines verbatim.
///
/// # Errors
///
/// Returns a human-readable error when `source` is not a parseable blueprint.
pub(super) fn apply_delta_preserving(
    source: &str,
    delta: &BlueprintDelta,
) -> Result<String, String> {
    let ast = parse_str("cairn.blueprint", source).map_err(|error| error.to_string())?;
    let canonical = serialize_canonical(&mutate_ast(ast.clone(), delta)?);
    let spliced = splice(source, &ast, delta);
    // The splice is only valid when each declaration owns its own lines. Verify
    // by re-parsing and comparing structure; fall back to canonical otherwise.
    if let Ok(reparsed) = parse_str("cairn.blueprint", &spliced)
        && serialize_canonical(&reparsed) == canonical
    {
        return Ok(spliced);
    }
    Ok(canonical)
}

/// Rewrites `source` line by line, replacing only the declarations the delta
/// changes. Correct only when each declaration occupies its own source lines;
/// [`apply_delta_preserving`] verifies the result before trusting it.
fn splice(source: &str, ast: &Ast, delta: &BlueprintDelta) -> String {
    let ends = close_lines(source);
    let mut next = 0;
    let layouts = build_layout(&ast.nodes, &ends, &mut next);
    let lines: Vec<&str> = source.split_inclusive('\n').collect();
    let edges: BTreeMap<usize, &Edge> = ast.edges.iter().map(|e| (e.span.line, e)).collect();

    let mut out = String::with_capacity(source.len() + 64);
    let total = lines.len();
    let mut node_index = 0;
    let mut line = 1;
    while line <= total {
        if node_index < layouts.len() && layouts[node_index].start == line {
            render_node(
                &ast.nodes[node_index],
                &layouts[node_index],
                &lines,
                delta,
                0,
                &mut out,
            );
            line = layouts[node_index].end + 1;
            node_index += 1;
            continue;
        }
        if let Some(edge) = edges.get(&line) {
            render_edge(edge, delta, lines[line - 1], &mut out);
            line += 1;
            continue;
        }
        out.push_str(lines[line - 1]);
        line += 1;
    }

    append_additions(&mut out, delta);
    out
}

/// Applies the delta directly to the parsed AST. This is the structural oracle
/// the splice is checked against, and the source for the canonical fallback.
fn mutate_ast(ast: Ast, delta: &BlueprintDelta) -> Result<Ast, String> {
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
    Ok(Ast { nodes, edges })
}

/// Serialises an AST canonically, discarding source trivia.
fn serialize_canonical(ast: &Ast) -> String {
    let mut out = String::new();
    for node in &ast.nodes {
        serialize_node(node, 0, &mut out);
    }
    for edge in &ast.edges {
        write_edge(&mut out, edge);
    }
    out
}

/// Returns the close-brace line of every node, indexed by node preorder.
///
/// The k-th `{` token in source order opens the k-th node visited in preorder,
/// so a brace stack pairs each opener with its closer without re-parsing.
fn close_lines(source: &str) -> Vec<usize> {
    let Ok(tokens) = tokenize("cairn.blueprint", source) else {
        return Vec::new();
    };
    let mut ends: Vec<usize> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    for token in &tokens {
        match token.kind {
            TokenKind::OpenBrace => {
                let index = ends.len();
                ends.push(0);
                stack.push(index);
            }
            TokenKind::CloseBrace => {
                if let Some(index) = stack.pop() {
                    ends[index] = token.span.line;
                }
            }
            _ => {}
        }
    }
    ends
}

/// Walks the AST in preorder, pairing each node with its close-brace line.
fn build_layout(nodes: &[Node], ends: &[usize], next: &mut usize) -> Vec<Layout> {
    nodes
        .iter()
        .map(|node| {
            let index = *next;
            *next += 1;
            let children = build_layout(&node.children, ends, next);
            Layout {
                start: node.span.line,
                end: ends.get(index).copied().unwrap_or(node.span.line),
                children,
            }
        })
        .collect()
}

/// Emits one node's post-delta source into `out`.
fn render_node(
    node: &Node,
    layout: &Layout,
    lines: &[&str],
    delta: &BlueprintDelta,
    indent: usize,
    out: &mut String,
) {
    let Some(mutated) = transform(node, delta) else {
        return; // removed: emit nothing
    };
    if mutated == *node {
        copy_lines(lines, layout.start, layout.end, out);
        return;
    }
    if delta.modified_nodes.contains(&mutated) {
        // The node itself was replaced wholesale; emit the canonical form.
        serialize_node(&mutated, indent, out);
        return;
    }
    // Renamed and/or a descendant changed: keep this node's own trivia, recurse
    // into children, and patch the id token when the node was renamed.
    render_recurse(node, &mutated, layout, lines, delta, indent, out);
}

/// Applies the delta's node operations to a single subtree, mirroring the order
/// used by the whole-graph path. Returns `None` when the root was removed.
fn transform(node: &Node, delta: &BlueprintDelta) -> Option<Node> {
    let mut sub = vec![node.clone()];
    for rename in &delta.renamed_nodes {
        rename_node_id(&mut sub, &rename.from, &rename.to);
    }
    for id in &delta.removed_nodes {
        remove_node(&mut sub, id);
    }
    for modified in &delta.modified_nodes {
        let _ = replace_node(&mut sub, modified);
    }
    sub.into_iter().next()
}

/// Re-emits a node that survives structurally, preserving its inner trivia and
/// recursing into changed children.
fn render_recurse(
    node: &Node,
    mutated: &Node,
    layout: &Layout,
    lines: &[&str],
    delta: &BlueprintDelta,
    indent: usize,
    out: &mut String,
) {
    let renamed = node.id != mutated.id;
    let mut id_patched = false;
    let mut children = node.children.iter().zip(&layout.children);
    let mut pending = children.next();
    let mut line = layout.start;
    while line <= layout.end {
        if let Some((child, child_layout)) = pending
            && child_layout.start == line
        {
            render_node(child, child_layout, lines, delta, indent + 4, out);
            line = child_layout.end + 1;
            pending = children.next();
            continue;
        }
        let text = lines[line - 1];
        if renamed
            && !id_patched
            && let Some(patched) = patch_id(text, &node.id, &mutated.id)
        {
            out.push_str(&patched);
            id_patched = true;
            line += 1;
            continue;
        }
        out.push_str(text);
        line += 1;
    }
}

/// Rewrites an `id "<old>"` token to `id "<new>"` on the first line carrying it.
fn patch_id(line: &str, old: &str, new: &str) -> Option<String> {
    let needle = format!("id \"{old}\"");
    let pos = line.find(&needle)?;
    let mut patched = String::with_capacity(line.len() + new.len());
    patched.push_str(&line[..pos]);
    let _ = write!(patched, "id \"{new}\"");
    patched.push_str(&line[pos + needle.len()..]);
    Some(patched)
}

/// Emits a top-level edge's post-delta form, keeping untouched edges verbatim.
fn render_edge(edge: &Edge, delta: &BlueprintDelta, original: &str, out: &mut String) {
    let mut from = edge.from.clone();
    let mut to = edge.to.clone();
    for rename in &delta.renamed_nodes {
        from = replace_exact_id(&from, &rename.from, &rename.to);
        to = replace_exact_id(&to, &rename.from, &rename.to);
    }
    let renamed = Edge {
        from,
        to,
        description: edge.description.clone(),
        span: edge.span.clone(),
    };
    let removed = delta.removed_edges.iter().any(|e| same_edge(&renamed, e))
        || delta
            .renamed_edges
            .iter()
            .any(|rename| same_edge(&renamed, &rename.from))
        || delta
            .modified_edges
            .iter()
            .any(|modified| modified.from == renamed.from && modified.to == renamed.to);
    if removed {
        return; // the replacement edge, if any, is appended at the end
    }
    if renamed.from == edge.from && renamed.to == edge.to {
        out.push_str(original);
    } else {
        write_edge(out, &renamed);
    }
}

/// Appends added nodes and added/replaced edges in canonical serialised form.
fn append_additions(out: &mut String, delta: &BlueprintDelta) {
    let has_additions = !delta.added_nodes.is_empty()
        || !delta.added_edges.is_empty()
        || !delta.renamed_edges.is_empty()
        || !delta.modified_edges.is_empty();
    if !has_additions {
        return;
    }
    if !out.is_empty() && !out.ends_with('\n') {
        out.push('\n');
    }
    for node in &delta.added_nodes {
        serialize_node(node, 0, out);
    }
    for rename in &delta.renamed_edges {
        write_edge(out, &rename.to);
    }
    for edge in &delta.modified_edges {
        write_edge(out, edge);
    }
    for edge in &delta.added_edges {
        write_edge(out, edge);
    }
}

fn write_edge(out: &mut String, edge: &Edge) {
    let _ = writeln!(out, "{} -> {} {:?}", edge.from, edge.to, edge.description);
}

/// Copies the inclusive source line range `[start, end]` into `out` verbatim.
fn copy_lines(lines: &[&str], start: usize, end: usize, out: &mut String) {
    for line in &lines[start - 1..end] {
        out.push_str(line);
    }
}
