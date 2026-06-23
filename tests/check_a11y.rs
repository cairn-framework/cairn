//! Integration tests for the webui accessibility static-audit gate script.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn test_check_a11y_script_behaviour() -> Result<(), Box<dyn std::error::Error>> {
    let vp = "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">";
    // Wrap a body fragment in a minimal, otherwise-conformant HTML document.
    let doc = |body: &str| {
        format!("<html lang=\"en\"><head><title>X</title>{vp}</head><body>{body}</body></html>\n")
    };

    // (label, source, should_pass, required stderr needle when it must fail).
    let cases: Vec<(&str, String, bool, &str)> = vec![
        // A conformant document passes every check.
        ("clean", doc("<img src=\"a.png\" alt=\"a stone cairn\">"), true, ""),
        // WCAG 1.1.1: an <img> with no alt fails.
        ("img-no-alt", doc("<img src=\"a.png\">"), false, "alt"),
        // A multi-line <img> carrying alt across lines still passes (tag-aware).
        ("img-multiline-alt", doc("<img class=\"shot\"\n  src=\"a.png\"\n  alt=\"a graph\">"), true, ""),
        // Empty alt marks a decorative image (WCAG 1.1.1): the attribute is
        // present, so it passes.
        ("img-empty-alt", doc("<img src=\"rule.png\" alt=\"\">"), true, ""),
        // WCAG 2.4.3: a positive tabindex fails; 0 and -1 are legitimate.
        ("tabindex-positive", doc("<div tabindex=\"2\">x</div>"), false, "tabindex"),
        ("tabindex-ok", doc("<div tabindex=\"0\">a</div><div tabindex=\"-1\">b</div>"), true, ""),
        // A custom data-tabindex attribute is not the tabindex property: passes.
        ("tabindex-data-attr-ok", doc("<div data-tabindex=\"2\">x</div>"), true, ""),
        // WCAG 3.1.1 / 2.4.2: a document needs <html lang> and a <title>.
        ("no-lang", format!("<html><head><title>X</title>{vp}</head><body></body></html>\n"), false, "lang"),
        ("no-title", format!("<html lang=\"en\"><head>{vp}</head><body></body></html>\n"), false, "title"),
        // WCAG 1.4.4: a viewport that disables pinch zoom fails.
        ("no-zoom-user-scalable", "<html lang=\"en\"><head><title>X</title><meta name=\"viewport\" content=\"width=device-width, user-scalable=no\"></head><body></body></html>\n".to_string(), false, "zoom"),
        ("no-zoom-max-scale", "<html lang=\"en\"><head><title>X</title><meta name=\"viewport\" content=\"width=device-width, maximum-scale=1\"></head><body></body></html>\n".to_string(), false, "zoom"),
        // maximum-scale above 1 still allows zoom and must pass (only =1 fails).
        ("zoom-ok-max-scale-1-5", "<html lang=\"en\"><head><title>X</title><meta name=\"viewport\" content=\"width=device-width, maximum-scale=1.5\"></head><body></body></html>\n".to_string(), true, ""),
        // Violations mentioned only inside a comment are prose, not markup.
        ("comment-exempt", doc("\n<!-- old markup used <img src=x> and tabindex=\"3\" -->\n"), true, ""),
        // A JS/htm fragment (no <html> root) skips document-level checks but is
        // still held to the element-level ones.
        ("fragment-img-alt", "const t = html`<div><img src=${url} alt=${label} /></div>`;\n".to_string(), true, ""),
        ("fragment-img-no-alt", "const t = html`<div><img src=${url} /></div>`;\n".to_string(), false, "alt"),
        // A fragment that merely contains the substring <html (not a root tag)
        // is not a document, so the lang/title checks do not apply: passes.
        ("fragment-html-substring", "const note = \"render <htmlblock> later\";\n".to_string(), true, ""),
    ];

    for (name, body, should_pass, needle) in &cases {
        let path = write_fixture(&format!("{name}.html"), body)?;
        let out = run_script(&path);
        let err = String::from_utf8_lossy(&out.stderr);
        if *should_pass {
            assert!(
                out.status.success(),
                "case {name} should pass; stderr:\n{err}"
            );
        } else {
            assert!(!out.status.success(), "case {name} should fail");
            assert!(
                err.contains(*needle),
                "case {name} expected '{needle}' in stderr, got:\n{err}"
            );
        }
    }

    Ok(())
}

/// The real web surfaces must stay accessibility-conformant: the webui shell,
/// the landing page, and the preact app all pass the default gate.
#[test]
fn test_real_surfaces_are_a11y_conformant() {
    for target in [
        "src/ui_assets/index.html",
        "docs/landing/index.html",
        "src/ui_assets/app.js",
    ] {
        let out = run_script(Path::new(target));
        assert!(
            out.status.success(),
            "{target} should pass the a11y gate; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
}

fn run_script(target: &Path) -> std::process::Output {
    Command::new("sh")
        .arg("scripts/check-a11y.sh")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("CAIRN_A11Y_TARGET", target)
        .output()
        .expect("script should execute")
}

fn write_fixture(name: &str, body: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let path = temp_root()?.join(name);
    fs::write(&path, body)?;
    Ok(path)
}

fn temp_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("cairn-a11y-{suffix}"));
    fs::create_dir_all(&root)?;
    Ok(root)
}
