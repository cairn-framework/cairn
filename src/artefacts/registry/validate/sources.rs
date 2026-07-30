//! Source-record validation: citation reachability, per-verification-mode
//! integrity (`verified` hashes, `external` URLs, `tracked` resolution and
//! containment), and index consistency.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path};

use super::super::io::{error, info, is_url, warning};
use super::super::sha256::sha256_hex;
use super::super::types::{ArtefactSet, Source, SourceVerification};

pub(super) fn validate_sources(root: &Path, source_ids: &BTreeSet<String>, set: &mut ArtefactSet) {
    let used_sources = set
        .research
        .iter()
        .flat_map(|item| item.sources.iter().cloned())
        .chain(
            set.decisions
                .iter()
                .flat_map(|item| item.informed_by.iter().cloned()),
        )
        .collect::<BTreeSet<_>>();
    let source_records = set.sources.clone();
    for source in &source_records {
        if !used_sources.contains(&source.id) {
            set.findings.push(warning(
                "CAIRN_SOURCE_ORPHAN",
                format!("source `{}` is not referenced", source.id),
                None,
                Some(source.path.clone()),
            ));
        }
        match source.verification {
            SourceVerification::Verified => validate_verified_source(root, source, set),
            SourceVerification::External => {
                if !is_url(&source.file) {
                    set.findings.push(error(
                        "CAIRN_SOURCE_EXTERNAL_URL",
                        format!("external source `{}` file is not a URL", source.id),
                        None,
                        Some(source.path.clone()),
                    ));
                }
            }
            SourceVerification::Unverified => set.findings.push(info(
                "CAIRN_SOURCE_UNVERIFIED",
                format!("source `{}` is unverified", source.id),
                None,
                Some(source.path.clone()),
            )),
            SourceVerification::Tracked => validate_tracked_source(root, source, set),
        }
    }
    for source in source_ids {
        if !set.sources.iter().any(|item| &item.id == source) {
            set.findings.push(warning(
                "CAIRN_SOURCE_INDEX_GAP",
                format!("source `{source}` is indexed but missing"),
                None,
                None,
            ));
        }
    }
}

pub(super) fn validate_verified_source(root: &Path, source: &Source, set: &mut ArtefactSet) {
    let Some(expected) = &source.sha256 else {
        set.findings.push(error(
            "CAIRN_SOURCE_SHA256_MISSING",
            format!("verified source `{}` lacks sha256", source.id),
            None,
            Some(source.path.clone()),
        ));
        return;
    };
    match fs::read(root.join(&source.file)) {
        Ok(bytes) => {
            let actual = sha256_hex(&bytes);
            if &actual != expected {
                set.findings.push(error(
                    "CAIRN_SOURCE_SHA256_MISMATCH",
                    format!("verified source `{}` sha256 mismatch", source.id),
                    None,
                    Some(source.path.clone()),
                ));
            }
        }
        Err(read_error) => set.findings.push(error(
            "CAIRN_SOURCE_READ_FAILED",
            format!(
                "failed to read verified source `{}`: {read_error}",
                source.id
            ),
            None,
            Some(source.path.clone()),
        )),
    }
}

/// Validates a `tracked` source: a live in-repo path read as it stands
/// (`dec.source-tracked-verification`). The path must be relative (an
/// optional leading `./` plus at least one normal component), resolve via a
/// metadata probe rather than a byte read (a tracked source may cite a
/// directory), and canonicalise to a location under the repository root so a
/// symlink cannot leave the tree. A declared `sha256` is rejected: an author
/// who pinned a hash asked for `verified`.
pub(super) fn validate_tracked_source(root: &Path, source: &Source, set: &mut ArtefactSet) {
    if source.sha256.is_some() {
        set.findings.push(error(
            "CAIRN_SOURCE_SHA256_UNEXPECTED",
            format!("tracked source `{}` declares sha256", source.id),
            None,
            Some(source.path.clone()),
        ));
        return;
    }
    let Some(issue) = tracked_path_issue(root, Path::new(&source.file)) else {
        return;
    };
    let reason = match issue {
        TrackedPathIssue::NotRelative => "is not a relative path into the repository".to_owned(),
        TrackedPathIssue::Escapes => "resolves outside the repository root".to_owned(),
        TrackedPathIssue::Unresolved(probe_error) => format!("does not resolve: {probe_error}"),
    };
    set.findings.push(error(
        "CAIRN_SOURCE_READ_FAILED",
        format!(
            "tracked source `{}` file `{}` {reason}",
            source.id, source.file
        ),
        None,
        Some(source.path.clone()),
    ));
}

/// Why a tracked source's `file:` is not a resolving in-repo path.
enum TrackedPathIssue {
    /// Absolute, `..`-traversing, or a bare `./`: rejected before any
    /// filesystem probe.
    NotRelative,
    /// The canonical path left the repository root.
    Escapes,
    /// Canonicalisation failed: the path is missing or unreadable.
    Unresolved(std::io::Error),
}

/// Metadata probe for a tracked path: lexical shape (optional leading `./`,
/// then normal components only), then canonical resolution and containment
/// under the canonical root. `None` means the path is valid.
fn tracked_path_issue(root: &Path, file: &Path) -> Option<TrackedPathIssue> {
    let mut components = file.components();
    let mut first = components.next();
    if first == Some(Component::CurDir) {
        first = components.next();
    }
    let relative = matches!(first, Some(Component::Normal(_)))
        && components.all(|component| matches!(component, Component::Normal(_)));
    if !relative {
        return Some(TrackedPathIssue::NotRelative);
    }
    let resolved = root.canonicalize().and_then(|canonical_root| {
        root.join(file)
            .canonicalize()
            .map(|canonical| (canonical_root, canonical))
    });
    match resolved {
        Ok((canonical_root, canonical)) if canonical.starts_with(&canonical_root) => None,
        Ok(_) => Some(TrackedPathIssue::Escapes),
        Err(probe_error) => Some(TrackedPathIssue::Unresolved(probe_error)),
    }
}
