//! `cairn pack resolve` and `cairn pack campaign` acceptance tests.
//!
//! Resolution turns an installed pack into an exact set of bytes; a campaign
//! makes those bytes immutable for its duration. The load-bearing properties
//! are that the resolver consumes loop mode's own declared closure, that a
//! session loads pinned copies rather than live pack paths, and that any
//! mismatch halts before work instead of being reported as information
//! (`dec.unified-cairn-dev-entry` clause 9).

use std::fs;
use std::path::{Path, PathBuf};

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

const SNAPSHOT: &str = ".cairn/state/agent-pack-campaign.json";
const SCOPE: &str = ".claude/skills/cairn-loop-scope/SKILL.md";
const ROUTER: &str = ".claude/skills/cairn-dev/SKILL.md";
const OMP_LOOP_MODE: &str = ".omp/skills/cairn-dev/references/loop-mode.md";

/// The pinned root a campaign reports. It is per-campaign, so a later campaign
/// can never substitute bytes at a path an earlier session verified.
fn pinned_root(root: &Path, tokens: &[&str]) -> PathBuf {
    let reported = resolved(root, tokens);
    root.join(reported["data"]["pinned_root"].as_str().unwrap())
}

fn resolved(root: &Path, tokens: &[&str]) -> serde_json::Value {
    let result = pack(root, tokens);
    assert_eq!(result.code, 0, "resolution must succeed: {}", result.stderr);
    serde_json::from_str(&result.stdout).unwrap()
}

