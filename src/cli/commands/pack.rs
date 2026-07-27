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
use super::pack_harness::{
    Adapter, ledger_matches_adapter, requested_harness, resolve_adapter, select_adapter,
};
use super::pack_manifest::{
    Action, BUNDLE_VERSION, InstalledFile, InstalledManifest, MANIFEST_PATH, MIGRATION_NOTES,
    classify, digest, has_loop_assets, migration_notes, read_manifest, write_manifest,
};
use super::pack_report::{append_modified, apply_json, envelope, status_json};
use super::wire::{atomic_write, contained_path, readable_path};
use std::fs;

/// Dispatches `cairn pack <subcommand>`.
pub(crate) fn run_pack_command(parsed: &ParsedArgs, root: &Path) -> CliResult {
    let with_loop = parsed.command_args.iter().any(|a| a == "--loop");
    let verb = subcommand(&parsed.command_args);
    let requested = match requested_harness(&parsed.command_args) {
        Ok(name) => name,
        Err(result) => return result,
    };
    // `campaign end` only deletes the snapshot, and it is the documented way out
    // of an unreadable campaign, so it must not be held hostage by an ownership
    // ledger this project cannot parse. `start` and `verify` do read the pack,
    // so they stay behind the ledger and adapter gates below.
    if verb == Some("campaign") && campaign_verb(&parsed.command_args) == Some("end") {
        return super::pack_campaign_lock::run_campaign(
            root,
            campaign_verb(&parsed.command_args),
            requested,
            with_loop,
            parsed.json,
        );
    }
    let installed = match read_manifest(root) {
        Ok(manifest) => manifest,
        Err(result) => return result,
    };
    let adapter = match select_adapter(&parsed.command_args, root, installed.as_ref()) {
        Ok(adapter) => adapter,
        Err(result) => return result,
    };
    if let Some(manifest) = installed.as_ref()
        && !ledger_matches_adapter(manifest, adapter)
    {
        return err(
            1,
            &copy::lookup("pack.err-mixed-ledger")
                .replace("{file}", MANIFEST_PATH)
                .replace("{harness}", adapter.name),
        );
    }
    match verb {
        Some("install" | "update") => run_apply(root, installed, adapter, with_loop, parsed.json),
        Some("status") => run_status(root, installed, adapter, with_loop, parsed),
        Some("uninstall") => run_uninstall(root, installed, parsed),
        Some("resolve") => {
            super::pack_campaign::run_resolve(root, requested, with_loop, parsed.json)
        }
        Some("campaign") => super::pack_campaign_lock::run_campaign(
            root,
            campaign_verb(&parsed.command_args),
            requested,
            with_loop,
            parsed.json,
        ),
        _ => err(2, copy::lookup("pack.usage")),
    }
}

/// Second bare token after `pack`: the campaign verb.
fn campaign_verb(args: &[String]) -> Option<&str> {
    bare_tokens(args).nth(1)
}

/// Bare tokens after `pack`, with the `--harness` value skipped so it can never
/// be mistaken for a subcommand or verb.
fn bare_tokens(args: &[String]) -> impl Iterator<Item = &str> {
    let mut skip_value = false;
    args.iter().skip(1).filter_map(move |arg| {
        if std::mem::take(&mut skip_value) {
            return None;
        }
        skip_value = arg == "--harness";
        (!arg.starts_with('-')).then_some(arg.as_str())
    })
}

/// Install the default base pack through the same ownership engine as
/// `cairn pack install`. Used by `cairn init` so bootstrap and maintenance can
/// never disagree about bytes or ownership.
pub(crate) fn install_default_pack(root: &Path, json: bool) -> CliResult {
    let installed = match read_manifest(root) {
        Ok(manifest) => manifest,
        Err(result) => return result,
    };
    let adapter = match resolve_adapter(None, root, installed.as_ref()) {
        Ok(adapter) => adapter,
        Err(result) => return result,
    };
    run_apply(root, installed, adapter, false, json)
}

/// First bare token after `pack`, so `pack --harness claude install` and
/// `pack install --harness claude` mean the same thing.
fn subcommand(args: &[String]) -> Option<&str> {
    bare_tokens(args).next()
}

#[derive(Default)]
pub(super) struct ApplyReport {
    pub(super) written: Vec<String>,
    pub(super) refreshed: Vec<String>,
    pub(super) adopted: Vec<String>,
    pub(super) modified: Vec<String>,
    pub(super) unchanged: Vec<String>,
}

