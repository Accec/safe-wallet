use k256::ecdsa::SigningKey;
use uuid::Uuid;

use crate::domains::wallets::{accounts, keystore, WalletsDomain};
use crate::error::WalletError;
use crate::models::Account;
use crate::storage::WalletDatabase;

pub(super) fn verify_wallet_password(
    database: &WalletDatabase,
    wallet_id: Uuid,
    password: &str,
) -> Result<(), WalletError> {
    WalletsDomain::new(database.clone()).verify_wallet_password(wallet_id, password)
}

pub(super) fn signing_key_for_account(
    database: &WalletDatabase,
    account: &Account,
    password: &str,
) -> Result<SigningKey, WalletError> {
    let keystore = database.wallets().load_wallet_keystore(account.wallet_id)?;
    match keystore.secret_kind.as_str() {
        keystore::SECRET_KIND_MNEMONIC => {
            let mnemonic = keystore::decrypt_mnemonic(&keystore, password)?;
            accounts::signing_key_for_mnemonic_path(&mnemonic, &account.derivation_path)
        }
        keystore::SECRET_KIND_PRIVATE_KEY => {
            let private_key = keystore::decrypt_private_key(&keystore, password)?;
            accounts::signing_key_for_private_key(&private_key)
        }
        _ => Err(WalletError::Crypto),
    }
}
