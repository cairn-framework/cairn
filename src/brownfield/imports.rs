//! Import extraction and edge derivation for brownfield discovery.
//!
//! Parses discovered candidate source files with the same tree-sitter
//! grammars the reconcilers use, extracts the module references each
//! file imports, and maps them onto co-discovered candidates to emit
//! directed, code-evidenced dependency edges.

use std::collections::BTreeMap;
use std::path::Path;

use crate::reconcile::{LanguageSpec, code::RUST, go::GO, python::PYTHON, typescript::TYPESCRIPT};

use super::discovery::{DiscoveredCandidate, DiscoveredEdge};

/// A single import reference observed in a source file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImportRef {
    /// A relative file reference (JS/TS `./x`, `../y`): resolved against
    /// the importing file's directory.
    Relative(String),
    /// A module path as ordered segments (`crate::db` -> `["db"]`,
    /// `from api.routes import x` -> `["api", "routes"]`).
    Segments(Vec<String>),
}

/// tree-sitter node kinds treated as import declarations per language.
fn import_kinds(spec: &'static LanguageSpec) -> &'static [&'static str] {
    match spec.language {
        crate::reconcile::target::Language::Rust => &["use_declaration"],
        crate::reconcile::target::Language::TypeScript => &["import_statement"],
        crate::reconcile::target::Language::Python => {
            &["import_statement", "import_from_statement"]
        }
        crate::reconcile::target::Language::Go => &["import_declaration"],
        crate::reconcile::target::Language::Unknown => &[],
    }
}

fn spec_for_extension(ext: &str) -> Option<&'static LanguageSpec> {
    // Discovery also walks plain .js files; the TypeScript grammar parses
    // JavaScript import statements, so route them to the same spec.
    if ext == "js" {
        return Some(&TYPESCRIPT);
    }
    [&RUST, &TYPESCRIPT, &PYTHON, &GO]
        .into_iter()
        .find(|spec| spec.extensions.contains(&ext))
}
/// Extract import references from one source file's contents.
///
/// Files whose extension no grammar owns, or that fail to parse, yield no
/// references: discovery never fabricates an edge it cannot observe.
#[must_use]
pub fn extract_imports(path: &Path, source: &str) -> Vec<ImportRef> {
    let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
        return Vec::new();
    };
    let Some(spec) = spec_for_extension(ext) else {
        return Vec::new();
    };
    let mut parser = tree_sitter::Parser::new();
    if parser.set_language(&(spec.grammar)()).is_err() {
        return Vec::new();
    }
    let Some(tree) = parser.parse(source, None) else {
        return Vec::new();
    };
    let kinds = import_kinds(spec);
    let mut refs = Vec::new();
    let mut cursor = tree.walk();
    collect_import_nodes(&mut cursor, source.as_bytes(), kinds, spec, &mut refs);
    refs
}

fn collect_import_nodes(
    cursor: &mut tree_sitter::TreeCursor<'_>,
    source: &[u8],
    kinds: &[&str],
    spec: &'static LanguageSpec,
    refs: &mut Vec<ImportRef>,
) {
    loop {
        let node = cursor.node();
        if kinds.contains(&node.kind()) {
            if let Ok(text) = node.utf8_text(source) {
                normalise(spec, text, refs);
            }
        } else if cursor.goto_first_child() {
            collect_import_nodes(cursor, source, kinds, spec, refs);
            cursor.goto_parent();
        }
        if !cursor.goto_next_sibling() {
            break;
        }
    }
}

/// Normalise one import declaration's text into references.
fn normalise(spec: &'static LanguageSpec, text: &str, refs: &mut Vec<ImportRef>) {
    match spec.language {
        crate::reconcile::target::Language::Rust => normalise_rust(text, refs),
        crate::reconcile::target::Language::TypeScript => normalise_ts(text, refs),
        crate::reconcile::target::Language::Python => normalise_python(text, refs),
        crate::reconcile::target::Language::Go => normalise_go(text, refs),
        crate::reconcile::target::Language::Unknown => {}
    }
}

