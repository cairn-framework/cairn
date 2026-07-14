//! Generic, [`LanguageSpec`]-parameterised code reconciler.
//!
//! Replaces the four near-identical per-language reconcilers
//! (`RustCodeReconciler`, `TypeScriptReconciler`, `PythonReconciler`,
//! `GoReconciler`) with one pipeline driven by [`LanguageSpec`] data. The
//! per-language modules now contain only a `LanguageSpec` constant plus the
//! tiny helpers it references.

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
    thread,
};

use tree_sitter::Parser;

use crate::{
    blueprint::{Ast, Node},
    map::graph::{Finding, FindingSeverity},
    reconcile::{
        ReconcileError, ReconcileReport, ReconcileRequest, Reconciler, ReconcilerId,
        fingerprint::InterfaceFingerprint,
        symbol::{SymbolKind, SymbolRecord},
        target::Language,
    },
    scanner::config::is_ignored,
};

/// Per-language data driving [`CodeReconciler`].
///
/// Keeping the language knowledge here (rather than in four duplicated
/// pipelines) is what lets a single `CodeReconciler` serve every language.
pub struct LanguageSpec {
    /// Language identity.
    pub language: Language,
    /// Human-readable name used in orphan findings.
    pub display_name: &'static str,
    /// tree-sitter grammar constructor.
    pub grammar: fn() -> tree_sitter::Language,
    /// File extensions owned by this language.
    pub extensions: &'static [&'static str],
    /// tree-sitter node kinds eligible for symbol extraction.
    pub exportable_kinds: &'static [&'static str],
    /// Resolves a node's symbol name and language-agnostic kind.
    pub name_and_kind: fn(tree_sitter::Node<'_>, &[u8]) -> (String, SymbolKind),
    /// Builds the stable interface signature for a node.
    pub interface_symbol: fn(tree_sitter::Node<'_>, &[u8]) -> String,
    /// Whether a node of an exportable kind is actually public.
    pub is_exportable: fn(tree_sitter::Node<'_>, &[u8]) -> bool,
    /// Skip the parser when the file cannot contain public symbols.
    pub fast_path: bool,
    /// Reconcile error code for grammar setup failures.
    pub grammar_error_code: &'static str,
    /// Reconcile error code for parse failures.
    pub parse_error_code: &'static str,
}

/// Code reconciler parameterised by a [`LanguageSpec`].
pub struct CodeReconciler<'a> {
    ast: &'a Ast,
    spec: &'static LanguageSpec,
}

impl<'a> CodeReconciler<'a> {
    /// Builds a reconciler for `spec` over `ast`.
    #[must_use]
    pub const fn new(ast: &'a Ast, spec: &'static LanguageSpec) -> Self {
        Self { ast, spec }
    }
}

impl Reconciler for CodeReconciler<'_> {
    fn id(&self) -> ReconcilerId {
        self.spec.language.reconciler_id()
    }

    fn reconcile(&self, request: ReconcileRequest<'_>) -> Result<ReconcileReport, ReconcileError> {
        let (claimed_files, findings) =
            discover_source_files(self.ast, self.spec, request.root, request.ignores)?;
        // The fast-path decision mirrors the original Rust reconciler: count
        // every discovered file (owned + orphan), not just the claimed ones.
        let discovered = claimed_files.values().map(Vec::len).sum::<usize>() + findings.len();
        if self.spec.fast_path && discovered < 16 {
            sequential_reconcile(self.spec, request.root, &claimed_files, findings)
        } else {
            parallel_reconcile(self.spec, request.root, &claimed_files, findings)
        }
    }
}

