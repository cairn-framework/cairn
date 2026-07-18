//! Mobile viewport tests for the instrument workspace.
//!
//! Guards against regressions in phone-width usability (issue #72). The
//! redesigned webui (todo.webui-ux-redesign) is a bounded workspace: the
//! page frame never scrolls, designated regions scroll internally, and
//! interactive controls meet the 44px touch target on tap surfaces. The
//! visual harness (`harness/eval.mjs`) verifies the rendered behaviour;
//! these tests pin the source-level markers that behaviour depends on.

use std::fs;

fn read_asset(name: &str) -> String {
    fs::read_to_string(format!(
        "{}/src/ui_assets/{name}",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap()
}

fn app_js() -> String {
    [
        "utils.js",
        "search.js",
        "status-bezel.js",
        "query-rail.js",
        "graph-workspace.js",
        "evidence-rail.js",
        "node-module.js",
        "channel-bar.js",
        "app.js",
    ]
    .iter()
    .map(|name| read_asset(name))
    .collect()
}

fn style_css() -> String {
    read_asset("style.css")
}

fn components_css() -> String {
    fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/design-system/components.css"
    ))
    .unwrap()
}

fn tokens_css() -> String {
    fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/design-system/tokens.css"
    ))
    .unwrap()
}

#[test]
fn test_css_has_narrow_width_media_query() {
    let narrow = |css: &str| css.contains("@media (max-width:");
    assert!(
        narrow(&style_css()) || narrow(&components_css()),
        "a narrow-width media query must reshape the workspace for phones"
    );
}

#[test]
fn test_tap_targets_meet_minimum_size() {
    assert!(
        tokens_css().contains("--tap-min: 44px"),
        "tokens.css must define the 44px --tap-min touch-target token"
    );
    assert!(
        components_css().contains("var(--tap-min)"),
        "components.css must apply --tap-min to interactive controls on tap surfaces"
    );
}

#[test]
fn test_page_frame_is_bounded() {
    let css = style_css();
    assert!(
        css.contains("overflow: hidden"),
        "the instrument shell must clip at the frame; only designated panels scroll"
    );
}

#[test]
fn test_selection_can_be_cleared() {
    let app = app_js();
    assert!(
        app.contains("Escape"),
        "Escape must clear the query or return the evidence rail to overview"
    );
}

#[test]
fn test_search_input_present() {
    let app = app_js();
    assert!(
        app.contains("query-input"),
        "the query rail must expose a search input"
    );
}

#[test]
fn test_evidence_rail_scrolls_internally() {
    let css = components_css();
    assert!(
        css.contains(".evidence-rail .rail-body") && css.contains("overflow: auto"),
        "the evidence rail body must be a designated internal scroll region"
    );
}
