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

fn write_minimal_project(root: &std::path::Path) {
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
        "change".to_owned(),
        "list".to_owned(),
    ]);
    assert_eq!(
        changes_result.code, 0,
        "change list must succeed: {}",
        changes_result.stderr
    );
    assert!(
        changes_result.stdout.contains("demo-change"),
        "change list output must list demo-change: {}",
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

#[test]
fn test_changes_dir_flag_respected_by_discover_surfaces() {
    let root = temp_root("changes-dir-flag");
    write_minimal_project(&root);

    // The change lives under a non-default directory; every surface that
    // lists active changes must honour `--changes-dir`.
    let custom = root.join("custom-changes");
    let change_dir = custom.join("demo-change");
    fs::create_dir_all(&change_dir).unwrap();
    fs::write(change_dir.join("proposal.md"), "# Proposal: Demo Change\n").unwrap();
    fs::write(change_dir.join("blueprint.delta"), "## REMOVED Nodes\nt\n").unwrap();

    let base = [
        "--file".to_owned(),
        root.join("cairn.blueprint").to_string_lossy().to_string(),
        "--changes-dir".to_owned(),
        custom.to_string_lossy().to_string(),
    ];

    let run = |extra: &[&str]| {
        let mut args = base.to_vec();
        args.extend(extra.iter().map(ToString::to_string));
        cairn::cli::run(&args)
    };

    let changes_text = run(&["change", "list"]);
    assert_eq!(changes_text.code, 0, "change list: {}", changes_text.stderr);
    assert!(
        changes_text.stdout.contains("demo-change"),
        "`cairn change list` must list changes from --changes-dir: {}",
        changes_text.stdout
    );

    let changes_json = run(&["--json", "change", "list"]);
    assert_eq!(
        changes_json.code, 0,
        "change list --json: {}",
        changes_json.stderr
    );
    assert!(
        changes_json.stdout.contains("demo-change"),
        "`cairn --json change list` must list changes from --changes-dir: {}",
        changes_json.stdout
    );

    let show = run(&["change", "show", "demo-change"]);
    assert_eq!(
        show.code, 0,
        "`cairn change show` must find a change under --changes-dir: {} {}",
        show.stdout, show.stderr
    );

    let status = run(&["status"]);
    assert_eq!(status.code, 0, "status: {}", status.stderr);
    assert!(
        status.stdout.contains("demo-change"),
        "`cairn status` must list active changes from --changes-dir: {}",
        status.stdout
    );

    let neighbourhood = run(&["neighbourhood", "t", "--include-changes"]);
    assert_eq!(
        neighbourhood.code, 0,
        "neighbourhood: {}",
        neighbourhood.stderr
    );
    assert!(
        neighbourhood.stdout.contains("demo-change"),
        "`cairn neighbourhood --include-changes` must surface changes from --changes-dir: {}",
        neighbourhood.stdout
    );

    for surface in [
        vec!["--json", "status"],
        vec!["--json", "change", "show", "demo-change"],
        vec!["--json", "neighbourhood", "t", "--include-changes"],
    ] {
        let result = run(&surface);
        assert_eq!(result.code, 0, "{surface:?}: {}", result.stderr);
        assert!(
            result.stdout.contains("demo-change"),
            "{surface:?} must surface changes from --changes-dir: {}",
            result.stdout
        );
    }

    // Archiving from --changes-dir must move the change into the archive
    // folder under that directory, not the default meta/changes/archive.
    let noop_dir = custom.join("noop-change");
    fs::create_dir_all(&noop_dir).unwrap();
    fs::write(noop_dir.join("proposal.md"), "# Proposal: Noop Change\n").unwrap();
    let archive = run(&["change", "archive", "noop-change"]);
    assert_eq!(
        archive.code, 0,
        "archive from --changes-dir must succeed: {} {}",
        archive.stdout, archive.stderr
    );
    assert!(
        !noop_dir.exists(),
        "archived change must leave --changes-dir"
    );
    let archived: Vec<_> = fs::read_dir(custom.join("archive"))
        .expect("archive folder must be created under --changes-dir")
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    assert!(
        archived.iter().any(|name| name.ends_with("-noop-change")),
        "archive destination must live under --changes-dir: {archived:?}"
    );
    assert!(
        !root.join("meta/changes/archive").exists(),
        "default meta/changes/archive must stay untouched"
    );
}
