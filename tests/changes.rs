//! Phase 3 change-system integration tests.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use cairn::{changes, scanner};

#[test]
fn test_delta_parser_handles_all_sections_and_invalid_rename()
-> Result<(), Box<dyn std::error::Error>> {
    let delta = changes::parse_blueprint_delta(
        "blueprint.delta",
        r#"## ADDED Nodes
Module Billing "billing" id "app.billing" {
    path "./src/billing"
}

## MODIFIED Nodes
Module Auth "auth v2" id "app.auth" {
    path "./src/auth"
}

## REMOVED Nodes
- app.old

## RENAMED Nodes
- app.legacy -> app.identity

## ADDED Edges
app.auth -> app.billing "calls"

## MODIFIED Edges
app.auth -> app.billing "calls v2"

## REMOVED Edges
app.auth -> app.old "legacy"

## RENAMED Edges
app.legacy -> app.auth "uses" => app.identity -> app.auth "uses"
"#,
    )?;

    assert_eq!(delta.added_nodes[0].id, "app.billing");
    assert_eq!(delta.modified_nodes[0].description, "auth v2");
    assert_eq!(delta.removed_nodes, vec!["app.old"]);
    assert_eq!(delta.renamed_nodes[0].to, "app.identity");
    assert_eq!(delta.added_edges[0].to, "app.billing");
    assert_eq!(delta.renamed_edges[0].to.from, "app.identity");

    let invalid = changes::parse_blueprint_delta(
        "blueprint.delta",
        "## RENAMED Nodes\n- app.auth app.identity\n",
    );
    assert!(invalid.is_err());

    Ok(())
}

#[test]
fn test_changes_command_and_default_query_isolation() -> Result<(), Box<dyn std::error::Error>> {
    let root = fixture("change-isolation")?;
    write_base_project(&root)?;
    write_change(
        &root,
        "add-billing",
        r#"## ADDED Nodes
Module Billing "billing" id "app.billing" {
    path "./src/billing"
}
"#,
    )?;

    let changes = run(&root, &["changes"])?;
    assert!(changes.status.success());
    let changes = String::from_utf8(changes.stdout)?;
    assert!(changes.contains("add-billing"));
    assert!(changes.contains("added_nodes"));

    let get = run(&root, &["get", "app.billing"])?;
    assert!(!get.status.success());
    let get = String::from_utf8(get.stdout)?;
    assert!(get.contains("CAIRN_QUERY_NODE_NOT_FOUND"));

    Ok(())
}

#[test]
fn test_neighbourhood_include_changes_lists_proposed_operations()
-> Result<(), Box<dyn std::error::Error>> {
    let root = fixture("include-changes")?;
    write_base_project(&root)?;
    write_change(
        &root,
        "modify-auth",
        r#"## MODIFIED Nodes
Module Auth "auth next" id "app.auth" {
    path "./src/auth"
}
"#,
    )?;

    let default = run(&root, &["neighbourhood", "app.auth"])?;
    assert!(default.status.success());
    let default = String::from_utf8(default.stdout)?;
    assert!(!default.contains("modify-auth"));

    let included = run(&root, &["neighbourhood", "app.auth", "--include-changes"])?;
    assert!(included.status.success());
    let included = String::from_utf8(included.stdout)?;
    assert!(included.contains("modify-auth: modified node app.auth"));

    Ok(())
}

#[test]
fn test_change_validation_reports_invalid_references_conflicts_and_artefact_errors()
-> Result<(), Box<dyn std::error::Error>> {
    let root = fixture("validation")?;
    write_base_project(&root)?;
    let change = root.join("meta/changes/invalid");
    fs::create_dir_all(change.join("decisions"))?;
    fs::write(
        change.join("proposal.md"),
        "# Proposal: invalid\n\nInvalid fixture.\n",
    )?;
    fs::write(
        change.join("blueprint.delta"),
        r#"## MODIFIED Nodes
Module Auth "auth" id "app.auth" {
    path "./src/auth"
}

## REMOVED Nodes
- app.auth

## ADDED Edges
app.auth -> app.missing "missing"
"#,
    )?;
    fs::write(
        change.join("decisions/missing-operation.md"),
        "---\nid: dec.missing\nnodes: [app.auth]\nstatus: accepted\ndate: 2026-04-20\n---\n# Missing operation\n",
    )?;
    fs::write(
        change.join("decisions/missing-target.md"),
        "---\noperation: modified\nid: dec.missing\nnodes: [app.auth]\nstatus: accepted\ndate: 2026-04-20\n---\n# Missing target\n",
    )?;
    fs::write(
        change.join("decisions/missing-rename.md"),
        "---\noperation: renamed\nid: dec.rename\nnodes: [app.auth]\nstatus: accepted\ndate: 2026-04-20\n---\n# Missing renamed_from\n",
    )?;

    let scan = scanner::load_project(&root, &root.join("cairn.blueprint"))?;
    let active = changes::discover(&root)?;
    let errors = changes::validate_change(&active[0], &scan.graph);

    assert!(
        errors
            .iter()
            .any(|error| error.contains("conflicting operations"))
    );
    assert!(
        errors
            .iter()
            .any(|error| error.contains("missing endpoint"))
    );
    assert!(
        errors
            .iter()
            .any(|error| error.contains("missing valid operation"))
    );
    assert!(errors.iter().any(|error| error.contains("does not exist")));
    assert!(
        errors
            .iter()
            .any(|error| error.contains("missing renamed_from"))
    );

    Ok(())
}

