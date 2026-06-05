use uuid::Uuid;

use crate::domains::auth::AuthDomain;
use crate::error::WalletError;
use crate::models::KeystoreExport;
use crate::storage::WalletDatabase;

use super::keystore;

pub(super) fn export_keystore(
    database: &WalletDatabase,
    wallet_id: Uuid,
    password: &str,
) -> Result<KeystoreExport, WalletError> {
    AuthDomain::new(database.clone()).unlock_app(password)?;
    let wallet = database.wallets().load_wallet_summary(wallet_id)?;
    let keystore = database.wallets().load_wallet_keystore(wallet_id)?;
    keystore::verify_keystore_password(&keystore, password)?;
    Ok(KeystoreExport {
        wallet_id,
        label: wallet.label,
        secret_kind: keystore.secret_kind,
        ciphertext_b64: keystore.ciphertext_b64,
        nonce_b64: keystore.nonce_b64,
        salt_b64: keystore.salt_b64,
        kdf_name: keystore.kdf_name,
        kdf_params_json: keystore.kdf_params_json,
        cipher_name: keystore.cipher_name,
        version: keystore.version,
    })
}

pub(super) fn delete_wallet(
    database: &WalletDatabase,
    wallet_id: Uuid,
    password: &str,
) -> Result<(), WalletError> {
    verify_wallet_password(database, wallet_id, password)?;
    database.wallets().delete_wallet(wallet_id)
}

pub(super) fn reveal_mnemonic(
    database: &WalletDatabase,
    wallet_id: Uuid,
    password: &str,
) -> Result<String, WalletError> {
    AuthDomain::new(database.clone()).unlock_app(password)?;
    let keystore = database.wallets().load_wallet_keystore(wallet_id)?;
    keystore::decrypt_mnemonic(&keystore, password)
}

pub(crate) fn verify_wallet_password(
    database: &WalletDatabase,
    wallet_id: Uuid,
    password: &str,
) -> Result<(), WalletError> {
    AuthDomain::new(database.clone()).unlock_app(password)?;
    let keystore = database.wallets().load_wallet_keystore(wallet_id)?;
    keystore::verify_keystore_password(&keystore, password)
}
