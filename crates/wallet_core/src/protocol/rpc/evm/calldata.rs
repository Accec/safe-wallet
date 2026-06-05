use crate::error::WalletError;

pub(super) fn encode_balance_of(address: &str) -> Result<String, WalletError> {
    let address = normalize_evm_address(address)?;
    Ok(format!("0x70a08231{address:0>64}"))
}

fn normalize_evm_address(address: &str) -> Result<String, WalletError> {
    let address = address
        .strip_prefix("0x")
        .ok_or(WalletError::InvalidAddress)?;
    if address.len() != 40 || !address.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(WalletError::InvalidAddress);
    }
    Ok(address.to_ascii_lowercase())
}
