use crate::error::WalletError;
use alloy_consensus::{SignableTransaction, TxEip1559, TxLegacy};
use alloy_eips::eip2718::Encodable2718;
use alloy_signer::SignerSync;
use alloy_signer_local::PrivateKeySigner;
use k256::ecdsa::SigningKey;

pub(super) fn sign_legacy_transaction(
    transaction: TxLegacy,
    signing_key: &SigningKey,
) -> Result<Vec<u8>, WalletError> {
    let key_bytes = signing_key.to_bytes();
    let signer =
        PrivateKeySigner::from_slice(key_bytes.as_slice()).map_err(|_| WalletError::Crypto)?;
    let signature = signer
        .sign_hash_sync(&transaction.signature_hash())
        .map_err(|_| WalletError::Crypto)?;
    Ok(transaction.into_signed(signature).encoded_2718())
}

pub(super) fn sign_eip1559_transaction(
    transaction: TxEip1559,
    signing_key: &SigningKey,
) -> Result<Vec<u8>, WalletError> {
    let key_bytes = signing_key.to_bytes();
    let signer =
        PrivateKeySigner::from_slice(key_bytes.as_slice()).map_err(|_| WalletError::Crypto)?;
    let signature = signer
        .sign_hash_sync(&transaction.signature_hash())
        .map_err(|_| WalletError::Crypto)?;
    Ok(transaction.into_signed(signature).encoded_2718())
}
