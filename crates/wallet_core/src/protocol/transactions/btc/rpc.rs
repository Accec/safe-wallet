use crate::error::WalletError;
use bitcoin::consensus::encode::serialize;
use bitcoin::Transaction;
use reqwest::blocking::Client;

pub(super) fn fee_rate_sat_vb(client: &Client, rpc_url: &str) -> Result<u64, WalletError> {
    let url = format!("{}/fee-estimates", rpc_url.trim().trim_end_matches('/'));
    let response = client.get(url).send();
    let Ok(response) = response else {
        return Ok(10);
    };
    let Ok(response) = response.error_for_status() else {
        return Ok(10);
    };
    let Ok(fees) = response.json::<serde_json::Map<String, serde_json::Value>>() else {
        return Ok(10);
    };
    let fee = fees
        .get("1")
        .or_else(|| fees.get("2"))
        .or_else(|| fees.get("3"))
        .and_then(|value| value.as_f64())
        .unwrap_or(10.0)
        .ceil() as u64;
    Ok(fee.clamp(1, 250))
}

pub(super) fn broadcast(
    client: &Client,
    rpc_url: &str,
    transaction: &Transaction,
) -> Result<(), WalletError> {
    let url = format!("{}/tx", rpc_url.trim().trim_end_matches('/'));
    let response = client
        .post(url)
        .body(hex::encode(serialize(transaction)))
        .send()
        .map_err(|_| WalletError::NetworkUnavailable)?
        .error_for_status()
        .map_err(|_| WalletError::NetworkUnavailable)?;
    let _ = response
        .text()
        .map_err(|_| WalletError::NetworkUnavailable)?;
    Ok(())
}
