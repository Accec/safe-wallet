use serde_json::Value;

use super::support::response;

#[test]
fn parse_payment_uri_command_returns_parsed_body_json() {
    let response = response(
        r#"{"command":"parse_payment_uri","payload":"bitcoin:bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu?amount=0.001"}"#,
    );
    let body: Value = serde_json::from_str(&response.body_json).unwrap();

    assert!(response.ok);
    assert_eq!(body["chain"], "btc");
    assert_eq!(body["amount"], "0.001");
}
