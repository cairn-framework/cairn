//! Live-repository byte fixtures: the two queued decisions must brief
//! correctly with the exact bytes a maintainer will see.

use super::tests::decision;
use super::*;

/// The two live queued decisions are the real byte fixtures the queue must
/// brief correctly; these fail loudly if either body or the parser drifts.
fn live_decision(file: &str) -> Decision {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("meta/decisions")
        .join(file);
    let body = std::fs::read_to_string(&path).expect("live decision file reads");
    decision(&body)
}

#[test]
fn live_control_plane_briefing_quotes_the_ruling_and_three_options() {
    let parsed = parse_pending_brief(
        std::path::Path::new("/repo"),
        &live_decision("control-plane-programme.md"),
        &[],
        None,
        false,
    );
    let summary = parsed.ruling_summary.expect("ruling summary");
    assert!(
        summary.starts_with("Cairn decides what work exists"),
        "summary quotes the ruling, not context: {summary}"
    );
    let rubric = parsed.rubric.expect("rubric");
    let options = rubric.options.expect("options");
    assert_eq!(options.len(), 3, "{options:?}");
    for (option, letter) in options.iter().zip(["(a)", "(b)", "(c)"]) {
        assert!(option.starts_with(letter), "{option}");
    }
}

#[test]
fn live_bootstrap_briefing_quotes_the_ruling_and_three_options() {
    let parsed = parse_pending_brief(
        std::path::Path::new("/repo"),
        &live_decision("bootstrap-fixture-corpus-split.md"),
        &[],
        None,
        false,
    );
    let summary = parsed.ruling_summary.expect("ruling summary");
    assert!(
        summary.starts_with("The bootstrap test fixture keeps two kinds of files apart"),
        "summary quotes the ruling: {summary}"
    );
    let rubric = parsed.rubric.expect("rubric");
    let tier = rubric.tier.expect("tier");
    assert!(tier.starts_with("local."), "backticks are cleaned: {tier}");
    assert!(
        tier.len() > 100,
        "tier joins its continuation lines into one item: {tier}"
    );
    let options = rubric.options.expect("options");
    assert_eq!(options.len(), 3, "{options:?}");
    for (option, letter) in options.iter().zip(["(a)", "(b)", "(c)"]) {
        assert!(option.starts_with(letter), "{option}");
    }
}
