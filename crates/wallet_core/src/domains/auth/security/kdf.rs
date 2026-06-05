use crate::error::WalletError;
use scrypt::{scrypt, Params};
use sha2::{Digest, Sha256};
use zeroize::Zeroizing;

pub(super) const SALT_LEN: usize = 16;
pub(super) const KEY_LEN: usize = 32;

const SCRYPT_LOG_N: u8 = 15;
const SCRYPT_R: u32 = 8;
const SCRYPT_P: u32 = 1;
const MASTER_PASSWORD_VERIFIER_LABEL: &[u8] = b"local-wallet/master-password-verifier/v1";
const DURESS_PASSWORD_VERIFIER_LABEL: &[u8] = b"local-wallet/duress-password-verifier/v1";
const KEYSTORE_ENCRYPTION_LABEL: &[u8] = b"local-wallet/keystore-encryption/v1";

pub fn derive_key(password: &str, salt: &[u8]) -> Result<Zeroizing<[u8; KEY_LEN]>, WalletError> {
    derive_labeled_key(password, salt, KEYSTORE_ENCRYPTION_LABEL)
}

pub(super) fn derive_master_password_verifier(
    password: &str,
    salt: &[u8],
) -> Result<Zeroizing<[u8; KEY_LEN]>, WalletError> {
    derive_labeled_key(password, salt, MASTER_PASSWORD_VERIFIER_LABEL)
}

pub(super) fn derive_duress_password_verifier(
    password: &str,
    salt: &[u8],
) -> Result<Zeroizing<[u8; KEY_LEN]>, WalletError> {
    derive_labeled_key(password, salt, DURESS_PASSWORD_VERIFIER_LABEL)
}

fn derive_raw_scrypt_key(
    password: &str,
    salt: &[u8],
) -> Result<Zeroizing<[u8; KEY_LEN]>, WalletError> {
    if salt.len() != SALT_LEN {
        return Err(WalletError::Crypto);
    }

    let params =
        Params::new(SCRYPT_LOG_N, SCRYPT_R, SCRYPT_P, KEY_LEN).map_err(|_| WalletError::Crypto)?;
    let mut key = Zeroizing::new([0u8; KEY_LEN]);
    scrypt(password.as_bytes(), salt, &params, key.as_mut()).map_err(|_| WalletError::Crypto)?;
    Ok(key)
}

fn derive_labeled_key(
    password: &str,
    salt: &[u8],
    label: &[u8],
) -> Result<Zeroizing<[u8; KEY_LEN]>, WalletError> {
    let raw_key = derive_raw_scrypt_key(password, salt)?;
    let mut hasher = Sha256::new();
    hasher.update(label);
    hasher.update(raw_key.as_slice());
    let digest = hasher.finalize();

    let mut key = Zeroizing::new([0u8; KEY_LEN]);
    key.copy_from_slice(&digest);
    Ok(key)
}
