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
fn install_fake_gh(fake_bin: &Path) {
    executable(
        &fake_bin.join("gh"),
        r#"#!/bin/sh
set -eu
if [ "$1" = "pr" ] && [ "$2" = "view" ]; then
  printf '%s\n' '{"headRefName":"feature","baseRefName":"main","title":"test","author":{"login":"tester"},"url":"https://example.test/123"}'
elif [ "$1" = "pr" ] && [ "$2" = "checkout" ]; then
  if [ "${AUTO_PR_CHECKOUT_CONFLICT:-}" = "1" ]; then
    printf '%s\n' remote >> tracked.txt
    git add tracked.txt
    git commit --quiet -m checkout
  fi
  exit 0
elif [ "$1" = "pr" ] && [ "$2" = "merge" ]; then
  printf '%s\n' "$*" > "$AUTO_PR_MERGE_LOG"
  if [ "${AUTO_PR_MERGE_FAIL:-}" = "1" ]; then exit 1; fi
else
  exit 1
fi
"#,
    );
}

#[cfg(unix)]
fn install_fake_jq(fake_bin: &Path) {
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
}

#[cfg(unix)]
fn install_fake_cargo(fake_bin: &Path) {
    executable(
        &fake_bin.join("cargo"),
        r#"#!/bin/sh
set -eu
if [ "$1" = "test" ] && [ "${AUTO_PR_MUTATE_HEAD:-}" = "1" ]; then
  printf '%s\n' mutated >> tracked.txt
  git add tracked.txt
  git commit --quiet -m mutate
fi
"#,
    );
}

#[cfg(unix)]
fn install_fake_commands(fake_bin: &Path) {
    fs::create_dir(fake_bin).expect("fake bin");
    install_fake_gh(fake_bin);
    install_fake_jq(fake_bin);
    install_fake_cargo(fake_bin);
    executable(&fake_bin.join("cairn"), "#!/bin/sh\nexit 0\n");
}

#[cfg(unix)]
fn install_fake_dogfood(root: &Path) {
    let scripts = root.join("scripts");
    fs::create_dir(&scripts).expect("scripts directory");
    executable(&scripts.join("dogfood.sh"), "#!/bin/sh\nexit 0\n");
}

#[cfg(unix)]
struct AutoPrFixture {
    root: tempfile::TempDir,
    gated_head: String,
    merge_log: std::path::PathBuf,
}

#[cfg(unix)]
fn auto_pr_fixture() -> AutoPrFixture {
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
    install_fake_commands(&root.path().join("fake-bin"));
    install_fake_dogfood(root.path());
    AutoPrFixture {
        merge_log: root.path().join("merge.log"),
        root,
        gated_head,
    }
}

#[cfg(unix)]
fn run_auto_pr(fixture: &AutoPrFixture, env: &[(&str, &str)]) -> std::process::Output {
    let fake_bin = fixture.root.path().join("fake-bin");
    let path = format!(
        "{}:{}",
        fake_bin.display(),
        std::env::var_os("PATH")
            .and_then(|value| value.into_string().ok())
            .expect("PATH")
    );
    let script = Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts/auto-pr.sh");
    let mut command = Command::new("bash");
    command
        .arg(script)
        .arg("123")
        .current_dir(fixture.root.path())
        .env("PATH", path)
        .env("AUTO_PR_MERGE_LOG", &fixture.merge_log);
    for (key, value) in env {
        command.env(*key, *value);
    }
    command.output().expect("run auto-pr")
}

#[cfg(unix)]
#[test]
fn auto_pr_passes_the_gated_head_to_merge_guard() {
    let fixture = auto_pr_fixture();
    let result = run_auto_pr(&fixture, &[]);
    assert!(
        result.status.success(),
        "auto-pr failed: stdout={} stderr={}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    let merge_args = fs::read_to_string(&fixture.merge_log).expect("merge command log");
    assert!(
        merge_args.contains(&format!("--match-head-commit {}", fixture.gated_head)),
        "merge must pin the reviewed head, got: {merge_args}"
    );
}

#[cfg(unix)]
#[test]
fn auto_pr_refuses_to_merge_after_a_gate_moves_head() {
    let fixture = auto_pr_fixture();
    let result = run_auto_pr(&fixture, &[("AUTO_PR_MUTATE_HEAD", "1")]);
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(!result.status.success(), "head drift must fail the merge");
    assert!(
        stdout.contains("checked-out head changed after gates"),
        "head drift should be reported: {stdout}"
    );
    assert!(
        !fixture.merge_log.exists(),
        "head drift must skip gh pr merge"
    );
}

#[cfg(unix)]
#[test]
fn auto_pr_restores_stash_when_match_head_merge_fails() {
    let fixture = auto_pr_fixture();
    fs::write(fixture.root.path().join("tracked.txt"), "gated\nlocal\n")
        .expect("unstaged local edit");
    let result = run_auto_pr(&fixture, &[("AUTO_PR_MERGE_FAIL", "1")]);
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(
        !result.status.success(),
        "remote mismatch must fail the merge"
    );
    assert!(
        stdout.contains("merge failed; the remote head may have moved"),
        "merge failure should be reported: {stdout}"
    );
    assert_eq!(
        fs::read_to_string(fixture.root.path().join("tracked.txt")).expect("restored edit"),
        "gated\nlocal\n"
    );
    assert!(
        git(fixture.root.path(), &["stash", "list"])
            .trim()
            .is_empty(),
        "the temporary stash must be restored"
    );
}

#[cfg(unix)]
#[test]
fn auto_pr_reports_stash_restore_conflict() {
    let fixture = auto_pr_fixture();
    fs::write(fixture.root.path().join("tracked.txt"), "gated\nlocal\n")
        .expect("unstaged local edit");
    let result = run_auto_pr(&fixture, &[("AUTO_PR_CHECKOUT_CONFLICT", "1")]);
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(
        !result.status.success(),
        "stash conflict must produce a nonzero result"
    );
    assert!(
        stdout.contains("Warning: stash pop had conflicts"),
        "stash conflict should be reported: {stdout}"
    );
    assert!(fixture.merge_log.exists(), "the merge command should run");
    assert!(
        git(fixture.root.path(), &["status", "--short"]).contains("UU tracked.txt"),
        "the conflict must remain visible"
    );
    assert!(
        !git(fixture.root.path(), &["stash", "list"])
            .trim()
            .is_empty(),
        "a conflicted stash must remain available"
    );
}
