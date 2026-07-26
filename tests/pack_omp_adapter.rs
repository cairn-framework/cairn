//! `cairn pack --harness omp` acceptance tests.
//!
//! The OMP adapter is a pack root and nothing else (`dec.pack-adapter-roots`):
//! the same bytes at the destinations OMP's own `native` provider discovers.
//! The load-bearing properties are that an install lands there and nowhere
//! else, that every later verb follows the harness the ledger records, and
//! that the runtime roots still match the destinations the canonical manifest
//! declares.

use std::fs;
use std::path::{Path, PathBuf};

const MANIFEST: &str = ".cairn/state/agent-pack.json";
const OMP_ROUTER: &str = ".omp/skills/cairn-dev/SKILL.md";
const OMP_LOOP_COMMAND: &str = ".omp/commands/cairn-loop.md";

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
fn the_omp_adapter_installs_the_same_bytes_at_omp_native_destinations() {
    let root = temp_root("omp-install");

    let installed = pack(&root, &["install", "--harness", "omp", "--loop"]);
    assert_eq!(
        installed.code, 0,
        "install must succeed: {}",
        installed.stderr
    );
    assert!(
        root.join(OMP_ROUTER).exists(),
        "OMP discovers project skills under .omp/skills, so the router lands there"
    );
    assert!(
        root.join(OMP_LOOP_COMMAND).exists(),
        "OMP discovers project commands under .omp/commands"
    );
    assert!(
        !root.join(".claude").exists(),
        "an OMP install must not write the Claude tree"
    );
    assert_eq!(
        fs::read_to_string(root.join(OMP_ROUTER)).unwrap(),
        include_str!("../.claude/skills/cairn-dev/SKILL.md"),
        "a harness changes where bytes land, never what they say"
    );
    assert_eq!(manifest_json(&root)["harness"], "omp");
}

#[test]
fn later_verbs_follow_the_installed_harness_and_refuse_a_conflicting_selector() {
    let root = temp_root("omp-binding");
    assert_eq!(pack(&root, &["install", "--harness", "omp"]).code, 0);

    let status = pack(&root, &["status"]);
    assert_eq!(status.code, 0, "status must succeed: {}", status.stderr);
    assert!(
        status.stdout.contains("omp"),
        "a bare status must report the installed adapter, not the default one"
    );
    assert!(
        status.stdout.contains("missing: 0"),
        "a bare status must classify the installed tree, not a Claude tree that was never written: {}",
        status.stdout
    );

    let crossed = pack(&root, &["update", "--harness", "claude"]);
    assert_eq!(
        crossed.code, 2,
        "an explicit selector that disagrees with the ledger must be refused"
    );
    assert!(
        !root.join(".claude").exists(),
        "a refused selector must write nothing"
    );

    let resolved = pack(&root, &["resolve", "--harness", "claude"]);
    assert_eq!(
        resolved.code, 2,
        "resolve must honour the selector rather than silently following the ledger"
    );
}

#[test]
fn a_first_install_detects_an_omp_native_project() {
    let root = temp_root("omp-detect");
    fs::create_dir_all(root.join(".omp")).unwrap();

    assert_eq!(pack(&root, &["install"]).code, 0);
    assert_eq!(
        manifest_json(&root)["harness"],
        "omp",
        "an OMP-native project with no Claude tree installs the OMP adapter"
    );
    assert!(root.join(OMP_ROUTER).exists());
}

#[test]
fn an_unknown_or_valueless_harness_selector_is_a_usage_error() {
    let root = temp_root("omp-selector");

    let unknown = pack(&root, &["install", "--harness", "cursor"]);
    assert_eq!(unknown.code, 2, "an unvalidated adapter must not install");
    assert!(unknown.stderr.contains("cursor"));

    let valueless = pack(&root, &["install", "--harness"]);
    assert_eq!(
        valueless.code, 2,
        "a selector with no value must fail rather than fall back to detection"
    );
    assert!(
        !root.join(MANIFEST).exists(),
        "neither usage error may install anything"
    );
}

