// cairn:allow-large-module reason: meta-test suite for CAIRN finding-code coverage and registry-completeness gates; both tests share one tree-sitter scan of src/ and splitting would duplicate that scan setup across files for no benefit
//! Meta-test guarding `CAIRN_*` finding/error codes against two kinds of
//! silent drift:
//!
//! - `test_every_emitted_code_is_tested_or_allowlisted`
//!   (`todo.finding-code-test-coverage`): a code ships with no test that
//!   ever triggers it.
//! - `test_every_emitted_code_has_a_registry_entry`
//!   (`todo.error-codes-registry-completeness`): a code ships with no entry
//!   in `docs/registries/error-codes.md`.
//!
//! Both tests share one scan of `src/`, built with `tree-sitter-rust` so
//! comments, doc comments, match-arm/dispatch references
//! (`"CAIRN_X" | "CAIRN_Y" => ...`, `.code == "CAIRN_X"`) and codes only
//! synthesised at runtime via `format!` are excluded by construction,
//! rather than by textual heuristics. A code counts as *emitted* only when
//! a `CAIRN_*` value (a string literal, or an identifier resolving through
//! a `const NAME: &str = "CAIRN_X";` alias) sits in an actual finding/error
//! `code` position:
//!
//! - a struct field initialiser named `code`, `grammar_error_code`, or
//!   `parse_error_code` (`Finding { code: "CAIRN_X", .. }` or
//!   `Finding { code: SOME_CONST.to_owned(), .. }`), including one written
//!   inside a macro invocation's raw token tree (`vec![Finding { code: .. }]`);
//! - the `"code"` key of a `json!({ .. })` macro invocation;
//! - a direct argument to one of the finding-constructor helpers in
//!   [`CALL_NAMES`];
//! - the body of a bare `|| "CAIRN_X"` fallback closure.
//!
//! `const` aliases are resolved wherever the identifier appears (same
//! position rules), not merely at their definition site: a code shipped
//! only as `code: SOME_CONST.to_owned()` still counts as emitted, and
//! `f.code == SOME_CONST` still does not.
//!
//! A code counts as *asserted* (test coverage) when the same literal, or a
//! `const` alias resolving to it, appears anywhere under `tests/` or
//! inside a `#[cfg(test)]` region of `src/` (inline `mod tests { .. }` or
//! an external `mod tests;` declaration resolved to its file).
//!
//! Deliberately out of scope: the older, separate `CXNNN` short-code
//! convention (`"CH001"`, `"CT002"`, `"CC001"`, ...). Those codes are a
//! pre-existing, fully-registered scheme this meta-test does not audit.
//!
//! A first audit (2026-07-16) found 99 distinct emitted `CAIRN_*` codes:
//! 35 untested (see `UNCOVERED_ALLOWLIST`) and 90 missing from the
//! registry (since reconciled). The originating todo estimated "about 20"
//! untested codes from a casual read; the real count, found by scanning
//! every emission site rather than sampling, is 35.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use tree_sitter::{Node, Parser, Tree};

/// Finding/error-constructor helpers whose code argument is a
/// `CAIRN_*` code. `required` is a special case whose second argument is
/// the code. Struct-literal construction (`Finding { code: .. }`)
/// does not need to be listed here: it is recognised via `field_initializer`
/// directly.
const CALL_NAMES: &[&str] = &[
    "error_finding",
    "warning",
    "error",
    "err_finding",
    "info",
    "error_json",
    "error_output",
    "detect_duplicate_targets",
    "required",
];

/// Struct field names that bind a `CAIRN_*` finding/error code.
const FIELD_NAMES: &[&str] = &["code", "grammar_error_code", "parse_error_code"];

