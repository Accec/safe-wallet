use super::{address, calldata, rpc};
use crate::error::WalletError;
use reqwest::blocking::Client;
use serde_json::json;

pub(super) fn fetch_native_balance(
    client: &Client,
    rpc_url: &str,
    address: &str,
) -> Result<String, WalletError> {
    let body = rpc::tron_post(
        client,
        rpc_url,
        "/wallet/getaccount",
        json!({
            "address": address::tron_base58_to_hex(address)?,
            "visible": false,
        }),
    )?;
    decode_tron_account_balance(&body)
}

pub(super) fn fetch_token_balance(
    client: &Client,
    rpc_url: &str,
    owner_address: &str,
    contract_address: &str,
    decimals: u8,
) -> Result<String, WalletError> {
    let body = rpc::tron_constant_call(
        client,
        rpc_url,
        owner_address,
        contract_address,
        "balanceOf(address)",
        &calldata::encode_tron_balance_of(owner_address)?,
    )?;
    decode_tron_constant_balance(&body, decimals)
}

pub(super) fn decode_tron_account_balance(body: &serde_json::Value) -> Result<String, WalletError> {
    let balance = body
        .get("balance")
        .and_then(|value| {
            value
                .as_u64()
                .map(|number| number.to_string())
                .or_else(|| value.as_str().map(str::to_string))
        })
        .unwrap_or_else(|| "0".to_string());
    Ok(super::super::units::format_decimal_units(&balance, 6))
}

pub(super) fn decode_tron_constant_balance(
    body: &serde_json::Value,
    decimals: u8,
) -> Result<String, WalletError> {
    let result = body
        .get("constant_result")
        .and_then(|value| value.as_array())
        .and_then(|values| values.first())
        .and_then(|value| value.as_str())
        .ok_or(WalletError::NetworkUnavailable)?;
    super::super::units::format_hex_units(result, decimals)
}
