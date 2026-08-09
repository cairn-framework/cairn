//! Scratch-workspace containment and the one path validator.

use super::super::prompt::Prompt;
use super::super::workspace::{Workspace, canonical_relative};
use super::{apply, edit, scratch_fixture};

#[test]
fn test_workspace_copies_the_fixture_without_mutating_it() {
    let (_guard, fixture) = scratch_fixture();
    let workspace = Workspace::from_fixture(&fixture).expect("workspace");

    apply(&workspace, &[edit("keep.txt", "rewritten")]).expect("apply");

    assert_eq!(
        std::fs::read_to_string(workspace.root().join("keep.txt")).expect("copy"),
        "rewritten"
    );
    assert_eq!(
        std::fs::read_to_string(fixture.join("keep.txt")).expect("fixture"),
        "original",
        "the fixture must never be the workspace"
    );
}

#[test]
fn test_workspace_is_removed_on_drop() {
    let (_guard, fixture) = scratch_fixture();
    let root = {
        let workspace = Workspace::from_fixture(&fixture).expect("workspace");
        workspace.root().to_path_buf()
    };
    assert!(!root.exists(), "the scratch workspace must not outlive it");
}

#[test]
fn test_workspace_rejects_paths_that_escape_the_root() {
    let (_guard, fixture) = scratch_fixture();
    let workspace = Workspace::from_fixture(&fixture).expect("workspace");

    for (path, reason) in [
        ("/etc/passwd", "is absolute"),
        ("../outside.md", "escapes the scratch workspace"),
        ("meta/../../outside.md", "escapes the scratch workspace"),
        ("", "is empty"),
        (".", "is not a plain relative file path"),
        ("./out.md", "is not a plain relative file path"),
        ("meta/", "is directory-shaped"),
        ("bad\0name.md", "contains a NUL byte"),
    ] {
        let reported = workspace
            .validate(&[edit(path, "x")])
            .expect_err("unusable path must be rejected");
        assert!(
            reported.contains(reason),
            "unexpected rejection for `{path}`: {reported}"
        );
    }
}

#[test]
fn test_workspace_rejects_the_whole_batch_before_writing_any_of_it() {
    let (_guard, fixture) = scratch_fixture();
    let workspace = Workspace::from_fixture(&fixture).expect("workspace");

    apply(
        &workspace,
        &[edit("good.md", "written"), edit("../escape.md", "written")],
    )
    .expect_err("a batch with one bad path must be rejected");

    assert!(
        !workspace.root().join("good.md").exists(),
        "no byte may be written when any path in the batch is rejected"
    );
}

#[test]
fn test_unmet_compares_canonical_paths_the_workspace_produced() {
    let prompt = Prompt {
        schema_version: 1,
        id: "p".to_owned(),
        instruction: "author it".to_owned(),
        expects: vec!["meta/decisions/x.md".to_owned()],
        replay: None,
    };

    assert!(
        prompt.unmet(&["meta/decisions/x.md".to_owned()]).is_empty(),
        "the canonical spelling satisfies the prompt"
    );
    assert_eq!(
        canonical_relative("meta//decisions/x.md").as_deref(),
        Ok("meta/decisions/x.md"),
        "repeated separators canonicalise to the same file, so a response spelling \
         it that way still satisfies the prompt once the workspace has canonicalised it"
    );
    assert!(
        canonical_relative("./meta/decisions/x.md").is_err(),
        "a `.` segment never reaches unmet: the workspace rejects it, and the runner \
         records that as a protocol violation"
    );
    assert_eq!(
        prompt.unmet(&["meta/decisions/other.md".to_owned()]),
        vec!["meta/decisions/x.md"],
        "an unrelated file does not satisfy the prompt"
    );
}

#[test]
fn test_validate_rejects_conflicting_and_duplicate_targets() {
    let (_guard, fixture) = scratch_fixture();
    let workspace = Workspace::from_fixture(&fixture).expect("workspace");

    let duplicate = workspace
        .validate(&[edit("out.md", "a"), edit("out//md/../out.md", "b")])
        .err();
    assert!(
        workspace
            .validate(&[edit("out.md", "a"), edit("out.md", "b")])
            .expect_err("a duplicate destination is ambiguous")
            .contains("more than once"),
        "duplicate targets must be rejected, got {duplicate:?}"
    );

    assert!(
        workspace
            .validate(&[edit("new", "a"), edit("new/child.md", "b")])
            .expect_err("one path cannot be both a file and a directory")
            .contains("cannot both exist")
    );

    // An ancestor that already exists as a file would make `create_dir_all`
    // fail partway through the batch. Nested two deep on purpose: with only one
    // level, a walk that iterated the ancestors backwards would still pass.
    std::fs::create_dir_all(workspace.root().join("outer")).expect("seed dir");
    std::fs::write(workspace.root().join("outer/blocker"), "file").expect("seed file");
    assert!(
        workspace
            .validate(&[edit("outer/blocker/child.md", "a")])
            .expect_err("an existing file ancestor must be caught before writing")
            .contains("ancestor")
    );

    // A directory cannot be overwritten with file content either.
    std::fs::create_dir_all(workspace.root().join("adir")).expect("seed dir");
    assert!(
        workspace
            .validate(&[edit("adir", "a")])
            .expect_err("an existing directory target must be rejected")
            .contains("existing directory")
    );
}

#[test]
fn test_canonical_relative_accepts_only_plain_relative_files() {
    assert_eq!(canonical_relative("a//b.md").as_deref(), Ok("a/b.md"));
    for bad in [
        "a/./b.md",
        "..",
        "../a.md",
        "/a.md",
        "a.md/",
        "",
        "a\0b.md",
        // Drive-relative on Windows: `is_absolute()` is false, yet joining it
        // onto the workspace root would replace the root outright.
        "C:out.md",
        "\\\\server\\share\\out.md",
    ] {
        assert!(canonical_relative(bad).is_err(), "`{bad}` must be rejected");
    }
}