/// Codes emitted in `src/` with no test that triggers them yet. Each entry
/// needs a reason; this is a burn-down list, not a resting place. Removing
/// an entry (because a test now covers it) requires no other change here;
/// adding one requires justification in this comment block.
///
/// - `CAIRN_ARTEFACT_*_READ_FAILED` / `CAIRN_ARTEFACT_MISSING_FIELD` /
///   `CAIRN_ARTEFACT_POINTER_MISSING`: artefact-registry I/O failure paths
///   (unreadable directory/file, missing pointer target) not currently
///   exercised by any fixture that forces a read error.
/// - `CAIRN_CONTRACT_READ_FAILED`, `CAIRN_CONFIG_READ_FAILED`,
///   `CAIRN_IGNORE_READ_FAILED`, `CAIRN_IO_READ_BLUEPRINT`,
///   `CAIRN_SOURCE_READ_FAILED`: same shape, other subsystems' unreadable-
///   file paths.
/// - `CAIRN_RECONCILE_*_LANGUAGE` / `CAIRN_RECONCILE_PARSE_*` /
///   `CAIRN_RECONCILE_READ_*`: the reconcile-per-language IO/grammar/parse
///   failure tier the originating todo named directly; no test currently
///   forces a grammar-load failure, a parse failure, or an unreadable
///   reconcile target for any of the five supported languages.
/// - `CAIRN_QUERY_INVALID_HOOK_KIND` / `CAIRN_QUERY_MUTATION_NOT_ALLOWED` /
///   `CAIRN_QUERY_UNIMPLEMENTED_TOOL` / `CAIRN_QUERY_UNKNOWN_TOOL`: MCP/
///   query-layer request-validation error paths, not currently driven by
///   an integration test that sends a malformed or disallowed request.
/// - `CAIRN_DRAFTS_LIST_FAILED` / `CAIRN_DRAFT_ACCEPT_FAILED` /
///   `CAIRN_DRAFT_INVALID_TRANSITION` / `CAIRN_DRAFT_NOT_FOUND`: change-
///   draft query-handler error paths.
/// - `CAIRN_SUMMARISER_CONFIG_ERROR` / `CAIRN_SUMMARISER_GENERATION_FAILED`
///   / `CAIRN_SUMMARISER_PROMPT_ERROR`: summariser backend failure paths
///   (only `CAIRN_SUMMARISER_DISABLED` is currently tested).
/// - `CAIRN_CLI_MISSING_CHANGE`, `CAIRN_RESEARCH_METHOD_INVALID`,
///   `CAIRN_UI_PROJECT_LOAD_FAILED`: single untested validation paths in
///   the CLI, artefact-registry parser, and local UI server respectively.
const UNCOVERED_ALLOWLIST: &[&str] = &[
    // Artefact directory unreadable or missing
    "CAIRN_ARTEFACT_DIR_READ_FAILED",
    // Required field missing in frontmatter
    "CAIRN_ARTEFACT_MISSING_FIELD",
    // Artefact pointer does not resolve to a file
    "CAIRN_ARTEFACT_POINTER_MISSING",
    // Individual artefact file read failure
    "CAIRN_ARTEFACT_READ_FAILED",
    // change-id parameter missing in CLI command
    "CAIRN_CLI_MISSING_CHANGE",
    // config file read failure
    "CAIRN_CONFIG_READ_FAILED",
    // contract file read failure
    "CAIRN_CONTRACT_READ_FAILED",
    // change drafts listing failure
    "CAIRN_DRAFTS_LIST_FAILED",
    // accept draft transaction failure
    "CAIRN_DRAFT_ACCEPT_FAILED",
    // transition invalid from current draft state
    "CAIRN_DRAFT_INVALID_TRANSITION",
    // requested draft not found
    "CAIRN_DRAFT_NOT_FOUND",
    // ignore file read failure
    "CAIRN_IGNORE_READ_FAILED",
    // blueprint file read failure
    "CAIRN_IO_READ_BLUEPRINT",
    // unexpected parser token encountered
    "CAIRN_PARSE_UNEXPECTED_TOKEN",
    // query tool got unknown hook kind
    "CAIRN_QUERY_INVALID_HOOK_KIND",
    // mutating query without mutating flag set
    "CAIRN_QUERY_MUTATION_NOT_ALLOWED",
    // tool registered but not implemented
    "CAIRN_QUERY_UNIMPLEMENTED_TOOL",
    // requested query tool not found
    "CAIRN_QUERY_UNKNOWN_TOOL",
    // Go grammar failing to load
    "CAIRN_RECONCILE_GO_LANGUAGE",
    // Go file parser syntax error
    "CAIRN_RECONCILE_PARSE_GO",
    // Python file parser syntax error
    "CAIRN_RECONCILE_PARSE_PYTHON",
    // Rust file parser syntax error
    "CAIRN_RECONCILE_PARSE_RUST",
    // TypeScript file parser syntax error
    "CAIRN_RECONCILE_PARSE_TYPESCRIPT",
    // Python grammar failing to load
    "CAIRN_RECONCILE_PYTHON_LANGUAGE",
    // directory entry read failure in reconciler
    "CAIRN_RECONCILE_READ_DIR_ENTRY",
    // individual source file read failure in reconciler
    "CAIRN_RECONCILE_READ_SOURCE",
    // Rust grammar failing to load
    "CAIRN_RECONCILE_RUST_LANGUAGE",
    // TypeScript grammar failing to load
    "CAIRN_RECONCILE_TYPESCRIPT_LANGUAGE",
    // research method is invalid
    "CAIRN_RESEARCH_METHOD_INVALID",
    // source file read failure in registry validation
    "CAIRN_SOURCE_READ_FAILED",
    // summariser settings parsing/config error
    "CAIRN_SUMMARISER_CONFIG_ERROR",
    // summariser model response generation failure
    "CAIRN_SUMMARISER_GENERATION_FAILED",
    // summariser prompt building failure
    "CAIRN_SUMMARISER_PROMPT_ERROR",
    // ui server project load failure
    "CAIRN_UI_PROJECT_LOAD_FAILED",
    // required query change parameter path is not currently exercised
    "CAIRN_QUERY_MISSING_CHANGE",
    // required query old-id parameter path is not currently exercised
    "CAIRN_QUERY_MISSING_OLD_ID",
    // required query new-id parameter path is not currently exercised
    "CAIRN_QUERY_MISSING_NEW_ID",
    // required query symbol parameter path is not currently exercised
    "CAIRN_QUERY_MISSING_SYMBOL",
    // CLI locate symbol argument path is not currently exercised
    "CAIRN_CLI_MISSING_SYMBOL",
];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// True for tokens shaped like `CAIRN_SOME_CODE`. Deliberately excludes the
/// short `CXNNN` convention (see module docs).
fn is_cairn_code(s: &str) -> bool {
    let Some(rest) = s.strip_prefix("CAIRN_") else {
        return false;
    };
    !rest.is_empty()
        && rest
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
}

