use serde_json::Value;

use super::support::response;

#[test]
fn app_status_returns_no_private_material() {
    let response = response(r#"{"command":"app_status"}"#);
    assert!(response.ok);
    assert!(!response.body_json.contains("private"));
    assert!(!response.body_json.contains("mnemonic"));
    assert!(!response.body_json.contains("seed"));
}

#[test]
fn invalid_command_returns_safe_error() {
    let response = response(r#"{"command":"unknown"}"#);

    assert!(!response.ok);
    assert_eq!(response.error.as_deref(), Some("Invalid command"));
}

#[test]
fn v2_envelope_preserves_request_id_and_returns_data() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let db_path = db_path.to_string_lossy();
    let response = response(&format!(
        r#"{{
                "protocol_version": 2,
                "request_id": "req-status-1",
                "domain": "app",
                "action": "status",
                "payload": {{"db_path": "{db_path}"}}
            }}"#
    ));
    let body: Value = serde_json::from_str(&response.body_json).unwrap();

    assert!(response.ok);
    assert_eq!(body["request_id"], "req-status-1");
    assert_eq!(body["ok"], true);
    assert_eq!(body["data"]["initialized"], false);
    assert!(body.get("error").is_none());
}

#[test]
fn v2_envelope_returns_structured_error_codes() {
    let response = response(
        r#"{
                "protocol_version": 2,
                "request_id": "req-bad-domain",
                "domain": "unknown",
                "action": "status",
                "payload": {}
            }"#,
    );
    let body: Value = serde_json::from_str(&response.body_json).unwrap();

    assert!(response.ok);
    assert_eq!(body["request_id"], "req-bad-domain");
    assert_eq!(body["ok"], false);
    assert_eq!(body["error"]["code"], "unknown_action");
    assert_eq!(body["error"]["retryable"], false);
}
