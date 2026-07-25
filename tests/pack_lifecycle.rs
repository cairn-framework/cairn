//! `cairn pack` lifecycle acceptance tests.
//!
//! The pack writes into a user's repository, so the load-bearing property is
//! not "the files appear": it is that a file the user edited survives every
//! verb, and that only files the manifest owns at a matching hash are ever
//! written or retired (`dec.agent-pack-packaging` clauses 5 and 6).

use std::fs;
use std::path::{Path, PathBuf};

const MANIFEST: &str = ".cairn/state/agent-pack.json";
const ROUTER: &str = ".claude/skills/cairn-dev/SKILL.md";
const LOOP_MODE: &str = ".claude/skills/cairn-dev/references/loop-mode.md";

fn temp_root(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("cairn-pack-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    fs::write(
        root.join("cairn.blueprint"),
        "System App \"T\" id \"t\" {}\n",
    )
    .unwrap();
    root
}

fn pack(root: &Path, tokens: &[&str]) -> cairn::cli::CliResult {
    let mut argv = vec![
        "--file".to_owned(),
        root.join("cairn.blueprint").to_string_lossy().to_string(),
        "pack".to_owned(),
    ];
    argv.extend(tokens.iter().map(|token| (*token).to_owned()));
    cairn::cli::run(&argv)
}

fn manifest_json(root: &Path) -> serde_json::Value {
    serde_json::from_str(&fs::read_to_string(root.join(MANIFEST)).unwrap()).unwrap()
}

#[test]
fn install_writes_the_pack_and_a_manifest_and_is_idempotent() {
    let root = temp_root("install");

    let first = pack(&root, &["install"]);
    assert_eq!(first.code, 0, "install must succeed: {}", first.stderr);
    assert!(root.join(ROUTER).exists(), "install must write the router");
    assert!(
        root.join(MANIFEST).exists(),
        "install must write a manifest"
    );

    let ledger = manifest_json(&root);
    let files = ledger["files"].as_array().unwrap().clone();
    assert!(!files.is_empty(), "the manifest must record what it owns");
    assert_eq!(
        ledger["schema_version"], 1,
        "the ledger must declare its schema version"
    );
    assert!(
        !root.join(LOOP_MODE).exists(),
        "loop mode is opt in: a default install must not make the router claim it is available"
    );

    let before = fs::read_to_string(root.join(ROUTER)).unwrap();
    let second = pack(&root, &["install"]);
    assert_eq!(second.code, 0);
    assert_eq!(
        fs::read_to_string(root.join(ROUTER)).unwrap(),
        before,
        "a second install must not rewrite pristine files"
    );
    assert_eq!(
        manifest_json(&root)["files"].as_array().unwrap().len(),
        files.len(),
        "a second install must not duplicate ledger rows"
    );
}

#[test]
fn a_user_edit_survives_update_and_uninstall() {
    let root = temp_root("modified");
    assert_eq!(pack(&root, &["install"]).code, 0);

    let edited = "# my own router\n";
    fs::write(root.join(ROUTER), edited).unwrap();

    let update = pack(&root, &["update"]);
    assert_eq!(update.code, 0, "update must succeed: {}", update.stderr);
    assert_eq!(
        fs::read_to_string(root.join(ROUTER)).unwrap(),
        edited,
        "update must never overwrite a file the user changed"
    );
    assert!(
        update.stdout.contains(ROUTER),
        "update must report the file it refused to touch:\n{}",
        update.stdout
    );

    let uninstall = pack(&root, &["uninstall"]);
    assert_eq!(uninstall.code, 0);
    assert_eq!(
        fs::read_to_string(root.join(ROUTER)).unwrap(),
        edited,
        "uninstall must keep a file the user changed"
    );
    assert!(
        !root.join(".claude/skills/cairn-explore/SKILL.md").exists(),
        "uninstall must retire the files it owns and still matches"
    );
    assert!(
        !root.join(MANIFEST).exists(),
        "uninstall must drop the ownership ledger"
    );
}

#[test]
fn init_delegates_pack_ownership_and_a_later_install_is_idempotent() {
    let root = temp_root("init-owned");
    let init = cairn::cli::run(&[
        "--file".to_owned(),
        root.join("cairn.blueprint").to_string_lossy().to_string(),
        "init".to_owned(),
    ]);
    assert_eq!(init.code, 0, "init must succeed: {}", init.stderr);
    assert!(root.join(ROUTER).exists(), "init must install the router");
    let before = manifest_json(&root);

    let install = pack(&root, &["install", "--json"]);
    assert_eq!(install.code, 0);
    let report: serde_json::Value = serde_json::from_str(&install.stdout).unwrap();
    assert!(
        report["data"]["written"].as_array().unwrap().is_empty(),
        "the lifecycle must not rewrite its own init installation"
    );
    assert_eq!(
        manifest_json(&root),
        before,
        "idempotent install must preserve the ownership ledger"
    );
}

