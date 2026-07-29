//! Exhaustive folder-claim validation for decision artefacts.

use std::collections::BTreeSet;
use std::path::Path;

use super::{ArtefactSet, Graph};
use crate::map::graph::{Finding, FindingSeverity};

/// Emits `CA003` for every decision whose exhaustive folder claim does not
/// match the folder's actual file contents, or whose claimed folder is
/// missing or unreadable.
pub(crate) fn check_claims(graph: &mut Graph, artefacts: &ArtefactSet, root: &Path) {
    for decision in &artefacts.decisions {
        let Some(claims) = &decision.claims else {
            continue;
        };
        if !matches!(claims.mode, crate::artefacts::ClaimsMode::Exhaustive) {
            continue;
        }
        let folder = root.join(&claims.folder);
        let actual: BTreeSet<String> = if let Ok(entries) = std::fs::read_dir(&folder) {
            entries
                .flatten()
                .filter(|e| e.file_type().is_ok_and(|ft| ft.is_file()))
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .collect()
        } else {
            graph.findings.push(claim_finding(
                decision,
                format!(
                    "decision `{}` claims exhaustive file list for folder `{}` which does not exist or is unreadable",
                    decision.id, claims.folder
                ),
            ));
            continue;
        };
        let claimed: BTreeSet<String> = claims.items.iter().cloned().collect();
        let missing: Vec<_> = actual.difference(&claimed).cloned().collect();
        let extra: Vec<_> = claimed.difference(&actual).cloned().collect();
        if !missing.is_empty() || !extra.is_empty() {
            let mut parts = Vec::new();
            if !missing.is_empty() {
                parts.push(format!("missing from claim: {}", missing.join(", ")));
            }
            if !extra.is_empty() {
                parts.push(format!("extra in claim: {}", extra.join(", ")));
            }
            graph.findings.push(claim_finding(
                decision,
                format!(
                    "decision `{}` exhaustive file claim for `{}` does not match actual contents: {}",
                    decision.id,
                    claims.folder,
                    parts.join("; ")
                ),
            ));
        }
    }
}

/// `CA003` constructor anchored on the claiming decision.
fn claim_finding(decision: &crate::artefacts::registry::Decision, message: String) -> Finding {
    Finding {
        code: "CA003".to_owned(),
        severity: FindingSeverity::Error,
        message,
        node: Some(decision.nodes.first().cloned().unwrap_or_default()),
        target: None,
        path: Some(decision.path.clone()),
        deferred_by: None,
        parked_by: None,
    }
}
