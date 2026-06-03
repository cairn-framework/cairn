//! Typed request/response shapes for the summariser local-command protocol.
//!
//! Wire schema aligned with phase-8 design.md:
//!   - `SummariserRequest` carries `schema_version`, `request_id`,
//!     `draft_type`, `target_node`, `map_facts`, `contract_excerpt`,
//!     `interface_findings`, `docstring_findings`, `project_context`,
//!     `rules`, and `code_samples`.
//!   - `SummariserResponse` carries `schema_version`, `draft_text`
//!     (primary generated prose), optional `summary`, and optional
//!     `metadata`.
//!
//! Producers and consumers share these struct definitions so drift
//! fails at parse time when `deny_unknown_fields` catches unrecognised
//! keys.

use serde::{Deserialize, Serialize};

/// Wire schema version constant.
pub const SUMMARISER_SCHEMA_VERSION: u32 = 1;

/// One bounded code sample included in the prompt.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodeSample {
    /// File path relative to project root.
    pub path: String,
    /// File content (already truncated to `max_sample_bytes_per_file`).
    pub content: String,
}

/// Local-command request envelope. Producers (cairn) send exactly one of
/// these JSON objects on the configured backend's stdin, ending with a
/// newline.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SummariserRequest {
    /// Wire schema version.
    pub schema_version: u32,
    /// Stable request identifier (correlates logs and drafts).
    pub request_id: String,
    /// Stable artefact-type discriminator. Currently always "contract".
    pub draft_type: String,
    /// Target node ID under summarisation.
    pub target_node: String,
    /// Facts drawn from the map graph (neighbourhood, ownership, etc.).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub map_facts: Vec<String>,
    /// Existing contract body excerpt, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_excerpt: Option<String>,
    /// Detected interface contradictions / findings.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interface_findings: Vec<String>,
    /// Docstring-related findings.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub docstring_findings: Vec<String>,
    /// Human-readable project context (description, scope, conventions).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub project_context: String,
    /// Rules or constraints the summariser must obey.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<String>,
    /// Bounded code samples from the repository.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code_samples: Vec<CodeSample>,
}

