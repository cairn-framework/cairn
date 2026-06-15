//! Tests for the summariser backend implementations.

use super::*;
use crate::summariser::request::SUMMARISER_SCHEMA_VERSION;

fn sample_request() -> SummariserRequest {
    SummariserRequest {
        schema_version: SUMMARISER_SCHEMA_VERSION,
        request_id: "req-a".to_owned(),
        draft_type: "contract".to_owned(),
        target_node: "node-a".to_owned(),
        map_facts: Vec::new(),
        contract_excerpt: None,
        interface_findings: Vec::new(),
        docstring_findings: Vec::new(),
        project_context: String::new(),
        rules: Vec::new(),
        code_samples: Vec::new(),
    }
}

#[test]
fn mode_default_is_disabled() {
    let mode = SummariserMode::default();
    assert!(matches!(mode, SummariserMode::Disabled));
    assert!(mode.timeout().is_none());
}

#[test]
fn local_command_mode_exposes_timeout() {
    let mode = SummariserMode::LocalCommand {
        command: "/bin/cat".to_owned(),
        args: Vec::new(),
        timeout_ms: 5000,
    };
    assert_eq!(mode.timeout(), Some(Duration::from_secs(5)));
}

#[test]
fn disabled_backend_returns_disabled_error() {
    let backend = DisabledBackend;
    let request = sample_request();
    let err = backend
        .invoke(&request, Duration::from_secs(1))
        .expect_err("should error");
    assert!(matches!(err, SummariserBackendError::Disabled));
}

#[test]
fn mode_round_trips_through_serde() {
    let mode = SummariserMode::LocalCommand {
        command: "/usr/local/bin/summariser".to_owned(),
        args: vec!["--quiet".to_owned()],
        timeout_ms: 30_000,
    };
    let json = serde_json::to_string(&mode).expect("serialise");
    let back: SummariserMode = serde_json::from_str(&json).expect("deserialise");
    assert_eq!(back, mode);
}

#[test]
fn test_fake_backend_ok_returns_configured_response() {
    let response = SummariserResponse {
        schema_version: SUMMARISER_SCHEMA_VERSION,
        draft_text: "fake draft text".to_owned(),
        summary: None,
        metadata: None,
    };
    let backend = FakeBackend::ok(response.clone());
    let result = backend.invoke(&sample_request(), Duration::from_secs(1));
    assert_eq!(result.unwrap(), response);
}

#[test]
fn test_fake_backend_err_returns_configured_error() {
    let backend = FakeBackend::err(SummariserBackendError::Disabled);
    let result = backend.invoke(&sample_request(), Duration::from_secs(1));
    assert!(matches!(result, Err(SummariserBackendError::Disabled)));
}

#[test]
fn test_fake_backend_different_requests_returns_same_response() {
    let response = SummariserResponse {
        schema_version: SUMMARISER_SCHEMA_VERSION,
        draft_text: "deterministic".to_owned(),
        summary: None,
        metadata: None,
    };
    let backend = FakeBackend::ok(response);
    let req1 = sample_request();
    let mut req2 = req1.clone();
    req2.target_node = "different".to_owned();
    let out1 = backend.invoke(&req1, Duration::from_millis(100)).unwrap();
    let out2 = backend.invoke(&req2, Duration::from_secs(10)).unwrap();
    assert_eq!(out1, out2);
}

#[test]
fn test_local_command_backend_echoes_valid_response() {
    let expected = SummariserResponse {
        schema_version: SUMMARISER_SCHEMA_VERSION,
        draft_text: "from shell".to_owned(),
        summary: None,
        metadata: None,
    };
    let json = serde_json::to_string(&expected).unwrap();
    let script = format!("printf '%s\\n' '{json}'");
    let backend = LocalCommandBackend::new("/bin/sh".to_owned(), vec!["-c".to_owned(), script]);
    let result = backend
        .invoke(&sample_request(), Duration::from_secs(5))
        .expect("should succeed");
    assert_eq!(result.draft_text, expected.draft_text);
}

#[test]
fn test_local_command_backend_non_zero_exit_returns_error() {
    let backend = LocalCommandBackend::new(
        "/bin/sh".to_owned(),
        vec!["-c".to_owned(), "echo 'bad' >&2; exit 1".to_owned()],
    );
    let result = backend.invoke(&sample_request(), Duration::from_secs(5));
    match result {
        Err(SummariserBackendError::NonZeroExit { code, stderr }) => {
            assert_eq!(code, 1);
            assert!(
                stderr.contains("bad"),
                "stderr should contain 'bad', got: {stderr}"
            );
        }
        other => panic!("expected NonZeroExit, got {other:?}"),
    }
}

#[test]
fn test_local_command_backend_invalid_json_returns_parse_error() {
    let backend = LocalCommandBackend::new(
        "/bin/sh".to_owned(),
        vec!["-c".to_owned(), "echo 'not json'".to_owned()],
    );
    let result = backend.invoke(&sample_request(), Duration::from_secs(5));
    assert!(
        matches!(result, Err(SummariserBackendError::Parse(_))),
        "expected Parse error for invalid JSON, got {result:?}"
    );
}

#[test]
fn test_local_command_backend_timeout_returns_timeout_error() {
    let backend = LocalCommandBackend::new(
        "/bin/sh".to_owned(),
        vec!["-c".to_owned(), "sleep 10".to_owned()],
    );
    let result = backend.invoke(&sample_request(), Duration::from_millis(100));
    assert!(
        matches!(result, Err(SummariserBackendError::Timeout { .. })),
        "expected Timeout error, got {result:?}"
    );
}

#[test]
fn test_hosted_backend_returns_unsupported_error() {
    let config = HostedConfig {
        adapter: "openai".to_owned(),
        base_url: Some("https://api.openai.com".to_owned()),
        timeout_ms: Some(30_000),
    };
    let backend = HostedBackend::new(config);
    let result = backend.invoke(&sample_request(), Duration::from_secs(5));
    match result {
        Err(SummariserBackendError::Io(msg)) => {
            assert_eq!(msg, "unsupported hosted backend: openai");
        }
        other => panic!("expected Io error, got {other:?}"),
    }
}

#[test]
fn test_hosted_config_round_trips_through_serde() {
    let config = HostedConfig {
        adapter: "anthropic".to_owned(),
        base_url: Some("https://api.anthropic.com".to_owned()),
        timeout_ms: Some(60_000),
    };
    let json = serde_json::to_string(&config).expect("serialise");
    let back: HostedConfig = serde_json::from_str(&json).expect("deserialise");
    assert_eq!(back, config);
}

#[test]
fn test_hosted_config_with_defaults_parses() {
    let json = r#"{"adapter":"test"}"#;
    let config: HostedConfig = serde_json::from_str(json).expect("parse");
    assert_eq!(config.adapter, "test");
    assert!(config.base_url.is_none());
    assert!(config.timeout_ms.is_none());
}
