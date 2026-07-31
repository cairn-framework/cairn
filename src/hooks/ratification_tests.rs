//! Git-range tests for the ratification hook.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use super::ratification::{RatificationMode, ratification_findings};
use crate::artefacts::registry::{
    ArtefactSet, Decision, DecisionStatus, RatificationTier, Review, ReviewType,
    manifest::compute_subject_hash,
};

#[test]
fn test_ratification_rider_in_earlier_commit_refused() {
    let root = git_root("rider-earlier");
    write(&root, "src/junk.rs", "junk\n");
    commit(&root, "junk");
    let artefacts = accepted_decision(&root, "src/subject.rs", "subject\n");
    commit(&root, "accept");
    let findings = ratification_findings(&root, &artefacts, RatificationMode::Head);
    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "CAIRN_HOOK_AFFECTS_SUBSET")
    );
    cleanup(root);
}

#[test]
fn test_ratification_rider_after_flip_refused() {
    let root = git_root("rider-after");
    let artefacts = accepted_decision(&root, "src/subject.rs", "subject\n");
    commit(&root, "accept");
    write(&root, "src/junk.rs", "junk\n");
    commit(&root, "junk");
    let findings = ratification_findings(&root, &artefacts, RatificationMode::Head);
    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "CAIRN_HOOK_AFFECTS_SUBSET")
    );
    cleanup(root);
}

#[test]
fn test_ratification_manifest_mismatch_refused() {
    let root = git_root("manifest-mismatch");
    let artefacts = accepted_decision(&root, "src/subject.rs", "subject\n");
    commit(&root, "accept");
    write(&root, "src/subject.rs", "changed subject\n");
    commit(&root, "governed edit");
    let findings = ratification_findings(&root, &artefacts, RatificationMode::Head);
    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "CAIRN_HOOK_MANIFEST_MISMATCH")
    );
    cleanup(root);
}

#[test]
fn test_ratification_happy_acceptance_passes() {
    let root = git_root("happy");
    let artefacts = accepted_decision(&root, "src/subject.rs", "subject\n");
    commit(&root, "accept");
    let findings = ratification_findings(&root, &artefacts, RatificationMode::Head);
    assert!(findings.is_empty(), "{findings:?}");
    cleanup(root);
}

#[test]
fn test_ratification_index_mode_unstaged_governed_overlap_refused() {
    let root = git_root("index-governed-overlap");
    let artefacts = accepted_decision(&root, "src/subject.rs", "subject\n");
    run(&root, ["add", "."]);
    write(&root, "src/subject.rs", "unstaged governed edit\n");
    let findings = ratification_findings(&root, &artefacts, RatificationMode::Index);
    assert!(findings.iter().any(|finding| {
        finding.code == "CAIRN_HOOK_MANIFEST_MISMATCH" && finding.message.contains("stage or stash")
    }));
    cleanup(root);
}

#[test]
fn test_ratification_index_mode_unstaged_unrelated_change_passes() {
    let root = git_root("index-unrelated-change");
    let artefacts = accepted_decision(&root, "src/subject.rs", "subject\n");
    run(&root, ["add", "."]);
    write(&root, "src/unrelated.rs", "unstaged unrelated edit\n");
    let findings = ratification_findings(&root, &artefacts, RatificationMode::Index);
    assert!(findings.is_empty(), "{findings:?}");
    cleanup(root);
}

#[test]
fn test_ratification_index_mode_untracked_governed_file_refused() {
    let root = git_root("index-untracked-governed");
    let artefacts = accepted_decision(&root, "src/untracked.rs", "untracked subject\n");
    run(&root, ["add", "meta"]);
    let findings = ratification_findings(&root, &artefacts, RatificationMode::Index);
    assert!(findings.iter().any(|finding| {
        finding.code == "CAIRN_HOOK_MANIFEST_MISMATCH"
            && finding
                .message
                .contains("commit cannot contain the reviewed bytes")
    }));
    cleanup(root);
}

#[test]
fn test_ratification_rename_rider_refused() {
    let root = git_root("rename-rider");
    run(&root, ["mv", "src/subject.rs", "src/covered.rs"]);
    let artefacts = accepted_decision(&root, "src/covered.rs", "subject\n");
    commit(&root, "accept renamed subject");
    let findings = ratification_findings(&root, &artefacts, RatificationMode::Head);
    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "CAIRN_HOOK_AFFECTS_SUBSET")
    );
    cleanup(root);
}

#[test]
fn test_ratification_missing_base_with_local_decision_fails_closed() {
    let root = git_root_without_remote("missing-base");
    let artefacts = accepted_decision(&root, "src/subject.rs", "subject\n");
    let findings = ratification_findings(&root, &artefacts, RatificationMode::Head);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].code, "CAIRN_HOOK_AFFECTS_SUBSET");
    assert!(findings[0].message.contains("origin/main"));
    cleanup(root);
}

