use crate::error::WalletError;
use reqwest::blocking::Client;
use serde_json::json;

use super::super::encoding::hex_quantity_to_u128;

pub(super) fn eth_u128(
    client: &Client,
    rpc_url: &str,
    method: &str,
    params: serde_json::Value,
) -> Result<u128, WalletError> {
    let value = eth_string(client, rpc_url, method, params)?;
    hex_quantity_to_u128(&value)
}

pub(super) fn eth_string(
    client: &Client,
    rpc_url: &str,
    method: &str,
    params: serde_json::Value,
) -> Result<String, WalletError> {
    let body = eth_value(client, rpc_url, method, params)?;
    body.as_str()
        .map(str::to_string)
        .ok_or(WalletError::NetworkUnavailable)
}

pub(super) fn eth_value(
    client: &Client,
    rpc_url: &str,
    method: &str,
    params: serde_json::Value,
) -> Result<serde_json::Value, WalletError> {
    let body = client
        .post(rpc_url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params,
        }))
        .send()
        .map_err(|_| WalletError::NetworkUnavailable)?
        .error_for_status()
        .map_err(|_| WalletError::NetworkUnavailable)?
        .json::<serde_json::Value>()
        .map_err(|_| WalletError::NetworkUnavailable)?;
    if body.get("error").is_some() {
        return Err(WalletError::NetworkUnavailable);
    }
    body.get("result")
        .cloned()
        .ok_or(WalletError::NetworkUnavailable)
}
