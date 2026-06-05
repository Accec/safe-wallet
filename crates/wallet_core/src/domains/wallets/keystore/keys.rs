use crate::error::WalletError;

pub(super) fn normalize_private_key(private_key: &str) -> Result<String, WalletError> {
    let private_key = private_key.trim();
    let private_key = private_key
        .strip_prefix("0x")
        .or_else(|| private_key.strip_prefix("0X"))
        .unwrap_or(private_key);
    if private_key.len() != 64 || !private_key.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(WalletError::InvalidPrivateKey);
    }
    let bytes = hex::decode(private_key).map_err(|_| WalletError::InvalidPrivateKey)?;
    k256::ecdsa::SigningKey::from_slice(&bytes).map_err(|_| WalletError::InvalidPrivateKey)?;
    Ok(private_key.to_ascii_lowercase())
}
