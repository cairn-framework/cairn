//! Summariser integration for brownfield candidates.
//!
//! Bridges `DiscoveredCandidate` from the discovery engine to
//! `SummariserRequest` consumed by the Phase 8 summariser backend.
//! Bounds code samples per task 2.1: at most five files per candidate
//! and 4,000 bytes per file by default.

use std::{path::Path, time::Duration};

use crate::summariser::{CodeSample, SummariserBackend, SummariserRequest};

use super::discovery::DiscoveredCandidate;

/// Maximum source files collected per candidate for summariser input.
pub const MAX_FILES_PER_CANDIDATE: usize = 5;

/// Maximum bytes read from each source file for summariser input.
pub const MAX_BYTES_PER_FILE: usize = 4000;

/// Supported source file extensions for code-sample collection.
const SOURCE_EXTS: &[&str] = &["rs", "ts", "js", "py", "go"];

/// Build a bounded `SummariserRequest` from a discovered candidate.
///
/// Collects up to `max_files` source files from the candidate's directory,
/// truncating each to `max_bytes_per_file`. Populates `map_facts` with
/// candidate metadata (name, description, tags, confidence, evidence).
///
/// # Panics
///
/// Never panics.
#[must_use]
pub fn build_request(
    candidate: &DiscoveredCandidate,
    root: &Path,
    max_files: usize,
    max_bytes_per_file: usize,
) -> SummariserRequest {
    let code_samples = collect_code_samples(candidate, root, max_files, max_bytes_per_file);
    let map_facts = build_map_facts(candidate);

    SummariserRequest {
        schema_version: crate::summariser::SUMMARISER_SCHEMA_VERSION,
        request_id: format!("bf-{}", candidate.id),
        draft_type: "contract".to_owned(),
        target_node: candidate.id.clone(),
        map_facts,
        contract_excerpt: None,
        interface_findings: Vec::new(),
        docstring_findings: Vec::new(),
        project_context: String::new(),
        rules: Vec::new(),
        code_samples,
    }
}

/// Enriched candidate metadata produced by the summariser backend.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnrichedCandidate {
    /// Human-readable name.
    pub name: String,
    /// Human-readable description.
    pub description: String,
    /// Suggested tags.
    pub tags: Vec<String>,
    /// Generated stub contract prose.
    pub contract_prose: String,
}

/// Invoke the summariser backend for a candidate and return enriched metadata.
///
/// Builds a bounded request from the candidate, invokes the backend, and
/// extracts `contract_prose` from the response `draft_text`.  If the backend
/// returns an error, every field falls back to the original candidate values
/// (path-derived name and description, existing tags, and the built-in stub
/// contract).
///
/// # Panics
///
/// Never panics.
pub fn enrich_candidate(
    backend: &dyn SummariserBackend,
    candidate: &DiscoveredCandidate,
    root: &Path,
    timeout: Duration,
) -> EnrichedCandidate {
    let request = build_request(candidate, root, MAX_FILES_PER_CANDIDATE, MAX_BYTES_PER_FILE);

    let Ok(response) = backend.invoke(&request, timeout) else {
        return EnrichedCandidate {
            name: candidate.name.clone(),
            description: candidate.description.clone(),
            tags: candidate.tags.clone(),
            contract_prose: super::stub_contract(candidate),
        };
    };

    // Attempt to extract structured fields from optional metadata.
    let name = extract_string_from_metadata(response.metadata.as_ref(), "name")
        .unwrap_or_else(|| candidate.name.clone());
    let description = extract_string_from_metadata(response.metadata.as_ref(), "description")
        .unwrap_or_else(|| candidate.description.clone());
    let tags = extract_string_array_from_metadata(response.metadata.as_ref(), "tags")
        .unwrap_or_else(|| candidate.tags.clone());

    EnrichedCandidate {
        name,
        description,
        tags,
        contract_prose: response.draft_text,
    }
}

/// Extract a string field from response metadata, if present.
fn extract_string_from_metadata(metadata: Option<&serde_json::Value>, key: &str) -> Option<String> {
    let value = metadata?;
    value
        .get(key)?
        .as_str()
        .map(std::string::ToString::to_string)
}

/// Extract a string array from response metadata, if present.
fn extract_string_array_from_metadata(
    metadata: Option<&serde_json::Value>,
    key: &str,
) -> Option<Vec<String>> {
    let value = metadata?;
    let array = value.get(key)?.as_array()?;
    let mut out = Vec::new();
    for item in array {
        if let Some(s) = item.as_str() {
            out.push(s.to_owned());
        }
    }
    Some(out)
}