/// Install or update. Both verbs run the same engine: the difference is only
/// whether a ledger already existed, which the report states.
fn run_apply(
    root: &Path,
    existing: Option<InstalledManifest>,
    adapter: Adapter,
    with_loop: bool,
    json: bool,
) -> CliResult {
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
    let assets = all_assets(
        adapter.pack_root,
        with_loop || existing.as_ref().is_some_and(has_loop_assets),
    );
    let mut report = ApplyReport::default();
    let mut ledger = Vec::new();

    for asset in &assets {
        let action = classify(root, asset, owned.get(asset.path.as_ref()).copied());
        if action == Action::Modified {
            report.modified.push(asset.path.clone().into_owned());
            // Keep ownership of a file we already owned so a later edit-revert
            // returns it to pristine rather than orphaning it.
            if let Some(recorded) = owned.get(asset.path.as_ref()) {
                ledger.push(InstalledFile {
                    path: asset.path.as_ref().to_owned(),
                    sha256: (*recorded).to_owned(),
                });
            }
            continue;
        }
        if action == Action::Backfill || action == Action::Refresh {
            if let Err(result) = write_asset(root, asset) {
                // Publish all prior ownership plus what already landed before
                // surfacing the failure. Dropping later owned rows here would
                // orphan exactly the files this recovery path protects.
                carry_owned(existing.as_ref(), &mut ledger);
                let _ = write_manifest(root, adapter.name, ledger);
                return result;
            }
            if action == Action::Backfill {
                report.written.push(asset.path.as_ref().to_owned());
            } else {
                report.refreshed.push(asset.path.as_ref().to_owned());
            }
        } else if action == Action::Adopt {
            report.adopted.push(asset.path.as_ref().to_owned());
        } else {
            report.unchanged.push(asset.path.as_ref().to_owned());
        }
        ledger.push(InstalledFile {
            path: asset.path.as_ref().to_owned(),
            sha256: digest(asset.content.as_bytes()),
        });
    }

    carry_owned(existing.as_ref(), &mut ledger);

    let notes = existing.as_ref().map_or_else(Vec::new, |manifest| {
        migration_notes(MIGRATION_NOTES, &manifest.bundle_version, BUNDLE_VERSION)
    });
    let previous = existing.map(|manifest| manifest.bundle_version);

    if let Err(result) = write_manifest(root, adapter.name, ledger) {
        return result;
    }

    if json {
        return ok(apply_json(adapter.name, &report));
    }
    ok(render_apply_human(
        adapter.name,
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

/// Write one asset through the single containment policy: project-relative,
/// and no component reachable through a symlink.
fn write_asset(root: &Path, asset: &PackAsset) -> Result<(), CliResult> {
    let owned = contained_path(root, asset.path.as_ref())?;
    let full = owned.as_path();
    let Some(parent) = full.parent() else {
        return Err(err(1, "pack destination has no parent directory"));
    };
    if let Err(error) = fs::create_dir_all(parent) {
        return Err(err(
            1,
            &copy::lookup("pack.err-write")
                .replace("{file}", asset.path.as_ref())
                .replace("{detail}", &error.to_string()),
        ));
    }
    atomic_write(parent, full, asset.content).map_err(|detail| {
        err(
            1,
            &copy::lookup("pack.err-write")
                .replace("{file}", asset.path.as_ref())
                .replace("{detail}", &detail),
        )
    })
}

/// Report installed versus bundled state. Drift is information, never a
/// failure exit (`dec.agent-pack-packaging` clause 6).
fn run_status(
    root: &Path,
    manifest: Option<InstalledManifest>,
    adapter: Adapter,
    with_loop: bool,
    parsed: &ParsedArgs,
) -> CliResult {
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
    let assets = all_assets(
        adapter.pack_root,
        with_loop || manifest.as_ref().is_some_and(has_loop_assets),
    );
    let mut missing = Vec::new();
    let mut modified = Vec::new();
    let mut stale = Vec::new();
    let mut pristine = Vec::new();
    for asset in &assets {
        let path = asset.path.as_ref().to_owned();
        match classify(root, asset, owned.get(asset.path.as_ref()).copied()) {
            Action::Backfill => missing.push(path),
            Action::Modified => modified.push(path),
            Action::Refresh => stale.push(path),
            Action::Adopt | Action::Unchanged => pristine.push(path),
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
fn run_uninstall(
    root: &Path,
    installed: Option<InstalledManifest>,
    parsed: &ParsedArgs,
) -> CliResult {
    let Some(manifest) = installed else {
        let message = copy::lookup("pack.not-installed").replace("{file}", MANIFEST_PATH);
        return if parsed.json {
            ok(envelope(
                "pack uninstall",
                &serde_json::json!({"removed": Vec::<String>::new(), "kept": Vec::<String>::new()}),
            ))
        } else {
            ok(format!("{message}\n"))
        };
    };
    let mut removed = Vec::new();
    let mut kept = Vec::new();
    for file in &manifest.files {
        // Read and remove only through a contained, regular path. A ledger row
        // whose parent became a symlink would otherwise delete the file it
        // points at, outside the project.
        let full = match readable_path(root, &file.path) {
            Ok(Some(full)) => full,
            // Already gone: nothing to retire, nothing to report as kept.
            Ok(None) => continue,
            Err(_) => {
                kept.push(file.path.clone());
                continue;
            }
        };
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
    let ledger = match contained_path(root, MANIFEST_PATH) {
        Ok(path) => path,
        Err(error) => return error,
    };
    if fs::remove_file(&ledger).is_err() {
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
