use super::builder;
use crate::error::WalletError;
use serde_json::json;

#[test]
fn extracts_unsigned_transaction_from_trc20_trigger_response() {
    let response = json!({
        "transaction": {
            "txID": "abc123",
            "raw_data_hex": "00"
        }
    });

    let transaction = builder::transaction_from_trigger_response(&response).unwrap();

    assert_eq!(transaction["txID"], "abc123");
}

#[test]
fn missing_trc20_trigger_transaction_is_network_error() {
    let response = json!({ "result": { "result": false } });

    let error = builder::transaction_from_trigger_response(&response).unwrap_err();

    assert_eq!(error, WalletError::NetworkUnavailable);
}
