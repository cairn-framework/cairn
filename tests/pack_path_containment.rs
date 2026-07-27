//! `cairn pack` path-containment acceptance tests.
//!
//! The pack reads, writes, and deletes inside a user's repository, so every
//! lifecycle verb must act only on a project-relative, regular file reached
//! without traversing a symlink. `write_asset` guarded its writes from the
//! start; reads, ledger publication, and removal did not, and each of the
//! cases below deleted or wrote outside the project root before this suite
//! existed.

use std::fs;
use std::path::{Path, PathBuf};

const MANIFEST: &str = ".cairn/state/agent-pack.json";
// Only the symlink and FIFO cases reference an owned asset path, and both are
// Unix-only fixtures.
#[cfg(unix)]
const ROUTER: &str = ".claude/skills/cairn-dev/SKILL.md";

fn temp_root(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("cairn-contain-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    fs::write(
        root.join("cairn.blueprint"),
        "System App \"T\" id \"t\" {}\n",
    )
    .unwrap();
    root
}

/// A directory outside the project, standing in for anything a symlink could
/// redirect a verb onto.
fn outside(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cairn-outside-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn pack(root: &Path, tokens: &[&str]) -> cairn::cli::CliResult {
    let mut argv = vec![
        "--file".to_owned(),
        root.join("cairn.blueprint").to_string_lossy().to_string(),
        "pack".to_owned(),
    ];
    argv.extend(tokens.iter().map(|token| (*token).to_owned()));
    cairn::cli::run(&argv)
}

#[cfg(unix)]
#[test]
fn an_owned_file_replaced_by_a_symlink_is_never_reported_pristine() {
    let root = temp_root("adopt-symlink");
    let away = outside("adopt-symlink");
    assert_eq!(pack(&root, &["install"]).code, 0);

    // Same bytes, but the destination is now a symlink pointing out of the
    // project. Content equality must not make it owned: a later refresh or
    // uninstall would then act on the target instead.
    let target = away.join("router.md");
    fs::copy(root.join(ROUTER), &target).unwrap();
    fs::remove_file(root.join(ROUTER)).unwrap();
    std::os::unix::fs::symlink(&target, root.join(ROUTER)).unwrap();
    // Assert the classification, not the bundled asset count: an unrelated
    // asset added to the pack must not break this security regression.
    let status: serde_json::Value =
        serde_json::from_str(&pack(&root, &["status", "--json"]).stdout).unwrap();
    let modified = status["data"]["modified"].as_array().unwrap();
    assert!(
        modified.iter().any(|path| path == ROUTER),
        "a symlinked destination must be reported as modified: {status}"
    );

    assert_eq!(pack(&root, &["uninstall"]).code, 0);
    assert!(
        target.exists(),
        "uninstall must not delete the file a symlinked destination points at"
    );
}

#[cfg(unix)]
#[test]
fn the_ledger_is_never_published_through_a_symlinked_parent() {
    let root = temp_root("ledger-escape");
    let away = outside("ledger-escape");

    // `.cairn/state` redirects out of the project before the first install.
    fs::create_dir_all(root.join(".cairn")).unwrap();
    std::os::unix::fs::symlink(&away, root.join(".cairn/state")).unwrap();

    let install = pack(&root, &["install"]);
    assert_ne!(
        install.code, 0,
        "publishing the ledger through a symlinked parent must fail closed"
    );
    assert!(
        !away.join("agent-pack.json").exists(),
        "the ownership ledger must never be written outside the project root"
    );
}

#[cfg(unix)]
#[test]
fn uninstall_never_removes_a_file_outside_the_project() {
    let root = temp_root("uninstall-escape");
    let away = outside("uninstall-escape");
    assert_eq!(pack(&root, &["install"]).code, 0);

    // The recorded row still resolves, but only by traversing a symlink whose
    // target holds the very bytes the ledger records, so the hash check alone
    // would authorise the delete.
    let skills = away.join("cairn-dev");
    fs::create_dir_all(&skills).unwrap();
    fs::copy(root.join(ROUTER), skills.join("SKILL.md")).unwrap();
    fs::remove_dir_all(root.join(".claude/skills/cairn-dev")).unwrap();
    std::os::unix::fs::symlink(&skills, root.join(".claude/skills/cairn-dev")).unwrap();

    assert_eq!(pack(&root, &["uninstall"]).code, 0);
    assert!(
        skills.join("SKILL.md").exists(),
        "uninstall must not delete through a symlinked parent directory"
    );
}

#[test]
fn a_ledger_row_that_escapes_the_project_is_refused() {
    let root = temp_root("ledger-row");
    let away = outside("ledger-row");
    assert_eq!(pack(&root, &["install"]).code, 0);

    let victim = away.join("victim.md");
    fs::write(&victim, "victim\n").unwrap();
    let digest = {
        use sha2::Digest;
        let mut hasher = sha2::Sha256::new();
        hasher.update(fs::read(&victim).unwrap());
        hasher
            .finalize()
            .iter()
            .fold(String::new(), |mut acc, byte| {
                use std::fmt::Write;
                let _ = write!(acc, "{byte:02x}");
                acc
            })
    };

    // A hand-edited ledger naming a traversal path, at the target's real hash.
    let mut ledger: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(root.join(MANIFEST)).unwrap()).unwrap();
    let relative = format!(
        "../{}/victim.md",
        away.file_name().unwrap().to_string_lossy()
    );
    ledger["files"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({
            "path": relative,
            "sha256": digest,
        }));
    fs::write(
        root.join(MANIFEST),
        serde_json::to_string_pretty(&ledger).unwrap(),
    )
    .unwrap();

    // The dispatcher's adapter-root check refuses this ledger outright, before
    // the containment guard is ever consulted. Both layers must hold: assert
    // the refusal is observable, not merely that nothing was deleted, or the
    // test would still pass if uninstall silently skipped the ledger.
    let uninstall = pack(&root, &["uninstall"]);
    assert_ne!(
        uninstall.code, 0,
        "a ledger naming a path outside the pack must be refused, not acted on"
    );
    assert!(
        uninstall.stderr.contains(MANIFEST),
        "the refusal must name the hand-edited ledger: {}",
        uninstall.stderr
    );
    assert!(
        victim.exists(),
        "a ledger row containing `..` must never reach a filesystem removal"
    );
}

