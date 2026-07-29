//! Selection folds Info findings while the lint wire is strict-green.
//!
//! `todo.lint-selection-folding` item 2, ratified 2026-07-29 (PR #528 sheet
//! W2): `cairn scan --strict` is the CI gate, so anything it tolerates is by
//! definition not iteration-blocking. The scanner half publishes
//! `strict_green` on the lint/scan JSON envelope; this file pins the asset
//! half, sentence by sentence, in every place selection or the gate can meet
//! an Info finding: default selection, both MISSION paths, the stop evidence,
//! the Verify blocking bar, the guardrail, and the landing re-verification.
//! Phrase assertions match a
//! whitespace-flattened body; rewording deliberately breaks them: that is the
//! review prompt, not a false positive.

use std::path::{Path, PathBuf};

fn pack_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn loop_mode() -> String {
    std::fs::read_to_string(pack_dir().join("content/skills/cairn-dev/references/loop-mode.md"))
        .unwrap()
}

fn landing() -> String {
    std::fs::read_to_string(pack_dir().join("content/skills/cairn-loop-landing/SKILL.md")).unwrap()
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
fn default_selection_folds_info_findings_under_strict_green() {
    let select = section("\n## Select ONE unit");
    assert!(
        select.contains(
            "skipping every finding that publishes a `deferred_by` and, while that wire \
             publishes `\"strict_green\": true`, every `info` finding"
        ),
        "default selection must fold Info findings whenever the wire is strict-green"
    );
    assert!(
        select.contains(
            "every `info` finding is non-selecting while the same wire publishes \
             `\"strict_green\": true`"
        ),
        "the fold must be stated as a rule, not implied by the skip"
    );
}

#[test]
fn strict_green_fold_trusts_only_the_published_field() {
    let select = section("\n## Select ONE unit");
    assert!(
        select.contains(
            "Trust only the published `strict_green` field, never a verdict you recompute \
             yourself"
        ),
        "the fold must key on the machine-visible wire field, not a session's own arithmetic"
    );
    assert!(
        select.contains("a wire that does not publish the field folds nothing"),
        "an older binary without the field must fail closed to the old rule"
    );
}

#[test]
fn blocking_findings_stay_selectable_whatever_the_verdict() {
    let select = section("\n## Select ONE unit");
    assert!(
        select.contains(
            "An `error` or `warning` finding with no published `deferred_by` is always \
             selectable"
        ),
        "the fold applies to Info alone; this is the regression that would quietly \
         disable the gate"
    );
}

#[test]
fn exhaustion_evidence_accounts_for_folded_info_findings() {
    let select = section("\n## Select ONE unit");
    assert!(
        select.contains(
            "lint clean apart from findings with a validated deferral or Info findings \
             folded by a published strict-green verdict"
        ),
        "stop evidence must not demand a lint state the standing Info set makes impossible"
    );
}

#[test]
fn mission_paths_apply_the_same_strict_green_fold() {
    let mission = section("\n## Input: MISSION");
    assert!(
        mission.contains(
            "A node id selects its first lint finding with no published `deferred_by` and \
             no strict-green fold"
        ),
        "the node-id path must not hand a MISSION a folded instance"
    );
    assert!(
        mission.contains(
            "select the first sorted instance with no published `deferred_by` and no \
             strict-green fold"
        ),
        "the finding-code path must take the first live, unfolded instance"
    );
    assert!(
        mission.contains(
            "has a validated deferral or stands folded by a published strict-green verdict"
        ),
        "a code is settled when every instance is deferred or folded"
    );
    assert!(
        mission.contains("folded by strict-green, or quarantined: report why"),
        "a MISSION naming a folded unit must report and exhaust, not select it"
    );
}

#[test]
fn verify_gate_blocks_on_strict_not_on_standing_info() {
    let verify = section("\n## Verify: the gate");
    assert!(
        verify.contains("Always `$CAIRN scan --strict` (the full report"),
        "Verify must instruct running strict itself, not deriving its verdict"
    );
    assert!(
        verify.contains("its exit 0 is the blocking bar"),
        "Verify must name the strict exit as the blocking authority"
    );
    assert!(
        verify.contains("a standing Info the strict gate tolerates does not"),
        "a selector that reaches the todo backlog past standing Info must also land past it"
    );
    assert!(
        verify.contains("zero findings is the target"),
        "strict as the bar must not surrender the zero-findings target"
    );
}

#[test]
fn guardrail_scopes_the_blocking_finding_to_strict() {
    let guardrails = section("\n## Guardrails");
    assert!(
        guardrails.contains("a finding that `$CAIRN scan --strict` rejects blocks the iteration"),
        "the guardrail must agree with Verify on what blocks"
    );
}

#[test]
fn landing_reverification_exercises_the_strict_bar() {
    let body = flatten(&landing());
    assert!(
        body.contains("\"$CAIRN\" scan --strict"),
        "the cleanup script must exercise the same blocking bar Verify names"
    );
    assert!(
        body.contains("re-run `$CAIRN scan --strict` and `$CAIRN hook all`"),
        "the change-apply path must re-verify against the strict bar"
    );
}