#[test]
fn resolve_reports_the_loop_prompt_and_its_declared_closure() {
    let root = temp_root("resolve");
    assert_eq!(pack(&root, &["install", "--loop"]).code, 0);

    let data = resolved(&root, &["resolve", "--loop", "--json"])["data"].clone();
    assert_eq!(data["entry"], "loop");
    assert_eq!(data["prompt"]["path"], ".claude/commands/cairn-loop.md");

    // The closure is loop mode's own declared list, in its declared order.
    let declared: Vec<String> = fs::read_to_string(root.join(LOOP_MODE))
        .unwrap()
        .split("## Required asset closure")
        .nth(1)
        .unwrap()
        .split("```")
        .nth(1)
        .unwrap()
        .trim_start_matches("text")
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| format!(".claude/{line}"))
        .collect();
    let resolved_paths: Vec<String> = data["closure"]
        .as_array()
        .unwrap()
        .iter()
        .map(|asset| asset["path"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(
        resolved_paths, declared,
        "the resolver must consume the declared closure and no other list"
    );
    // `dec.loop-reconcile-step` added reconcile to the closure: the resolver
    // must carry it, not merely agree with whatever the document happens to say.
    for required in [LOOP_MODE, ".claude/skills/cairn-loop-reconcile/SKILL.md"] {
        assert!(
            resolved_paths.iter().any(|path| path == required),
            "the resolved closure must include {required}: {resolved_paths:?}"
        );
    }
    assert!(
        data["bundle_digest"].as_str().unwrap().len() == 64
            && data["cli_digest"].as_str().unwrap().len() == 64,
        "resolution must carry bundle and CLI hashes"
    );
}

#[test]
fn resolve_refuses_an_edited_or_unowned_asset() {
    let root = temp_root("resolve-drift");
    assert_eq!(pack(&root, &["install", "--loop"]).code, 0);

    let scope = root.join(SCOPE);
    let original = fs::read_to_string(&scope).unwrap();
    fs::write(&scope, format!("{original}edited\n")).unwrap();
    let drifted = pack(&root, &["resolve", "--loop"]);
    assert_ne!(drifted.code, 0, "an edited asset must not resolve");
    assert!(drifted.stderr.contains(SCOPE), "stderr: {}", drifted.stderr);

    fs::write(&scope, original).unwrap();
    assert_eq!(pack(&root, &["resolve", "--loop"]).code, 0);

    fs::remove_file(&scope).unwrap();
    let missing = pack(&root, &["resolve", "--loop"]);
    assert_ne!(missing.code, 0, "a missing asset must not resolve");
}

#[test]
fn a_campaign_pins_bytes_verifies_them_and_halts_on_any_change() {
    let root = temp_root("campaign");
    assert_eq!(pack(&root, &["install", "--loop"]).code, 0);

    assert_eq!(pack(&root, &["campaign", "start", "--loop"]).code, 0);
    let pinned_snapshot = fs::read_to_string(root.join(SNAPSHOT)).unwrap();
    let pinned: serde_json::Value = serde_json::from_str(&pinned_snapshot).unwrap();
    assert_eq!(pinned["entry"], "loop");
    let pinned_scope = pinned_root(&root, &["campaign", "verify", "--json"]).join(SCOPE);
    assert_eq!(
        fs::read_to_string(&pinned_scope).unwrap(),
        fs::read_to_string(root.join(SCOPE)).unwrap(),
        "start must pin an immutable copy of every closure asset"
    );
    assert!(
        fs::metadata(&pinned_scope)
            .unwrap()
            .permissions()
            .readonly(),
        "pinned copies must not be writable by the session that reads them"
    );

    assert_eq!(
        pack(&root, &["campaign", "verify"]).code,
        0,
        "unchanged bytes must verify"
    );

    // An active campaign is immutable: repinning is refused, not silently done.
    let repin = pack(&root, &["campaign", "start", "--loop"]);
    assert_ne!(repin.code, 0, "an active campaign must not be repinned");
    assert_eq!(
        fs::read_to_string(root.join(SNAPSHOT)).unwrap(),
        pinned_snapshot,
        "the refused start must leave the pinned bytes untouched"
    );

    let scope = root.join(SCOPE);
    fs::write(&scope, "tampered\n").unwrap();
    let halted = pack(&root, &["campaign", "verify"]);
    assert_ne!(halted.code, 0, "changed bytes must halt the campaign");
    assert!(halted.stderr.contains("HALT"), "stderr: {}", halted.stderr);

    // Outside a campaign the same drift is information, not failure.
    assert_eq!(pack(&root, &["campaign", "end"]).code, 0);
    assert!(!root.join(SNAPSHOT).exists());
    assert!(!pinned_scope.exists(), "end must release the pinned copies");
    let status = pack(&root, &["status", "--loop", "--json"]);
    assert_eq!(status.code, 0, "drift outside a campaign must not fail");
    let reported: serde_json::Value = serde_json::from_str(&status.stdout).unwrap();
    assert!(
        reported["data"]["modified"]
            .as_array()
            .unwrap()
            .iter()
            .any(|path| path == SCOPE),
        "status must still report the drift it refuses to fix"
    );
}

#[test]
fn a_repair_restores_the_campaign_but_a_real_change_halts_it() {
    let root = temp_root("campaign-update");
    assert_eq!(pack(&root, &["install", "--loop"]).code, 0);
    assert_eq!(pack(&root, &["campaign", "start", "--loop"]).code, 0);

    // Backfilling a deleted asset restores the exact pinned bytes, so the
    // campaign is intact: pinning is about content, not about write events.
    fs::remove_file(root.join(SCOPE)).unwrap();
    assert_eq!(pack(&root, &["update", "--loop"]).code, 0);
    let repaired = pack(&root, &["campaign", "verify"]);
    assert_eq!(
        repaired.code, 0,
        "restored identical bytes still satisfy the pin: {}",
        repaired.stderr
    );

    // Dropping loop assets from the ledger is a different revision of the pack.
    assert_eq!(pack(&root, &["uninstall"]).code, 0);
    assert_eq!(pack(&root, &["install"]).code, 0);
    let halted = pack(&root, &["campaign", "verify"]);
    assert_ne!(halted.code, 0, "a reinstalled pack must halt the campaign");
    assert!(halted.stderr.contains("HALT"), "{}", halted.stderr);
    assert!(
        root.join(SNAPSHOT).exists(),
        "a halt must leave the campaign pinned"
    );

    let unknown = pack(&root, &["campaign", "restart"]);
    assert_eq!(unknown.code, 2, "an unknown verb is a usage error");
}

#[test]
fn an_edit_after_verification_cannot_reach_the_pinned_bytes() {
    let root = temp_root("campaign-post-check");
    assert_eq!(pack(&root, &["install", "--loop"]).code, 0);
    assert_eq!(pack(&root, &["campaign", "start", "--loop"]).code, 0);
    assert_eq!(pack(&root, &["campaign", "verify"]).code, 0);

    let pinned_loop_mode = pinned_root(&root, &["campaign", "verify", "--json"]).join(LOOP_MODE);
    let pinned_before = fs::read_to_string(&pinned_loop_mode).unwrap();
    fs::write(root.join(LOOP_MODE), "# not the procedure\n").unwrap();

    assert_eq!(
        fs::read_to_string(&pinned_loop_mode).unwrap(),
        pinned_before,
        "a post-verification edit must not reach the bytes the session loads"
    );
    let next_session = pack(&root, &["campaign", "verify"]);
    assert_ne!(
        next_session.code, 0,
        "the next fresh session must halt on the edited pack"
    );
}

#[test]
fn the_router_entry_pins_the_index_a_session_starts_from() {
    let root = temp_root("campaign-router");
    assert_eq!(pack(&root, &["install"]).code, 0);

    let resolution = resolved(&root, &["resolve", "--json"])["data"].clone();
    assert_eq!(resolution["entry"], "router");
    assert_eq!(
        resolution["prompt"]["path"], ROUTER,
        "the router entry resolves to the index the session reads first"
    );
    assert!(
        resolution["closure"].as_array().unwrap().is_empty(),
        "the router loads its references just in time, so it pins no closure"
    );

    assert_eq!(
        pack(&root, &["campaign", "start"]).code,
        0,
        "a campaign on the default entry must start without --loop"
    );
    let pinned = pinned_root(&root, &["campaign", "verify", "--json"]).join(ROUTER);
    assert_eq!(
        fs::read_to_string(&pinned).unwrap(),
        fs::read_to_string(root.join(ROUTER)).unwrap(),
        "the pinned router must hold the resolved bytes"
    );
    assert_eq!(pack(&root, &["campaign", "end"]).code, 0);
    assert!(!pinned.exists(), "end must release the router campaign");
}

#[test]
fn a_tampered_snapshot_never_reaches_the_filesystem() {
    let root = temp_root("campaign-tampered");
    assert_eq!(pack(&root, &["install", "--loop"]).code, 0);
    assert_eq!(pack(&root, &["campaign", "start", "--loop"]).code, 0);

    // The snapshot is an ordinary file. A digest that is really a traversal
    // must not become a directory this command deletes or reads.
    let mut snapshot: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(root.join(SNAPSHOT)).unwrap()).unwrap();
    snapshot["bundle_digest"] = serde_json::json!("../../../victim");
    fs::write(
        root.join(SNAPSHOT),
        serde_json::to_string_pretty(&snapshot).unwrap(),
    )
    .unwrap();
    // `.cairn/state/campaign/../../../victim` resolves to `<root>/victim`.
    let victim = root.join("victim");
    fs::create_dir_all(&victim).unwrap();
    fs::write(victim.join("keep.txt"), "keep\n").unwrap();

    let verified = pack(&root, &["campaign", "verify"]);
    assert_ne!(verified.code, 0, "a tampered snapshot must not verify");
    let ended = pack(&root, &["campaign", "end"]);
    assert_eq!(ended.code, 0, "end must release it: {}", ended.stderr);
    assert!(
        victim.join("keep.txt").exists(),
        "no snapshot value may direct a delete outside the campaign tree"
    );
    assert!(!root.join(SNAPSHOT).exists());
}