/// Parses one file and extracts its public symbols and records.
fn parse_file(
    spec: &'static LanguageSpec,
    root: &Path,
    file_rel: &str,
) -> Result<(Vec<String>, Vec<SymbolRecord>), ReconcileError> {
    let path = root.join(file_rel);
    let source = fs::read_to_string(&path).map_err(|error| ReconcileError {
        code: "CAIRN_RECONCILE_READ_SOURCE".to_owned(),
        message: format!("failed to read `{}`: {error}", path.display()),
    })?;
    // The `pub ` pre-parse skip is Rust-specific: only Rust public items are
    // guarded by `pub`, so applying it to other languages would drop every
    // symbol. It stays on for Rust to preserve the original fast path.
    if spec.fast_path
        && spec.language == Language::Rust
        && !source.as_bytes().windows(4).any(|w| w == b"pub ")
    {
        return Ok((Vec::new(), Vec::new()));
    }
    let mut parser = Parser::new();
    parser
        .set_language(&(spec.grammar)())
        .map_err(|error| ReconcileError {
            code: spec.grammar_error_code.to_owned(),
            message: error.to_string(),
        })?;
    let tree = parser.parse(&source, None).ok_or_else(|| ReconcileError {
        code: spec.parse_error_code.to_owned(),
        message: format!("failed to parse `{}`", path.display()),
    })?;
    let mut symbols = Vec::new();
    let mut records = Vec::new();
    collect_public_symbols(
        spec,
        tree.root_node(),
        source.as_bytes(),
        file_rel,
        &mut symbols,
        &mut records,
    );
    Ok((symbols, records))
}

/// Recursively collects public symbols from a parsed tree.
#[allow(clippy::too_many_arguments)] // Reason: collect_public_symbols takes the spec, node, source, file rel, and two accumulators by design.
fn collect_public_symbols(
    spec: &'static LanguageSpec,
    node: tree_sitter::Node<'_>,
    source: &[u8],
    file_rel: &str,
    symbols: &mut Vec<String>,
    records: &mut Vec<SymbolRecord>,
) {
    if node.child_count() == 0 {
        return;
    }
    let kind = node.kind();
    let is_target = spec.exportable_kinds.contains(&kind);
    if is_target && (spec.is_exportable)(node, source) {
        let record = build_symbol(spec, node, source, file_rel);
        symbols.push(record.signature.clone());
        records.push(record);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_public_symbols(spec, child, source, file_rel, symbols, records);
    }
}

/// Builds a [`SymbolRecord`] for an exportable node.
fn build_symbol(
    spec: &'static LanguageSpec,
    node: tree_sitter::Node<'_>,
    source: &[u8],
    file_rel: &str,
) -> SymbolRecord {
    let (name, kind) = (spec.name_and_kind)(node, source);
    SymbolRecord {
        name,
        kind,
        signature: (spec.interface_symbol)(node, source),
        file: file_rel.to_owned(),
        line: u32::try_from(node.start_position().row).unwrap_or(u32::MAX) + 1,
        end_line: u32::try_from(node.end_position().row).unwrap_or(u32::MAX) + 1,
    }
}

/// Discovers source files, assigns each to its most-specific owning node, and
/// records orphan findings for files no node owns.
type ClaimedFiles = BTreeMap<String, Vec<String>>;

fn discover_source_files(
    ast: &Ast,
    spec: &'static LanguageSpec,
    root: &Path,
    ignores: &[String],
) -> Result<(ClaimedFiles, Vec<Finding>), ReconcileError> {
    let owners = eligible_owners(ast);
    let mut files = Vec::with_capacity(128);
    walk(root, root, ignores, spec.extensions, &mut files)?;
    files.sort_unstable();
    let mut claimed_files: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut findings = Vec::new();
    for path in files {
        let rel = normalize(path.strip_prefix(root).unwrap_or(&path));
        if let Some(owner) = most_specific_owner(&owners, &rel) {
            claimed_files
                .entry(owner)
                .or_default()
                .push(rel.into_owned());
        } else {
            findings.push(Finding {
                code: "CAIRN_RECONCILE_ORPHANED_FILE".to_owned(),
                severity: FindingSeverity::Info,
                message: format!(
                    "{} file `{rel}` is not owned by any eligible node",
                    spec.display_name
                ),
                node: None,
                target: None,
                path: Some(rel.into_owned()),
                deferred_by: None,
            });
        }
    }
    Ok((claimed_files, findings))
}