#[cfg(unix)]
#[test]
fn a_non_regular_file_at_an_owned_destination_does_not_block_a_verb() {
    let root = temp_root("fifo");
    assert_eq!(pack(&root, &["install"]).code, 0);

    fs::remove_file(root.join(ROUTER)).unwrap();
    let made = std::process::Command::new("mkfifo")
        .arg(root.join(ROUTER))
        .status()
        .expect("mkfifo must be available to exercise the blocking-read regression");
    assert!(made.success(), "could not create the FIFO fixture");

    // The regression this defends is an unbounded read: a following read on a
    // FIFO with no writer never returns. Run the real binary in a child so the
    // reintroduced hang fails this test instead of wedging the whole suite.
    let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_cairn"))
        .args([
            "--file",
            &root.join("cairn.blueprint").to_string_lossy(),
            "pack",
            "status",
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();

    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    let finished = loop {
        match child.try_wait().unwrap() {
            Some(status) => break Some(status),
            None if std::time::Instant::now() >= deadline => break None,
            None => std::thread::sleep(std::time::Duration::from_millis(50)),
        }
    };
    if finished.is_none() {
        let _ = child.kill();
        let _ = child.wait();
    }
    let Some(status) = finished else {
        panic!("pack status blocked on a FIFO at an owned destination instead of classifying it");
    };
    assert!(
        status.success(),
        "pack status must classify a FIFO and succeed, not abort: {status}"
    );
}

#[cfg(unix)]
#[test]
fn campaign_end_never_removes_a_snapshot_outside_the_project() {
    let root = temp_root("campaign-escape");
    let away = outside("campaign-escape");
    assert_eq!(pack(&root, &["install", "--loop"]).code, 0);

    // `campaign end` runs before the ownership ledger is consulted, so it is
    // the one verb the ledger's adapter-root check cannot protect.
    fs::rename(root.join(".cairn/state"), away.join("state")).unwrap();
    std::os::unix::fs::symlink(away.join("state"), root.join(".cairn/state")).unwrap();
    let victim = away.join("state/agent-pack-campaign.json");
    fs::write(&victim, "{\"victim\":true}\n").unwrap();

    let end = pack(&root, &["campaign", "end"]);
    assert_ne!(
        end.code, 0,
        "releasing a campaign through a symlinked state directory must fail closed"
    );
    assert!(
        victim.exists(),
        "campaign end must not delete a snapshot outside the project root"
    );
}

#[cfg(unix)]
#[test]
fn init_never_scaffolds_outside_the_project() {
    let root = temp_root("init-escape");
    let away = outside("init-escape");
    fs::remove_file(root.join("cairn.blueprint")).unwrap();

    // Scaffolding runs before the pack lifecycle, so the pack's own guard is
    // too late: the agent guide and state directory land first.
    std::os::unix::fs::symlink(&away, root.join(".cairn")).unwrap();

    let argv = [
        "--file".to_owned(),
        root.join("cairn.blueprint").to_string_lossy().to_string(),
        "init".to_owned(),
        "--wire".to_owned(),
    ];
    let init = cairn::cli::run(&argv);
    assert_ne!(
        init.code, 0,
        "scaffolding through a symlinked `.cairn` must fail closed"
    );
    assert!(
        !away.join("AGENTS.md").exists() && !away.join("state").exists(),
        "init must not write the agent guide or state directory outside the project"
    );
}

#[cfg(unix)]
#[test]
fn wire_never_blocks_on_a_non_regular_instructions_file() {
    let root = temp_root("wire-fifo");
    // `init --wire` detects the instructions file, then reads it. A FIFO with
    // no writer never returns from that read, so the type must be refused.
    let made = std::process::Command::new("mkfifo")
        .arg(root.join("AGENTS.md"))
        .status()
        .expect("mkfifo must be available to exercise the blocking-read regression");
    assert!(made.success(), "could not create the FIFO fixture");

    let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_cairn"))
        .args([
            "--file",
            &root.join("cairn.blueprint").to_string_lossy(),
            "init",
            "--wire",
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();

    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    let finished = loop {
        match child.try_wait().unwrap() {
            Some(status) => break Some(status),
            None if std::time::Instant::now() >= deadline => break None,
            None => std::thread::sleep(std::time::Duration::from_millis(50)),
        }
    };
    if finished.is_none() {
        let _ = child.kill();
        let _ = child.wait();
    }
    let Some(status) = finished else {
        panic!("init --wire blocked on a FIFO instructions file instead of refusing it");
    };
    assert!(
        !status.success(),
        "a non-regular instructions file must be refused, not accepted"
    );
}

#[cfg(unix)]
#[test]
fn an_unreadable_owned_file_is_never_overwritten_or_silently_dropped() {
    use std::os::unix::fs::PermissionsExt;
    let root = temp_root("unreadable");
    assert_eq!(pack(&root, &["install"]).code, 0);

    // Present, owned, regular, and unreadable, holding bytes that differ from
    // the bundle so an erroneous write is detectable. The parent stays
    // writable, so an atomic replace would still succeed: only classification
    // stops it.
    let owned = root.join(ROUTER);
    let sentinel = "cairn-unreadable-sentinel\n";
    fs::write(&owned, sentinel).unwrap();
    fs::set_permissions(&owned, fs::Permissions::from_mode(0o000)).unwrap();
    if fs::read(&owned).is_ok() {
        // Running as root, where mode 0o000 is not unreadable. The property
        // under test cannot be established here.
        fs::set_permissions(&owned, fs::Permissions::from_mode(0o644)).unwrap();
        return;
    }

    let status: serde_json::Value =
        serde_json::from_str(&pack(&root, &["status", "--json"]).stdout).unwrap();
    assert!(
        status["data"]["modified"]
            .as_array()
            .unwrap()
            .iter()
            .any(|path| path == ROUTER),
        "an unreadable owned file must be reported, not treated as missing: {status}"
    );

    assert_eq!(pack(&root, &["update"]).code, 0);
    fs::set_permissions(&owned, fs::Permissions::from_mode(0o644)).unwrap();
    assert_eq!(
        fs::read_to_string(&owned).unwrap(),
        sentinel,
        "update must not overwrite a file it cannot read"
    );
    fs::set_permissions(&owned, fs::Permissions::from_mode(0o000)).unwrap();

    let uninstall: serde_json::Value =
        serde_json::from_str(&pack(&root, &["uninstall", "--json"]).stdout).unwrap();
    assert!(
        uninstall["data"]["kept"]
            .as_array()
            .unwrap()
            .iter()
            .any(|path| path == ROUTER),
        "an unreadable owned file must be reported as kept, not dropped: {uninstall}"
    );
    fs::set_permissions(&owned, fs::Permissions::from_mode(0o644)).unwrap();
}
