//! Contract asserted-numeral drift: verifies opt-in `` `NAME = N` `` assertions
//! in contract prose against the constant's literal in the owning node's
//! claimed Rust files.
//!
//! The source-of-truth mapping is explicit and opt-in: a contract binds a
//! numeral to code by writing an inline code span of the form `NAME = N`,
//! where `NAME` is a `SCREAMING_SNAKE` constant name and `N` an unsigned
//! integer. The asserted value is compared against every `const NAME`
//! integer literal in the node's claimed `.rs` files. Prose that merely
//! names version semantics (`v1-to-v2`, `schema-v2`, protocol dates, or a
//! `NAME: u32` type note) never matches the form and is never flagged, so
//! stable migration and protocol-version obligations are preserved.
//!
//! Source extraction is syntax-aware: claimed files are parsed with the
//! tree-sitter Rust grammar and only real `const_item` declarations count,
//! so a `const NAME` shape inside a string literal, comment, or
//! const-generic parameter list is never a source.

use std::path::Path;

use crate::artefacts::contract::ContractSet;
use crate::map::graph::{Finding, FindingSeverity, Graph};

/// One opt-in assertion extracted from contract prose.
struct Assertion {
    /// Constant name as asserted.
    name: String,
    /// Asserted value.
    value: u64,
}

/// Emits `CAIRN_CONTRACT_NUMERAL_DRIFT` for every contract assertion whose
/// constant is absent from, or disagrees with, the owning node's claimed
/// Rust files.
///
/// When one name has several declarations (cfg variants), the assertion
/// passes if ANY declared value matches: the check flags a value no
/// declaration carries, never adjudicates between variants. Each name is
/// flagged at most once per contract.
pub(crate) fn check_contract_numeral_drift(
    graph: &mut Graph,
    contracts: &ContractSet,
    root: &Path,
) {
    // Buffered: `graph.nodes` is borrowed inside the loop, so pushing
    // directly into `graph.findings` would conflict with that borrow.
    let mut findings = Vec::new();
    for contract in contracts.contracts.values() {
        let assertions = extract_assertions(&contract.body);
        if assertions.is_empty() {
            continue;
        }
        let Some(node) = graph.nodes.get(&contract.declared_by) else {
            continue;
        };
        let mut sources = Vec::new();
        for file in &node.files {
            if !Path::new(file)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("rs"))
            {
                continue;
            }
            if let Ok(content) = std::fs::read_to_string(root.join(file)) {
                sources.push(content);
            }
        }
        let mut flagged = std::collections::BTreeSet::new();
        for assertion in assertions {
            let declared = const_values(&sources, &assertion.name);
            if declared.contains(&assertion.value) || !flagged.insert(assertion.name.clone()) {
                continue;
            }
            let asserted = assertion.value.to_string();
            let body_key = if declared.is_empty() {
                "findings.codes.CAIRN_CONTRACT_NUMERAL_DRIFT.body_missing"
            } else {
                "findings.codes.CAIRN_CONTRACT_NUMERAL_DRIFT.body"
            };
            let actual = declared
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join(", ");
            findings.push(Finding {
                code: "CAIRN_CONTRACT_NUMERAL_DRIFT".to_owned(),
                severity: FindingSeverity::Warning,
                message: crate::copy::lookup(body_key)
                    .replace("{node}", &contract.declared_by)
                    .replace("{name}", &assertion.name)
                    .replace("{asserted}", &asserted)
                    .replace("{actual}", &actual),
                node: Some(contract.declared_by.clone()),
                target: Some(assertion.name),
                path: Some(contract.path.clone()),
                deferred_by: None,
                parked_by: None,
            });
        }
    }
    graph.findings.extend(findings);
}

