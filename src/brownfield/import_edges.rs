//! Edge derivation from observed imports for brownfield discovery.
//!
//! Consumes the resolved references `super::imports` extracts and maps
//! them onto co-discovered candidates to emit directed, code-evidenced
//! dependency edges. External dependencies never match: a reference only
//! becomes an edge when it provably resolves inside the repository.

use std::collections::BTreeMap;
use std::path::Path;

use super::discovery::{DiscoveredCandidate, DiscoveredEdge};
use super::imports::{ImportRef, extract_imports};

/// Populate `candidates[i].edges` from imports observed in evidence files.
///
/// Path references match a candidate exactly or by directory prefix, most
/// specific (longest) candidate path first. Suffix references match the
/// candidate whose full path is the longest suffix of the segments. Edge
/// confidence scales with the number of observed imports.
pub(super) fn derive_import_edges(root: &Path, candidates: &mut [DiscoveredCandidate]) {
    let paths: Vec<String> = candidates
        .iter()
        .map(|c| normalise_separators(&c.path))
        .collect();
    let mut counts: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    for (i, candidate) in candidates.iter().enumerate() {
        for file in &candidate.evidence {
            let Ok(source) = std::fs::read_to_string(root.join(file)) else {
                continue;
            };
            let file = normalise_separators(file);
            for import in extract_imports(Path::new(&file), &source) {
                if let Some(j) = match_candidate(&import, &paths)
                    && j != i
                {
                    *counts.entry((i, j)).or_default() += 1;
                }
            }
        }
    }
    for ((i, j), count) in counts {
        let target = candidates[j].id.clone();
        let name = candidates[j].name.clone();
        candidates[i].edges.push(DiscoveredEdge {
            target,
            description: format!("Observed imports of {name} ({count} in code)"),
            confidence: edge_confidence(count),
        });
    }
}

/// Resolve one import reference to a candidate index, or `None` when the
/// reference is external or resolves outside every candidate.
fn match_candidate(import: &ImportRef, paths: &[String]) -> Option<usize> {
    match import {
        ImportRef::Path(resolved) => paths
            .iter()
            .enumerate()
            .filter(|(_, p)| *resolved == **p || resolved.starts_with(&format!("{p}/")))
            .max_by_key(|(_, p)| p.len())
            .map(|(idx, _)| idx),
        ImportRef::Suffix(segments) => paths
            .iter()
            .enumerate()
            .filter(|(_, p)| {
                let comps: Vec<&str> = p.split('/').collect();
                comps.len() <= segments.len()
                    && segments[segments.len() - comps.len()..]
                        .iter()
                        .zip(&comps)
                        .all(|(s, c)| s == c)
            })
            .max_by_key(|(_, p)| p.len())
            .map(|(idx, _)| idx),
    }
}

/// Windows evidence and candidate paths carry backslashes; comparisons are
/// slash-normalised throughout.
fn normalise_separators(path: &str) -> String {
    path.replace('\\', "/")
}

fn edge_confidence(count: usize) -> f64 {
    if count >= 5 {
        0.95
    } else if count >= 2 {
        0.8
    } else {
        0.6
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_match_prefers_most_specific_candidate() {
        let paths = vec!["src".to_owned(), "src/auth".to_owned()];
        assert_eq!(
            match_candidate(&ImportRef::Path("src/auth/session".to_owned()), &paths),
            Some(1)
        );
    }

    #[test]
    fn suffix_match_uses_full_candidate_path() {
        let paths = vec!["services/a/auth".to_owned(), "services/b/auth".to_owned()];
        assert_eq!(
            match_candidate(
                &ImportRef::Suffix(["app", "services", "a", "auth"].map(str::to_owned).to_vec()),
                &paths
            ),
            Some(0)
        );
    }

    #[test]
    fn external_paths_match_nothing() {
        let paths = vec!["src/auth".to_owned()];
        assert_eq!(
            match_candidate(&ImportRef::Path("requests/auth".to_owned()), &paths),
            None
        );
    }

    #[test]
    fn confidence_scales_with_count() {
        assert!((edge_confidence(1) - 0.6).abs() < f64::EPSILON);
        assert!((edge_confidence(3) - 0.8).abs() < f64::EPSILON);
        assert!((edge_confidence(7) - 0.95).abs() < f64::EPSILON);
    }

    #[test]
    fn backslash_paths_normalise() {
        assert_eq!(normalise_separators("src\\auth"), "src/auth");
    }
}