/// Collect bounded code samples from the candidate's directory.
fn collect_code_samples(
    candidate: &DiscoveredCandidate,
    root: &Path,
    max_files: usize,
    max_bytes_per_file: usize,
) -> Vec<CodeSample> {
    let dir = root.join(&candidate.path);
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };

    let mut paths: Vec<std::path::PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| is_source_file(p))
        .collect();
    paths.sort();

    let mut samples = Vec::new();
    for path in paths {
        if samples.len() >= max_files {
            break;
        }
        let rel = match path.strip_prefix(root) {
            Ok(p) => p.to_string_lossy().into_owned(),
            Err(_) => continue,
        };
        let Ok(content) = std::fs::read(&path) else {
            continue;
        };
        let Ok(text) = String::from_utf8(content) else {
            continue;
        };
        let truncated = if text.len() > max_bytes_per_file {
            let mut end = max_bytes_per_file;
            while !text.is_char_boundary(end) && end > 0 {
                end -= 1;
            }
            text[..end].to_owned()
        } else {
            text
        };
        samples.push(CodeSample {
            path: rel,
            content: truncated,
        });
    }

    samples
}

/// Check whether a path has a supported source-file extension.
fn is_source_file(path: &Path) -> bool {
    let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
        return false;
    };
    SOURCE_EXTS.contains(&ext)
}

