mod crypto;
mod exports;
mod keys;
mod model;

#[cfg(test)]
mod tests;

use super::mnemonic;
use crate::error::WalletError;

pub use exports::encrypted_keystore_from_export;
#[cfg(test)]
pub use model::CIPHER_NAME;
pub use model::{EncryptedKeystore, SECRET_KIND_MNEMONIC, SECRET_KIND_PRIVATE_KEY};

pub fn encrypt_mnemonic(mnemonic: &str, password: &str) -> Result<EncryptedKeystore, WalletError> {
    mnemonic::validate_mnemonic(mnemonic)?;
    crypto::encrypt_secret(SECRET_KIND_MNEMONIC, mnemonic, password)
}

pub fn encrypt_private_key(
    private_key: &str,
    password: &str,
) -> Result<EncryptedKeystore, WalletError> {
    let private_key = keys::normalize_private_key(private_key)?;
    crypto::encrypt_secret(SECRET_KIND_PRIVATE_KEY, &private_key, password)
}

pub fn verify_keystore_password(
    keystore: &EncryptedKeystore,
    password: &str,
) -> Result<(), WalletError> {
    let _secret = crypto::decrypt_secret(keystore, password)?;
    Ok(())
}

pub fn decrypt_mnemonic(
    keystore: &EncryptedKeystore,
    password: &str,
) -> Result<String, WalletError> {
    if keystore.secret_kind != SECRET_KIND_MNEMONIC {
        return Err(WalletError::Crypto);
    }
    let mnemonic = crypto::decrypt_secret(keystore, password)?;
    mnemonic::validate_mnemonic(&mnemonic)?;
    Ok(mnemonic.to_string())
}

pub fn decrypt_private_key(
    keystore: &EncryptedKeystore,
    password: &str,
) -> Result<String, WalletError> {
    if keystore.secret_kind != SECRET_KIND_PRIVATE_KEY {
        return Err(WalletError::Crypto);
    }
    let private_key = crypto::decrypt_secret(keystore, password)?;
    keys::normalize_private_key(&private_key)
}
