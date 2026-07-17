//! Integration tests for the file-size gate script.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn test_check_file_sizes_script_behaviour() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root()?;
    let src = root.join("src");
    fs::create_dir_all(&src)?;

    write_lines(&src.join("exact.rs"), 500, None)?;
    assert!(run_script(&root).status.success());

    write_lines(&src.join("too_big.rs"), 501, None)?;
    let failed = run_script(&root);
    assert!(!failed.status.success());
    let stderr = String::from_utf8(failed.stderr)?;
    assert!(stderr.contains("too_big.rs"));
    assert!(stderr.contains("501"));

    write_lines(
        &src.join("allowed.rs"),
        501,
        Some("// cairn:allow-large-module reason: scheduled-for-phase-7.5b-split"),
    )?;
    fs::remove_file(src.join("too_big.rs"))?;
    assert!(run_script(&root).status.success());

    for empty_reason in [
        "// cairn:allow-large-module reason:",
        "// cairn:allow-large-module reason: ",
    ] {
        write_lines(&src.join("missing_reason.rs"), 501, Some(empty_reason))?;
        let missing_reason = run_script(&root);
        assert!(
            !missing_reason.status.success(),
            "expected failure for {empty_reason:?}"
        );
        let stderr = String::from_utf8(missing_reason.stderr)?;
        assert!(stderr.contains("missing_reason.rs"));
        assert!(
            stderr.contains("missing non-empty allow-list reason"),
            "expected missing-reason diagnostic for {empty_reason:?}, got: {stderr}",
        );
    }

    Ok(())
}

#[test]
fn test_claimed_non_rust_paths_are_gated() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root()?;
    let claimed = root.join("web/assets");
    let unclaimed = root.join("unclaimed");
    fs::create_dir_all(&claimed)?;
    fs::create_dir_all(&unclaimed)?;
    fs::write(
        root.join("cairn.blueprint"),
        "System Fixture \"F\" id \"fx\" {\n    Module Web \"Web\" id \"fx.web\" {\n        path [\"./web/assets\", \"./web/other\"]\n    }\n}\n",
    )?;

    // A fresh oversized file under a claimed directory fails, even though no
    // snapshot (map.json) has ever enumerated it, and even via list-form path.
    write_css(&claimed.join("big.css"), 501, None)?;
    let failed = run_script(&root);
    assert!(!failed.status.success());
    let stderr = String::from_utf8(failed.stderr)?;
    assert!(stderr.contains("big.css"));

    // The CSS allow marker suppresses it.
    write_css(
        &claimed.join("big.css"),
        501,
        Some("/* cairn:allow-large-module reason: fixture */"),
    )?;
    assert!(run_script(&root).status.success());

    // A claimed oversized JS file fails, and the JS marker suppresses it.
    write_js(&claimed.join("big.js"), 501, None)?;
    let js_failed = run_script(&root);
    assert!(!js_failed.status.success());
    assert!(String::from_utf8(js_failed.stderr)?.contains("big.js"));
    write_js(
        &claimed.join("big.js"),
        501,
        Some("// cairn:allow-large-module reason: fixture"),
    )?;
    assert!(run_script(&root).status.success());

    // An oversized file in an unclaimed directory is not gated.
    write_js(&unclaimed.join("huge.js"), 800, None)?;
    assert!(run_script(&root).status.success());

    // Vendored assets under a claim are excluded.
    let vendor = claimed.join("vendor");
    fs::create_dir_all(&vendor)?;
    write_css(&vendor.join("lib.js"), 900, None)?;
    assert!(run_script(&root).status.success());

    // Gitignored files under a claim are excluded once the root is a repo.
    let ignored = claimed.join("generated.css");
    write_css(&ignored, 700, None)?;
    assert!(!run_script(&root).status.success());
    let git = |args: &[&str]| {
        Command::new("git")
            .args(args)
            .current_dir(&root)
            .output()
            .expect("git should execute")
    };
    assert!(git(&["init", "-q", "."]).status.success());
    fs::write(root.join(".gitignore"), "web/assets/generated.css\n")?;
    assert!(run_script(&root).status.success());

    Ok(())
}

