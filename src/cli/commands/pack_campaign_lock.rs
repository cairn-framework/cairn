//! Campaign lock: pin resolved pack bytes, verify them, and release them.
//!
//! The resolver in `pack_campaign` says what the bytes are. This module makes
//! them immutable for a campaign: one exclusive snapshot, read-only copies the
//! session loads from, and a fail-closed check before each fresh session
//! (`dec.unified-cairn-dev-entry` clause 9).

// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;

use super::pack_campaign::{
    Buffered, HeldAsset, PINNED_ROOT, Resolution, SNAPSHOT_PATH, campaign_json, envelope,
    pinned_root, render_resolution, resolve,
};
use super::pack_manifest::digest;
use super::wire::check_symlink_containment;
use std::fs;

pub(crate) fn run_campaign(
    root: &Path,
    verb: Option<&str>,
    with_loop: bool,
    json: bool,
) -> CliResult {
    match verb {
        Some("start") => campaign_start(root, with_loop, json),
        Some("verify") => campaign_verify(root, json),
        Some("end") => campaign_end(root, json),
        _ => err(2, copy::lookup("pack.campaign-usage")),
    }
}

/// A snapshot is a file anyone can edit, so nothing read from it reaches the
/// filesystem until it looks like something this command could have written:
/// a hex digest for the campaign directory, and project-relative pack paths.
fn validate_snapshot(snapshot: &Resolution) -> Result<(), CliResult> {
    let digest_ok = snapshot.bundle_digest.len() == 64
        && snapshot
            .bundle_digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
    let paths_ok = std::iter::once(&snapshot.prompt)
        .chain(&snapshot.closure)
        .all(|asset| {
            asset.path.starts_with(".claude/")
                && Path::new(&asset.path)
                    .components()
                    .all(|part| matches!(part, std::path::Component::Normal(_)))
        });
    if digest_ok && paths_ok {
        return Ok(());
    }
    Err(err(1, copy::lookup("pack.campaign-unreadable")))
}

/// Claim the snapshot path exclusively, then fill it. Creation is the lock: two
/// concurrent starts cannot both win, and an active campaign is never repinned.
/// A crash between claim and fill leaves an unparseable snapshot, which halts
/// verification rather than pinning half a campaign.
fn publish_snapshot(root: &Path, body: &str) -> Result<(), CliResult> {
    use std::io::Write as _;
    let path = root.join(SNAPSHOT_PATH);
    // The claim's own path must not be reachable through a symlink, or the
    // campaign state lands outside the project.
    let containment = check_symlink_containment(root, &path);
    if containment.code != 0 {
        return Err(containment);
    }
    if let Some(parent) = path.parent()
        && let Err(error) = fs::create_dir_all(parent)
    {
        return Err(err(
            1,
            &copy::lookup("pack.err-write")
                .replace("{file}", SNAPSHOT_PATH)
                .replace("{detail}", &error.to_string()),
        ));
    }
    let mut file = match fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
    {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            return Err(err(1, copy::lookup("pack.campaign-active")));
        }
        Err(error) => {
            return Err(err(
                1,
                &copy::lookup("pack.err-write")
                    .replace("{file}", SNAPSHOT_PATH)
                    .replace("{detail}", &error.to_string()),
            ));
        }
    };
    file.write_all(body.as_bytes()).map_err(|error| {
        err(
            1,
            &copy::lookup("pack.err-write")
                .replace("{file}", SNAPSHOT_PATH)
                .replace("{detail}", &error.to_string()),
        )
    })
}

/// Write every pinned asset from the bytes the resolution buffered, mirroring
/// its pack path under this campaign's pinned root, and mark each copy
/// read-only. Nothing is reread from the live pack: the copies are exactly the
/// bytes that were hashed.
fn pin_bytes(root: &Path, buffered: &Buffered) -> Result<(), CliResult> {
    let base = root.join(pinned_root(&buffered.resolution));
    let mut written: Vec<&str> = Vec::new();
    for held in std::iter::once(&buffered.prompt).chain(&buffered.closure) {
        // The router entry is its own closure head in some layouts: write each
        // distinct path once, since the first copy is already read-only.
        if written.contains(&held.asset.path.as_str()) {
            continue;
        }
        // Refuse a destination reachable through a symlink before creating any
        // of it: exclusive creation only guards the final component.
        let containment = check_symlink_containment(root, &base.join(&held.asset.path));
        if containment.code != 0 {
            return Err(containment);
        }
        if let Err(error) = write_pinned(&base, held) {
            return Err(err(
                1,
                &copy::lookup("pack.err-write")
                    .replace("{file}", &held.asset.path)
                    .replace("{detail}", &error.to_string()),
            ));
        }
        written.push(&held.asset.path);
    }
    Ok(())
}

