use crate::error::WalletError;
use k256::ecdsa::SigningKey;

pub(super) fn private_key_signing_key(private_key: &str) -> Result<SigningKey, WalletError> {
    let private_key = private_key.trim();
    let private_key = private_key
        .strip_prefix("0x")
        .or_else(|| private_key.strip_prefix("0X"))
        .unwrap_or(private_key);
    if private_key.len() != 64 || !private_key.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(WalletError::InvalidPrivateKey);
    }
    let bytes = hex::decode(private_key).map_err(|_| WalletError::InvalidPrivateKey)?;
    SigningKey::from_slice(&bytes).map_err(|_| WalletError::InvalidPrivateKey)
}