/// String-literal content, quotes stripped. Only plain `"..."` literals are
/// handled (`raw_string_literal` is skipped): no `CAIRN_*` code in this
/// codebase is ever spelled as a raw string.
fn literal_content<'a>(node: Node<'_>, source: &'a [u8]) -> Option<&'a str> {
    if node.kind() != "string_literal" {
        return None;
    }
    let text = node.utf8_text(source).ok()?;
    text.strip_prefix('"')?.strip_suffix('"')
}

/// The `CAIRN_*` code carried by `node`, if any: either a direct string
/// literal, or an `identifier` resolving through `consts`.
fn code_of(node: Node<'_>, source: &[u8], consts: &BTreeMap<String, String>) -> Option<String> {
    if let Some(text) = literal_content(node, source) {
        return is_cairn_code(text).then(|| text.to_owned());
    }
    if node.kind() == "identifier"
        && let Ok(name) = node.utf8_text(source)
    {
        return consts.get(name).cloned();
    }
    None
}

/// Walks up from a literal/identifier code node through pure "wrapper"
/// positions (method-call receiver, `&`/`&mut`, parens) to the first
/// position that carries real meaning, classifying it as an emission site
/// or not. Works identically for a `string_literal` node or an
/// `identifier` node aliasing a `const` code, since it only inspects
/// parent structure.
fn classify(node: Node<'_>, source: &[u8]) -> bool {
    let mut cur = node;
    loop {
        let Some(parent) = cur.parent() else {
            return false;
        };
        match parent.kind() {
            // `<expr>.method()`: `cur` is the receiver; `&<expr>`/`&mut
            // <expr>`/`(<expr>)`: `cur` is the inner expression. Either
            // way this is a pure wrapper: keep unwrapping.
            "field_expression"
            | "reference_expression"
            | "unary_expression"
            | "parenthesized_expression" => {
                cur = parent;
            }
            // `<expr>(...)`: only a wrapper when `cur` is the *callee*
            // (i.e. this is a method call on `cur`, like `.to_owned()`);
            // otherwise `parent` is `cur`'s own enclosing call and is
            // handled by the "arguments" arm below.
            "call_expression" if parent.child_by_field_name("function") == Some(cur) => {
                cur = parent;
            }
            // Bare `|| "CAIRN_X"` fallback closure: the closure body is
            // the code value itself.
            "closure_expression"
                if parent
                    .utf8_text(source)
                    .is_ok_and(|t| t.trim_start().starts_with("||")) =>
            {
                return true;
            }
            // Direct argument to some call: check the call's function name.
            "arguments" => {
                let Some(call) = parent.parent().filter(|c| c.kind() == "call_expression") else {
                    return false;
                };
                let Some(func) = call.child_by_field_name("function") else {
                    return false;
                };
                let Ok(func_text) = func.utf8_text(source) else {
                    return false;
                };
                let name = func_text.rsplit("::").next().unwrap_or(func_text);
                if name == "required" {
                    return parent.named_child(1) == Some(cur);
                }
                return CALL_NAMES.contains(&name);
            }
            "field_initializer" => {
                let Some(name_node) = parent.child_by_field_name("field") else {
                    return false;
                };
                let Ok(name) = name_node.utf8_text(source) else {
                    return false;
                };
                return FIELD_NAMES.contains(&name);
            }
            // Inside any macro invocation (`vec!`, `format!`, `json!`, ..),
            // the body is a raw token tree: no `field_initializer` or
            // `call_expression` nodes exist. Recognise the `key :` token
            // pair immediately preceding this literal, where `key` is
            // either a bare struct-field identifier (`code: "X"` written
            // inside e.g. `vec![Finding { code: "X".to_owned(), .. }]`) or
            // a JSON string key (`"code": "X"` inside `json!({ .. })`).
            "token_tree" => {
                let Some(colon) = cur.prev_sibling().filter(|n| n.kind() == ":") else {
                    return false;
                };
                let Some(key) = colon.prev_sibling() else {
                    return false;
                };
                if let Some(text) = literal_content(key, source) {
                    return text == "code";
                }
                if key.kind() == "identifier"
                    && let Ok(name) = key.utf8_text(source)
                {
                    return FIELD_NAMES.contains(&name);
                }
                return false;
            }
            _ => return false,
        }
    }
}

