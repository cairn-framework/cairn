//! The reconcile step of `cairn-dev` loop mode.
//!
//! `todo.agent-guidance-campaign-reconciliation` requires more than a recipe
//! that exists: the loop must load and run it after proof and before the
//! terminal token, and its plan edits must land inside the unit's single
//! commit. A recipe the loop never reaches leaves the next fresh session
//! reading a stale plan, which is the failure this unit exists to close.
//!
//! The subject is markdown, so an obligation can only be pinned by the sentence
//! that carries it. Phrase assertions match a whitespace-flattened body, and
//! each one names an obligation the recipe would stop enforcing if the sentence
//! disappeared. Rewording deliberately breaks them: that is the review prompt,
//! not a false positive.

use std::path::{Path, PathBuf};

fn pack_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn loop_mode() -> String {
    std::fs::read_to_string(pack_dir().join("content/skills/cairn-dev/references/loop-mode.md"))
        .unwrap()
}

fn recipe() -> String {
    std::fs::read_to_string(pack_dir().join("content/skills/cairn-loop-reconcile/SKILL.md"))
        .unwrap()
}

/// Hard-wrapped prose puts line breaks in arbitrary places, so phrase
/// assertions match against a single-spaced flattening rather than the wrap.
fn recipe_prose() -> String {
    recipe().split_whitespace().collect::<Vec<_>>().join(" ")
}

fn offset(body: &str, needle: &str) -> usize {
    body.find(needle)
        .unwrap_or_else(|| panic!("loop mode no longer contains {needle:?}"))
}

#[test]
fn loop_mode_reaches_reconcile_after_the_gate_and_before_the_terminal_token() {
    let body = loop_mode();
    let verify = offset(&body, "\n## Verify: the gate");
    let reconcile = offset(&body, "\n## Reconcile the plan");
    let land = offset(&body, "\n## Land and Cleanup");
    let end = offset(&body, "\n## End");

    assert!(
        verify < reconcile,
        "reconcile must run after proof; a plan reconciled before the gate can be \
         invalidated by the fix the gate forces"
    );
    assert!(
        reconcile < land,
        "reconcile must run before Land, which owns staging: edits made after it \
         cannot reach this unit's commit"
    );
    assert!(
        land < end,
        "the terminal token is emitted at End, which must stay last"
    );
}

#[test]
fn the_reconcile_step_delegates_to_the_required_asset() {
    let body = loop_mode();
    let start = offset(&body, "\n## Reconcile the plan");
    let section = &body[start..start + offset(&body[start..], "\n## Land and Cleanup")];

    assert!(
        section.contains("cairn-loop-reconcile"),
        "the step must name the asset it loads, or loop mode has inlined the \
         procedure and created a second authority"
    );
    assert!(
        section.contains("RECONCILED"),
        "the step must name the exit it routes on"
    );
    assert!(
        !body.contains("\n## Record\n"),
        "the old Record step still exists alongside reconcile, so decision \
         authoring has two homes in one procedure"
    );
}

#[test]
fn the_recipe_claims_no_terminal_token_of_its_own() {
    // The positive direction (RECONCILED, LOOP HALTED are declared) is covered
    // for every procedure by router_route_tests. Only the negative is this
    // recipe's own risk: a provenance step that emits a terminal token would
    // end the iteration before it lands.
    let body = recipe();
    for foreign in ["ITERATION COMPLETE", "LOOP EXHAUSTED"] {
        assert!(
            !body.contains(foreign),
            "the recipe names {foreign}, a terminal token only loop mode's End step \
             and the landing procedure may emit"
        );
    }
}

#[test]
fn the_recipe_maps_each_obligation_to_a_sanctioned_authoring_path() {
    let body = recipe();
    for path in [
        "meta/research/",
        "$CAIRN decision new",
        "$CAIRN todo set",
        "$CAIRN todo new",
    ] {
        assert!(
            body.contains(path),
            "the recipe leaves an obligation without its authoring path: {path} is \
             never named, so the step is prose rather than a procedure"
        );
    }
    assert!(
        recipe_prose().contains("read-only queries, not creation verbs"),
        "the recipe must say which cairn verbs only read, or an agent will try to \
         create artefacts with a query"
    );
}

#[test]
fn the_recipe_lands_its_edits_inside_the_units_commit() {
    let prose = recipe_prose();
    assert!(
        prose.contains("Land stages them"),
        "the recipe must hand its edits to Land; a reconciliation the unit's commit \
         does not carry is invisible to the next fresh session"
    );
    assert!(
        prose.contains("never commits or pushes"),
        "the recipe must disclaim committing, or it competes with the landing \
         procedure for the single commit"
    );
}

#[test]
fn the_recipe_blocks_a_dependant_only_against_its_full_dependency_list() {
    let body = recipe();
    assert!(
        recipe_prose().contains("every entry in its own `Depends on` list is done"),
        "the recipe must require checking the whole Depends on list; one landed unit \
         can satisfy one dependency while others remain"
    );
    assert!(
        body.contains("status: accepted"),
        "the verdict-gated rule must key on an accepted decision, not on a round \
         merely being done"
    );
}

#[test]
fn the_worked_scenario_shows_a_blocked_dependant_with_a_linked_decision() {
    let body = recipe();
    let start = body
        .find("## Worked scenario")
        .expect("the recipe must carry one worked campaign scenario");
    let scenario = &body[start..];
    assert!(
        scenario.contains("todo set") && scenario.contains("blocked"),
        "the scenario must show the dependant actually moved to blocked"
    );
    assert!(
        scenario.contains("decision new"),
        "the scenario must show the decision that justifies the block"
    );
    assert!(
        scenario.contains("commits once") || scenario.contains("same commit"),
        "the scenario must show both landing in one commit"
    );
}

#[test]
fn the_recipe_never_becomes_a_selector() {
    let body = recipe();
    assert!(
        recipe_prose().contains("never selects the next unit"),
        "the recipe must disclaim selection; `dec.no-orchestrator` keeps scheduling \
         out of Cairn"
    );
    assert!(
        !body.contains("cairn next"),
        "the recipe reaches for a selector, which belongs to the harness"
    );
}
