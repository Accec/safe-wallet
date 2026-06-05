use super::*;
use base64::{engine::general_purpose::STANDARD, Engine};

const MNEMONIC: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

#[test]
fn encrypted_keystore_round_trips_mnemonic() {
    let keystore = encrypt_mnemonic(MNEMONIC, "master-password").unwrap();
    let decrypted = decrypt_mnemonic(&keystore, "master-password").unwrap();
    assert_eq!(decrypted, MNEMONIC);
}

#[test]
fn unsupported_keystore_metadata_returns_crypto_error() {
    let mut keystore = encrypt_mnemonic(MNEMONIC, "master-password").unwrap();
    keystore.cipher_name = "AES-128-GCM".to_string();

    let error = decrypt_mnemonic(&keystore, "master-password").unwrap_err();

    assert_eq!(error, WalletError::Crypto);
}

#[test]
fn tampered_keystore_metadata_fails_without_panic() {
    let mut keystore = encrypt_mnemonic(MNEMONIC, "master-password").unwrap();
    keystore.version += 1;

    let result = std::panic::catch_unwind(|| decrypt_mnemonic(&keystore, "master-password"));

    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap_err(), WalletError::Crypto);
}

#[test]
fn keystore_debug_redacts_sensitive_fields() {
    let keystore = encrypt_mnemonic(MNEMONIC, "master-password").unwrap();
    let debug = format!("{keystore:?}");

    assert!(!debug.contains(&keystore.ciphertext_b64));
    assert!(!debug.contains(&keystore.nonce_b64));
    assert!(!debug.contains(&keystore.salt_b64));
}

#[test]
fn encrypted_keystore_does_not_contain_plaintext_mnemonic() {
    let keystore = encrypt_mnemonic(MNEMONIC, "master-password").unwrap();
    assert!(!keystore.ciphertext_b64.contains("abandon"));
}

#[test]
fn wrong_password_cannot_decrypt_keystore() {
    let keystore = encrypt_mnemonic(MNEMONIC, "master-password").unwrap();
    let error = decrypt_mnemonic(&keystore, "wrong-password").unwrap_err();
    assert_eq!(error, WalletError::InvalidPassword);
}

#[test]
fn repeated_encryptions_use_different_ciphertext_and_nonce() {
    let first = encrypt_mnemonic(MNEMONIC, "master-password").unwrap();
    let second = encrypt_mnemonic(MNEMONIC, "master-password").unwrap();

    assert_ne!(first.ciphertext_b64, second.ciphertext_b64);
    assert_ne!(first.nonce_b64, second.nonce_b64);
}

#[test]
fn invalid_mnemonic_is_rejected_by_encrypt_mnemonic() {
    let error = encrypt_mnemonic("abandon abandon abandon", "master-password").unwrap_err();

    assert_eq!(error, WalletError::InvalidMnemonic);
}

#[test]
fn malformed_nonce_length_returns_error_without_panic() {
    let mut keystore = encrypt_mnemonic(MNEMONIC, "master-password").unwrap();
    keystore.nonce_b64 = STANDARD.encode([1u8; 11]);

    let result = std::panic::catch_unwind(|| decrypt_mnemonic(&keystore, "master-password"));

    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap_err(), WalletError::Crypto);
}

#[test]
fn malformed_salt_length_returns_error_without_panic() {
    let mut keystore = encrypt_mnemonic(MNEMONIC, "master-password").unwrap();
    keystore.salt_b64 = STANDARD.encode([1u8; 15]);

    let result = std::panic::catch_unwind(|| decrypt_mnemonic(&keystore, "master-password"));

    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap_err(), WalletError::Crypto);
}

#[test]
fn empty_ciphertext_returns_error_without_panic() {
    let mut keystore = encrypt_mnemonic(MNEMONIC, "master-password").unwrap();
    keystore.ciphertext_b64 = STANDARD.encode([]);

    let result = std::panic::catch_unwind(|| decrypt_mnemonic(&keystore, "master-password"));

    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap_err(), WalletError::Crypto);
}