/// One directory's worth of `#[cfg(test)] mod NAME;` (external module)
/// resolution: `NAME.rs` or `NAME/mod.rs`, rooted per Rust 2018 module
/// path rules (sibling of a `mod.rs`/`lib.rs`/`main.rs`; subdirectory
/// named after the file stem otherwise).
fn resolve_external_test_mod(file: &Path, mod_name: &str) -> Option<PathBuf> {
    let is_dir_root = matches!(
        file.file_name().and_then(|n| n.to_str()),
        Some("mod.rs" | "lib.rs" | "main.rs")
    );
    let base = if is_dir_root {
        file.parent()?.to_path_buf()
    } else {
        file.parent()?.join(file.file_stem()?)
    };
    let flat = base.join(format!("{mod_name}.rs"));
    if flat.is_file() {
        return Some(flat);
    }
    let nested = base.join(mod_name).join("mod.rs");
    nested.is_file().then_some(nested)
}

fn attr_is_cfg_test(node: Node<'_>, source: &[u8]) -> bool {
    node.kind() == "attribute_item"
        && node
            .utf8_text(source)
            .is_ok_and(|t| t.split_whitespace().collect::<String>() == "#[cfg(test)]")
}

/// First non-comment named sibling after `node`.
fn next_meaningful_sibling(node: Node<'_>) -> Option<Node<'_>> {
    let mut cur = node.next_named_sibling();
    while let Some(n) = cur {
        if matches!(n.kind(), "line_comment" | "block_comment") {
            cur = n.next_named_sibling();
        } else {
            return Some(n);
        }
    }
    None
}

fn parse_rust(source: &str) -> Option<Tree> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("rust grammar must load");
    parser.parse(source, None)
}

/// A parsed source file kept around across the two scan passes.
struct Parsed {
    path: PathBuf,
    source: String,
    tree: Tree,
}

