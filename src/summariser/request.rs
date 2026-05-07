//! Typed request/response shapes for the summariser local-command protocol.

use serde::{Deserialize, Serialize};

/// Node context passed to the summariser backend.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
pub struct SummariserRequest {
    /// Wire schema version.
    pub version: u32,
    /// Stable artefact-type discriminator. Currently always "contract".
    pub artefact_type: String,
    /// Node-level context for the summariser.
    pub node: NodeContext,
}

/// Local-command response envelope. Backends emit exactly one of these
/// JSON objects on stdout, ending with a newline. The cairn store reads
/// only `draft_text`; producer-supplied metadata is ignored.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SummariserResponse {
    /// Wire schema version.
    pub version: u32,
    /// Generated draft text for the artefact body.
    pub draft_text: String,
    /// Optional rationale string. Currently ignored by cairn.
    #[serde(default)]
    pub rationale: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_round_trips_through_serde() {
        let req = SummariserRequest {
            version: 1,
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
            version: 1,
            draft_text: "# Auth Service\n\nReturns the authenticated user.".to_owned(),
            rationale: Some("aligned with code".to_owned()),
        };
        let json = serde_json::to_string(&resp).expect("serialise");
        let back: SummariserResponse = serde_json::from_str(&json).expect("deserialise");
        assert_eq!(back, resp);
    }

    #[test]
    fn response_without_rationale_parses() {
        let resp: SummariserResponse =
            serde_json::from_str(r#"{"version":1,"draft_text":"hi"}"#).expect("parse");
        assert_eq!(resp.draft_text, "hi");
        assert!(resp.rationale.is_none());
    }
}
