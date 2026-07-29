//! Selection skips todo-parked Info findings.
//!
//! `todo.lint-selection-folding` item 1a, ratified 2026-07-29 (PR #528 sheet
//! W2): an Info finding that a `blocked` todo parks through a matching
//! `defers:` reference (finding code plus the path or node it was raised
//! against) is not a selectable loop unit. The scanner half publishes
//! `parked_by` per finding on the lint/scan wire; this file pins the asset
//! half in every place selection can meet a parked finding: default
//! selection, both MISSION paths, and the stop evidence. Phrase assertions
//! match a whitespace-flattened body; rewording deliberately breaks them:
//! that is the review prompt, not a false positive.

use std::path::{Path, PathBuf};

fn pack_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn loop_mode() -> String {
    std::fs::read_to_string(pack_dir().join("content/skills/cairn-dev/references/loop-mode.md"))
        .unwrap()
}

/// Hard-wrapped prose puts line breaks in arbitrary places, so phrase
/// assertions match against a single-spaced flattening rather than the wrap.
fn flatten(body: &str) -> String {
    body.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The named section's body, flattened: from its `## ` heading to the next
/// `## ` heading (or end of file), so an obligation asserted here cannot be
/// satisfied by a sentence that drifted into an unrelated section.
fn section(heading: &str) -> String {
    let body = loop_mode();
    let start = body
        .find(heading)
        .unwrap_or_else(|| panic!("loop mode no longer contains section {heading:?}"));
    let rest = &body[start + heading.len()..];
    let end = rest.find("\n## ").map_or(rest.len(), |offset| offset);
    flatten(&rest[..end])
}

#[test]
fn default_selection_skips_parked_info_findings() {
    let select = section("\n## Select ONE unit");
    assert!(
        select.contains("every `info` finding that publishes a `parked_by`"),
        "default selection must skip parked Info findings before falling through to todos"
    );
    assert!(
        select.contains(
            "an Info finding that a `blocked` todo parks through a matching `defers:` \
             reference is not selectable"
        ),
        "the parked exception must be stated as a rule, not implied by the skip"
    );
}

#[test]
fn parked_fold_trusts_only_the_published_field() {
    let select = section("\n## Select ONE unit");
    assert!(
        select.contains("trust only the published `parked_by` field on the lint wire"),
        "the fold must key on the machine-visible wire field, not artefact prose"
    );
    assert!(
        select.contains("never a todo that merely mentions a code in prose"),
        "a prose mention must park nothing"
    );
    assert!(
        select.contains("A wire that does not publish the field parks nothing"),
        "an older binary without the field must fail closed to the previous rule"
    );
}

#[test]
fn parking_never_unselects_a_blocking_finding() {
    let select = section("\n## Select ONE unit");
    assert!(
        select.contains(
            "a `parked_by` on an `error` or `warning` is a wire defect and never unselects it"
        ),
        "parking is Info-only; this is the regression that would quietly disable the gate"
    );
}

#[test]
fn mission_paths_apply_the_same_parked_filter() {
    let mission = section("\n## Input: MISSION");
    assert!(
        mission.contains("not parked (an `info` finding publishing `parked_by`"),
        "the node-id path must not hand a MISSION a parked instance"
    );
    assert!(
        mission.contains("stands parked (`info` with published `parked_by`)"),
        "a code is settled only when every instance is deferred, parked, or folded"
    );
    assert!(
        mission.contains("parked by a blocked todo, folded by strict-green, or quarantined"),
        "a MISSION naming a parked unit must report and exhaust, not select it"
    );
}

#[test]
fn exhaustion_evidence_accounts_for_parked_findings() {
    let select = section("\n## Select ONE unit");
    assert!(
        select.contains("or parked by a `blocked` todo (published `parked_by`)"),
        "stop evidence must not demand a lint state the parked standing set makes impossible"
    );
}
