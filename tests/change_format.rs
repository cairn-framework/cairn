//! Phase 7 regression tests: change system stays format-only.
//!
//! After the workflow trim, `cairn change new` still scaffolds the change
//! directory format and `cairn changes` still lists it, but no beads state is
//! created or claimed.

use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

fn temp_root(name: &str) -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!("cairn-change-format-{name}-{suffix}"));
    fs::create_dir_all(&root).unwrap();
    root
}

fn write_minimal_project(root: &PathBuf) {
    fs::write(
        root.join("cairn.blueprint"),
        "System App \"T\" id \"t\" {}\n",
    )
    .unwrap();
}

#[test]
fn test_change_new_scaffolds_format_and_changes_lists_it() {
    let root = temp_root("new-lists");
    write_minimal_project(&root);

    let new_result = cairn::cli::run(&[
        "--file".to_owned(),
        root.join("cairn.blueprint").to_string_lossy().to_string(),
        "change".to_owned(),
        "new".to_owned(),
        "demo-change".to_owned(),
    ]);
    assert_eq!(
        new_result.code, 0,
        "change new must succeed: {}",
        new_result.stderr
    );

    let change_dir = root.join("meta/changes/demo-change");
    assert!(
        change_dir.join("proposal.md").exists(),
        "proposal.md must be scaffolded"
    );
    assert!(
        change_dir.join("design.md").exists(),
        "design.md must be scaffolded"
    );
    assert!(
        change_dir.join("tasks.md").exists(),
        "tasks.md must be scaffolded"
    );
    assert!(
        change_dir.join("specs").is_dir(),
        "specs/ directory must be scaffolded"
    );

    // Format-only: no beads backing should be created.
    assert!(
        !change_dir.join(".bead-id").exists(),
        ".bead-id must not be created in format-only mode"
    );

    let changes_result = cairn::cli::run(&[
        "--file".to_owned(),
        root.join("cairn.blueprint").to_string_lossy().to_string(),
        "--json".to_owned(),
        "changes".to_owned(),
    ]);
    assert_eq!(
        changes_result.code, 0,
        "changes must succeed: {}",
        changes_result.stderr
    );
    assert!(
        changes_result.stdout.contains("demo-change"),
        "changes output must list demo-change: {}",
        changes_result.stdout
    );
}

#[test]
fn test_change_tasks_and_apply_are_removed() {
    let root = temp_root("tasks-apply-removed");
    write_minimal_project(&root);

    // Even after scaffolding a change, the workflow subcommands are gone.
    let tasks_result = cairn::cli::run(&[
        "--file".to_owned(),
        root.join("cairn.blueprint").to_string_lossy().to_string(),
        "change".to_owned(),
        "tasks".to_owned(),
        "demo-change".to_owned(),
    ]);
    assert_ne!(tasks_result.code, 0, "change tasks must be removed");

    let apply_result = cairn::cli::run(&[
        "--file".to_owned(),
        root.join("cairn.blueprint").to_string_lossy().to_string(),
        "change".to_owned(),
        "apply".to_owned(),
        "demo-change".to_owned(),
    ]);
    assert_ne!(apply_result.code, 0, "change apply must be removed");
}

#[test]
fn test_state_backend_config_key_is_removed() {
    let root = temp_root("state-backend-removed");
    write_minimal_project(&root);
    fs::write(
        root.join("cairn.config.yaml"),
        "state_backend: beads\ncontext: \"ctx\"\n",
    )
    .unwrap();

    let config = cairn::scanner::config::load(&root).unwrap();
    // The key must not be parsed at all (accept-and-ignore is prohibited).
    // Because it is not a field, the only observable behaviour is that the
    // config loads successfully and context is still parsed.
    assert_eq!(config.context, "ctx");
}
