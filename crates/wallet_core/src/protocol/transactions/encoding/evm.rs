use crate::error::WalletError;

use super::super::amount::decimal_amount_to_be_bytes;
use super::padding::left_pad_32;

pub(in crate::protocol::transactions) fn encode_erc20_transfer(
    to_address: &str,
    amount: &str,
    decimals: u8,
) -> Result<Vec<u8>, WalletError> {
    let address_bytes = normalize_evm_address(to_address)?;
    let amount = decimal_amount_to_be_bytes(amount, decimals)?;
    let mut data = hex::decode("a9059cbb").map_err(|_| WalletError::Crypto)?;
    data.extend_from_slice(&left_pad_32(&address_bytes));
    data.extend_from_slice(&left_pad_32(&amount));
    Ok(data)
}

pub(in crate::protocol::transactions) fn evm_hex_quantity(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "0x0".to_string();
    }
    let encoded = hex::encode(bytes);
    let trimmed = encoded.trim_start_matches('0');
    if trimmed.is_empty() {
        "0x0".to_string()
    } else {
        format!("0x{trimmed}")
    }
}

pub(in crate::protocol::transactions) fn normalize_evm_address(
    address: &str,
) -> Result<Vec<u8>, WalletError> {
    let address = address
        .trim()
        .strip_prefix("0x")
        .ok_or(WalletError::InvalidAddress)?;
    if address.len() != 40 || !address.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(WalletError::InvalidAddress);
    }
    hex::decode(address).map_err(|_| WalletError::InvalidAddress)
}
