use super::address::tron_base58_to_hex;
use crate::error::WalletError;

pub(super) fn encode_tron_balance_of(address: &str) -> Result<String, WalletError> {
    let hex_address = tron_base58_to_hex(address)?;
    let address_without_prefix = hex_address
        .strip_prefix("41")
        .ok_or(WalletError::InvalidAddress)?;
    Ok(format!("{address_without_prefix:0>64}"))
}
