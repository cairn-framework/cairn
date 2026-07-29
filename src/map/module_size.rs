//! Module-size integrity check.
//!
//! Emits [`CAIRN_MODULE_OVERSIZED`] when a node-owned claimed file exceeds
//! [`MODULE_SIZE_LIMIT_LINES`] lines without a `cairn:allow-large-module`
//! marker on its first non-blank line. Mirrors the allow-marker protocol
//! enforced in CI by `scripts/check-file-sizes.sh` byte-for-byte: keep the
//! threshold, marker syntax, extension case-sensitivity, and vendor-path
//! scoping in sync between the two if either changes. v1 checks file size
//! only; fan-in/fan-out quotas and import-graph edge realisation are out of
//! scope (see `meta/todos/todo.modularity-scan-finding.md`).
//!
//! Parity notes with the shell gate:
//! - Extensions are matched case-sensitively (`rs`/`js`/`css`, not
//!   `.eq_ignore_ascii_case`), mirroring `find -name '*.rs'` etc.
//! - The `vendor/` path-segment exclusion applies only to `.js`/`.css`
//!   (mirroring the shell gate's `! -path '*/vendor/*'`, which is scoped to
//!   the `ui_assets` JS/CSS `find` loops only). Rust files under a
//!   `vendor/` path are still checked, matching the shell gate's unscoped
//!   `$root/src -name '*.rs'` loop.
//! - Content is read as raw bytes, not a UTF-8 `String`: a non-UTF-8
//!   claimed file is still counted (matching `wc -l` byte semantics) and
//!   marker parsing splits on raw `\n` bytes without stripping `\r`, so a
//!   CRLF-terminated CSS marker line fails the exact-suffix match exactly
//!   as the shell gate's `case "$first_nonblank" in "$prefix"*"$suffix")`
//!   glob would.
//!
//! Severity is Warning. It started as Info because `cairn.tests`-owned
//! files under `tests/` were oversized and unmarked (the shell gate never
//! discovers `tests/`, so none carried a marker). `todo.oversized-test-file-baseline`
//! cleared that baseline: every previously oversized `tests/*.rs` file was
//! resolved (marked or split below the limit), so `cairn scan --strict`
//! stays green on a clean checkout and Warning now blocks any newly
//! introduced unmarked oversized claimed file project-wide. The shell gate
//! discovers Rust only under `src/`, while JS/CSS are also walked from
//! blueprint-declared paths; it never discovers `tests/*.rs`.

use std::{fs, path::Path};

use super::graph::{Finding, FindingSeverity, Graph};

/// Maximum lines a claimed `.rs`/`.js`/`.css` file may hold before it needs
/// an allow-list marker. Mirrored by `scripts/check-file-sizes.sh`; keep
/// both in sync if the limit ever changes.
const MODULE_SIZE_LIMIT_LINES: usize = 500;

/// Allow-list marker delimiters, keyed by file extension. Mirrors
/// `scripts/check-file-sizes.sh::check_file`'s `prefix`/`suffix` pair.
#[derive(Clone, Copy)]
enum MarkerStyle {
    /// `// cairn:allow-large-module reason: ...` (Rust and JS).
    LineComment,
    /// `/* cairn:allow-large-module reason: ... */` (CSS, single line).
    BlockComment,
}

const LINE_COMMENT_PREFIX: &[u8] = b"// cairn:allow-large-module reason:";
const BLOCK_COMMENT_PREFIX: &[u8] = b"/* cairn:allow-large-module reason:";
const BLOCK_COMMENT_SUFFIX: &[u8] = b"*/";

/// Marker style and vendor-exclusion applicability for a policed extension.
/// `None` for any extension the shell gate does not police (no marker
/// syntax is defined for it).
fn policy_for(file: &str) -> Option<(MarkerStyle, bool)> {
    // Case-sensitive: mirrors `find -name '*.rs'` etc., which never matches
    // `.RS`/`.JS`/`.CSS` on a case-sensitive filesystem.
    match Path::new(file).extension()?.to_str()? {
        "rs" => Some((MarkerStyle::LineComment, false)),
        "js" => Some((MarkerStyle::LineComment, true)),
        "css" => Some((MarkerStyle::BlockComment, true)),
        _ => None,
    }
}