fn parse_all(files: &[PathBuf]) -> Vec<Parsed> {
    files
        .iter()
        .map(|path| {
            let source = fs::read_to_string(path).unwrap_or_else(|error| {
                panic!("failed to read Rust file {}: {error}", path.display())
            });
            let tree = parse_rust(&source)
                .unwrap_or_else(|| panic!("failed to parse Rust file {}", path.display()));
            assert!(
                !tree.root_node().has_error(),
                "syntax errors in Rust file {}: scan would silently miss codes",
                path.display()
            );
            Parsed {
                path: path.clone(),
                source,
                tree,
            }
        })
        .collect()
}

/// Accumulator for pass 1 ([`collect_consts_and_test_mods`]): every
/// `const NAME: &str = "CAIRN_X"` definition found so far, and every
/// external `#[cfg(test)] mod NAME;` target.
struct ConstScan<'a> {
    bytes: &'a [u8],
    path: &'a Path,
    consts: BTreeMap<String, String>,
    external_test_files: BTreeSet<PathBuf>,
}

fn walk_consts(node: Node<'_>, scan: &mut ConstScan<'_>) {
    if node.kind() == "const_item"
        && let (Some(name_node), Some(value_node)) = (
            node.child_by_field_name("name"),
            node.child_by_field_name("value"),
        )
        && let Ok(name) = name_node.utf8_text(scan.bytes)
    {
        let mut v = value_node;
        // unwrap `.to_owned()`/`.into()` wrappers to reach the literal
        while v.kind() == "call_expression"
            && let Some(f) = v.child_by_field_name("function")
            && f.kind() == "field_expression"
            && let Some(recv) = f.child_by_field_name("value")
        {
            v = recv;
        }
        if let Some(code) = literal_content(v, scan.bytes)
            && is_cairn_code(code)
        {
            scan.consts.insert(name.to_owned(), code.to_owned());
        }
    }
    if attr_is_cfg_test(node, scan.bytes)
        && let Some(item) = next_meaningful_sibling(node)
        && item.kind() == "mod_item"
        && item.child_by_field_name("body").is_none()
        && let Some(name_node) = item.child_by_field_name("name")
        && let Ok(name) = name_node.utf8_text(scan.bytes)
        && let Some(target) = resolve_external_test_mod(scan.path, name)
    {
        scan.external_test_files.insert(target);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        walk_consts(child, scan);
    }
}

/// Pass 1: collect every `const NAME: &str = "CAIRN_X";` definition and
/// every external `#[cfg(test)] mod NAME;` target, without classifying
/// emission sites yet (that needs the completed const map).
fn collect_consts_and_test_mods(files: &[Parsed]) -> (BTreeMap<String, String>, BTreeSet<PathBuf>) {
    let mut consts = BTreeMap::new();
    let mut external_test_files = BTreeSet::new();
    for file in files {
        let mut scan = ConstScan {
            bytes: file.source.as_bytes(),
            path: &file.path,
            consts: BTreeMap::new(),
            external_test_files: BTreeSet::new(),
        };
        walk_consts(file.tree.root_node(), &mut scan);
        consts.extend(scan.consts);
        external_test_files.extend(scan.external_test_files);
    }
    (consts, external_test_files)
}

/// Accumulator for pass 2 ([`classify_file`]): the emitted/asserted codes
/// found so far in one file, plus the byte ranges of `#[cfg(test)]`
/// regions discovered along the way.
struct ClassifyScan<'a> {
    bytes: &'a [u8],
    consts: &'a BTreeMap<String, String>,
    whole_file_is_test: bool,
    test_regions: Vec<(usize, usize)>,
    emitted: BTreeSet<String>,
    asserted: BTreeSet<String>,
}

fn is_assert_macro(node: Node<'_>, bytes: &[u8]) -> bool {
    let Ok(text) = node.utf8_text(bytes) else {
        return false;
    };
    let Some((name, _)) = text.split_once('!') else {
        return false;
    };
    let name = name.rsplit([':', '.']).next().unwrap_or(name).trim();
    matches!(
        name,
        "assert"
            | "assert_eq"
            | "assert_ne"
            | "assert_snapshot"
            | "assert_debug_snapshot"
            | "assert_json_snapshot"
            | "assert_yaml_snapshot"
            | "assert_binary_snapshot"
            | "assert_compact_debug_snapshot"
    )
}

