//! Integration tests for the webui design-token conformance gate script.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn test_check_design_tokens_script_behaviour() -> Result<(), Box<dyn std::error::Error>> {
    // Clean stylesheet: every colour and size comes from a token var. Passes.
    let clean = write_css(
        "clean.css",
        "body { color: var(--ink-char); margin: var(--space-2); width: 100%; }\n",
    )?;
    assert!(run_script(&clean).status.success());

    // Hardcoded hex colour: fails, naming the offending file and line.
    let hex = write_css(
        "hex.css",
        "body { color: var(--ink); }\na { color: #ff0000; }\n",
    )?;
    let hex_run = run_script(&hex);
    assert!(!hex_run.status.success());
    let hex_err = String::from_utf8(hex_run.stderr)?;
    assert!(
        hex_err.contains("hex"),
        "expected hex diagnostic, got: {hex_err}"
    );
    assert!(hex_err.contains("#ff0000"));

    // Hardcoded rem value: fails.
    let rem = write_css("rem.css", "a { margin: 1.5rem; }\n")?;
    let rem_run = run_script(&rem);
    assert!(!rem_run.status.success());
    let rem_err = String::from_utf8(rem_run.stderr)?;
    assert!(
        rem_err.contains("rem"),
        "expected rem diagnostic, got: {rem_err}"
    );

    // Hex/rem mentioned only inside a CSS comment is exempt: comments are
    // stripped before scanning, so prose referencing old values still passes.
    let comment = write_css(
        "comment.css",
        "/* migrated from #ff0000 and 1.5rem to tokens */\na { color: var(--accent); }\n",
    )?;
    assert!(
        run_script(&comment).status.success(),
        "hex/rem inside a comment must not trip the gate"
    );

    // Leading-decimal rem (.5rem) is the same hardcoded size and must fail.
    let rem_dot = write_css("rem_dot.css", "a { gap: .5rem; }\n")?;
    assert!(
        !run_script(&rem_dot).status.success(),
        ".5rem must be caught"
    );

    // A hardcoded colour inside any colour function must still fail: guards
    // against over-correcting the url() exemption below.
    let gradient = write_css(
        "gradient.css",
        "a { background: linear-gradient(#fff, #000); }\n",
    )?;
    assert!(
        !run_script(&gradient).status.success(),
        "hardcoded hex inside linear-gradient must be caught"
    );

    // False-positive guards: an SVG `url(#frag)` reference and an id selector
    // with a non-hex name are not hardcoded colours and must pass.
    let url_ref = write_css(
        "url_ref.css",
        "a { fill: url(#accent); clip-path: url(#abc); }\n",
    )?;
    assert!(
        run_script(&url_ref).status.success(),
        "url(#frag) references must not be flagged as hex colours"
    );
    let id_sel = write_css("id_sel.css", "#accent-gradient { color: var(--ink); }\n")?;
    assert!(
        run_script(&id_sel).status.success(),
        "an id selector with a non-hex name must not be flagged"
    );

    // Identifiers that merely contain the text `rem` (a class like `.m1rem` or
    // a custom property like `--gap-1rem`) are not hardcoded rem values and
    // must pass: the rem matcher is bounded on both sides.
    let rem_ident = write_css(
        "rem_ident.css",
        ".m1rem { color: var(--ink); }\na { --gap-1rem: var(--space-1); }\n",
    )?;
    assert!(
        run_script(&rem_ident).status.success(),
        "identifiers containing the text rem must not be flagged"
    );

    // A negative rem literal is still a hardcoded size and must fail.
    let rem_neg = write_css("rem_neg.css", "a { margin-top: -1.5rem; }\n")?;
    assert!(
        !run_script(&rem_neg).status.success(),
        "a negative rem literal must be caught"
    );

    Ok(())
}

fn run_script(target: &Path) -> std::process::Output {
    Command::new("sh")
        .arg("scripts/check-design-tokens.sh")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("CAIRN_DESIGN_TOKENS_TARGET", target)
        .output()
        .expect("script should execute")
}

fn write_css(name: &str, body: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let path = temp_root()?.join(name);
    fs::write(&path, body)?;
    Ok(path)
}

fn temp_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("cairn-design-tokens-{suffix}"));
    fs::create_dir_all(&root)?;
    Ok(root)
}
