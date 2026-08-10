//! Route integrity for the `cairn-dev` router and its loop-mode closure.
//!
//! The router is an index: every row points at a reference the pack must
//! actually ship, and loop mode names an ordered required-asset closure that
//! adapters and campaign locks consume. These tests prove the pointers resolve,
//! so a route cannot silently dangle after a rename.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn pack_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn manifest() -> toml::Value {
    let text = std::fs::read_to_string(pack_dir().join("manifest.toml")).unwrap();
    toml::from_str(&text).unwrap()
}

/// canonical source path -> claude destination, for the whole pack.
fn source_to_destination() -> BTreeMap<String, String> {
    let manifest = manifest();
    let sources: BTreeMap<(String, String), String> = manifest["canonical"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            (
                (
                    row["entry"].as_str().unwrap().to_owned(),
                    row["mode"].as_str().unwrap().to_owned(),
                ),
                row["source"].as_str().unwrap().to_owned(),
            )
        })
        .collect();
    manifest["adapters"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|row| row["harness"].as_str() == Some("claude"))
        .map(|row| {
            let key = (
                row["entry"].as_str().unwrap().to_owned(),
                row["mode"].as_str().unwrap().to_owned(),
            );
            (
                sources[&key].clone(),
                row["destination"].as_str().unwrap().to_owned(),
            )
        })
        .collect()
}

fn router_body() -> String {
    std::fs::read_to_string(pack_dir().join("content/skills/cairn-dev/SKILL.md")).unwrap()
}

/// Every `references/<file>.md` the router names, from its route table and prose.
fn routes_named_by_router() -> BTreeSet<String> {
    let body = router_body();
    let mut named = BTreeSet::new();
    let mut rest = body.as_str();
    while let Some(start) = rest.find("`references/") {
        let tail = &rest[start + 1..];
        let end = tail.find('`').unwrap();
        named.insert(tail[..end].to_owned());
        rest = &tail[end..];
    }
    named
}

#[test]
fn every_router_route_resolves_to_a_shipped_reference() {
    let shipped: BTreeSet<String> = source_to_destination()
        .keys()
        .filter_map(|source| {
            source
                .strip_prefix("content/skills/cairn-dev/")
                .map(str::to_owned)
        })
        .collect();
    let named = routes_named_by_router();
    assert!(
        !named.is_empty(),
        "the router names no references at all, so the route table is broken"
    );
    let dangling: Vec<_> = named.difference(&shipped).collect();
    assert!(
        dangling.is_empty(),
        "the router points at references the pack does not ship: {dangling:?}"
    );
}

#[test]
fn every_shipped_reference_is_reachable_from_the_router() {
    let shipped: BTreeSet<String> = source_to_destination()
        .keys()
        .filter_map(|source| {
            source
                .strip_prefix("content/skills/cairn-dev/")
                .map(str::to_owned)
        })
        .filter(|relative| relative.starts_with("references/"))
        .collect();
    let named = routes_named_by_router();
    let unreachable: Vec<_> = shipped.difference(&named).collect();
    assert!(
        unreachable.is_empty(),
        "the pack ships references no route reaches, so a session can never load \
         them: {unreachable:?}"
    );
}

/// The ordered closure declared in loop mode's fenced `text` block.
fn required_asset_closure() -> Vec<String> {
    let body = std::fs::read_to_string(
        pack_dir().join("content/skills/cairn-dev/references/loop-mode.md"),
    )
    .unwrap();
    let heading = body
        .find("## Required asset closure")
        .expect("loop mode must declare a required asset closure");
    let after = &body[heading..];
    let open = after.find("```text").expect("closure must be a text block");
    let rest = &after[open + "```text".len()..];
    let close = rest.find("```").expect("unterminated closure block");
    rest[..close]
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect()
}

