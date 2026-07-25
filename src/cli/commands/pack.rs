//! `cairn pack`: install, update, inspect, and remove the agent pack.
//!
//! One lifecycle family with a harness selector, and one manifest recording
//! what the packager owns (`dec.agent-pack-packaging` clauses 4 and 5). The
//! packager writes, refreshes, or retires only files the manifest lists at a
//! matching hash: anything a user edited is reported and left alone.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use super::pack_assets::{PackAsset, all_assets};
use super::pack_manifest::{
    Action, BUNDLE_VERSION, InstalledFile, InstalledManifest, MANIFEST_PATH, MIGRATION_NOTES,
    classify, digest, has_loop_assets, migration_notes, read_manifest, write_manifest,
};
use super::wire::{atomic_write, check_symlink_containment};
use std::fs;

/// Harnesses with a validated adapter. Unverified rows are contracts, not
/// facts (`dec.agent-pack-packaging` clause 2), so they are not offered here.
const SUPPORTED_HARNESSES: &[&str] = &["claude"];

/// Dispatches `cairn pack <subcommand>`.
pub(crate) fn run_pack_command(parsed: &ParsedArgs, root: &Path) -> CliResult {
    let harness = match select_harness(&parsed.command_args) {
        Ok(name) => name,
        Err(result) => return result,
    };
    let with_loop = parsed.command_args.iter().any(|a| a == "--loop");
    match subcommand(&parsed.command_args) {
        Some("install" | "update") => run_apply(root, harness, with_loop, parsed),
        Some("status") => run_status(root, with_loop, parsed),
        Some("uninstall") => run_uninstall(root, parsed),
        _ => err(2, copy::lookup("pack.usage")),
    }
}

/// First bare token after `pack`, so `pack --harness claude install` and
/// `pack install --harness claude` mean the same thing. `--harness` takes a
/// value, which must not be mistaken for the subcommand.
fn subcommand(args: &[String]) -> Option<&str> {
    let mut rest = args.iter().skip(1);
    while let Some(arg) = rest.next() {
        if arg == "--harness" {
            let _ = rest.next();
        } else if !arg.starts_with('-') {
            return Some(arg.as_str());
        }
    }
    None
}

/// Resolve `--harness <name>`, defaulting to the single validated adapter.
fn select_harness(args: &[String]) -> Result<&'static str, CliResult> {
    let requested = args
        .iter()
        .position(|a| a == "--harness")
        .and_then(|idx| args.get(idx + 1));
    match requested {
        None => Ok(SUPPORTED_HARNESSES[0]),
        Some(name) => SUPPORTED_HARNESSES
            .iter()
            .find(|supported| *supported == name)
            .copied()
            .ok_or_else(|| {
                err(
                    2,
                    &copy::lookup("pack.err-unknown-harness")
                        .replace("{harness}", name)
                        .replace("{supported}", &SUPPORTED_HARNESSES.join(", ")),
                )
            }),
    }
}

#[derive(Default)]
struct ApplyReport {
    written: Vec<String>,
    refreshed: Vec<String>,
    adopted: Vec<String>,
    modified: Vec<String>,
    unchanged: Vec<String>,
}

