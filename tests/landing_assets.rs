//! Repo-integrity test: the landing page must reference assets that exist.
//!
//! Regression guard for cairn-s2t. The hero `<img>`, `og:image`, and
//! `twitter:image` pointed at `docs/images/webui-v2-empty.png` (never committed)
//! via the retired `dev` branch, so the hero rendered as broken alt-text and the
//! social cards 404'd. These tests fail if any local asset reference in the
//! landing page does not resolve on disk, or if a social-card image still points
//! at the missing asset or the retired branch.

use std::path::{Path, PathBuf};

const LANDING: &str = "docs/landing/index.html";

/// GitHub Pages serves `docs/` at this origin (see the page's `og:url` and
/// `.github/workflows/pages.yml`). Social-card images must ride the same origin
/// so they resolve to a committed file under `docs/`.
const PAGES_ORIGIN: &str = "https://cairn-framework.github.io/cairn/";

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn landing_html() -> String {
    let path = repo_root().join(LANDING);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("landing page {} should be readable: {e}", path.display()))
}

/// Directory the landing page's relative references resolve against.
fn landing_dir() -> PathBuf {
    repo_root().join(LANDING).parent().unwrap().to_path_buf()
}

/// Collect every double-quoted value of `attr` (e.g. `src`, `href`) in `html`.
fn attr_values(html: &str, attr: &str) -> Vec<String> {
    let needle = format!("{attr}=\"");
    let mut out = Vec::new();
    let mut rest = html;
    while let Some(i) = rest.find(&needle) {
        let after = &rest[i + needle.len()..];
        match after.find('"') {
            Some(end) => {
                out.push(after[..end].to_string());
                rest = &after[end + 1..];
            }
            None => break,
        }
    }
    out
}

/// The `content="..."` of the meta tag identified by `marker`
/// (e.g. `property="og:image"`).
fn meta_content(html: &str, marker: &str) -> String {
    let start = html
        .find(marker)
        .unwrap_or_else(|| panic!("landing page is missing a `{marker}` meta tag"));
    let tag_end = html[start..].find('>').map_or(html.len(), |i| start + i);
    let tag = &html[start..tag_end];
    let value_start = tag
        .find("content=\"")
        .unwrap_or_else(|| panic!("`{marker}` meta tag has no content attribute"));
    let rest = &tag[value_start + "content=\"".len()..];
    let end = rest
        .find('"')
        .expect("`content` attribute should be terminated");
    rest[..end].to_string()
}

/// A reference that points at a file in this repo rather than an external host,
/// in-page anchor, or inline data URI.
fn is_local_file_ref(value: &str) -> bool {
    !value.is_empty()
        && !["http://", "https://", "//", "data:", "#", "mailto:", "tel:"]
            .iter()
            .any(|p| value.starts_with(p))
}

/// Every relative `src`/`href` on the landing page resolves to a real file.
#[test]
fn local_asset_references_resolve() {
    let html = landing_html();
    let dir = landing_dir();

    let mut refs: Vec<String> = attr_values(&html, "src");
    refs.extend(attr_values(&html, "href"));

    let local: Vec<&String> = refs.iter().filter(|v| is_local_file_ref(v)).collect();
    assert!(
        !local.is_empty(),
        "expected at least one local asset reference on the landing page"
    );

    for value in local {
        let resolved = dir.join(value);
        assert!(
            resolved.exists(),
            "landing page references `{value}`, but {} does not exist",
            resolved.display()
        );
    }
}

/// The social-card images exist on the Pages origin and no longer point at the
/// missing asset or the retired `dev` branch.
#[test]
fn social_card_images_exist_and_are_not_stale() {
    let html = landing_html();

    for marker in ["property=\"og:image\"", "name=\"twitter:image\""] {
        let url = meta_content(&html, marker);

        assert!(
            !url.contains("webui-v2-empty"),
            "{marker} still references the missing asset: {url}"
        );
        assert!(
            !url.contains("/dev/"),
            "{marker} still references the retired `dev` branch: {url}"
        );
        let rel = url.strip_prefix(PAGES_ORIGIN).unwrap_or_else(|| {
            panic!("{marker} should be served from the Pages origin {PAGES_ORIGIN}, got: {url}")
        });
        let resolved = repo_root().join("docs").join(rel);
        assert!(
            resolved.exists(),
            "{marker} points at `{url}`, but {} does not exist",
            resolved.display()
        );
    }
}