/// Build map-fact strings from candidate metadata.
fn build_map_facts(candidate: &DiscoveredCandidate) -> Vec<String> {
    let mut facts = Vec::new();
    if !candidate.name.is_empty() {
        facts.push(format!("name: {}", candidate.name));
    }
    if !candidate.description.is_empty() {
        facts.push(format!("description: {}", candidate.description));
    }
    if !candidate.tags.is_empty() {
        facts.push(format!("tags: {}", candidate.tags.join(", ")));
    }
    facts.push(format!("confidence: {}", candidate.confidence));
    if !candidate.evidence.is_empty() {
        facts.push(format!("evidence: {}", candidate.evidence.join(", ")));
    }
    facts
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir_with_files(name: &str, files: &[(String, String)]) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(name);
        std::fs::create_dir_all(&root).unwrap();
        for (rel, content) in files {
            let path = root.join(rel);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            std::fs::write(&path, content).unwrap();
        }
        root
    }

    #[test]
    fn test_build_request_populates_target_and_draft_type() {
        let root = temp_dir_with_files("bf-summ-1", &[]);
        let candidate = DiscoveredCandidate {
            id: "app.core".to_owned(),
            name: "core".to_owned(),
            description: "Core module".to_owned(),
            path: "src/core".to_owned(),
            tags: vec![],
            confidence: 0.8,
            evidence: vec![],
            edges: vec![],
        };
        let req = build_request(&candidate, &root, 5, 4000);
        assert_eq!(req.target_node, "app.core");
        assert_eq!(req.draft_type, "contract");
        assert_eq!(req.schema_version, 1);
    }

    #[test]
    fn test_collect_code_samples_bounds_file_count() {
        let mut files = Vec::new();
        for i in 0..7 {
            files.push((format!("src/many/file{i}.rs"), format!("fn f{i}() {{}}\n")));
        }
        let root = temp_dir_with_files("bf-summ-2", &files);
        let candidate = DiscoveredCandidate {
            id: "src.many".to_owned(),
            name: "many".to_owned(),
            description: "Many files".to_owned(),
            path: "src/many".to_owned(),
            tags: vec![],
            confidence: 0.5,
            evidence: vec![],
            edges: vec![],
        };
        let samples = collect_code_samples(&candidate, &root, 5, 4000);
        assert_eq!(samples.len(), 5);
    }

    #[test]
    fn test_collect_code_samples_truncates_large_files() {
        let big = "a".repeat(6000);
        let files = vec![("src/big/large.rs".to_owned(), big)];
        let root = temp_dir_with_files("bf-summ-3", &files);
        let candidate = DiscoveredCandidate {
            id: "src.big".to_owned(),
            name: "big".to_owned(),
            description: "Big file".to_owned(),
            path: "src/big".to_owned(),
            tags: vec![],
            confidence: 0.5,
            evidence: vec![],
            edges: vec![],
        };
        let samples = collect_code_samples(&candidate, &root, 5, 4000);
        assert_eq!(samples.len(), 1);
        assert!(samples[0].content.len() <= 4000);
    }

    #[test]
    fn test_collect_code_samples_skips_non_source_files() {
        let files = vec![
            ("src/mixed/code.rs".to_owned(), "fn main() {}".to_owned()),
            ("src/mixed/readme.md".to_owned(), "# Readme".to_owned()),
            ("src/mixed/notes.txt".to_owned(), "notes".to_owned()),
        ];
        let root = temp_dir_with_files("bf-summ-4", &files);
        let candidate = DiscoveredCandidate {
            id: "src.mixed".to_owned(),
            name: "mixed".to_owned(),
            description: "Mixed files".to_owned(),
            path: "src/mixed".to_owned(),
            tags: vec![],
            confidence: 0.5,
            evidence: vec![],
            edges: vec![],
        };
        let samples = collect_code_samples(&candidate, &root, 5, 4000);
        assert_eq!(samples.len(), 1);
        assert_eq!(samples[0].path, "src/mixed/code.rs");
    }

    #[test]
    fn test_build_map_facts_omits_empty_fields() {
        let candidate = DiscoveredCandidate {
            id: "a.b".to_owned(),
            name: String::new(),
            description: String::new(),
            path: "a/b".to_owned(),
            tags: vec![],
            confidence: 0.5,
            evidence: vec![],
            edges: vec![],
        };
        let facts = build_map_facts(&candidate);
        assert_eq!(facts.len(), 1);
        assert_eq!(facts.len(), 1);
        assert!(facts[0].starts_with("confidence:"));
    }

    #[test]
    fn test_enrich_candidate_uses_backend_draft_text() {
        use crate::summariser::{FakeBackend, SummariserResponse};

        let root = temp_dir_with_files("bf-enrich-ok", &[]);
        let candidate = DiscoveredCandidate {
            id: "app.core".to_owned(),
            name: "core".to_owned(),
            description: "Core module".to_owned(),
            path: "src/core".to_owned(),
            tags: vec!["tag1".to_owned()],
            confidence: 0.8,
            evidence: vec![],
            edges: vec![],
        };
        let backend = FakeBackend::ok(SummariserResponse {
            schema_version: 1,
            draft_text: "# Enriched".to_owned(),
            summary: None,
            metadata: None,
        });

        let enriched = enrich_candidate(&backend, &candidate, &root, Duration::from_secs(1));

        assert_eq!(enriched.contract_prose, "# Enriched");
        assert_eq!(enriched.name, "core");
        assert_eq!(enriched.tags, vec!["tag1"]);
    }

    #[test]
    fn test_enrich_candidate_extracts_metadata_fields() {
        use crate::summariser::{FakeBackend, SummariserResponse};

        let root = temp_dir_with_files("bf-enrich-meta", &[]);
        let candidate = DiscoveredCandidate {
            id: "app.auth".to_owned(),
            name: "auth".to_owned(),
            description: "Old desc".to_owned(),
            path: "src/auth".to_owned(),
            tags: vec![],
            confidence: 0.8,
            evidence: vec![],
            edges: vec![],
        };
        let backend = FakeBackend::ok(SummariserResponse {
            schema_version: 1,
            draft_text: "contract".to_owned(),
            summary: None,
            metadata: Some(serde_json::json!({
                "name": "Authentication",
                "description": "Auth module",
                "tags": ["security", "identity"]
            })),
        });

        let enriched = enrich_candidate(&backend, &candidate, &root, Duration::from_secs(1));

        assert_eq!(enriched.name, "Authentication");
        assert_eq!(enriched.description, "Auth module");
        assert_eq!(enriched.tags, vec!["security", "identity"]);
    }

    #[test]
    fn test_enrich_candidate_falls_back_on_backend_error() {
        use crate::summariser::{FakeBackend, SummariserBackendError};

        let root = temp_dir_with_files("bf-enrich-err", &[]);
        let candidate = DiscoveredCandidate {
            id: "app.fail".to_owned(),
            name: "fail".to_owned(),
            description: "Fail module".to_owned(),
            path: "src/fail".to_owned(),
            tags: vec!["a".to_owned()],
            confidence: 0.5,
            evidence: vec![],
            edges: vec![],
        };
        let backend = FakeBackend::err(SummariserBackendError::Disabled);

        let enriched = enrich_candidate(&backend, &candidate, &root, Duration::from_secs(1));

        assert_eq!(enriched.name, "fail");
        assert_eq!(enriched.description, "Fail module");
        assert_eq!(enriched.tags, vec!["a"]);
        assert_eq!(
            enriched.contract_prose,
            crate::brownfield::stub_contract(&candidate)
        );
    }
}
