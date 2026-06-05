use super::*;
use crate::error::WalletError;
use base64::{engine::general_purpose::STANDARD, Engine};

#[test]
fn master_password_verifier_accepts_original_password() {
    let verifier = create_master_password_verifier("correct horse battery staple").unwrap();
    verify_master_password("correct horse battery staple", &verifier).unwrap();
}

#[test]
fn master_password_verifier_rejects_wrong_password() {
    let verifier = create_master_password_verifier("correct horse battery staple").unwrap();
    let error = verify_master_password("wrong password", &verifier).unwrap_err();
    assert_eq!(error, WalletError::InvalidPassword);
}

#[test]
fn unsupported_verifier_metadata_returns_invalid_password() {
    let mut verifier = create_master_password_verifier("correct horse battery staple").unwrap();
    verifier.kdf_name = "argon2id".to_string();

    let error = verify_master_password("correct horse battery staple", &verifier).unwrap_err();

    assert_eq!(error, WalletError::InvalidPassword);
}

#[test]
fn verifier_debug_redacts_sensitive_fields() {
    let verifier = create_master_password_verifier("correct horse battery staple").unwrap();
    let debug = format!("{verifier:?}");

    assert!(!debug.contains(&verifier.salt_b64));
    assert!(!debug.contains(&verifier.verifier_b64));
}

#[test]
fn malformed_verifier_length_returns_invalid_password() {
    let mut verifier = create_master_password_verifier("correct horse battery staple").unwrap();
    verifier.verifier_b64 = STANDARD.encode([1u8; 31]);

    let error = verify_master_password("correct horse battery staple", &verifier).unwrap_err();

    assert_eq!(error, WalletError::InvalidPassword);
}

#[test]
fn malformed_verifier_salt_length_returns_invalid_password() {
    let verifier = create_master_password_verifier("correct horse battery staple").unwrap();
    let verifier = MasterPasswordVerifier {
        salt_b64: STANDARD.encode([1u8; 15]),
        ..verifier
    };

    let error = verify_master_password("correct horse battery staple", &verifier).unwrap_err();

    assert_eq!(error, WalletError::InvalidPassword);
}

#[test]
fn verifier_bytes_are_domain_separated_from_keystore_encryption_key() {
    let verifier = create_master_password_verifier("correct horse battery staple").unwrap();
    let salt = STANDARD.decode(&verifier.salt_b64).unwrap();
    let keystore_key = derive_key("correct horse battery staple", &salt).unwrap();
    let verifier_bytes = STANDARD.decode(&verifier.verifier_b64).unwrap();

    assert_ne!(verifier_bytes.as_slice(), keystore_key.as_slice());
}