/// Install or update. Both verbs run the same engine: the difference is only
/// whether a ledger already existed, which the report states.
fn run_apply(root: &Path, harness: &str, with_loop: bool, parsed: &ParsedArgs) -> CliResult {
    let existing = match read_manifest(root) {
        Ok(manifest) => manifest,
        Err(result) => return result,
    };
    let owned: std::collections::BTreeMap<&str, &str> =
        existing
            .as_ref()
            .map_or_else(std::collections::BTreeMap::new, |manifest| {
                manifest
                    .files
                    .iter()
                    .map(|file| (file.path.as_str(), file.sha256.as_str()))
                    .collect()
            });

    // A ledger that already owns loop assets keeps owning them: an update
    // without `--loop` must still refresh what it is responsible for, or
    // `status` reports outdated files that no verb ever fixes.
    let assets = all_assets(with_loop || existing.as_ref().is_some_and(has_loop_assets));
    let mut report = ApplyReport::default();
    let mut ledger = Vec::new();

    for asset in &assets {
        let action = classify(root, asset, owned.get(asset.path).copied());
        let full = root.join(asset.path);
        if action == Action::Modified {
            report.modified.push(asset.path.to_owned());
            // Keep ownership of a file we already owned so a later edit-revert
            // returns it to pristine rather than orphaning it.
            if let Some(recorded) = owned.get(asset.path) {
                ledger.push(InstalledFile {
                    path: asset.path.to_owned(),
                    sha256: (*recorded).to_owned(),
                });
            }
            continue;
        }
        if action == Action::Backfill || action == Action::Refresh {
            if let Err(result) = write_asset(root, &full, asset) {
                // Publish all prior ownership plus what already landed before
                // surfacing the failure. Dropping later owned rows here would
                // orphan exactly the files this recovery path protects.
                carry_owned(existing.as_ref(), &mut ledger);
                let _ = write_manifest(root, harness, ledger);
                return result;
            }
            if action == Action::Backfill {
                report.written.push(asset.path.to_owned());
            } else {
                report.refreshed.push(asset.path.to_owned());
            }
        } else if action == Action::Adopt {
            report.adopted.push(asset.path.to_owned());
        } else {
            report.unchanged.push(asset.path.to_owned());
        }
        ledger.push(InstalledFile {
            path: asset.path.to_owned(),
            sha256: digest(asset.content.as_bytes()),
        });
    }

    carry_owned(existing.as_ref(), &mut ledger);

    let notes = existing.as_ref().map_or_else(Vec::new, |manifest| {
        migration_notes(MIGRATION_NOTES, &manifest.bundle_version, BUNDLE_VERSION)
    });
    let previous = existing.map(|manifest| manifest.bundle_version);

    if let Err(result) = write_manifest(root, harness, ledger) {
        return result;
    }

    if parsed.json {
        return ok(apply_json(harness, &report));
    }
    ok(render_apply_human(
        harness,
        previous.is_some(),
        &report,
        &notes,
    ))
}

/// Merge every previously-owned row not already replaced by this run, then
/// normalise the ledger for deterministic publication.
fn carry_owned(existing: Option<&InstalledManifest>, ledger: &mut Vec<InstalledFile>) {
    if let Some(manifest) = existing {
        for file in &manifest.files {
            if !ledger.iter().any(|entry| entry.path == file.path) {
                ledger.push(InstalledFile {
                    path: file.path.clone(),
                    sha256: file.sha256.clone(),
                });
            }
        }
    }
    ledger.sort_by(|a, b| a.path.cmp(&b.path));
    ledger.dedup_by(|a, b| a.path == b.path);
}

fn render_apply_human(
    harness: &str,
    updating: bool,
    report: &ApplyReport,
    notes: &[&str],
) -> String {
    let mut out = copy::lookup(if updating {
        "pack.updated"
    } else {
        "pack.installed"
    })
    .replace("{harness}", harness)
    .replace("{bundle}", BUNDLE_VERSION)
    .replace("{cli}", env!("CARGO_PKG_VERSION"));
    out.push('\n');
    for (label, paths) in [
        ("written", &report.written),
        ("refreshed", &report.refreshed),
        ("adopted", &report.adopted),
        ("unchanged", &report.unchanged),
    ] {
        let _ = writeln!(out, "  {label}: {}", paths.len());
    }
    append_modified(&mut out, &report.modified);
    for note in notes {
        out.push_str(note);
        out.push('\n');
    }
    out
}