fn write_pinned(base: &Path, held: &HeldAsset) -> std::io::Result<()> {
    let target = base.join(&held.asset.path);
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }
    // Exclusive creation, so a path planted ahead of this write (a symlink out
    // of the campaign tree, or a hard link to live bytes) is refused rather
    // than followed and overwritten.
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&target)?;
    std::io::Write::write_all(&mut file, &held.bytes)?;
    drop(file);
    let mut permissions = fs::metadata(&target)?.permissions();
    permissions.set_readonly(true);
    fs::set_permissions(&target, permissions)
}

/// Confirm the pinned copies still hold the bytes the snapshot recorded.
fn verify_pinned(root: &Path, snapshot: &Resolution) -> Result<(), String> {
    let base = root.join(pinned_root(snapshot));
    for asset in std::iter::once(&snapshot.prompt).chain(&snapshot.closure) {
        let Ok(bytes) = fs::read(base.join(&asset.path)) else {
            return Err(copy::lookup("pack.campaign-absent").replace("{file}", &asset.path));
        };
        if digest(&bytes) != asset.sha256 {
            return Err(copy::lookup("pack.campaign-changed").replace("{file}", &asset.path));
        }
    }
    Ok(())
}

fn campaign_start(root: &Path, with_loop: bool, json: bool) -> CliResult {
    let buffered = match resolve(root, with_loop) {
        Ok(buffered) => buffered,
        Err(result) => return result,
    };
    let resolution = &buffered.resolution;
    let Ok(body) = serde_json::to_string_pretty(resolution) else {
        return err(1, "could not serialise the campaign snapshot");
    };
    // The snapshot file is the claim: take it before writing any pinned bytes,
    // so a second starter cannot populate a campaign it did not win.
    if let Err(result) = publish_snapshot(root, &format!("{body}\n")) {
        return result;
    }
    if let Err(result) = pin_bytes(root, &buffered) {
        // Only take back a claim that is still this campaign's: a concurrent
        // end plus start can already have replaced it, and deleting a
        // successor's state would be worse than leaving debris.
        if read_snapshot(root).ok().as_ref() == Some(resolution) {
            let _ = remove_pinned(root, resolution);
            let _ = fs::remove_file(root.join(SNAPSHOT_PATH));
        }
        return result;
    }
    // The claim must still be present. An `end` racing the pinning above would
    // otherwise leave a started campaign with no snapshot; report the
    // concurrency and leave whatever now owns the path alone.
    if read_snapshot(root).ok().as_ref() != Some(resolution) {
        return err(1, copy::lookup("pack.resolve-concurrent"));
    }
    if json {
        return ok(envelope("pack campaign start", &campaign_json(resolution)));
    }
    ok(format!(
        "{}\n{}  pinned: {}\n",
        copy::lookup("pack.campaign-started")
            .replace("{entry}", &resolution.entry)
            .replace("{count}", &resolution.closure.len().to_string()),
        render_resolution(resolution),
        pinned_root(resolution)
    ))
}

/// Verify the live pack against the snapshot before a fresh session. Any
/// difference halts: the campaign's bytes are immutable for its duration.
fn campaign_verify(root: &Path, json: bool) -> CliResult {
    let snapshot = match read_snapshot(root) {
        Ok(snapshot) => snapshot,
        Err(result) => return result,
    };
    let live = match resolve(root, snapshot.entry == "loop") {
        Ok(buffered) => buffered.resolution,
        Err(mut result) => {
            result.stderr = format!("{}\n{}", copy::lookup("pack.campaign-halt"), result.stderr);
            return result;
        }
    };
    if live != snapshot {
        let detail = mismatch_detail(&snapshot, &live);
        return err(
            1,
            &format!("{}\n{detail}", copy::lookup("pack.campaign-halt")),
        );
    }
    // The pinned copies are what the session will actually load, so they are
    // part of what verification proves.
    if let Err(detail) = verify_pinned(root, &snapshot) {
        return err(
            1,
            &format!("{}\n{detail}", copy::lookup("pack.campaign-halt")),
        );
    }
    if json {
        return ok(envelope("pack campaign verify", &campaign_json(&live)));
    }
    ok(format!(
        "{}\n  pinned: {}\n",
        copy::lookup("pack.campaign-verified")
            .replace("{entry}", &live.entry)
            .replace("{count}", &live.closure.len().to_string()),
        pinned_root(&live)
    ))
}

