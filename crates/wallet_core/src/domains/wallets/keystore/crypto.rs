use crate::domains::auth::security;
use crate::error::WalletError;
use aes_gcm::aead::{Aead, KeyInit, Payload};
use aes_gcm::{Aes256Gcm, Nonce};
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::{rngs::OsRng, RngCore};
use zeroize::Zeroizing;

use super::model::{EncryptedKeystore, CIPHER_NAME};

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;

pub(super) fn encrypt_secret(
    secret_kind: &str,
    plaintext: &str,
    password: &str,
) -> Result<EncryptedKeystore, WalletError> {
    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);
    let mut nonce = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce);

    let key = security::derive_key(password, &salt)?;
    let cipher = Aes256Gcm::new_from_slice(key.as_slice()).map_err(|_| WalletError::Crypto)?;
    let kdf_name = security::KDF_NAME.to_string();
    let kdf_params_json = security::KDF_PARAMS_JSON.to_string();
    let cipher_name = CIPHER_NAME.to_string();
    let version = security::CRYPTO_VERSION;
    let aad = metadata_aad(&kdf_name, &kdf_params_json, &cipher_name, version);
    let ciphertext = cipher
        .encrypt(
            Nonce::from_slice(&nonce),
            Payload {
                msg: plaintext.as_bytes(),
                aad: aad.as_slice(),
            },
        )
        .map_err(|_| WalletError::Crypto)?;

    Ok(EncryptedKeystore {
        secret_kind: secret_kind.to_string(),
        ciphertext_b64: STANDARD.encode(ciphertext),
        nonce_b64: STANDARD.encode(nonce),
        salt_b64: STANDARD.encode(salt),
        kdf_name,
        kdf_params_json,
        cipher_name,
        version,
    })
}

pub(super) fn decrypt_secret(
    keystore: &EncryptedKeystore,
    password: &str,
) -> Result<Zeroizing<String>, WalletError> {
    if !keystore.has_supported_metadata() {
        return Err(WalletError::Crypto);
    }

    let ciphertext = STANDARD
        .decode(&keystore.ciphertext_b64)
        .map_err(|_| WalletError::Crypto)?;
    if ciphertext.is_empty() {
        return Err(WalletError::Crypto);
    }

    let nonce = decode_fixed::<NONCE_LEN>(&keystore.nonce_b64)?;
    let salt = decode_fixed::<SALT_LEN>(&keystore.salt_b64)?;

    let key = security::derive_key(password, salt.as_slice())?;
    let cipher = Aes256Gcm::new_from_slice(key.as_slice()).map_err(|_| WalletError::Crypto)?;
    let aad = keystore.metadata_aad();
    let plaintext = cipher
        .decrypt(
            Nonce::from_slice(nonce.as_slice()),
            Payload {
                msg: ciphertext.as_slice(),
                aad: aad.as_slice(),
            },
        )
        .map_err(|_| WalletError::InvalidPassword)?;
    let plaintext = String::from_utf8(plaintext).map_err(|_| WalletError::InvalidPassword)?;
    Ok(Zeroizing::new(plaintext))
}

impl EncryptedKeystore {
    fn has_supported_metadata(&self) -> bool {
        self.kdf_name == security::KDF_NAME
            && self.kdf_params_json == security::KDF_PARAMS_JSON
            && self.cipher_name == CIPHER_NAME
            && self.version == security::CRYPTO_VERSION
    }

    fn metadata_aad(&self) -> Vec<u8> {
        metadata_aad(
            &self.kdf_name,
            &self.kdf_params_json,
            &self.cipher_name,
            self.version,
        )
    }
}

fn metadata_aad(kdf_name: &str, kdf_params_json: &str, cipher_name: &str, version: u32) -> Vec<u8> {
    format!("{kdf_name}\0{kdf_params_json}\0{cipher_name}\0{version}").into_bytes()
}

fn decode_fixed<const N: usize>(value_b64: &str) -> Result<Zeroizing<[u8; N]>, WalletError> {
    let decoded = Zeroizing::new(
        STANDARD
            .decode(value_b64)
            .map_err(|_| WalletError::Crypto)?,
    );
    if decoded.len() != N {
        return Err(WalletError::Crypto);
    }

    let mut value = Zeroizing::new([0u8; N]);
    value.copy_from_slice(decoded.as_slice());
    Ok(value)
}