/// True when `file` has a `vendor` path segment, mirroring
/// `scripts/check-file-sizes.sh`'s `! -path '*/vendor/*'` exclusion.
fn is_vendored(file: &str) -> bool {
    Path::new(file)
        .components()
        .any(|component| component.as_os_str() == "vendor")
}

/// True when `line` is blank per `awk`'s default field splitting (space
/// and tab only). A lone `\r` byte (the remainder of a CRLF blank line
/// after splitting on `\n`) is NOT blank under awk's default `FS`, so
/// `awk 'NF { print; exit }'` would pick it as the "first non-blank line"
/// rather than skipping past it to a real marker below. This predicate
/// reproduces that exact selection quirk.
fn is_awk_field_blank(line: &[u8]) -> bool {
    line.iter().all(|&b| b == b' ' || b == b'\t')
}

/// True when `bytes` is entirely ASCII whitespace (including empty),
/// mirroring the shell gate's `sed 's/^[[:space:]]*//; s/[[:space:]]*$//'`
/// trim (POSIX `[[:space:]]` includes `\r`, unlike awk's default `FS`).
fn is_reason_blank(bytes: &[u8]) -> bool {
    bytes.iter().all(u8::is_ascii_whitespace)
}

/// True when `content`'s first non-blank line (per [`is_awk_field_blank`],
/// split on raw `\n` with `\r` preserved) carries a valid allow-list
/// marker for `style`: the exact prefix (and, for block comments, the
/// exact trailing suffix) with a non-blank reason in between. Mirrors
/// `scripts/check-file-sizes.sh::check_file`'s `case` match precisely,
/// including the "matched prefix but empty reason still fails" branch and
/// CRLF sensitivity (a trailing `\r` before the CSS suffix breaks the
/// match, exactly as the shell glob would).
fn has_allow_marker(content: &[u8], style: MarkerStyle) -> bool {
    let Some(first) = content
        .split(|&b| b == b'\n')
        .find(|line| !is_awk_field_blank(line))
    else {
        return false;
    };
    let rest = match style {
        MarkerStyle::LineComment => first.strip_prefix(LINE_COMMENT_PREFIX),
        MarkerStyle::BlockComment => first
            .strip_prefix(BLOCK_COMMENT_PREFIX)
            .and_then(|r| r.strip_suffix(BLOCK_COMMENT_SUFFIX)),
    };
    rest.is_some_and(|r| !is_reason_blank(r))
}