/// Extracts `NAME = N` assertions from inline code spans in `body`,
/// skipping fenced code blocks.
///
/// A fence opens on a line starting with a run of three or more backticks
/// or tildes and closes only on a run of the same character at least as
/// long, per `CommonMark`; a mismatched delimiter line inside a fence is
/// content.
fn extract_assertions(body: &str) -> Vec<Assertion> {
    let mut assertions = Vec::new();
    let mut fence: Option<(char, usize)> = None;
    for line in body.lines() {
        let marker = line.trim_start();
        let delimiter = marker
            .chars()
            .next()
            .filter(|c| matches!(c, '`' | '~'))
            .map(|c| (c, marker.chars().take_while(|x| *x == c).count()));
        if let Some((char_, run)) = delimiter
            && run >= 3
        {
            match fence {
                None => fence = Some((char_, run)),
                Some((open, len)) if open == char_ && run >= len => fence = None,
                Some(_) => {}
            }
            continue;
        }
        if fence.is_some() {
            continue;
        }
        for span in inline_code_spans(line) {
            if let Some(assertion) = parse_assertion(span) {
                assertions.push(assertion);
            }
        }
    }
    assertions
}

/// Contents of closed single-backtick code spans on one line. An unclosed
/// trailing backtick opens no span, so its tail is never parsed.
fn inline_code_spans(line: &str) -> Vec<&str> {
    let parts: Vec<&str> = line.split('`').collect();
    // Balanced backticks give an odd part count; an even count means the
    // final part follows an unclosed backtick and is not a span.
    let closed = parts.len() - usize::from(parts.len().is_multiple_of(2));
    parts[..closed].iter().skip(1).step_by(2).copied().collect()
}

/// Parses `NAME = N`: a `SCREAMING_SNAKE` name (at least three characters,
/// containing an underscore), one `=`, and an unsigned integer.
fn parse_assertion(span: &str) -> Option<Assertion> {
    let (name, value) = span.split_once('=')?;
    let name = name.trim();
    let value = value.trim();
    let named_const = name.len() >= 3
        && name.contains('_')
        && name.starts_with(|c: char| c.is_ascii_uppercase())
        && name
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');
    if !named_const || value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    Some(Assertion {
        name: name.to_owned(),
        value: value.parse().ok()?,
    })
}

/// Every decimal integer literal assigned to a `const <name>` item across
/// `sources`, extracted from the tree-sitter Rust parse.
///
/// Only an initialiser that is a plain decimal `integer_literal` (digits,
/// optional `_` separators, optional integer type suffix) counts. Anything
/// else (hex, expressions, other constants) is skipped: it is not a
/// deterministic source for this check.
fn const_values(sources: &[String], name: &str) -> Vec<u64> {
    let mut parser = tree_sitter::Parser::new();
    if parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .is_err()
    {
        return Vec::new();
    }
    let mut values = Vec::new();
    for source in sources {
        let Some(tree) = parser.parse(source, None) else {
            continue;
        };
        collect_const_values(tree.root_node(), source.as_bytes(), name, &mut values);
    }
    values
}

/// Recursively collects matching `const_item` literal values under `node`.
fn collect_const_values(node: tree_sitter::Node, source: &[u8], name: &str, values: &mut Vec<u64>) {
    if node.kind() == "const_item"
        && node
            .child_by_field_name("name")
            .and_then(|n| n.utf8_text(source).ok())
            == Some(name)
    {
        if let Some(value) = node
            .child_by_field_name("value")
            .filter(|n| n.kind() == "integer_literal")
            .and_then(|n| n.utf8_text(source).ok())
            .and_then(parse_decimal_literal)
        {
            values.push(value);
        }
        return;
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_const_values(child, source, name, values);
    }
}

/// Parses a whole Rust decimal integer literal: digits with optional `_`
/// separators and an optional `u*`/`i*` type suffix. Returns `None` for
/// anything else.
fn parse_decimal_literal(literal: &str) -> Option<u64> {
    let digits_end = literal
        .find(|c: char| !c.is_ascii_digit() && c != '_')
        .unwrap_or(literal.len());
    let (digits, suffix) = literal.split_at(digits_end);
    let valid_suffix = matches!(
        suffix,
        "" | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "usize"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "isize"
    );
    if digits.is_empty() || !valid_suffix {
        return None;
    }
    digits.replace('_', "").parse().ok()
}
