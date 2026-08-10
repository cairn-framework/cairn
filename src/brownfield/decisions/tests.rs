//! Unit tests for the decision-evidence index predicates and the owner resolver.

use super::collect::{evidence_headings, heading_text, invariant_detail, is_evidence_heading};
use super::*;

#[test]
fn evidence_headings_are_the_closed_set_only() {
    assert!(is_evidence_heading("Decision"));
    assert!(is_evidence_heading("rationale"));
    assert!(is_evidence_heading("Invariant:"));
    assert!(
        !is_evidence_heading("Decisions"),
        "the set is exact, not a prefix match"
    );
    assert!(!is_evidence_heading("Design decision"));
}

#[test]
fn heading_text_follows_commonmark_atx_rules() {
    assert_eq!(heading_text("## Decision ##"), Some("Decision"));
    assert_eq!(heading_text("  # Title"), Some("Title"));
    assert_eq!(heading_text("not a heading"), None);
    assert_eq!(
        heading_text("#Decision"),
        None,
        "ATX requires whitespace after the markers"
    );
    assert_eq!(
        heading_text("####### Decision"),
        None,
        "seven markers is not a heading"
    );
    assert_eq!(
        heading_text("## Decision###"),
        Some("Decision###"),
        "a closing run counts only when whitespace separates it, so this is not the closed set"
    );
    assert!(
        !is_evidence_heading(heading_text("## Decision###").expect("a heading")),
        "and therefore it is not evidence"
    );
}

#[test]
fn evidence_headings_skip_fenced_code_and_read_setext() {
    let text = "# Demo\n\n```\n## Decision\n```\n\nDecision\n========\n\n## Rationale\n";
    let found = evidence_headings(text);
    assert_eq!(
        found,
        vec![(6, "Decision".to_owned()), (9, "Rationale".to_owned())],
        "a heading inside a fence is an example, not evidence; a setext heading is evidence"
    );
}

#[test]
fn evidence_headings_respect_info_strings_and_indented_code() {
    // An info-string line opens a fence and must not close one, or the fenced
    // heading below would be indexed.
    let fenced = "```rust\n// sample\n```\n\n## Decision\n";
    assert_eq!(
        evidence_headings(fenced),
        vec![(4, "Decision".to_owned())],
        "only the heading outside the fence is evidence"
    );

    let info_string_close = "```\n```foo\n## Decision\n```\n";
    assert_eq!(
        evidence_headings(info_string_close),
        Vec::<(usize, String)>::new(),
        "a delimiter run with trailing text does not close the fence"
    );

    let indented = "    # Decision\n\n    Decision\n    ========\n";
    assert_eq!(
        evidence_headings(indented),
        Vec::<(usize, String)>::new(),
        "four leading spaces is indented code, not a heading"
    );
}

#[test]
fn invariant_detail_reads_both_markers_and_nothing_else() {
    assert_eq!(
        invariant_detail("    // invariant: ids are stable", false),
        Some("ids are stable")
    );
    assert_eq!(
        invariant_detail("# invariant: rows are sorted", true),
        Some("rows are sorted")
    );
    assert_eq!(
        invariant_detail("let x = 1; // invariant: x never shrinks", false),
        Some("x never shrinks"),
        "a comment after code on the same line still carries the marker"
    );
    assert_eq!(
        invariant_detail("x = 1  # invariant: rows stay sorted", true),
        Some("rows stay sorted")
    );
    assert_eq!(invariant_detail("// note: not an invariant", false), None);
    assert_eq!(
        invariant_detail("// invariant is spelled without a colon", false),
        None
    );
}

#[test]
fn invariant_detail_ignores_a_marker_that_does_not_open_a_comment() {
    assert_eq!(
        invariant_detail(
            r##"const MARKERS: &[&str] = &["// invariant:", "# invariant:"];"##,
            false
        ),
        None,
        "this module's own marker table must not index itself"
    );
    assert_eq!(
        invariant_detail(r##"    let line = "# invariant: not evidence";"##, false),
        None,
        "a marker inside a string literal is not a comment"
    );
    assert_eq!(
        invariant_detail("let u = 'http:// invariant: foo';", true),
        None,
        "a URL scheme colon before the marker means it is not a comment"
    );
    assert_eq!(
        invariant_detail("see http:// invariant: foo in prose", true),
        None,
        "prose quoting a URL is not a comment"
    );
    assert_eq!(
        invariant_detail("let s = 'not // invariant: prose';", true),
        None,
        "an odd apostrophe count means the marker sits inside a quoted string"
    );
    assert_eq!(
        invariant_detail("let s = `not // invariant: prose`;", true),
        None,
        "backtick strings are handled the same way"
    );
    assert_eq!(
        invariant_detail("fn f<'a>(x: &str) { // invariant: still evidence", false),
        Some("still evidence"),
        "a Rust lifetime is not a quote, so apostrophes are not counted there"
    );
    assert_eq!(
        invariant_detail("/// A doc comment mentioning // invariant: in prose", false),
        None
    );
    assert_eq!(
        invariant_detail("//! invariant: not this form either", false),
        None
    );
}

#[test]
fn owner_of_reports_an_equally_specific_tie_as_unbound() {
    // Two nodes declaring the same path: `new` normalises `./src` and `src` to
    // the same string, so both contend at the same specificity. The reconciler
    // breaks that tie in declaration order, which the graph does not preserve,
    // so no binding is invented.
    let tie = OwnerResolver {
        owners: vec![("app.a", "src".to_owned()), ("app.z", "src".to_owned())],
    };
    assert_eq!(tie.owner_of("src/main.rs"), None);
    assert_eq!(
        tie.owner_of("docs/readme.md"),
        None,
        "a tie on one path does not affect unrelated paths"
    );

    let single = OwnerResolver {
        owners: vec![("app.a", "src".to_owned())],
    };
    assert_eq!(single.owner_of("src/main.rs"), Some("app.a"));

    // One node declaring the same path twice is not a tie.
    let same_node = OwnerResolver {
        owners: vec![("app.a", "src".to_owned()), ("app.a", "src".to_owned())],
    };
    assert_eq!(same_node.owner_of("src/main.rs"), Some("app.a"));

    // A more specific path still wins over a shorter one.
    let nested = OwnerResolver {
        owners: vec![
            ("app.core", "src/core".to_owned()),
            ("app.root", "src".to_owned()),
        ],
    };
    assert_eq!(nested.owner_of("src/core/a.rs"), Some("app.core"));
}

#[test]
fn owner_paths_are_normalised_and_sorted_most_specific_first() {
    // What `new` must produce from declared `./src` and `./src/core`: the `./`
    // stripped, and the longer path considered first.
    let resolver = OwnerResolver {
        owners: vec![
            ("app.core", crate::map::paths::trim_dot("./src/core")),
            ("app.root", crate::map::paths::trim_dot("./src")),
        ],
    };
    assert_eq!(resolver.owner_of("src/core/a.rs"), Some("app.core"));
    assert_eq!(resolver.owner_of("src/main.rs"), Some("app.root"));
    assert_eq!(
        resolver.owner_of("src_generated/a.rs"),
        None,
        "containment stops at a component boundary"
    );
}
