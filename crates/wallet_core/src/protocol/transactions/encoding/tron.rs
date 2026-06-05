use crate::error::WalletError;
use sha2::{Digest, Sha256};

use super::super::amount::decimal_amount_to_be_bytes;
use super::padding::left_pad_32;

pub(in crate::protocol::transactions) fn encode_trc20_transfer_parameter(
    to_address: &str,
    amount: &str,
    decimals: u8,
) -> Result<String, WalletError> {
    let address = tron_base58_to_hex(to_address)?;
    let address = address
        .strip_prefix("41")
        .ok_or(WalletError::InvalidAddress)?;
    let address_bytes = hex::decode(address).map_err(|_| WalletError::InvalidAddress)?;
    let amount = decimal_amount_to_be_bytes(amount, decimals)?;
    Ok(format!(
        "{}{}",
        hex::encode(left_pad_32(&address_bytes)),
        hex::encode(left_pad_32(&amount))
    ))
}

pub(in crate::protocol::transactions) fn encode_tron_balance_of_parameter(
    address_hex: &str,
) -> Result<String, WalletError> {
    let address = address_hex
        .strip_prefix("41")
        .ok_or(WalletError::InvalidAddress)?;
    let address_bytes = hex::decode(address).map_err(|_| WalletError::InvalidAddress)?;
    Ok(hex::encode(left_pad_32(&address_bytes)))
}

pub(in crate::protocol::transactions) fn tron_base58_to_hex(
    address: &str,
) -> Result<String, WalletError> {
    let decoded = bs58::decode(address)
        .into_vec()
        .map_err(|_| WalletError::InvalidAddress)?;
    if decoded.len() != 25 {
        return Err(WalletError::InvalidAddress);
    }
    let (payload, checksum) = decoded.split_at(21);
    if payload.first() != Some(&0x41) {
        return Err(WalletError::InvalidAddress);
    }
    let first = Sha256::digest(payload);
    let second = Sha256::digest(first);
    if checksum != &second[..4] {
        return Err(WalletError::InvalidAddress);
    }
    Ok(hex::encode(payload))
}

pub(in crate::protocol::transactions) fn tron_rest_url(rpc_url: &str, path: &str) -> String {
    let mut base = rpc_url.trim().trim_end_matches('/');
    if let Some(rest_base) = base.strip_suffix("/jsonrpc") {
        base = rest_base;
    }
    if base.ends_with("/wallet") && path.starts_with("/wallet/") {
        let path = path.trim_start_matches("/wallet");
        return format!("{base}{path}");
    }
    if path.starts_with('/') {
        format!("{base}{path}")
    } else {
        format!("{base}/{path}")
    }
}
