use crate::error::WalletError;
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::{rngs::OsRng, RngCore};
use zeroize::Zeroizing;

use super::kdf::{
    derive_duress_password_verifier, derive_master_password_verifier, KEY_LEN, SALT_LEN,
};
use super::model::{MasterPasswordVerifier, CRYPTO_VERSION, KDF_NAME, KDF_PARAMS_JSON};

pub fn create_master_password_verifier(
    password: &str,
) -> Result<MasterPasswordVerifier, WalletError> {
    create_password_verifier(password, derive_master_password_verifier)
}

pub fn create_duress_password_verifier(
    password: &str,
) -> Result<MasterPasswordVerifier, WalletError> {
    create_password_verifier(password, derive_duress_password_verifier)
}

fn create_password_verifier(
    password: &str,
    derive_verifier: fn(&str, &[u8]) -> Result<Zeroizing<[u8; KEY_LEN]>, WalletError>,
) -> Result<MasterPasswordVerifier, WalletError> {
    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);
    let verifier = derive_verifier(password, &salt)?;
    Ok(MasterPasswordVerifier {
        salt_b64: STANDARD.encode(salt),
        verifier_b64: STANDARD.encode(verifier.as_slice()),
        kdf_name: KDF_NAME.to_string(),
        kdf_params_json: KDF_PARAMS_JSON.to_string(),
        version: CRYPTO_VERSION,
    })
}

pub fn verify_master_password(
    password: &str,
    verifier: &MasterPasswordVerifier,
) -> Result<(), WalletError> {
    verify_password_with(password, verifier, derive_master_password_verifier)
}

pub fn verify_duress_password(
    password: &str,
    verifier: &MasterPasswordVerifier,
) -> Result<(), WalletError> {
    verify_password_with(password, verifier, derive_duress_password_verifier)
}

fn verify_password_with(
    password: &str,
    verifier: &MasterPasswordVerifier,
    derive_verifier: fn(&str, &[u8]) -> Result<Zeroizing<[u8; KEY_LEN]>, WalletError>,
) -> Result<(), WalletError> {
    if !verifier.has_supported_metadata() {
        return Err(WalletError::InvalidPassword);
    }

    let salt = decode_fixed::<SALT_LEN>(&verifier.salt_b64, WalletError::InvalidPassword)?;
    let expected = decode_fixed::<KEY_LEN>(&verifier.verifier_b64, WalletError::InvalidPassword)?;
    let actual =
        derive_verifier(password, salt.as_slice()).map_err(|_| WalletError::InvalidPassword)?;
    if constant_time_eq(actual.as_slice(), expected.as_slice()) {
        Ok(())
    } else {
        Err(WalletError::InvalidPassword)
    }
}

fn decode_fixed<const N: usize>(
    value_b64: &str,
    error: WalletError,
) -> Result<Zeroizing<[u8; N]>, WalletError> {
    let decoded = match STANDARD.decode(value_b64) {
        Ok(decoded) => Zeroizing::new(decoded),
        Err(_) => return Err(error),
    };
    if decoded.len() != N {
        return Err(error);
    }

    let mut value = Zeroizing::new([0u8; N]);
    value.copy_from_slice(decoded.as_slice());
    Ok(value)
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }

    let mut diff = 0u8;
    for (&left_byte, &right_byte) in left.iter().zip(right.iter()) {
        diff |= left_byte ^ right_byte;
    }
    diff == 0
}
