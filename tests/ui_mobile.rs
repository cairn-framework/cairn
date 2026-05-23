//! Mobile viewport tests for the graph explorer.
//!
//! Guards against regressions in phone-width usability (issue #72).

use std::fs;

fn css_content() -> String {
    fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/ui_assets/style.css"
    ))
    .unwrap()
}

#[test]
fn test_css_has_phone_width_media_query() {
    let css = css_content();
    assert!(
        css.contains("@media (max-width: 480px)") || css.contains("@media (max-width: 640px)"),
        "style.css must have a phone-width media query (max-width: 480px or 640px)"
    );
}

#[test]
fn test_css_has_minimum_tap_target_size() {
    let css = css_content();
    assert!(
        css.contains("min-height: 44px")
            || css.contains("min-width: 44px")
            || css.contains("44px") && css.contains("tap")
            || css.contains("padding:") && css.contains("44px"),
        "style.css must ensure tap targets are at least 44px for phone use"
    );
}

#[test]
fn test_inspector_has_close_affordance_on_mobile() {
    let app =
        fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/ui_assets/app.js")).unwrap();
    // The ModuleInspector must expose an onClose prop so the parent can clear selection.
    let has_close_prop = app.contains("onClose") && app.contains("ModuleInspector");
    // And the rendered inspector must contain a close button that calls it.
    let has_close_button = app.contains("ins-close") || app.contains("close-inspector");
    assert!(
        has_close_prop && has_close_button,
        "mobile inspector must have a close button wired to clear selection"
    );
}

#[test]
fn test_no_horizontal_scroll_on_phone() {
    let css = css_content();
    // The body or root should prevent horizontal overflow.
    assert!(
        css.contains("overflow-x: hidden") || css.contains("overflow-x:hidden"),
        "style.css must prevent horizontal scrolling on narrow viewports"
    );
}

#[test]
fn test_findings_panel_readable_on_narrow_screens() {
    let css = css_content();
    // Findings panel should not have a fixed wide width that breaks on phones.
    assert!(
        !css.contains("findings-panel")
            || css.contains("findings-panel")
                && (css.contains("max-width: 100%") || css.contains("width: 100%")),
        "findings panel must be full-width on narrow screens"
    );
}

#[test]
fn test_search_controls_sticky_or_accessible() {
    let css = css_content();
    let app =
        fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/ui_assets/app.js")).unwrap();
    let has_sticky = css.contains("position: sticky") || css.contains("position:sticky");
    let has_search = app.contains("search") && app.contains("input");
    assert!(
        has_sticky || has_search,
        "search controls must be accessible on mobile"
    );
}

#[test]
fn test_phone_breakpoint_collapses_inspector_to_bottom_sheet() {
    let css = css_content();
    // Inspector should become a bottom sheet or overlay at phone width.
    let has_phone_query = css.contains("max-width: 480px") || css.contains("max-width: 640px");
    let inspector_bottom = css.contains("inspector")
        && (css.contains("bottom: 0")
            || css.contains("position: fixed")
            || css.contains("position:fixed")
            || css.contains("transform: translateY")
            || css.contains("max-height: 50vh")
            || css.contains("max-height: 60vh"));
    assert!(
        has_phone_query && inspector_bottom,
        "inspector must collapse to a bottom-friendly layout at phone width"
    );
}
