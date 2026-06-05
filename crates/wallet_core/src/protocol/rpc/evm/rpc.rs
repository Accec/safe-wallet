use crate::error::WalletError;
use reqwest::blocking::Client;
use serde_json::json;

pub(super) fn eth_get_balance(
    client: &Client,
    rpc_url: &str,
    address: &str,
) -> Result<String, WalletError> {
    let response = client
        .post(rpc_url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_getBalance",
            "params": [address, "latest"],
        }))
        .send()
        .map_err(|_| WalletError::NetworkUnavailable)?
        .error_for_status()
        .map_err(|_| WalletError::NetworkUnavailable)?;
    let body = response
        .json::<serde_json::Value>()
        .map_err(|_| WalletError::NetworkUnavailable)?;
    body.get("result")
        .and_then(|value| value.as_str())
        .map(str::to_string)
        .ok_or(WalletError::NetworkUnavailable)
}

pub(super) fn eth_call(
    client: &Client,
    rpc_url: &str,
    contract_address: &str,
    data: &str,
) -> Result<String, WalletError> {
    let body = client
        .post(rpc_url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_call",
            "params": [
                {
                    "to": contract_address,
                    "data": data,
                },
                "latest"
            ],
        }))
        .send()
        .map_err(|_| WalletError::NetworkUnavailable)?
        .error_for_status()
        .map_err(|_| WalletError::NetworkUnavailable)?
        .json::<serde_json::Value>()
        .map_err(|_| WalletError::NetworkUnavailable)?;
    if body.get("error").is_some() {
        return Err(WalletError::InvalidTokenContract);
    }
    let result = body
        .get("result")
        .and_then(|value| value.as_str())
        .ok_or(WalletError::InvalidTokenContract)?;
    if result == "0x" {
        return Err(WalletError::InvalidTokenContract);
    }
    Ok(result.to_string())
}