#[test]
fn the_loop_required_asset_closure_is_complete_and_shipped() {
    let closure = required_asset_closure();
    let shipped: BTreeSet<String> = source_to_destination()
        .keys()
        .filter_map(|source| source.strip_prefix("content/").map(str::to_owned))
        .collect();

    for asset in &closure {
        assert!(
            shipped.contains(asset),
            "loop mode requires {asset}, which the pack does not ship, so loop mode \
             would fail closed on every invocation"
        );
    }

    // The closure is the contract both adapters and campaign locks read, so the
    // five procedures loop mode delegates to must all be in it.
    for procedure in [
        "skills/cairn-loop-scope/SKILL.md",
        "skills/cairn-loop-implement/SKILL.md",
        "skills/cairn-loop-recovery/SKILL.md",
        "skills/cairn-loop-reconcile/SKILL.md",
        "skills/cairn-loop-landing/SKILL.md",
    ] {
        assert!(
            closure.iter().any(|asset| asset == procedure),
            "{procedure} is delegated to by loop mode but missing from its required \
             asset closure"
        );
    }
    assert_eq!(
        closure.first().map(String::as_str),
        Some("skills/cairn-dev/references/loop-mode.md"),
        "the closure is ordered and must open with loop mode itself"
    );
}

#[test]
fn each_loop_procedure_declares_its_typed_exits() {
    let expected: [(&str, &[&str]); 5] = [
        (
            "skills/cairn-loop-scope/SKILL.md",
            &["SCOPED", "REROUTED", "LOOP HALTED"],
        ),
        (
            "skills/cairn-loop-implement/SKILL.md",
            &["IMPLEMENTED", "LOOP HALTED"],
        ),
        (
            "skills/cairn-loop-recovery/SKILL.md",
            &["RECOVERED", "LOOP HALTED"],
        ),
        (
            "skills/cairn-loop-reconcile/SKILL.md",
            &["RECONCILED", "LOOP HALTED"],
        ),
        (
            "skills/cairn-loop-landing/SKILL.md",
            &["ITERATION COMPLETE", "LOOP HALTED"],
        ),
    ];
    let loop_mode = std::fs::read_to_string(
        pack_dir().join("content/skills/cairn-dev/references/loop-mode.md"),
    )
    .unwrap();
    for (asset, tokens) in expected {
        let body = std::fs::read_to_string(pack_dir().join("content").join(asset)).unwrap();
        for token in tokens {
            assert!(
                body.contains(token),
                "{asset} is routed on {token} but never declares it"
            );
            assert!(
                loop_mode.contains(token),
                "loop mode routes on {token} from {asset} but never names it"
            );
        }
    }
}

#[test]
fn the_adapter_command_carries_no_procedure_of_its_own() {
    let command =
        std::fs::read_to_string(pack_dir().join("content/commands/cairn-loop.md")).unwrap();
    assert!(
        command.contains("skills/cairn-dev/references/loop-mode.md"),
        "the adapter-native command must resolve to canonical loop mode"
    );
    // A second authority is the failure this unit exists to prevent: transport
    // that restates the procedure drifts from it.
    for procedural in ["## Preflight", "| State | Action |", "Sizing rule"] {
        assert!(
            !command.contains(procedural),
            "the adapter command restates loop-mode procedure ({procedural}), creating a \
             competing authority"
        );
    }
    assert!(
        command.len() < 2_500,
        "the adapter command is transport; at {} bytes it is carrying procedure",
        command.len()
    );
}

#[test]
fn the_retired_generic_coding_skill_is_gone_everywhere() {
    let root = repo_root();
    assert!(
        !root.join(".claude/skills/karpathy-guidelines").exists(),
        "the retired karpathy-guidelines skill is still installed"
    );
    let agents = std::fs::read_to_string(root.join("AGENTS.md")).unwrap();
    assert!(
        !agents.contains("karpathy-guidelines"),
        "AGENTS.md still points at the retired karpathy-guidelines skill"
    );
}

