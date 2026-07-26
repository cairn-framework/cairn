//! Rendering for the `cairn pack` verbs: the human report, the never-overwritten
//! list, and the JSON envelopes. Split from `pack.rs` so the lifecycle engine
//! and the way it is printed stay separately readable.

// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use super::pack::ApplyReport;
use super::pack_manifest::{BUNDLE_VERSION, InstalledManifest};

/// Append the never-overwritten list, if there is one.
pub(super) fn append_modified(out: &mut String, modified: &[String]) {
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
pub(super) fn envelope(command: &str, data: &serde_json::Value) -> String {
    let body = serde_json::json!({"command": command, "status": "ok", "data": data});
    format!("{body}\n")
}

pub(super) fn apply_json(harness: &str, report: &ApplyReport) -> String {
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

pub(super) fn status_json(
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