#[test]
fn an_interrupted_claim_halts_but_can_still_be_released() {
    let root = temp_root("campaign-interrupted");
    assert_eq!(pack(&root, &["install", "--loop"]).code, 0);

    // A start that died between claiming the snapshot and filling it.
    fs::create_dir_all(root.join(SNAPSHOT).parent().unwrap()).unwrap();
    fs::write(root.join(SNAPSHOT), "{\"entry\":").unwrap();

    assert_ne!(
        pack(&root, &["campaign", "verify"]).code,
        0,
        "an unreadable claim must halt, never pass"
    );
    assert_ne!(
        pack(&root, &["campaign", "start", "--loop"]).code,
        0,
        "an unreleased claim must block a new campaign"
    );
    assert_eq!(
        pack(&root, &["campaign", "end"]).code,
        0,
        "end must recover an interrupted claim"
    );
    assert_eq!(
        pack(&root, &["campaign", "start", "--loop"]).code,
        0,
        "a released campaign can start again"
    );
}

#[test]
fn an_omp_campaign_pins_the_closure_under_the_omp_pack_root() {
    let root = temp_root("omp-campaign");
    assert_eq!(
        pack(&root, &["install", "--harness", "omp", "--loop"]).code,
        0
    );

    let resolution = resolved(&root, &["resolve", "--loop", "--json"]);
    assert_eq!(resolution["data"]["harness"], "omp");
    let closure: Vec<&str> = resolution["data"]["closure"]
        .as_array()
        .unwrap()
        .iter()
        .map(|asset| asset["path"].as_str().unwrap())
        .collect();
    assert!(
        closure.iter().all(|path| path.starts_with(".omp/")),
        "the declared closure must resolve under the installed pack root: {closure:?}"
    );
    assert!(closure.contains(&OMP_LOOP_MODE));

    assert_eq!(pack(&root, &["campaign", "start", "--loop"]).code, 0);
    assert_eq!(
        pack(&root, &["campaign", "verify", "--loop"]).code,
        0,
        "an untouched OMP pack must verify"
    );

    fs::write(
        root.join(OMP_LOOP_MODE),
        "edited under the running campaign\n",
    )
    .unwrap();
    let halted = pack(&root, &["campaign", "verify", "--loop"]);
    assert_ne!(
        halted.code, 0,
        "a changed procedure must halt an OMP campaign before work"
    );
    assert!(halted.stderr.contains("HALT") || halted.stdout.contains("HALT"));
}
