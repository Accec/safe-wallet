use chrono::Utc;
use uuid::Uuid;

use crate::chains;
use crate::domains::auth::AuthDomain;
use crate::error::WalletError;
use crate::models::{KeystoreExport, WalletSummary};
use crate::storage::WalletDatabase;

use super::{accounts, keystore};

pub(super) fn import_private_key(
    database: &WalletDatabase,
    label: &str,
    private_key: &str,
    password: &str,
) -> Result<WalletSummary, WalletError> {
    AuthDomain::new(database.clone()).unlock_app(password)?;
    let keystore = keystore::encrypt_private_key(private_key, password)?;
    let wallet = WalletSummary {
        id: Uuid::new_v4(),
        label: label.to_string(),
        created_at: Utc::now(),
    };
    let accounts = accounts::derive_private_key_accounts(wallet.id, private_key)?;
    database.wallets().save_wallet(
        &wallet,
        &keystore,
        &accounts,
        &chains::default_chain_settings(),
    )?;
    Ok(wallet)
}

pub(super) fn import_keystore(
    database: &WalletDatabase,
    label: &str,
    keystore_json: &str,
    keystore_password: &str,
    password: &str,
) -> Result<WalletSummary, WalletError> {
    AuthDomain::new(database.clone()).unlock_app(password)?;
    let export =
        serde_json::from_str::<KeystoreExport>(keystore_json).map_err(|_| WalletError::Crypto)?;
    let imported_keystore = keystore::encrypted_keystore_from_export(&export);
    let label = if label.trim().is_empty() {
        export.label.as_str()
    } else {
        label.trim()
    };
    let wallet = WalletSummary {
        id: Uuid::new_v4(),
        label: label.to_string(),
        created_at: Utc::now(),
    };
    let (keystore, accounts) = match imported_keystore.secret_kind.as_str() {
        keystore::SECRET_KIND_MNEMONIC => {
            let mnemonic = keystore::decrypt_mnemonic(&imported_keystore, keystore_password)?;
            let keystore = keystore::encrypt_mnemonic(&mnemonic, password)?;
            let accounts = accounts::derive_default_accounts(wallet.id, &mnemonic)?;
            (keystore, accounts)
        }
        keystore::SECRET_KIND_PRIVATE_KEY => {
            let private_key = keystore::decrypt_private_key(&imported_keystore, keystore_password)?;
            let keystore = keystore::encrypt_private_key(&private_key, password)?;
            let accounts = accounts::derive_private_key_accounts(wallet.id, &private_key)?;
            (keystore, accounts)
        }
        _ => return Err(WalletError::Crypto),
    };
    database.wallets().save_wallet(
        &wallet,
        &keystore,
        &accounts,
        &chains::default_chain_settings(),
    )?;
    Ok(wallet)
}
