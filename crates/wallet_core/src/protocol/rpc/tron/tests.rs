use super::{address, balances, calldata, urls};
use crate::models::ChainId;
use crate::protocol::rpc::{AssetBalanceClient, RpcNativeBalanceClient};
use serde_json::json;

#[test]
fn rpc_client_supports_tron_and_encodes_tron_contract_calls() {
    let settings = crate::protocol::network::default_network_privacy_settings();
    let client = RpcNativeBalanceClient::new(&settings).unwrap();

    assert!(client.supports_chain(ChainId::Tron));
    assert_eq!(
        address::tron_base58_to_hex("TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7").unwrap(),
        "4174472e7d35395a6b5add427eecb7f4b62ad2b071"
    );
    assert_eq!(
        calldata::encode_tron_balance_of("TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7").unwrap(),
        "00000000000000000000000074472e7d35395a6b5add427eecb7f4b62ad2b071"
    );
}

#[test]
fn decodes_tron_api_balance_results() {
    let account = json!({ "balance": 1234567 });
    let token = json!({
        "constant_result": [
            "000000000000000000000000000000000000000000000000000000000001e240"
        ]
    });

    assert_eq!(
        balances::decode_tron_account_balance(&account).unwrap(),
        "1.234567"
    );
    assert_eq!(
        balances::decode_tron_constant_balance(&token, 6).unwrap(),
        "0.123456"
    );
}

#[test]
fn tron_rest_url_accepts_jsonrpc_public_rpc_urls() {
    assert_eq!(
        urls::tron_rest_url("https://api.trongrid.io/jsonrpc", "/wallet/getaccount"),
        "https://api.trongrid.io/wallet/getaccount"
    );
    assert_eq!(
        urls::tron_rest_url("https://api.trongrid.io/", "/wallet/getaccount"),
        "https://api.trongrid.io/wallet/getaccount"
    );
}