#[test]
fn test_blueprint_path_declaration_parsing() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root()?;
    let claimed = root.join("web/assets");
    fs::create_dir_all(&claimed)?;
    let other = root.join("web/other");
    fs::create_dir_all(&other)?;

    // A multi-line path list is parsed and its claims are gated.
    fs::write(
        root.join("cairn.blueprint"),
        "System Fixture \"F\" id \"fx\" {\n    Module Web \"Web\" id \"fx.web\" {\n        path [\"./web/assets\",\n              \"./web/other\"]\n    }\n}\n",
    )?;
    let other = root.join("web/other");
    fs::create_dir_all(&other)?;
    write_css(&other.join("multi.css"), 600, None)?;
    let multi = run_script(&root);
    assert!(!multi.status.success());
    assert!(String::from_utf8(multi.stderr)?.contains("multi.css"));
    fs::remove_file(other.join("multi.css"))?;

    // A directly claimed asset FILE is gated, not only directories.
    fs::write(
        root.join("cairn.blueprint"),
        "System Fixture \"F\" id \"fx\" {\n    Module Web \"Web\" id \"fx.web\" {\n        path \"./web/direct.js\"\n    }\n}\n",
    )?;
    write_js(&root.join("web/direct.js"), 501, None)?;
    let direct = run_script(&root);
    assert!(!direct.status.success());
    assert!(String::from_utf8(direct.stderr)?.contains("direct.js"));
    fs::remove_file(root.join("web/direct.js"))?;

    // A directly claimed file under a vendor path is excluded like the walk.
    fs::write(
        root.join("cairn.blueprint"),
        "System Fixture \"F\" id \"fx\" {\n    Module Web \"Web\" id \"fx.web\" {\n        path \"./web/vendor/direct.js\"\n    }\n}\n",
    )?;
    let vendor_direct = root.join("web/vendor");
    fs::create_dir_all(&vendor_direct)?;
    write_js(&vendor_direct.join("direct.js"), 900, None)?;
    assert!(run_script(&root).status.success());

    // Bracket characters inside a quoted scalar are not list syntax.
    fs::write(
        root.join("cairn.blueprint"),
        "System Fixture \"F\" id \"fx\" {\n    Module Web \"Web\" id \"fx.web\" {\n        path \"./web/[draft\"\n    }\n}\n",
    )?;
    assert!(run_script(&root).status.success());

    // A bare path keyword with its value elsewhere fails closed.
    fs::write(
        root.join("cairn.blueprint"),
        "System Fixture \"F\" id \"fx\" {\n    Module Web \"Web\" id \"fx.web\" {\n        path\n    }\n}\n",
    )?;
    let bare = run_script(&root);
    assert!(!bare.status.success());
    assert!(String::from_utf8(bare.stderr)?.contains("unparsable path declaration"));

    // A comment containing ] inside a multi-line list does not fake closure.
    fs::write(
        root.join("cairn.blueprint"),
        "System Fixture \"F\" id \"fx\" {\n    Module Web \"Web\" id \"fx.web\" {\n        path [\"./web/assets\", # ] pending cleanup\n              \"./web/other\"]\n    }\n}\n",
    )?;
    write_css(&other.join("commented.css"), 600, None)?;
    let commented = run_script(&root);
    assert!(!commented.status.success());
    assert!(String::from_utf8(commented.stderr)?.contains("commented.css"));
    fs::remove_file(other.join("commented.css"))?;

    // Escaped characters in a quoted path value fail closed.
    fs::write(
        root.join("cairn.blueprint"),
        "System Fixture \"F\" id \"fx\" {\n    Module Web \"Web\" id \"fx.web\" {\n        path \"./web/quo\\\\\"ted\"\n    }\n}\n",
    )?;
    let escaped = run_script(&root);
    assert!(!escaped.status.success());
    assert!(String::from_utf8(escaped.stderr)?.contains("escaped characters"));

    // An unterminated list fails closed.
    fs::write(
        root.join("cairn.blueprint"),
        "System Fixture \"F\" id \"fx\" {\n    Module Web \"Web\" id \"fx.web\" {\n        path [\"./web/assets\",\n    }\n}\n",
    )?;
    let unterminated = run_script(&root);
    assert!(!unterminated.status.success());
    assert!(String::from_utf8(unterminated.stderr)?.contains("unterminated path list"));

    Ok(())
}

fn write_js(
    path: &Path,
    count: usize,
    first_line: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = Vec::with_capacity(count);
    if let Some(first_line) = first_line {
        lines.push(first_line.to_owned());
    }
    while lines.len() < count {
        lines.push(format!("let l{} = 0;", lines.len()));
    }
    fs::write(path, format!("{}\n", lines.join("\n")))?;
    Ok(())
}

fn write_css(
    path: &Path,
    count: usize,
    first_line: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = Vec::with_capacity(count);
    if let Some(first_line) = first_line {
        lines.push(first_line.to_owned());
    }
    while lines.len() < count {
        lines.push(format!(".c{} {{ margin: 0; }}", lines.len()));
    }
    fs::write(path, format!("{}\n", lines.join("\n")))?;
    Ok(())
}

fn run_script(root: &Path) -> std::process::Output {
    Command::new("sh")
        .arg("scripts/check-file-sizes.sh")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("CAIRN_FILE_SIZE_ROOT", root)
        .output()
        .expect("script should execute")
}

fn write_lines(
    path: &Path,
    count: usize,
    first_line: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = Vec::with_capacity(count);
    if let Some(first_line) = first_line {
        lines.push(first_line.to_owned());
    }
    while lines.len() < count {
        lines.push(format!("fn line_{}() {{}}", lines.len()));
    }
    fs::write(path, format!("{}\n", lines.join("\n")))?;
    Ok(())
}

fn temp_root() -> Result<TempRoot, Box<dyn std::error::Error>> {
    let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("cairn-file-sizes-{suffix}"));
    fs::create_dir_all(&root)?;
    Ok(TempRoot(root))
}

/// Fixture directory removed on drop so runs leave no debris behind.
struct TempRoot(PathBuf);

impl Drop for TempRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

impl AsRef<Path> for TempRoot {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl std::ops::Deref for TempRoot {
    type Target = Path;

    fn deref(&self) -> &Path {
        &self.0
    }
}