/// Recursively walks `dir` collecting files with one of `extensions`.
fn walk(
    root: &Path,
    dir: &Path,
    ignores: &[String],
    extensions: &[&str],
    files: &mut Vec<PathBuf>,
) -> Result<(), ReconcileError> {
    for entry in fs::read_dir(dir).map_err(|error| ReconcileError {
        code: "CAIRN_RECONCILE_READ_DIR".to_owned(),
        message: format!("failed to read `{}`: {error}", dir.display()),
    })? {
        let entry = entry.map_err(|error| ReconcileError {
            code: "CAIRN_RECONCILE_READ_DIR_ENTRY".to_owned(),
            message: error.to_string(),
        })?;
        let path = entry.path();
        let rel = normalize(path.strip_prefix(root).unwrap_or(&path));
        if is_ignored(&rel, ignores) {
            continue;
        }
        let file_type = entry.file_type().map_err(|error| ReconcileError {
            code: "CAIRN_RECONCILE_READ_DIR_ENTRY".to_owned(),
            message: error.to_string(),
        })?;
        if file_type.is_dir() {
            walk(root, &path, ignores, extensions, files)?;
        } else if file_type.is_file()
            && let Some(ext) = path.extension().and_then(|e| e.to_str())
            && extensions.contains(&ext)
        {
            files.push(path);
        }
    }
    Ok(())
}

/// Sequential reconcile for small Rust trees (preserves the original fast path).
fn sequential_reconcile(
    spec: &'static LanguageSpec,
    root: &Path,
    claimed_files: &BTreeMap<String, Vec<String>>,
    findings: Vec<Finding>,
) -> Result<ReconcileReport, ReconcileError> {
    let mut symbols = Vec::new();
    let mut node_symbols: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut node_symbol_records: BTreeMap<String, Vec<SymbolRecord>> = BTreeMap::new();
    for (node_id, files) in claimed_files {
        for file_rel in files {
            let (file_symbols, file_records) = parse_file(spec, root, file_rel)?;
            symbols.extend(file_symbols.clone());
            node_symbols
                .entry(node_id.clone())
                .or_default()
                .extend(file_symbols);
            node_symbol_records
                .entry(node_id.clone())
                .or_default()
                .extend(file_records);
        }
    }
    symbols.sort_unstable();
    for node_syms in node_symbols.values_mut() {
        node_syms.sort_unstable();
    }
    for records in node_symbol_records.values_mut() {
        records.sort_by(|a, b| a.signature.cmp(&b.signature));
    }
    Ok(ReconcileReport {
        fingerprint: InterfaceFingerprint::from_sorted(&symbols),
        claimed_files: claimed_files.clone(),
        symbols: Arc::new(symbols),
        node_symbols,
        node_symbol_records,
        findings,
    })
}

