//! Which adapter a `cairn pack` invocation acts on.
//!
//! An adapter is a pack root (`dec.pack-adapter-roots`): the harness name the
//! ownership ledger records, and the project directory that harness discovers.
//! Both come from one table row in `pack_assets`, so a selector can never name
//! one harness and write another's paths.

// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use super::pack_assets::{CLAUDE_ROOT, HARNESS_ROOTS, OMP_ROOT};
use super::pack_manifest::{InstalledManifest, MANIFEST_PATH};

/// One resolved adapter: the harness name the ledger records, and the pack root
/// it installs into. Both come from the same table row, so a name can never
/// drift from the paths it writes.
#[derive(Clone, Copy)]
pub(crate) struct Adapter {
    pub(crate) name: &'static str,
    pub(crate) pack_root: &'static str,
}

/// Resolve which adapter this invocation acts on. A `--harness` with no value
/// is a usage error, never a silent fall back to detection.
/// The raw `--harness` value, before any ledger is read. A flag with no value
/// is a usage error, never a silent fall back to detection.
pub(crate) fn requested_harness(args: &[String]) -> Result<Option<&str>, CliResult> {
    let Some(flag) = args.iter().position(|a| a == "--harness") else {
        return Ok(None);
    };
    args.get(flag + 1)
        .map(String::as_str)
        // The next token is a value only when it is not itself a flag: reading
        // `--harness --loop` as harness `--loop` would report an unknown
        // adapter instead of the usage error it is.
        .filter(|name| !name.starts_with('-'))
        .map(Some)
        .ok_or_else(|| err(2, copy::lookup("pack.usage")))
}

pub(crate) fn select_adapter(
    args: &[String],
    root: &Path,
    installed: Option<&InstalledManifest>,
) -> Result<Adapter, CliResult> {
    resolve_adapter(requested_harness(args)?, root, installed)
}

/// An install owns one adapter tree, so every later verb acts on the harness
/// the ledger records. An explicit selector that disagrees is refused rather
/// than silently writing a second tree into the same ledger; with no ledger
/// yet, the host decides.
pub(crate) fn resolve_adapter(
    requested: Option<&str>,
    root: &Path,
    installed: Option<&InstalledManifest>,
) -> Result<Adapter, CliResult> {
    match (requested, installed) {
        (Some(name), Some(manifest)) if manifest.harness != name => Err(err(
            2,
            &copy::lookup("pack.err-harness-mismatch")
                .replace("{requested}", name)
                .replace("{installed}", &manifest.harness)
                .replace("{file}", MANIFEST_PATH),
        )),
        (Some(name), _) => adapter(name),
        (None, Some(manifest)) => adapter(&manifest.harness),
        (None, None) => Ok(detect_adapter(root)),
    }
}

/// The pack root this project's agent assets live under: the harness its
/// ownership ledger records, else the one its layout implies. Guidance the CLI
/// emits reads this, so a project is never told to load a tree that was never
/// written.
/// A ledger that cannot be read is treated as absent here, so guidance falls
/// back to what the project layout shows rather than to a fixed adapter. The
/// lifecycle verbs still refuse to act on an unreadable ledger; this decides
/// only which path the CLI names in prose.
pub(crate) fn project_pack_root(root: &Path) -> &'static str {
    let installed = super::pack_manifest::read_manifest(root).ok().flatten();
    resolve_adapter(None, root, installed.as_ref()).map_or_else(
        |_| detect_adapter(root).pack_root,
        |adapter| adapter.pack_root,
    )
}

/// Every ledger row must be a plain project-relative path under the adapter the
/// ledger names. A row outside it means the file was hand-edited into claiming
/// another tree, and acting on it would break one-install-one-adapter or write
/// outside the project.
pub(crate) fn ledger_matches_adapter(manifest: &InstalledManifest, adapter: Adapter) -> bool {
    manifest.files.iter().all(|file| {
        file.path.starts_with(adapter.pack_root)
            && Path::new(&file.path)
                .components()
                .all(|part| matches!(part, std::path::Component::Normal(_)))
    })
}

/// Look one harness name up in the adapter table.
fn adapter(name: &str) -> Result<Adapter, CliResult> {
    HARNESS_ROOTS
        .iter()
        .find(|(supported, _)| *supported == name)
        .map(|(name, pack_root)| Adapter { name, pack_root })
        .ok_or_else(|| {
            let supported: Vec<&str> = HARNESS_ROOTS.iter().map(|(name, _)| *name).collect();
            err(
                2,
                &copy::lookup("pack.err-unknown-harness")
                    .replace("{harness}", name)
                    .replace("{supported}", &supported.join(", ")),
            )
        })
}

/// Pick the adapter for a first install from what the target repository shows.
/// An OMP-native project directory means OMP; anything else, including a
/// project carrying both, installs the Claude adapter.
fn detect_adapter(root: &Path) -> Adapter {
    let has = |pack_root: &str| root.join(pack_root.trim_end_matches('/')).is_dir();
    if has(OMP_ROOT) && !has(CLAUDE_ROOT) {
        Adapter {
            name: "omp",
            pack_root: OMP_ROOT,
        }
    } else {
        Adapter {
            name: "claude",
            pack_root: CLAUDE_ROOT,
        }
    }
}
