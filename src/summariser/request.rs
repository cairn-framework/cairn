//! Typed request/response shapes for the summariser local-command protocol.
//!
//! Wire schema renamed per phase-8 design.md to use `schema_version` /
//! `summary` / `metadata` instead of the older `version` / `draft_text` /
//! `rationale` shape. Producers and consumers share these struct
//! definitions so drift fails at parse time when `deny_unknown_fields`
//! catches unrecognised keys.

use serde::{Deserialize, Serialize};

/// Wire schema version constant.
pub const SUMMARISER_SCHEMA_VERSION: u32 = 1;

/// Node context passed to the summariser backend.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeContext {
    /// Node ID under summarisation.
    pub node_id: String,
    /// Human-readable node name.
    pub name: String,
    /// Node description.
    pub description: String,
    /// Existing contract body, if any.
    #[serde(default)]
    pub contract: Option<String>,
    /// Detected interface contradiction message, if any.
    #[serde(default)]
    pub contradiction: Option<String>,
}

/// Local-command request envelope. Producers (cairn) send exactly one of
/// these JSON objects on the configured backend's stdin, ending with a
/// newline.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SummariserRequest {
    /// Wire schema version.
    pub schema_version: u32,
    /// Stable artefact-type discriminator. Currently always "contract".
    pub artefact_type: String,
    /// Node-level context for the summariser.
    pub node: NodeContext,
}

/// Local-command response envelope. Backends emit exactly one of these
/// JSON objects on stdout, ending with a newline. Cairn stores only
/// `summary` as generated prose; `metadata` is captured for audit but
/// does not affect contract bodies. `deny_unknown_fields` ensures
/// producer drift fails loudly.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SummariserResponse {
    /// Wire schema version.
    pub schema_version: u32,
    /// Generated draft text for the artefact body.
    pub summary: String,
    /// Optional structured audit metadata (token counts, model id, etc.).
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_round_trips_through_serde() {
        let req = SummariserRequest {
            schema_version: SUMMARISER_SCHEMA_VERSION,
            artefact_type: "contract".to_owned(),
            node: NodeContext {
                node_id: "node-a".to_owned(),
                name: "Auth Service".to_owned(),
                description: "Handles authentication".to_owned(),
                contract: Some("# Auth\nReturns user".to_owned()),
                contradiction: Some("interface drift detected".to_owned()),
            },
        };
        let json = serde_json::to_string(&req).expect("serialise");
        let back: SummariserRequest = serde_json::from_str(&json).expect("deserialise");
        assert_eq!(back, req);
    }

    #[test]
    fn response_round_trips_through_serde() {
        let resp = SummariserResponse {
            schema_version: SUMMARISER_SCHEMA_VERSION,
            summary: "# Auth Service\n\nReturns the authenticated user.".to_owned(),
            metadata: Some(serde_json::json!({"tokens_in": 100, "tokens_out": 50})),
        };
        let json = serde_json::to_string(&resp).expect("serialise");
        let back: SummariserResponse = serde_json::from_str(&json).expect("deserialise");
        assert_eq!(back, resp);
    }

    #[test]
    fn response_without_metadata_parses() {
        let resp: SummariserResponse =
            serde_json::from_str(r#"{"schema_version":1,"summary":"hi"}"#).expect("parse");
        assert_eq!(resp.summary, "hi");
        assert!(resp.metadata.is_none());
    }

    #[test]
    fn response_rejects_unknown_fields() {
        let result: Result<SummariserResponse, _> =
            serde_json::from_str(r#"{"schema_version":1,"summary":"hi","rationale":"unexpected"}"#);
        assert!(
            result.is_err(),
            "deny_unknown_fields must reject extra keys"
        );
    }

    #[test]
    fn request_rejects_unknown_fields() {
        let result: Result<SummariserRequest, _> = serde_json::from_str(
            r#"{"schema_version":1,"artefact_type":"contract","node":{"node_id":"a","name":"a","description":""},"extra":1}"#,
        );
        assert!(result.is_err());
    }
}
