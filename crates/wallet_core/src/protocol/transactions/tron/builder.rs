use crate::error::WalletError;
use reqwest::blocking::Client;
use serde_json::json;

use super::super::encoding::{encode_trc20_transfer_parameter, tron_base58_to_hex};
use super::rpc;

pub(super) fn create_native_transfer(
    client: &Client,
    rpc_url: &str,
    owner_address: &str,
    to_address: &str,
    amount: u64,
) -> Result<serde_json::Value, WalletError> {
    rpc::post(
        client,
        rpc_url,
        "/wallet/createtransaction",
        json!({
            "owner_address": owner_address,
            "to_address": tron_base58_to_hex(to_address)?,
            "amount": amount,
            "visible": false,
        }),
    )
}

pub(super) fn create_trc20_transfer(
    client: &Client,
    rpc_url: &str,
    owner_address: &str,
    contract_address: &str,
    to_address: &str,
    amount: &str,
    decimals: u8,
) -> Result<serde_json::Value, WalletError> {
    let response = rpc::post(
        client,
        rpc_url,
        "/wallet/triggersmartcontract",
        json!({
            "owner_address": owner_address,
            "contract_address": contract_address,
            "function_selector": "transfer(address,uint256)",
            "parameter": encode_trc20_transfer_parameter(to_address, amount, decimals)?,
            "call_value": 0,
            "fee_limit": 100_000_000_u64,
            "visible": false,
        }),
    )?;
    transaction_from_trigger_response(&response)
}

pub(super) fn transaction_from_trigger_response(
    response: &serde_json::Value,
) -> Result<serde_json::Value, WalletError> {
    response
        .get("transaction")
        .cloned()
        .ok_or(WalletError::NetworkUnavailable)
}