#[test]
fn test_change_commands_cover_json_and_human_output_shapes()
-> Result<(), Box<dyn std::error::Error>> {
    let root = fixture("output-shapes")?;
    write_base_project(&root)?;
    write_change(
        &root,
        "add-billing",
        r#"## ADDED Nodes
Module Billing "billing" id "app.billing" {
    path "./src/billing"
}
"#,
    )?;

    let changes = run(&root, &["--json", "changes"])?;
    assert!(changes.status.success());
    let changes = String::from_utf8(changes.stdout)?;
    assert!(changes.contains("\"changes\""));
    assert!(changes.contains("\"id\":\"add-billing\""));
    assert!(changes.contains("\"summary\""));

    let show = run(&root, &["show", "add-billing"])?;
    assert!(show.status.success());
    let show = String::from_utf8(show.stdout)?;
    assert!(show.contains("Blueprint delta:"));
    assert!(show.contains("added node app.billing"));

    let status = run(&root, &["--json", "status"])?;
    assert!(status.status.success());
    let status = String::from_utf8(status.stdout)?;
    assert!(status.contains("\"active_changes\""));
    assert!(status.contains("add-billing"));

    Ok(())
}

#[test]
fn test_archive_applies_change_moves_directory_and_updates_log()
-> Result<(), Box<dyn std::error::Error>> {
    let root = fixture("archive-success")?;
    write_base_project(&root)?;
    write_change(
        &root,
        "add-billing",
        r#"## ADDED Nodes
Module Billing "billing" id "app.billing" {
    path "./src/billing"
}

## ADDED Edges
app.auth -> app.billing "calls"
"#,
    )?;

    let archive = run(&root, &["archive", "add-billing"])?;
    assert!(
        archive.status.success(),
        "{}",
        String::from_utf8(archive.stdout)?
    );
    let blueprint = fs::read_to_string(root.join("cairn.blueprint"))?;
    assert!(blueprint.contains("app.billing"));
    assert!(root.join("meta/changes/archive").exists());
    assert!(!root.join("meta/changes/add-billing").exists());
    let changes = run(&root, &["changes"])?;
    assert!(changes.status.success());
    let changes = String::from_utf8(changes.stdout)?;
    assert!(!changes.contains("add-billing"));
    let log = fs::read_to_string(root.join(".cairn/log.md"))?;
    assert!(log.contains("archive: add-billing merged"));

    Ok(())
}

#[test]
fn test_archive_rolls_back_when_validation_scan_fails() -> Result<(), Box<dyn std::error::Error>> {
    let root = fixture("archive-rollback")?;
    write_base_project(&root)?;
    let original = fs::read_to_string(root.join("cairn.blueprint"))?;
    write_change(
        &root,
        "bad-id",
        r#"## ADDED Nodes
Module Bad "bad" id "App.Bad" {
    path "./src/bad"
}
"#,
    )?;

    let archive = run(&root, &["archive", "bad-id"])?;
    assert!(!archive.status.success());
    assert_eq!(fs::read_to_string(root.join("cairn.blueprint"))?, original);
    assert!(root.join("meta/changes/bad-id").exists());
    assert!(!root.join("meta/changes/archive").exists());

    Ok(())
}

#[test]
fn test_rename_creates_reviewable_change_without_mutating_main_tree()
-> Result<(), Box<dyn std::error::Error>> {
    let root = fixture("rename")?;
    write_base_project(&root)?;
    fs::create_dir_all(root.join("meta/decisions"))?;
    fs::write(
        root.join("meta/decisions/auth.md"),
        "---\nid: dec.auth\nnodes: [app.auth]\nstatus: accepted\ndate: 2026-04-20\n---\n# Auth\n",
    )?;
    let original = fs::read_to_string(root.join("cairn.blueprint"))?;

    let rename = run(&root, &["rename", "app.auth", "app.identity"])?;
    assert!(
        rename.status.success(),
        "{}",
        String::from_utf8(rename.stdout)?
    );
    assert_eq!(fs::read_to_string(root.join("cairn.blueprint"))?, original);
    let change_dir = root.join("meta/changes/rename-app.auth-to-app.identity");
    assert!(change_dir.join("blueprint.delta").exists());
    let delta = fs::read_to_string(change_dir.join("blueprint.delta"))?;
    assert!(delta.contains("app.auth -> app.identity"));
    let decision = fs::read_to_string(change_dir.join("decisions/auth.md"))?;
    assert!(decision.contains("operation: modified"));
    assert!(decision.contains("nodes: [app.identity]"));

    Ok(())
}

fn write_base_project(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(root.join("src/auth"))?;
    fs::write(root.join("src/auth/lib.rs"), "pub fn login() {}\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module Auth "auth" id "app.auth" {
        path "./src/auth"
    }
}
"#,
    )?;
    Ok(())
}

fn write_change(root: &Path, id: &str, delta: &str) -> Result<(), Box<dyn std::error::Error>> {
    let change = root.join("meta/changes").join(id);
    fs::create_dir_all(&change)?;
    fs::write(
        change.join("proposal.md"),
        format!("# Proposal: {id}\n\nChange fixture.\n"),
    )?;
    fs::write(change.join("blueprint.delta"), delta)?;
    Ok(())
}

fn run(root: &Path, args: &[&str]) -> Result<std::process::Output, Box<dyn std::error::Error>> {
    Ok(Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(root)
        .args(args)
        .output()?)
}

fn fixture(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("cairn-{name}-{stamp}"));
    fs::create_dir_all(&root)?;
    Ok(root)
}