fn inside_assert_macro(node: Node<'_>, bytes: &[u8]) -> bool {
    let mut current = node.parent();
    while let Some(ancestor) = current {
        if ancestor.kind() == "macro_invocation" && is_assert_macro(ancestor, bytes) {
            return true;
        }
        current = ancestor.parent();
    }
    false
}

/// True when a `CAIRN_*` literal participates in an equality comparison
/// inside a closure predicate (e.g. `f.code == "CAIRN_X"` passed to
/// `find`/`filter`/`any`). Such comparisons verify an exercised result
/// even when the enclosing assertion is a later `assert_eq!` on the
/// filtered set or an `.expect()` on the found item, so they count as
/// coverage alongside assert-family macro ancestry. The closure
/// requirement excludes dead statement-level comparisons; a discarded
/// iterator predicate cannot survive the `clippy -D warnings` gate's
/// `must_use` enforcement.
fn is_predicate_code_comparison(node: Node<'_>, bytes: &[u8]) -> bool {
    let mut saw_comparison = false;
    let mut current = node.parent();
    while let Some(ancestor) = current {
        if !saw_comparison && ancestor.kind() == "binary_expression" {
            let mut cursor = ancestor.walk();
            saw_comparison = ancestor
                .children(&mut cursor)
                .any(|child| matches!(child.kind(), "==" | "!="));
        }
        if saw_comparison
            && ancestor.kind() == "closure_expression"
            && closure_is_predicate_argument(ancestor, bytes)
        {
            return true;
        }
        current = ancestor.parent();
    }
    false
}

/// True when a closure is the argument of a recognised predicate method
/// (`find`, `filter`, `any`, `all`, `position`, `find_map`), whose result
/// selects or tests items rather than transforming them. Comparisons in
/// `map`/`for_each`/stored closures do not count.
fn closure_is_predicate_argument(closure: Node<'_>, bytes: &[u8]) -> bool {
    let Some(args) = closure.parent().filter(|p| p.kind() == "arguments") else {
        return false;
    };
    let Some(call) = args.parent().filter(|p| p.kind() == "call_expression") else {
        return false;
    };
    let Some(function) = call.child_by_field_name("function") else {
        return false;
    };
    if function.kind() != "field_expression" {
        return false;
    }
    function
        .child_by_field_name("field")
        .and_then(|field| field.utf8_text(bytes).ok())
        .is_some_and(|name| {
            matches!(
                name,
                "find" | "filter" | "any" | "all" | "position" | "find_map"
            )
        })
}

