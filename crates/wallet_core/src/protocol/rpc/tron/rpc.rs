use super::{address::tron_base58_to_hex, urls::tron_rest_url};
use crate::error::WalletError;
use reqwest::blocking::Client;
use serde_json::json;

pub(super) fn tron_constant_call(
    client: &Client,
    rpc_url: &str,
    owner_address: &str,
    contract_address: &str,
    function_selector: &str,
    parameter: &str,
) -> Result<serde_json::Value, WalletError> {
    tron_post(
        client,
        rpc_url,
        "/wallet/triggerconstantcontract",
        json!({
            "owner_address": tron_base58_to_hex(owner_address)?,
            "contract_address": tron_base58_to_hex(contract_address)?,
            "function_selector": function_selector,
            "parameter": parameter,
            "visible": false,
        }),
    )
}

pub(super) fn tron_post(
    client: &Client,
    rpc_url: &str,
    path: &str,
    body: serde_json::Value,
) -> Result<serde_json::Value, WalletError> {
    let url = tron_rest_url(rpc_url, path);
    let body = client
        .post(url)
        .json(&body)
        .send()
        .map_err(|_| WalletError::NetworkUnavailable)?
        .error_for_status()
        .map_err(|_| WalletError::NetworkUnavailable)?
        .json::<serde_json::Value>()
        .map_err(|_| WalletError::NetworkUnavailable)?;
    if body
        .get("result")
        .and_then(|value| value.get("result"))
        .and_then(|value| value.as_bool())
        == Some(false)
    {
        return Err(WalletError::NetworkUnavailable);
    }
    Ok(body)
}
