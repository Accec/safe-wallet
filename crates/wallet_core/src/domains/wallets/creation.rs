use chrono::Utc;
use uuid::Uuid;

use crate::chains;
use crate::domains::auth::AuthDomain;
use crate::error::WalletError;
use crate::models::WalletSummary;
use crate::storage::WalletDatabase;

use super::{accounts, keystore, mnemonic};

pub(super) fn create_wallet(
    database: &WalletDatabase,
    label: &str,
    mnemonic: &str,
    password: &str,
) -> Result<WalletSummary, WalletError> {
    AuthDomain::new(database.clone()).unlock_app(password)?;
    mnemonic::validate_mnemonic(mnemonic)?;
    let keystore = keystore::encrypt_mnemonic(mnemonic, password)?;
    let wallet = WalletSummary {
        id: Uuid::new_v4(),
        label: label.to_string(),
        created_at: Utc::now(),
    };
    let accounts = accounts::derive_default_accounts(wallet.id, mnemonic)?;
    database.wallets().save_wallet(
        &wallet,
        &keystore,
        &accounts,
        &chains::default_chain_settings(),
    )?;
    Ok(wallet)
}
