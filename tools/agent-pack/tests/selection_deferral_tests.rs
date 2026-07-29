//! Selection skips decision-deferred findings.
//!
//! `dec.loop-selection-deferred-findings`: a finding whose published
//! `deferred_by` names an accepted decision is standing evidence, not a
//! selectable loop unit, and a MISSION naming a finding code resolves per
//! instance rather than per code. The obligations live in prose, so each is
//! pinned by the sentence that carries it, in both places selection can reach
//! a finding: the default-selection paragraph under `## Select ONE unit` and
//! MISSION precedence item 2 under `## Input: MISSION`. Phrase assertions
//! match a whitespace-flattened body; rewording deliberately breaks them: that
//! is the review prompt, not a false positive.

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
fn default_selection_skips_findings_with_a_published_deferral() {
    let select = section("\n## Select ONE unit");
    assert!(
        select.contains(
            "the first `$CAIRN lint --json` finding, skipping every finding that publishes a `deferred_by`"
        ),
        "default selection must skip deferred findings before falling through to todos"
    );
    assert!(
        select.contains("A finding carrying a validated `deferred_by` is not selectable"),
        "the deferral exception must be stated as a rule, not implied by the skip"
    );
    assert!(
        select.contains(
            "An `error` or `warning` finding with no published `deferred_by` is always selectable"
        ),
        "the deferral exception must stay narrow: nothing but a published deferral may \
         unselect a blocking finding (`todo.lint-selection-folding` item 2 folds Info only)"
    );
}

#[test]
fn exhaustion_evidence_accounts_for_validated_deferrals() {
    let select = section("\n## Select ONE unit");
    assert!(
        select.contains("lint clean apart from findings with a validated deferral"),
        "stop evidence must not demand a lint state a standing deferral makes impossible"
    );
}

#[test]
fn mission_resolves_a_finding_code_per_instance_not_per_code() {
    let mission = section("\n## Input: MISSION");
    assert!(
        mission.contains("A finding code resolves per finding instance, not per code"),
        "MISSION precedence must resolve instances, or a deferred instance hides a live one"
    );
    assert!(
        mission.contains("select the first sorted instance with no published `deferred_by`"),
        "the MISSION path must take the first live instance"
    );
    assert!(
        mission.contains(
            "report settled only when every instance carrying that code has a validated deferral"
        ),
        "a code is settled only when no live instance remains"
    );
}

#[test]
fn mission_node_path_applies_the_same_deferral_filter() {
    let mission = section("\n## Input: MISSION");
    assert!(
        mission
            .contains("A node id selects its first lint finding with no published `deferred_by`"),
        "the node-id path must not hand a MISSION a deferred instance"
    );
}
