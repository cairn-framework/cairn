//! Campaign resolution: turn an installed pack into an immutable set of bytes
//! for the duration of a campaign.
//!
//! The resolver reports bytes. It never selects work, starts a session, retries,
//! or interprets a terminal token (`dec.no-orchestrator`). Outside an active
//! campaign, pack drift stays information (`cairn pack status`); inside one, a
//! mismatch halts before any work runs (`dec.unified-cairn-dev-entry` clause 9).

// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use super::pack_assets::harness_root;

use super::pack_manifest::{InstalledManifest, MANIFEST_PATH, digest};
use std::fs;

/// The campaign snapshot: the exact bytes a campaign is pinned to.
pub(crate) const SNAPSHOT_PATH: &str = ".cairn/state/agent-pack-campaign.json";
/// Immutable copies of the pinned bytes. Prompt execution and procedure
/// loading read these, never the live pack, so an edit landing after
/// verification cannot reach a running campaign.
pub(crate) const PINNED_ROOT: &str = ".cairn/state/campaign";

/// Entry point of the router, which every mode resolves through. Pack-root
/// relative: the installed harness decides the root it hangs under.
const ROUTER: &str = "skills/cairn-dev/SKILL.md";
/// Loop mode: the normative procedure, and the declared closure's own head.
const LOOP_MODE: &str = "skills/cairn-dev/references/loop-mode.md";
/// Adapter-native transport for loop mode. Carries no procedure of its own.
const LOOP_COMMAND: &str = "commands/cairn-loop.md";

/// One resolved asset: the destination and the bytes actually on disk now.
#[derive(Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub(crate) struct ResolvedAsset {
    /// Destination relative to the project root.
    pub(crate) path: String,
    /// Lowercase hex SHA-256 of the bytes read during this resolution.
    pub(crate) sha256: String,
}

/// A complete resolution of one entry: the prompt plus its ordered closure.
#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub(crate) struct Resolution {
    /// `cairn-dev` entry that was resolved.
    pub(crate) entry: String,
    /// Harness whose adapter owns these destinations.
    pub(crate) harness: String,
    /// Bundle version recorded in the ownership ledger.
    pub(crate) bundle_version: String,
    /// CLI version recorded in the ownership ledger.
    pub(crate) cli_version: String,
    /// Digest of the ownership ledger itself, read before and after buffering.
    pub(crate) ledger_generation: String,
    /// Digest over the resolved bundle: every pinned path and its content hash,
    /// in closure order. One value an adapter can carry and compare.
    pub(crate) bundle_digest: String,
    /// Digest of the cairn binary that resolved these bytes. A same-version
    /// rebuild is still a different resolver, and a campaign must notice.
    pub(crate) cli_digest: String,
    /// Prompt the harness invokes for this entry.
    pub(crate) prompt: ResolvedAsset,
    /// The ordered required-asset closure, exactly as the entry declares it.
    pub(crate) closure: Vec<ResolvedAsset>,
}

/// Dispatches `cairn pack resolve` and `cairn pack campaign <verb>`.
pub(crate) fn run_resolve(
    root: &Path,
    requested: Option<&str>,
    with_loop: bool,
    json: bool,
) -> CliResult {
    match resolve(root, requested, with_loop) {
        Ok(buffered) => {
            if json {
                return ok(envelope(
                    "pack resolve",
                    &resolution_json(&buffered.resolution),
                ));
            }
            ok(render_resolution(&buffered.resolution))
        }
        Err(result) => result,
    }
}