fn walk_classify(node: Node<'_>, scan: &mut ClassifyScan<'_>) {
    if attr_is_cfg_test(node, scan.bytes)
        && let Some(item) = next_meaningful_sibling(node)
        && item.kind() == "mod_item"
        && let Some(body) = item.child_by_field_name("body")
    {
        scan.test_regions.push((body.start_byte(), body.end_byte()));
    } else if attr_is_cfg_test(node, scan.bytes)
        && let Some(item) = next_meaningful_sibling(node)
        && item.kind() != "mod_item"
    {
        scan.test_regions.push((item.start_byte(), item.end_byte()));
    }

    if let Some(code) = code_of(node, scan.bytes, scan.consts) {
        if scan.whole_file_is_test {
            if inside_assert_macro(node, scan.bytes)
                || is_predicate_code_comparison(node, scan.bytes)
            {
                scan.asserted.insert(code);
            }
        } else {
            let in_test_region = scan
                .test_regions
                .iter()
                .any(|(s, e)| *s <= node.start_byte() && node.start_byte() < *e);
            if in_test_region {
                if inside_assert_macro(node, scan.bytes)
                    || is_predicate_code_comparison(node, scan.bytes)
                {
                    scan.asserted.insert(code);
                }
            } else if classify(node, scan.bytes) {
                scan.emitted.insert(code);
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        walk_classify(child, scan);
    }
}

/// Pass 2: walk a parsed file, collecting emitted codes (production
/// positions only) and asserted codes (`#[cfg(test)]` regions, or the
/// whole file when `whole_file_is_test`), resolving `const` aliases via
/// the global `consts` map built in pass 1.
fn classify_file(
    file: &Parsed,
    whole_file_is_test: bool,
    consts: &BTreeMap<String, String>,
    emitted: &mut BTreeSet<String>,
    asserted: &mut BTreeSet<String>,
) {
    let mut scan = ClassifyScan {
        bytes: file.source.as_bytes(),
        consts,
        whole_file_is_test,
        test_regions: Vec::new(),
        emitted: BTreeSet::new(),
        asserted: BTreeSet::new(),
    };
    walk_classify(file.tree.root_node(), &mut scan);
    emitted.extend(scan.emitted);
    asserted.extend(scan.asserted);
}

fn rust_files_under(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let entries = fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("failed to read directory {}: {error}", dir.display()));
    for entry in entries {
        let path = entry
            .unwrap_or_else(|error| {
                panic!(
                    "failed to read directory entry in {}: {error}",
                    dir.display()
                )
            })
            .path();
        if path.is_dir() {
            out.extend(rust_files_under(&path));
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
    out
}

/// Emitted codes (production `src/`) and asserted codes (`tests/` plus
/// every `#[cfg(test)]` region/file in `src/`), scanned once and shared by
/// both tests below.
fn scan_codebase() -> (BTreeSet<String>, BTreeSet<String>) {
    let root = repo_root();
    // This file mentions every `CAIRN_*` code by name (`CALL_NAMES`,
    // `UNCOVERED_ALLOWLIST`, module docs) without asserting any of them;
    // scanning it as test coverage would make the allowlist self-satisfy.
    let self_path = PathBuf::from(file!());
    let test_files: Vec<PathBuf> = rust_files_under(&root.join("tests"))
        .into_iter()
        .filter(|p| p.file_name() != self_path.file_name())
        .collect();
    let src_files = rust_files_under(&root.join("src"));

    let parsed_src = parse_all(&src_files);
    let parsed_tests = parse_all(&test_files);

    // Pass 1: const aliases and external `mod tests;` targets, from every
    // file (src and tests: a const could in principle live in either).
    let (mut consts, external_test_files) = collect_consts_and_test_mods(&parsed_src);
    let (test_consts, _) = collect_consts_and_test_mods(&parsed_tests);
    consts.extend(test_consts);

    // Pass 2: classify.
    let mut emitted = BTreeSet::new();
    let mut asserted = BTreeSet::new();
    for file in &parsed_src {
        let whole_file_is_test = external_test_files.contains(&file.path);
        classify_file(
            file,
            whole_file_is_test,
            &consts,
            &mut emitted,
            &mut asserted,
        );
    }
    for file in &parsed_tests {
        classify_file(file, true, &consts, &mut emitted, &mut asserted);
    }

    (emitted, asserted)
}

/// Result of [`scan_codebase`], computed once per test binary and shared
/// by both `#[test]`s below (each doing its own full scan would parse the
/// whole repo twice).
static SCAN: std::sync::LazyLock<(BTreeSet<String>, BTreeSet<String>)> =
    std::sync::LazyLock::new(scan_codebase);

#[test]
fn test_every_emitted_code_is_tested_or_allowlisted() {
    let (emitted, asserted) = &*SCAN;
    let allowlist: BTreeSet<&str> = UNCOVERED_ALLOWLIST.iter().copied().collect();

    let undocumented: Vec<&String> = emitted
        .iter()
        .filter(|code| !asserted.contains(code.as_str()) && !allowlist.contains(code.as_str()))
        .collect();
    assert!(
        undocumented.is_empty(),
        "emitted CAIRN_* codes with no test and no UNCOVERED_ALLOWLIST entry \
         (tests/finding_code_coverage.rs): {undocumented:#?}\n\
         Either add a test that triggers the code, or add it to \
         UNCOVERED_ALLOWLIST with a reason."
    );

    // Burn-down hygiene: an allowlist entry for a code that is no longer
    // emitted, or that a test now covers, is stale and must be removed.
    let stale: Vec<&&str> = UNCOVERED_ALLOWLIST
        .iter()
        .filter(|code| !emitted.contains(**code) || asserted.contains(**code))
        .collect();
    assert!(
        stale.is_empty(),
        "UNCOVERED_ALLOWLIST entries that are stale (no longer emitted, or \
         now covered by a test): {stale:#?}\nRemove them from the allowlist."
    );
}

/// True for a real registry allocation row: `- CXNNN -- ...` (a leading
/// `CXNNN` code, e.g. `CK001`, right after the list-item marker). Filters
/// out headings, the `## Rules` prose, and any other line that merely
/// mentions a `CAIRN_*` name without allocating it a row.
fn is_registry_row(line: &str) -> bool {
    let Some(rest) = line.trim_start().strip_prefix("- ") else {
        return false;
    };
    let bytes = rest.as_bytes();
    bytes.len() > 5
        && bytes[0] == b'C'
        && bytes[1].is_ascii_uppercase()
        && bytes[2].is_ascii_digit()
        && bytes[3].is_ascii_digit()
        && bytes[4].is_ascii_digit()
        && bytes[5] == b' '
}

/// Extracts each parenthesised `CAIRN_*` token from valid registry rows.
/// This guarantees exact token matching rather than substring checks.
fn registered_codes(registry: &str) -> BTreeSet<String> {
    let mut codes = BTreeSet::new();
    for line in registry.lines().filter(|l| is_registry_row(l)) {
        // Look for the `(CAIRN_...)` parenthetical token
        if let Some(start_idx) = line.find("(CAIRN_") {
            let candidate = &line[start_idx + 1..];
            if let Some(end_idx) = candidate.find(')') {
                let token = &candidate[..end_idx];
                if is_cairn_code(token) {
                    codes.insert(token.to_owned());
                }
            }
        }
    }
    codes
}

#[test]
fn test_every_emitted_code_has_a_registry_entry() {
    let (emitted, _asserted) = &*SCAN;
    let registry_path = repo_root().join("docs/registries/error-codes.md");
    let registry = fs::read_to_string(&registry_path).expect("error-codes.md must be readable");
    let registered = registered_codes(&registry);

    let unregistered: Vec<&String> = emitted
        .iter()
        .filter(|code| !registered.contains(*code))
        .collect();
    assert!(
        unregistered.is_empty(),
        "emitted CAIRN_* codes with no allocated row in docs/registries/error-codes.md: \
         {unregistered:#?}\nAdd a `- CXNNN -- description (CODE) -- phase` row for each \
         under the matching category."
    );
}

/// Classifier fixtures: a predicate comparison counts as coverage, a dead
/// statement-level comparison does not.
#[test]
fn predicate_comparison_counts_dead_comparison_does_not() {
    let consts = BTreeMap::new();
    let classify_source = |source: &str| {
        let tree = parse_rust(source).expect("fixture must parse");
        assert!(!tree.root_node().has_error(), "fixture must be valid Rust");
        let parsed = Parsed {
            path: PathBuf::from("fixture.rs"),
            source: source.to_owned(),
            tree,
        };
        let mut emitted = BTreeSet::new();
        let mut asserted = BTreeSet::new();
        classify_file(&parsed, true, &consts, &mut emitted, &mut asserted);
        asserted
    };

    let covered = classify_source(
        r#"fn t(findings: Vec<Finding>) {
            findings
                .iter()
                .find(|f| f.code == "CAIRN_FIXTURE_PREDICATE")
                .expect("finding");
        }"#,
    );
    assert!(
        covered.contains("CAIRN_FIXTURE_PREDICATE"),
        "closure predicate comparison must count as coverage: {covered:?}"
    );

    let dead = classify_source(
        r#"fn t(f: &Finding) {
            let _ = f.code == "CAIRN_FIXTURE_DEAD";
        }"#,
    );
    assert!(
        !dead.contains("CAIRN_FIXTURE_DEAD"),
        "statement-level dead comparison must not count as coverage: {dead:?}"
    );

    let mapped = classify_source(
        r#"fn t(findings: Vec<Finding>) {
            let _flags: Vec<bool> = findings
                .iter()
                .map(|f| f.code == "CAIRN_FIXTURE_MAPPED")
                .collect();
        }"#,
    );
    assert!(
        !mapped.contains("CAIRN_FIXTURE_MAPPED"),
        "comparison inside map() must not count as coverage: {mapped:?}"
    );
}