/// Write one asset, refusing any destination reachable through a symlink.
fn write_asset(root: &Path, full: &Path, asset: &PackAsset) -> Result<(), CliResult> {
    let containment = check_symlink_containment(root, full);
    if containment.code != 0 {
        return Err(containment);
    }
    let Some(parent) = full.parent() else {
        return Err(err(1, "pack destination has no parent directory"));
    };
    if let Err(error) = fs::create_dir_all(parent) {
        return Err(err(
            1,
            &copy::lookup("pack.err-write")
                .replace("{file}", asset.path)
                .replace("{detail}", &error.to_string()),
        ));
    }
    atomic_write(parent, full, asset.content).map_err(|detail| {
        err(
            1,
            &copy::lookup("pack.err-write")
                .replace("{file}", asset.path)
                .replace("{detail}", &detail),
        )
    })
}

/// Report installed versus bundled state. Drift is information, never a
/// failure exit (`dec.agent-pack-packaging` clause 6).
fn run_status(root: &Path, with_loop: bool, parsed: &ParsedArgs) -> CliResult {
    let manifest = match read_manifest(root) {
        Ok(manifest) => manifest,
        Err(result) => return result,
    };
    let owned: std::collections::BTreeMap<&str, &str> =
        manifest
            .as_ref()
            .map_or_else(std::collections::BTreeMap::new, |manifest| {
                manifest
                    .files
                    .iter()
                    .map(|file| (file.path.as_str(), file.sha256.as_str()))
                    .collect()
            });
    let assets = all_assets(with_loop || manifest.as_ref().is_some_and(has_loop_assets));
    let mut missing = Vec::new();
    let mut modified = Vec::new();
    let mut stale = Vec::new();
    let mut pristine = Vec::new();
    for asset in &assets {
        match classify(root, asset, owned.get(asset.path).copied()) {
            Action::Backfill => missing.push(asset.path.to_owned()),
            Action::Modified => modified.push(asset.path.to_owned()),
            Action::Refresh => stale.push(asset.path.to_owned()),
            Action::Adopt | Action::Unchanged => pristine.push(asset.path.to_owned()),
        }
    }
    if parsed.json {
        return ok(status_json(
            manifest.as_ref(),
            &missing,
            &modified,
            &stale,
            &pristine,
        ));
    }
    let Some(manifest) = manifest else {
        let mut out = copy::lookup("pack.not-installed").replace("{file}", MANIFEST_PATH);
        out.push('\n');
        if !pristine.is_empty() {
            let _ = writeln!(
                out,
                "{}",
                copy::lookup("pack.legacy-detected")
                    .replace("{count}", &pristine.len().to_string())
            );
        }
        return ok(out);
    };
    let mut out = copy::lookup("pack.status-header")
        .replace("{harness}", &manifest.harness)
        .replace("{bundle}", &manifest.bundle_version)
        .replace("{cli}", &manifest.cli_version);
    out.push('\n');
    if manifest.bundle_version != BUNDLE_VERSION
        || manifest.cli_version != env!("CARGO_PKG_VERSION")
    {
        let _ = writeln!(
            out,
            "{}",
            copy::lookup("pack.drift")
                .replace("{bundle}", BUNDLE_VERSION)
                .replace("{cli}", env!("CARGO_PKG_VERSION"))
        );
    }
    let _ = writeln!(
        out,
        "  pristine: {}\n  outdated: {}\n  missing: {}",
        pristine.len(),
        stale.len(),
        missing.len()
    );
    append_modified(&mut out, &modified);
    ok(out)
}