/// Drop one campaign's pinned copies. They are read-only by design, so on
/// Windows the bit comes off first.
fn remove_pinned(root: &Path, resolution: &Resolution) -> std::io::Result<()> {
    let pinned = root.join(pinned_root(resolution));
    // A recursive delete never follows a path out of the project, whatever the
    // snapshot or an intermediate symlink claims.
    if check_symlink_containment(root, &pinned).code != 0 {
        return Err(std::io::Error::other(
            "the campaign directory is not contained by the project root",
        ));
    }
    if !pinned.exists() {
        return Ok(());
    }
    // Unix deletes a read-only file when its directory is writable; Windows
    // refuses, so the bit comes off there before the tree goes.
    #[cfg(windows)]
    clear_readonly(&pinned)?;
    fs::remove_dir_all(pinned)
}

#[cfg(windows)]
fn clear_readonly(dir: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            clear_readonly(&path)?;
        } else {
            let mut permissions = fs::metadata(&path)?.permissions();
            // Reason: the pinned copy is deliberately read-only; Windows cannot
            // delete it until that is undone.
            #[allow(clippy::permissions_set_readonly_false)]
            permissions.set_readonly(false);
            fs::set_permissions(&path, permissions)?;
        }
    }
    Ok(())
}

fn campaign_end(root: &Path, json: bool) -> CliResult {
    if !root.join(SNAPSHOT_PATH).exists() {
        return if json {
            ok(envelope(
                "pack campaign end",
                &serde_json::json!({"ended": false}),
            ))
        } else {
            ok(format!("{}\n", copy::lookup("pack.campaign-none")))
        };
    }
    // An interrupted start leaves a claim nothing can parse. `end` is the
    // release verb, so it clears that state too: otherwise the campaign could
    // never be ended and no new one could start.
    if let Ok(snapshot) = read_snapshot(root) {
        // Release the bytes before the claim. A start racing behind an early
        // snapshot removal would otherwise lose its pinned tree.
        if let Err(error) = remove_pinned(root, &snapshot) {
            return err(
                1,
                &copy::lookup("pack.err-write")
                    .replace("{file}", &pinned_root(&snapshot))
                    .replace("{detail}", &error.to_string()),
            );
        }
    } else {
        let base = root.join(PINNED_ROOT);
        if check_symlink_containment(root, &base).code == 0 {
            let _ = fs::remove_dir_all(base);
        }
    }
    if let Err(error) = fs::remove_file(root.join(SNAPSHOT_PATH)) {
        return err(
            1,
            &copy::lookup("pack.err-write")
                .replace("{file}", SNAPSHOT_PATH)
                .replace("{detail}", &error.to_string()),
        );
    }
    if json {
        return ok(envelope(
            "pack campaign end",
            &serde_json::json!({"ended": true}),
        ));
    }
    ok(format!("{}\n", copy::lookup("pack.campaign-ended")))
}

/// Read the pinned campaign. The snapshot is user-writable, so it is validated
/// before any of its values reach the filesystem.
fn read_snapshot(root: &Path) -> Result<Resolution, CliResult> {
    let Ok(body) = fs::read_to_string(root.join(SNAPSHOT_PATH)) else {
        return Err(err(1, copy::lookup("pack.campaign-none")));
    };
    let snapshot: Resolution = serde_json::from_str(&body)
        .map_err(|_| err(1, copy::lookup("pack.campaign-unreadable")))?;
    validate_snapshot(&snapshot)?;
    Ok(snapshot)
}

/// Name what changed. A campaign halt must say which bytes moved, or the
/// operator cannot tell an interrupted update from a hand edit.
fn mismatch_detail(snapshot: &Resolution, live: &Resolution) -> String {
    if snapshot.prompt != live.prompt {
        return copy::lookup("pack.campaign-changed").replace("{file}", &snapshot.prompt.path);
    }
    for pinned in &snapshot.closure {
        match live.closure.iter().find(|asset| asset.path == pinned.path) {
            None => return copy::lookup("pack.campaign-absent").replace("{file}", &pinned.path),
            Some(current) if current.sha256 != pinned.sha256 => {
                return copy::lookup("pack.campaign-changed").replace("{file}", &pinned.path);
            }
            Some(_) => {}
        }
    }
    for current in &live.closure {
        if !snapshot
            .closure
            .iter()
            .any(|pinned| pinned.path == current.path)
        {
            return copy::lookup("pack.campaign-added").replace("{file}", &current.path);
        }
    }
    if snapshot.cli_digest != live.cli_digest {
        return copy::lookup("pack.campaign-cli").to_owned();
    }
    copy::lookup("pack.campaign-metadata").to_owned()
}