fn accepted_decision(root: &Path, subject_path: &str, subject: &str) -> ArtefactSet {
    write(root, subject_path, subject);
    let decision_path = "meta/decisions/dec.local.md";
    let receipt_path = "meta/reviews/rev.correctness.md";
    let decision = decision_source(subject_path, receipt_path);
    write(root, decision_path, &decision);
    let hash = compute_subject_hash(
        root,
        decision_path,
        &decision,
        &[decision_path.to_owned(), subject_path.to_owned()],
    )
    .unwrap();
    write(root, receipt_path, "receipt\n");
    ArtefactSet {
        decisions: vec![Decision {
            id: "dec.local".to_owned(),
            path: root.join(decision_path).to_string_lossy().into_owned(),
            nodes: Vec::new(),
            status: DecisionStatus::Accepted,
            ratification: RatificationTier::Local,
            affects: vec![
                decision_path.to_owned(),
                subject_path.to_owned(),
                receipt_path.to_owned(),
            ],
            ratified_by_machine: false,
            receipts: vec!["rev.correctness".to_owned()],
            date: "2026-07-30".to_owned(),
            revisited: None,
            revisit_triggers: Vec::new(),
            informed_by: Vec::new(),
            supersedes: Vec::new(),
            refines: Vec::new(),
            related: Vec::new(),
            orphaned: false,
            orphan_reason: None,
            gap: false,
            claims: None,
            body: String::new(),
        }],
        reviews: vec![Review {
            path: root.join(receipt_path).to_string_lossy().into_owned(),
            node: "app".to_owned(),
            review_type: ReviewType::AgentCrossModel,
            date: "2026-07-30".to_owned(),
            reviewer: "model/lens".to_owned(),
            subject_hash: Some(hash),
            lens_prompt_hash: Some(
                "sha256:0000000000000000000000000000000000000000000000000000000000000000"
                    .to_owned(),
            ),
            related_change: None,
            body: "## Verdict\nPASS\n".to_owned(),
        }],
        ..ArtefactSet::default()
    }
}

fn decision_source(subject_path: &str, receipt_path: &str) -> String {
    format!(
        "---\nid: dec.local\nstatus: accepted\nratification: local\naffects:\n  - meta/decisions/dec.local.md\n  - {subject_path}\n  - {receipt_path}\nreceipts:\n  - rev.correctness\n---\nDecision body.\n"
    )
}

const BASE_ALLOWLIST: &str = "# Binding surface\n\n- docs/spec.md\n- docs/registries/\n- tools/agent-pack/content/\n- src/artefacts/registry/\n- cairn.blueprint\n";

fn git_root(name: &str) -> PathBuf {
    git_root_with_allowlist(name, Some(BASE_ALLOWLIST))
}

fn git_root_with_allowlist(name: &str, allowlist: Option<&str>) -> PathBuf {
    let root = git_root_without_remote(name);
    write(&root, "src/subject.rs", "subject\n");
    write(&root, "src/unrelated.rs", "unrelated\n");
    write(&root, "src/artefacts/registry/types.rs", "binding\n");
    if let Some(allowlist) = allowlist {
        write(&root, "docs/registries/binding-surface.md", allowlist);
    }
    commit(&root, "base");
    let base = output(&root, ["rev-parse", "HEAD"]);
    run(&root, ["branch", "-M", "main"]);
    run(
        &root,
        ["update-ref", "refs/remotes/origin/main", base.trim()],
    );
    run(&root, ["checkout", "-q", "-b", "feature"]);
    root
}

fn git_root_without_remote(name: &str) -> PathBuf {
    let root =
        std::env::temp_dir().join(format!("cairn-ratification-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    run(&root, ["init", "--quiet"]);
    run(&root, ["config", "user.email", "tests@example.com"]);
    run(&root, ["config", "user.name", "Tests"]);
    root
}

fn write(root: &Path, path: &str, content: &str) {
    let path = root.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}

fn commit(root: &Path, message: &str) {
    run(root, ["add", "."]);
    run(root, ["commit", "--quiet", "-m", message]);
}

fn run<'a>(root: &Path, args: impl IntoIterator<Item = &'a str>) {
    assert!(
        Command::new("git")
            .current_dir(root)
            .args(args)
            .status()
            .unwrap()
            .success()
    );
}

fn output<'a>(root: &Path, args: impl IntoIterator<Item = &'a str>) -> String {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap()
}

fn cleanup(root: PathBuf) {
    let _ = fs::remove_dir_all(root);
}

#[test]
fn test_ratification_base_allowlist_missing_refuses() {
    // A local acceptance cannot be validated without the binding surface it
    // must avoid, so an absent merge-base allowlist fails closed.
    let root = git_root_with_allowlist("base-allowlist-missing", None);
    let artefacts = accepted_decision(&root, "src/subject.rs", "subject\n");
    commit(&root, "accept");
    let findings = ratification_findings(&root, &artefacts, RatificationMode::Head);
    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "CAIRN_DECISION_TIER_BINDING_PATH"),
        "{findings:?}"
    );
    cleanup(root);
}

#[test]
fn test_ratification_self_weakened_allowlist_refused() {
    // The candidate range must not be able to delete its own gate: the hook
    // classifies against the merge-base allowlist, so rewriting the file in
    // the same range cannot unlock a binding surface.
    let root = git_root("self-weakened-allowlist");
    let artefacts = accepted_decision(&root, "src/artefacts/registry/types.rs", "tampered\n");
    write(
        &root,
        "docs/registries/binding-surface.md",
        "# Binding surface\n\n- docs/spec.md\n",
    );
    commit(&root, "weaken and accept");
    let findings = ratification_findings(&root, &artefacts, RatificationMode::Head);
    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "CAIRN_DECISION_TIER_BINDING_PATH"),
        "{findings:?}"
    );
    cleanup(root);
}