/// Retire owned, pristine files. Anything edited stays, and so does anything
/// the ledger never claimed.
fn run_uninstall(root: &Path, parsed: &ParsedArgs) -> CliResult {
    let manifest = match read_manifest(root) {
        Ok(Some(manifest)) => manifest,
        Ok(None) => {
            let message = copy::lookup("pack.not-installed").replace("{file}", MANIFEST_PATH);
            return if parsed.json {
                ok(envelope(
                    "pack uninstall",
                    &serde_json::json!({"removed": Vec::<String>::new(), "kept": Vec::<String>::new()}),
                ))
            } else {
                ok(format!("{message}\n"))
            };
        }
        Err(result) => return result,
    };
    let mut removed = Vec::new();
    let mut kept = Vec::new();
    for file in &manifest.files {
        let full = root.join(&file.path);
        match fs::read(&full) {
            Ok(disk) if digest(&disk) == file.sha256 => {
                if fs::remove_file(&full).is_err() {
                    kept.push(file.path.clone());
                    continue;
                }
                removed.push(file.path.clone());
                prune_empty_parents(root, &full);
            }
            Ok(_) => kept.push(file.path.clone()),
            // Already gone: nothing to retire, nothing to report as kept.
            Err(_) => {}
        }
    }
    if fs::remove_file(root.join(MANIFEST_PATH)).is_err() {
        return err(
            1,
            &copy::lookup("pack.err-write")
                .replace("{file}", MANIFEST_PATH)
                .replace("{detail}", "could not remove the manifest"),
        );
    }
    if parsed.json {
        return ok(envelope(
            "pack uninstall",
            &serde_json::json!({"removed": removed, "kept": kept}),
        ));
    }
    let mut out = copy::lookup("pack.removed").replace("{count}", &format!("{}", removed.len()));
    out.push('\n');
    append_modified(&mut out, &kept);
    ok(out)
}

/// Remove directories the retired file left empty, stopping at the first
/// non-empty one and never leaving the project root.
fn prune_empty_parents(root: &Path, file: &Path) {
    let mut current = file.parent().map(Path::to_path_buf);
    while let Some(dir) = current {
        if dir == root || !dir.starts_with(root) {
            return;
        }
        let Ok(mut entries) = fs::read_dir(&dir) else {
            return;
        };
        if entries.next().is_some() || fs::remove_dir(&dir).is_err() {
            return;
        }
        current = dir.parent().map(Path::to_path_buf);
    }
}

/// Append the never-overwritten list, if there is one.
fn append_modified(out: &mut String, modified: &[String]) {
    if modified.is_empty() {
        return;
    }
    out.push_str(copy::lookup("pack.modified-note"));
    out.push('\n');
    for path in modified {
        out.push_str("  ");
        out.push_str(path);
        out.push('\n');
    }
}

/// Wrap a payload in the CLI's `{command, status, data}` envelope. Built
/// through `serde_json` rather than string formatting: these payloads carry
/// filesystem paths, which are exactly the values hand-rolled escaping gets
/// wrong.
fn envelope(command: &str, data: &serde_json::Value) -> String {
    let body = serde_json::json!({"command": command, "status": "ok", "data": data});
    format!("{body}\n")
}

fn apply_json(harness: &str, report: &ApplyReport) -> String {
    envelope(
        "pack",
        &serde_json::json!({
            "harness": harness,
            "bundle_version": BUNDLE_VERSION,
            "cli_version": env!("CARGO_PKG_VERSION"),
            "written": report.written,
            "refreshed": report.refreshed,
            "adopted": report.adopted,
            "modified": report.modified,
            "unchanged": report.unchanged,
        }),
    )
}

fn status_json(
    manifest: Option<&InstalledManifest>,
    missing: &[String],
    modified: &[String],
    stale: &[String],
    pristine: &[String],
) -> String {
    envelope(
        "pack status",
        &serde_json::json!({
            "installed": manifest.is_some(),
            "harness": manifest.map(|m| m.harness.as_str()),
            "installed_bundle_version": manifest.map(|m| m.bundle_version.as_str()),
            "installed_cli_version": manifest.map(|m| m.cli_version.as_str()),
            "bundle_version": BUNDLE_VERSION,
            "cli_version": env!("CARGO_PKG_VERSION"),
            "pristine": pristine,
            "outdated": stale,
            "missing": missing,
            "modified": modified,
        }),
    )
}
