//! Regression guard for cairn-9ey: the dogfood gate must run the working
//! tree's freshly-built cairn, never a PATH-installed (and possibly stale)
//! binary. With a stale `~/.cargo/bin/cairn`, the pre-push gate can false-green
//! by linting with an old binary that lacks the working tree's newer checks.

use std::path::Path;

#[cfg(unix)]
use std::{fs, process::Command};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(unix)]
fn executable(path: &Path, body: &str) {
    fs::write(path, body).expect("write fake command");
    let mut permissions = fs::metadata(path)
        .expect("fake command metadata")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).expect("make fake command executable");
}

#[cfg(unix)]
fn git(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .expect("run git");
    assert!(
        output.status.success(),
        "git {:?} failed: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("git output is utf8")
}

fn dogfood_script() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts/dogfood.sh");
    std::fs::read_to_string(path).expect("scripts/dogfood.sh should exist")
}

#[test]
fn dogfood_builds_and_runs_the_working_tree_binary() {
    let script = dogfood_script();
    assert!(
        script.contains("cargo run") && script.contains("--bin cairn"),
        "dogfood.sh must build and run the working-tree cairn (cargo run --bin cairn), \
         not a stale installed binary (cairn-9ey)"
    );
}

#[test]
fn dogfood_never_invokes_path_cairn() {
    // A line invoking `cairn` directly (at any indentation) resolves via PATH
    // to a possibly-stale binary. The gate must reach cairn only through the
    // cargo-built path.
    let script = dogfood_script();
    for line in script.lines() {
        assert!(
            !line.trim_start().starts_with("cairn "),
            "dogfood.sh must not invoke bare `cairn` (PATH can resolve a stale binary); found: {line:?}"
        );
    }
}

#[cfg(unix)]
#[test]
fn auto_pr_passes_the_gated_head_to_merge_guard() {
    let root = tempfile::tempdir().expect("temporary repository");
    git(root.path(), &["init", "--quiet"]);
    git(
        root.path(),
        &["config", "user.email", "cairn-test@example.com"],
    );
    git(root.path(), &["config", "user.name", "Cairn Test"]);
    fs::write(root.path().join("tracked.txt"), "gated\n").expect("tracked file");
    git(root.path(), &["add", "tracked.txt"]);
    git(root.path(), &["commit", "--quiet", "-m", "fixture"]);
    let gated_head = git(root.path(), &["rev-parse", "HEAD"]).trim().to_owned();

    let fake_bin = root.path().join("fake-bin");
    fs::create_dir(&fake_bin).expect("fake bin");
    executable(
        &fake_bin.join("gh"),
        r#"#!/bin/sh
set -eu
if [ "$1" = "pr" ] && [ "$2" = "view" ]; then
  printf '%s\n' '{"headRefName":"feature","baseRefName":"main","title":"test","author":{"login":"tester"},"url":"https://example.test/123"}'
elif [ "$1" = "pr" ] && [ "$2" = "checkout" ]; then
  exit 0
elif [ "$1" = "pr" ] && [ "$2" = "merge" ]; then
  printf '%s\n' "$*" > "$AUTO_PR_MERGE_LOG"
else
  exit 1
fi
"#,
    );
    executable(
        &fake_bin.join("jq"),
        r#"#!/bin/sh
case "$*" in
  *.headRefName*) printf '%s\n' feature ;;
  *.baseRefName*) printf '%s\n' main ;;
  *.title*) printf '%s\n' test ;;
  *.author.login*) printf '%s\n' tester ;;
  *.url*) printf '%s\n' https://example.test/123 ;;
  *) exit 1 ;;
esac
"#,
    );
    executable(&fake_bin.join("cargo"), "#!/bin/sh\nexit 0\n");
    executable(&fake_bin.join("cairn"), "#!/bin/sh\nexit 0\n");
    fs::create_dir(root.path().join("scripts")).expect("scripts directory");
    executable(
        &root.path().join("scripts/dogfood.sh"),
        "#!/bin/sh\nexit 0\n",
    );

    let log = root.path().join("merge.log");
    let path = format!(
        "{}:{}",
        fake_bin.display(),
        std::env::var_os("PATH")
            .and_then(|value| value.into_string().ok())
            .expect("PATH")
    );
    let script = Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts/auto-pr.sh");
    let result = Command::new("bash")
        .arg(script)
        .arg("123")
        .current_dir(root.path())
        .env("PATH", path)
        .env("AUTO_PR_MERGE_LOG", &log)
        .output()
        .expect("run auto-pr");
    assert!(
        result.status.success(),
        "auto-pr failed: stdout={} stderr={}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    let merge_args = fs::read_to_string(log).expect("merge command log");
    assert!(
        merge_args.contains(&format!("--match-head-commit {gated_head}")),
        "merge must pin the reviewed head, got: {merge_args}"
    );
}