#[test]
fn a_matching_legacy_install_is_adopted_without_rewriting_it() {
    let root = temp_root("legacy");
    assert_eq!(pack(&root, &["install"]).code, 0);
    let router_before = fs::read_to_string(root.join(ROUTER)).unwrap();
    fs::remove_file(root.join(MANIFEST)).unwrap();

    let install = pack(&root, &["install"]);
    assert_eq!(install.code, 0);
    let ledger = manifest_json(&root);
    assert!(
        ledger["files"]
            .as_array()
            .unwrap()
            .iter()
            .any(|file| file["path"] == ROUTER),
        "adoption must record the existing file"
    );
    assert_eq!(
        fs::read_to_string(root.join(ROUTER)).unwrap(),
        router_before,
        "adoption must not rewrite matching legacy bytes"
    );
}

#[test]
fn update_backfills_a_deleted_file() {
    let root = temp_root("backfill");
    assert_eq!(pack(&root, &["install"]).code, 0);
    fs::remove_file(root.join(ROUTER)).unwrap();

    let status: serde_json::Value =
        serde_json::from_str(&pack(&root, &["status", "--json"]).stdout).unwrap();
    assert_eq!(
        status["data"]["missing"].as_array().unwrap().len(),
        1,
        "status must notice the deleted file"
    );

    assert_eq!(pack(&root, &["update"]).code, 0);
    assert!(
        root.join(ROUTER).exists(),
        "update must backfill a missing owned file"
    );
}

#[test]
fn loop_mode_installs_only_when_asked_and_stays_owned_afterwards() {
    let root = temp_root("loop");
    assert_eq!(pack(&root, &["install", "--loop"]).code, 0);
    assert!(
        root.join(LOOP_MODE).exists(),
        "--loop must install loop mode"
    );
    assert!(
        root.join(".claude/skills/cairn-loop-reconcile/SKILL.md")
            .exists(),
        "--loop must install the full required asset closure"
    );

    // A later plain install must not orphan the loop assets it already owns.
    assert_eq!(pack(&root, &["install"]).code, 0);
    let ledger = manifest_json(&root);
    assert!(
        ledger["files"]
            .as_array()
            .unwrap()
            .iter()
            .any(|file| file["path"] == LOOP_MODE),
        "a plain install must keep ownership of previously installed loop assets"
    );

    let uninstall = pack(&root, &["uninstall"]);
    assert_eq!(uninstall.code, 0);
    assert!(
        !root.join(LOOP_MODE).exists(),
        "uninstall must retire the loop assets it owns"
    );
}

#[test]
fn the_harness_selector_accepts_either_argument_order_and_rejects_unknown_names() {
    let root = temp_root("harness");
    assert_eq!(
        pack(&root, &["--harness", "claude", "install"]).code,
        0,
        "the selector must work before the subcommand"
    );
    assert_eq!(
        pack(&root, &["install", "--harness", "claude"]).code,
        0,
        "the selector must work after the subcommand"
    );
    let unknown = pack(&root, &["install", "--harness", "borges"]);
    assert_eq!(unknown.code, 2, "an unvalidated harness must be refused");
    assert!(
        unknown.stderr.contains("claude"),
        "the refusal must name what is supported: {}",
        unknown.stderr
    );
}

#[test]
fn the_bundle_version_matches_the_canonical_pack_manifest() {
    // The binary carries a bundle version; the canonical manifest declares one.
    // If they drift, every installed ledger records a version that never
    // existed, and migration notes key off nothing.
    let canonical = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tools/agent-pack/manifest.toml"),
    )
    .unwrap();
    let declared = canonical
        .lines()
        .find_map(|line| line.strip_prefix("bundle_version = "))
        .map(|value| value.trim_matches('"').to_owned())
        .expect("the canonical manifest must declare a bundle version");

    let root = temp_root("version");
    assert_eq!(pack(&root, &["install"]).code, 0);
    assert_eq!(
        manifest_json(&root)["bundle_version"],
        declared,
        "the compiled bundle version must match tools/agent-pack/manifest.toml"
    );
}

#[test]
fn a_missing_or_unknown_subcommand_is_a_usage_error() {
    let root = temp_root("usage");
    for tokens in [vec![], vec!["instal"], vec!["--loop"]] {
        let result = pack(&root, &tokens);
        assert_eq!(
            result.code, 2,
            "`cairn pack {tokens:?}` must be a usage error, not a silent success"
        );
        assert!(
            result.stderr.contains("install"),
            "the usage line must name the verbs: {}",
            result.stderr
        );
    }
    assert!(
        !root.join(MANIFEST).exists(),
        "a usage error must not write anything"
    );
}

