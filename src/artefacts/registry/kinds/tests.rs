//! Tests for the data-driven artefact kind table.

use super::*;

#[test]
fn artefact_kinds_cover_five_pointer_fields() {
    let pointers: Vec<&str> = ARTEFACT_KINDS.iter().map(|k| k.pointer).collect();
    assert_eq!(
        pointers,
        vec!["todos", "decisions", "reviews", "research", "sources"]
    );
}

#[test]
fn load_one_todo_scalar_defers_is_malformed_not_silent() {
    // A scalar `defers: CODE path` reaches only frontmatter `values`, so
    // without the guard it would be a silent no-op instead of CA043.
    let parsed = crate::artefacts::frontmatter::parse(
        "---\nnode: app.api\nstatus: blocked\ncreated: 2026-07-29\ndefers: CAIRN_TEST src/lib.rs\n---\nbody\n",
    );
    let mut set = ArtefactSet::default();
    load_one_todo(Path::new("meta/todos/todo.scalar.md"), &parsed, &mut set);
    assert_eq!(set.todos.len(), 1);
    assert!(set.todos[0].defers.is_empty());
    assert_eq!(set.findings.len(), 1);
    assert_eq!(set.findings[0].code, "CAIRN_TODO_DEFERS_INVALID");
}

#[test]
fn load_one_todo_inline_list_defers_parses() {
    // The inline `[..]` form populates both frontmatter maps and must not
    // trip the scalar guard.
    let parsed = crate::artefacts::frontmatter::parse(
        "---\nnode: app.api\nstatus: blocked\ncreated: 2026-07-29\ndefers: [CAIRN_TEST src/lib.rs]\n---\nbody\n",
    );
    let mut set = ArtefactSet::default();
    load_one_todo(Path::new("meta/todos/todo.inline.md"), &parsed, &mut set);
    assert_eq!(set.findings.len(), 0, "{:?}", set.findings);
    assert_eq!(set.todos[0].defers.len(), 1);
    assert_eq!(set.todos[0].defers[0].code, "CAIRN_TEST");
    assert_eq!(set.todos[0].defers[0].location, "src/lib.rs");
}

fn decision(extra: &str) -> crate::artefacts::frontmatter::Frontmatter {
    crate::artefacts::frontmatter::parse(&format!(
        "---\nid: dec.test\nstatus: proposed\ndate: 2026-07-30\nnodes: [app.api]\n{extra}---\nbody\n"
    ))
}

fn review(extra: &str) -> crate::artefacts::frontmatter::Frontmatter {
    crate::artefacts::frontmatter::parse(&format!(
        "---\nnode: app.api\ndate: 2026-07-30\nreviewer: model/lens\n{extra}---\nbody\n"
    ))
}

#[test]
fn test_decision_all_ratification_fields_parse() {
    let mut set = ArtefactSet::default();
    load_one_decision(
        Path::new("meta/decisions/dec.test.md"),
        &decision(
            "ratification: local\nratified_by: machine\naffects: [src/lib.rs]\nreceipts: [rev.test]\n",
        ),
        &mut set,
    );
    assert_eq!(set.decisions[0].ratification, RatificationTier::Local);
    assert_eq!(set.decisions[0].affects, ["src/lib.rs"]);
    assert!(set.decisions[0].ratified_by_machine);
    assert_eq!(set.decisions[0].receipts, ["rev.test"]);
    let subject_hash = format!("sha256:{}", "a".repeat(64));
    let lens_prompt_hash = format!("sha256:{}", "b".repeat(64));
    load_one_review(
        Path::new("meta/reviews/rev.test.md"),
        &review(&format!(
            "review_type: agent_cross_model\nsubject_hash: {subject_hash}\nlens_prompt_hash: {lens_prompt_hash}\n"
        )),
        &mut set,
    );
    let review = &set.reviews[0];
    assert_eq!(review.subject_hash.as_deref(), Some(subject_hash.as_str()));
    assert_eq!(
        review.lens_prompt_hash.as_deref(),
        Some(lens_prompt_hash.as_str())
    );
}

#[test]
fn test_decision_absent_ratification_fields_default() {
    let mut set = ArtefactSet::default();
    load_one_decision(
        Path::new("meta/decisions/dec.test.md"),
        &decision(""),
        &mut set,
    );
    let item = &set.decisions[0];
    assert_eq!(item.ratification, RatificationTier::Binding);
    assert!(item.affects.is_empty() && !item.ratified_by_machine && item.receipts.is_empty());
}

#[test]
fn test_ratification_invalid_value_emits_ca045() {
    let mut set = ArtefactSet::default();
    load_one_decision(
        Path::new("d.md"),
        &decision("ratification: invalid\n"),
        &mut set,
    );
    assert!(
        set.findings
            .iter()
            .any(|f| f.code == "CAIRN_DECISION_RATIFICATION_INVALID")
    );
}

#[test]
fn test_ratified_by_invalid_value_emits_ca046() {
    let mut set = ArtefactSet::default();
    load_one_decision(
        Path::new("d.md"),
        &decision("ratified_by: human\n"),
        &mut set,
    );
    assert!(
        set.findings
            .iter()
            .any(|f| f.code == "CAIRN_DECISION_RATIFIED_BY_INVALID")
    );
}

#[test]
fn test_affects_invalid_value_emits_ca047() {
    let mut set = ArtefactSet::default();
    load_one_decision(
        Path::new("d.md"),
        &decision("affects: [/tmp/file]\n"),
        &mut set,
    );
    assert!(
        set.findings
            .iter()
            .any(|f| f.code == "CAIRN_DECISION_AFFECTS_INVALID")
    );
}

#[test]
fn test_affects_directory_entry_parses_cleanly() {
    let mut set = ArtefactSet::default();
    load_one_decision(
        Path::new("d.md"),
        &decision("affects: [tests/fixtures/cairn-bootstrap/]\n"),
        &mut set,
    );
    assert!(set.findings.is_empty(), "{:?}", set.findings);
    assert_eq!(
        set.decisions[0].affects,
        ["tests/fixtures/cairn-bootstrap/"]
    );
}

#[test]
fn test_subject_hash_invalid_value_emits_ca048() {
    let mut set = ArtefactSet::default();
    load_one_review(
        Path::new("r.md"),
        &review("subject_hash: invalid\n"),
        &mut set,
    );
    assert!(
        set.findings
            .iter()
            .any(|f| f.code == "CAIRN_REVIEW_SUBJECT_HASH_INVALID")
    );
}

#[test]
fn test_reviewer_invalid_value_emits_ca049() {
    let mut set = ArtefactSet::default();
    load_one_review(
        Path::new("r.md"),
        &review(&format!(
            "reviewer: invalid\nsubject_hash: sha256:{}\nlens_prompt_hash: sha256:{}\n",
            "a".repeat(64),
            "b".repeat(64)
        )),
        &mut set,
    );
    assert!(
        set.findings
            .iter()
            .any(|f| f.code == "CAIRN_REVIEW_REVIEWER_INVALID")
    );
}

#[test]
fn test_lens_prompt_hash_invalid_value_emits_ca050() {
    for value in ["", "lens_prompt_hash: invalid\n"] {
        let mut set = ArtefactSet::default();
        load_one_review(
            Path::new("r.md"),
            &review(&format!("subject_hash: sha256:{}\n{value}", "a".repeat(64))),
            &mut set,
        );
        assert!(
            set.findings
                .iter()
                .any(|f| f.code == "CAIRN_REVIEW_LENS_PROMPT_HASH_INVALID")
        );
    }
}