/// Local-command response envelope. Backends emit exactly one of these
/// JSON objects on stdout, ending with a newline. Cairn stores only
/// `draft_text` as generated prose; `summary` and `metadata` are
/// captured for audit but do not affect contract bodies.
/// `deny_unknown_fields` ensures producer drift fails loudly.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SummariserResponse {
    /// Wire schema version.
    pub schema_version: u32,
    /// Generated draft text for the artefact body (canonical).
    pub draft_text: String,
    /// Optional human-readable summary of the draft.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Optional structured audit metadata (token counts, model id, etc.).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_request() -> SummariserRequest {
        SummariserRequest {
            schema_version: SUMMARISER_SCHEMA_VERSION,
            request_id: "req-001".to_owned(),
            draft_type: "contract".to_owned(),
            target_node: "app.auth".to_owned(),
            map_facts: vec!["owned_by: app".to_owned()],
            contract_excerpt: Some("# Auth\nReturns user".to_owned()),
            interface_findings: vec!["interface drift detected".to_owned()],
            docstring_findings: vec!["missing doc on fn login".to_owned()],
            project_context: "Auth service handles user login".to_owned(),
            rules: vec!["use present tense".to_owned()],
            code_samples: vec![CodeSample {
                path: "src/auth.rs".to_owned(),
                content: "fn login() {}".to_owned(),
            }],
        }
    }

    #[test]
    fn request_round_trips_through_serde() {
        let req = sample_request();
        let json = serde_json::to_string(&req).expect("serialise");
        let back: SummariserRequest = serde_json::from_str(&json).expect("deserialise");
        assert_eq!(back, req);
    }

    #[test]
    fn request_serialises_with_design_doc_field_names() {
        let req = sample_request();
        let json = serde_json::to_string(&req).expect("serialise");
        assert!(json.contains("\"schema_version\""));
        assert!(json.contains("\"request_id\""));
        assert!(json.contains("\"draft_type\""));
        assert!(json.contains("\"target_node\""));
        assert!(json.contains("\"map_facts\""));
        assert!(json.contains("\"contract_excerpt\""));
        assert!(json.contains("\"interface_findings\""));
        assert!(json.contains("\"docstring_findings\""));
        assert!(json.contains("\"project_context\""));
        assert!(json.contains("\"rules\""));
        assert!(json.contains("\"code_samples\""));
    }

    #[test]
    fn request_omits_empty_defaults() {
        let req = SummariserRequest {
            schema_version: SUMMARISER_SCHEMA_VERSION,
            request_id: "req-002".to_owned(),
            draft_type: "contract".to_owned(),
            target_node: "app.core".to_owned(),
            map_facts: Vec::new(),
            contract_excerpt: None,
            interface_findings: Vec::new(),
            docstring_findings: Vec::new(),
            project_context: String::new(),
            rules: Vec::new(),
            code_samples: Vec::new(),
        };
        let json = serde_json::to_string(&req).expect("serialise");
        // Empty collections and None should be omitted
        assert!(!json.contains("\"map_facts\""));
        assert!(!json.contains("\"contract_excerpt\""));
        assert!(!json.contains("\"interface_findings\""));
        assert!(!json.contains("\"docstring_findings\""));
        assert!(!json.contains("\"project_context\""));
        assert!(!json.contains("\"rules\""));
        assert!(!json.contains("\"code_samples\""));
        // But required fields are present
        assert!(json.contains("\"schema_version\""));
        assert!(json.contains("\"request_id\""));
        assert!(json.contains("\"draft_type\""));
        assert!(json.contains("\"target_node\""));
    }

    #[test]
    fn response_round_trips_through_serde() {
        let resp = SummariserResponse {
            schema_version: SUMMARISER_SCHEMA_VERSION,
            draft_text: "# Auth Service\n\nReturns the authenticated user.".to_owned(),
            summary: Some("Short summary".to_owned()),
            metadata: Some(serde_json::json!({"tokens_in": 100, "tokens_out": 50})),
        };
        let json = serde_json::to_string(&resp).expect("serialise");
        let back: SummariserResponse = serde_json::from_str(&json).expect("deserialise");
        assert_eq!(back, resp);
    }

    #[test]
    fn response_without_summary_parses() {
        let resp: SummariserResponse =
            serde_json::from_str(r#"{"schema_version":1,"draft_text":"hi"}"#).expect("parse");
        assert_eq!(resp.draft_text, "hi");
        assert!(resp.summary.is_none());
        assert!(resp.metadata.is_none());
    }

    #[test]
    fn response_without_metadata_parses() {
        let resp: SummariserResponse =
            serde_json::from_str(r#"{"schema_version":1,"draft_text":"hi","summary":"short"}"#)
                .expect("parse");
        assert_eq!(resp.draft_text, "hi");
        assert_eq!(resp.summary, Some("short".to_owned()));
        assert!(resp.metadata.is_none());
    }

    #[test]
    fn response_rejects_unknown_fields() {
        let result: Result<SummariserResponse, _> = serde_json::from_str(
            r#"{"schema_version":1,"draft_text":"hi","rationale":"unexpected"}"#,
        );
        assert!(
            result.is_err(),
            "deny_unknown_fields must reject extra keys"
        );
    }

    #[test]
    fn request_rejects_unknown_fields() {
        let result: Result<SummariserRequest, _> = serde_json::from_str(
            r#"{"schema_version":1,"request_id":"r","draft_type":"c","target_node":"n","extra":1}"#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn response_legacy_summary_only_is_rejected() {
        // Old wire format used `summary` as the primary text field.
        // The new schema requires `draft_text`.
        let result: Result<SummariserResponse, _> =
            serde_json::from_str(r#"{"schema_version":1,"summary":"hi"}"#);
        assert!(result.is_err(), "missing draft_text must be rejected");
    }
}
