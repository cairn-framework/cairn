//! Git-range tests for configured ratification decision pointers.

use std::fs;

use super::ratification::{RatificationMode, ratification_findings};
use super::ratification_tests::{
    accepted_decision, accepted_decision_at, cleanup, commit, git_root, pointer_blueprint, run,
};
use crate::artefacts::contract::ContractSet;

#[test]
fn test_ratification_non_default_decisions_pointer_refuses_uncovered_change() {
    let root = git_root("non-default-pointer");
    write_pointer_blueprint(&root);
    let _manual = accepted_decision_at(
        &root,
        "docs/policies/decisions",
        "src/subject.rs",
        "subject\n",
    );
    let artefacts = loaded_artefacts(&root);
    commit(&root, "accept");
    write(&root, "src/junk.rs", "junk\n");
    commit(&root, "junk");
    let findings = ratification_findings(&root, &artefacts, RatificationMode::Head);
    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "CAIRN_HOOK_AFFECTS_SUBSET"),
        "{findings:?}"
    );
    cleanup(root);
}

#[test]
fn test_ratification_non_default_decisions_pointer_index_refuses_uncovered_change() {
    let root = git_root("non-default-pointer-index");
    write_pointer_blueprint(&root);
    let _manual = accepted_decision_at(
        &root,
        "docs/policies/decisions",
        "src/subject.rs",
        "subject\n",
    );
    let artefacts = loaded_artefacts(&root);
    write(&root, "src/junk.rs", "junk\n");
    run(&root, ["add", "."]);
    let findings = ratification_findings(&root, &artefacts, RatificationMode::Index);
    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "CAIRN_HOOK_AFFECTS_SUBSET"),
        "{findings:?}"
    );
    cleanup(root);
}

#[test]
fn test_ratification_candidate_pointer_configuration_drift_refuses() {
    let root = git_root("pointer-configuration-drift");
    let artefacts = accepted_decision(&root, "src/subject.rs", "subject\n");
    let default_blueprint = fs::read_to_string(root.join("cairn.blueprint")).unwrap();
    write_pointer_blueprint(&root);
    run(&root, ["add", "."]);
    write(&root, "cairn.blueprint", &default_blueprint);
    let findings = ratification_findings(&root, &artefacts, RatificationMode::Index);
    assert!(
        findings
            .iter()
            .any(|finding| finding.message.contains("pointer configurations differ")),
        "{findings:?}"
    );
    cleanup(root);
}

#[cfg(unix)]
#[test]
fn test_ratification_candidate_symlinked_pointer_refuses() {
    use std::os::unix::fs::symlink;

    let root = git_root("symlinked-candidate-pointer");
    let artefacts = accepted_decision(&root, "src/subject.rs", "subject\n");
    let decision_path = root.join("meta/decisions/dec.local.md");
    let decision = fs::read_to_string(&decision_path).unwrap();
    fs::remove_dir_all(root.join("meta/decisions")).unwrap();
    fs::create_dir_all(root.join("outside-decisions")).unwrap();
    fs::write(root.join("outside-decisions/dec.local.md"), decision).unwrap();
    symlink(root.join("outside-decisions"), root.join("meta/decisions")).unwrap();
    commit(&root, "accept");
    let findings = ratification_findings(&root, &artefacts, RatificationMode::Head);
    assert!(
        findings
            .iter()
            .any(|finding| finding.message.contains("candidate decisions")),
        "{findings:?}"
    );
    cleanup(root);
}

fn write_pointer_blueprint(root: &std::path::Path) {
    write(
        root,
        "cairn.blueprint",
        &pointer_blueprint("docs/policies/decisions"),
    );
}

fn loaded_artefacts(root: &std::path::Path) -> crate::artefacts::registry::ArtefactSet {
    let ast = crate::blueprint::parser::parse_str(
        "cairn.blueprint",
        &fs::read_to_string(root.join("cairn.blueprint")).unwrap(),
    )
    .unwrap();
    let artefacts = crate::artefacts::registry::load_artefacts(root, &ast, ContractSet::default());
    assert_eq!(artefacts.decisions.len(), 1, "{:?}", artefacts.findings);
    artefacts
}

fn write(root: &std::path::Path, path: &str, content: &str) {
    let path = root.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}