/// Read the ledger once, buffer the complete ordered closure from disk, hash
/// those exact buffers, then reread the ledger. Any disagreement means the pack
/// changed under the resolution and it fails before any work. Everything the
/// caller may later write comes from these buffers, never from a second read of
/// the live pack.
pub(crate) fn resolve(
    root: &Path,
    requested: Option<&str>,
    with_loop: bool,
) -> Result<Buffered, CliResult> {
    let ledger = read_ledger(root)?;
    let generation_before = digest(&ledger);
    let manifest: InstalledManifest = serde_json::from_slice(&ledger).map_err(|error| {
        err(
            1,
            &copy::lookup("pack.err-manifest-parse")
                .replace("{file}", MANIFEST_PATH)
                .replace("{detail}", &error.to_string()),
        )
    })?;

    // The ledger read above is the authoritative one, and it is the read whose
    // generation is checked again below. An explicit selector is validated
    // against it, not only against whatever the dispatcher happened to see.
    if let Some(name) = requested
        && name != manifest.harness
    {
        return Err(err(
            2,
            &copy::lookup("pack.err-harness-mismatch")
                .replace("{requested}", name)
                .replace("{installed}", &manifest.harness)
                .replace("{file}", MANIFEST_PATH),
        ));
    }

    // The ledger records which adapter owns this install, and the adapter is a
    // pack root: resolve every declared path under it, never under a root the
    // caller guessed.
    let Some(pack_root) = harness_root(&manifest.harness) else {
        return Err(err(
            1,
            &copy::lookup("pack.err-unknown-harness")
                .replace("{harness}", &manifest.harness)
                .replace("{supported}", "claude, omp"),
        ));
    };
    let rooted = |path: &str| format!("{pack_root}{path}");
    let loop_mode = rooted(LOOP_MODE);

    let prompt = buffer(
        root,
        &rooted(if with_loop { LOOP_COMMAND } else { ROUTER }),
        &manifest,
    )?;
    let closure = if with_loop {
        // Validate loop mode itself before trusting the list it declares, and
        // parse that list out of the validated buffer: a drifted asset must not
        // be able to write itself out of its own closure.
        let head = buffer(root, &loop_mode, &manifest)?;
        let declared = declared_closure(&head.bytes, pack_root)?;
        if declared.iter().filter(|path| **path == loop_mode).count() != 1 {
            return Err(err(1, copy::lookup("pack.resolve-no-closure")));
        }
        let mut closure = Vec::with_capacity(declared.len());
        for path in declared {
            closure.push(if path == loop_mode {
                head.clone()
            } else {
                buffer(root, &path, &manifest)?
            });
        }
        closure
    } else {
        // The router is an index: it loads its references at the step that needs
        // one, so the entry itself is the whole pinned surface.
        Vec::new()
    };

    if generation_before != digest(&read_ledger(root)?) {
        return Err(err(1, copy::lookup("pack.resolve-concurrent")));
    }

    let resolution = Resolution {
        entry: if with_loop { "loop" } else { "router" }.to_owned(),
        harness: manifest.harness,
        bundle_version: manifest.bundle_version,
        cli_version: manifest.cli_version,
        ledger_generation: generation_before,
        bundle_digest: bundle_digest(&prompt, &closure),
        cli_digest: cli_digest()?,
        prompt: prompt.asset.clone(),
        closure: closure.iter().map(|held| held.asset.clone()).collect(),
    };
    Ok(Buffered {
        resolution,
        prompt,
        closure,
    })
}

/// A resolution plus the bytes it was computed from.
pub(crate) struct Buffered {
    /// What the resolution reports.
    pub(crate) resolution: Resolution,
    /// The prompt bytes, held for pinning.
    pub(crate) prompt: HeldAsset,
    /// The closure bytes, in declared order, held for pinning.
    pub(crate) closure: Vec<HeldAsset>,
}

/// One buffered asset: what it resolved to, and the exact bytes behind it.
#[derive(Clone)]
pub(crate) struct HeldAsset {
    /// Path and hash as reported.
    pub(crate) asset: ResolvedAsset,
    /// The bytes those values were computed from.
    pub(crate) bytes: Vec<u8>,
}

/// Fold the resolved bytes into one value: order and content both matter, so a
/// reordered closure is a different bundle even with identical members.
fn bundle_digest(prompt: &HeldAsset, closure: &[HeldAsset]) -> String {
    let mut material = format!("{} {}\n", prompt.asset.path, prompt.asset.sha256);
    for held in closure {
        let _ = writeln!(material, "{} {}", held.asset.path, held.asset.sha256);
    }
    digest(material.as_bytes())
}

/// Hash the running binary. Unreadable means the resolution cannot state what
/// produced it, which fails before work rather than pinning an unknown.
fn cli_digest() -> Result<String, CliResult> {
    let bytes = std::env::current_exe()
        .and_then(fs::read)
        .map_err(|error| {
            err(
                1,
                &copy::lookup("pack.resolve-unreadable").replace("{file}", &error.to_string()),
            )
        })?;
    Ok(digest(&bytes))
}

/// Read the ledger. Its own bytes are the generation: any install, update,
/// uninstall, or hand edit changes them, and an interrupted write never
/// publishes.
fn read_ledger(root: &Path) -> Result<Vec<u8>, CliResult> {
    match fs::read(root.join(MANIFEST_PATH)) {
        Ok(bytes) => Ok(bytes),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Err(err(
            1,
            &copy::lookup("pack.not-installed").replace("{file}", MANIFEST_PATH),
        )),
        Err(error) => Err(err(
            1,
            &copy::lookup("pack.err-manifest-parse")
                .replace("{file}", MANIFEST_PATH)
                .replace("{detail}", &error.to_string()),
        )),
    }
}