/// `use crate::db;`, `use crate::{api, auth::tokens};`, `use super::x;`.
fn normalise_rust(text: &str, refs: &mut Vec<ImportRef>) {
    let body = text
        .trim()
        .trim_start_matches("pub")
        .trim()
        .trim_start_matches("use")
        .trim()
        .trim_end_matches(';');
    for path in split_rust_paths(body) {
        let segments: Vec<String> = path
            .split("::")
            .map(str::trim)
            .filter(|s| !s.is_empty() && *s != "crate" && *s != "self" && *s != "super")
            .map(|s| {
                s.split_whitespace()
                    .next()
                    .unwrap_or(s)
                    .trim_matches('*')
                    .to_owned()
            })
            .filter(|s| !s.is_empty())
            .collect();
        if !segments.is_empty() {
            refs.push(ImportRef::Segments(segments));
        }
    }
}

/// Expand `a::{b, c::d}` group syntax into flat paths.
fn split_rust_paths(body: &str) -> Vec<String> {
    match body.find('{') {
        None => vec![body.to_owned()],
        Some(open) => {
            let prefix = body[..open].trim_end_matches("::").trim();
            let inner = body[open + 1..].trim_end_matches('}');
            split_top_level(inner)
                .into_iter()
                .flat_map(|item| split_rust_paths(item.trim()))
                .map(|item| {
                    if prefix.is_empty() {
                        item
                    } else {
                        format!("{prefix}::{item}")
                    }
                })
                .collect()
        }
    }
}

/// Split on commas not nested inside braces.
fn split_top_level(s: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0usize;
    let mut start = 0;
    for (i, c) in s.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                parts.push(&s[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    parts.push(&s[start..]);
    parts
}

/// `import { x } from "./auth";` / `import db from "../db/pool";`.
fn normalise_ts(text: &str, refs: &mut Vec<ImportRef>) {
    let Some(source) = quoted_value(text) else {
        return;
    };
    if source.starts_with('.') {
        refs.push(ImportRef::Relative(source));
    } else {
        let segments: Vec<String> = source.split('/').map(str::to_owned).collect();
        refs.push(ImportRef::Segments(segments));
    }
}

/// `import a.b as c` / `from a.b import c, d`.
fn normalise_python(text: &str, refs: &mut Vec<ImportRef>) {
    let trimmed = text.trim();
    if let Some(rest) = trimmed.strip_prefix("from ") {
        let module = rest.split(" import").next().unwrap_or("").trim();
        push_python_module(module, refs);
    } else if let Some(rest) = trimmed.strip_prefix("import ") {
        for module in rest.split(',') {
            let module = module.split(" as ").next().unwrap_or("").trim();
            push_python_module(module, refs);
        }
    }
}

fn push_python_module(module: &str, refs: &mut Vec<ImportRef>) {
    let segments: Vec<String> = module
        .split('.')
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect();
    if !segments.is_empty() {
        refs.push(ImportRef::Segments(segments));
    }
}

/// `import "app/db"` / `import ( a "app/auth"\n "app/db" )`.
fn normalise_go(text: &str, refs: &mut Vec<ImportRef>) {
    for line in text.lines() {
        if let Some(source) = quoted_value(line) {
            let segments: Vec<String> = source.split('/').map(str::to_owned).collect();
            refs.push(ImportRef::Segments(segments));
        }
    }
}

/// First double- or single-quoted string in `text`, unquoted.
fn quoted_value(text: &str) -> Option<String> {
    for quote in ['"', '\''] {
        if let Some(start) = text.find(quote)
            && let Some(len) = text[start + 1..].find(quote)
        {
            return Some(text[start + 1..start + 1 + len].to_owned());
        }
    }
    None
}
/// Populate `candidates[i].edges` from imports observed in evidence files.
///
/// An import maps to a candidate either by resolving a relative reference
/// (JS/TS `./x`) to a repo-relative path under the candidate's directory,
/// or by matching a path segment against a candidate's directory name when
/// exactly one candidate carries that name. Edge confidence scales with
/// the number of importing files.
pub(super) fn derive_import_edges(root: &Path, candidates: &mut [DiscoveredCandidate]) {
    let dir_names: Vec<String> = candidates.iter().map(|c| name_key(&c.path)).collect();
    let mut counts: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    for (i, candidate) in candidates.iter().enumerate() {
        for file in &candidate.evidence {
            let Ok(source) = std::fs::read_to_string(root.join(file)) else {
                continue;
            };
            for import in extract_imports(Path::new(file), &source) {
                if let Some(j) = match_candidate(&import, file, candidates, &dir_names)
                    && j != i
                {
                    *counts.entry((i, j)).or_default() += 1;
                }
            }
        }
    }
    for ((i, j), count) in counts {
        let target = candidates[j].id.clone();
        let name = candidates[j].name.clone();
        candidates[i].edges.push(DiscoveredEdge {
            target,
            description: format!("Observed imports of {name} ({count} in code)"),
            confidence: edge_confidence(count),
        });
    }
}

/// Last path component of a candidate directory, the import-name key.
fn name_key(path: &str) -> String {
    path.rsplit(['/', '\\']).next().unwrap_or(path).to_owned()
}

/// Resolve one import reference to a candidate index, or `None` when the
/// reference is external, self-referential, or ambiguous.
fn match_candidate(
    import: &ImportRef,
    importing_file: &str,
    candidates: &[DiscoveredCandidate],
    dir_names: &[String],
) -> Option<usize> {
    match import {
        ImportRef::Relative(rel) => {
            let base = Path::new(importing_file).parent()?;
            let resolved = resolve_relative(base, rel)?;
            candidates
                .iter()
                .position(|c| resolved == c.path || resolved.starts_with(&format!("{}/", c.path)))
        }
        ImportRef::Segments(segments) => {
            for segment in segments {
                let mut hits = dir_names.iter().enumerate().filter(|(_, n)| *n == segment);
                if let Some((idx, _)) = hits.next() {
                    // Ambiguous names (two candidates share a directory
                    // name) are skipped rather than guessed.
                    if hits.next().is_none() {
                        return Some(idx);
                    }
                    return None;
                }
            }
            None
        }
    }
}

/// Lexically resolve `./`/`../` against `base`, repo-relative.
fn resolve_relative(base: &Path, rel: &str) -> Option<String> {
    let mut parts: Vec<&str> = base
        .to_str()?
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();
    for seg in rel.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            other => parts.push(other),
        }
    }
    Some(parts.join("/"))
}