/// Parallel reconcile for larger trees and non-Rust languages.
fn parallel_reconcile(
    spec: &'static LanguageSpec,
    root: &Path,
    claimed_files: &BTreeMap<String, Vec<String>>,
    findings: Vec<Finding>,
) -> Result<ReconcileReport, ReconcileError> {
    let thread_count = thread::available_parallelism().map_or(2, usize::from);
    let items: Vec<(String, String)> = claimed_files
        .iter()
        .flat_map(|(node_id, files)| files.iter().map(|file| (node_id.clone(), file.clone())))
        .collect();
    let chunk_size = items.len().div_ceil(thread_count).max(1);
    let chunks: Vec<_> = items.chunks(chunk_size).collect();
    thread::scope(|s| {
        let mut handles = Vec::with_capacity(chunks.len());
        for chunk in chunks {
            handles.push(s.spawn(move || {
                let mut claimed = BTreeMap::<String, Vec<String>>::new();
                let mut node_symbols = BTreeMap::<String, Vec<String>>::new();
                let mut node_symbol_records = BTreeMap::<String, Vec<SymbolRecord>>::new();
                let mut symbols = Vec::new();
                for (node_id, file_rel) in chunk {
                    let (file_symbols, file_records) = parse_file(spec, root, file_rel)?;
                    claimed
                        .entry(node_id.clone())
                        .or_default()
                        .push(file_rel.clone());
                    node_symbols
                        .entry(node_id.clone())
                        .or_default()
                        .extend(file_symbols.clone());
                    node_symbol_records
                        .entry(node_id.clone())
                        .or_default()
                        .extend(file_records);
                    symbols.extend(file_symbols);
                }
                Ok::<_, ReconcileError>((claimed, node_symbols, node_symbol_records, symbols))
            }));
        }
        let mut all_claimed = BTreeMap::<String, Vec<String>>::new();
        let mut all_node_symbols = BTreeMap::<String, Vec<String>>::new();
        let mut all_node_symbol_records = BTreeMap::<String, Vec<SymbolRecord>>::new();
        let mut all_symbols = Vec::new();
        for handle in handles {
            let (claimed, node_symbols, node_symbol_records, symbols) = handle.join().unwrap()?;
            for (owner, files) in claimed {
                all_claimed.entry(owner).or_default().extend(files);
            }
            for (owner, syms) in node_symbols {
                all_node_symbols.entry(owner).or_default().extend(syms);
            }
            for (owner, records) in node_symbol_records {
                all_node_symbol_records
                    .entry(owner)
                    .or_default()
                    .extend(records);
            }
            all_symbols.extend(symbols);
        }
        all_symbols.sort_unstable();
        for node_syms in all_node_symbols.values_mut() {
            node_syms.sort_unstable();
        }
        for records in all_node_symbol_records.values_mut() {
            records.sort_by(|a, b| a.signature.cmp(&b.signature));
        }
        Ok(ReconcileReport {
            fingerprint: InterfaceFingerprint::from_sorted(&all_symbols),
            claimed_files: all_claimed,
            symbols: Arc::new(all_symbols),
            node_symbols: all_node_symbols,
            node_symbol_records: all_node_symbol_records,
            findings,
        })
    })
}

/// Returns `(node_id, path)` pairs for every node that can own files, sorted
/// most-specific path first.
fn eligible_owners(ast: &Ast) -> Vec<(String, String)> {
    let mut owners = Vec::new();
    for node in &ast.nodes {
        collect_owner(node, &mut owners);
    }
    owners.sort_by_key(|b| std::cmp::Reverse(b.1.len()));
    owners
}

/// Accumulates `(node_id, path)` pairs for `node` and its descendants.
fn collect_owner(node: &Node, owners: &mut Vec<(String, String)>) {
    let is_internal = !node.children.is_empty();
    if !is_internal || node.owns_files {
        for path in &node.paths {
            owners.push((node.id.clone(), trim_dot(path)));
        }
    }
    for child in &node.children {
        collect_owner(child, owners);
    }
}

/// Returns the most-specific owning node for `file`, if any.
fn most_specific_owner(owners: &[(String, String)], file: &str) -> Option<String> {
    for (id, path) in owners {
        if path.is_empty()
            || path == "."
            || file == path
            || (file.starts_with(path) && file.as_bytes().get(path.len()) == Some(&b'/'))
        {
            return Some(id.clone());
        }
    }
    None
}

/// Strips a leading `./` from a path segment.
fn trim_dot(path: &str) -> String {
    path.trim_start_matches("./").to_owned()
}

/// Normalises a path to forward slashes for stable comparison.
fn normalize(path: &Path) -> std::borrow::Cow<'_, str> {
    let s = path.to_string_lossy();
    if s.contains('\\') {
        std::borrow::Cow::Owned(s.replace('\\', "/"))
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_dot_strips_leading_dot_slash() {
        assert_eq!(trim_dot("./src/main.rs"), "src/main.rs");
    }

    #[test]
    fn trim_dot_leaves_unchanged_when_no_leading_dot_slash() {
        assert_eq!(trim_dot("src/main.rs"), "src/main.rs");
    }

    #[test]
    fn normalize_backslash_to_forward_slash() {
        let path = PathBuf::from("src\\main.rs");
        assert_eq!(normalize(&path), "src/main.rs");
    }

    #[test]
    fn normalize_forward_slashes_unchanged() {
        let path = PathBuf::from("src/main.rs");
        assert_eq!(normalize(&path), "src/main.rs");
    }
}