/// Emits `CAIRN_MODULE_OVERSIZED` for claimed source/asset files over
/// [`MODULE_SIZE_LIMIT_LINES`] lines with no valid allow-list marker.
pub(crate) fn validate_module_sizes(graph: &mut Graph, root: &Path) {
    for node in graph.nodes.values() {
        for file in &node.files {
            let Some((style, vendor_excluded)) = policy_for(file) else {
                continue;
            };
            if vendor_excluded && is_vendored(file) {
                continue;
            }
            let Ok(bytes) = fs::read(root.join(file)) else {
                continue;
            };
            // Matches `wc -l` exactly: count newline bytes. `split` avoids
            // clippy::naive_bytecount's iter/filter/count pattern; a buffer
            // with N newlines splits into N+1 segments.
            let lines = bytes.split(|&b| b == b'\n').count() - 1;
            if lines <= MODULE_SIZE_LIMIT_LINES || has_allow_marker(&bytes, style) {
                continue;
            }
            graph.findings.push(Finding {
                code: "CAIRN_MODULE_OVERSIZED".to_owned(),
                severity: FindingSeverity::Warning,
                message: format!(
                    "module `{}` claims `{file}` at {lines} lines, over the {MODULE_SIZE_LIMIT_LINES}-line guideline with no allow-list marker",
                    node.id
                ),
                node: Some(node.id.clone()),
                target: None,
                path: Some(file.clone()),
                deferred_by: None,
                parked_by: None,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{
        artefacts::contract::ContractSet,
        blueprint::{Ast, Edge, Node, NodeKind, Span},
        map::{build_graph, graph::FindingSeverity},
    };

    use super::*;

    fn span() -> Span {
        Span::point("test.blueprint", 1, 1)
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

    fn ast(nodes: Vec<Node>, edges: Vec<Edge>) -> Ast {
        Ast { nodes, edges }
    }

    fn codes(g: &Graph) -> Vec<&str> {
        g.findings.iter().map(|f| f.code.as_str()).collect()
    }

    fn build_with_files(
        root: &std::path::Path,
        a: &Ast,
        claimed: &mut BTreeMap<String, Vec<String>>,
    ) -> Graph {
        build_graph(a, root, &ContractSet::default(), claimed, Vec::new())
    }

    #[test]
    fn module_size_missing_marker_emits_finding() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        // 501 newlines: over the 500-line limit, no allow-list marker.
        std::fs::write(root.join("big.rs"), "x\n".repeat(501)).unwrap();
        let a = ast(vec![leaf("app.api")], vec![]);
        let mut claimed = BTreeMap::new();
        claimed.insert("app.api".to_owned(), vec!["big.rs".to_owned()]);
        let g = build_with_files(root, &a, &mut claimed);
        let flagged = codes(&g);
        assert!(
            flagged.contains(&"CAIRN_MODULE_OVERSIZED"),
            "unmarked oversized module must be flagged: {flagged:?}"
        );
        let finding = g
            .findings
            .iter()
            .find(|f| f.code == "CAIRN_MODULE_OVERSIZED")
            .unwrap();
        assert_eq!(finding.severity, FindingSeverity::Warning);
        assert_eq!(finding.node.as_deref(), Some("app.api"));
        assert_eq!(finding.path.as_deref(), Some("big.rs"));
    }

    #[test]
    fn module_size_allow_marker_suppresses_finding() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let mut content =
            "// cairn:allow-large-module reason: fixture for allow-marker suppression\n".to_owned();
        content.push_str(&"x\n".repeat(510));
        std::fs::write(root.join("big.rs"), content).unwrap();
        let a = ast(vec![leaf("app.api")], vec![]);
        let mut claimed = BTreeMap::new();
        claimed.insert("app.api".to_owned(), vec!["big.rs".to_owned()]);
        let g = build_with_files(root, &a, &mut claimed);
        assert!(
            !codes(&g).contains(&"CAIRN_MODULE_OVERSIZED"),
            "allow-marked oversized module must not be flagged: {:?}",
            codes(&g)
        );
    }

    #[test]
    fn module_size_css_block_marker_suppresses_finding() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let mut content =
            "/* cairn:allow-large-module reason: fixture for CSS marker */\n".to_owned();
        content.push_str(&"x {}\n".repeat(510));
        std::fs::write(root.join("big.css"), content).unwrap();
        let a = ast(vec![leaf("app.ui")], vec![]);
        let mut claimed = BTreeMap::new();
        claimed.insert("app.ui".to_owned(), vec!["big.css".to_owned()]);
        let g = build_with_files(root, &a, &mut claimed);
        assert!(
            !codes(&g).contains(&"CAIRN_MODULE_OVERSIZED"),
            "CSS allow-marked oversized module must not be flagged: {:?}",
            codes(&g)
        );
    }

    #[test]
    fn module_size_vendor_path_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::create_dir_all(root.join("vendor")).unwrap();
        // 501 newlines, unmarked, but under a vendor/ path segment. JS is
        // vendor-excluded, mirroring the shell gate's ui_assets JS/CSS loops.
        std::fs::write(root.join("vendor/lib.js"), "x\n".repeat(501)).unwrap();
        let a = ast(vec![leaf("app.ui")], vec![]);
        let mut claimed = BTreeMap::new();
        claimed.insert("app.ui".to_owned(), vec!["vendor/lib.js".to_owned()]);
        let g = build_with_files(root, &a, &mut claimed);
        assert!(
            !codes(&g).contains(&"CAIRN_MODULE_OVERSIZED"),
            "vendor-pathed .js file must be skipped: {:?}",
            codes(&g)
        );
    }

    #[test]
    fn module_size_vendor_path_rust_still_flagged() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::create_dir_all(root.join("vendor")).unwrap();
        // 501 newlines, unmarked, under a vendor/ path segment. Rust is NOT
        // vendor-excluded: the shell gate's `$root/src -name '*.rs'` loop
        // has no vendor exclusion, unlike the ui_assets JS/CSS loops.
        std::fs::write(root.join("vendor/lib.rs"), "x\n".repeat(501)).unwrap();
        let a = ast(vec![leaf("app.api")], vec![]);
        let mut claimed = BTreeMap::new();
        claimed.insert("app.api".to_owned(), vec!["vendor/lib.rs".to_owned()]);
        let g = build_with_files(root, &a, &mut claimed);
        assert!(
            codes(&g).contains(&"CAIRN_MODULE_OVERSIZED"),
            "vendor-pathed .rs file must still be flagged, matching the shell gate: {:?}",
            codes(&g)
        );
    }

    #[test]
    fn module_size_crlf_marker_not_recognised() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        // CRLF-terminated CSS marker: the shell gate's `case` glob requires
        // the line to end with the literal "*/" suffix, which a trailing
        // \r breaks. Byte-level parsing here must reject it the same way.
        let mut content = b"/* cairn:allow-large-module reason: crlf fixture */\r\n".to_vec();
        content.extend_from_slice(&b"x {}\n".repeat(510));
        std::fs::write(root.join("big.css"), content).unwrap();
        let a = ast(vec![leaf("app.ui")], vec![]);
        let mut claimed = BTreeMap::new();
        claimed.insert("app.ui".to_owned(), vec!["big.css".to_owned()]);
        let g = build_with_files(root, &a, &mut claimed);
        assert!(
            codes(&g).contains(&"CAIRN_MODULE_OVERSIZED"),
            "CRLF-terminated marker line must NOT suppress the finding (shell parity): {:?}",
            codes(&g)
        );
    }

    #[test]
    fn module_size_leading_blank_crlf_does_not_skip_to_marker() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        // awk treats the `\r` remainder of a blank CRLF record as a
        // non-blank first record, so it never reaches this later marker.
        let mut content = b"\r\n/* cairn:allow-large-module reason: later marker */\n".to_vec();
        content.extend_from_slice(&b"x {}\n".repeat(510));
        std::fs::write(root.join("big.css"), content).unwrap();
        let a = ast(vec![leaf("app.ui")], vec![]);
        let mut claimed = BTreeMap::new();
        claimed.insert("app.ui".to_owned(), vec!["big.css".to_owned()]);
        let g = build_with_files(root, &a, &mut claimed);
        assert!(
            codes(&g).contains(&"CAIRN_MODULE_OVERSIZED"),
            "leading blank CRLF record must prevent reaching later marker: {:?}",
            codes(&g)
        );
    }

    #[test]
    fn module_size_exactly_at_limit_not_flagged() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        // Exactly 500 newlines: at the limit, not over it.
        std::fs::write(root.join("big.rs"), "x\n".repeat(500)).unwrap();
        let a = ast(vec![leaf("app.api")], vec![]);
        let mut claimed = BTreeMap::new();
        claimed.insert("app.api".to_owned(), vec!["big.rs".to_owned()]);
        let g = build_with_files(root, &a, &mut claimed);
        assert!(
            !codes(&g).contains(&"CAIRN_MODULE_OVERSIZED"),
            "file exactly at the limit must not be flagged: {:?}",
            codes(&g)
        );
    }

    #[test]
    fn module_size_non_policed_extension_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("data.json"), "x\n".repeat(501)).unwrap();
        let a = ast(vec![leaf("app.data")], vec![]);
        let mut claimed = BTreeMap::new();
        claimed.insert("app.data".to_owned(), vec!["data.json".to_owned()]);
        let g = build_with_files(root, &a, &mut claimed);
        assert!(
            !codes(&g).contains(&"CAIRN_MODULE_OVERSIZED"),
            "extension with no defined marker syntax must be skipped: {:?}",
            codes(&g)
        );
    }
}