#[test]
fn every_manifest_destination_is_tracked_by_git() {
    // A local `.git/info/exclude` entry for `.claude/skills` silently keeps new
    // rendered destinations untracked, which ships a pack whose assets are
    // missing from the crate. Catch that here rather than in a user's repo.
    let root = repo_root();
    let untracked: Vec<String> = source_to_destination()
        .values()
        .filter(|destination| {
            let status = std::process::Command::new("git")
                .arg("ls-files")
                .arg("--error-unmatch")
                .arg(destination)
                .current_dir(&root)
                .output();
            match status {
                Ok(output) => !output.status.success(),
                // No git available (packaged crate, sandbox): skip rather than fail.
                Err(_) => false,
            }
        })
        .cloned()
        .collect();
    assert!(
        untracked.is_empty(),
        "rendered destinations exist on disk but are not tracked by git, so they \
         will not ship: {untracked:?} (add with `git add -f`)"
    );
}

#[test]
fn the_brownfield_extraction_reference_keeps_drafts_proposed() {
    // The reference hands a model the decision writer, so the clauses that stop
    // it from self-ratifying an inferred decision are the shipped contract
    // (`dec.brownfield-extraction-mechanism` clauses 2 and 3). A green scan
    // proves artefact integrity, never that these sentences survived an edit.
    let raw = std::fs::read_to_string(
        pack_dir()
            .join("content/skills/cairn-dev/references/task-brownfield-decision-extraction.md"),
    )
    .unwrap();
    // Matched against whitespace-collapsed prose: a reflowed line break inside a
    // clause is not a lost clause, and failing on one would teach the next author
    // to widen the paragraph rather than keep the rule.
    let body = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    for clause in [
        "cairn onboard decisions --json",
        "cairn decision new <slug> --node <id> --informed-by res.<slug>",
        "method: primary",
        "Never pass it as a node id",
        "Leave every extracted decision at `status: proposed`.",
        "Do not set `status: accepted`, `ratified_by`, `receipts`, or `supersedes`.",
        "Do not use `cairn gap`",
        "keep the `cairn onboard decisions` report that produced each draft with it",
        "put the draft to the maintainer for acceptance or rejection",
        "Never accept your own extracted decision.",
        // A second decision writer is the failure mode the ruling names: every
        // draft goes through the existing command.
        "do not add a second writer",
    ] {
        assert!(
            body.contains(clause),
            "the brownfield extraction reference no longer carries its required \
             clause: {clause:?}"
        );
    }
}

#[test]
fn the_brownfield_extraction_reference_carries_the_external_run_guards() {
    // Two prose defects the external `rancher/turtles` run hit
    // (`res.brownfield-extraction-external-run` sections 1 and 7): step 0 assumed
    // a System block the brownfield entry point never writes, and section 3's
    // read-every-candidate step was load-bearing without saying why. Both cost
    // that run real work, and both are invisible to a green scan, so pin them.
    let raw = std::fs::read_to_string(
        pack_dir()
            .join("content/skills/cairn-dev/references/task-brownfield-decision-extraction.md"),
    )
    .unwrap();
    let body = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    for clause in [
        // Gap 1: the fact, the action it implies, and why the wrapper is advice
        // rather than a parser rule. The action is the half that closes the gap.
        "`cairn init --from-code --apply` writes no System block",
        "Cairn collects these pointers from any node at any depth",
        "so a System block is a convention, not a parser requirement",
        "Wrap that list in a System and declare both pointers there",
        // Gap 2: the instruction, the wire property that makes it load-bearing,
        // and the run that proves it. All three chains, because one surviving
        // example would read as an anecdote rather than as a pattern.
        "Read every candidate at its `path` and `line`",
        "one flat list with no status and no supersession",
        "ADR 0009 reversed accepted ADR 0005",
        "ADRs 0008 and 0011 retired half of ADR 0003",
        "ADR 0011 superseded ADR 0010",
        "was withdrawn once all 19 were read",
    ] {
        assert!(
            body.contains(clause),
            "the brownfield extraction reference no longer warns about a gap the \
             external run found: {clause:?}"
        );
    }
}