fn edge_confidence(count: usize) -> f64 {
    if count >= 5 {
        0.95
    } else if count >= 2 {
        0.8
    } else {
        0.6
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn segs(refs: &[ImportRef]) -> Vec<Vec<String>> {
        refs.iter()
            .filter_map(|r| match r {
                ImportRef::Segments(s) => Some(s.clone()),
                ImportRef::Relative(_) => None,
            })
            .collect()
    }

    #[test]
    fn rust_use_crate_paths() {
        let refs = extract_imports(
            Path::new("mod.rs"),
            "use crate::db;\nuse crate::{api, auth::tokens};\npub fn f() {}\n",
        );
        assert_eq!(
            segs(&refs),
            vec![
                vec!["db".to_owned()],
                vec!["api".to_owned()],
                vec!["auth".to_owned(), "tokens".to_owned()],
            ]
        );
    }

    #[test]
    fn rust_ignores_comments_and_strings() {
        let refs = extract_imports(
            Path::new("a.rs"),
            "// use crate::db;\nfn f() { let _ = \"use crate::api;\"; }\n",
        );
        assert!(refs.is_empty());
    }

    #[test]
    fn ts_relative_and_bare_imports() {
        let refs = extract_imports(
            Path::new("a.ts"),
            "import { login } from \"./auth\";\nimport fs from \"node:fs\";\n",
        );
        assert_eq!(refs[0], ImportRef::Relative("./auth".to_owned()));
        assert_eq!(refs[1], ImportRef::Segments(vec!["node:fs".to_owned()]));
    }

    #[test]
    fn python_import_forms() {
        let refs = extract_imports(
            Path::new("a.py"),
            "import auth.tokens as t\nfrom db import pool\n",
        );
        assert_eq!(
            segs(&refs),
            vec![
                vec!["auth".to_owned(), "tokens".to_owned()],
                vec!["db".to_owned()],
            ]
        );
    }

    #[test]
    fn go_grouped_imports() {
        let refs = extract_imports(
            Path::new("a.go"),
            "package api\n\nimport (\n\ta \"app/auth\"\n\t\"app/db\"\n)\n",
        );
        assert_eq!(
            segs(&refs),
            vec![
                vec!["app".to_owned(), "auth".to_owned()],
                vec!["app".to_owned(), "db".to_owned()],
            ]
        );
    }

    #[test]
    fn unknown_extension_yields_nothing() {
        assert!(extract_imports(Path::new("a.md"), "import x\n").is_empty());
    }
}
