use crate::error::WalletError;
use alloy_primitives::U256;

pub(in crate::protocol::transactions) fn hex_quantity_to_u128(
    value: &str,
) -> Result<u128, WalletError> {
    let value = value
        .strip_prefix("0x")
        .ok_or(WalletError::NetworkUnavailable)?;
    if value.is_empty() {
        return Ok(0);
    }
    u128::from_str_radix(value, 16).map_err(|_| WalletError::NetworkUnavailable)
}

pub(in crate::protocol::transactions) fn hex_string_to_u256(
    value: &str,
) -> Result<U256, WalletError> {
    let value = value
        .strip_prefix("0x")
        .ok_or(WalletError::NetworkUnavailable)?;
    if value.is_empty() {
        return Ok(U256::ZERO);
    }
    if value.len() > 64 {
        return Err(WalletError::NetworkUnavailable);
    }
    let normalized = if value.len() % 2 == 0 {
        value.to_string()
    } else {
        format!("0{value}")
    };
    let bytes = hex::decode(normalized).map_err(|_| WalletError::NetworkUnavailable)?;
    Ok(U256::from_be_slice(&bytes))
}