/// Read one asset and require the ledger to own it at exactly these bytes.
/// A missing, unreadable, unowned, or edited asset fails the resolution.
fn buffer(root: &Path, path: &str, manifest: &InstalledManifest) -> Result<HeldAsset, CliResult> {
    let Ok(bytes) = fs::read(root.join(path)) else {
        return Err(err(
            1,
            &copy::lookup("pack.resolve-unreadable").replace("{file}", path),
        ));
    };
    let sha256 = digest(&bytes);
    let Some(owned) = manifest
        .files
        .iter()
        .find(|file| file.path == path)
        .map(|file| file.sha256.as_str())
    else {
        return Err(err(
            1,
            &copy::lookup("pack.resolve-unowned").replace("{file}", path),
        ));
    };
    if owned != sha256 {
        return Err(err(
            1,
            &copy::lookup("pack.resolve-drifted").replace("{file}", path),
        ));
    }
    Ok(HeldAsset {
        asset: ResolvedAsset {
            path: path.to_owned(),
            sha256,
        },
        bytes,
    })
}

/// Parse the ordered closure loop mode declares in its fenced `text` block.
/// Adapters and campaign locks consume that list and no other, so the resolver
/// reads it from the installed asset rather than carrying a second copy.
fn declared_closure(loop_mode: &[u8], pack_root: &str) -> Result<Vec<String>, CliResult> {
    let Ok(body) = std::str::from_utf8(loop_mode) else {
        return Err(err(
            1,
            &copy::lookup("pack.resolve-unreadable").replace("{file}", LOOP_MODE),
        ));
    };
    let heading = body
        .find("## Required asset closure")
        .ok_or_else(|| err(1, copy::lookup("pack.resolve-no-closure")))?;
    let after = &body[heading..];
    let open = after
        .find("```text")
        .ok_or_else(|| err(1, copy::lookup("pack.resolve-no-closure")))?
        + "```text".len();
    let rest = &after[open..];
    let close = rest
        .find("```")
        .ok_or_else(|| err(1, copy::lookup("pack.resolve-no-closure")))?;
    let closure: Vec<String> = rest[..close]
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| format!("{pack_root}{line}"))
        .collect();
    // A declared row is a project-relative pack destination and nothing else:
    // no traversal, no absolute path, no escape from the pack root.
    if closure.is_empty()
        || closure.iter().any(|path| {
            Path::new(path)
                .components()
                .any(|part| !matches!(part, std::path::Component::Normal(_)))
        })
    {
        return Err(err(1, copy::lookup("pack.resolve-no-closure")));
    }
    Ok(closure)
}

pub(crate) fn render_resolution(resolution: &Resolution) -> String {
    let mut out = copy::lookup("pack.resolve-header")
        .replace("{entry}", &resolution.entry)
        .replace("{harness}", &resolution.harness)
        .replace("{bundle}", &resolution.bundle_version)
        .replace("{cli}", &resolution.cli_version);
    out.push('\n');
    let _ = writeln!(out, "  bundle: {}", resolution.bundle_digest);
    let _ = writeln!(out, "  cli: {}", resolution.cli_digest);
    let _ = writeln!(
        out,
        "  prompt: {} {}",
        resolution.prompt.path, resolution.prompt.sha256
    );
    for asset in &resolution.closure {
        let _ = writeln!(out, "  closure: {} {}", asset.path, asset.sha256);
    }
    out
}

fn resolution_json(resolution: &Resolution) -> serde_json::Value {
    serde_json::json!({
        "entry": resolution.entry,
        "harness": resolution.harness,
        "bundle_version": resolution.bundle_version,
        "cli_version": resolution.cli_version,
        "ledger_generation": resolution.ledger_generation,
        "bundle_digest": resolution.bundle_digest,
        "cli_digest": resolution.cli_digest,
        "prompt": {"path": resolution.prompt.path, "sha256": resolution.prompt.sha256},
        "closure": resolution
            .closure
            .iter()
            .map(|asset| serde_json::json!({"path": asset.path, "sha256": asset.sha256}))
            .collect::<Vec<_>>(),
    })
}

/// Where one campaign's immutable copies live. The bundle digest is in the
/// path, so a later campaign can never substitute different bytes at a path a
/// running session already verified.
pub(crate) fn pinned_root(resolution: &Resolution) -> String {
    format!("{PINNED_ROOT}/{}", resolution.bundle_digest)
}

/// Campaign payloads add where the pinned copies live: a session loads from
/// there, not from the live pack paths.
pub(crate) fn campaign_json(resolution: &Resolution) -> serde_json::Value {
    let mut payload = resolution_json(resolution);
    if let Some(object) = payload.as_object_mut() {
        object.insert(
            "pinned_root".to_owned(),
            serde_json::Value::String(pinned_root(resolution)),
        );
    }
    payload
}

/// Wrap a payload in the CLI's `{command, status, data}` envelope.
pub(crate) fn envelope(command: &str, data: &serde_json::Value) -> String {
    let body = serde_json::json!({"command": command, "status": "ok", "data": data});
    format!("{body}\n")
}
