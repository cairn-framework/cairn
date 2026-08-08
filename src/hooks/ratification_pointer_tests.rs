//! Git-range tests for configured ratification decision pointers.

use std::{fs, path::Path};

use super::ratification::{
    RatificationMode, ratification_findings, ratification_findings_with_blueprint,
};
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
    let _ = accepted_decision(&root, "src/subject.rs", "subject\n");
    let artefacts = loaded_artefacts(&root);
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

#[test]
fn test_ratification_uses_selected_blueprint_path() {
    let root = git_root("selected-blueprint");
    let _manual = accepted_decision_at(
        &root,
        "docs/policies/decisions",
        "src/subject.rs",
        "subject\n",
    );
    write(
        &root,
        "alt.blueprint",
        &pointer_blueprint("docs/policies/decisions"),
    );
    let artefacts = loaded_artefacts_at(&root, Path::new("alt.blueprint"));
    commit(&root, "accept");
    write(&root, "src/junk.rs", "junk\n");
    commit(&root, "junk");
    let findings = ratification_findings_with_blueprint(
        &root,
        &artefacts,
        RatificationMode::Head,
        Path::new("alt.blueprint"),
    );
    assert!(
        findings.iter().any(|finding| {
            finding.code == "CAIRN_HOOK_AFFECTS_SUBSET"
                && !finding.message.contains("pointer configurations differ")
        }),
        "{findings:?}"
    );
    cleanup(root);
}
#[test]
fn test_scan_result_records_lexically_relative_blueprint_path() {
    let root = git_root("selected-blueprint-parent");
    write(
        &root,
        "alt.blueprint",
        &pointer_blueprint("docs/policies/decisions"),
    );
    fs::create_dir_all(root.join("nested")).unwrap();
    let selected = root.join("nested/../alt.blueprint");
    let scan = crate::scanner::load_project(&root, &selected).unwrap();
    assert_eq!(scan.blueprint_path, Path::new("alt.blueprint"));
    cleanup(root);
}

#[test]
fn test_ratification_candidate_invalid_utf8_head_refuses() {
    candidate_invalid_utf8_refuses(RatificationMode::Head, "invalid-utf8-candidate-head");
}

#[test]
fn test_ratification_candidate_invalid_utf8_index_refuses() {
    candidate_invalid_utf8_refuses(RatificationMode::Index, "invalid-utf8-candidate-index");
}

fn candidate_invalid_utf8_refuses(mode: RatificationMode, name: &str) {
    let root = git_root(name);
    let _ = accepted_decision(&root, "src/subject.rs", "subject\n");
    let artefacts = loaded_artefacts(&root);
    let decision_path = root.join("meta/decisions/dec.local.md");
    fs::write(&decision_path, b"---\n\xff\n").unwrap();
    match mode {
        RatificationMode::Head => commit(&root, "invalid decision"),
        RatificationMode::Index => run(&root, ["add", "."]),
    }
    let findings = ratification_findings(&root, &artefacts, mode);
    assert!(
        findings
            .iter()
            .any(|finding| finding.message.contains("candidate decisions")),
        "{findings:?}"
    );
    cleanup(root);
}

#[cfg(unix)]
#[test]
fn test_ratification_candidate_symlinked_pointer_refuses() {
    use std::os::unix::fs::symlink;

    let root = git_root("symlinked-candidate-pointer");
    let _ = accepted_decision(&root, "src/subject.rs", "subject\n");
    let artefacts = loaded_artefacts(&root);
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

#[cfg(unix)]
#[test]
fn test_ratification_candidate_child_symlink_head_refuses() {
    candidate_child_symlink_refuses(RatificationMode::Head, "symlinked-candidate-child-head");
}

#[cfg(unix)]
#[test]
fn test_ratification_candidate_child_symlink_index_refuses() {
    candidate_child_symlink_refuses(RatificationMode::Index, "symlinked-candidate-child-index");
}

#[cfg(unix)]
fn candidate_child_symlink_refuses(mode: RatificationMode, name: &str) {
    use std::os::unix::fs::symlink;

    let root = git_root(name);
    let _ = accepted_decision(&root, "src/subject.rs", "subject\n");
    let artefacts = loaded_artefacts(&root);
    let decision_path = root.join("meta/decisions/dec.local.md");
    let decision = fs::read_to_string(&decision_path).unwrap();
    fs::create_dir_all(root.join("outside-decisions")).unwrap();
    fs::write(root.join("outside-decisions/dec.local.md"), &decision).unwrap();
    fs::remove_file(&decision_path).unwrap();
    symlink(root.join("outside-decisions/dec.local.md"), &decision_path).unwrap();
    match mode {
        RatificationMode::Head => commit(&root, "symlinked decision"),
        RatificationMode::Index => run(&root, ["add", "."]),
    }
    fs::remove_file(&decision_path).unwrap();
    fs::write(&decision_path, decision).unwrap();
    let findings = ratification_findings(&root, &artefacts, mode);
    assert!(
        findings
            .iter()
            .any(|finding| finding.message.contains("candidate decisions")),
        "{findings:?}"
    );
    cleanup(root);
}

fn write_pointer_blueprint(root: &Path) {
    write(
        root,
        "cairn.blueprint",
        &pointer_blueprint("docs/policies/decisions"),
    );
}

fn loaded_artefacts(root: &Path) -> crate::artefacts::registry::ArtefactSet {
    loaded_artefacts_at(root, Path::new("cairn.blueprint"))
}

fn loaded_artefacts_at(
    root: &Path,
    blueprint_path: &Path,
) -> crate::artefacts::registry::ArtefactSet {
    let path = root.join(blueprint_path);
    let source = fs::read_to_string(&path).unwrap();
    let ast = crate::blueprint::parser::parse_str(&path.to_string_lossy(), &source).unwrap();
    let artefacts = crate::artefacts::registry::load_artefacts(root, &ast, ContractSet::default());
    assert_eq!(artefacts.decisions.len(), 1, "{:?}", artefacts.findings);
    artefacts
}

fn write(root: &Path, path: &str, content: &str) {
    let path = root.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}
