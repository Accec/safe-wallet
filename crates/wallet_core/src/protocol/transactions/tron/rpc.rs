use crate::error::WalletError;
use reqwest::blocking::Client;

use super::super::encoding::tron_rest_url;

pub(super) fn post(
    client: &Client,
    rpc_url: &str,
    path: &str,
    body: serde_json::Value,
) -> Result<serde_json::Value, WalletError> {
    let url = tron_rest_url(rpc_url, path);
    client
        .post(url)
        .json(&body)
        .send()
        .map_err(|_| WalletError::NetworkUnavailable)?
        .error_for_status()
        .map_err(|_| WalletError::NetworkUnavailable)?
        .json::<serde_json::Value>()
        .map_err(|_| WalletError::NetworkUnavailable)
}