#[test]
fn the_runtime_adapter_roots_match_the_canonical_manifest() {
    // `tools/agent-pack/manifest.toml` declares one adapter row per harness
    // (`dec.agent-pack-packaging` clauses 1 and 2). The binary carries one copy
    // of the bytes and roots them per harness, so this test is what keeps the
    // two descriptions of the same adapter from drifting.
    let manifest: toml::Value =
        toml::from_str(&fs::read_to_string("tools/agent-pack/manifest.toml").unwrap()).unwrap();
    let destinations = |harness: &str| -> std::collections::BTreeSet<String> {
        manifest["adapters"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|row| row["harness"].as_str() == Some(harness))
            .map(|row| row["destination"].as_str().unwrap().to_owned())
            .collect()
    };
    let claude = destinations("claude");
    let omp = destinations("omp");
    assert!(!omp.is_empty(), "the manifest must declare the OMP adapter");
    let expected: std::collections::BTreeSet<String> = claude
        .iter()
        .map(|path| path.replacen(".claude/", ".omp/", 1))
        .collect();
    assert_eq!(
        omp, expected,
        "the OMP rows must be the Claude rows rooted at .omp/, which is what the binary installs"
    );

    let root = temp_root("omp-manifest-sync");
    assert_eq!(
        pack(&root, &["install", "--harness", "omp", "--loop"]).code,
        0
    );
    let owned: std::collections::BTreeSet<String> = manifest_json(&root)["files"]
        .as_array()
        .unwrap()
        .iter()
        .map(|file| file["path"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(
        owned, omp,
        "an OMP install must own exactly the destinations the manifest declares"
    );
}

#[test]
fn a_hand_edited_ledger_that_claims_two_trees_stops_every_verb() {
    let root = temp_root("omp-mixed-ledger");
    assert_eq!(pack(&root, &["install", "--harness", "omp"]).code, 0);

    // Flip the recorded harness without moving the files: the ledger now names
    // the Claude adapter while owning `.omp/` rows.
    let mut ledger = manifest_json(&root);
    ledger["harness"] = serde_json::Value::String("claude".to_owned());
    fs::write(
        root.join(MANIFEST),
        serde_json::to_string_pretty(&ledger).unwrap(),
    )
    .unwrap();

    for verb in [
        vec!["status"],
        vec!["update"],
        vec!["uninstall"],
        vec!["resolve"],
    ] {
        let result = pack(&root, &verb);
        assert_eq!(
            result.code,
            1,
            "`pack {}` must refuse a ledger whose rows are outside its own adapter",
            verb.join(" ")
        );
    }
    assert!(
        root.join(OMP_ROUTER).exists(),
        "a refused verb must not retire the files it could not classify"
    );
}

#[test]
fn releasing_a_campaign_never_depends_on_the_ownership_ledger() {
    let root = temp_root("omp-campaign-release");
    assert_eq!(
        pack(&root, &["install", "--harness", "omp", "--loop"]).code,
        0
    );
    assert_eq!(pack(&root, &["campaign", "start", "--loop"]).code, 0);

    fs::write(root.join(MANIFEST), "{ not json").unwrap();
    // `campaign end` is the documented way out of a stuck campaign, so it must
    // not be gated on a ledger the project can no longer parse.
    let released = pack(&root, &["campaign", "end"]);
    assert_eq!(
        released.code, 0,
        "campaign end must release the snapshot: {}",
        released.stderr
    );
    assert!(!root.join(".cairn/state/agent-pack-campaign.json").exists());
}

#[test]
fn a_crossed_selector_is_refused_by_the_resolver_that_reads_the_ledger() {
    let root = temp_root("omp-campaign-selector");
    assert_eq!(
        pack(&root, &["install", "--harness", "omp", "--loop"]).code,
        0
    );

    let started = pack(
        &root,
        &["campaign", "start", "--harness", "claude", "--loop"],
    );
    assert_eq!(
        started.code, 2,
        "campaign start must validate the selector against the ledger it resolves from"
    );
    assert!(!root.join(".cairn/state/agent-pack-campaign.json").exists());
}

#[test]
fn init_wires_the_adapter_it_installed() {
    let root = temp_root("omp-init-wire");
    fs::create_dir_all(root.join(".omp")).unwrap();
    fs::write(root.join("AGENTS.md"), "# Project\n").unwrap();

    let argv = [
        "--file".to_owned(),
        root.join("cairn.blueprint").to_string_lossy().to_string(),
        "init".to_owned(),
        "--wire".to_owned(),
    ];
    let result = cairn::cli::run(&argv);
    assert_eq!(
        result.code, 0,
        "init --wire must succeed: {}",
        result.stderr
    );

    let wired = fs::read_to_string(root.join("AGENTS.md")).unwrap();
    assert!(
        wired.contains(".omp/skills/cairn-dev/SKILL.md"),
        "an OMP project must be wired to the tree init actually wrote: {wired}"
    );
    assert!(
        !wired.contains(".claude/skills/cairn-dev/SKILL.md"),
        "wiring must not name a tree that was never written: {wired}"
    );
    assert!(
        !result.stdout.contains(".claude/skills/cairn-dev"),
        "next steps must name the installed pack root: {}",
        result.stdout
    );
}

#[test]
fn campaign_start_and_verify_stay_behind_the_ledger_gate() {
    let root = temp_root("omp-campaign-gate");
    assert_eq!(
        pack(&root, &["install", "--harness", "omp", "--loop"]).code,
        0
    );
    let mut ledger = manifest_json(&root);
    ledger["harness"] = serde_json::Value::String("claude".to_owned());
    fs::write(
        root.join(MANIFEST),
        serde_json::to_string_pretty(&ledger).unwrap(),
    )
    .unwrap();

    for verb in [
        vec!["campaign", "start", "--loop"],
        vec!["campaign", "verify", "--loop"],
    ] {
        let result = pack(&root, &verb);
        assert_eq!(
            result.code,
            1,
            "`pack {}` reads the pack, so it must refuse a mixed ledger",
            verb.join(" ")
        );
    }
    assert!(
        !root.join(".cairn/state/agent-pack-campaign.json").exists(),
        "a refused start must pin nothing"
    );
}

#[test]
fn the_agent_guide_and_wiring_follow_a_changed_adapter() {
    let root = temp_root("omp-adapter-switch");
    fs::write(root.join("AGENTS.md"), "# Project\n").unwrap();
    let init = [
        "--file".to_owned(),
        root.join("cairn.blueprint").to_string_lossy().to_string(),
        "init".to_owned(),
        "--wire".to_owned(),
    ];
    assert_eq!(cairn::cli::run(&init).code, 0);
    let guide = fs::read_to_string(root.join(".cairn/AGENTS.md")).unwrap();
    assert!(
        guide.contains(".claude/skills/cairn-dev/SKILL.md") && !guide.contains(".omp/"),
        "a Claude project's guide names the Claude router only: {guide}"
    );

    // Switch adapters the sanctioned way, then re-scaffold and re-wire.
    assert_eq!(pack(&root, &["uninstall"]).code, 0);
    fs::create_dir_all(root.join(".omp")).unwrap();
    let _ = fs::remove_dir_all(root.join(".claude"));
    fs::remove_file(root.join(".cairn/AGENTS.md")).unwrap();
    assert_eq!(cairn::cli::run(&init).code, 0);

    let guide = fs::read_to_string(root.join(".cairn/AGENTS.md")).unwrap();
    assert!(
        guide.contains(".omp/skills/cairn-dev/SKILL.md") && !guide.contains(".claude/"),
        "the guide must name the adapter now installed: {guide}"
    );
    let wired = fs::read_to_string(root.join("AGENTS.md")).unwrap();
    assert!(
        wired.contains(".omp/skills/cairn-dev/SKILL.md"),
        "wiring must be refreshed when the adapter changed: {wired}"
    );
    assert!(
        !wired.contains(".claude/skills/cairn-dev/SKILL.md"),
        "the stale block must not survive beside the new one: {wired}"
    );
    assert_eq!(
        wired.matches("<!-- cairn:agent-guide-begin -->").count(),
        1,
        "re-wiring must replace the block, not append a second one: {wired}"
    );
}

#[test]
fn a_truncated_orientation_block_is_reported_not_rewritten() {
    let root = temp_root("omp-wire-truncated");
    fs::create_dir_all(root.join(".omp")).unwrap();
    let instructions = "# Project\n\n<!-- cairn:agent-guide-begin -->\nhand-truncated\n";
    fs::write(root.join("AGENTS.md"), instructions).unwrap();

    let argv = [
        "--file".to_owned(),
        root.join("cairn.blueprint").to_string_lossy().to_string(),
        "init".to_owned(),
        "--wire".to_owned(),
    ];
    let result = cairn::cli::run(&argv);
    assert_ne!(
        result.code, 0,
        "a block with no closing marker must be reported, not rewritten"
    );
    assert_eq!(
        fs::read_to_string(root.join("AGENTS.md")).unwrap(),
        instructions,
        "the user's file must be left exactly as it was"
    );
}
