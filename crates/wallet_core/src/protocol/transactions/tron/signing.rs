use crate::error::WalletError;
use k256::ecdsa::SigningKey;
use serde_json::json;

pub(super) fn sign_transaction(
    mut transaction: serde_json::Value,
    signing_key: &SigningKey,
) -> Result<serde_json::Value, WalletError> {
    let raw_data_hex = transaction
        .get("raw_data_hex")
        .and_then(|value| value.as_str())
        .ok_or(WalletError::NetworkUnavailable)?;
    let raw_data = hex::decode(raw_data_hex).map_err(|_| WalletError::NetworkUnavailable)?;
    let key_bytes = signing_key.to_bytes();
    let mut private_key = [0_u8; 32];
    private_key.copy_from_slice(key_bytes.as_slice());
    let signer = signer_tron::Signer::from_bytes(&private_key).map_err(|_| WalletError::Crypto)?;
    let signature_bytes = signer
        .sign_transaction(&raw_data)
        .map_err(|_| WalletError::Crypto)?
        .to_bytes();
    transaction["signature"] = json!([hex::encode(signature_bytes)]);
    Ok(transaction)
}