#[test]
fn uninstall_leaves_directories_that_still_hold_user_files() {
    let root = temp_root("prune");
    assert_eq!(pack(&root, &["install"]).code, 0);
    // A hand-authored skill the packager never owned, in a directory it does.
    let hand_authored = root.join(".claude/skills/my-own/SKILL.md");
    fs::create_dir_all(hand_authored.parent().unwrap()).unwrap();
    fs::write(&hand_authored, "# mine\n").unwrap();

    assert_eq!(pack(&root, &["uninstall"]).code, 0);
    assert!(
        hand_authored.exists(),
        "uninstall must not touch a skill cairn never wrote"
    );
    assert!(
        root.join(".claude").exists(),
        "pruning must stop at a directory that still holds user files"
    );
}

#[test]
fn a_plain_update_repairs_owned_loop_assets_and_a_stale_ledger_hash() {
    let root = temp_root("repair");
    assert_eq!(pack(&root, &["install", "--loop"]).code, 0);

    // Simulate an update interrupted after the bytes landed but before the
    // ledger was published: the disk holds the bundled content, the manifest
    // records something else.
    let mut ledger: serde_json::Value = manifest_json(&root);
    for file in ledger["files"].as_array_mut().unwrap() {
        if file["path"] == LOOP_MODE {
            file["sha256"] = serde_json::Value::String("0".repeat(64));
        }
    }
    fs::write(
        root.join(MANIFEST),
        serde_json::to_string_pretty(&ledger).unwrap(),
    )
    .unwrap();

    // No --loop: an update must still repair what the ledger already owns.
    let update = pack(&root, &["update"]);
    assert_eq!(update.code, 0, "update must succeed: {}", update.stderr);
    assert!(
        !update.stdout.contains(LOOP_MODE),
        "a file already holding the bundled bytes is not a user edit:\n{}",
        update.stdout
    );

    let repaired = manifest_json(&root);
    let recorded = repaired["files"]
        .as_array()
        .unwrap()
        .iter()
        .find(|file| file["path"] == LOOP_MODE)
        .expect("the ledger must still own the loop asset")["sha256"]
        .as_str()
        .unwrap()
        .to_owned();
    assert_ne!(
        recorded,
        "0".repeat(64),
        "update must re-record the real hash rather than leave the file trapped as modified"
    );

    let status: serde_json::Value =
        serde_json::from_str(&pack(&root, &["status", "--json"]).stdout).unwrap();
    assert!(
        status["data"]["modified"].as_array().unwrap().is_empty(),
        "nothing should be left reported as modified: {status}"
    );
}

#[test]
fn a_mid_update_write_failure_preserves_the_complete_ownership_ledger() {
    let root = temp_root("partial-failure");
    assert_eq!(pack(&root, &["install"]).code, 0);
    let before = manifest_json(&root);
    let before_paths: Vec<String> = before["files"]
        .as_array()
        .unwrap()
        .iter()
        .map(|file| file["path"].as_str().unwrap().to_owned())
        .collect();

    // A directory at an owned file path makes atomic replacement fail after
    // earlier assets have already been classified. The recovery publication
    // must not drop later rows from the ledger.
    let blocked = root.join(".claude/skills/cairn-propose/SKILL.md");
    fs::remove_file(&blocked).unwrap();
    fs::create_dir(&blocked).unwrap();

    let update = pack(&root, &["update"]);
    assert_ne!(update.code, 0, "the impossible file write must fail");

    let after = manifest_json(&root);
    let after_paths: Vec<String> = after["files"]
        .as_array()
        .unwrap()
        .iter()
        .map(|file| file["path"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(
        after_paths, before_paths,
        "a partial failure must preserve ownership of every pre-existing row"
    );
}

#[test]
fn an_unreadable_manifest_is_an_error_not_a_legacy_install() {
    let root = temp_root("manifest-io");
    assert_eq!(pack(&root, &["install"]).code, 0);
    let router_before = fs::read_to_string(root.join(ROUTER)).unwrap();

    fs::remove_file(root.join(MANIFEST)).unwrap();
    fs::create_dir(root.join(MANIFEST)).unwrap();

    let update = pack(&root, &["update"]);
    assert_ne!(
        update.code, 0,
        "a manifest that exists but cannot be read is not an absent legacy ledger"
    );
    assert!(
        update.stderr.contains(MANIFEST),
        "the failure must identify the unreadable ledger: {}",
        update.stderr
    );
    assert_eq!(
        fs::read_to_string(root.join(ROUTER)).unwrap(),
        router_before,
        "no pack bytes may change when ownership cannot be established"
    );
}
