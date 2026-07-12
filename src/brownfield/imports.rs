//! Import extraction for brownfield discovery edges.
//!
//! Parses discovered candidate source files with the same tree-sitter
//! grammars the reconcilers use and resolves the module references each
//! file imports to repository-relative path guesses. `super::import_edges`
//! maps those references onto co-discovered candidates.

use std::path::Path;

use crate::reconcile::{
    LanguageSpec, code::RUST, go::GO, python::PYTHON, target::Language, typescript::TYPESCRIPT,
};

/// A resolved import reference observed in a source file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImportRef {
    /// A repository-relative path guess (`crate::db` in `src/auth/mod.rs`
    /// -> `src/db`; `../auth/session` in `src/api/index.ts` ->
    /// `src/auth/session`). Matches a candidate exactly or by directory
    /// prefix, most specific candidate first.
    Path(String),
    /// An ordered module path whose repository anchor is unknown (Go
    /// `import "app/auth"`). Matches a candidate whose full path is a
    /// suffix of the segments.
    Suffix(Vec<String>),
}

/// tree-sitter node kinds treated as import declarations per language.
fn import_kinds(language: Language) -> &'static [&'static str] {
    match language {
        Language::Rust => &["use_declaration"],
        // `export ... from "./x"` reads another module just like an import.
        Language::TypeScript => &["import_statement", "export_statement"],
        Language::Python => &["import_statement", "import_from_statement"],
        Language::Go => &["import_declaration"],
        Language::Unknown => &[],
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

/// Extract resolved import references from one source file.
///
/// `file` is the file's repository-relative path; resolution of relative
/// and crate-rooted references anchors on it. Files whose extension no
/// grammar owns, or that fail to parse, yield no references: discovery
/// never fabricates an edge it cannot observe.
#[must_use]
pub fn extract_imports(file: &Path, source: &str) -> Vec<ImportRef> {
    let Some(ext) = file.extension().and_then(|e| e.to_str()) else {
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
    let file_dir =
        normalise_separators(&file.parent().map(Path::to_string_lossy).unwrap_or_default());
    let kinds = import_kinds(spec.language);
    let mut refs = Vec::new();
    let mut cursor = tree.walk();
    collect(
        &mut cursor,
        source.as_bytes(),
        kinds,
        spec.language,
        &file_dir,
        &mut refs,
    );
    refs
}

fn collect(
    cursor: &mut tree_sitter::TreeCursor<'_>,
    source: &[u8],
    kinds: &[&str],
    language: Language,
    file_dir: &str,
    refs: &mut Vec<ImportRef>,
) {
    loop {
        let node = cursor.node();
        if kinds.contains(&node.kind()) {
            resolve_node(language, node, source, file_dir, refs);
        } else if cursor.goto_first_child() {
            collect(cursor, source, kinds, language, file_dir, refs);
            cursor.goto_parent();
        }
        if !cursor.goto_next_sibling() {
            break;
        }
    }
}

/// Resolve one import declaration node into repository-anchored references.
fn resolve_node(
    language: Language,
    node: tree_sitter::Node<'_>,
    source: &[u8],
    file_dir: &str,
    refs: &mut Vec<ImportRef>,
) {
    match language {
        Language::Rust => {
            if let Ok(text) = node.utf8_text(source) {
                resolve_rust(text, file_dir, refs);
            }
        }
        Language::TypeScript => resolve_ts(node, source, file_dir, refs),
        Language::Python => {
            if let Ok(text) = node.utf8_text(source) {
                resolve_python(text, file_dir, refs);
            }
        }
        Language::Go => resolve_go(node, source, refs),
        Language::Unknown => {}
    }
}

/// `use crate::db;`, `use crate::{api, auth::tokens};`, `use super::x;`,
/// `use self::y;`. Bare paths (`use serde::x`) are external crates and are
/// skipped: they cannot resolve inside the repository.
fn resolve_rust(text: &str, file_dir: &str, refs: &mut Vec<ImportRef>) {
    let body = text
        .trim()
        .trim_start_matches("pub")
        .trim()
        .trim_start_matches("use")
        .trim()
        .trim_end_matches(';');
    for path in split_rust_paths(body) {
        let mut segments = path.split("::").map(str::trim).peekable();
        let base = match segments.peek().copied() {
            Some("crate") => {
                segments.next();
                rust_crate_root(file_dir)
            }
            Some("self") => {
                segments.next();
                file_dir.to_owned()
            }
            Some("super") => {
                segments.next();
                parent_dir(file_dir)
            }
            _ => continue, // external crate or malformed
        };
        let rest: Vec<String> = segments
            .map(|s| {
                s.split_whitespace()
                    .next()
                    .unwrap_or(s)
                    .trim_matches('*')
                    .to_owned()
            })
            .filter(|s| !s.is_empty())
            .collect();
        if !rest.is_empty() {
            refs.push(ImportRef::Path(join_path(&base, &rest)));
        }
    }
}

/// Crate root for a file: the path up to and including its `src`
/// component, or the repository root when the file lives outside one.
fn rust_crate_root(file_dir: &str) -> String {
    let parts: Vec<&str> = file_dir.split('/').filter(|s| !s.is_empty()).collect();
    match parts.iter().position(|p| *p == "src") {
        Some(i) => parts[..=i].join("/"),
        None => String::new(),
    }
}

fn parent_dir(dir: &str) -> String {
    match dir.rsplit_once('/') {
        Some((parent, _)) => parent.to_owned(),
        None => String::new(),
    }
}

fn join_path(base: &str, segments: &[String]) -> String {
    if base.is_empty() {
        segments.join("/")
    } else {
        format!("{base}/{}", segments.join("/"))
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

/// `import { x } from "./auth"` / `export * from "../db"`. The module is
/// read from the node's `source` field, never scanned from raw text, so
/// import attributes and comments cannot fabricate a reference. Bare
/// specifiers (`react`, `node:fs`) are package imports and are skipped.
fn resolve_ts(
    node: tree_sitter::Node<'_>,
    source: &[u8],
    file_dir: &str,
    refs: &mut Vec<ImportRef>,
) {
    let Some(source_node) = node.child_by_field_name("source") else {
        return; // plain `export { x }` has no module source
    };
    let Ok(raw) = source_node.utf8_text(source) else {
        return;
    };
    let module = raw.trim_matches(['"', '\'']);
    if module.starts_with('.')
        && let Some(resolved) = resolve_relative(file_dir, module)
    {
        refs.push(ImportRef::Path(resolved));
    }
}

/// `import a.b as c` / `from a.b import c` / `from . import auth` /
/// `from ..auth import X`. Dotted-relative forms resolve against the
/// importing file's package; absolute forms anchor at the repository root.
fn resolve_python(text: &str, file_dir: &str, refs: &mut Vec<ImportRef>) {
    let trimmed = text.trim();
    if let Some(rest) = trimmed.strip_prefix("from ") {
        let Some((module, imported)) = rest.split_once(" import ") else {
            return;
        };
        let module = module.trim();
        let dots = module.chars().take_while(|c| *c == '.').count();
        if dots > 0 {
            // Relative: one dot is the current package, each further dot
            // one level up. `from . import auth` names the sibling in the
            // import list itself.
            let mut base = file_dir.to_owned();
            for _ in 1..dots {
                base = parent_dir(&base);
            }
            let module_rest = &module[dots..];
            if module_rest.is_empty() {
                for name in imported.split(',') {
                    let name = name.split(" as ").next().unwrap_or("").trim();
                    if !name.is_empty() {
                        refs.push(ImportRef::Path(join_path(&base, &[name.to_owned()])));
                    }
                }
            } else {
                let segs: Vec<String> = module_rest.split('.').map(str::to_owned).collect();
                refs.push(ImportRef::Path(join_path(&base, &segs)));
            }
        } else {
            push_python_absolute(module, refs);
        }
    } else if let Some(rest) = trimmed.strip_prefix("import ") {
        for module in rest.split(',') {
            let module = module.split(" as ").next().unwrap_or("").trim();
            push_python_absolute(module, refs);
        }
    }
}

fn push_python_absolute(module: &str, refs: &mut Vec<ImportRef>) {
    let segments: Vec<String> = module
        .split('.')
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect();
    if !segments.is_empty() {
        refs.push(ImportRef::Path(segments.join("/")));
    }
}

/// `import "app/db"` / grouped import blocks. Paths are read from each
/// `import_spec`'s string node. A first segment containing a dot is a
/// domain (external module) and is skipped.
fn resolve_go(node: tree_sitter::Node<'_>, source: &[u8], refs: &mut Vec<ImportRef>) {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        let spec = match child.kind() {
            "import_spec" => child,
            "import_spec_list" => {
                let mut inner = child.walk();
                for spec in child.named_children(&mut inner) {
                    if spec.kind() == "import_spec" {
                        push_go_spec(spec, source, refs);
                    }
                }
                continue;
            }
            _ => continue,
        };
        push_go_spec(spec, source, refs);
    }
}

fn push_go_spec(spec: tree_sitter::Node<'_>, source: &[u8], refs: &mut Vec<ImportRef>) {
    let Some(path_node) = spec.child_by_field_name("path") else {
        return;
    };
    let Ok(raw) = path_node.utf8_text(source) else {
        return;
    };
    let module = raw.trim_matches('"');
    let segments: Vec<String> = module.split('/').map(str::to_owned).collect();
    match segments.first() {
        Some(first) if !first.contains('.') && !first.is_empty() => {
            refs.push(ImportRef::Suffix(segments));
        }
        _ => {}
    }
}
/// Lexically resolve `./`/`../` against `base`, repository-relative.
/// Returns `None` when the reference escapes the repository root.
fn resolve_relative(base: &str, rel: &str) -> Option<String> {
    let mut parts: Vec<&str> = base.split('/').filter(|s| !s.is_empty()).collect();
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

/// Windows evidence and candidate paths carry backslashes; comparisons
/// are slash-normalised throughout.
pub(super) fn normalise_separators(path: &str) -> String {
    path.replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paths(refs: &[ImportRef]) -> Vec<String> {
        refs.iter()
            .filter_map(|r| match r {
                ImportRef::Path(p) => Some(p.clone()),
                ImportRef::Suffix(_) => None,
            })
            .collect()
    }

    #[test]
    fn rust_crate_paths_resolve_from_src_root() {
        let refs = extract_imports(
            Path::new("src/auth/mod.rs"),
            "use crate::db;\nuse crate::{api, auth::tokens};\nuse super::util;\npub fn f() {}\n",
        );
        assert_eq!(
            paths(&refs),
            vec!["src/db", "src/api", "src/auth/tokens", "src/util"]
        );
    }

    #[test]
    fn rust_external_crates_are_skipped() {
        let refs = extract_imports(
            Path::new("src/a.rs"),
            "use serde::Deserialize;\nuse std::path::Path;\n",
        );
        assert!(refs.is_empty(), "bare paths are external crates");
    }

    #[test]
    fn rust_ignores_comments_and_strings() {
        let refs = extract_imports(
            Path::new("src/a.rs"),
            "// use crate::db;\nfn f() { let _ = \"use crate::api;\"; }\n",
        );
        assert!(refs.is_empty());
    }

    #[test]
    fn ts_relative_resolves_and_bare_is_skipped() {
        let refs = extract_imports(
            Path::new("src/api/index.ts"),
            "import { login } from \"../auth/session\";\nimport fs from \"node:fs\";\nimport react from \"react\";\n",
        );
        assert_eq!(refs, vec![ImportRef::Path("src/auth/session".to_owned())]);
    }

    #[test]
    fn ts_export_from_counts_as_import() {
        let refs = extract_imports(
            Path::new("src/api/index.ts"),
            "export { login } from \"./auth\";\nexport const x = 1;\n",
        );
        assert_eq!(refs, vec![ImportRef::Path("src/api/auth".to_owned())]);
    }

    #[test]
    fn ts_import_attributes_do_not_redirect_source() {
        let refs = extract_imports(
            Path::new("src/api/index.ts"),
            "import data from './auth' with { type: \"json\" };\n",
        );
        assert_eq!(refs, vec![ImportRef::Path("src/api/auth".to_owned())]);
    }

    #[test]
    fn python_absolute_and_relative_forms() {
        let refs = extract_imports(
            Path::new("pkg/api/handlers.py"),
            "import auth.tokens as t\nfrom db import pool\nfrom . import routes\nfrom ..auth import login\n",
        );
        assert_eq!(
            paths(&refs),
            vec!["auth/tokens", "db", "pkg/api/routes", "pkg/auth"]
        );
    }

    #[test]
    fn go_grouped_imports_and_domain_skip() {
        let refs = extract_imports(
            Path::new("api/a.go"),
            "package api\n\nimport (\n\ta \"app/auth\"\n\t\"app/db\"\n\t\"example.com/vendor/auth\"\n)\n",
        );
        assert_eq!(
            refs,
            vec![
                ImportRef::Suffix(vec!["app".to_owned(), "auth".to_owned()]),
                ImportRef::Suffix(vec!["app".to_owned(), "db".to_owned()]),
            ]
        );
    }

    #[test]
    fn unknown_extension_yields_nothing() {
        assert!(extract_imports(Path::new("a.md"), "import x\n").is_empty());
    }

    #[test]
    fn relative_escape_above_root_is_rejected() {
        assert_eq!(resolve_relative("src", "../../evil"), None);
    }
}
